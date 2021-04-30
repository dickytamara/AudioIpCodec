
use super::prelude::*;
// use super::utils;
use pjsip_ua_sys::*;



impl AutoCreate<pjsip_timer_setting> for pjsip_timer_setting {
    fn new() -> pjsip_timer_setting {
        pjsip_timer_setting {
            min_se: 0,
            sess_expires: 0,
        }
    }
}