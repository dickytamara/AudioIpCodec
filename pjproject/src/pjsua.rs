
use pj_sys::{PJ_SUCCESS, pj_pool_factory, pj_pool_release, pj_pool_safe_release, pj_pool_t, pj_str_t, pj_time_val, pj_timer_entry};
use pjmedia_sys::{pjmedia_codec_param, pjmedia_echo_stat, pjmedia_endpt, pjmedia_port, pjmedia_sdp_session, pjmedia_snd_dev_info, pjmedia_snd_port, pjmedia_snd_port_param, pjmedia_srtp_crypto, pjmedia_stream_info};
use pjnath_sys::{pj_stun_nat_type, pj_turn_sock_tls_cfg};
use pjsip_simple_sys::{pjrpid_element, pjsip_evsub_state};
use pjsip_sys::{PJSIP_MAX_TRANSPORTS, pjsip_endpoint, pjsip_method, pjsip_rx_data, pjsip_tpfactory, pjsip_transport, pjsip_tx_data};
use pjsua_sys::*;

use crate::pj::PJPool;

use super::prelude::*;
use super::utils;


use std::os::raw::{c_void};
use std::ffi::CString;
use std::ptr;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;


pub mod auto;
pub mod ua;
pub mod media;
pub mod account;
pub mod transport;
pub mod buddy;
pub mod call;

// basic api struct
pub type UALoggingConfig = pjsua_sys::pjsua_logging_config;
pub type UAMwiInfo = pjsua_sys::pjsua_mwi_info;
pub type UARegInfo = pjsua_sys::pjsua_reg_info;
pub type UAOnStreamCreatedParam = pjsua_sys::pjsua_on_stream_created_param;
pub type UAMedTpStateInfo = pjsua_sys::pjsua_med_tp_state_info;
pub type UASrtpOpt = pjsua_sys::pjsua_srtp_opt;
pub type UAIpChangeOpInfo = pjsua_sys::pjsua_ip_change_op_info;
pub type UADtmfInfo = pjsua_sys::pjsua_dtmf_info;
pub type UACallSetting = pjsua_sys::pjsua_call_setting;
pub type UACallback = pjsua_sys::pjsua_callback;
pub type UAConfig = pjsua_sys::pjsua_config;
pub type UAMsgData = pjsua_sys::pjsua_msg_data;
pub type UAStunResolveResult = pjsua_sys::pj_stun_resolve_result;
pub type UAIpChangeParam = pjsua_sys::pjsua_ip_change_param;
pub type UAIpChangeAccCfg = pjsua_sys::pjsua_ip_change_acc_cfg;

// signalling and trasport
pub type UATransportConfig = pjsua_sys::pjsua_transport_config;
pub type UATransportInfo = pjsua_sys::pjsua_transport_info;

// PJSUA-API Accounts Management
pub type UAIceConfig = pjsua_sys::pjsua_ice_config;
pub type UATurnConfig = pjsua_sys::pjsua_turn_config;
pub type UAAccConfig = pjsua_sys::pjsua_acc_config;
pub type UAAccInfo = pjsua_sys::pjsua_acc_info;

// PJSUA-API Calls Management
pub type UACallMediaInfo = pjsua_sys::pjsua_call_media_info;
pub type UACallInfo = pjsua_sys::pjsua_call_info;
pub type UAStreamInfo = pjsua_sys::pjsua_stream_info;
pub type UAStreamStat = pjsua_sys::pjsua_stream_stat;
pub type UACallVidStrmOpParam = pjsua_sys::pjsua_call_vid_strm_op_param;
pub type UACallSendDtmfParam = pjsua_sys::pjsua_call_send_dtmf_param;

// PJSUA-API Buddy, Presence, and Instant Messaging
pub type BuddyInfo = pjsua_sys::pjsua_buddy_info;
pub type UABuddyConfig = pjsua_sys::pjsua_buddy_config;

/// PJSUA-API Media Manipulation
pub type UAMediaConfig = pjsua_sys::pjsua_media_config;
pub type UACodecInfo = pjsua_sys::pjsua_codec_info;
pub type UAConfPortInfo = pjsua_sys::pjsua_conf_port_info;
pub type UAMediaTransport = pjsua_sys::pjsua_media_transport;
pub type UASndDevParam = pjsua_sys::pjsua_snd_dev_param;
pub type UAConfConnectParam = pjsua_sys::pjsua_conf_connect_param;

