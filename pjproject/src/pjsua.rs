
use pj_sys::{PJ_SUCCESS, pj_pool_factory, pj_pool_release, pj_pool_safe_release, pj_pool_t, pj_str_t, pj_time_val, pj_timer_entry};
use pjmedia_sys::{pjmedia_aud_dev_info, pjmedia_codec_param, pjmedia_echo_stat, pjmedia_endpt, pjmedia_port, pjmedia_sdp_session, pjmedia_snd_dev_info, pjmedia_snd_port, pjmedia_snd_port_param, pjmedia_srtp_crypto, pjmedia_stream_info, pjmedia_transport_info, pjmedia_wav_player_info};
use pjnath_sys::{pj_stun_nat_type, pj_turn_sock_tls_cfg};
use pjsip_simple_sys::{pjrpid_element, pjsip_evsub_state};
use pjsip_sys::{PJSIP_MAX_TRANSPORTS, pjsip_dialog_cap_status, pjsip_endpoint, pjsip_method, pjsip_redirect_op, pjsip_rx_data, pjsip_tpfactory, pjsip_transport, pjsip_transport_type_e, pjsip_tx_data};
use pjsua_sys::*;

use super::prelude::*;
use super::utils;


use std::os::raw::{c_uint, c_void};
use std::ffi::CString;
use std::ptr;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;


pub mod auto;
pub mod ua;
pub mod media;
pub mod account;
pub mod log;
pub mod transport;
pub mod buddy;
pub mod call;
pub mod codec;
pub mod message;

// config, Options and setting struct
pub use pjsua_sys::pjsua_config as UAConfig;
pub use pjsua_sys::pjsua_logging_config as LogConfig;
pub use pjsua_sys::pjsua_media_config as MediaConfig;
pub use pjsua_sys::pjsua_acc_config as AccountConfig;
pub use pjsua_sys::pjsua_transport_config as TransportConfig;
pub use pjsua_sys::pjsua_ice_config as ICEConfig;
pub use pjsua_sys::pjsua_turn_config as TurnConfig;
pub use pjsua_sys::pjsua_buddy_config as BuddyConfig;
pub use pjsua_sys::pjsua_call_setting as CallSetting;
pub use pjsua_sys::pjsua_srtp_opt as SRTPOption;
pub use pjsua_sys::pjsua_ip_change_acc_cfg as IPChangeAccountConfig;
pub use pjsua_sys::pjsua_turn_config as TURNConfig;

pub type RtcpFbSetting = pjmedia_sys::pjmedia_rtcp_fb_setting;
pub type RtcpFbInfo = pjmedia_sys::pjmedia_rtcp_fb_info;
pub type RtcpFbCapability = pjmedia_sys::pjmedia_rtcp_fb_cap;

// info and status struct
pub use pjsua_sys::pjsua_acc_info as AccountInfo;
pub use pjsua_sys::pjsua_buddy_info as BuddyInfo;
pub use pjsua_sys::pjsua_transport_info as TransportInfo;
pub use pjsua_sys::pjsua_call_media_info as CallMediaInfo;
pub use pjsua_sys::pjsua_call_info as CallInfo;
pub use pjsua_sys::pjsua_conf_port_info as ConferencePortInfo;
pub use pjsua_sys::pjsua_stream_info as StreamInfo;
pub use pjmedia_sys::pjmedia_stream_info as MediaStreamInfo;
pub use pjmedia_sys::pjmedia_codec_info as MediaCodecInfo;
pub use pjmedia_sys::pjmedia_codec_param as MediaCodecParam;
pub use pjsua_sys::pjsua_stream_stat as StreamStatus;
pub use pjsua_sys::pjsua_codec_info as CodecInfo;
pub use pjsip_sys::pjsip_cred_info as CredentialInfo;

// data struct
pub use pjsua_sys::pjsua_msg_data as MessageData;

// callback struct
pub use pjsua_sys::pjsua_callback as UACallback;

pub const INVALID_ID: i32 = -1;
pub const MAX_ACC: usize = pjsua_sys::PJSUA_MAX_ACC as usize;
pub const MAX_BUDDIES: usize = pjsua_sys::PJSUA_MAX_BUDDIES as usize;


