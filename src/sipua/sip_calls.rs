
use super::pj_sys::*;
use super::pjsip_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjlib::*;
use super::pjsip::*;
use super::pjsua::*;
use super::pjdefault::*;

use std::ptr;
use std::ffi::CString;
use super::pjsua;

#[derive(Copy, Clone)]
pub struct SIPCall {
    id: pjsua_call_id,
    ringback_on: bool,
    ring_on: bool
}

impl SIPCall {

    pub fn new() -> SIPCall {
        SIPCall {
            id: -1,
            ringback_on: false,
            ring_on: false
        }
    }

    pub fn from(call_id: pjsua_call_id) -> SIPCall {
        SIPCall {
            id: call_id,
            ringback_on: false,
            ring_on: false
        }
    }

    // skiped procedure
    // this was for video call
    // pjsua_vid_win_id pjsua_call_get_vid_win 	( pjsua_call_id call_id	)
    // pjsua_conf_port_id pjsua_call_get_vid_conf_port 	( pjsua_call_id call_id, pjmedia_dir dir)

    // skiped procedure
    // this for avatar or something related to user data. mostly used by im implementation
    // call_set_user_data()
    // call_get_user_data()

    pub fn is_active(&self) -> bool {
        pjsua::call_is_active(self.id)
    }

    pub fn has_media(&self) -> bool {
        pjsua::call_has_media(self.id)
    }

    pub fn get_conf_port(&self) -> i32 {
        pjsua::call_get_conf_port(self.id)
    }

    pub fn get_info(&self) -> Result<pjsua_call_info, i32> {

        let mut info = pjsua_call_info::new();

        let status = pjsua::call_get_info(
            self.id,
            &mut info
        );

        if status == PJ_SUCCESS as pj_status_t {
            Ok(info)
        } else {
            println!("ERR cant get call info");
            Err(status)
        }
    }

    pub fn remote_has_cap(&self, htype: i32, hname: String, token: String) -> pjsip_dialog_cap_status {

        pjsua::call_remote_has_cap(
            self.id,
            htype,
            hname,
            token
        )
    }