/// PJSUA-API Video

// outside
pub type CredentialInfo = pjsip_sys::pjsip_cred_info;

pub type UASrvPres = pjsua_sys::pjsua_srv_pres;

pub const INVALID_ID: i32 = -1;
pub const MAX_ACC: usize = pjsua_sys::PJSUA_MAX_ACC as usize;
pub const MAX_BUDDIES: usize = pjsua_sys::PJSUA_MAX_BUDDIES as usize;
pub const PRES_TIMER: usize = pjsua_sys::PJSUA_PRES_TIMER as usize;


/// pub type pjsua_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAState {
    Null = pjsua_sys::PJSUA_STATE_NULL,
    Created = pjsua_sys::PJSUA_STATE_CREATED,
    Init = pjsua_sys::PJSUA_STATE_INIT,
    Starting = pjsua_sys::PJSUA_STATE_STARTING,
    Running = pjsua_sys::PJSUA_STATE_RUNNING,
    Closing = pjsua_sys::PJSUA_STATE_CLOSING,
}

/// pub type pjsua_med_tp_st = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAMedTpSt {
    Null = pjsua_sys::PJSUA_MED_TP_NULL,
    Creating = pjsua_sys::PJSUA_MED_TP_CREATING,
    Idle = pjsua_sys::PJSUA_MED_TP_IDLE,
    Init = pjsua_sys::PJSUA_MED_TP_INIT,
    Running = pjsua_sys::PJSUA_MED_TP_RUNNING,
    Disabled = pjsua_sys::PJSUA_MED_TP_DISABLED,
}

/// pub type pjsua_contact_rewrite_method = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAContactRewriteMethod {
    Unregister = pjsua_sys::PJSUA_CONTACT_REWRITE_UNREGISTER,
    NoUnreg = pjsua_sys::PJSUA_CONTACT_REWRITE_NO_UNREG,
    AlwaysUpdate = pjsua_sys::PJSUA_CONTACT_REWRITE_ALWAYS_UPDATE,
}

/// pub type pjsua_ip_change_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAIpChangeOp {
    Null = pjsua_sys::PJSUA_IP_CHANGE_OP_NULL,
    RestartLis = pjsua_sys::PJSUA_IP_CHANGE_OP_RESTART_LIS,
    AccShutdownTp = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP,
    AccUpdateContact = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT,
    AccHangupCalls = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS,
    AccReinviteCalls = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS,
    Completed = pjsua_sys::PJSUA_IP_CHANGE_OP_COMPLETED,
}

/// pub type pjsua_dtmf_method = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UADtmfMethod {
    Rfc2833 = pjsua_sys::PJSUA_DTMF_METHOD_RFC2833,
    SipInfo = pjsua_sys::PJSUA_DTMF_METHOD_SIP_INFO,
}

/// pub type pjsua_sip_timer_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAConfigSipTimerUse {
    Inactive = pjsua_sys::PJSUA_SIP_TIMER_INACTIVE,
    Optional = pjsua_sys::PJSUA_SIP_TIMER_OPTIONAL,
    Required = pjsua_sys::PJSUA_SIP_TIMER_REQUIRED,
    Always = pjsua_sys::PJSUA_SIP_TIMER_ALWAYS
}

/// pub type pjsua_100rel_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAConfig100relUse {
    NotUsed = pjsua_sys::PJSUA_100REL_NOT_USED,
    Mandatory = pjsua_sys::PJSUA_100REL_MANDATORY,
    Optional = pjsua_sys::PJSUA_100REL_OPTIONAL
}

/// pub type pjsua_destroy_flag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UADestroyFlag {
    NoRxMsg = pjsua_sys::PJSUA_DESTROY_NO_RX_MSG,
    NoTxMsg = pjsua_sys::PJSUA_DESTROY_NO_TX_MSG,
    NoNetwork = pjsua_sys::PJSUA_DESTROY_NO_NETWORK,
}

/// pub type pjsua_call_hold_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum CallHoldType {
    Rfc3264 = pjsua_sys::PJSUA_CALL_HOLD_TYPE_RFC3264,
    Rfc2543 = pjsua_sys::PJSUA_CALL_HOLD_TYPE_RFC2543,
}