/// pub type pjsua_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SUAState {
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
pub enum SUAMedTpSt {
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
pub enum SUAContactRewriteMethod {
    Unregister = pjsua_sys::PJSUA_CONTACT_REWRITE_UNREGISTER,
    NoUnreg = pjsua_sys::PJSUA_CONTACT_REWRITE_NO_UNREG,
    AlwaysUpdate = pjsua_sys::PJSUA_CONTACT_REWRITE_ALWAYS_UPDATE,
}

/// pub type pjsua_ip_change_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SUAIpChangeOp {
    Null = pjsua_sys::PJSUA_IP_CHANGE_OP_NULL,
    RestartLis = pjsua_sys::PJSUA_IP_CHANGE_OP_RESTART_LIS,
    AccShutdownTp = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP, 
    AccUpdateContact = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT, 
    AccHangupCalls = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS, 
    AccReinviteCalls = pjsua_sys::PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS, 
    Completed = pjsua_sys::PJSUA_IP_CHANGE_OP_COMPLETED, 
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
pub enum SUADestroyFlag {
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
pub enum SUAIpV6Use {
    Disabled = pjsua_sys::PJSUA_IPV6_DISABLED,
    Enabled = pjsua_sys::PJSUA_IPV6_ENABLED,
}

/// pub type pjsua_nat64_opt = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SUANat64Use {
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
pub enum SUACallVidStrmOp {
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
pub enum SUASndDevId {
    DefaultCaptureDev = pjsua_sys::PJSUA_SND_DEFAULT_CAPTURE_DEV,
    DefaultPlaybackDev = pjsua_sys::PJSUA_SND_DEFAULT_PLAYBACK_DEV,
    NoDev = pjsua_sys::PJSUA_SND_NO_DEV,
    NullDev = pjsua_sys::PJSUA_SND_NULL_DEV,
}

/// pub type pjsua_snd_dev_mode = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SUASndDevMode {
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
pub enum MediaConfigChannel {
    Mono = 1,
    Stereo = 2
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaConfigClockRate {
    ClockRate8000 = 8000,
    ClockRate16000 = 16000,
    ClockRate24000 = 24000,
    ClockRate32000 = 32000,
    ClockRate44100 = 44100,
    ClockRate48000 = 48000,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaConfigEncodingQuality {
    Level1 = 1, Level2 = 2, Level3 = 3, Level4 = 4,
    Level5 = 5, Level6 = 6, Level7 = 7, Level8 = 8,
    Level9 = 9, Level10 = 10,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaConfigIlbcMode {
    Mode20 = 20,
    Mode30 = 30,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CredentialInfoType {
    PlainText = 0,
    HashDigest = 1,
}








#[link(name="pjsua")]
extern "C" {
    pub fn pjsua_conf_get_msignal_level(
        slot: i32,
        tx_level_l: *mut c_uint,
        tx_level_r: *mut c_uint,
        rx_level_l: *mut c_uint,
        rx_level_r: *mut c_uint,
    ) -> i32;
}

pub fn conf_get_msignal_level(
    slot: i32,
    tx_level_l: &mut u32,
    tx_level_r: &mut u32,
    rx_level_l: &mut u32,
    rx_level_r: &mut u32
) -> i32 {

    unsafe {
        pjsua_conf_get_msignal_level(
            slot,
            tx_level_l as *mut _,
            tx_level_r as *mut _,
            rx_level_l as *mut _,
            rx_level_r as *mut _
        )
    }

}



// function helper
pub fn pool_create(pool_name: &str) -> *mut pj_pool_t {
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

pub fn logging_config_default(cfg: &mut LogConfig) {
    unsafe { pjsua_sys::pjsua_logging_config_default(cfg as *mut _); }
}

pub fn config_default(cfg: &mut UAConfig) {
    unsafe { pjsua_sys::pjsua_config_default(cfg as *mut _); }
}

pub fn create () -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_create()) }
}

pub fn init (ua_cfg: &mut UAConfig, log_cfg: &mut LogConfig, media_cfg: &mut MediaConfig) -> Result<(), i32> {
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

pub fn logging_config_dup (dst: &mut LogConfig, src: &mut LogConfig) {
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

pub fn msg_data_init(msg_data: &mut MessageData) {
    unsafe { pjsua_sys::pjsua_msg_data_init(msg_data as *mut _); }
}

pub fn msg_data_clone (rhs: &mut MessageData) -> *mut MessageData {
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

pub fn reconfigure_logging (c: &mut LogConfig) -> Result<(), i32> {
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

pub fn handle_ip_change(param: &mut pjsua_ip_change_param) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_handle_ip_change( param as *const _ )) }
}


// call helper function

pub fn call_setting_default (opt: &mut CallSetting) {
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
    opt: Option<&mut CallSetting>,
    msg_data: Option<&mut MessageData>,
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

pub fn call_get_info (call_id: i32, info: &mut CallInfo) -> Result<(), i32> {
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

pub fn call_answer (call_id: i32, code: u32, reason: Option<String>, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

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
    opt: &mut CallSetting,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut MessageData>
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
    opt: &mut CallSetting,
    code: u32,
    reason: Option<String>,
    msg_data: Option<&mut MessageData>
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
    msg_data: Option<&mut MessageData>
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

pub fn call_set_hold (call_id: i32, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_set_hold( call_id, msg_data)) }
}

pub fn call_set_hold2 (call_id: i32, options: u32, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_set_hold2(call_id, options, msg_data)) }
}

pub fn call_reinvite(call_id: i32, options: u32, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_reinvite( call_id, options, msg_data)) }
}

pub fn call_reinvite2(call_id: i32, opt: &mut CallSetting, msg_data: Option<&mut MessageData> ) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_reinvite2( call_id, opt as *const _, msg_data )) }
}

