use super::sipcore::*;

use std::{cell::{RefCell, RefMut}, rc::Rc};
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
            SIP_CORE = Some(Rc::new(RefCell::new(SIPCore::new())));
        }

        SIPUserAgent {}
    }

    /// start user agent
    pub fn start(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow_mut().init();
        }
    }

    pub fn restart(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow_mut().restart();
        }
    }

    /// get output device list
    // pub fn get_output_device_list(&self) -> Vec<String> {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref mut sipcore) => {
    //                 sipcore.media_config.get_output_device_list()
    //             },
    //             _ => panic!("")
    //         }
    //     }
    // }

    // pub fn get_input_device_list(&self) -> Vec<String> {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref mut sipcore ) => {
    //                 sipcore.media_config.get_input_device_list()
    //             },
    //             _ => panic!("")
    //         }
    //     }
    // }

    pub fn call_answer(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow().call_answer();
        }
    }

    pub fn call_hangup(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow().call_hangup();
        }
    }

    // User Agent settings part
    pub fn set_autoanswer(&self, value: bool) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow_mut().auto_answer(value);
        }
    }

    pub fn set_no_refersub(&self, value: bool) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow_mut().set_no_refersub(value);
        }
    }

    pub fn set_compact_form(&self, value: bool) {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow_mut().set_compact_form(value);
        }
    }


    pub fn get_context(&self) -> RefMut<SIPCore> {
        unsafe {
            SIP_CORE.as_ref().unwrap().borrow_mut()
        }
    }


    // get input port 0 level
    // pub fn get_input_level(&self) -> i32 {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref sipua) => {
    //                 sipua.media_config.get_input_level()
    //             },
    //             _ => { 0 }
    //         }
    //     }
    // }

    // get output port 0 level
    // pub fn get_output_level(&self) -> i32 {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref sipua) => {
    //                 sipua.media_config.get_output_level()
    //             },
    //             _ => { 0 }
    //         }
    //     }
    // }

    // set input port 0 level
    // pub fn set_input_level(&self, value: i32) {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref mut sipua) => {
    //                 sipua.media_config.set_input_level(value);
    //             },
    //             _ => panic!("")
    //         }
    //     }
    // }

    // set output port 0 level
    // pub fn set_output_level(&self, value: i32) {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref mut sipua) => {
    //                 sipua.media_config.set_output_level(value);
    //             },
    //             _ => panic!("")
    //         }
    //     }
    // }

    // pub fn input_mute(&self, is_mute: bool) {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref sipua) => sipua.media_config.input_mute(is_mute),
    //             _ => panic!("")
    //         }
    //     }
    // }

    // pub fn output_mute(&self, is_mute: bool) {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref sipua) => sipua.media_config.output_mute(is_mute),
    //             _ => panic!("")
    //         }
    //     }
    // }

    // port 0 signal level
    // pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
    //     unsafe {
    //         match SIP_CORE {
    //             Some(ref sipua) => {
    //                 sipua.media_config.get_signal_level()
    //             },
    //             _ => (0,0,0,0)
    //         }
    //     }
    // }

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