/// pub type pjsua_stun_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AccountConfigStunUse {
    Default = pjsua_sys::PJSUA_STUN_USE_DEFAULT,
    Disabled = pjsua_sys::PJSUA_STUN_USE_DISABLED,
    RetryOnFailure = pjsua_sys::PJSUA_STUN_RETRY_ON_FAILURE,
}

/// pub type pjsua_ice_config_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AccountConfigIceUse {
    Default = pjsua_sys::PJSUA_ICE_CONFIG_USE_DEFAULT,
    Custom = pjsua_sys::PJSUA_ICE_CONFIG_USE_CUSTOM,
}

/// pub type pjsua_turn_config_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AccountConfigTurnUse {
    Default = pjsua_sys::PJSUA_TURN_CONFIG_USE_DEFAULT,
    Custom = pjsua_sys::PJSUA_TURN_CONFIG_USE_CUSTOM,
}

/// pub type pjsua_ipv6_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UAIpV6Use {
    Disabled = pjsua_sys::PJSUA_IPV6_DISABLED,
    Enabled = pjsua_sys::PJSUA_IPV6_ENABLED,
}

/// pub type pjsua_nat64_opt = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UANat64Use {
    Disabled = pjsua_sys::PJSUA_NAT64_DISABLED,
    Enabled = pjsua_sys::PJSUA_NAT64_ENABLED,
}

/// pub type pjsua_call_media_status = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum CallMediaStatus {
    None = pjsua_sys::PJSUA_CALL_MEDIA_NONE,
    Active = pjsua_sys::PJSUA_CALL_MEDIA_ACTIVE,
    LocalHold = pjsua_sys::PJSUA_CALL_MEDIA_LOCAL_HOLD,
    RemoteHold = pjsua_sys::PJSUA_CALL_MEDIA_REMOTE_HOLD,
    Error = pjsua_sys::PJSUA_CALL_MEDIA_ERROR,
}

/// pub type pjsua_vid_req_keyframe_method = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum KeyFrameMethod {
    SipInfo = pjsua_sys::PJSUA_VID_REQ_KEYFRAME_SIP_INFO,
    RtcpPLI = pjsua_sys::PJSUA_VID_REQ_KEYFRAME_RTCP_PLI,
}

/// pub type pjsua_call_flag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum CallFlags {
    Unhold = pjsua_sys::PJSUA_CALL_UNHOLD,
    UpdateContact = pjsua_sys::PJSUA_CALL_UPDATE_CONTACT,
    IncludeDisabledMedia = pjsua_sys::PJSUA_CALL_INCLUDE_DISABLED_MEDIA,
    NoSdpOffer = pjsua_sys::PJSUA_CALL_NO_SDP_OFFER,
    ReinitMedia = pjsua_sys::PJSUA_CALL_REINIT_MEDIA,
    UpdateVia = pjsua_sys::PJSUA_CALL_UPDATE_VIA,
    UpdateTarget = pjsua_sys::PJSUA_CALL_UPDATE_TARGET,
}

/// pub type pjsua_call_vid_strm_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UACallVidStrmOp {
    NoOp = pjsua_sys::PJSUA_CALL_VID_STRM_NO_OP,
    Add = pjsua_sys::PJSUA_CALL_VID_STRM_ADD, 
    Remove = pjsua_sys::PJSUA_CALL_VID_STRM_REMOVE, 
    ChangeDir = pjsua_sys::PJSUA_CALL_VID_STRM_CHANGE_DIR, 
    ChangeCapDev = pjsua_sys::PJSUA_CALL_VID_STRM_CHANGE_CAP_DEV,
    StartTransmit = pjsua_sys::PJSUA_CALL_VID_STRM_START_TRANSMIT, 
    StopTransmit = pjsua_sys::PJSUA_CALL_VID_STRM_STOP_TRANSMIT, 
    SendKeyframe = pjsua_sys::PJSUA_CALL_VID_STRM_SEND_KEYFRAME, 
}

/// pub type pjsua_buddy_status = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum BuddyStatus {
    Unknown = pjsua_sys::PJSUA_BUDDY_STATUS_UNKNOWN,
    Online = pjsua_sys::PJSUA_BUDDY_STATUS_ONLINE,
    Offline = pjsua_sys::PJSUA_BUDDY_STATUS_OFFLINE,
}