pub fn call_update (call_id: i32, options: u32, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_sys::pjsua_call_update( call_id, options, msg_data)) }
}

pub fn call_update2 (call_id: i32, opt: &mut CallSetting, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_update2( call_id, opt as *const _, msg_data))
    }
}

pub fn call_xfer (call_id: i32, dest: String, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

    let mut dest = pj_str_t::from_string(dest);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_xfer(call_id,&mut dest as *const _,msg_data))
    }
}

pub fn call_xfer_replaces(call_id: i32, dest_call_id: i32, options: u32, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

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

pub fn call_send_im (call_id: i32, mime_type: String, content: String, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

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

pub fn call_send_typing_ind (call_id: i32, is_typing: bool, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

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

pub fn call_send_request (call_id: i32, method: String, msg_data: Option<&mut MessageData>) -> Result<(), i32> {

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

pub fn call_get_stream_info (call_id: i32, med_idx: u32, psi: &mut StreamInfo) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_call_get_stream_info (call_id, med_idx, psi as *mut _))
    }
}

pub fn call_get_stream_stat (call_id: i32, med_idx: u32, stat: &mut StreamStatus) -> Result<(), i32> {
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



// pjsua sound and media device function helper
pub fn media_config_default(cfg: &mut MediaConfig) {
    unsafe { pjsua_sys::pjsua_media_config_default(cfg as *mut _); }
}

pub fn snd_dev_param_default (prm: &mut pjsua_snd_dev_param) {
    unsafe { pjsua_sys::pjsua_snd_dev_param_default(prm as *mut _); }
}

pub fn conf_connect_param_defautl(prm: &mut pjsua_conf_connect_param) {
    unsafe { pjsua_sys::pjsua_conf_connect_param_default(prm as *mut _); }
}

pub fn conf_get_max_ports() -> u32 {
    unsafe { pjsua_sys::pjsua_conf_get_max_ports() }
}

pub fn conf_get_active_ports() -> u32 {
    unsafe { pjsua_sys::pjsua_conf_get_active_ports() }
}

pub fn enum_conf_ports(id: &mut [i32; pjsua_sys::PJSUA_MAX_CONF_PORTS as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_conf_ports( id.as_mut_ptr(), count as *mut _))
    }
}

pub fn conf_get_port_info (port_id: i32, info: &mut ConferencePortInfo) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_conf_get_port_info( port_id, info as *mut _ ))
    }
}

pub fn conf_add_port(port: *mut pjmedia_port, p_id: Option<&mut i32>) -> Result<(), i32> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };


    unsafe {
        let pool = pool_create("tmp-pool");

        let result = pjsua_sys::pjsua_conf_add_port(pool, port, p_id);

        pool_release(pool);

        if result == PJ_SUCCESS as i32 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

pub fn conf_remove_port (port_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_conf_remove_port(port_id)) }
}

pub fn conf_connect(source: i32, sink: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_conf_connect(source, sink)) }
}

pub fn conf_connect2 (source: i32, sink: i32, prm: &mut pjsua_conf_connect_param) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_conf_connect2( source, sink, prm as *const _ ))
    }
}

pub fn conf_disconnect(source: i32, sink: i32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_conf_disconnect(source, sink))
    }
}

