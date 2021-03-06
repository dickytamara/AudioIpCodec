#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]

extern crate pj_sys;
extern crate pjmedia_sys;
extern crate pjsip_simple_sys;
extern crate pjsip_sys;
extern crate pjsua_sys;

mod sip_account;
mod sip_buddy;
mod sip_calls;
mod sip_core;
mod sip_media;
mod sip_presence;
mod sip_tones;
mod sip_transport;
mod sip_wav;

use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsip_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;
use super::pjproject::*;

use super::pjproject;


use super::pjproject::pjdefault::{AutoCreate, ToString};
use super::pjproject::pjlib::PjTimerEntry;
use super::pjproject::pjsip::PjsipModuleCallback;
use super::pjproject::pjsua::PjsuaCallback;

use super::pjproject::pjlib::*;
use super::pjproject::pjmedia::*;
use super::pjproject::pjsip::*;
use super::pjproject::pjsua::*;

use sip_core::*;

use std::ffi::CString;
use std::ffi::CStr;
use std::fmt::format;
use std::mem;
use std::ops::Drop;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

pub struct SIPIMessages {}





// SIPUserAgent
#[derive(Clone)]
pub struct SIPUserAgent {}



trait SIPUserAgentInternal {
    fn get_config(&mut self) -> &mut pjsua_config;
    fn get_log_config(&mut self) -> &mut pjsua_logging_config;
    fn get_media_config(&mut self) -> &mut pjsua_media_config;
}

impl SIPUserAgent {
    // create sip user sip user agent with default ivalue
    pub fn new() -> SIPUserAgent {
        unsafe {
            SIP_CORE = Some(SIPCore::new());
            CURRENT_CALL = Some(PJSUA_INVALID_ID);
        }

        SIPUserAgent {}
    }

    pub fn start(&self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => sipcore.start(),
                _ => panic!(""),
            }
        }
    }

    pub fn get_output_device_list(&self) -> Vec<String> {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => {
                    sipcore.media_config.get_output_device_list()
                },
                _ => panic!("")
            }
        }
    }

    pub fn get_input_device_list(&self) -> Vec<String> {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore ) => {
                    sipcore.media_config.get_input_device_list()
                },
                _ => panic!("")
            }
        }
    }

    pub fn call(&self, call_addr: &str){
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.call(call_addr);
                },
                _ => panic!("")
            }
        }
    }

    pub fn get_input_level(&self) -> i32 {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.get_input_level()
                },
                _ => { 0 }
            }
        }
    }

    pub fn get_output_level(&self) -> i32 {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.get_output_level()
                },
                _ => { 0 }
            }
        }
    }

    pub fn set_input_level(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_input_level(value);
                },
                _ => panic!("")
            }
        }
    }

    pub fn set_output_level(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_output_level(value);
                },
                _ => panic!("")
            }
        }
    }

    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.get_signal_level()
                },
                _ => (0,0,0,0)
            }
        }
    }

}

impl Drop for SIPUserAgent {
    fn drop(&mut self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => {
                    sipcore.deinit();
                }
                _ => (),
            }
        }
    }
}

