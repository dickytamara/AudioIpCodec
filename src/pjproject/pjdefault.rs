use pj_sys::{PJ_FALSE, PJ_SUCCESS, PJ_TRUE, pj_bool_t, pj_status_t};



//use std::os::raw::{c_char, c_void};
//use std::mem;

// global trait for new struct
pub trait AutoCreate<T> {
    fn new() -> T;
}

pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait FromString<T> {
    fn from_string(value: String) -> T;
}

pub fn check_status(status: pj_status_t) -> Result<(), pj_status_t> {
    if status == PJ_SUCCESS as pj_status_t {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn check_boolean(value: pj_bool_t) -> bool {
    if value == PJ_TRUE as pj_bool_t {
        true
    } else {
        false
    }
}

pub fn boolean_to_pjbool(value: bool) -> pj_bool_t {

    if value {
        PJ_TRUE as pj_bool_t
    } else {
        PJ_FALSE as pj_bool_t
    }
}