pub fn conf_adjust_tx_level (slot: i32, level: f32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_conf_adjust_tx_level(slot, level))
    }
}

pub fn conf_adjust_rx_level (slot: i32, level: f32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_conf_adjust_rx_level(slot, level)) }
}

pub fn conf_get_signal_level (slot: i32, tx_level: &mut u32, rx_level: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_conf_get_signal_level (slot, tx_level as *mut _, rx_level as *mut _))
    }
}

pub fn player_create(filename: String, options: u32, p_id: &mut i32) -> Result<(), i32> {

    let filename: *const pj_str_t = &mut pj_str_t::from_string(filename) as *const _;

    unsafe {
        utils::check_status(pjsua_sys::pjsua_player_create( filename, options, p_id as *mut _))
    }
}

// i32 	pjsua_playlist_create (const pj_str_t file_names[], unsigned file_count, const pj_str_t *label, unsigned options, pjsua_player_id *p_id)

pub fn player_get_conf_port(id: i32) -> i32 {
    unsafe { pjsua_sys::pjsua_player_get_conf_port(id) }
}

pub fn player_get_port(id: i32, p_port: &mut pjmedia_port) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_player_get_port(id, &mut (p_port as *mut _) as *mut _))
    }
}

pub fn player_get_info(id: i32, info: &mut pjmedia_wav_player_info) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_player_get_info( id, info as *mut _))
    }
}

pub fn player_get_pos(id: i32) -> i64 {
    unsafe { pjsua_sys::pjsua_player_get_pos(id) }
}

pub fn player_set_pos(id: i32, samples: u32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_player_set_pos(id, samples)) }
}

pub fn player_destroy (id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_player_destroy(id)) }
}

// skiped function

// i32 	pjsua_recorder_create (const pj_str_t *filename, unsigned enc_type, void *enc_param, pj_ssize_t max_size, unsigned options, pjsua_recorder_id *p_id)
// pjsua_conf_port_id 	pjsua_recorder_get_conf_port (pjsua_recorder_id id)
// i32 	pjsua_recorder_get_port (pjsua_recorder_id id, pjmedia_port **p_port)
// i32 	pjsua_recorder_destroy (pjsua_recorder_id id)

pub fn enum_aud_devs(info: &mut [pjmedia_aud_dev_info; 256], count: &mut u32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_enum_aud_devs( info.as_mut_ptr(), count as *mut _)) }
}

pub fn enum_snd_devs(info: &mut [pjmedia_snd_dev_info; 256], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_snd_devs( info.as_mut_ptr(), count as *mut _))
    }
}

pub fn get_snd_dev(capture_dev: &mut i32, playback_dev: &mut i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_get_snd_dev( capture_dev as *mut _, playback_dev as *mut _ )) }
}

pub fn set_snd_dev(capture_dev: i32, playback_dev: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_set_snd_dev(capture_dev, playback_dev)) }
}

pub fn set_snd_dev2(snd_param: &mut pjsua_snd_dev_param) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_set_snd_dev2( snd_param as *mut _)) }
}

pub fn set_null_snd_dev() -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_set_null_snd_dev()) }
}

pub fn set_no_snd_dev() -> *mut pjmedia_port {
    unsafe { pjsua_sys::pjsua_set_no_snd_dev() }
}

pub fn set_ec(tail_ms: u32, options: u32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_set_ec(tail_ms, options)) }
}

pub fn get_ec_tail(p_tail_ms: &mut u32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_get_ec_tail(p_tail_ms)) }
}

pub fn get_ec_stat(p_stat: &mut pjmedia_echo_stat) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_get_ec_stat( p_stat as *mut _ )) }
}

pub fn snd_is_active() -> bool {
    unsafe { utils::check_boolean(pjsua_sys::pjsua_snd_is_active()) }
}


// skiped function for detailed audio dev setting
// i32 	pjsua_snd_set_setting (pjmedia_aud_dev_cap cap, const void *pval, pj_bool_t keep)
// i32 	pjsua_snd_get_setting (pjmedia_aud_dev_cap cap, void *pval)


// TODO check this create and destroy
pub fn ext_snd_dev_create(param: &mut pjmedia_snd_port_param, p_snd: &mut pjsua_ext_snd_dev) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_ext_snd_dev_create( param as *mut _, &mut (p_snd as *mut _) as *mut _ ))
    }
}

