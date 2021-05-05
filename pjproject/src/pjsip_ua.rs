
use super::prelude::*;
use pjsip_ua_sys::*;

pub mod auto;

impl AutoCreate<pjsip_timer_setting> for pjsip_timer_setting {
    fn new() -> Self {
        Self {
            min_se: 0,
            sess_expires: 0,
        }
    }
}