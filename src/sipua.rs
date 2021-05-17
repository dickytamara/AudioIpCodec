use pjproject::pjsua::UALoggingConfig;

use super::sipcore::*;

use std::cell::RefMut;
// use std::ops::Drop;


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
            SIP_CORE.as_ref().unwrap().init();
        }
    }

    pub fn restart(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().restart();
        }
    }

    pub fn call_answer(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().call_answer();
        }
    }

    pub fn call_hangup(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().call_hangup();
        }
    }

    // User Agent settings part
    // pub fn set_autoanswer(&self, value: bool) {
    //     unsafe {
    //         SIP_CORE.as_ref().unwrap().auto_answer(value);
    //     }
    // }

    // pub fn set_no_refersub(&self, value: bool) {
    //     unsafe {
    //         SIP_CORE.as_ref().unwrap().set_no_refersub(value);
    //     }
    // }

    // pub fn set_compact_form(&self, value: bool) {
    //     unsafe {
    //         SIP_CORE.as_ref().unwrap().set_compact_form(value);
    //     }
    // }


    pub fn get_context(&self) -> &SIPCore {
        unsafe {
            SIP_CORE.as_ref().unwrap()
        }
    }

    pub fn log_config(&self) -> RefMut<UALoggingConfig> {
        unsafe {
            SIP_CORE.as_ref().unwrap().log_config.borrow_mut()
        }
    }




    // pub fn connect_invite<F> (&self, f: F)
    // where
    //     F: FnMut(SIPInviteState) + 'static
    // {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref mut sipua) => {
    //                 sipua.connect_invite(f)
    //             },
    //             _ => panic!("")
    //         }
    //     }
    // }

    // pub fn connect_incoming_call<F> (&self, f: F)
    // where
    //     F: Fn() + 'static
    // {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref mut sipua) => {
    //                 // sipua.connect_incoming_call(f)
    //             },
    //             _ => panic!("")
    //         }
    //     }
    // }
}



// impl Drop for SIPUserAgent {
//     fn drop(&mut self) {
//         unsafe {
//             SIP_CORE.borrow().unwrap().stop();
//         }
//     }
// }

