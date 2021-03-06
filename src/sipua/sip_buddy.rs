
use super::pj_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;

pub struct SIPBuddy {
    id: i32,
    ctx: pjsua_buddy_config,
}

impl SIPBuddy {
    pub fn new() -> SIPBuddy {
        SIPBuddy {
            id: -1,
            ctx: pjsua_buddy_config::new(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let status = pjsua_buddy_add(&mut self.ctx as *const _, &mut self.id as *mut _);

            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPBuddy");
            }

            assert_ne!(self.id, -1);
        }
    }
}
