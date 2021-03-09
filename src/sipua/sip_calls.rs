

use super::pj_sys::*;
use super::pjsip_sys::*;
use super::pjsua_sys::*;

use super::pjlib::*;
use super::pjsip::*;
use super::pjsua::*;
use super::pjdefault::*;

use std::ptr;
use std::ffi::CString;

#[derive(Copy, Clone)]
pub struct SIPCall {
    timer: pj_timer_entry,
    ringback_on: bool,
    ring_on: bool
}

impl SIPCall {
    pub fn new() -> SIPCall {
        SIPCall {
            timer: pj_timer_entry::new(),
            ringback_on: false,
            ring_on: false
        }
    }
}

pub struct SIPCalls {
    id_list: [pjsua_call_id; 32],
    call_data: [SIPCall; 32],
    call_opt: pjsua_call_setting,
    ringback_on: bool,
    ring_on: bool,
}

impl SIPCalls {

    pub fn new() -> SIPCalls {
        SIPCalls {
            id_list: [0;32],
            call_data: [SIPCall::new(); 32],
            call_opt: pjsua_call_setting::new(),
            ringback_on: false,
            ring_on: false,
        }
    }

    pub fn set_audio_count(&mut self, value: u32) {
        self.call_opt.aud_cnt = value;
    }

    pub fn get_call_opt(&self) -> pjsua_call_setting {
        self.call_opt.clone()
    }
}


impl PjTimerEntry for SIPCalls {

    unsafe extern "C" fn pj_timer_heap_callback(
        timer_heap: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
    ) {
        let call_id: pjsua_call_id = (*entry).id;
        let mut msg_data_: pjsua_msg_data = pjsua_msg_data::new();
        let mut warn: pjsip_generic_string_hdr = pjsip_generic_string_hdr::new();
        let mut hname = pj_str(CString::new("Warning").expect("Error").into_raw());
        let mut hvalue = pj_str(
            CString::new("339 \" Call duration exceeded \"")
                .expect("Error")
                .into_raw(),
        );

        if call_id == PJSUA_INVALID_ID {
            println!("Invalid call ED intimer callback");
        }

        pjsua_msg_data_init(&mut msg_data_ as *mut _);
        pjsip_generic_string_hdr_init2(
            &mut warn as *mut _,
            &mut hname as *mut _,
            &mut hvalue as *mut _,
        );
        pj_list_insert_before(
            (&mut msg_data_.hdr_list as *mut _) as *mut _,
            (&mut warn as *mut _) as *mut _,
        );

        println!(
            "Duration (seconds) has been exceeded for call {}, disconnectiong the call.",
            call_id
        );

        (*entry).id = PJSUA_INVALID_ID;
        pjsua_call_hangup(call_id, 200, ptr::null(), &mut msg_data_ as *mut _);
    }
}
