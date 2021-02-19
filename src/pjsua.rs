#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use super::pjdefault::AutoCreate;
use super::pjlib::*;
use super::pjmedia::*;
use super::pjsip::*;

use std::os::raw::{c_int, c_uint, c_void, c_char};
use std::ptr;


pub const PJSUA_POOL_LEN: u32 = 1000;
pub const PJSUA_POOL_INC: u32 = 1000;
pub const PJSUA_POOL_LEN_ACC: u32 = 512;
pub const PJSUA_POOL_INC_ACC: u32 = 256;
pub const PJSUA_ACC_MAX_PROXIES: u32 = 8;
pub const PJSUA_DEFAULT_SRTP_SECURE_SIGNALING: u32 = 1;
pub const PJSUA_ADD_ICE_TAGS: u32 = 1;
pub const PJSUA_ACQUIRE_CALL_TIMEOUT: u32 = 2000;
pub const PJSUA_HAS_VIDEO: u32 = 0;
pub const PJSUA_VID_REQ_KEYFRAME_INTERVAL: u32 = 3000;
pub const PJSUA_SEPARATE_WORKER_FOR_TIMER: u32 = 0;
pub const PJSUA_ICE_TRANSPORT_OPTION: u32 = 0;
pub const PJSUA_TRICKLE_ICE_NEW_CAND_CHECK_INTERVAL: u32 = 100;
pub const PJSUA_MAX_ACC: u32 = 8;
pub const PJSUA_REG_INTERVAL: u32 = 300;
pub const PJSUA_UNREG_TIMEOUT: u32 = 4000;
pub const PJSUA_DEFAULT_ACC_PRIORITY: u32 = 0;
pub const PJSUA_UNPUBLISH_MAX_WAIT_TIME_MSEC: u32 = 2000;
pub const PJSUA_REG_RETRY_INTERVAL: u32 = 300;
pub const PJSUA_REG_USE_OUTBOUND_PROXY: u32 = 1;
pub const PJSUA_REG_USE_ACC_PROXY: u32 = 2;
pub const PJSUA_MAX_CALLS: u32 = 32;
pub const PJSUA_MAX_VID_WINS: u32 = 16;
pub const PJSUA_CALL_SEND_DTMF_DURATION_DEFAULT: u32 = 160;
pub const PJSUA_XFER_NO_REQUIRE_REPLACES: u32 = 1;
pub const PJSUA_MAX_BUDDIES: u32 = 256;
pub const PJSUA_PRES_TIMER: u32 = 300;
pub const PJSUA_MEDIA_HAS_PJMEDIA: u32 = 1;
pub const PJSUA_THIRD_PARTY_STREAM_HAS_GET_INFO: u32 = 0;
pub const PJSUA_THIRD_PARTY_STREAM_HAS_GET_STAT: u32 = 0;
pub const PJSUA_MAX_CONF_PORTS: u32 = 254;
pub const PJSUA_DEFAULT_CLOCK_RATE: u32 = 16000;
pub const PJSUA_DEFAULT_AUDIO_FRAME_PTIME: u32 = 20;
pub const PJSUA_DEFAULT_CODEC_QUALITY: u32 = 8;
pub const PJSUA_DEFAULT_ILBC_MODE: u32 = 30;
pub const PJSUA_DEFAULT_EC_TAIL_LEN: u32 = 200;
pub const PJSUA_MAX_PLAYERS: u32 = 32;
pub const PJSUA_MAX_RECORDERS: u32 = 32;
pub const PJSUA_SDP_SESS_HAS_CONN: u32 = 0;
pub const PJSUA_TRANSPORT_RESTART_DELAY_TIME: u32 = 10;



pub const pjsua_invalid_id_const__PJSUA_INVALID_ID: pjsua_invalid_id_const_ = -1;
pub type pjsua_invalid_id_const_ = c_int;
pub type pjsua_call_id = c_int;
pub type pjsua_acc_id = c_int;
pub type pjsua_buddy_id = c_int;
pub type pjsua_player_id = c_int;
pub type pjsua_recorder_id = c_int;
pub type pjsua_conf_port_id = c_int;

pub const pjsua_state_PJSUA_STATE_NULL: pjsua_state = 0;
pub const pjsua_state_PJSUA_STATE_CREATED: pjsua_state = 1;
pub const pjsua_state_PJSUA_STATE_INIT: pjsua_state = 2;
pub const pjsua_state_PJSUA_STATE_STARTING: pjsua_state = 3;
pub const pjsua_state_PJSUA_STATE_RUNNING: pjsua_state = 4;
pub const pjsua_state_PJSUA_STATE_CLOSING: pjsua_state = 5;
pub type pjsua_state = c_uint;

pub const pjsua_med_tp_st_PJSUA_MED_TP_NULL: pjsua_med_tp_st = 0;
pub const pjsua_med_tp_st_PJSUA_MED_TP_CREATING: pjsua_med_tp_st = 1;
pub const pjsua_med_tp_st_PJSUA_MED_TP_IDLE: pjsua_med_tp_st = 2;
pub const pjsua_med_tp_st_PJSUA_MED_TP_INIT: pjsua_med_tp_st = 3;
pub const pjsua_med_tp_st_PJSUA_MED_TP_RUNNING: pjsua_med_tp_st = 4;
pub const pjsua_med_tp_st_PJSUA_MED_TP_DISABLED: pjsua_med_tp_st = 5;
pub type pjsua_med_tp_st = c_uint;

pub type pj_stun_resolve_cb = Option<unsafe extern "C" fn(result: *const pj_stun_resolve_result)>;
pub const pjsua_create_media_transport_flag_PJSUA_MED_TP_CLOSE_MEMBER: pjsua_create_media_transport_flag = 1;
pub type pjsua_create_media_transport_flag = c_uint;

pub const pjsua_contact_rewrite_method_PJSUA_CONTACT_REWRITE_UNREGISTER: pjsua_contact_rewrite_method = 1;
pub const pjsua_contact_rewrite_method_PJSUA_CONTACT_REWRITE_NO_UNREG: pjsua_contact_rewrite_method = 2;
pub const pjsua_contact_rewrite_method_PJSUA_CONTACT_REWRITE_ALWAYS_UPDATE: pjsua_contact_rewrite_method = 4;
pub type pjsua_contact_rewrite_method = c_uint;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_NULL: pjsua_ip_change_op = 0;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_RESTART_LIS: pjsua_ip_change_op = 1;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP: pjsua_ip_change_op = 2;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT: pjsua_ip_change_op = 3;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS: pjsua_ip_change_op = 4;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS: pjsua_ip_change_op = 5;
pub const pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_COMPLETED: pjsua_ip_change_op = 6;
pub type pjsua_ip_change_op = c_uint;