pub fn ext_snd_dev_destroy(snd: &mut pjsua_ext_snd_dev) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_ext_snd_dev_destroy(snd as *mut _)) }
}

pub fn ext_snd_dev_get_snd_port(snd: &mut pjsua_ext_snd_dev) -> *mut pjmedia_snd_port {
    unsafe { pjsua_sys::pjsua_ext_snd_dev_get_snd_port( snd as *mut _) }
}

pub fn ext_snd_dev_get_conf_port(snd: &mut pjsua_ext_snd_dev) -> i32 {
    unsafe { pjsua_sys::pjsua_ext_snd_dev_get_conf_port( snd as *mut _ ) }
}

pub fn enum_codecs(id: &mut [CodecInfo; 32], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_codecs( id.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn codec_set_priority(codec_id: String, priority: u8) -> Result<(), i32> {
    let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;
    unsafe { utils::check_status(pjsua_sys::pjsua_codec_set_priority( codec_id, priority)) }
}

pub fn codec_get_param(codec_id: String, param: &mut pjmedia_codec_param) -> Result<(), i32> {

    let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;

    unsafe {
        utils::check_status(pjsua_sys::pjsua_codec_get_param( codec_id, param as *mut _))
    }
}

pub fn codec_set_param(codec_id: String, param: &mut pjmedia_codec_param) -> Result<(), i32> {
    let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;
    unsafe {
        utils::check_status(pjsua_sys::pjsua_codec_set_param( codec_id, param as *const _ ))
    }
}


// pjsua account helper

pub fn ice_config_from_media_config(dst: &mut ICEConfig, src: &mut MediaConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_ice_config_from_media_config(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn ice_config_dup(dst: &mut ICEConfig, src: &mut ICEConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_ice_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn turn_config_from_media_config(dst: &mut TURNConfig, src: &mut MediaConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_turn_config_from_media_config(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn turn_config_dup(dst: &mut TURNConfig, src: &mut TURNConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_turn_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn srtp_opt_default(cfg: &mut SRTPOption) {
    unsafe {
        pjsua_sys::pjsua_srtp_opt_default(
            cfg as *mut _
        )
    }
}

pub fn srtp_opt_dup(dst: &mut SRTPOption, src: &mut SRTPOption, check_str: bool) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_srtp_opt_dup(
            pool,
            dst as *mut _,
            src as *const _,
            utils::boolean_to_pjbool(check_str)
        )
    }

    pool_release(pool);
}

pub fn acc_config_default (cfg: &mut AccountConfig) {
    unsafe { pjsua_sys::pjsua_acc_config_default(cfg as *mut _); }
}

pub fn acc_config_dup (dst: &mut AccountConfig, src: &mut AccountConfig) {
    unsafe {
        let pool = pool_create("tmp-pool");

        pjsua_sys::pjsua_acc_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool);
    }
}

pub fn acc_get_count() -> u32 {
    unsafe { pjsua_sys::pjsua_acc_get_count() }
}

pub fn acc_is_valid(acc_id: i32) -> bool {
    unsafe { utils::check_boolean(pjsua_sys::pjsua_acc_is_valid(acc_id)) }
}

pub fn acc_set_default(acc_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_acc_set_default(acc_id)) }
}

pub fn acc_get_default() -> i32 {
    unsafe { pjsua_sys::pjsua_acc_get_default() }
}

pub fn acc_add(acc_cfg: &mut AccountConfig, is_default: bool, p_acc_id: &mut i32) -> Result<(), i32> {
    unsafe {

        let status = pjsua_sys::pjsua_acc_add(
            acc_cfg as *const _,
            utils::boolean_to_pjbool(is_default),
            p_acc_id as *mut _
        );

        utils::check_status(status)
    }
}

pub fn acc_add_local(tid: i32, is_default: bool, p_acc_id: &mut i32) -> Result<(), i32> {
    unsafe {

        let status = pjsua_sys::pjsua_acc_add_local(
            tid,
            utils::boolean_to_pjbool(is_default),
            p_acc_id as *mut _
        );

        utils::check_status(status)
    }
}

// i32 	pjsua_acc_set_user_data (i32 acc_id, void *user_data)
// void * 	pjsua_acc_get_user_data (i32 acc_id)

pub fn acc_del(acc_id: i32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_del(acc_id))
    }
}

pub fn acc_get_config (acc_id: i32, acc_cfg: &mut AccountConfig) -> Result<(), i32> {
    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_sys::pjsua_acc_get_config(acc_id, pool, acc_cfg as *mut _);

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_modify(acc_id: i32, acc_cfg: &mut AccountConfig) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_modify( acc_id, acc_cfg as *const _ ))
    }
}

