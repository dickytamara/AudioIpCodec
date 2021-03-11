
use super::pj_sys::*;
use super::pjsip_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;
use super::pjsua::*;

use super::pjsua;

use std::ptr;
use std::ffi::CString;
use std::os::raw::c_uint;


#[derive(Clone, Copy)]
pub struct SIPAccount {
    id: i32,
    ctx: pjsua_acc_config,
}

impl SIPAccount {

    fn new() -> SIPAccount {
        SIPAccount {
            id: -1,
            ctx: pjsua_acc_config::new(),
        }
    }

    // set sip id for account
    pub fn set_id(&mut self, value: String) {

        self.ctx.id = pj_str_t::from_string(value);

    }

    // set registrar uri
    pub fn set_reg_uri(&mut self, value: String) {

        self.ctx.reg_uri = pj_str_t::from_string(value);

    }

    // set realm for account
    pub fn set_realm(&mut self, value: String) {

        self.ctx.cred_info[0].realm = pj_str_t::from_string(value);

    }

    // set username
    pub fn set_username(&mut self, value: String) {

        self.ctx.cred_info[0].username = pj_str_t::from_string(value);

    }

    // set password
    pub fn set_password(&mut self, value: String) {

        self.ctx.cred_info[0].data = pj_str_t::from_string(value);

    }

    // check if this account valid or not
    pub fn is_valid(&self) -> bool {
        unsafe {
            if pjsua_acc_is_valid(self.id) == PJ_TRUE as i32 {
                true
            } else {
                false
            }
        }
    }

    // set this account to be default
    pub fn set_default(&self) {
        unsafe {
            let result = pjsua_acc_set_default(self.id);

            if result != PJ_SUCCESS as pj_status_t {
                println!("ERR cant set acc id={} to be default account.",
                self.id)
            }
        }
    }

    // add account to internal pjsua
    pub fn add(&mut self, is_default: bool) {

        let mut default = PJ_FALSE as pj_bool_t;

        if is_default {
            default = PJ_TRUE as pj_bool_t;
        }

        unsafe {

            let result = pjsua_acc_add(
                    &mut self.ctx as *const _,
                    default,
                    &mut self.id as *mut _
                );

            if result != PJ_SUCCESS as pj_status_t {
                println!("ERR cant add account to pjsua.");
            }
        }
    }

    // remove account from internal pjsua
    pub fn del(&self) {
        unsafe{
            let result = pjsua_acc_del(self.id);
            if result != PJ_SUCCESS as pj_status_t {
                println!("ERR cant delete account from pjsua..");
            }
        }
    }

    // get inner config
    pub fn get_config(&self) -> Result<pjsua_acc_config, i32> {
        unsafe{

            let mut acc_cfg = pjsua_acc_config::new();

            let pool = pjsua::pool_create("tmp-pool");

            let result = pjsua_acc_get_config(
                    self.id,
                    pool,
                    &mut acc_cfg as *mut _
                );


            pjsua::pool_release(pool);
            // pjsua::pool_safe_release(pool as *mut _);

            if result != PJ_SUCCESS as pj_status_t {
                println!("ERR cant get account config");
                Err(result)
            } else {
                Ok(acc_cfg)
            }
        }
    }

    // modify acoount config
    pub fn modify(&self, acc_config: &mut pjsua_acc_config) {
        unsafe {
            let result = pjsua_acc_modify(
                    self.id,
                    acc_config as *mut _
                );

            if result != PJ_SUCCESS as pj_status_t {
                println!("ERR cant modify account config");
            }
        }
    }