#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_srv_pres {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_logging_config {
    pub msg_logging: pj_bool_t,
    pub level: c_uint,
    pub console_level: c_uint,
    pub decor: c_uint,
    pub log_filename: pj_str_t,
    pub log_file_flags: c_uint,
    pub cb: Option<
        unsafe extern "C" fn(
            level: c_int,
            data: *const c_char,
            len: c_int,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_mwi_info {
    pub evsub: *mut pjsip_evsub,
    pub rdata: *mut pjsip_rx_data,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_reg_info {
    pub cbparam: *mut pjsip_regc_cbparam,
    pub regc: *mut pjsip_regc,
    pub renew: pj_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_stream_info {
    pub type_: pjmedia_type,
    pub info: pjsua_stream_info__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsua_stream_info__bindgen_ty_1 {
    pub aud: pjmedia_stream_info,
    pub vid: pjmedia_vid_stream_info,
    _bindgen_union_align: [u64; 277usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_stream_stat {
    pub rtcp: pjmedia_rtcp_stat,
    pub jbuf: pjmedia_jb_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_on_stream_precreate_param {
    pub stream_idx: c_uint,
    pub stream_info: pjsua_stream_info,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_on_stream_created_param {
    pub stream: *mut pjmedia_stream,
    pub stream_idx: c_uint,
    pub destroy_port: pj_bool_t,
    pub port: *mut pjmedia_port,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_med_tp_state_info {
    pub med_idx: c_uint,
    pub state: pjsua_med_tp_st,
    pub status: pj_status_t,
    pub sip_err_code: c_int,
    pub ext_info: *mut c_void,
}
pub type pjsua_med_tp_state_cb = Option<
    unsafe extern "C" fn(
        call_id: pjsua_call_id,
        info: *const pjsua_med_tp_state_info,
    ) -> pj_status_t,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_srtp_opt {
    pub crypto_count: c_uint,
    pub crypto: [pjmedia_srtp_crypto; 16usize],
    pub keying_count: c_uint,
    pub keying: [pjmedia_srtp_keying_method; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsua_ip_change_op_info {
    pub lis_restart: pjsua_ip_change_op_info__bindgen_ty_1,
    pub acc_shutdown_tp: pjsua_ip_change_op_info__bindgen_ty_2,
    pub acc_update_contact: pjsua_ip_change_op_info__bindgen_ty_3,
    pub acc_hangup_calls: pjsua_ip_change_op_info__bindgen_ty_4,
    pub acc_reinvite_calls: pjsua_ip_change_op_info__bindgen_ty_5,
    _bindgen_union_align: [u32; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_op_info__bindgen_ty_1 {
    pub transport_id: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_op_info__bindgen_ty_2 {
    pub acc_id: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_op_info__bindgen_ty_3 {
    pub acc_id: pjsua_acc_id,
    pub is_register: pj_bool_t,
    pub code: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_op_info__bindgen_ty_4 {
    pub acc_id: pjsua_acc_id,
    pub call_id: pjsua_call_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_op_info__bindgen_ty_5 {
    pub acc_id: pjsua_acc_id,
    pub call_id: pjsua_call_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_dtmf_info {
    pub method: pjsua_dtmf_method,
    pub digit: c_uint,
    pub duration: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_dtmf_event {
    pub method: pjsua_dtmf_method,
    pub timestamp: c_uint,
    pub digit: c_uint,
    pub duration: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_call_setting {
    pub flag: c_uint,
    pub req_keyframe_method: c_uint,
    pub aud_cnt: c_uint,
    pub vid_cnt: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_callback {
    pub on_call_state:
        Option<unsafe extern "C" fn(call_id: pjsua_call_id, e: *mut pjsip_event)>,
    pub on_incoming_call: Option<
        unsafe extern "C" fn(
            acc_id: pjsua_acc_id,
            call_id: pjsua_call_id,
            rdata: *mut pjsip_rx_data,
        ),
    >,
    pub on_call_tsx_state: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            tsx: *mut pjsip_transaction,
            e: *mut pjsip_event,
        ),
    >,
    pub on_call_media_state: Option<unsafe extern "C" fn(call_id: pjsua_call_id)>,
    pub on_call_sdp_created: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            sdp: *mut pjmedia_sdp_session,
            pool: *mut pj_pool_t,
            rem_sdp: *const pjmedia_sdp_session,
        ),
    >,
    pub on_stream_precreate: Option<
        unsafe extern "C" fn(call_id: pjsua_call_id, param: *mut pjsua_on_stream_precreate_param),
    >,
    pub on_stream_created: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            strm: *mut pjmedia_stream,
            stream_idx: c_uint,
            p_port: *mut *mut pjmedia_port,
        ),
    >,
    pub on_stream_created2: Option<
        unsafe extern "C" fn(call_id: pjsua_call_id, param: *mut pjsua_on_stream_created_param),
    >,
    pub on_stream_destroyed: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            strm: *mut pjmedia_stream,
            stream_idx: c_uint,
        ),
    >,
    pub on_dtmf_digit: Option<
        unsafe extern "C" fn(call_id: pjsua_call_id, digit: c_int),
    >,
    pub on_dtmf_digit2: Option<
        unsafe extern "C" fn(call_id: pjsua_call_id, info: *const pjsua_dtmf_info),
    >,
    pub on_dtmf_event: Option<
        unsafe extern "C" fn(call_id: pjsua_call_id, event: *const pjsua_dtmf_event),
    >,
    pub on_call_transfer_request: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            dst: *const pj_str_t,
            code: *mut pjsip_status_code,
        ),
    >,
    pub on_call_transfer_request2: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            dst: *const pj_str_t,
            code: *mut pjsip_status_code,
            opt: *mut pjsua_call_setting,
        ),
    >,
    pub on_call_transfer_status: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            st_code: c_int,
            st_text: *const pj_str_t,
            final_: pj_bool_t,
            p_cont: *mut pj_bool_t,
        ),
    >,
    pub on_call_replace_request: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            rdata: *mut pjsip_rx_data,
            st_code: *mut c_int,
            st_text: *mut pj_str_t,
        ),
    >,
    pub on_call_replace_request2: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            rdata: *mut pjsip_rx_data,
            st_code: *mut c_int,
            st_text: *mut pj_str_t,
            opt: *mut pjsua_call_setting,
        ),
    >,
    pub on_call_replaced: Option<
        unsafe extern "C" fn(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id),
    >,
    pub on_call_rx_offer: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            offer: *const pjmedia_sdp_session,
            reserved: *mut c_void,
            code: *mut pjsip_status_code,
            opt: *mut pjsua_call_setting,
        ),
    >,
    pub on_call_rx_reinvite: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            offer: *const pjmedia_sdp_session,
            rdata: *mut pjsip_rx_data,
            reserved: *mut c_void,
            async_: *mut pj_bool_t,
            code: *mut pjsip_status_code,
            opt: *mut pjsua_call_setting,
        ),
    >,
    pub on_call_tx_offer: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            reserved: *mut c_void,
            opt: *mut pjsua_call_setting,
        ),
    >,
    pub on_reg_started:
        Option<unsafe extern "C" fn(acc_id: pjsua_acc_id, renew: pj_bool_t)>,
    pub on_reg_started2: Option<
        unsafe extern "C" fn(acc_id: pjsua_acc_id, info: *mut pjsua_reg_info),
    >,
    pub on_reg_state: Option<unsafe extern "C" fn(acc_id: pjsua_acc_id)>,
    pub on_reg_state2: Option<
        unsafe extern "C" fn(acc_id: pjsua_acc_id, info: *mut pjsua_reg_info),
    >,
    pub on_incoming_subscribe: Option<
        unsafe extern "C" fn(
            acc_id: pjsua_acc_id,
            srv_pres: *mut pjsua_srv_pres,
            buddy_id: pjsua_buddy_id,
            from: *const pj_str_t,
            rdata: *mut pjsip_rx_data,
            code: *mut pjsip_status_code,
            reason: *mut pj_str_t,
            msg_data: *mut pjsua_msg_data,
        ),
    >,
    pub on_srv_subscribe_state: Option<
        unsafe extern "C" fn(
            acc_id: pjsua_acc_id,
            srv_pres: *mut pjsua_srv_pres,
            remote_uri: *const pj_str_t,
            state: pjsip_evsub_state,
            event: *mut pjsip_event,
        ),
    >,
    pub on_buddy_state: Option<unsafe extern "C" fn(buddy_id: pjsua_buddy_id)>,
    pub on_buddy_evsub_state: Option<
        unsafe extern "C" fn(
            buddy_id: pjsua_buddy_id,
            sub: *mut pjsip_evsub,
            event: *mut pjsip_event,
        ),
    >,
    pub on_pager: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            from: *const pj_str_t,
            to: *const pj_str_t,
            contact: *const pj_str_t,
            mime_type: *const pj_str_t,
            body: *const pj_str_t,
        ),
    >,
    pub on_pager2: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            from: *const pj_str_t,
            to: *const pj_str_t,
            contact: *const pj_str_t,
            mime_type: *const pj_str_t,
            body: *const pj_str_t,
            rdata: *mut pjsip_rx_data,
            acc_id: pjsua_acc_id,
        ),
    >,
    pub on_pager_status: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            to: *const pj_str_t,
            body: *const pj_str_t,
            user_data: *mut c_void,
            status: pjsip_status_code,
            reason: *const pj_str_t,
        ),
    >,
    pub on_pager_status2: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            to: *const pj_str_t,
            body: *const pj_str_t,
            user_data: *mut c_void,
            status: pjsip_status_code,
            reason: *const pj_str_t,
            tdata: *mut pjsip_tx_data,
            rdata: *mut pjsip_rx_data,
            acc_id: pjsua_acc_id,
        ),
    >,
    pub on_typing: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            from: *const pj_str_t,
            to: *const pj_str_t,
            contact: *const pj_str_t,
            is_typing: pj_bool_t,
        ),
    >,
    pub on_typing2: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            from: *const pj_str_t,
            to: *const pj_str_t,
            contact: *const pj_str_t,
            is_typing: pj_bool_t,
            rdata: *mut pjsip_rx_data,
            acc_id: pjsua_acc_id,
        ),
    >,
    pub on_nat_detect:
        Option<unsafe extern "C" fn(res: *const pj_stun_nat_detect_result)>,
    pub on_call_redirected: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            target: *const pjsip_uri,
            e: *const pjsip_event,
        ) -> pjsip_redirect_op,
    >,
    pub on_mwi_state:
        Option<unsafe extern "C" fn(acc_id: pjsua_acc_id, evsub: *mut pjsip_evsub)>,
    pub on_mwi_info: Option<
        unsafe extern "C" fn(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info),
    >,
    pub on_transport_state: pjsip_tp_state_callback,
    pub on_call_media_transport_state: pjsua_med_tp_state_cb,
    pub on_ice_transport_error: Option<
        unsafe extern "C" fn(
            index: c_int,
            op: pj_ice_strans_op,
            status: pj_status_t,
            param: *mut c_void,
        ),
    >,
    pub on_snd_dev_operation: Option<
        unsafe extern "C" fn(operation: c_int) -> pj_status_t,
    >,
    pub on_call_media_event: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            med_idx: c_uint,
            event: *mut pjmedia_event,
        ),
    >,
    pub on_create_media_transport: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            media_idx: c_uint,
            base_tp: *mut pjmedia_transport,
            flags: c_uint,
        ) -> *mut pjmedia_transport,
    >,
    pub on_create_media_transport_srtp: Option<
        unsafe extern "C" fn(
            call_id: pjsua_call_id,
            media_idx: c_uint,
            srtp_opt: *mut pjmedia_srtp_setting,
        ),
    >,
    pub on_acc_find_for_incoming: Option<
        unsafe extern "C" fn(rdata: *const pjsip_rx_data, acc_id: *mut pjsua_acc_id),
    >,
    pub on_stun_resolution_complete: pj_stun_resolve_cb,
    pub on_ip_change_progress: Option<
        unsafe extern "C" fn(
            op: pjsua_ip_change_op,
            status: pj_status_t,
            info: *const pjsua_ip_change_op_info,
        ),
    >,
    pub on_media_event: Option<unsafe extern "C" fn(event: *mut pjmedia_event)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_config {
    pub max_calls: c_uint,
    pub thread_cnt: c_uint,
    pub nameserver_count: c_uint,
    pub nameserver: [pj_str_t; 4usize],
    pub force_lr: pj_bool_t,
    pub outbound_proxy_cnt: c_uint,
    pub outbound_proxy: [pj_str_t; 4usize],
    pub stun_domain: pj_str_t,
    pub stun_host: pj_str_t,
    pub stun_srv_cnt: c_uint,
    pub stun_srv: [pj_str_t; 8usize],
    pub stun_try_ipv6: pj_bool_t,
    pub stun_ignore_failure: pj_bool_t,
    pub stun_map_use_stun2: pj_bool_t,
    pub nat_type_in_sdp: c_int,
    pub require_100rel: pjsua_100rel_use,
    pub use_timer: pjsua_sip_timer_use,
    pub enable_unsolicited_mwi: pj_bool_t,
    pub timer_setting: pjsip_timer_setting,
    pub cred_count: c_uint,
    pub cred_info: [pjsip_cred_info; 8usize],
    pub cb: pjsua_callback,
    pub user_agent: pj_str_t,
    pub use_srtp: pjmedia_srtp_use,
    pub srtp_secure_signaling: c_int,
    pub srtp_optional_dup_offer: pj_bool_t,
    pub srtp_opt: pjsua_srtp_opt,
    pub hangup_forked_call: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_msg_data {
    pub target_uri: pj_str_t,
    pub hdr_list: pjsip_hdr,
    pub content_type: pj_str_t,
    pub msg_body: pj_str_t,
    pub multipart_ctype: pjsip_media_type,
    pub multipart_parts: pjsip_multipart_part,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_stun_resolve_result {
    pub token: *mut c_void,
    pub status: pj_status_t,
    pub name: pj_str_t,
    pub addr: pj_sockaddr,
    pub index: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_param {
    pub restart_listener: pj_bool_t,
    pub restart_lis_delay: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ip_change_acc_cfg {
    pub shutdown_tp: pj_bool_t,
    pub hangup_calls: pj_bool_t,
    pub reinvite_flags: c_uint,
}


pub type pjsua_transport_id = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_transport_config {
    pub port: c_uint,
    pub port_range: c_uint,
    pub public_addr: pj_str_t,
    pub bound_addr: pj_str_t,
    pub tls_setting: pjsip_tls_setting,
    pub qos_type: pj_qos_type,
    pub qos_params: pj_qos_params,
    pub sockopt_params: pj_sockopt_params,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_transport_info {
    pub id: pjsua_transport_id,
    pub type_: pjsip_transport_type_e,
    pub type_name: pj_str_t,
    pub info: pj_str_t,
    pub flag: c_uint,
    pub addr_len: c_uint,
    pub local_addr: pj_sockaddr,
    pub local_name: pjsip_host_port,
    pub usage_count: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ice_config {
    pub enable_ice: pj_bool_t,
    pub ice_max_host_cands: c_int,
    pub ice_opt: pj_ice_sess_options,
    pub ice_no_rtcp: pj_bool_t,
    pub ice_always_update: pj_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_turn_config {
    pub enable_turn: pj_bool_t,
    pub turn_server: pj_str_t,
    pub turn_conn_type: pj_turn_tp_type,
    pub turn_auth_cred: pj_stun_auth_cred,
    pub turn_tls_setting: pj_turn_sock_tls_cfg,
}

pub const pjsua_dtmf_method_PJSUA_DTMF_METHOD_RFC2833: pjsua_dtmf_method = 0;
pub const pjsua_dtmf_method_PJSUA_DTMF_METHOD_SIP_INFO: pjsua_dtmf_method = 1;
pub type pjsua_dtmf_method = c_uint;

pub const pjsua_sip_timer_use_PJSUA_SIP_TIMER_INACTIVE: pjsua_sip_timer_use = 0;
pub const pjsua_sip_timer_use_PJSUA_SIP_TIMER_OPTIONAL: pjsua_sip_timer_use = 1;
pub const pjsua_sip_timer_use_PJSUA_SIP_TIMER_REQUIRED: pjsua_sip_timer_use = 2;
pub const pjsua_sip_timer_use_PJSUA_SIP_TIMER_ALWAYS: pjsua_sip_timer_use = 3;
pub type pjsua_sip_timer_use = c_uint;
pub const pjsua_100rel_use_PJSUA_100REL_NOT_USED: pjsua_100rel_use = 0;
pub const pjsua_100rel_use_PJSUA_100REL_MANDATORY: pjsua_100rel_use = 1;
pub const pjsua_100rel_use_PJSUA_100REL_OPTIONAL: pjsua_100rel_use = 2;
pub type pjsua_100rel_use = c_uint;

pub const pjsua_destroy_flag_PJSUA_DESTROY_NO_RX_MSG: pjsua_destroy_flag = 1;
pub const pjsua_destroy_flag_PJSUA_DESTROY_NO_TX_MSG: pjsua_destroy_flag = 2;
pub const pjsua_destroy_flag_PJSUA_DESTROY_NO_NETWORK: pjsua_destroy_flag = 3;
pub type pjsua_destroy_flag = c_uint;

pub const pjsua_call_hold_type_PJSUA_CALL_HOLD_TYPE_RFC3264: pjsua_call_hold_type = 0;
pub const pjsua_call_hold_type_PJSUA_CALL_HOLD_TYPE_RFC2543: pjsua_call_hold_type = 1;
pub type pjsua_call_hold_type = c_uint;
pub const pjsua_stun_use_PJSUA_STUN_USE_DEFAULT: pjsua_stun_use = 0;
pub const pjsua_stun_use_PJSUA_STUN_USE_DISABLED: pjsua_stun_use = 1;
pub const pjsua_stun_use_PJSUA_STUN_RETRY_ON_FAILURE: pjsua_stun_use = 2;
pub type pjsua_stun_use = c_uint;
pub const pjsua_ice_config_use_PJSUA_ICE_CONFIG_USE_DEFAULT: pjsua_ice_config_use = 0;
pub const pjsua_ice_config_use_PJSUA_ICE_CONFIG_USE_CUSTOM: pjsua_ice_config_use = 1;
pub type pjsua_ice_config_use = c_uint;
pub const pjsua_turn_config_use_PJSUA_TURN_CONFIG_USE_DEFAULT: pjsua_turn_config_use = 0;
pub const pjsua_turn_config_use_PJSUA_TURN_CONFIG_USE_CUSTOM: pjsua_turn_config_use = 1;
pub type pjsua_turn_config_use = c_uint;

pub const pjsua_ipv6_use_PJSUA_IPV6_DISABLED: pjsua_ipv6_use = 0;
pub const pjsua_ipv6_use_PJSUA_IPV6_ENABLED: pjsua_ipv6_use = 1;
pub type pjsua_ipv6_use = c_uint;
pub const pjsua_nat64_opt_PJSUA_NAT64_DISABLED: pjsua_nat64_opt = 0;
pub const pjsua_nat64_opt_PJSUA_NAT64_ENABLED: pjsua_nat64_opt = 1;
pub type pjsua_nat64_opt = c_uint;

pub type pjsua_vid_win_id = c_int;
pub const pjsua_call_media_status_PJSUA_CALL_MEDIA_NONE: pjsua_call_media_status = 0;
pub const pjsua_call_media_status_PJSUA_CALL_MEDIA_ACTIVE: pjsua_call_media_status = 1;
pub const pjsua_call_media_status_PJSUA_CALL_MEDIA_LOCAL_HOLD: pjsua_call_media_status = 2;
pub const pjsua_call_media_status_PJSUA_CALL_MEDIA_REMOTE_HOLD: pjsua_call_media_status = 3;
pub const pjsua_call_media_status_PJSUA_CALL_MEDIA_ERROR: pjsua_call_media_status = 4;
pub type pjsua_call_media_status = c_uint;
pub const pjsua_vid_req_keyframe_method_PJSUA_VID_REQ_KEYFRAME_SIP_INFO: pjsua_vid_req_keyframe_method = 1;
pub const pjsua_vid_req_keyframe_method_PJSUA_VID_REQ_KEYFRAME_RTCP_PLI: pjsua_vid_req_keyframe_method = 2;
pub type pjsua_vid_req_keyframe_method = c_uint;

pub const pjsua_call_flag_PJSUA_CALL_UNHOLD: pjsua_call_flag = 1;
pub const pjsua_call_flag_PJSUA_CALL_UPDATE_CONTACT: pjsua_call_flag = 2;
pub const pjsua_call_flag_PJSUA_CALL_INCLUDE_DISABLED_MEDIA: pjsua_call_flag = 4;
pub const pjsua_call_flag_PJSUA_CALL_NO_SDP_OFFER: pjsua_call_flag = 8;
pub const pjsua_call_flag_PJSUA_CALL_REINIT_MEDIA: pjsua_call_flag = 16;
pub const pjsua_call_flag_PJSUA_CALL_UPDATE_VIA: pjsua_call_flag = 32;
pub const pjsua_call_flag_PJSUA_CALL_UPDATE_TARGET: pjsua_call_flag = 64;
pub type pjsua_call_flag = c_uint;

pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_NO_OP: pjsua_call_vid_strm_op = 0;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_ADD: pjsua_call_vid_strm_op = 1;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_REMOVE: pjsua_call_vid_strm_op = 2;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_CHANGE_DIR: pjsua_call_vid_strm_op = 3;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_CHANGE_CAP_DEV: pjsua_call_vid_strm_op = 4;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_START_TRANSMIT: pjsua_call_vid_strm_op = 5;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_STOP_TRANSMIT: pjsua_call_vid_strm_op = 6;
pub const pjsua_call_vid_strm_op_PJSUA_CALL_VID_STRM_SEND_KEYFRAME: pjsua_call_vid_strm_op = 7;
pub type pjsua_call_vid_strm_op = c_uint;

pub const pjsua_buddy_status_PJSUA_BUDDY_STATUS_UNKNOWN: pjsua_buddy_status = 0;
pub const pjsua_buddy_status_PJSUA_BUDDY_STATUS_ONLINE: pjsua_buddy_status = 1;
pub const pjsua_buddy_status_PJSUA_BUDDY_STATUS_OFFLINE: pjsua_buddy_status = 2;
pub type pjsua_buddy_status = c_uint;

pub const pjsua_snd_dev_id_PJSUA_SND_DEFAULT_CAPTURE_DEV: pjsua_snd_dev_id = -1;
pub const pjsua_snd_dev_id_PJSUA_SND_DEFAULT_PLAYBACK_DEV: pjsua_snd_dev_id = -2;
pub const pjsua_snd_dev_id_PJSUA_SND_NO_DEV: pjsua_snd_dev_id = -3;
pub const pjsua_snd_dev_id_PJSUA_SND_NULL_DEV: pjsua_snd_dev_id = -99;
pub type pjsua_snd_dev_id = c_int;
pub const pjsua_snd_dev_mode_PJSUA_SND_DEV_SPEAKER_ONLY: pjsua_snd_dev_mode = 1;
pub const pjsua_snd_dev_mode_PJSUA_SND_DEV_NO_IMMEDIATE_OPEN: pjsua_snd_dev_mode = 2;
pub type pjsua_snd_dev_mode = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_acc_config {
    pub user_data: *mut c_void,
    pub priority: c_int,
    pub id: pj_str_t,
    pub reg_uri: pj_str_t,
    pub reg_hdr_list: pjsip_hdr,
    pub reg_contact_params: pj_str_t,
    pub sub_hdr_list: pjsip_hdr,
    pub mwi_enabled: pj_bool_t,
    pub mwi_expires: c_uint,
    pub publish_enabled: pj_bool_t,
    pub publish_opt: pjsip_publishc_opt,
    pub unpublish_max_wait_time_msec: c_uint,
    pub auth_pref: pjsip_auth_clt_pref,
    pub pidf_tuple_id: pj_str_t,
    pub force_contact: pj_str_t,
    pub contact_params: pj_str_t,
    pub contact_uri_params: pj_str_t,
    pub require_100rel: pjsua_100rel_use,
    pub use_timer: pjsua_sip_timer_use,
    pub timer_setting: pjsip_timer_setting,
    pub proxy_cnt: c_uint,
    pub proxy: [pj_str_t; 8usize],
    pub lock_codec: c_uint,
    pub reg_timeout: c_uint,
    pub reg_delay_before_refresh: c_uint,
    pub unreg_timeout: c_uint,
    pub cred_count: c_uint,
    pub cred_info: [pjsip_cred_info; 8usize],
    pub transport_id: pjsua_transport_id,
    pub allow_contact_rewrite: pj_bool_t,
    pub contact_rewrite_method: c_int,
    pub contact_use_src_port: pj_bool_t,
    pub allow_via_rewrite: pj_bool_t,
    pub allow_sdp_nat_rewrite: pj_bool_t,
    pub use_rfc5626: c_uint,
    pub rfc5626_instance_id: pj_str_t,
    pub rfc5626_reg_id: pj_str_t,
    pub ka_interval: c_uint,
    pub ka_data: pj_str_t,
    pub vid_in_auto_show: pj_bool_t,
    pub vid_out_auto_transmit: pj_bool_t,
    pub vid_wnd_flags: c_uint,
    pub vid_cap_dev: pjmedia_vid_dev_index,
    pub vid_rend_dev: pjmedia_vid_dev_index,
    pub vid_stream_rc_cfg: pjmedia_vid_stream_rc_config,
    pub vid_stream_sk_cfg: pjmedia_vid_stream_sk_config,
    pub rtp_cfg: pjsua_transport_config,
    pub nat64_opt: pjsua_nat64_opt,
    pub ipv6_media_use: pjsua_ipv6_use,
    pub sip_stun_use: pjsua_stun_use,
    pub media_stun_use: pjsua_stun_use,
    pub use_loop_med_tp: pj_bool_t,
    pub enable_loopback: pj_bool_t,
    pub ice_cfg_use: pjsua_ice_config_use,
    pub ice_cfg: pjsua_ice_config,
    pub turn_cfg_use: pjsua_turn_config_use,
    pub turn_cfg: pjsua_turn_config,
    pub use_srtp: pjmedia_srtp_use,
    pub srtp_secure_signaling: c_int,
    pub srtp_optional_dup_offer: pj_bool_t,
    pub srtp_opt: pjsua_srtp_opt,
    pub reg_retry_interval: c_uint,
    pub reg_first_retry_interval: c_uint,
    pub reg_retry_random_interval: c_uint,
    pub drop_calls_on_reg_fail: pj_bool_t,
    pub reg_use_proxy: c_uint,
    pub call_hold_type: pjsua_call_hold_type,
    pub register_on_acc_add: pj_bool_t,
    pub ip_change_cfg: pjsua_ip_change_acc_cfg,
    pub enable_rtcp_mux: pj_bool_t,
    pub rtcp_fb_cfg: pjmedia_rtcp_fb_setting,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_acc_info {
    pub id: pjsua_acc_id,
    pub is_default: pj_bool_t,
    pub acc_uri: pj_str_t,
    pub has_registration: pj_bool_t,
    pub expires: c_uint,
    pub status: pjsip_status_code,
    pub reg_last_err: pj_status_t,
    pub status_text: pj_str_t,
    pub online_status: pj_bool_t,
    pub online_status_text: pj_str_t,
    pub rpid: pjrpid_element,
    pub buf_: [c_char; 80usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_call_media_info {
    pub index: c_uint,
    pub type_: pjmedia_type,
    pub dir: pjmedia_dir,
    pub status: pjsua_call_media_status,
    pub stream: pjsua_call_media_info__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsua_call_media_info__bindgen_ty_1 {
    pub aud: pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1,
    pub vid: pjsua_call_media_info__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 {
    pub conf_slot: pjsua_conf_port_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_call_media_info__bindgen_ty_1__bindgen_ty_2 {
    pub win_in: pjsua_vid_win_id,
    pub dec_slot: pjsua_conf_port_id,
    pub enc_slot: pjsua_conf_port_id,
    pub cap_dev: pjmedia_vid_dev_index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_call_info {
    pub id: pjsua_call_id,
    pub role: pjsip_role_e,
    pub acc_id: pjsua_acc_id,
    pub local_info: pj_str_t,
    pub local_contact: pj_str_t,
    pub remote_info: pj_str_t,
    pub remote_contact: pj_str_t,
    pub call_id: pj_str_t,
    pub setting: pjsua_call_setting,
    pub state: pjsip_inv_state,
    pub state_text: pj_str_t,
    pub last_status: pjsip_status_code,
    pub last_status_text: pj_str_t,
    pub media_status: pjsua_call_media_status,
    pub media_dir: pjmedia_dir,
    pub conf_slot: pjsua_conf_port_id,
    pub media_cnt: c_uint,
    pub media: [pjsua_call_media_info; 16usize],
    pub prov_media_cnt: c_uint,
    pub prov_media: [pjsua_call_media_info; 16usize],
    pub connect_duration: pj_time_val,
    pub total_duration: pj_time_val,
    pub rem_offerer: pj_bool_t,
    pub rem_aud_cnt: c_uint,
    pub rem_vid_cnt: c_uint,
    pub buf_: pjsua_call_info__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_call_info__bindgen_ty_1 {
    pub local_info: [c_char; 256usize],
    pub local_contact: [c_char; 256usize],
    pub remote_info: [c_char; 256usize],
    pub remote_contact: [c_char; 256usize],
    pub call_id: [c_char; 128usize],
    pub last_status_text: [c_char; 128usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_call_vid_strm_op_param {
    pub med_idx: c_int,
    pub dir: pjmedia_dir,
    pub cap_dev: pjmedia_vid_dev_index,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_call_send_dtmf_param {
    pub method: pjsua_dtmf_method,
    pub duration: c_uint,
    pub digits: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_buddy_config {
    pub uri: pj_str_t,
    pub subscribe: pj_bool_t,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_buddy_info {
    pub id: pjsua_buddy_id,
    pub uri: pj_str_t,
    pub contact: pj_str_t,
    pub status: pjsua_buddy_status,
    pub status_text: pj_str_t,
    pub monitor_pres: pj_bool_t,
    pub sub_state: pjsip_evsub_state,
    pub sub_state_name: *const c_char,
    pub sub_term_code: c_uint,
    pub sub_term_reason: pj_str_t,
    pub rpid: pjrpid_element,
    pub pres_status: pjsip_pres_status,
    pub buf_: [c_char; 512usize],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_media_config {
    pub clock_rate: c_uint,
    pub snd_clock_rate: c_uint,
    pub channel_count: c_uint,
    pub audio_frame_ptime: c_uint,
    pub max_media_ports: c_uint,
    pub has_ioqueue: pj_bool_t,
    pub thread_cnt: c_uint,
    pub quality: c_uint,
    pub ptime: c_uint,
    pub no_vad: pj_bool_t,
    pub ilbc_mode: c_uint,
    pub tx_drop_pct: c_uint,
    pub rx_drop_pct: c_uint,
    pub ec_options: c_uint,
    pub ec_tail_len: c_uint,
    pub snd_rec_latency: c_uint,
    pub snd_play_latency: c_uint,
    pub jb_init: c_int,
    pub jb_min_pre: c_int,
    pub jb_max_pre: c_int,
    pub jb_max: c_int,
    pub jb_discard_algo: pjmedia_jb_discard_algo,
    pub enable_ice: pj_bool_t,
    pub ice_max_host_cands: c_int,
    pub ice_opt: pj_ice_sess_options,
    pub ice_no_rtcp: pj_bool_t,
    pub ice_always_update: pj_bool_t,
    pub enable_turn: pj_bool_t,
    pub turn_server: pj_str_t,
    pub turn_conn_type: pj_turn_tp_type,
    pub turn_auth_cred: pj_stun_auth_cred,
    pub turn_tls_setting: pj_turn_sock_tls_cfg,
    pub snd_auto_close_time: c_int,
    pub vid_preview_enable_native: pj_bool_t,
    pub no_smart_media_update: pj_bool_t,
    pub no_rtcp_sdes_bye: pj_bool_t,
    pub on_aud_prev_play_frame:
        Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
    pub on_aud_prev_rec_frame:
        Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_codec_info {
    pub codec_id: pj_str_t,
    pub priority: pj_uint8_t,
    pub desc: pj_str_t,
    pub buf_: [c_char; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_conf_port_info {
    pub slot_id: pjsua_conf_port_id,
    pub name: pj_str_t,
    pub format: pjmedia_format,
    pub clock_rate: c_uint,
    pub channel_count: c_uint,
    pub samples_per_frame: c_uint,
    pub bits_per_sample: c_uint,
    pub tx_level_adj: f32,
    pub rx_level_adj: f32,
    pub listener_cnt: c_uint,
    pub listeners: [pjsua_conf_port_id; 254usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_media_transport {
    pub skinfo: pjmedia_sock_info,
    pub transport: *mut pjmedia_transport,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_snd_dev_param {
    pub capture_dev: c_int,
    pub playback_dev: c_int,
    pub mode: c_uint,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_conf_connect_param {
    pub level: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsua_ext_snd_dev {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_vid_preview_param {
    pub rend_id: pjmedia_vid_dev_index,
    pub show: pj_bool_t,
    pub wnd_flags: c_uint,
    pub format: pjmedia_format,
    pub wnd: pjmedia_vid_dev_hwnd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_vid_win_info {
    pub is_native: pj_bool_t,
    pub hwnd: pjmedia_vid_dev_hwnd,
    pub rdr_dev: pjmedia_vid_dev_index,
    pub slot_id: pjsua_conf_port_id,
    pub show: pj_bool_t,
    pub pos: pjmedia_coord,
    pub size: pjmedia_rect_size,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsua_vid_conf_port_info {
    pub slot_id: pjsua_conf_port_id,
    pub name: pj_str_t,
    pub format: pjmedia_format,
    pub listener_cnt: c_uint,
    pub listeners: [pjsua_conf_port_id; 254usize],
    pub transmitter_cnt: c_uint,
    pub transmitters: [pjsua_conf_port_id; 254usize],
}

extern "C" {
    pub fn pjsua_logging_config_default(cfg: *mut pjsua_logging_config);
    pub fn pjsua_logging_config_dup( pool: *mut pj_pool_t, dst: *mut pjsua_logging_config, src: *const pjsua_logging_config, );
    pub fn pjsua_config_default(cfg: *mut pjsua_config);
    pub fn pjsua_config_dup(pool: *mut pj_pool_t, dst: *mut pjsua_config, src: *const pjsua_config);
    pub fn pjsua_msg_data_init(msg_data: *mut pjsua_msg_data);
    pub fn pjsua_msg_data_clone( pool: *mut pj_pool_t, rhs: *const pjsua_msg_data, ) -> *mut pjsua_msg_data;
    pub fn pjsua_create() -> pj_status_t;
    pub fn pjsua_init( ua_cfg: *const pjsua_config, log_cfg: *const pjsua_logging_config, media_cfg: *const pjsua_media_config, ) -> pj_status_t;
    pub fn pjsua_start() -> pj_status_t;
    pub fn pjsua_destroy() -> pj_status_t;
    pub fn pjsua_get_state() -> pjsua_state;
    pub fn pjsua_destroy2(flags: c_uint) -> pj_status_t;
    pub fn pjsua_handle_events(msec_timeout: c_uint) -> c_int;
    pub fn pjsua_stop_worker_threads();
    pub fn pjsua_pool_create( name: *const c_char, init_size: pj_size_t, increment: pj_size_t, ) -> *mut pj_pool_t;
    pub fn pjsua_reconfigure_logging(c: *const pjsua_logging_config) -> pj_status_t;
    pub fn pjsua_get_pjsip_endpt() -> *mut pjsip_endpoint;
    pub fn pjsua_get_pjmedia_endpt() -> *mut pjmedia_endpt;
    pub fn pjsua_get_pool_factory() -> *mut pj_pool_factory;
    pub fn pjsua_ip_change_param_default(param: *mut pjsua_ip_change_param);
    pub fn pjsua_detect_nat_type() -> pj_status_t;
    pub fn pjsua_get_nat_type(type_: *mut pj_stun_nat_type) -> pj_status_t;
    pub fn pjsua_update_stun_servers( count: c_uint, srv: *mut pj_str_t, wait: pj_bool_t, ) -> pj_status_t;
    pub fn pjsua_resolve_stun_servers( count: c_uint, srv: *mut pj_str_t, wait: pj_bool_t, token: *mut c_void, cb: pj_stun_resolve_cb, ) -> pj_status_t;
    pub fn pjsua_cancel_stun_resolution( token: *mut c_void, notify_cb: pj_bool_t, ) -> pj_status_t;
    pub fn pjsua_verify_sip_url(url: *const c_char) -> pj_status_t;
    pub fn pjsua_verify_url(url: *const c_char) -> pj_status_t;
    pub fn pjsua_schedule_timer_dbg( entry: *mut pj_timer_entry, delay: *const pj_time_val, src_file: *const c_char, src_line: c_int, ) -> pj_status_t;
    pub fn pjsua_schedule_timer2_dbg( cb: Option<unsafe extern "C" fn(user_data: *mut c_void)>, user_data: *mut c_void, msec_delay: c_uint, src_file: *const c_char, src_line: c_int, ) -> pj_status_t;
    pub fn pjsua_cancel_timer(entry: *mut pj_timer_entry);
    pub fn pjsua_perror( sender: *const c_char, title: *const c_char, status: pj_status_t, );
    pub fn pjsua_dump(detail: pj_bool_t);
    pub fn pjsua_handle_ip_change(param: *const pjsua_ip_change_param) -> pj_status_t;
    pub fn pjsua_transport_config_default(cfg: *mut pjsua_transport_config);
    pub fn pjsua_transport_config_dup( pool: *mut pj_pool_t, dst: *mut pjsua_transport_config, src: *const pjsua_transport_config, );
    pub fn pjsua_transport_create( type_: pjsip_transport_type_e, cfg: *const pjsua_transport_config, p_id: *mut pjsua_transport_id, ) -> pj_status_t;
    pub fn pjsua_transport_register( tp: *mut pjsip_transport, p_id: *mut pjsua_transport_id, ) -> pj_status_t;
    pub fn pjsua_tpfactory_register( tf: *mut pjsip_tpfactory, p_id: *mut pjsua_transport_id, ) -> pj_status_t;
    pub fn pjsua_enum_transports( id: *mut pjsua_transport_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_transport_get_info( id: pjsua_transport_id, info: *mut pjsua_transport_info, ) -> pj_status_t;
    pub fn pjsua_transport_set_enable(id: pjsua_transport_id, enabled: pj_bool_t) -> pj_status_t;
    pub fn pjsua_transport_close(id: pjsua_transport_id, force: pj_bool_t) -> pj_status_t;
    pub fn pjsua_transport_lis_start( id: pjsua_transport_id, cfg: *const pjsua_transport_config, ) -> pj_status_t;
    pub fn pjsua_ice_config_from_media_config( pool: *mut pj_pool_t, dst: *mut pjsua_ice_config, src: *const pjsua_media_config, );
    pub fn pjsua_ice_config_dup( pool: *mut pj_pool_t, dst: *mut pjsua_ice_config, src: *const pjsua_ice_config, );
    pub fn pjsua_turn_config_from_media_config( pool: *mut pj_pool_t, dst: *mut pjsua_turn_config, src: *const pjsua_media_config, );
    pub fn pjsua_turn_config_dup( pool: *mut pj_pool_t, dst: *mut pjsua_turn_config, src: *const pjsua_turn_config, );
    pub fn pjsua_srtp_opt_default(cfg: *mut pjsua_srtp_opt);
    pub fn pjsua_srtp_opt_dup( pool: *mut pj_pool_t, dst: *mut pjsua_srtp_opt, src: *const pjsua_srtp_opt, check_str: pj_bool_t, );
    pub fn pjsua_acc_config_default(cfg: *mut pjsua_acc_config);
    pub fn pjsua_acc_config_dup( pool: *mut pj_pool_t, dst: *mut pjsua_acc_config, src: *const pjsua_acc_config, );
    pub fn pjsua_acc_get_count() -> c_uint;
    pub fn pjsua_acc_is_valid(acc_id: pjsua_acc_id) -> pj_bool_t;
    pub fn pjsua_acc_set_default(acc_id: pjsua_acc_id) -> pj_status_t;
    pub fn pjsua_acc_get_default() -> pjsua_acc_id;
    pub fn pjsua_acc_add( acc_cfg: *const pjsua_acc_config, is_default: pj_bool_t, p_acc_id: *mut pjsua_acc_id, ) -> pj_status_t;
    pub fn pjsua_acc_add_local( tid: pjsua_transport_id, is_default: pj_bool_t, p_acc_id: *mut pjsua_acc_id, ) -> pj_status_t;
    pub fn pjsua_acc_set_user_data( acc_id: pjsua_acc_id, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_acc_get_user_data(acc_id: pjsua_acc_id) -> *mut c_void;
    pub fn pjsua_acc_del(acc_id: pjsua_acc_id) -> pj_status_t;
    pub fn pjsua_acc_get_config( acc_id: pjsua_acc_id, pool: *mut pj_pool_t, acc_cfg: *mut pjsua_acc_config, ) -> pj_status_t;
    pub fn pjsua_acc_modify(acc_id: pjsua_acc_id, acc_cfg: *const pjsua_acc_config) -> pj_status_t;
    pub fn pjsua_acc_set_online_status(acc_id: pjsua_acc_id, is_online: pj_bool_t) -> pj_status_t;
    pub fn pjsua_acc_set_online_status2( acc_id: pjsua_acc_id, is_online: pj_bool_t, pr: *const pjrpid_element, ) -> pj_status_t;
    pub fn pjsua_acc_set_registration(acc_id: pjsua_acc_id, renew: pj_bool_t) -> pj_status_t;
    pub fn pjsua_acc_get_info(acc_id: pjsua_acc_id, info: *mut pjsua_acc_info) -> pj_status_t;
    pub fn pjsua_enum_accs( ids: *mut pjsua_acc_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_acc_enum_info( info: *mut pjsua_acc_info, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_acc_find_for_outgoing(url: *const pj_str_t) -> pjsua_acc_id;
    pub fn pjsua_acc_find_for_incoming(rdata: *mut pjsip_rx_data) -> pjsua_acc_id;
    pub fn pjsua_acc_create_request( acc_id: pjsua_acc_id, method: *const pjsip_method, target: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsua_acc_create_uac_contact( pool: *mut pj_pool_t, contact: *mut pj_str_t, acc_id: pjsua_acc_id, uri: *const pj_str_t, ) -> pj_status_t;
    pub fn pjsua_acc_create_uas_contact( pool: *mut pj_pool_t, contact: *mut pj_str_t, acc_id: pjsua_acc_id, rdata: *mut pjsip_rx_data,) -> pj_status_t;
    pub fn pjsua_acc_set_transport(acc_id: pjsua_acc_id, tp_id: pjsua_transport_id) -> pj_status_t;
    pub fn pjsua_call_setting_default(opt: *mut pjsua_call_setting);
    pub fn pjsua_call_vid_strm_op_param_default(param: *mut pjsua_call_vid_strm_op_param);
    pub fn pjsua_call_send_dtmf_param_default(param: *mut pjsua_call_send_dtmf_param);
    pub fn pjsua_call_get_max_count() -> c_uint;
    pub fn pjsua_call_get_count() -> c_uint;
    pub fn pjsua_enum_calls( ids: *mut pjsua_call_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_call_make_call( acc_id: pjsua_acc_id, dst_uri: *const pj_str_t, opt: *const pjsua_call_setting, user_data: *mut c_void, msg_data: *const pjsua_msg_data, p_call_id: *mut pjsua_call_id, ) -> pj_status_t;
    pub fn pjsua_call_is_active(call_id: pjsua_call_id) -> pj_bool_t;
    pub fn pjsua_call_has_media(call_id: pjsua_call_id) -> pj_bool_t;
    pub fn pjsua_call_get_conf_port(call_id: pjsua_call_id) -> pjsua_conf_port_id;
    pub fn pjsua_call_get_vid_win(call_id: pjsua_call_id) -> pjsua_vid_win_id;
    pub fn pjsua_call_get_vid_conf_port( call_id: pjsua_call_id, dir: pjmedia_dir, ) -> pjsua_conf_port_id;
    pub fn pjsua_call_get_info(call_id: pjsua_call_id, info: *mut pjsua_call_info) -> pj_status_t;
    pub fn pjsua_call_remote_has_cap( call_id: pjsua_call_id, htype: c_int, hname: *const pj_str_t, token: *const pj_str_t, ) -> pjsip_dialog_cap_status;
    pub fn pjsua_call_set_user_data( call_id: pjsua_call_id, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_call_get_user_data(call_id: pjsua_call_id) -> *mut c_void;
    pub fn pjsua_call_get_rem_nat_type( call_id: pjsua_call_id, p_type: *mut pj_stun_nat_type, ) -> pj_status_t;
    pub fn pjsua_call_answer( call_id: pjsua_call_id, code: c_uint, reason: *const pj_str_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_answer2( call_id: pjsua_call_id, opt: *const pjsua_call_setting, code: c_uint, reason: *const pj_str_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_answer_with_sdp( call_id: pjsua_call_id, sdp: *const pjmedia_sdp_session, opt: *const pjsua_call_setting, code: c_uint, reason: *const pj_str_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_hangup( call_id: pjsua_call_id, code: c_uint, reason: *const pj_str_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_process_redirect( call_id: pjsua_call_id, cmd: pjsip_redirect_op, ) -> pj_status_t;
    pub fn pjsua_call_set_hold( call_id: pjsua_call_id, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_set_hold2( call_id: pjsua_call_id, options: c_uint, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_reinvite( call_id: pjsua_call_id, options: c_uint, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_reinvite2( call_id: pjsua_call_id, opt: *const pjsua_call_setting, msg_data: *const pjsua_msg_data,) -> pj_status_t;
    pub fn pjsua_call_update( call_id: pjsua_call_id, options: c_uint, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_update2( call_id: pjsua_call_id, opt: *const pjsua_call_setting, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_xfer( call_id: pjsua_call_id, dest: *const pj_str_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_xfer_replaces( call_id: pjsua_call_id, dest_call_id: pjsua_call_id, options: c_uint, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_dial_dtmf(call_id: pjsua_call_id, digits: *const pj_str_t) -> pj_status_t;
    pub fn pjsua_call_send_dtmf( call_id: pjsua_call_id, param: *const pjsua_call_send_dtmf_param, ) -> pj_status_t;
    pub fn pjsua_call_send_im( call_id: pjsua_call_id, mime_type: *const pj_str_t, content: *const pj_str_t, msg_data: *const pjsua_msg_data, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_call_send_typing_ind( call_id: pjsua_call_id, is_typing: pj_bool_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_send_request( call_id: pjsua_call_id, method: *const pj_str_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_call_hangup_all();
    pub fn pjsua_call_dump( call_id: pjsua_call_id, with_media: pj_bool_t, buffer: *mut c_char, maxlen: c_uint, indent: *const c_char, ) -> pj_status_t;
    pub fn pjsua_call_get_vid_stream_idx(call_id: pjsua_call_id) -> c_int;
    pub fn pjsua_call_vid_stream_is_running( call_id: pjsua_call_id, med_idx: c_int, dir: pjmedia_dir, ) -> pj_bool_t;
    pub fn pjsua_call_set_vid_strm( call_id: pjsua_call_id, op: pjsua_call_vid_strm_op, param: *const pjsua_call_vid_strm_op_param, ) -> pj_status_t;
    pub fn pjsua_call_get_stream_info( call_id: pjsua_call_id, med_idx: c_uint, psi: *mut pjsua_stream_info, ) -> pj_status_t;
    pub fn pjsua_call_get_stream_stat( call_id: pjsua_call_id, med_idx: c_uint, stat: *mut pjsua_stream_stat, ) -> pj_status_t;
    pub fn pjsua_call_get_med_transport_info( call_id: pjsua_call_id, med_idx: c_uint, t: *mut pjmedia_transport_info, ) -> pj_status_t;
    pub fn pjsua_buddy_config_default(cfg: *mut pjsua_buddy_config);
    pub fn pjsua_get_buddy_count() -> c_uint;
    pub fn pjsua_buddy_is_valid(buddy_id: pjsua_buddy_id) -> pj_bool_t;
    pub fn pjsua_enum_buddies( ids: *mut pjsua_buddy_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_buddy_find(uri: *const pj_str_t) -> pjsua_buddy_id;
    pub fn pjsua_buddy_get_info( buddy_id: pjsua_buddy_id, info: *mut pjsua_buddy_info, ) -> pj_status_t;
    pub fn pjsua_buddy_set_user_data( buddy_id: pjsua_buddy_id, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_buddy_get_user_data(buddy_id: pjsua_buddy_id) -> *mut c_void;
    pub fn pjsua_buddy_add( buddy_cfg: *const pjsua_buddy_config, p_buddy_id: *mut pjsua_buddy_id, ) -> pj_status_t;
    pub fn pjsua_buddy_del(buddy_id: pjsua_buddy_id) -> pj_status_t;
    pub fn pjsua_buddy_subscribe_pres( buddy_id: pjsua_buddy_id, subscribe: pj_bool_t,) -> pj_status_t;
    pub fn pjsua_buddy_update_pres(buddy_id: pjsua_buddy_id) -> pj_status_t;
    pub fn pjsua_pres_notify( acc_id: pjsua_acc_id, srv_pres: *mut pjsua_srv_pres, state: pjsip_evsub_state, state_str: *const pj_str_t, reason: *const pj_str_t, with_body: pj_bool_t, msg_data: *const pjsua_msg_data, ) -> pj_status_t;
    pub fn pjsua_pres_dump(verbose: pj_bool_t);
    pub static pjsip_message_method: pjsip_method;
    pub static pjsip_info_method: pjsip_method;
    pub fn pjsua_im_send( acc_id: pjsua_acc_id, to: *const pj_str_t, mime_type: *const pj_str_t, content: *const pj_str_t, msg_data: *const pjsua_msg_data, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_im_typing( acc_id: pjsua_acc_id, to: *const pj_str_t, is_typing: pj_bool_t, msg_data: *const pjsua_msg_data,) -> pj_status_t;
    pub fn pjsua_media_config_default(cfg: *mut pjsua_media_config);
    pub fn pjsua_snd_dev_param_default(prm: *mut pjsua_snd_dev_param);
    pub fn pjsua_conf_connect_param_default(prm: *mut pjsua_conf_connect_param);
    pub fn pjsua_conf_get_max_ports() -> c_uint;
    pub fn pjsua_conf_get_active_ports() -> c_uint;
    pub fn pjsua_enum_conf_ports( id: *mut pjsua_conf_port_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_conf_get_port_info( port_id: pjsua_conf_port_id, info: *mut pjsua_conf_port_info, ) -> pj_status_t;
    pub fn pjsua_conf_add_port( pool: *mut pj_pool_t, port: *mut pjmedia_port, p_id: *mut pjsua_conf_port_id, ) -> pj_status_t;
    pub fn pjsua_conf_remove_port(port_id: pjsua_conf_port_id) -> pj_status_t;
    pub fn pjsua_conf_connect(source: pjsua_conf_port_id, sink: pjsua_conf_port_id) -> pj_status_t;
    pub fn pjsua_conf_connect2( source: pjsua_conf_port_id, sink: pjsua_conf_port_id, prm: *const pjsua_conf_connect_param, ) -> pj_status_t;
    pub fn pjsua_conf_disconnect( source: pjsua_conf_port_id, sink: pjsua_conf_port_id, ) -> pj_status_t;
    pub fn pjsua_conf_adjust_tx_level(slot: pjsua_conf_port_id, level: f32) -> pj_status_t;
    pub fn pjsua_conf_adjust_rx_level(slot: pjsua_conf_port_id, level: f32) -> pj_status_t;
    pub fn pjsua_conf_get_signal_level( slot: pjsua_conf_port_id, tx_level: *mut c_uint, rx_level: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_player_create( filename: *const pj_str_t, options: c_uint, p_id: *mut pjsua_player_id, ) -> pj_status_t;
    pub fn pjsua_playlist_create( file_names: *const pj_str_t, file_count: c_uint, label: *const pj_str_t, options: c_uint, p_id: *mut pjsua_player_id, ) -> pj_status_t;
    pub fn pjsua_player_get_conf_port(id: pjsua_player_id) -> pjsua_conf_port_id;
    pub fn pjsua_player_get_port( id: pjsua_player_id, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjsua_player_get_info( id: pjsua_player_id, info: *mut pjmedia_wav_player_info, ) -> pj_status_t;
    pub fn pjsua_player_get_pos(id: pjsua_player_id) -> pj_ssize_t;
    pub fn pjsua_player_set_pos(id: pjsua_player_id, samples: pj_uint32_t) -> pj_status_t;
    pub fn pjsua_player_destroy(id: pjsua_player_id) -> pj_status_t;
    pub fn pjsua_recorder_create( filename: *const pj_str_t, enc_type: c_uint, enc_param: *mut c_void, max_size: pj_ssize_t, options: c_uint, p_id: *mut pjsua_recorder_id, ) -> pj_status_t;
    pub fn pjsua_recorder_get_conf_port(id: pjsua_recorder_id) -> pjsua_conf_port_id;
    pub fn pjsua_recorder_get_port( id: pjsua_recorder_id, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjsua_recorder_destroy(id: pjsua_recorder_id) -> pj_status_t;
    pub fn pjsua_enum_aud_devs( info: *mut pjmedia_aud_dev_info, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_enum_snd_devs( info: *mut pjmedia_snd_dev_info, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_get_snd_dev( capture_dev: *mut c_int, playback_dev: *mut c_int, ) -> pj_status_t;
    pub fn pjsua_set_snd_dev( capture_dev: c_int, playback_dev: c_int, ) -> pj_status_t;
    pub fn pjsua_set_snd_dev2(snd_param: *mut pjsua_snd_dev_param) -> pj_status_t;
    pub fn pjsua_set_null_snd_dev() -> pj_status_t;
    pub fn pjsua_set_no_snd_dev() -> *mut pjmedia_port;
    pub fn pjsua_set_ec( tail_ms: c_uint, options: c_uint, ) -> pj_status_t;
    pub fn pjsua_get_ec_tail(p_tail_ms: *mut c_uint) -> pj_status_t;
    pub fn pjsua_get_ec_stat(p_stat: *mut pjmedia_echo_stat) -> pj_status_t;
    pub fn pjsua_snd_is_active() -> pj_bool_t;
    pub fn pjsua_snd_set_setting( cap: pjmedia_aud_dev_cap, pval: *const c_void, keep: pj_bool_t, ) -> pj_status_t;
    pub fn pjsua_snd_get_setting( cap: pjmedia_aud_dev_cap, pval: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_ext_snd_dev_create( param: *mut pjmedia_snd_port_param, p_snd: *mut *mut pjsua_ext_snd_dev, ) -> pj_status_t;
    pub fn pjsua_ext_snd_dev_destroy(snd: *mut pjsua_ext_snd_dev) -> pj_status_t;
    pub fn pjsua_ext_snd_dev_get_snd_port(snd: *mut pjsua_ext_snd_dev) -> *mut pjmedia_snd_port;
    pub fn pjsua_ext_snd_dev_get_conf_port(snd: *mut pjsua_ext_snd_dev) -> pjsua_conf_port_id;
    pub fn pjsua_enum_codecs( id: *mut pjsua_codec_info, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_codec_set_priority(codec_id: *const pj_str_t, priority: pj_uint8_t) -> pj_status_t;
    pub fn pjsua_codec_get_param( codec_id: *const pj_str_t, param: *mut pjmedia_codec_param, ) -> pj_status_t;
    pub fn pjsua_codec_set_param( codec_id: *const pj_str_t, param: *const pjmedia_codec_param, ) -> pj_status_t;
    pub fn pjsua_vid_dev_count() -> c_uint;
    pub fn pjsua_vid_dev_get_info( id: pjmedia_vid_dev_index, vdi: *mut pjmedia_vid_dev_info, ) -> pj_status_t;
    pub fn pjsua_vid_dev_is_active(id: pjmedia_vid_dev_index) -> pj_bool_t;
    pub fn pjsua_vid_dev_set_setting( id: pjmedia_vid_dev_index, cap: pjmedia_vid_dev_cap, pval: *const c_void, keep: pj_bool_t, ) -> pj_status_t;
    pub fn pjsua_vid_dev_get_setting( id: pjmedia_vid_dev_index, cap: pjmedia_vid_dev_cap, pval: *mut c_void, ) -> pj_status_t;
    pub fn pjsua_vid_enum_devs( info: *mut pjmedia_vid_dev_info, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_vid_preview_param_default(p: *mut pjsua_vid_preview_param);
    pub fn pjsua_vid_preview_has_native(id: pjmedia_vid_dev_index) -> pj_bool_t;
    pub fn pjsua_vid_preview_start( id: pjmedia_vid_dev_index, p: *const pjsua_vid_preview_param, ) -> pj_status_t;
    pub fn pjsua_vid_preview_get_win(id: pjmedia_vid_dev_index) -> pjsua_vid_win_id;
    pub fn pjsua_vid_preview_get_vid_conf_port(id: pjmedia_vid_dev_index) -> pjsua_conf_port_id;
    pub fn pjsua_vid_preview_stop(id: pjmedia_vid_dev_index) -> pj_status_t;
    pub fn pjsua_vid_enum_wins( wids: *mut pjsua_vid_win_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_vid_win_get_info( wid: pjsua_vid_win_id, wi: *mut pjsua_vid_win_info, ) -> pj_status_t;
    pub fn pjsua_vid_win_set_show(wid: pjsua_vid_win_id, show: pj_bool_t) -> pj_status_t;
    pub fn pjsua_vid_win_set_pos(wid: pjsua_vid_win_id, pos: *const pjmedia_coord) -> pj_status_t;
    pub fn pjsua_vid_win_set_size( wid: pjsua_vid_win_id, size: *const pjmedia_rect_size, ) -> pj_status_t;
    pub fn pjsua_vid_win_set_win( wid: pjsua_vid_win_id, win: *const pjmedia_vid_dev_hwnd, ) -> pj_status_t;
    pub fn pjsua_vid_win_rotate(wid: pjsua_vid_win_id, angle: c_int) -> pj_status_t;
    pub fn pjsua_vid_win_set_fullscreen(wid: pjsua_vid_win_id, enabled: pj_bool_t) -> pj_status_t;
    pub fn pjsua_vid_enum_codecs( id: *mut pjsua_codec_info, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_vid_codec_set_priority( codec_id: *const pj_str_t, priority: pj_uint8_t, ) -> pj_status_t;
    pub fn pjsua_vid_codec_get_param( codec_id: *const pj_str_t, param: *mut pjmedia_vid_codec_param, ) -> pj_status_t;
    pub fn pjsua_vid_codec_set_param( codec_id: *const pj_str_t, param: *const pjmedia_vid_codec_param, ) -> pj_status_t;
    pub fn pjsua_vid_conf_get_active_ports() -> c_uint;
    pub fn pjsua_vid_conf_enum_ports( id: *mut pjsua_conf_port_id, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsua_vid_conf_get_port_info( port_id: pjsua_conf_port_id, info: *mut pjsua_vid_conf_port_info, ) -> pj_status_t;
    pub fn pjsua_vid_conf_add_port( pool: *mut pj_pool_t, port: *mut pjmedia_port, param: *const c_void, p_id: *mut pjsua_conf_port_id, ) -> pj_status_t;
    pub fn pjsua_vid_conf_remove_port(port_id: pjsua_conf_port_id) -> pj_status_t;
    pub fn pjsua_vid_conf_connect( source: pjsua_conf_port_id, sink: pjsua_conf_port_id, param: *const c_void, ) -> pj_status_t;
    pub fn pjsua_vid_conf_disconnect( source: pjsua_conf_port_id, sink: pjsua_conf_port_id, ) -> pj_status_t;
}

impl AutoCreate<pjsua_srtp_opt> for pjsua_srtp_opt {
    fn new() -> pjsua_srtp_opt {
        pjsua_srtp_opt {
            crypto_count: 0,
            crypto: [pjmedia_srtp_crypto::new(); 16],
            keying_count: 0,
            keying: [0, 0],
        }
    }
}

pub trait PjsuaCallback {
    unsafe extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {}
    unsafe extern "C" fn on_incoming_call(
        acc_id: pjsua_acc_id,
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
    ) {
    }
    unsafe extern "C" fn on_call_tsx_state(
        call_id: pjsua_call_id,
        tsx: *mut pjsip_transaction,
        e: *mut pjsip_event,
    ) {
    }
    unsafe extern "C" fn on_call_media_state(call_id: pjsua_call_id) {}
    unsafe extern "C" fn on_call_sdp_created(
        call_id: pjsua_call_id,
        sdp: *mut pjmedia_sdp_session,
        pool: *mut pj_pool_t,
        rem_sdp: *const pjmedia_sdp_session,
    ) {
    }
    unsafe extern "C" fn on_stream_precreate(
        call_id: pjsua_call_id,
        param: *mut pjsua_on_stream_precreate_param,
    ) {
    }

    unsafe extern "C" fn on_stream_created(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
        p_port: *mut *mut pjmedia_port,
    ) {
    }
    unsafe extern "C" fn on_stream_created2(
        call_id: pjsua_call_id,
        param: *mut pjsua_on_stream_created_param,
    ) {
    }
    unsafe extern "C" fn on_stream_destroyed(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
    ) {
    }
    unsafe extern "C" fn on_dtmf_digit(call_id: pjsua_call_id, digit: c_int) {}
    unsafe extern "C" fn on_dtmf_digit2(call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {}
    unsafe extern "C" fn on_dtmf_event(call_id: pjsua_call_id, event: *const pjsua_dtmf_event) {}
    unsafe extern "C" fn on_call_transfer_request(
        call_id: pjsua_call_id,
        dst: *const pj_str_t,
        code: *mut pjsip_status_code,
    ) {
    }
    unsafe extern "C" fn on_call_transfer_request2(
        call_id: pjsua_call_id,
        dst: *const pj_str_t,
        code: *mut pjsip_status_code,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_transfer_status(
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
    }
    unsafe extern "C" fn on_call_replace_request(
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
        st_code: *mut c_int,
        st_text: *mut pj_str_t,
    ) {
    }
    unsafe extern "C" fn on_call_replace_request2(
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
        st_code: *mut c_int,
        st_text: *mut pj_str_t,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_replaced(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id) {}
    unsafe extern "C" fn on_call_rx_offer(
        call_id: pjsua_call_id,
        offer: *const pjmedia_sdp_session,
        reserved: *mut c_void,
        code: *mut pjsip_status_code,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_rx_reinvite(
        call_id: pjsua_call_id,
        offer: *const pjmedia_sdp_session,
        rdata: *mut pjsip_rx_data,
        reserved: *mut c_void,
        async_: *mut pj_bool_t,
        code: *mut pjsip_status_code,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_tx_offer(
        call_id: pjsua_call_id,
        reserved: *mut c_void,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_reg_started(acc_id: pjsua_acc_id, renew: pj_bool_t) {}
    unsafe extern "C" fn on_reg_started2(acc_id: pjsua_acc_id, info: *mut pjsua_reg_info) {}
    unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {}
    unsafe extern "C" fn on_reg_state2(acc_id: pjsua_acc_id, info: *mut pjsua_reg_info) {}
    unsafe extern "C" fn on_incoming_subscribe(
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data,
    ) {
    }

    unsafe extern "C" fn on_srv_subscribe_state(
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        remote_uri: *const pj_str_t,
        state: pjsip_evsub_state,
        event: *mut pjsip_event,
    ) {
    }

    unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {}

    unsafe extern "C" fn on_buddy_evsub_state(
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
    }

    unsafe extern "C" fn on_pager(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
    ) {
    }

    unsafe extern "C" fn on_pager2(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        acc_id: pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_pager_status(
        call_id: pjsua_call_id,
        to: *const pj_str_t,
        body: *const pj_str_t,
        user_data: *mut c_void,
        status: pjsip_status_code,
        reason: *const pj_str_t,
    ) {
    }

    unsafe extern "C" fn on_pager_status2(
        call_id: pjsua_call_id,
        to: *const pj_str_t,
        body: *const pj_str_t,
        user_data: *mut c_void,
        status: pjsip_status_code,
        reason: *const pj_str_t,
        tdata: *mut pjsip_tx_data,
        rdata: *mut pjsip_rx_data,
        acc_id: pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_typing(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
    }

    unsafe extern "C" fn on_typing2(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
        rdata: *mut pjsip_rx_data,
        acc_id: pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_nat_detect(res: *const pj_stun_nat_detect_result) {}

    unsafe extern "C" fn on_call_redirected(
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        0
    }

    unsafe extern "C" fn on_mwi_state(acc_id: pjsua_acc_id, evsub: *mut pjsip_evsub) {}

    unsafe extern "C" fn on_mwi_info(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {}

    unsafe extern "C" fn on_transport_state(
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
    }

    unsafe extern "C" fn on_call_media_transport_state(
        call_id: pjsua_call_id,
        info: *const pjsua_med_tp_state_info,
    ) -> pj_status_t {
        0
    }

    unsafe extern "C" fn on_ice_transport_error(
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
    }

    unsafe extern "C" fn on_snd_dev_operation(operation: c_int) -> pj_status_t {
        0
    }

    unsafe extern "C" fn on_call_media_event(
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event,
    ) {
    }

    unsafe extern "C" fn on_create_media_transport(
        call_id: pjsua_call_id,
        media_idx: c_uint,
        base_tp: *mut pjmedia_transport,
        flags: c_uint,
    ) -> *mut pjmedia_transport {
        let mut transport: pjmedia_transport = pjmedia_transport::new();
        &mut transport as *mut _
    }

    unsafe extern "C" fn on_create_media_transport_srtp(
        call_id: pjsua_call_id,
        media_idx: c_uint,
        srtp_opt: *mut pjmedia_srtp_setting,
    ) {
    }

    unsafe extern "C" fn on_acc_find_for_incoming(
        rdata: *const pjsip_rx_data,
        acc_id: *mut pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_stun_resolution_complete(result: *const pj_stun_resolve_result) {}

    unsafe extern "C" fn on_ip_change_progress(
        op: pjsua_ip_change_op,
        status: pj_status_t,
        info: *const pjsua_ip_change_op_info,
    ) {
    }
    unsafe extern "C" fn on_media_event(event: *mut pjmedia_event) {}
}

impl AutoCreate<pjsua_callback> for pjsua_callback {
    fn new() -> pjsua_callback {
        pjsua_callback {
            on_call_state: None,
            on_incoming_call: None,
            on_call_tsx_state: None,
            on_call_media_state: None,
            on_call_sdp_created: None,
            on_stream_precreate: None,
            on_stream_created: None,
            on_stream_created2: None,
            on_stream_destroyed: None,
            on_dtmf_digit: None,
            on_dtmf_digit2: None,
            on_dtmf_event: None,
            on_call_transfer_request: None,
            on_call_transfer_request2: None,
            on_call_transfer_status: None,
            on_call_replace_request: None,
            on_call_replace_request2: None,
            on_call_replaced: None,
            on_call_rx_offer: None,
            on_call_rx_reinvite: None,
            on_call_tx_offer: None,
            on_reg_started: None,
            on_reg_started2: None,
            on_reg_state: None,
            on_reg_state2: None,
            on_incoming_subscribe: None,
            on_srv_subscribe_state: None,
            on_buddy_state: None,
            on_buddy_evsub_state: None,
            on_pager: None,
            on_pager2: None,
            on_pager_status: None,
            on_pager_status2: None,
            on_typing: None,
            on_typing2: None,
            on_nat_detect: None,
            on_call_redirected: None,
            on_mwi_state: None,
            on_mwi_info: None,
            on_transport_state: None as pjsip_tp_state_callback,
            on_call_media_transport_state: None as pjsua_med_tp_state_cb,
            on_ice_transport_error: None,
            on_snd_dev_operation: None,
            on_call_media_event: None,
            on_create_media_transport: None,
            on_create_media_transport_srtp: None,
            on_acc_find_for_incoming: None,
            on_stun_resolution_complete: None as pj_stun_resolve_cb,
            on_ip_change_progress: None,
            on_media_event: None,
        }
    }
}

impl AutoCreate<pjsua_logging_config> for pjsua_logging_config {
    fn new() -> pjsua_logging_config {
        let mut config = pjsua_logging_config {
            msg_logging: pj_constants__PJ_FALSE as pj_bool_t,
            level: 0,
            console_level: 0,
            decor: 0,
            log_filename: pj_str_t::new(),
            log_file_flags: 0,
            cb: None,
        };

        unsafe {
            pjsua_logging_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_config> for pjsua_config {
    fn new() -> pjsua_config {
        let mut config = pjsua_config {
            max_calls: 0,
            thread_cnt: 0,
            nameserver_count: 0,
            nameserver: [pj_str_t::new(); 4],
            force_lr: 0,
            outbound_proxy_cnt: 0,
            outbound_proxy: [pj_str_t::new(); 4],
            stun_domain: pj_str_t::new(),
            stun_host: pj_str_t::new(),
            stun_srv_cnt: 0,
            stun_srv: [pj_str_t::new(); 8],
            stun_try_ipv6: pj_constants__PJ_FALSE as pj_bool_t,
            stun_ignore_failure: pj_constants__PJ_FALSE as pj_bool_t,
            stun_map_use_stun2: pj_constants__PJ_FALSE as pj_bool_t,
            nat_type_in_sdp: 0,
            require_100rel: pj_constants__PJ_FALSE,
            use_timer: pj_constants__PJ_FALSE,
            enable_unsolicited_mwi: pj_constants__PJ_FALSE as pj_bool_t,
            timer_setting: pjsip_timer_setting {
                min_se: 0,
                sess_expires: 0,
            },
            cred_count: 0,
            cred_info: [pjsip_cred_info::new(); 8],
            cb: pjsua_callback::new(),
            user_agent: pj_str_t::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: pj_constants__PJ_FALSE as pj_bool_t,
            srtp_opt: pjsua_srtp_opt::new(),
            hangup_forked_call: pj_constants__PJ_FALSE as pj_bool_t,
        };

        // set with pjsua default
        unsafe {
            pjsua_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_media_config> for pjsua_media_config {
    fn new() -> pjsua_media_config {
        let mut config = pjsua_media_config {
            clock_rate: 0,
            snd_clock_rate: 0,
            channel_count: 0,
            audio_frame_ptime: 0,
            max_media_ports: 0,
            has_ioqueue: pj_constants__PJ_FALSE as pj_bool_t,
            thread_cnt: 0,
            quality: 0,
            ptime: 0,
            no_vad: pj_constants__PJ_FALSE as pj_bool_t,
            ilbc_mode: 0,
            tx_drop_pct: 0,
            rx_drop_pct: 0,
            ec_options: 0,
            ec_tail_len: 0,
            snd_rec_latency: 0,
            snd_play_latency: 0,
            jb_init: 0,
            jb_min_pre: 0,
            jb_max_pre: 0,
            jb_max: 0,
            jb_discard_algo: 0,
            enable_ice: pj_constants__PJ_FALSE as pj_bool_t,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: pj_constants__PJ_FALSE as pj_bool_t,
            ice_always_update: pj_constants__PJ_FALSE as pj_bool_t,
            enable_turn: pj_constants__PJ_FALSE as pj_bool_t,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0 as pj_turn_tp_type,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
            snd_auto_close_time: 0,
            vid_preview_enable_native: pj_constants__PJ_FALSE as pj_bool_t,
            no_smart_media_update: pj_constants__PJ_FALSE as pj_bool_t,
            no_rtcp_sdes_bye: pj_constants__PJ_FALSE as pj_bool_t,
            on_aud_prev_play_frame: None,
            on_aud_prev_rec_frame: None,
        };

        unsafe {
            pjsua_media_config_default(&mut config as *mut _);
        }
        
        config
    }
}

impl AutoCreate<pjsip_publishc_opt> for pjsip_publishc_opt {
    fn new() -> pjsip_publishc_opt {
        pjsip_publishc_opt {
            queue_request: pj_constants__PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pjsip_auth_clt_pref> for pjsip_auth_clt_pref {
    fn new() -> pjsip_auth_clt_pref {
        pjsip_auth_clt_pref {
            initial_auth: pj_constants__PJ_FALSE as pj_bool_t,
            algorithm: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjsip_timer_setting> for pjsip_timer_setting {
    fn new() -> pjsip_timer_setting {
        pjsip_timer_setting {
            min_se: 0,
            sess_expires: 0,
        }
    }
}

impl AutoCreate<pjsua_transport_config> for pjsua_transport_config {
    fn new() -> pjsua_transport_config {
        let mut config = pjsua_transport_config {
            port: 0,
            port_range: 0,
            public_addr: pj_str_t::new(),
            bound_addr: pj_str_t::new(),
            tls_setting: pjsip_tls_setting::new(),
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            sockopt_params: pj_sockopt_params::new(),
        };

        unsafe {
            pjsua_transport_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_ice_config> for pjsua_ice_config {
    fn new() -> pjsua_ice_config {
        pjsua_ice_config {
            enable_ice: pj_constants__PJ_FALSE as pj_bool_t,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: pj_constants__PJ_FALSE as pj_bool_t,
            ice_always_update: pj_constants__PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pjsua_turn_config> for pjsua_turn_config {
    fn new() -> pjsua_turn_config {
        pjsua_turn_config {
            enable_turn: pj_constants__PJ_FALSE as pj_bool_t,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
        }
    }
}

impl AutoCreate<pjsua_ip_change_acc_cfg> for pjsua_ip_change_acc_cfg {
    fn new() -> pjsua_ip_change_acc_cfg {
        pjsua_ip_change_acc_cfg {
            shutdown_tp: pj_constants__PJ_FALSE as pj_bool_t,
            hangup_calls: pj_constants__PJ_FALSE as pj_bool_t,
            reinvite_flags: 0,
        }
    }
}

impl AutoCreate<pjsua_acc_config> for pjsua_acc_config {
    fn new() -> pjsua_acc_config {
        let mut config = pjsua_acc_config {
            user_data: ptr::null_mut(),
            priority: 0,
            id: pj_str_t::new(),
            reg_uri: pj_str_t::new(),
            reg_hdr_list: pjsip_hdr::new(),
            reg_contact_params: pj_str_t::new(),
            sub_hdr_list: pjsip_hdr::new(),
            mwi_enabled: pj_constants__PJ_FALSE as pj_bool_t,
            mwi_expires: 0,
            publish_enabled: pj_constants__PJ_FALSE as pj_bool_t,
            publish_opt: pjsip_publishc_opt::new(),
            unpublish_max_wait_time_msec: 0,
            auth_pref: pjsip_auth_clt_pref::new(),
            pidf_tuple_id: pj_str_t::new(),
            force_contact: pj_str_t::new(),
            contact_params: pj_str_t::new(),
            contact_uri_params: pj_str_t::new(),
            require_100rel: 0,
            use_timer: 0,
            timer_setting: pjsip_timer_setting::new(),
            proxy_cnt: 0,
            proxy: [pj_str_t::new(); 8],
            lock_codec: 0,
            reg_timeout: 0,
            reg_delay_before_refresh: 0,
            unreg_timeout: 0,
            cred_count: 0,
            cred_info: [pjsip_cred_info::new(); 8],
            transport_id: 0,
            allow_contact_rewrite: pj_constants__PJ_FALSE as pj_bool_t,
            contact_rewrite_method: 0,
            contact_use_src_port: pj_constants__PJ_FALSE as pj_bool_t,
            allow_via_rewrite: pj_constants__PJ_FALSE as pj_bool_t,
            allow_sdp_nat_rewrite: pj_constants__PJ_FALSE as pj_bool_t,
            use_rfc5626: 0,
            rfc5626_instance_id: pj_str_t::new(),
            rfc5626_reg_id: pj_str_t::new(),
            ka_interval: 0,
            ka_data: pj_str_t::new(),
            vid_in_auto_show: pj_constants__PJ_FALSE as pj_bool_t,
            vid_out_auto_transmit: pj_constants__PJ_FALSE as pj_bool_t,
            vid_wnd_flags: 0,
            vid_cap_dev: 0,
            vid_rend_dev: 0,
            vid_stream_rc_cfg: pjmedia_vid_stream_rc_config::new(),
            vid_stream_sk_cfg: pjmedia_vid_stream_sk_config::new(),
            rtp_cfg: pjsua_transport_config::new(),
            nat64_opt: 0,
            ipv6_media_use: 0,
            sip_stun_use: 0,
            media_stun_use: 0,
            use_loop_med_tp: pj_constants__PJ_FALSE as pj_bool_t,
            enable_loopback: pj_constants__PJ_FALSE as pj_bool_t,
            ice_cfg_use: 0,
            ice_cfg: pjsua_ice_config::new(),
            turn_cfg_use: 0,
            turn_cfg: pjsua_turn_config::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: pj_constants__PJ_FALSE as pj_bool_t,
            srtp_opt: pjsua_srtp_opt::new(),
            reg_retry_interval: 0,
            reg_first_retry_interval: 0,
            reg_retry_random_interval: 0,
            drop_calls_on_reg_fail: pj_constants__PJ_FALSE as pj_bool_t,
            reg_use_proxy: 0,
            call_hold_type: 0,
            register_on_acc_add: pj_constants__PJ_FALSE as pj_bool_t,
            ip_change_cfg: pjsua_ip_change_acc_cfg::new(),
            enable_rtcp_mux: pj_constants__PJ_FALSE as pj_bool_t,
            rtcp_fb_cfg: pjmedia_rtcp_fb_setting::new(),
        };

        unsafe {
            pjsua_acc_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_buddy_config> for pjsua_buddy_config {
    fn new() -> pjsua_buddy_config {
        let mut config = pjsua_buddy_config {
            uri: pj_str_t::new(),
            subscribe: pj_constants__PJ_FALSE as pj_bool_t,
            user_data: ptr::null_mut(),
        };

        unsafe {
            pjsua_buddy_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_transport_info> for pjsua_transport_info {
    fn new() -> pjsua_transport_info {
        pjsua_transport_info {
            id: -1,
            type_: 0,
            type_name: pj_str_t::new(),
            info: pj_str_t::new(),
            flag: 0,
            addr_len: 0,
            local_addr: pj_sockaddr::new(),
            local_name: pjsip_host_port::new(),
            usage_count: 0,
        }
    }
}

impl AutoCreate<pjsua_acc_info> for pjsua_acc_info {
    fn new() -> pjsua_acc_info {
        pjsua_acc_info {
            id: -1,
            is_default: pj_constants__PJ_FALSE as pj_bool_t,
            acc_uri: pj_str_t::new(),
            has_registration: pj_constants__PJ_FALSE as pj_bool_t,
            expires: 0,
            status: 0,
            reg_last_err: pj_constants__PJ_FALSE as pj_status_t,
            status_text: pj_str_t::new(),
            online_status: pj_constants__PJ_FALSE as pj_bool_t,
            online_status_text: pj_str_t::new(),
            rpid: pjrpid_element::new(),
            buf_: [0; 80],
        }
    }
}

impl AutoCreate<pjsua_call_setting> for pjsua_call_setting {
    fn new() -> pjsua_call_setting {
        pjsua_call_setting {
            flag: 0,
            req_keyframe_method: 0,
            aud_cnt: 0,
            vid_cnt: 0,
        }
    }
}

impl AutoCreate<pjsua_call_info__bindgen_ty_1> for pjsua_call_info__bindgen_ty_1 {
    fn new() -> pjsua_call_info__bindgen_ty_1 {
        pjsua_call_info__bindgen_ty_1 {
            local_info: [0; 256],
            local_contact: [0; 256],
            remote_info: [0; 256],
            remote_contact: [0; 256],
            call_id: [0; 128],
            last_status_text: [0; 128],
        }
    }
}

impl AutoCreate<pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1>
    for pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1
{
    fn new() -> pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 {
        pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 { conf_slot: 0 }
    }
}

impl AutoCreate<pjsua_call_media_info__bindgen_ty_1> for pjsua_call_media_info__bindgen_ty_1 {
    fn new() -> pjsua_call_media_info__bindgen_ty_1 {
        pjsua_call_media_info__bindgen_ty_1 {
            aud: pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_call_media_info> for pjsua_call_media_info {
    fn new() -> pjsua_call_media_info {
        pjsua_call_media_info {
            index: 0,
            type_: 0,
            dir: 0,
            status: 0,
            stream: pjsua_call_media_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_call_info> for pjsua_call_info {
    fn new() -> pjsua_call_info {
        pjsua_call_info {
            id: -1,
            role: 0,
            acc_id: -1,
            local_info: pj_str_t::new(),
            local_contact: pj_str_t::new(),
            remote_info: pj_str_t::new(),
            remote_contact: pj_str_t::new(),
            call_id: pj_str_t::new(),
            setting: pjsua_call_setting::new(),
            state: 0,
            state_text: pj_str_t::new(),
            last_status: 0,
            last_status_text: pj_str_t::new(),
            media_status: 0,
            media_dir: 0,
            conf_slot: -1,
            media_cnt: 0,
            media: [pjsua_call_media_info::new(); 16],
            prov_media_cnt: 0,
            prov_media: [pjsua_call_media_info::new(); 16],
            connect_duration: pj_time_val::new(),
            total_duration: pj_time_val::new(),
            rem_offerer: pj_constants__PJ_FALSE as pj_bool_t,
            rem_aud_cnt: 0,
            rem_vid_cnt: 0,
            buf_: pjsua_call_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_buddy_info> for pjsua_buddy_info {
    fn new() -> pjsua_buddy_info {
        pjsua_buddy_info {
            id: -1,
            uri: pj_str_t::new(),
            contact: pj_str_t::new(),
            status: 0,
            status_text: pj_str_t::new(),
            monitor_pres: pj_constants__PJ_FALSE as pj_bool_t,
            sub_state: 0,
            sub_state_name: ptr::null_mut(),
            sub_term_code: 0,
            sub_term_reason: pj_str_t::new(),
            rpid: pjrpid_element::new(),
            pres_status: pjsip_pres_status::new(),
            buf_: [0; 512],
        }
    }
}

impl AutoCreate<pjsua_msg_data> for pjsua_msg_data {
    fn new () -> pjsua_msg_data{
        pjsua_msg_data {
            target_uri: pj_str_t::new(),
            hdr_list: pjsip_hdr::new(),
            content_type: pj_str_t::new(),
            msg_body: pj_str_t::new(),
            multipart_ctype: pjsip_media_type::new(),
            multipart_parts: pjsip_multipart_part::new()
        }
    }
}




