
use std::convert::TryFrom;
use crate::{pjmedia::{MediaDir, MediaType}, pjsip::{SIPRole, SIPStatusCode}, pjsip_ua::SIPInvState, utils::check_boolean};

use super::*;


// readonly
pub trait UACallMediaInfoExt {
    fn get_index(&self) -> u32;
    fn get_type_(&self) -> MediaType;
    fn get_dir(&self) -> MediaDir;
    fn get_status(&self) -> CallMediaStatus;
    // fn get_stream(&self) -> pjsua_call_media_info__bindgen_ty_1,
}

pub trait UACallInfoExt {

    /// Call identification.
    fn get_id (&self) -> i32;

    /// Initial call role (UAC == caller)
    fn get_role (&self) -> SIPRole;

    /// The account ID where this call belongs.
    fn get_acc_id (&self) -> i32;

    /// Local URI
    fn get_local_info (&self) -> String;

    /// Local Contact
    fn get_local_contact (&self) -> String;

    /// Remote URI
    fn get_remote_info (&self) -> String;

    /// Remote contact
    fn get_remote_contact (&self) -> String;

    /// Dialog Call-ID string.
    fn get_call_id (&self) -> String;

    /// Call setting
    fn get_setting (&self) -> (CallFlags, KeyFrameMethod, u32, u32);

    /// Call state
    fn get_state (&self) -> SIPInvState;

    /// Text describing the state
    fn get_state_text (&self) -> String;

    /// Last status code heard, which can be used as cause code
    fn get_last_status (&self) -> SIPStatusCode;

    /// The reason phrase describing the status.
    fn get_last_status_text (&self) -> String;

    /// Media status of the default audio stream. Default audio stream is chosen according to this
    /// priority:
    ///
    /// - enabled, i.e: SDP media port not zero
    /// - transport protocol in the SDP matching account config's secure media transport usage
    ///   (use_srtp field).
    /// - active, i.e: SDP media direction is not "inactive"
    /// - media order (according to the SDP).
    fn get_media_status (&self) -> CallMediaStatus;

    /// Media direction of the default audio stream. See media_status above on how the default
    /// is chosen.
    fn get_media_dir (&self) -> MediaDir;

    /// The conference port number for the default audio stream. See media_status above on how
    /// the default is chosen.
    fn get_conf_slot (&self) -> i32;

    /// Number of active media info in this call.
    fn get_media_cnt (&self) -> u32;

    /// Array of active media information.
    fn get_media (&self) -> [pjsua_call_media_info; 16usize];

    /// Number of provisional media info in this call.
    fn get_prov_media_cnt (&self) -> u32;

    /// Array of provisional media information. This contains the media info in the provisioning state,
    /// that is when the media session is being created/updated (SDP offer/answer is on progress).
    fn get_prov_media (&self) -> [pjsua_call_media_info; 16usize];

    /// Up-to-date call connected duration (zero when call is not established)
    fn get_connect_duration (&self) -> (i64, i64);

    /// Total call duration, including set-up time
    fn get_total_duration (&self) -> (i64, i64);

    fn get_rem_offerer (&self) -> bool;

    /// Number of audio streams offered by remote
    fn get_rem_aud_cnt (&self) -> u32;

    /// Number of video streams offered by remote
    fn get_rem_vid_cnt (&self) -> u32;

    // fn get_buf_ (&self) -> pjsua_call_info__bindgen_ty_1;
}

pub trait UAStreamInfoExt {
    fn get_type_(&self) -> MediaType;
    // fn get_info(&self) -> pjsua_stream_info__bindgen_ty_1;
}

pub trait UAStreamStatExt {
    fn get_rtcp(&self) -> &pjmedia_sys::pjmedia_rtcp_stat;
    fn get_jbuf(&self) -> &pjmedia_sys::pjmedia_jb_state;
}

pub trait UACallVidStrmOpParamExt {

    fn set_med_idx(&mut self, value: i32);
    fn get_med_idx(&self) -> i32;

    fn set_dir(&mut self, value: MediaDir);
    fn get_dir(&self) -> MediaDir;

    fn set_cap_dev(&mut self, value: i32);
    fn get_cap_dev(&self) -> i32;

}

pub trait UACallSendDtmfParamExt {

    fn set_method(&mut self, value: UADtmfMethod);
    fn get_method(&self) -> UADtmfMethod;

    fn set_duration(&mut self, value: u32);
    fn get_duration(&self) -> u32;

    fn set_digits(&mut self, value: String);
    fn get_digits(&self) -> String;

}

impl UACallMediaInfoExt for UACallMediaInfo {

    fn get_index(&self) -> u32 {
        self.index
    }

    fn get_type_(&self) -> MediaType {
        MediaType::try_from(self.type_)
        .expect("Error UACallMediaInfoExt get type_")
    }

