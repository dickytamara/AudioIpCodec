
use pj_sys::*;
use pjsip_sys::*;
use pjmedia_sys::*;
use pjsua_sys::*;

use crate::pjproject::pjdefault::{self, AutoCreate, FromString};
use crate::pjproject::pjlib;
use crate::pjproject::pjsip;
use crate::pjproject::pjsua;

use std::ptr;
use std::ffi::CString;

#[derive(Copy, Clone)]
pub struct SIPCall {
    id: pjsua_call_id,
}


// void 	pjsua_call_setting_default (pjsua_call_setting *opt)
// void 	pjsua_call_vid_strm_op_param_default (pjsua_call_vid_strm_op_param *param)
// void 	pjsua_call_send_dtmf_param_default (pjsua_call_send_dtmf_param *param)
// unsigned 	pjsua_call_get_max_count (void)
// unsigned 	pjsua_call_get_count (void)
// pj_status_t 	pjsua_enum_calls (pjsua_call_id ids[], unsigned *count)




// pjsua_vid_win_id 	pjsua_call_get_vid_win (pjsua_call_id call_id)
// pjsua_conf_port_id 	pjsua_call_get_vid_conf_port (pjsua_call_id call_id, pjmedia_dir dir)





// pj_status_t 	pjsua_call_set_user_data (pjsua_call_id call_id, void *user_data)
// void * 	pjsua_call_get_user_data (pjsua_call_id call_id)




















// void 	pjsua_call_hangup_all (void)

