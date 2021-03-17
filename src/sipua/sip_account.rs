
use super::pj_sys::*;
use super::pjsip_simple_sys::*;
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
    info: pjsua_acc_info,
    ctx: pjsua_acc_config,
}

impl SIPAccount {

    pub fn new() -> SIPAccount {
        SIPAccount {
            id: -1,
            info: pjsua_acc_info::new(),
            ctx: pjsua_acc_config::new(),
        }
    }

    pub fn from(acc_id: pjsua_acc_id) -> Result<SIPAccount, i32> {
        let mut ret = SIPAccount {
            id: acc_id,
            info: pjsua_acc_info::new(),
            ctx: pjsua_acc_config::new()
        };

        let status = pjsua::acc_get_info(ret.id, &mut ret.info);
        if status != PJ_SUCCESS as pj_status_t {
            return Err(status);
        }

        let status = pjsua::acc_get_config(acc_id, &mut ret.ctx);
        if status != PJ_SUCCESS as pj_status_t {
            return Err(status);
        }

        Ok(ret)
    }

    // configure for default setting.
    pub fn config_default(&mut self) {

        pjsua::acc_config_default(&mut self.ctx);

        self.ctx.cred_count = 1;
        self.ctx.reg_retry_interval = 300;
        self.ctx.reg_first_retry_interval = 60;
        self.ctx.cred_info[0].data_type = 0;
        self.ctx.cred_info[0].scheme = pj_str_t::from_string(String::from("Digest"));
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
        let result = pjsua::acc_set_default(self.id);

        if result != PJ_SUCCESS as pj_status_t {
            println!("ERR cant set acc id={} to be default account.",
            self.id)
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

            // update account info
            let result = pjsua::acc_get_info(self.id, &mut self.info);
            if result != PJ_SUCCESS as pj_status_t {
                println!("ERR cant update account info.");
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

    // Send NOTIFY to inform account presence status or to terminate server side presence subscription.
    // If application wants to reject the incoming request,
    // it should set the state to PJSIP_EVSUB_STATE_TERMINATED.
    pub fn pres_notify (&self,
        srv_pres: &mut pjsua_srv_pres,
        state: pjsip_evsub_state,
        state_str: String,
        reason: String,
        with_body: bool,
        msg_data: &mut pjsua_msg_data
    ) {

        let mut awith_body = PJ_FALSE as pj_bool_t;

        if with_body {
            awith_body = PJ_TRUE as pj_bool_t;
        }

        unsafe {

            let status = pjsua_pres_notify(
                self.id,
                srv_pres as *mut _,
                state,
                &mut pj_str_t::from_string(state_str) as *const _,
                &mut pj_str_t::from_string(reason) as *const _,
                awith_body,
                msg_data as *const _
            );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant nofify presents for account.");
            }
        }
    }

    // Send instant messaging outside dialog, using the specified account for route set and authentication
    pub fn im_send(&self,
        to: String,
        mime_type: String,
        content: String,
        msg_data: &mut pjsua_msg_data
    ) {
        // we not intended to support file transfer
        // so user_data will set to null
        unsafe {

            let status = pjsua_im_send(
                self.id,
                &mut pj_str_t::from_string(to) as *const _,
                &mut pj_str_t::from_string(mime_type) as *const _,
                &mut pj_str_t::from_string(content) as *const _,
                msg_data as *const _,
                ptr::null_mut()
            );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant send im for account");
            }
        }
    }

    // Send typing indication outside dialog.
    pub fn im_typing(&self, to: String, is_typing: bool, msg_data: &mut pjsua_msg_data) {

        let ais_typing = PJ_FALSE as pj_bool_t;

        unsafe {

            let status = pjsua_im_typing(
                self.id,
                &mut pj_str_t::from_string(to) as *const _,
                ais_typing as pj_bool_t,
                msg_data as *const _
            );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant im typing indication for account")
            }

        }
    }

    pub fn set_rtp_config(&mut self, rtp_config: pjsua_transport_config) {
        self.ctx.rtp_cfg = rtp_config;
    }

}

// true interface for managing accounts
pub struct SIPAccounts {  }

// please remember default pjsua account
// was 8 see pjsua sample application at
// pjproject.
impl SIPAccounts {

    // create new accounts
    pub fn new() -> SIPAccounts {
        SIPAccounts {
            // acc_list: [SIPAccount::new(); PJSUA_MAX_ACC as usize],
        }
    }

    // get default account
    pub fn get_default(&self) -> pjsua_acc_id {
        unsafe {
            pjsua_acc_get_default()
        }
    }

    // set default rtp config
    pub fn set_rtp_config(&self, rtp_config: pjsua_transport_config) {

        let ids = self.enum_accs_id();

        ids.iter().for_each(|acc_id| {

            let mut acc = SIPAccount::from(acc_id.clone());

            match acc {
                Ok(ref mut sipacc) => {
                    sipacc.set_rtp_config(rtp_config);
                },
                Err(err) => println!("print error ={}", err)
            }
        });
    }

    // set register retry interval
    pub fn set_reg_retry_interval(&self, value: u32) {

        let ids = self.enum_accs_id();

        ids.iter().for_each(|acc_id| {

            let mut acc = SIPAccount::from(acc_id.clone());

            match acc {
                Ok(ref mut sipacc) => {
                    sipacc.ctx.reg_retry_interval = value;
                },
                Err(err) => println!("print error ={}", err)
            }
        });
    }

    // set first register retry interval
    pub fn set_reg_first_retry_interval(&self, value: u32) {

        let ids = self.enum_accs_id();

        ids.iter().for_each(|acc_id| {

            let mut acc = SIPAccount::from(acc_id.clone());

            match acc {
                Ok(ref mut sipacc) => {
                    sipacc.ctx.reg_first_retry_interval = value;
                },
                Err(err) => println!("print error ={}", err)
            }
        });
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
            let mut count = 8u32;

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
