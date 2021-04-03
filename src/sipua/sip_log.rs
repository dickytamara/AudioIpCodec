
use super::pj_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::AutoCreate;


pub struct SIPLog {
    ctx: pjsua_logging_config
}

impl SIPLog {
    pub fn new() -> Self {
        SIPLog {
            ctx: pjsua_logging_config::new()
        }
    }

    pub fn get_context(&mut self) -> pjsua_logging_config {
        self.ctx.clone()
    }
}