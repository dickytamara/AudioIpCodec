

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