    pub fn get_rem_nat_type(&self, p_type: &mut pj_stun_nat_type) {

        let status = pjsua::call_get_rem_nat_type(
                self.id,
                p_type
            );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR cant get remote nat type.");
        }
    }

    pub fn answer(&self, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_answer(
            self.id,
            code,
            reason,
            msg_data);

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't answer call");
        }
    }

    pub fn answer2(&self,
        opt: &mut pjsua_call_setting,
        code: u32,
        reason: Option<String>,
        msg_data: Option<&mut pjsua_msg_data>
    ) {

        let status = pjsua::call_answer2(
                self.id,
                opt,
                code,
                reason,
                msg_data
            );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't answer2 call");
        }

    }

    pub fn answer_with_sdp (&self,
        sdp: &mut pjmedia_sdp_session,
        opt: &mut pjsua_call_setting,
        code: u32,
        reason: Option<String>,
        msg_data: Option<&mut pjsua_msg_data>
    ) {

        let status = pjsua::call_answer_with_sdp(
            self.id,
            sdp,
            opt,
            code,
            reason,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't answer call with sdp.");
        }
    }

    pub fn hangup(&self, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_hangup (
                self.id,
                code,
                reason,
                msg_data
            );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't hangup calls");
        }
    }

    pub fn process_redirect(&self, cmd: pjsip_redirect_op) {

        let status = pjsua::call_process_redirect(
                self.id,
                cmd
            );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't process redirect.")
        }
    }

    pub fn set_hold(&self, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_set_hold(
                self.id,
                msg_data
            );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't set hold a call.");
        }
    }

    pub fn set_hold2(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_set_hold2(
            self.id,
            options,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't set hold2 a call");
        }
    }

    pub fn reinvite(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_reinvite(
            self.id,
            options,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't reinvite call.");
        }
    }

    pub fn reinvite2(&self, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_reinvite2(
            self.id,
            opt,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't reinvite2 a call.");
        }
    }

    pub fn update(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_update(
            self.id,
            options,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't update calls.");
        }
    }

    pub fn update2(&self, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_update2(
            self.id,
            opt,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't update2 calls.");
        }
    }

    pub fn xfer(&self, dest: String, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_xfer(
            self.id,
            dest,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't xfer for call.");
        }
    }

    pub fn xfer_replaces(&self,
        dest_call_id: pjsua_call_id,
        options: u32,
        msg_data: Option<&mut pjsua_msg_data>
    ) {

        let status = pjsua::call_xfer_replaces(
            self.id,
            dest_call_id,
            options,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't replaces xfer.");
        }
    }

    pub fn dial_dtmf(&self, digits: String) {

        let status = pjsua::call_dial_dtmf(
            self.id,
            digits
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't dial DTMF.");
        }
    }

    pub fn send_dtmf(&self, param: &mut pjsua_call_send_dtmf_param) {

        let status = pjsua::call_send_dtmf(
            self.id,
            param
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't send DTMF.");
        }
    }

    pub fn send_im(&self,
        mime_type: String,
        content: String,
        msg_data: Option<&mut pjsua_msg_data>,
    ) {

        let status = pjsua::call_send_im(
            self.id,
            mime_type,
            content,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't send im for this call.");
        }
    }

    pub fn send_typing_ind(&self,
        is_typing: bool,
        msg_data: Option<&mut pjsua_msg_data>
    ) {

        let status = pjsua::call_send_typing_ind(
            self.id,
            is_typing,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't send typing indication.");
        }

    }

    pub fn send_request(&self, method: String, msg_data: Option<&mut pjsua_msg_data>) {

        let status = pjsua::call_send_request(
            self.id,
            method,
            msg_data
        );

        if status != PJ_SUCCESS as pj_status_t {
            println!("ERR can't send request for this call.");
        }
    }

    pub fn get_stream_info(&self, med_idx: u32) -> Result<pjsua_stream_info, i32> {

        let mut info = pjsua_stream_info::new();

        let status = pjsua::call_get_stream_info(
            self.id,
            med_idx,
            &mut info
        );

        if status == PJ_SUCCESS as pj_status_t {
            Ok(info)
        } else {
            println!("ERR can't get stream info for call.");
            Err(status)
        }
    }

    pub fn get_stream_stat(&self, med_idx: u32) -> Result<pjsua_stream_stat, i32> {

        let mut stat = pjsua_stream_stat::new();

        let status = pjsua::call_get_stream_stat(
            self.id,
            med_idx,
            &mut stat
        );

        if status == PJ_SUCCESS as pj_status_t {
            Ok(stat)
        } else {
            println!("ERR can't get stream status for call.");
            Err(status)
        }
    }

    pub fn get_med_transport_info(&self, med_idx: u32) -> Result<pjmedia_transport_info, i32> {

        let mut info = pjmedia_transport_info::new();


        let status = pjsua::call_get_med_transport_info(
            self.id,
            med_idx,
            &mut info
        );

        if status == PJ_SUCCESS as pj_status_t {
            Ok(info)
        } else {
            println!("ERR can't get media transport info for call");
            Err(status)
        }
    }


}

#[derive(Clone, Copy)]
pub struct SIPCalls {
    id_list: [pjsua_call_id; PJSUA_MAX_CALLS as usize],
    call_data: [SIPCall; PJSUA_MAX_CALLS as usize],
    call_opt: pjsua_call_setting,
    ringback_on: bool,
    ring_on: bool,
}

// skiped procedure
// void 	pjsua_call_setting_default (pjsua_call_setting *opt)
// void 	pjsua_call_vid_strm_op_param_default (pjsua_call_vid_strm_op_param *param)
// void 	pjsua_call_send_dtmf_param_default (pjsua_call_send_dtmf_param *param)
// pj_status_t pjsua_call_make_call 	( 	pjsua_acc_id  	acc_id,


impl SIPCalls {

    pub fn new() -> SIPCalls {
        SIPCalls {
            id_list: [0; PJSUA_MAX_CALLS as usize],
            call_data: [SIPCall::new(); PJSUA_MAX_CALLS as usize],
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

    pub fn setting_default(&self, opt: &mut pjsua_call_setting) {
        pjsua::call_setting_default(opt);
    }

    pub fn get_max_count(&self) -> u32 {
        pjsua::call_get_max_count()
    }

    pub fn get_count(&self) -> u32 {
        pjsua::call_get_count()
    }

    pub fn hangup_all(&self) {
        pjsua::call_hangup_all();
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
        let mut hname = pj_str_t::from_string(String::from("Warning"));
        let mut hvalue = pj_str_t::from_string(String::from("339 \" Call duration exceeded \""));

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
