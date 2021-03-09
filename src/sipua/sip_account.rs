
use super::pj_sys::*;
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

    pub fn set_id(&mut self, value: String) {

        self.ctx.id = pj_str_t::from_string(value);

    }

    pub fn set_reg_uri(&mut self, value: String) {

        self.ctx.reg_uri = pj_str_t::from_string(value);

    }

    pub fn set_realm(&mut self, value: String) {

        self.ctx.cred_info[0].realm = pj_str_t::from_string(value);

    }

    pub fn set_username(&mut self, value: String) {

        self.ctx.cred_info[0].username = pj_str_t::from_string(value);

    }

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

}

// true interface for managing accounts
pub struct SIPAccounts {
    acc_list: [SIPAccount; PJSUA_MAX_ACC as usize]
}


// please remember default pjsua account
// was 8 see pjsua sample application at
// pjproject.
impl SIPAccounts {

    pub fn new() -> SIPAccounts {
        SIPAccounts {
            acc_list: [SIPAccount::new(); PJSUA_MAX_ACC as usize],
        }
    }

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

    pub fn get_count(&self) -> u32 {
        unsafe {
            pjsua_acc_get_count()
        }
    }
}
