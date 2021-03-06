
use super::pj_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;
use super::pjsua::*;

pub struct SIPAccount {
    id: i32,
    ctx: pjsua_acc_config,
}

impl SIPAccount {
    pub fn new(rtp_config: pjsua_transport_config) -> SIPAccount {
        let mut acc = SIPAccount {
            id: -1,
            ctx: pjsua_acc_config::new(),
        };

        acc.ctx.rtp_cfg = rtp_config;
        acc.ctx.reg_retry_interval = 300;
        acc.ctx.reg_first_retry_interval = 60;

        unsafe {
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
}

// true interface for managing accounts
pub struct SIPAccounts {
    acc_list: Vec<SIPAccount>
}

impl SIPAccounts {

  pub fn new() -> SIPAccounts {
        SIPAccounts {
            acc_list: Vec::<SIPAccount>::new(),
        }
    }

    pub fn add() {}

    pub fn delete() {}

    pub fn modify() {}

    pub fn register() {}

    pub fn unregister() {}

    pub fn next() {}

    pub fn prev() {}

    pub fn show() {}
}