// int 	pjsua_call_get_vid_stream_idx (pjsua_call_id call_id)
// pj_bool_t 	pjsua_call_vid_stream_is_running (pjsua_call_id call_id, int med_idx, pjmedia_dir dir)
// pj_status_t 	pjsua_call_set_vid_strm (pjsua_call_id call_id, pjsua_call_vid_strm_op op, const pjsua_call_vid_strm_op_param *param)





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
    // pj_bool_t 	pjsua_call_is_active (pjsua_call_id call_id)
    /// Check if the specified call has active INVITE session and the INVITE session has not been disconnected.
    pub fn is_active(&self) -> bool {
        pjsua::call_is_active(self.id)
    }
 
    /// Check if call has an active media session.
    pub fn has_media(&self) -> bool {
        pjsua::call_has_media(self.id)
    }
 
    /// Get the conference port identification associated with the call.
    pub fn get_conf_port(&self) -> i32 {
        pjsua::call_get_conf_port(self.id)
    }
 
    /// Obtain detail information about the specified call.
    pub fn get_info(&self) -> Result<pjsua_call_info, i32> {

        let mut info = pjsua_call_info::new();

        if let Err(e) = pjsua::call_get_info(self.id, &mut info) {
            return Err(e);
        }

        Ok(info)
    }
 
    /// Check if remote peer support the specified capability.
    pub fn remote_has_cap(&self, htype: i32, hname: String, token: String) -> pjsip_dialog_cap_status {
        pjsua::call_remote_has_cap(self.id, htype, hname, token)
    }
 
    /// Get the NAT type of remote's endpoint. This is a proprietary feature of PJSUA-LIB which
    /// sends its NAT type in the SDP when nat_type_in_sdp is set in pjsua_config.
    ///
    /// This function can only be called after SDP has been received from remote, which means
    /// for incoming call, this function can be called as soon as call is received as long as
    /// incoming call contains SDP, and for outgoing call, this function can be called only after
    /// SDP is received (normally in 200/OK response to INVITE). As a general case, application
    /// should call this function after or in on_call_media_state() callback.
    pub fn get_rem_nat_type(&self, p_type: &mut pj_stun_nat_type) {
        pjsua::call_get_rem_nat_type( self.id, p_type)
        .expect("SIPCall::pjsua_call_get_rem_nat_type");
    }
 
    /// Send response to incoming INVITE request. Depending on the status code specified as
    /// parameter, this function may send provisional response, establish the call, or
    /// terminate the call. See also pjsua_call_answer2().
    pub fn answer(&self, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_answer( self.id, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_answer");
    }
 
    /// Send response to incoming INVITE request with call setting param. Depending on the status
    /// code specified as parameter, this function may send provisional response, establish the
    /// call, or terminate the call. Notes about call setting:
    ///
    /// - if call setting is changed in the subsequent call to this function, only the first call
    ///   setting supplied will applied. So normally application will not supply call setting
    ///   before getting confirmation from the user.
    /// - if no call setting is supplied when SDP has to be sent, i.e: answer with status code 183
    ///   or 2xx, the default call setting will be used, check pjsua_call_setting for its default
    ///   values.
    pub fn answer2(&self,
        opt: &mut pjsua_call_setting,
        code: u32,
        reason: Option<String>,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_answer2( self.id, opt, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_answer2");
    }
 
    /// Same as pjsua_call_answer2() but this function will set the SDP answer first before sending
    /// the response.
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
 
    /// Hangup call by using method that is appropriate according to the call state. This function
    /// is different than answering the call with 3xx-6xx response (with pjsua_call_answer()), in
    /// that this function will hangup the call regardless of the state and role of the call,
    /// while pjsua_call_answer() only works with incoming calls on EARLY state.
    pub fn hangup(&self, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_hangup (self.id, code, reason, msg_data)
        .expect("SIPCall::pjsua_call_hangup");
    }
 
    /// Accept or reject redirection response. Application MUST call this function after it signaled
    /// PJSIP_REDIRECT_PENDING in the on_call_redirected() callback, to notify the call whether to
    /// accept or reject the redirection to the current target. Application can use the combination
    /// of PJSIP_REDIRECT_PENDING command in on_call_redirected() callback and this function to ask
    /// for user permission before redirecting the call.
    ///
    /// Note that if the application chooses to reject or stop redirection (by using PJSIP_REDIRECT_REJECT
    /// or PJSIP_REDIRECT_STOP respectively), the call disconnection callback will be called before
    /// this function returns. And if the application rejects the target, the on_call_redirected()
    /// callback may also be called before this function returns if there is another target to try.
    pub fn process_redirect(&self, cmd: pjsip_redirect_op) {
        pjsua::call_process_redirect( self.id, cmd)
        .expect("SIPCall::pjsua_call_process_redirect");
    }
 
    /// Put the specified call on hold. This will send re-INVITE with the appropriate SDP to inform
    /// remote that the call is being put on hold. The final status of the request itself will be
    /// reported on the on_call_media_state() callback, which inform the application that the media
    /// state of the call has changed.
    pub fn set_hold(&self, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_set_hold(self.id, msg_data)
        .expect("SIPCall::pjsua_call_set_hold");
    }
 
    /// Put the specified call on hold. This will send re-INVITE with the appropriate SDP to inform
    /// remote that the call is being put on hold. The final status of the request itself will be
    /// reported on the on_call_media_state() callback, which inform the application that the media
    /// state of the call has changed.
    pub fn set_hold2(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_set_hold2(self.id, options, msg_data)
        .expect("SIPCall::pjsua_call_set_hold2");
    }
 
    /// Send re-INVITE request or release hold. The final status of the request itself will be
    /// reported on the on_call_media_state() callback, which inform the application that the media
    /// state of the call has changed.
    pub fn reinvite(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_reinvite(self.id, options, msg_data)
        .expect("SIPCall::pjsua_call_reinvite")
    }
 
    /// Send re-INVITE request or release hold. The final status of the request itself will be
    /// reported on the on_call_media_state() callback, which inform the application that the
    /// media state of the call has changed.
    pub fn reinvite2(&self, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_reinvite2(self.id, opt, msg_data)
        .expect("SIPCall::pjsua_call_reinvite2");
    }
 
    /// Send UPDATE request.
    pub fn update(&self, options: u32, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_update(self.id, options, msg_data)
        .expect("SIPCall::pjsua_call_update");
    }
 
    /// Send UPDATE request.
    pub fn update2(&self, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_update2(self.id, opt, msg_data)
        .expect("SIPCall::pjsua_call_udpate2");
    }
 
    /// Initiate call transfer to the specified address. This function will send REFER request to
    /// instruct remote call party to initiate a new INVITE session to the specified
    /// destination/target.
    ///
    /// If application is interested to monitor the successfulness and the progress of the transfer
    /// request, it can implement on_call_transfer_status() callback which will report the progress
    /// of the call transfer request.
    pub fn xfer(&self, dest: String, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_xfer(self.id, dest, msg_data)
        .expect("SIPCall::pjsua_call_xfer");
    }
 
    /// Initiate attended call transfer. This function will send REFER request to instruct remote
    /// call party to initiate new INVITE session to the URL of dest_call_id. The party at dest_call_id
    /// then should "replace" the call with us with the new call from the REFER recipient.
    pub fn xfer_replaces(&self,
        dest_call_id: pjsua_call_id,
        options: u32,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_xfer_replaces(self.id, dest_call_id, options, msg_data)
        .expect("SIPCall::pjsua_call_exfer_replaces");
    }
 
    /// Send DTMF digits to remote using RFC 2833 payload formats. Use pjsua_call_send_dtmf() to send
    /// DTMF using SIP INFO or other method in pjsua_dtmf_method. App can use on_dtmf_digit() or
    /// on_dtmf_digit2() callback to monitor incoming DTMF.
    pub fn dial_dtmf(&self, digits: String) {
        pjsua::call_dial_dtmf(self.id, digits)
        .expect("SIPCall::pjsua_call_dial_dtmf");
    }
 
    /// Send DTMF digits to remote. Use this method to send DTMF using the method in pjsua_dtmf_method.
    /// This method will call pjsua_call_dial_dtmf() when sending DTMF using PJSUA_DTMF_METHOD_RFC2833.
    /// Note that on_dtmf_digit() callback can only monitor incoming DTMF using RFC 2833. App can use
    /// on_dtmf_digit2() to monitor incoming DTMF using the method in pjsua_dtmf_method. Note that
    /// on_dtmf_digit() will not be called once on_dtmf_digit2() is implemented.
    pub fn send_dtmf(&self, param: &mut pjsua_call_send_dtmf_param) {
        pjsua::call_send_dtmf(self.id, param)
        .expect("SIPCall::pjsua_call_send_dtmf");
    }
 
    /// Send instant messaging inside INVITE session.
    pub fn send_im(&self,
        mime_type: String,
        content: String,
        msg_data: Option<&mut pjsua_msg_data>,
    ) {
        pjsua::call_send_im(self.id, mime_type, content, msg_data)
        .expect("SIPCall::pjsua_call_send_im");
    }
 
    /// Send IM typing indication inside INVITE session.
    pub fn send_typing_ind(&self,
        is_typing: bool,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::call_send_typing_ind( self.id, is_typing, msg_data)
        .expect("SIPCall::pjsua_call_snd_typing_ind");
    }
 
    /// Send arbitrary request with the call. This is useful for example to send INFO request. Note that
    /// application should not use this function to send requests which would change the invite session's
    /// state, such as re-INVITE, UPDATE, PRACK, and BYE.
    pub fn send_request(&self, method: String, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::call_send_request(self.id, method, msg_data)
        .expect("SIPCall:pjsua_send_request");
    }
 
    /// Dump call and media statistics to string.
    pub fn dump(&self, with_media: bool, buffer: String, maxlen: u32, indent: String) {
        pjsua::call_dump(self.id, with_media, buffer, maxlen, indent)
        .expect("SIPCall::pjsua_call_dump");
    }
 
    /// Get media stream info for the specified media index.
    pub fn get_stream_info(&self, med_idx: u32) -> Result<pjsua_stream_info, i32> {

        let mut info = pjsua_stream_info::new();

        if let Err(e) = pjsua::call_get_stream_info(self.id, med_idx, &mut info) {
            return Err(e);
        }

        Ok(info)
    }
 
    /// Get media stream statistic for the specified media index.
    pub fn get_stream_stat(&self, med_idx: u32) -> Result<pjsua_stream_stat, i32> {

        let mut stat = pjsua_stream_stat::new();

        if let Err(e)= pjsua::call_get_stream_stat(self.id, med_idx, &mut stat) {
            return Err(e);
        }

        Ok(stat)
    }
 
    /// Get media transport info for the specified media index.
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
    call_opt: pjsua_call_setting,
}

// skiped procedure
// void 	pjsua_call_setting_default (pjsua_call_setting *opt)
// void 	pjsua_call_vid_strm_op_param_default (pjsua_call_vid_strm_op_param *param)
// void 	pjsua_call_send_dtmf_param_default (pjsua_call_send_dtmf_param *param)
// pj_status_t pjsua_call_make_call 	( 	pjsua_acc_id  	acc_id,

impl SIPCalls {

    pub fn new() -> Self {
        SIPCalls {
            call_opt: pjsua_call_setting::new(),
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

        let mut id= [-1i32; PJSUA_MAX_CALLS as usize];
        let mut count = 32u32;

        pjsua::enum_calls(&mut id, &mut count)
        .expect("SIPCalls::pjsua_enum_calls");

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

    // pj_status_t 	pjsua_call_make_call (pjsua_acc_id acc_id, const pj_str_t *dst_uri, const pjsua_call_setting *opt, void *user_data, const pjsua_msg_data *msg_data, pjsua_call_id *p_call_id)
    pub fn make_call(
        &self,
        acc_id: pjsua_acc_id,
        dst_uri: String,
        opt: Option<&mut pjsua_call_setting>,
        msg_data: Option<&mut pjsua_msg_data>,
        p_call_id: Option<&mut pjsua_call_id>
    ) {
        pjsua::call_make_call(acc_id, dst_uri, opt, msg_data, p_call_id)
        .expect("SIPCalls::pjsua_call_make_call");
    }

}

