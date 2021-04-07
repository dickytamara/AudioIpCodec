
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
}

impl SIPCall {

    /// create new SIPCall
    pub fn new() -> Self {
        SIPCall {
            id: -1,
        }
    }

    /// construct SIPCall from given call_id
    pub fn from(call_id: pjsua_call_id) -> Self {
        SIPCall {
            id: call_id,
        }
    }

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

        if let Err(e) = pjsua::call_get_info(self.id, &mut info) {
            return Err(e);
        }

        Ok(info)
    }

    pub fn remote_has_cap(&self, htype: i32, hname: String, token: String) -> pjsip_dialog_cap_status {
        pjsua::call_remote_has_cap(self.id, htype, hname, token)
    }

    pub fn get_rem_nat_type(&self, p_type: &mut pj_stun_nat_type) {
        pjsua::call_get_rem_nat_type( self.id, p_type)
        .expect("SIPCall::pjsua_call_get_rem_nat_type");
    }

    pub fn answer(&self, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_answer( self.id, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_answer");
    }

    pub fn answer2(&self,
        opt: &mut pjsua_call_setting,
        code: u32,
        reason: Option<String>,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_answer2( self.id, opt, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_answer2");
    }

    pub fn answer_with_sdp (&self,
        sdp: &mut pjmedia_sdp_session,
        opt: &mut pjsua_call_setting,
        code: u32,
        reason: Option<String>,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_answer_with_sdp(self.id, sdp, opt, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_answer_with_sdp");
    }

    pub fn hangup(&self, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) {

        pjsua::call_hangup (self.id, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_hangup");
    }

    pub fn process_redirect(&self, cmd: pjsip_redirect_op) {
        pjsua::call_process_redirect( self.id, cmd)
        .expect("SIPCall::pjsua_call_process_redirect");
    }

    pub fn set_hold(&self, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_set_hold(self.id, msg_data)
        .expect("SIPCall::pjsua_call_set_hold");
    }

    pub fn set_hold2(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_set_hold2(self.id, options, msg_data)
        .expect("SIPCall::pjsua_call_set_hold2");
    }

    pub fn reinvite(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_reinvite(self.id, options, msg_data)
        .expect("SIPCall::pjsua_call_reinvite")
    }

    pub fn reinvite2(&self, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_reinvite2(self.id, opt, msg_data)
        .expect("SIPCall::pjsua_call_reinvite2");
    }

    pub fn update(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_update(self.id, options, msg_data)
        .expect("SIPCall::pjsua_call_update");
    }

    pub fn update2(&self, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_update2(self.id, opt, msg_data)
        .expect("SIPCall::pjsua_call_udpate2");
    }

    pub fn xfer(&self, dest: String, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_xfer(self.id, dest, msg_data)
        .expect("SIPCall::pjsua_call_xfer");
    }

    pub fn xfer_replaces(&self,
        dest_call_id: pjsua_call_id,
        options: u32,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_xfer_replaces(self.id, dest_call_id, options, msg_data)
        .expect("SIPCall::pjsua_call_exfer_replaces");
    }

    pub fn dial_dtmf(&self, digits: String) {
        pjsua::call_dial_dtmf(self.id, digits)
        .expect("SIPCall::pjsua_call_dial_dtmf");
    }

    pub fn send_dtmf(&self, param: &mut pjsua_call_send_dtmf_param) {
        pjsua::call_send_dtmf(self.id, param)
        .expect("SIPCall::pjsua_call_send_dtmf");
    }

    pub fn send_im(&self,
        mime_type: String,
        content: String,
        msg_data: Option<&mut pjsua_msg_data>,
    ) {
        pjsua::call_send_im(self.id, mime_type, content, msg_data)
        .expect("SIPCall::pjsua_call_send_im");
    }

    pub fn send_typing_ind(&self,
        is_typing: bool,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_send_typing_ind( self.id, is_typing, msg_data)
        .expect("SIPCall::pjsua_call_snd_typing_ind");
    }

    pub fn send_request(&self, method: String, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_send_request(self.id, method, msg_data)
        .expect("SIPCall:pjsua_send_request");
    }

    pub fn get_stream_info(&self, med_idx: u32) -> Result<pjsua_stream_info, i32> {

        let mut info = pjsua_stream_info::new();

        if let Err(e) = pjsua::call_get_stream_info(self.id, med_idx, &mut info) {
            return Err(e);
        }

        Ok(info)
    }

    pub fn get_stream_stat(&self, med_idx: u32) -> Result<pjsua_stream_stat, i32> {

        let mut stat = pjsua_stream_stat::new();

        if let Err(e)= pjsua::call_get_stream_stat(self.id, med_idx, &mut stat) {
            return Err(e);
        }

        Ok(stat)
    }

    pub fn get_med_transport_info(&self, med_idx: u32) -> Result<pjmedia_transport_info, i32> {

        let mut info = pjmedia_transport_info::new();

        if let Err(e) = pjsua::call_get_med_transport_info(self.id, med_idx, &mut info) {
            return Err(e);
        }

        Ok(info)
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

    pub fn new() -> Self {
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

    pub fn enum_calls(&self) -> Vec<SIPCall> {

        let mut ret: Vec<SIPCall> = Vec::new();

        let mut id:[pjsua_call_id; PJSUA_MAX_CALLS as usize] = [-1i32; PJSUA_MAX_CALLS as usize];
        let mut count = 32u32;

        pjsua::enum_calls(&mut id, &mut count).unwrap();

        for i in 0..count as usize {
            ret.push(SIPCall::from(id[i]));
        }

        ret
    }

    pub fn get_current_call(&self) -> Option<SIPCall> {
        if self.get_count() > 0 {

            let calls = self.enum_calls();
            calls.last().cloned()
        } else {
            None
        }
    }

    pub fn make_call(
        &self,
        acc_id: pjsua_acc_id,
        dst_uri: String,
        opt: Option<&mut pjsua_call_setting>,
        msg_data: Option<&mut pjsua_msg_data>,
        p_call_id: Option<&mut pjsua_call_id>
    ) {
        pjsua::call_make_call(acc_id, dst_uri, opt, msg_data, p_call_id).unwrap();
    }

}
