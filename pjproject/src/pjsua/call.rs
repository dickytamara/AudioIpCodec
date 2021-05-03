
use std::convert::TryFrom;
use crate::utils::check_boolean;

use super::*;


// readonly
pub trait CallInfoExt {

    /// Call identification.
    fn get_id (&self) -> i32;

    /// Initial call role (UAC == caller)
    fn get_role (&self) -> CallInfoRole;

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
    fn get_state (&self) -> InviteState;

    /// Text describing the state
    fn get_state_text (&self) -> String;

    /// Last status code heard, which can be used as cause code
    fn get_last_status (&self) -> SipStatusCode;

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
    fn get_media_dir (&self) -> MediaDirection;

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


impl CallInfoExt for CallInfo {

    fn get_id (&self) -> i32 {
        self.id
    }

    fn get_role (&self) -> CallInfoRole {
        CallInfoRole::try_from(self.role)
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

    fn get_state (&self) -> InviteState {
        InviteState::try_from(self.state)
        .expect("Error CallInfo get state")
    }

    fn get_state_text (&self) -> String {
        self.state_text.to_string()
    }

    fn get_last_status (&self) -> SipStatusCode {
        SipStatusCode::try_from(self.last_status)
        .expect("Error CallInfo get last_status")
    }

    fn get_last_status_text (&self) -> String {
        self.last_status_text.to_string()
    }

    fn get_media_status (&self) -> CallMediaStatus {
        CallMediaStatus::try_from(self.media_status)
        .expect("Error CallInfo get media_status")
    }

    fn get_media_dir (&self) -> MediaDirection {
        MediaDirection::try_from(self.media_dir)
        .expect("Error CallInfo get media_dir")
    }

    fn get_conf_slot (&self) -> i32 {
        self.conf_slot
    }

    fn get_media_cnt (&self) -> u32 {
        self.media_cnt
    }

    fn get_media (&self) -> [CallMediaInfo; 16usize] {
        todo!()
    }

    fn get_prov_media_cnt (&self) -> u32 {
        self.prov_media_cnt
    }

    fn get_prov_media (&self) -> [CallMediaInfo; 16usize] {
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