    fn get_dir(&self) -> MediaDir {
        MediaDir::try_from(self.dir)
        .expect("Error UACallMediaInfo get dir")
    }

    fn get_status(&self) -> CallMediaStatus {
        CallMediaStatus::try_from(self.status)
        .expect("Error UACallMediaInfo get status")
    }
}

impl UACallInfoExt for UACallInfo {

    fn get_id (&self) -> i32 {
        self.id
    }

    fn get_role (&self) -> SIPRole {
        SIPRole::try_from(self.role)
        .expect("Error CallInfo get role")
    }

    fn get_acc_id (&self) -> i32 {
        self.acc_id
    }

    fn get_local_info (&self) -> String {
        self.local_info.to_string()
    }

    fn get_local_contact (&self) -> String {
        self.local_contact.to_string()
    }

    fn get_remote_info (&self) -> String {
        self.remote_info.to_string()
    }

    fn get_remote_contact (&self) -> String {
        self.remote_contact.to_string()
    }

    fn get_call_id (&self) -> String {
        self.call_id.to_string()
    }

    fn get_setting (&self) -> (CallFlags, KeyFrameMethod, u32, u32) {
        (
            CallFlags::try_from(self.setting.flag)
            .expect("Error CallInfo get setting"),
            KeyFrameMethod::try_from(self.setting.req_keyframe_method)
            .expect("Error CallInfo get req_keyframe_method"),
            self.setting.aud_cnt,
            self.setting.vid_cnt
        )
    }

    fn get_state (&self) -> SIPInvState {
        SIPInvState::try_from(self.state)
        .expect("Error CallInfo get state")
    }

    fn get_state_text (&self) -> String {
        self.state_text.to_string()
    }

    fn get_last_status (&self) -> SIPStatusCode {
        SIPStatusCode::try_from(self.last_status)
        .expect("Error CallInfo get last_status")
    }

    fn get_last_status_text (&self) -> String {
        self.last_status_text.to_string()
    }

    fn get_media_status (&self) -> CallMediaStatus {
        CallMediaStatus::try_from(self.media_status)
        .expect("Error CallInfo get media_status")
    }

    fn get_media_dir (&self) -> MediaDir {
        MediaDir::try_from(self.media_dir)
        .expect("Error CallInfo get media_dir")
    }

    fn get_conf_slot (&self) -> i32 {
        self.conf_slot
    }

    fn get_media_cnt (&self) -> u32 {
        self.media_cnt
    }

    fn get_media (&self) -> [UACallMediaInfo; 16usize] {
        todo!()
    }

    fn get_prov_media_cnt (&self) -> u32 {
        self.prov_media_cnt
    }

    fn get_prov_media (&self) -> [UACallMediaInfo; 16usize] {
        todo!()
    }

    fn get_connect_duration (&self) -> (i64, i64) {
        (
            self.connect_duration.sec,
            self.connect_duration.msec
        )
    }

    fn get_total_duration (&self) -> (i64, i64) {
        (
            self.total_duration.sec,
            self.total_duration.msec,
        )
    }

    fn get_rem_offerer (&self) -> bool {
        check_boolean(self.rem_offerer)
    }

    fn get_rem_aud_cnt (&self) -> u32 {
        self.rem_aud_cnt
    }

    fn get_rem_vid_cnt (&self) -> u32 {
        self.rem_vid_cnt
    }
}

impl UAStreamInfoExt for UAStreamInfo {

    fn get_type_(&self) -> MediaType {
        MediaType::try_from(self.type_)
        .expect("Error UAStreamInfo get type_")
    }

}

impl UAStreamStatExt for UAStreamStat {

    fn get_rtcp(&self) -> &pjmedia_sys::pjmedia_rtcp_stat {
        &self.rtcp
    }

    fn get_jbuf(&self) -> &pjmedia_sys::pjmedia_jb_state {
        &self.jbuf
    }

}

impl UACallVidStrmOpParamExt for UACallVidStrmOpParam {

    fn set_med_idx(&mut self, value: i32) {
        self.med_idx = value;
    }

    fn get_med_idx(&self) -> i32 {
        self.med_idx
    }

    fn set_dir(&mut self, value: MediaDir) {
        self.dir = value.into();
    }

    fn get_dir(&self) -> MediaDir {
        MediaDir::try_from(self.dir)
        .expect("Error UACallVidStrmOpParam get dir")
    }

    fn set_cap_dev(&mut self, value: i32) {
        self.cap_dev = value;
    }

    fn get_cap_dev(&self) -> i32 {
        self.cap_dev
    }

}

impl UACallSendDtmfParamExt for UACallSendDtmfParam {

