#![allow(dead_code, unused_variables, non_upper_case_globals)]

pub mod prelude;
// pub mod sip_account;
// pub mod sip_buddy;
// pub mod sip_calls;
// pub mod sip_codec;
pub mod sip_core;
// pub mod sip_log;
// pub mod sip_media;
// pub mod sip_module;
// pub mod sip_presence;
// pub mod sip_tones;
// pub mod sip_transport;
pub mod sip_ua;
// pub mod sip_wav;

use prelude::*;
use sip_core::{SIPCore, SIPCoreEventsExt, SIP_CORE};
use std::ops::Drop;


#[derive(Clone, Copy)]
pub enum SIPInviteState {
    Null,
    Calling,
    Incoming,
    Early,
    Connecting,
    Confirmed,
    Disconnected,
    Unknown
}


// SIPUserAgent
#[derive(Clone)]
pub struct SIPUserAgent {}


impl SIPUserAgent {
    // create sip user sip user agent with default ivalue
    pub fn new() -> SIPUserAgent {
        unsafe {
            SIP_CORE = Some(SIPCore::new());
        }

        SIPUserAgent {}
    }

    /// start user agent
    pub fn start(&self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => sipcore.init(),
                _ => panic!(""),
            }
        }
    }

    pub fn restart(&self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => sipcore.restart(),
                _ => panic!(""),
            }
        }
    }

    /// get output device list
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
                Some(ref sipua) => sipua.call(call_addr),
                _ => panic!("")
            }
        }
    }

    pub fn call_answer(&self) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => sipua.call_answer(),
                _ => panic!("")
            }
        }
    }

    pub fn call_hangup(&self) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => sipua.call_hangup(),
                _ => panic!("")
            }
        }
    }

    // User Agent settings part
    pub fn set_autoanswer(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => sipua.auto_answer(value),
                _ => panic!("")
            }
        }
    }

    pub fn set_no_refersub(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_no_refersub(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_no_forcelr(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_no_forcelr(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_compact_form(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_compact_form(value);
                },
                None => panic!("")
            }
        }
    }

    /// set default stun server
    pub fn set_stun_server(&self, value: Vec<SIPStunServerData>) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.ua_config.set_stun_srv(value)
                    .expect("SIPUserAgent:: Set stun server failed.");
                },
                None => panic!("")
            }
        }
    }

    /// set default outbound proxy
    pub fn set_outbound_proxy(&self, value: Vec<SIPOutboundProxyServerData>) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.ua_config.set_outbound_proxy(value)
                    .expect("SIPUserAgent:: Set outbound proxy server failed.")
                },
                None => panic!("")
            }
        }
    }

    /// set default nameserver DNS
    pub fn set_nameserver(&self, value: Vec<String>) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.ua_config.set_nameserver(value)
                    .expect("SIPUserAgent:: Set nameserver failed.")
                },
                None => panic!("")
            }
        }
    }

    // turn settings part
    pub fn set_use_turn(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_enable_turn(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_turn_conn_type(&self, value: u32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_turn_conn_type(value);
                },
                None => panic!("")
            }
        }
    } 

    pub fn set_turn_server(&self, value: SIPTurnServerData) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_turn_server(value);
                },
                None => panic!("")
            }
        }
    }


    // ICE Settings part
    pub fn set_use_ice(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_enable_ice(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_no_rtcp(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_ice_no_rtcp(value);
                },
                None => panic!("")
            }
        }
    }

    // set deprecated for this almost ice
    // settings wtih aggresive config
    pub fn set_aggressive_nomination(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    todo!();
                },
                None => panic!("")
            }
        }
    }

    // deprecated
    pub fn set_trickle_method(&self, value: u32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    todo!();
                },
                None => panic!("")
            }
        }
    }

    pub fn set_ice_max_host_cands(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_ice_max_host_cands(value);
                },
                None => panic!("")
            }
        }
    }


    // audio Settings part
    pub fn set_jb_max(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_jb_max(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_ptime(&self, value: u32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_ptime(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_quality(&self, value: u32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_quality(value)
                },
                None => panic!("")
            }
        }
    }

    pub fn set_no_vad(&self, value: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_no_vad(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_ec_tail_len(&self, value: u32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_ec_tail_len(value);
                },
                None => panic!("")
            }
        }
    }

    pub fn set_ec_options(&self, value: u32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_ec_options(value);
                },
                None => panic!("")
            }
        }
    }

    /// get input port 0 level
    pub fn get_input_level(&self) -> i32 {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.media_config.get_input_level()
                },
                _ => { 0 }
            }
        }
    }

    /// get output port 0 level
    pub fn get_output_level(&self) -> i32 {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.media_config.get_output_level()
                },
                _ => { 0 }
            }
        }
    }

    /// set input port 0 level
    pub fn set_input_level(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_input_level(value);
                },
                _ => panic!("")
            }
        }
    }

    /// set ouput port 0 level
    pub fn set_output_level(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.media_config.set_output_level(value);
                },
                _ => panic!("")
            }
        }
    }

    pub fn input_mute(&self, is_mute: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => sipua.media_config.input_mute(is_mute),
                _ => panic!("")
            }
        }
    }

    pub fn output_mute(&self, is_mute: bool) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => sipua.media_config.output_mute(is_mute),
                _ => panic!("")
            }
        }
    }

    /// port 0 signal level
    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.media_config.get_signal_level()
                },
                _ => (0,0,0,0)
            }
        }
    }

    pub fn connect_invite<F> (&self, f: F)
    where
        F: FnMut(SIPInviteState) + 'static
    {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.connect_invite(f)
                },
                _ => panic!("")
            }
        }
    }

    pub fn connect_incoming_call<F> (&self, f: F)
    where
        F: Fn() + 'static
    {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.connect_incoming_call(f)
                },
                _ => panic!("")
            }
        }
    }
}


impl Drop for SIPUserAgent {
    fn drop(&mut self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => {
                    sipcore.stop();
                }
                _ => (),
            }
        }
    }
}