/// pub type pjsua_snd_dev_id = i32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum UASndDevId {
    DefaultCaptureDev = pjsua_sys::PJSUA_SND_DEFAULT_CAPTURE_DEV,
    DefaultPlaybackDev = pjsua_sys::PJSUA_SND_DEFAULT_PLAYBACK_DEV,
    NoDev = pjsua_sys::PJSUA_SND_NO_DEV,
    NullDev = pjsua_sys::PJSUA_SND_NULL_DEV,
}

/// pub type pjsua_snd_dev_mode = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UASndDevMode {
    SpeakerOnly = pjsua_sys::PJSUA_SND_DEV_SPEAKER_ONLY,
    ImmediateOpen = pjsua_sys::PJSUA_SND_DEV_NO_IMMEDIATE_OPEN,
}


#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum UAConfigSrtpSecureSignaling {
    Disable = 0,
    Tls = 1,
    Sips = 3,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ConfigChannel {
    Mono = 1,
    Stereo = 2
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ClockRate {
    ClockRate8000 = 8000,
    ClockRate16000 = 16000,
    ClockRate24000 = 24000,
    ClockRate32000 = 32000,
    ClockRate44100 = 44100,
    ClockRate48000 = 48000,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum EncodingQuality {
    Level1 = 1, Level2 = 2, Level3 = 3, Level4 = 4,
    Level5 = 5, Level6 = 6, Level7 = 7, Level8 = 8,
    Level9 = 9, Level10 = 10,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IlbcMode {
    Mode20 = 20,
    Mode30 = 30,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CredentialInfoType {
    PlainText = 0,
    HashDigest = 1,
}


// function helper
pub fn pool_create(pool_name: &str) -> *mut PJPool {
    unsafe {

        let ret = pjsua_sys::pjsua_pool_create(
            CString::new(pool_name)
            .expect("String error create pool_name")
            .into_raw(),
            1000,
            1000
        );

        assert_ne!(ret.is_null(), true);
        ret
    }
}

pub fn pool_release(pool: *mut pj_pool_t) {
    unsafe {
        pj_pool_release(pool);
    }
}

pub fn pool_safe_release(ppool: *mut *mut pj_pool_t) {
    unsafe {
        pj_pool_safe_release(ppool);
    }
}



pub fn create () -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_create()) }
}

pub fn init (ua_cfg: &mut UAConfig, log_cfg: &mut UALoggingConfig, media_cfg: &mut UAMediaConfig) -> Result<(), i32> {
    unsafe {
        let status = pjsua_sys::pjsua_init(
        ua_cfg as *const _,
        log_cfg as *const _,
        media_cfg as *const _
        );

        utils::check_status(status)
    }
}

pub fn start () -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_start()) }
}

pub fn destroy () -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_destroy()) }
}

pub fn get_state () -> pjsua_state {
    unsafe { pjsua_sys::pjsua_get_state() }
}

pub fn destroy2 (flags: u32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_destroy2(flags)) }
}

pub fn logging_config_dup (dst: &mut UALoggingConfig, src: &mut UALoggingConfig) {
    unsafe {

        let pool = pool_create("tmp-pool");

        pjsua_sys::pjsua_logging_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool)
    }
}

pub fn config_dup (dst: &mut UAConfig, src: &mut UAConfig) {
    unsafe {

        let pool = pool_create("tmp-pool");

        pjsua_sys::pjsua_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool);
    }
}

pub fn msg_data_init(msg_data: &mut UAMsgData) {
    unsafe { pjsua_sys::pjsua_msg_data_init(msg_data as *mut _); }
}

pub fn msg_data_clone (rhs: &mut UAMsgData) -> *mut UAMsgData {
    unsafe {

        let pool = pool_create("tmp-pool");

        let ret = pjsua_sys::pjsua_msg_data_clone(pool, rhs as *const _ );

        pool_release(pool);

        ret
    }
}

pub fn handle_events(msec_timeout: u32) -> i32 {
    unsafe { pjsua_sys::pjsua_handle_events(msec_timeout) }
}

pub fn stop_worker_threads() {
    unsafe { pjsua_sys::pjsua_stop_worker_threads() }
}

pub fn reconfigure_logging (c: &mut UALoggingConfig) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_reconfigure_logging(c as *const _)) }
}

pub fn get_pjsip_endpt() -> *mut pjsip_endpoint {
    unsafe { pjsua_sys::pjsua_get_pjsip_endpt() }
}