    fn set_method(&mut self, value: UADtmfMethod) {
        self.method = value.into();
    }

    fn get_method(&self) -> UADtmfMethod {
        UADtmfMethod::try_from(self.method)
        .expect("Error UACallSendDtmfParam get methof")
    }

    fn set_duration(&mut self, value: u32) {
        self.duration = value;
    }

    fn get_duration(&self) -> u32 {
        self.duration
    }

    fn set_digits(&mut self, value: String) {
        self.digits = pj_str_t::from_string(value);
    }


    fn get_digits(&self) -> String {
        self.digits.to_string()
    }

}



// call helper function

pub fn call_setting_default (opt: &mut UACallSetting) {
    unsafe { pjsua_sys::pjsua_call_setting_default(opt as * mut _) }
}

pub fn call_send_dtmf_param_default (param: &mut pjsua_call_send_dtmf_param) {
    unsafe { pjsua_sys::pjsua_call_send_dtmf_param_default(param as *mut _) }
}

pub fn call_get_max_count () -> u32 {
    unsafe { pjsua_sys::pjsua_call_get_max_count() }
}

pub fn call_get_count () -> u32 {
    unsafe { pjsua_sys::pjsua_call_get_count() }
}

pub fn enum_calls (ids: &mut [i32; pjsua_sys::PJSUA_MAX_CALLS as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_calls( ids.as_mut_ptr(), count as *mut _))
    }
}

// i32 	pjsua_call_make_call (i32 acc_id, const pj_str_t *dst_uri, const pjsua_call_setting *opt, void *user_data, const pjsua_msg_data *msg_data, pjsua_call_id *p_call_id)
pub fn call_make_call (
    acc_id: i32,
    dst_uri: String,
    opt: Option<&mut UACallSetting>,
    msg_data: Option<&mut UAMsgData>,
    p_call_id: Option<&mut i32>
) -> Result<(), i32> {

    let mut dst_uri = pj_str_t::from_string(dst_uri);

    let opt = match opt {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    let p_call_id = match p_call_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };


    unsafe {

        let status = pjsua_sys::pjsua_call_make_call(
            acc_id,
            &mut dst_uri as *const _,
            opt,
            ptr::null_mut(),
            msg_data,
            p_call_id
        );

        utils::check_status(status)
    }
}

pub fn call_is_active (call_id: i32) -> bool {
    unsafe { utils::check_boolean(pjsua_sys::pjsua_call_is_active(call_id)) }
}

pub fn call_has_media (call_id: i32) -> bool {
    unsafe { utils::check_boolean(pjsua_sys::pjsua_call_has_media(call_id)) }
}

pub fn call_get_conf_port (call_id: i32) -> i32 {
    unsafe { pjsua_sys::pjsua_call_get_conf_port(call_id) }
}

pub fn call_get_info (call_id: i32, info: &mut UACallInfo) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_call_get_info(call_id, info as *mut _)) }
}

pub fn call_remote_has_cap (call_id: i32, htype: i32, hname: String, token: String) -> pjsip_dialog_cap_status {
    let hname: *const pj_str_t = &mut pj_str_t::from_string(hname) as *const _;
    let token: *const pj_str_t = &mut pj_str_t::from_string(token) as *const _;

    unsafe {pjsua_sys::pjsua_call_remote_has_cap(call_id, htype, hname, token)}
}

// unused function
// i32 	pjsua_call_set_user_data (pjsua_call_id call_id, void *user_data)
// void * 	pjsua_call_get_user_data (pjsua_call_id call_id)

pub fn call_get_rem_nat_type (call_id: i32, p_type: &mut pj_stun_nat_type) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_get_rem_nat_type(call_id, p_type as *mut _))
    }
}

pub fn call_answer (call_id: i32, code: u32, reason: Option<String>, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const pj_str_t,
        None => ptr::null_mut(),
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_answer( call_id, code, reason, msg_data)) }
}

pub fn call_answer2 (
    call_id: i32,
    opt: &mut UACallSetting,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut UAMsgData>
) -> Result<(), i32> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const _ ,
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_answer2(call_id, opt, code, reason, msg_data)) }
}

pub fn call_answer_with_sdp(
    call_id: i32,
    sdp: &mut pjmedia_sdp_session,
    opt: &mut UACallSetting,
    code: u32,
    reason: Option<String>,
    msg_data: Option<&mut UAMsgData>
) -> Result<(), i32> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value),
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_answer_with_sdp(
            call_id, sdp as *const _, opt as *const _,
            code, reason, msg_data))
    }
}

pub fn call_hangup(
    call_id: i32,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut UAMsgData>
) -> Result<(), i32> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const _,
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_hangup(call_id, code, reason, msg_data)) }
}