pub fn acc_set_online_status(acc_id: i32, is_online: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_set_online_status( acc_id, utils::boolean_to_pjbool(is_online)))
    }
}

pub fn acc_set_online_status2(acc_id: i32, is_online: bool, pr: &mut  pjrpid_element) -> Result<(), i32> {

    unsafe {
        let status = pjsua_sys::pjsua_acc_set_online_status2(
            acc_id,
            utils::boolean_to_pjbool(is_online),
            pr as *const _
        );
        utils::check_status(status)
    }
}

pub fn acc_set_registration(acc_id: i32, renew: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_set_registration( acc_id, utils::boolean_to_pjbool(renew)))
    }
}

pub fn acc_get_info (acc_id: i32, info: &mut AccountInfo) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_get_info(acc_id, info as *mut _))
    }
}

pub fn enum_accs(ids: &mut [i32; pjsua_sys::PJSUA_MAX_ACC as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_accs( ids.as_mut_ptr(), count as *mut _))
    }
}

pub fn acc_enum_info(info: &mut [AccountInfo; pjsua_sys::PJSUA_MAX_ACC as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_enum_info( info.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn acc_find_for_outgoing(url: String) -> i32 {

    let mut url = pj_str_t::from_string(url);

    unsafe {
        pjsua_sys::pjsua_acc_find_for_outgoing(
            &mut url as *const _
        )
    }

}

pub fn acc_find_for_incoming(rdata: &mut pjsip_rx_data) -> i32 {

    unsafe {
        pjsua_sys::pjsua_acc_find_for_incoming(
            rdata as *mut _
        )
    }
}

pub fn acc_create_request(acc_id: i32, method: &mut pjsip_method, target: String, p_tdata: &mut pjsip_tx_data) -> Result<(), i32> {

    let mut target = pj_str_t::from_string(target);

    unsafe {
        let status = pjsua_sys::pjsua_acc_create_request(
            acc_id,
            method as *const _,
            &mut target as *const _,
            (p_tdata as *mut _) as *mut _
        );

        utils::check_status(status)
    }
}

pub fn acc_create_uac_contact(contact: String, acc_id: i32, uri: String) -> Result<(), i32> {

    let mut contact = pj_str_t::from_string(contact);
    let mut uri = pj_str_t::from_string(uri);

    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_sys::pjsua_acc_create_uac_contact(
            pool,
            &mut contact as *mut _,
            acc_id,
            &mut uri as *mut _
        );

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_create_uas_contact(contact: String, acc_id: i32, rdata: &mut pjsip_rx_data) -> Result<(), i32> {

    let mut contact = pj_str_t::from_string(contact);

    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_sys::pjsua_acc_create_uas_contact(
            pool,
            &mut contact as *mut _,
            acc_id,
            rdata as *mut _
        );

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_set_transport(acc_id: i32, tp_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_acc_set_transport( acc_id, tp_id )) }
}


// JSUA-API Buddy, Presence, and Instant Messaging

pub fn buddy_config_default(cfg: &mut BuddyConfig) {
    unsafe {
        pjsua_sys::pjsua_buddy_config_default(
            cfg as *mut _
        )
    }
}

pub fn get_buddy_count() -> u32 {
    unsafe { pjsua_sys::pjsua_get_buddy_count() }
}

pub fn buddy_is_valid(buddy_id: i32) -> bool {
    unsafe {utils::check_boolean(pjsua_sys::pjsua_buddy_is_valid(buddy_id)) }
}

pub fn enum_buddies(ids: &mut [i32; pjsua_sys::PJSUA_MAX_BUDDIES as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_buddies( ids.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn buddy_find(uri: String) -> i32 {
    let uri: *const pj_str_t = &mut pj_str_t::from_string(uri) as *const _;
    unsafe { pjsua_sys::pjsua_buddy_find( uri ) }
}

pub fn buddy_get_info(buddy_id: i32, info: &mut BuddyInfo) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_buddy_get_info( buddy_id, info as *mut _ )) }
}

// skiped function this mosly for trasfer data
// i32 	pjsua_buddy_set_user_data (pjsua_buddy_id buddy_id, void *user_data)
// void * 	pjsua_buddy_get_user_data (pjsua_buddy_id buddy_id)

pub fn buddy_add(buddy_cfg: &mut BuddyConfig, p_buddy_id: *mut i32) -> Result<(), i32> {
    unsafe {
        let status = pjsua_sys::pjsua_buddy_add (
            buddy_cfg as *const _,
            p_buddy_id as *mut _
        );
        utils::check_status(status)
    }
}

pub fn buddy_del(buddy_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_buddy_del(buddy_id)) }
}

pub fn buddy_subscribe_pres(buddy_id: i32, subscribe: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_buddy_subscribe_pres(buddy_id, utils::boolean_to_pjbool(subscribe)))
    }
}

