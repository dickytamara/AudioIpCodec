
use pjsip_ua_sys::pjsip_timer_setting;
use crate::prelude::AutoCreate;




impl AutoCreate<pjsip_timer_setting> for pjsip_timer_setting {
    fn new() -> Self {
        Self {
            min_se: 0,
            sess_expires: 0,
        }
    }
}