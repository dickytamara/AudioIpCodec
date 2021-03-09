
use super::pj_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;
use super::pjsua::*;

use std::ptr;
use std::ffi::CString;

pub struct SIPAccount {
    id: i32,
    ctx: pjsua_acc_config,
}

impl SIPAccount {

    pub fn new(rtp_config: pjsua_transport_config) -> SIPAccount {
        let mut acc = SIPAccount {
            id: 0,
            ctx: pjsua_acc_config::new(),
        };

        acc.ctx.rtp_cfg = rtp_config;
        acc.ctx.reg_retry_interval = 300;
        acc.ctx.reg_first_retry_interval = 60;
        acc.ctx.cred_info[0].data_type = 0;


        unsafe {

            acc.ctx.cred_info[0].scheme = pj_str(
                CString::new("Digest")
                    .expect("Error create digest string")
                    .into_raw() as *mut _
            );

            // pjsua_acc_config acc_cfg;
            // pj_status_t status;

            // pjsua_acc_config_default(&acc_cfg);
            // acc_cfg.id = cval->argv[1];
            // acc_cfg.reg_uri = cval->argv[2];
            // acc_cfg.cred_count = 1;
            // acc_cfg.cred_info[0].scheme = pj_str("Digest");
            // acc_cfg.cred_info[0].realm = cval->argv[3];
            // acc_cfg.cred_info[0].username = cval->argv[4];
            // acc_cfg.cred_info[0].data_type = 0;
            // acc_cfg.cred_info[0].data = cval->argv[5];

            // acc_cfg.rtp_cfg = app_config.rtp_cfg;
            // app_config_init_video(&acc_cfg);

            // status = pjsua_acc_add(&acc_cfg, PJ_TRUE, NULL);
            // if (status != PJ_SUCCESS) {
            // pjsua_perror(THIS_FILE, "Error adding new account", status);
            // }


            // let mut status = pjsua_acc_add(
            //     &mut acc.ctx as *mut _,
            //     PJ_TRUE as pj_bool_t,
            //     ptr::null_mut(),
            // );

            let mut status = pjsua_acc_add(
                &mut acc.ctx as *const _,
                PJ_TRUE as pj_bool_t,
                &mut acc.id as *mut _,
            );


            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPAccount");
            }

            status = pjsua_acc_set_online_status(
                pjsua_acc_get_default(),
                PJ_TRUE as pj_bool_t,
            );
            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPAccount");
            }
        }
        assert_ne!(acc.id, -1);

        acc
    }

    pub fn set_id(&mut self, value: String) {
        unsafe {
            self.ctx.id = pj_str(CString::new(value.as_str())
                .expect("invalid string id")
                .into_raw() as *mut _
            );
        }
    }

    pub fn set_reg_uri(&mut self, value: String) {
        unsafe {
            self.ctx.reg_uri = pj_str(CString::new(value.as_str())
                .expect("invalid reg uri")
                .into_raw() as *mut _
            );
        }
    }

    pub fn set_realm(&mut self, value: String) {
        unsafe {
            self.ctx.cred_info[0].realm = pj_str(
                CString::new(value.as_str())
                .expect("invalid string realm")
                .into_raw() as *mut _
            );
        }
    }

    pub fn set_username(&mut self, value: String) {
        unsafe {
            self.ctx.cred_info[0].username = pj_str(
                CString::new(value.as_str())
                .expect("invalid string username")
                .into_raw() as *mut _
            );
        }
    }

    pub fn set_password(&mut self, value: String) {
        unsafe {
            self.ctx.cred_info[0].data = pj_str(
                CString::new(value.as_str())
                .expect("invalid string password")
                .into_raw() as *mut _
            );
        }
    }

}

// true interface for managing accounts
pub struct SIPAccounts {
    acc_list: Vec<SIPAccount>
}


// please remember default pjsua account
// was 32 see pjsua sample application at
// pjproject.
impl SIPAccounts {

  pub fn new() -> SIPAccounts {
        SIPAccounts {
            acc_list: Vec::new(),
        }
    }

    pub fn add(&mut self, rtp_config: pjsua_transport_config) {
        // just add to list
        let newacc = SIPAccount::new(rtp_config);
        self.acc_list.push(newacc);
    }

    pub fn delete() {}

    pub fn modify() {}

    pub fn register() {}

    pub fn unregister() {}

    pub fn next() {}

    pub fn prev() {}

    pub fn show() {}
}
