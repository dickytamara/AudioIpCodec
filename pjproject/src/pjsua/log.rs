
use std::{convert::TryFrom, path::PathBuf};
use crate::utils::{boolean_to_pjbool, check_boolean};

use super::*;

pub trait LogConfigExt {

    /// Get Log incoming and outgoing SIP message? Yes!
    fn set_msg_logging(&mut self, value: bool);
    fn get_msg_logging(&self) -> bool;

    /// Get Input verbosity level. Value 5 is reasonable.
    fn set_level(&mut self, value: u32);
    fn get_level(&self) -> u32;

    /// Get Verbosity level for console. Value 4 is reasonable.
    fn set_console_level(&mut self, value: u32);
    fn get_console_level(&self) -> u32;

    /// Get Log decoration.
    fn set_decor(&mut self, value: u32);
    fn get_decor (&self) -> u32;

    /// Set Optional log filename.
    fn set_log_filename(&mut self, value: PathBuf);
    fn get_log_filename(&self) -> PathBuf;

    /// Get Additional flags to be given to pj_file_open() when opening the log file. By default,
    /// the flag is PJ_O_WRONLY. Application may set PJ_O_APPEND here so that
    /// logs are appended to existing file instead of overwriting it.
    ///
    /// # Default
    /// is 0.
    fn set_log_file_flags(&mut self, value: LogConfigFileFlags);
    fn get_log_file_flags(&self) -> LogConfigFileFlags;

    // TODO Implement callback API.
    // pub cb: Option< unsafe extern "C" fn( level: c_int, data: *const c_char, len: c_int, ) >
}

impl LogConfigExt for LogConfig {

    fn get_msg_logging(&self) -> bool {
        check_boolean(self.msg_logging)
    }

    fn set_msg_logging(&mut self, value: bool) {
        self.msg_logging = boolean_to_pjbool(value);
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn set_level(&mut self, value: u32) {
        self.level = value;
    }

    fn get_console_level(&self) -> u32 {
        self.console_level
    }

    fn set_console_level(&mut self, value: u32) {
        self.console_level = value;
    }

    fn get_decor (&self) -> u32 {
        self.decor
    }

    fn set_decor(&mut self, value: u32) {
        self.decor = value;
    }

    fn get_log_filename(&self) -> PathBuf {
        PathBuf::from(self.log_filename.to_string().as_str())
    }

    fn set_log_filename(&mut self, value: PathBuf) {
        self.log_filename = pj_str_t::from_string(String::from(value.to_str().unwrap()));
    }

    fn get_log_file_flags(&self) -> LogConfigFileFlags {
        LogConfigFileFlags::try_from(self.log_file_flags)
        .expect("Error LogConfig get log_file_flags")
    }

    fn set_log_file_flags(&mut self, value: LogConfigFileFlags) {
        self.log_file_flags = value.into();
    }
}