pub fn buddy_update_pres(buddy_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_buddy_update_pres(buddy_id)) }
}

pub fn pres_notify(
    acc_id: i32,
    srv_pres: &mut pjsua_srv_pres,
    state: pjsip_evsub_state,
    state_str: String,
    reason: String,
    with_body: bool,
    msg_data: Option<&mut MessageData>
) -> Result<(), i32> {

    let mut state_str = pj_str_t::from_string(state_str);
    let mut reason = pj_str_t::from_string(reason);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_pres_notify(
            acc_id,
            srv_pres,
            state,
            &mut state_str as *const _,
            &mut reason as *const _,
            utils::boolean_to_pjbool(with_body),
            msg_data
        );

        utils::check_status(status)
    }
}

pub fn pres_dump(verbose: bool) {
    unsafe { pjsua_sys::pjsua_pres_dump ( utils::boolean_to_pjbool(verbose))}
}

pub fn im_send(
    acc_id: i32,
    to: String,
    mime_type: String,
    content: String,
    msg_data: Option<&mut MessageData>
) -> Result<(), i32> {

    let mut to = pj_str_t::from_string(to);
    let mut mime_type = pj_str_t::from_string(mime_type);
    let mut content = pj_str_t::from_string(content);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_im_send(
            acc_id,
            &mut to as *const _,
            &mut mime_type as *const _,
            &mut content as *const _,
            msg_data,
            ptr::null_mut()
        );
        utils::check_status(status)
    }

}

pub fn im_typing(
    acc_id: i32,
    to:String,
    is_typing: bool,
    msg_data: Option<&mut MessageData>
) -> Result<(), i32> {

    let mut to = pj_str_t::from_string(to);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_im_typing(
            acc_id,
            &mut to as *const _,
            utils::boolean_to_pjbool(is_typing),
            msg_data
        );

        utils::check_status(status)
    }

}

// PJSUA-API Signaling Transport

pub fn transport_config_default(cfg: &mut TransportConfig) {
    unsafe { pjsua_sys::pjsua_transport_config_default(cfg as *mut _) }
}

pub fn transport_config_dup(dst: &mut TransportConfig, src: &mut TransportConfig) {
    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_transport_config_dup(
            pool,
            dst as *mut _,
            src as *mut _
        );
    }

    pool_release(pool)
}

pub fn transport_create(type_: pjsip_transport_type_e, cfg: &mut TransportConfig, p_id: Option<&mut i32>) -> Result<(), i32> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_transport_create(
            type_,
            cfg as *const _,
            p_id
        );
        utils::check_status(status)
    }
}

pub fn transport_register(tp: &mut pjsip_transport, p_id: Option<&mut i32>) -> Result<(), i32> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_transport_register( tp as *mut _, p_id))
    }
}

pub fn tpfactory_register(tf: &mut pjsip_tpfactory, p_id: Option<&mut i32>) -> Result<(), i32> {
    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_sys::pjsua_tpfactory_register( tf as *mut _, p_id ))
    }
}

pub fn enum_transports(id: &mut [i32; PJSIP_MAX_TRANSPORTS as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_transports( id.as_mut_ptr(), count as *mut _))
    }
}

pub fn transport_get_info(id: i32, info: &mut TransportInfo) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_transport_get_info(id, info as *mut _))
    }
}

pub fn transport_set_enable(id: i32, enabled: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_transport_set_enable( id, utils::boolean_to_pjbool(enabled) ))
    }
}

pub fn transport_close (id: i32, force: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_transport_close ( id, utils::boolean_to_pjbool(force)))
    }
}

pub fn transport_lis_start(id: i32, cfg: &mut TransportConfig) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_transport_lis_start( id, cfg as *const _)) }
}