pub fn call_process_redirect (call_id: i32, cmd: pjsip_redirect_op) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_call_process_redirect(call_id, cmd)) }
}

pub fn call_set_hold (call_id: i32, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_set_hold( call_id, msg_data)) }
}

pub fn call_set_hold2 (call_id: i32, options: u32, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_set_hold2(call_id, options, msg_data)) }
}

pub fn call_reinvite(call_id: i32, options: u32, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_reinvite( call_id, options, msg_data)) }
}

pub fn call_reinvite2(call_id: i32, opt: &mut UACallSetting, msg_data: Option<&mut UAMsgData> ) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_reinvite2( call_id, opt as *const _, msg_data )) }
}

pub fn call_update (call_id: i32, options: u32, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_update( call_id, options, msg_data)) }
}

pub fn call_update2 (call_id: i32, opt: &mut UACallSetting, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_update2( call_id, opt as *const _, msg_data))
    }
}

pub fn call_xfer (call_id: i32, dest: String, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let mut dest = pj_str_t::from_string(dest);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_xfer(call_id,&mut dest as *const _,msg_data))
    }
}

pub fn call_xfer_replaces(call_id: i32, dest_call_id: i32, options: u32, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_xfer_replaces(call_id, dest_call_id, options, msg_data))
    }
}

pub fn call_dial_dtmf (call_id: i32, digits: String) -> Result<(), i32> {

    let mut digits = pj_str_t::from_string(digits);

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_dial_dtmf(call_id, &mut digits as *const _))
    }

}

pub fn call_send_dtmf (call_id: i32, param: &mut pjsua_call_send_dtmf_param) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_send_dtmf (call_id, param as *const _))
    }
}

pub fn call_send_im (call_id: i32, mime_type: String, content: String, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let mut mime_type = pj_str_t::from_string(mime_type);
    let mut content = pj_str_t::from_string(content);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_call_send_im(
            call_id,
            &mut mime_type as *const _,
            &mut content as *const _,
            msg_data,
            ptr::null_mut()
        );
        utils::check_status(status)
    }
}

pub fn call_send_typing_ind (call_id: i32, is_typing: bool, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_call_send_typing_ind(
            call_id,
            utils::boolean_to_pjbool(is_typing),
            msg_data
        );

        utils::check_status(status)
    }
}

pub fn call_send_request (call_id: i32, method: String, msg_data: Option<&mut UAMsgData>) -> Result<(), i32> {

    let mut method = pj_str_t::from_string(method);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_send_request( call_id, &mut method as *const _, msg_data ))
    }
}

pub fn call_hangup_all () {
    unsafe { pjsua_sys::pjsua_call_hangup_all() }
}

pub fn call_dump(
    call_id: i32,
    with_media: bool,
    buffer: String,
    maxlen: u32,
    indent: String,
) -> Result<(), i32> {

    let buffer: *mut i8 = CString::new(buffer.as_str()).expect("CString::pjsua_call_dump fail.").into_raw();
    let indent: *const i8 = CString::new(indent.as_str()).expect("CString::pjsua_call_dump fail.").into_raw();

    unsafe {

        let status = pjsua_sys::pjsua_call_dump(
            call_id,
            utils::boolean_to_pjbool(with_media),
            buffer,
            maxlen,
            indent as *const _
        );

        utils::check_status(status)
    }
}

pub fn call_get_stream_info (call_id: i32, med_idx: u32, psi: &mut UAStreamInfo) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_get_stream_info (call_id, med_idx, psi as *mut _))
    }
}

pub fn call_get_stream_stat (call_id: i32, med_idx: u32, stat: &mut UAStreamStat) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_get_stream_stat( call_id, med_idx, stat as *mut _))
    }
}

// i32 	pjsua_call_get_med_transport_info (pjsua_call_id call_id, unsigned med_idx, pjmedia_transport_info *t)
pub fn call_get_med_transport_info (call_id: i32, med_idx: u32, t: &mut pjmedia_transport_info) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_get_med_transport_info( call_id, med_idx, t as *mut _))
    }
}

// void 	pjsua_call_vid_strm_op_param_default (pjsua_call_vid_strm_op_param *param)


// pjsua_vid_win_id 	pjsua_call_get_vid_win (pjsua_call_id call_id)
// pjsua_conf_port_id 	pjsua_call_get_vid_conf_port (pjsua_call_id call_id, pjmedia_dir dir)
// i32 	pjsua_call_set_vid_strm (pjsua_call_id call_id, pjsua_call_vid_strm_op op, const pjsua_call_vid_strm_op_param *param)
// pj_bool_t 	pjsua_call_vid_stream_is_running (pjsua_call_id call_id, int med_idx, pjmedia_dir dir)
// int 	pjsua_call_get_vid_stream_idx (pjsua_call_id call_id)
