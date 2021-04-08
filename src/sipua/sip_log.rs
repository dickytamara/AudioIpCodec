
use std::{borrow::BorrowMut, cell::{RefCell, RefMut}};

use pj_sys::*;
use pjsua_sys::*;

use crate::pjproject::utils::{self, AutoCreate, ToString, FromString};
use crate::pjproject::pjsua;


pub struct SIPLog {
    ctx: RefCell<pjsua_logging_config>
}

pub trait SIPLogExt {
    /// Get Log incoming and outgoing SIP message? Yes!
    fn get_msg_logging(&self) -> bool;
    /// Set Log incoming and outgoing SIP message? Yes!
    fn set_msg_logging(&self, value: bool);
    /// Get Input verbosity level. Value 5 is reasonable.
    fn get_level(&self) -> u32;
    /// Set Input verbosity level. Value 5 is reasonable.
    fn set_level(&self, level: u32);
    /// Get Verbosity level for console. Value 4 is reasonable.
    fn get_console_level(&self) -> u32;
    /// Set Verbosity level for console. Value 4 is reasonable.
    fn set_console_level(&self, console_level: u32);
    /// Get Log decoration.
    fn get_decor (&self) -> u32;
    /// Set Log decoration.
    fn set_decor(&self, decor: u32);
    /// Set Optional log filename.
    fn get_log_filename(&self) -> String;
    /// Get Optional log filename.
    fn set_log_filename(&self, log_filename: String);
    /// Get Additional flags to be given to pj_file_open() when opening the log file. By default,
    /// the flag is PJ_O_WRONLY. Application may set PJ_O_APPEND here so that
    /// logs are appended to existing file instead of overwriting it.
    ///
    /// # Default
    /// is 0.
    fn get_log_file_flags(&self) -> u32;
    /// Set Additional flags to be given to pj_file_open() when opening the log file. By default,
    /// the flag is PJ_O_WRONLY. Application may set PJ_O_APPEND here so that
    /// logs are appended to existing file instead of overwriting it.
    ///
    /// # Default
    /// is 0.
    fn set_log_file_flags(&self, log_file_flags: u32);
    // TODO Implement callback API.
    // pub cb: Option< unsafe extern "C" fn( level: c_int, data: *const c_char, len: c_int, ) >
}

impl SIPLog {

    pub fn new() -> Self {
        SIPLog {
            ctx: RefCell::new(pjsua_logging_config::new())
        }
    }

    pub fn get_context(&mut self) -> RefMut<pjsua_logging_config> {
        self.ctx.borrow_mut()
    }

    /// Use this function to initialize logging config.
    pub fn config_default(&self, cfg: &mut pjsua_logging_config) {
        pjsua::logging_config_default(cfg);
    }

    /// Use this function to duplicate logging config.
    pub fn config_dup(&self, dst: &mut pjsua_logging_config) {
        pjsua::logging_config_dup(dst, &mut self.ctx.borrow_mut());
    }

    /// Application can call this function at any time (after create(), of course)
    /// to change logging settings.
    pub fn reconfigure(&self) {
        pjsua::reconfigure_logging(&mut self.ctx.borrow_mut())
        .expect("SIPLog::pjsua_reconfigure_logging");
    }

}

impl SIPLogExt for SIPLog {

    fn get_msg_logging(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().msg_logging)
    }

    fn set_msg_logging(&self, value: bool) {
        self.ctx.borrow_mut().msg_logging = utils::boolean_to_pjbool(value);
    }

    fn get_level(&self) -> u32 {
        self.ctx.borrow().level
    }

    fn set_level(&self, level: u32) {
        self.ctx.borrow_mut().level = level;
    }

    fn get_console_level(&self) -> u32 {
        self.ctx.borrow().console_level
    }

    fn set_console_level(&self, console_level: u32) {
        self.ctx.borrow_mut().console_level = console_level;
    }

    fn get_decor (&self) -> u32 {
        self.ctx.borrow().decor
    }

    fn set_decor(&self, decor: u32) {
        self.ctx.borrow_mut().decor = decor;
    }

    fn get_log_filename(&self) -> String {
        self.ctx.borrow().log_filename.to_string().clone()
    }

    fn set_log_filename(&self, log_filename: String) {
        self.ctx.borrow_mut().log_filename = pj_str_t::from_string(log_filename);
    }

    fn get_log_file_flags(&self) -> u32 {
        self.ctx.borrow().log_file_flags
    }

    fn set_log_file_flags(&self, log_file_flags: u32) {
        self.ctx.borrow_mut().log_file_flags = log_file_flags;
    }

}