pub fn get_pjmedia_endpt() -> *mut pjmedia_endpt {
    unsafe { pjsua_sys::pjsua_get_pjmedia_endpt() }
}

pub fn get_pool_factory() -> *mut pj_pool_factory {
    unsafe { pjsua_sys::pjsua_get_pool_factory() }
}

pub fn ip_change_param_default(param: &mut pjsua_ip_change_param) {
    unsafe { pjsua_sys::pjsua_ip_change_param_default(param as *mut _) }
}

pub fn detect_nat_type () -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_detect_nat_type()) }
}

pub fn get_nat_type(type_: &mut pj_stun_nat_type) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_get_nat_type(type_ as *mut _)) }
}

pub fn update_stun_servers (count: u32, srv: &mut [pj_str_t; 8], wait: bool) -> Result<(), i32> {
    unsafe {
        // todo fix this and compare result with c code.
        let status = pjsua_sys::pjsua_update_stun_servers(
                count,
                srv.as_mut_ptr(),
                utils::boolean_to_pjbool(wait)
            );

        utils::check_status(status)
    }
}

// i32 	pjsua_resolve_stun_servers (unsigned count, pj_str_t srv[], pj_bool_t wait, void *token, pj_stun_resolve_cb cb)
pub fn resolve_stun_servers<T> (
    count: u32,
    srv: &mut [pj_str_t; 8],
    wait: bool,
    token: Option<&mut T>,
    cb: pj_stun_resolve_cb
) -> Result<(), i32> {
        // todo check token
    unsafe {

        let token = match token {
            Some(value) => (value as *mut _) as *mut c_void,
            None => ptr::null_mut()
        };

        let status = pjsua_sys::pjsua_resolve_stun_servers(
            count,
            srv.as_mut_ptr(),
            utils::boolean_to_pjbool(wait),
            token,
            cb
        );

        utils::check_status(status)
    }
}

// i32 	pjsua_cancel_stun_resolution (void *token, pj_bool_t notify_cb)
pub fn cancel_stun_resolution<T> (token: Option<&mut T>, notify_cb: bool) -> Result<(), i32> {
    unsafe {

        let token = match token {
            Some(val) => (val as *mut _) as *mut c_void,
            None => ptr::null_mut()
        };

        let status = pjsua_sys::pjsua_cancel_stun_resolution (
            token,
            utils::boolean_to_pjbool(notify_cb)
        );

        utils::check_status(status)
    }
}

pub fn verify_sip_url(url: String) -> Result<(), i32> {
    let url: *const i8 = CString::new(url).expect("pjsua_verify_sip_url").into_raw();
    unsafe { utils::check_status(pjsua_sys::pjsua_verify_sip_url( url )) }
}

pub fn verify_url (url: String) -> Result<(), i32> {
    let url: *const i8 = CString::new(url).expect("pjsua_verify_url").into_raw();
    unsafe {
        utils::check_status(pjsua_sys::pjsua_verify_url(url ))
    }
}

pub fn schedule_timer (entry: &mut pj_timer_entry, delay: &mut pj_time_val) -> Result<(), i32> {
    unsafe {
        // because we use debug pjsua
        // will provide timer with debug suport
        let status = pjsua_sys::pjsua_schedule_timer_dbg(
            entry as *mut _,
            delay as *const _,
            ptr::null_mut(),
            0
        );

        utils::check_status(status)
     }
}

// i32 	pjsua_schedule_timer2 (void(*cb)(void *user_data), void *user_data, unsigned msec_delay)

pub fn cancel_timer(entry: &mut pj_timer_entry) {
    unsafe { pjsua_sys::pjsua_cancel_timer(entry as *mut _) }
}

pub fn perror(sender: String, title: String, status: i32) {
    let sender: *const i8 = CString::new(sender.as_str()).expect("pjsua_perror").into_raw();
    let title: *const i8 = CString::new(title.as_str()).expect("pjusa_perror").into_raw();

    unsafe { pjsua_sys::pjsua_perror( sender, title, status ); }
}

pub fn dump(detail: bool) {
    unsafe { pjsua_sys::pjsua_dump(utils::boolean_to_pjbool(detail)); }
}

// pjsua_ip_change_param
pub fn handle_ip_change(param: &mut UAIpChangeParam) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_handle_ip_change( param as *const _ )) }
}










