#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]


use super::prelude::*;
use super::utils;


use std::os::raw::{c_uint, c_void};
use std::ffi::CString;
use std::ptr;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;


#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunMethod {
    Binding = pjnath_sys::PJ_STUN_BINDING_METHOD,
    Secret = pjnath_sys::PJ_STUN_SHARED_SECRET_METHOD,
    Allocate = pjnath_sys::PJ_STUN_ALLOCATE_METHOD,
    Refresh = pjnath_sys::PJ_STUN_REFRESH_METHOD,
    Send = pjnath_sys::PJ_STUN_SEND_METHOD,
    Data = pjnath_sys::PJ_STUN_DATA_METHOD,
    CreatePerm = pjnath_sys::PJ_STUN_CREATE_PERM_METHOD,
    ChannelBind = pjnath_sys::PJ_STUN_CHANNEL_BIND_METHOD,
    Connect = pjnath_sys::PJ_STUN_CONNECT_METHOD,
    ConnectionBind = pjnath_sys::PJ_STUN_CONNECTION_BIND_METHOD,
    ConnectionAttempt = pjnath_sys::PJ_STUN_CONNECTION_ATTEMPT_METHOD,
    Max = pjnath_sys::PJ_STUN_METHOD_MAX,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunMessageClass {
    Request = pjnath_sys::PJ_STUN_REQUEST_CLASS,
    Indication = pjnath_sys::PJ_STUN_INDICATION_CLASS,
    Success = pjnath_sys::PJ_STUN_SUCCESS_CLASS,
    Error = pjnath_sys::PJ_STUN_ERROR_CLASS,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunMessageType {
    BindingRequest = pjnath_sys::PJ_STUN_BINDING_REQUEST,
    BindingRespose = pjnath_sys::PJ_STUN_BINDING_RESPONSE,
    BindingErrorResponse = pjnath_sys::PJ_STUN_BINDING_ERROR_RESPONSE,
    BindingIndication = pjnath_sys::PJ_STUN_BINDING_INDICATION,
    SharedSecretRequest = pjnath_sys::PJ_STUN_SHARED_SECRET_REQUEST,
    SharedSecretRespose = pjnath_sys::PJ_STUN_SHARED_SECRET_RESPONSE,
    SharedSecretErrorRespose = pjnath_sys::PJ_STUN_SHARED_SECRET_ERROR_RESPONSE,
    AllocateRequest = pjnath_sys::PJ_STUN_ALLOCATE_REQUEST,
    AllocateRespose = pjnath_sys::PJ_STUN_ALLOCATE_RESPONSE,
    AlocateErrorResponse = pjnath_sys::PJ_STUN_ALLOCATE_ERROR_RESPONSE,
    RefreshRequest = pjnath_sys::PJ_STUN_REFRESH_REQUEST,
    RefreshResponse = pjnath_sys::PJ_STUN_REFRESH_RESPONSE,
    RefreshErrorResponse = pjnath_sys::PJ_STUN_REFRESH_ERROR_RESPONSE,
    SendIndication = pjnath_sys::PJ_STUN_SEND_INDICATION,
    DataIndication = pjnath_sys::PJ_STUN_DATA_INDICATION,
    CreatePermRequest = pjnath_sys::PJ_STUN_CREATE_PERM_REQUEST,
    CreatePermResponse = pjnath_sys::PJ_STUN_CREATE_PERM_RESPONSE,
    CreatePermErrorResponse = pjnath_sys::PJ_STUN_CREATE_PERM_ERROR_RESPONSE,
    ChannelBindRequest = pjnath_sys::PJ_STUN_CHANNEL_BIND_REQUEST,
    ChannelBindResponse = pjnath_sys::PJ_STUN_CHANNEL_BIND_RESPONSE,
    ChannelBindErrorResponse = pjnath_sys::PJ_STUN_CHANNEL_BIND_ERROR_RESPONSE,
    ConnectionBindRequest = pjnath_sys::PJ_STUN_CONNECTION_BIND_REQUEST,
    ConnectionAttemptIndication = pjnath_sys::PJ_STUN_CONNECTION_ATTEMPT_INDICATION,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunAttrType {
    MappedAddr = pjnath_sys::PJ_STUN_ATTR_MAPPED_ADDR,
    ResponseAddr = pjnath_sys::PJ_STUN_ATTR_RESPONSE_ADDR,
    ChangeRequest = pjnath_sys::PJ_STUN_ATTR_CHANGE_REQUEST,
    SourceAddr = pjnath_sys::PJ_STUN_ATTR_SOURCE_ADDR,
    ChangedAddr = pjnath_sys::PJ_STUN_ATTR_CHANGED_ADDR,
    AttrUsername = pjnath_sys::PJ_STUN_ATTR_USERNAME,
    AttrPassword = pjnath_sys::PJ_STUN_ATTR_PASSWORD,
    MessageIntegrity = pjnath_sys::PJ_STUN_ATTR_MESSAGE_INTEGRITY,
    ErrorCode = pjnath_sys::PJ_STUN_ATTR_ERROR_CODE,
    UnknownAttributes = pjnath_sys::PJ_STUN_ATTR_UNKNOWN_ATTRIBUTES,
    ReflectedFrom = pjnath_sys::PJ_STUN_ATTR_REFLECTED_FROM,
    ChannelNumber = pjnath_sys::PJ_STUN_ATTR_CHANNEL_NUMBER,
    Lifetime = pjnath_sys::PJ_STUN_ATTR_LIFETIME,
    MagicCookie = pjnath_sys::PJ_STUN_ATTR_MAGIC_COOKIE,
    Bandwidth = pjnath_sys::PJ_STUN_ATTR_BANDWIDTH,
    PeerAddr = pjnath_sys::PJ_STUN_ATTR_XOR_PEER_ADDR,
    Data = pjnath_sys::PJ_STUN_ATTR_DATA,
    Realm = pjnath_sys::PJ_STUN_ATTR_REALM,
    Nonce = pjnath_sys::PJ_STUN_ATTR_NONCE,
    RelayedAddr = pjnath_sys::PJ_STUN_ATTR_XOR_RELAYED_ADDR,
    ReqAddrType = pjnath_sys::PJ_STUN_ATTR_REQ_ADDR_TYPE,
    // ReqAddrFamily = pjnath_sys::PJ_STUN_ATTR_REQ_ADDR_FAMILY,
    EvenPort = pjnath_sys::PJ_STUN_ATTR_EVEN_PORT,
    ReqTransport = pjnath_sys::PJ_STUN_ATTR_REQ_TRANSPORT,
    DontFragment = pjnath_sys::PJ_STUN_ATTR_DONT_FRAGMENT,
    XorMappedAddr = pjnath_sys::PJ_STUN_ATTR_XOR_MAPPED_ADDR,
    TimerVal = pjnath_sys::PJ_STUN_ATTR_TIMER_VAL,
    ReservationToken = pjnath_sys::PJ_STUN_ATTR_RESERVATION_TOKEN,
    XorReflectedFrom = pjnath_sys::PJ_STUN_ATTR_XOR_REFLECTED_FROM,
    Priority = pjnath_sys::PJ_STUN_ATTR_PRIORITY,
    UseCandidate = pjnath_sys::PJ_STUN_ATTR_USE_CANDIDATE,
    ConnectionId = pjnath_sys::PJ_STUN_ATTR_CONNECTION_ID,
    Icmp = pjnath_sys::PJ_STUN_ATTR_ICMP,
    EndMandatoryAttr = pjnath_sys::PJ_STUN_ATTR_END_MANDATORY_ATTR,
    StartExtendAttr = pjnath_sys::PJ_STUN_ATTR_START_EXTENDED_ATTR,
    Software = pjnath_sys::PJ_STUN_ATTR_SOFTWARE,
    AlternateSever = pjnath_sys::PJ_STUN_ATTR_ALTERNATE_SERVER,
    RefreshInternal = pjnath_sys::PJ_STUN_ATTR_REFRESH_INTERVAL,
    Fingerprint = pjnath_sys::PJ_STUN_ATTR_FINGERPRINT,
    IceControlled = pjnath_sys::PJ_STUN_ATTR_ICE_CONTROLLED,
    IceControlling = pjnath_sys::PJ_STUN_ATTR_ICE_CONTROLLING,
    EndExtendedAttr = pjnath_sys::PJ_STUN_ATTR_END_EXTENDED_ATTR,
}

pub const PJ_STUN_SC_TRY_ALTERNATE: pj_stun_status = 300;
pub const PJ_STUN_SC_BAD_REQUEST: pj_stun_status = 400;
pub const PJ_STUN_SC_UNAUTHORIZED: pj_stun_status = 401;
pub const PJ_STUN_SC_FORBIDDEN: pj_stun_status = 403;
pub const PJ_STUN_SC_UNKNOWN_ATTRIBUTE: pj_stun_status = 420;
pub const PJ_STUN_SC_ALLOCATION_MISMATCH: pj_stun_status = 437;
pub const PJ_STUN_SC_STALE_NONCE: pj_stun_status = 438;
pub const PJ_STUN_SC_TRANSITIONING: pj_stun_status = 439;
pub const PJ_STUN_SC_WRONG_CREDENTIALS: pj_stun_status = 441;
pub const PJ_STUN_SC_UNSUPP_TRANSPORT_PROTO: pj_stun_status = 442;
pub const PJ_STUN_SC_OPER_TCP_ONLY: pj_stun_status = 445;
pub const PJ_STUN_SC_CONNECTION_FAILURE: pj_stun_status = 446;
pub const PJ_STUN_SC_CONNECTION_TIMEOUT: pj_stun_status = 447;
pub const PJ_STUN_SC_ALLOCATION_QUOTA_REACHED: pj_stun_status = 486;
pub const PJ_STUN_SC_ROLE_CONFLICT: pj_stun_status = 487;
pub const PJ_STUN_SC_SERVER_ERROR: pj_stun_status = 500;
pub const PJ_STUN_SC_INSUFFICIENT_CAPACITY: pj_stun_status = 508;
pub const PJ_STUN_SC_GLOBAL_FAILURE: pj_stun_status = 600;
pub type pj_stun_status = u32;