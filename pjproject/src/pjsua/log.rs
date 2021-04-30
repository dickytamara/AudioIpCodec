
use std::path::Path;


pub trait LogConfigExt {
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
    fn set_log_filename(&self, log_filename: Path);
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