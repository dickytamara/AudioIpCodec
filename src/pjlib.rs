

//mod pjproject_sys;
use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;
use std::os::raw::c_char;
use std::mem;


impl AutoCreate<pj_str_t>  for pj_str_t {
    fn new() -> pj_str_t {
        unsafe {
            let zeromem: *mut c_char  = mem::zeroed() ;
            pj_str(zeromem)
        }
    }
}