    // set online or ofline status
    pub fn set_online_status(&self, is_online: bool) {

        let mut online = PJ_FALSE;

        if is_online {
            online = PJ_TRUE;
        }

        unsafe {

            let status = pjsua_acc_set_online_status(
                self.id,
                online as pj_bool_t
            );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant set online or offline account");
            }
        }
    }

    // this online status more like presence state for account
    pub fn set_online_status2(&self, is_online: bool, pr: &mut pjrpid_element) {

        let mut online = PJ_FALSE;

        if is_online {
            online = PJ_TRUE;
        }

        unsafe {

            let status = pjsua_acc_set_online_status2(
                    self.id,
                    online as pj_bool_t,
                    pr as *const _
                );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant set account presence.");
            }
        }
    }

    // set registration process
    pub fn set_registration(&self, renew: bool) {

        let mut new = PJ_FALSE;

        if renew {
            new = PJ_TRUE
        }

        unsafe {

            let status = pjsua_acc_set_registration(
                    self.id,
                    new as pj_bool_t
                );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant set registration status.");
            }
        }
    }

    // get inner account info
    pub fn get_info(&self) -> Result<pjsua_acc_info, i32> {

        let mut info = pjsua_acc_info::new();

        unsafe {

            let status = pjsua_acc_get_info(
                    self.id,
                    &mut info as *mut _
                );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant get account info");
                Err(status)
            } else {
                Ok(info)
            }
        }

    }

    // Create arbitrary requests using the account.
    // Application should only use this function to create auxiliary requests outside dialog,
    // such as OPTIONS, and use the call or presence API to create dialog related requests.
    pub fn create_request(&self, method: &mut pjsip_method, target: String) -> Result<pjsip_tx_data, i32> {

        let mut rdata = pjsip_tx_data::new();
        let mut rtarget = pj_str_t::from_string(target);
        unsafe {

            let status = pjsua_acc_create_request(
                    self.id,
                    method as *const _,
                    &mut rtarget as *const _,
                    (&mut rdata as *mut _) as *mut _
                );

            if status == PJ_SUCCESS as pj_status_t {
                println!("ERR cant create request for account");
                Ok(rdata)
            } else {
                Err(status)
            }
        }
    }

    // Create a suitable Contact header value,
    // based on the specified target URI for the specified account.
    pub fn create_uac_contact(&self, contact: String, uri: String) {

        let pool = pjsua::pool_create("tmp-pool");
        let mut acontact = pj_str_t::from_string(contact);
        let mut auri = pj_str_t::from_string(uri);

        unsafe {

            let status = pjsua_acc_create_uac_contact(
                    pool,
                    &mut acontact as *mut _,
                    self.id,
                    &mut auri as *mut _
                );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant create uac contact for account.");
            }
        }

        pjsua::pool_release(pool);

    }

    // Create a suitable Contact header value, based on the information in the incoming request.
    pub fn create_uas_contact(&self, contact: String, rdata: &mut pjsip_rx_data) {

        let pool = pjsua::pool_create("tmp-pool");
        let mut acontact = pj_str_t::from_string(contact);

        unsafe {

            let status = pjsua_acc_create_uas_contact(
                    pool,
                    &mut acontact as *mut _,
                    self.id,
                    rdata as *mut _
                );

            if status != PJ_SUCCESS as pj_status_t {

                println!("ERR cant create uas contact for account")

            }

        }

        pjsua::pool_release(pool)
    }

    // set transport by given id transport id for account
    pub fn set_transport(&self, tp_id: pjsua_transport_id) {

        unsafe {

            let status = pjsua_acc_set_transport(
                    self.id,
                    tp_id
                );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant set transport for account.");
            }
        }
    }

}

// true interface for managing accounts
pub struct SIPAccounts {
    acc_list: [SIPAccount; PJSUA_MAX_ACC as usize]
}


// please remember default pjsua account
// was 8 see pjsua sample application at
// pjproject.
impl SIPAccounts {

    // create new accounts
    pub fn new() -> SIPAccounts {
        SIPAccounts {
            acc_list: [SIPAccount::new(); PJSUA_MAX_ACC as usize],
        }
    }

    // get default account
    pub fn get_default(&self) -> pjsua_acc_id {
        unsafe {
            pjsua_acc_get_default()
        }
    }

    // set default rtp config
    pub fn set_rtp_config(&mut self, rtp_config: pjsua_transport_config) {
        for acc in self.acc_list.iter_mut() {
            acc.ctx.rtp_cfg = rtp_config;
        }
    }

    // set register retry interval
    pub fn set_reg_retry_interval(&mut self, value: u32) {
        for acc in self.acc_list.iter_mut() {
            acc.ctx.reg_retry_interval = value;
        }
    }

    // set first register retry interval
    pub fn set_reg_first_retry_interval(&mut self, value: u32) {
        for acc in self.acc_list.iter_mut() {
            acc.ctx.reg_first_retry_interval = value;
        }
    }

    // get number of registered to pjsua library
    pub fn get_count(&self) -> u32 {
        unsafe {
            pjsua_acc_get_count()
        }
    }

    // enumerate current registered account id
    pub fn enum_accs_id(&self) -> Vec<pjsua_acc_id> {

        let mut ret: Vec<pjsua_acc_id> = Vec::new();

        unsafe {

            let mut acc_id: [pjsua_acc_id; PJSUA_MAX_ACC as usize] = [-1; PJSUA_MAX_ACC as usize];
            let mut count = 0u32;

            let status = pjsua_enum_accs(
                    acc_id.as_mut_ptr(),
                    &mut count as *mut _
                );

            if status == PJ_SUCCESS as pj_status_t {

                for i in 0..count as usize {
                    ret.push(acc_id[i]);
                }

            } else {
                println!("ERR cant enumerate accounts id.");
            }
        }

        ret
    }

    // enumerate current registered account info
    pub fn enum_info(&self) -> Vec<pjsua_acc_info> {

        let mut ret: Vec<pjsua_acc_info> = Vec::new();

        unsafe {

            let mut acc_info: [pjsua_acc_info; PJSUA_MAX_ACC as usize] = [pjsua_acc_info::new(); PJSUA_MAX_ACC as usize];
            let mut count = 0u32;

            let status = pjsua_acc_enum_info(
                    acc_info.as_mut_ptr(),
                    &mut count as *mut _
                );

            if status == PJ_SUCCESS as pj_status_t {

                for i in 0..count as usize {
                    ret.push(acc_info[i]);
                }

            } else {
                println!("ERR cant enumerate account info.");
            }
        }

        ret
    }

    // This is an internal function to find the most appropriate account to used to reach to the specified URL.
    pub fn find_for_outgoing(&self, value: String) -> pjsua_acc_id {
        unsafe {
            pjsua_acc_find_for_outgoing(&mut pj_str_t::from_string(value) as *const _)
        }
    }

    // This is an internal function to find the most appropriate account to be used to handle incoming calls.
    pub fn on_acc_find_for_incoming (&self, rdata: &mut pjsip_rx_data) -> pjsua_acc_id {
        unsafe {
            pjsua_acc_find_for_incoming(rdata as *mut _)
        }
    }



}
