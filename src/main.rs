

extern crate pjsua_sys;
extern crate pj_sys;

mod pjdefault;
mod pjlib;
mod pjsip;
mod pjmedia;
mod pjsua;

use pjsua_sys::*;
//use std::mem;
use pjdefault::AutoCreate;


/// create new pjsua_logging_config

/// create new pjsua_media_config
// pub fn new_media_config() -> pjsua_media_config {
// }

// public struct untuk pjsua
pub struct Pjsua {
    pub config: Option<pjsua_config>,
    pub logging: Option<pjsua_logging_config>,
    pub media: Option<pjsua_media_config>,
    pub status: i32
}


pub trait PjsuaCallback { }


impl Pjsua {
    /// create new pjsua_config

    // create new pjsua
    pub fn new() -> Pjsua {
        let ret = unsafe { pjsua_create() };

        Pjsua { config: None, logging: None, media: None, status: ret }
    }

    // pub fn init() {}
}



fn main() {
    let pjsua = Pjsua::new();
    let mut config: pjsua_config = pjsua_config::new();
    let mut log_config: pjsua_logging_config = pjsua_logging_config::new();
    let mut media_config: pjsua_media_config = pjsua_media_config::new();
    unsafe {
        // let mut config: pjsua_config = pjsua_config::new();
        pjsua_config_default(&mut config as *mut _);
        pjsua_logging_config_default(&mut log_config as *mut _);
        pjsua_media_config_default(&mut media_config as *mut _);
    }
    // println!("app_config: {}", config.);
    assert_eq!(0, pjsua.status);

    println!("Hello, world!");
}
