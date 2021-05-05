#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]


pub mod auto;

use pj_sys::*;
use pjlib_util_sys::*;
use pjsip_sys::*;

use super::utils;
use super::utils::check_status;

use num_enum::*;

use std::{ffi::CString, ptr};


/// pub type pjsip_transport_type_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPTransportType {
    Unsepecified = pjsip_sys::PJSIP_TRANSPORT_UNSPECIFIED,
    Udp = pjsip_sys::PJSIP_TRANSPORT_UDP,
    Tcp = pjsip_sys::PJSIP_TRANSPORT_TCP,
    Tls = pjsip_sys::PJSIP_TRANSPORT_TLS,
    Dtls = pjsip_sys::PJSIP_TRANSPORT_DTLS,
    Sctp = pjsip_sys::PJSIP_TRANSPORT_SCTP,
    Loop = pjsip_sys::PJSIP_TRANSPORT_LOOP,
    LoopDgram = pjsip_sys::PJSIP_TRANSPORT_LOOP_DGRAM,
    StartOther = pjsip_sys::PJSIP_TRANSPORT_START_OTHER,
    IpV6 = pjsip_sys::PJSIP_TRANSPORT_IPV6,
    UdpV6 = pjsip_sys::PJSIP_TRANSPORT_UDP6,
    TcpV6 = pjsip_sys::PJSIP_TRANSPORT_TCP6,
    TlsV6 = pjsip_sys::PJSIP_TRANSPORT_TLS6,
    DtlsV6 = pjsip_sys::PJSIP_TRANSPORT_DTLS6,
}

/// pub type pjsip_role_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPRole {
    Uac = pjsip_sys::PJSIP_ROLE_UAC,
    Uas = pjsip_sys::PJSIP_ROLE_UAS,
    // RoleUac = pjsip_sys::PJSIP_UAC_ROLE,
    // RoleUas = pjsip_sys::PJSIP_UAS_ROLE,
}

/// pub type pjsip_uri_context_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPUriContext {
    ReqUri = pjsip_sys::PJSIP_URI_IN_REQ_URI,
    FromToHdr = pjsip_sys::PJSIP_URI_IN_FROMTO_HDR,
    ContactHdr = pjsip_sys::PJSIP_URI_IN_CONTACT_HDR,
    RoutingHdr = pjsip_sys::PJSIP_URI_IN_ROUTING_HDR,
    Other = pjsip_sys::PJSIP_URI_IN_OTHER,
}


/// pub type pjsip_method_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPMethod {
    Invite = pjsip_sys::PJSIP_INVITE_METHOD,
    Cancel = pjsip_sys::PJSIP_CANCEL_METHOD,
    Ack = pjsip_sys::PJSIP_ACK_METHOD,
    Bye = pjsip_sys::PJSIP_BYE_METHOD,
    Register = pjsip_sys::PJSIP_REGISTER_METHOD,
    Options = pjsip_sys::PJSIP_OPTIONS_METHOD,
    Other = pjsip_sys::PJSIP_OTHER_METHOD,
}

/// pub type pjsip_hdr_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPHdr {
    Accept = pjsip_sys::PJSIP_H_ACCEPT,
    EncodingUnimp = pjsip_sys::PJSIP_H_ACCEPT_ENCODING_UNIMP,
    LanguangeUnimp = pjsip_sys::PJSIP_H_ACCEPT_LANGUAGE_UNIMP,
    AlertInfoUnimp = pjsip_sys::PJSIP_H_ALERT_INFO_UNIMP,
    Allow = pjsip_sys::PJSIP_H_ALLOW,
    AuthenticationInfoUnimp = pjsip_sys::PJSIP_H_AUTHENTICATION_INFO_UNIMP,
    Authorization = pjsip_sys::PJSIP_H_AUTHORIZATION,
    CallId = pjsip_sys::PJSIP_H_CALL_ID,
    CallInfoUnimp = pjsip_sys::PJSIP_H_CALL_INFO_UNIMP,
    Contact = pjsip_sys::PJSIP_H_CONTACT,
    ContentDispositionUnimp = pjsip_sys::PJSIP_H_CONTENT_DISPOSITION_UNIMP,
    ContentEncodingUnimp = pjsip_sys::PJSIP_H_CONTENT_ENCODING_UNIMP,
    ContentLanguageUnimp = pjsip_sys::PJSIP_H_CONTENT_LANGUAGE_UNIMP,
    ContentLength = pjsip_sys::PJSIP_H_CONTENT_LENGTH,
    ContentType = pjsip_sys::PJSIP_H_CONTENT_TYPE,
    Cseq = pjsip_sys::PJSIP_H_CSEQ,
    DateUnimp = pjsip_sys::PJSIP_H_DATE_UNIMP,
    ErrorInfoUnimp = pjsip_sys::PJSIP_H_ERROR_INFO_UNIMP,
    Expires = pjsip_sys::PJSIP_H_EXPIRES,
    From = pjsip_sys::PJSIP_H_FROM,
    InReplyToUnimp = pjsip_sys::PJSIP_H_IN_REPLY_TO_UNIMP,
    MaxForwards = pjsip_sys::PJSIP_H_MAX_FORWARDS,
    MimeVersionUnimp = pjsip_sys::PJSIP_H_MIME_VERSION_UNIMP,
    MinExpires = pjsip_sys::PJSIP_H_MIN_EXPIRES,
    OrganizationUnimp = pjsip_sys::PJSIP_H_ORGANIZATION_UNIMP,
    PriorityUnimp = pjsip_sys::PJSIP_H_PRIORITY_UNIMP,
    ProxyAutenticate = pjsip_sys::PJSIP_H_PROXY_AUTHENTICATE,
    ProxyAuthorization = pjsip_sys::PJSIP_H_PROXY_AUTHORIZATION,
    ProxyRequireUnimp= pjsip_sys::PJSIP_H_PROXY_REQUIRE_UNIMP,
    RecordRoute = pjsip_sys::PJSIP_H_RECORD_ROUTE,
    ReplyToUnimp = pjsip_sys::PJSIP_H_REPLY_TO_UNIMP,
    Require = pjsip_sys::PJSIP_H_REQUIRE,
    RetryAfter = pjsip_sys::PJSIP_H_RETRY_AFTER,
    Route = pjsip_sys::PJSIP_H_ROUTE,
    ServerUnimp = pjsip_sys::PJSIP_H_SERVER_UNIMP,
    SubjectUnimp = pjsip_sys::PJSIP_H_SUBJECT_UNIMP,
    Supported = pjsip_sys::PJSIP_H_SUPPORTED,
    TimestampUnimp = pjsip_sys::PJSIP_H_TIMESTAMP_UNIMP,
    To = pjsip_sys::PJSIP_H_TO,
    Unsupported = pjsip_sys::PJSIP_H_UNSUPPORTED,
    UserAggentUnimp = pjsip_sys::PJSIP_H_USER_AGENT_UNIMP,
    Via = pjsip_sys::PJSIP_H_VIA,
    WarningUnimp = pjsip_sys::PJSIP_H_WARNING_UNIMP,
    WwwAuthenticate = pjsip_sys::PJSIP_H_WWW_AUTHENTICATE,
    Other = pjsip_sys::PJSIP_H_OTHER,
}

/// pub type pjsip_status_code = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPStatusCode {
    Null = pjsip_sys::PJSIP_SC_NULL,
    Trying = pjsip_sys::PJSIP_SC_TRYING,
    Ringing = pjsip_sys::PJSIP_SC_RINGING,
    CallBeingForwarded = pjsip_sys::PJSIP_SC_CALL_BEING_FORWARDED,
    Queued = pjsip_sys::PJSIP_SC_QUEUED,
    Progress = pjsip_sys::PJSIP_SC_PROGRESS,
    EarlyDialogTerminated = pjsip_sys::PJSIP_SC_EARLY_DIALOG_TERMINATED,
    Ok = pjsip_sys::PJSIP_SC_OK,
    Accepted = pjsip_sys::PJSIP_SC_ACCEPTED,
    NoNotification = pjsip_sys::PJSIP_SC_NO_NOTIFICATION,
    MultipleChoices = pjsip_sys::PJSIP_SC_MULTIPLE_CHOICES,
    MovedPermanently = pjsip_sys::PJSIP_SC_MOVED_PERMANENTLY,
    MovedTemporarily = pjsip_sys::PJSIP_SC_MOVED_TEMPORARILY,
    UseProxy = pjsip_sys::PJSIP_SC_USE_PROXY,
    AlternativeService = pjsip_sys::PJSIP_SC_ALTERNATIVE_SERVICE,
    BadRequest = pjsip_sys::PJSIP_SC_BAD_REQUEST,
    Unauthorized = pjsip_sys::PJSIP_SC_UNAUTHORIZED,
    PaymentRequired = pjsip_sys::PJSIP_SC_PAYMENT_REQUIRED,
    Forbiden = pjsip_sys::PJSIP_SC_FORBIDDEN,
    NotFound = pjsip_sys::PJSIP_SC_NOT_FOUND,
    MethodNotallowed = pjsip_sys::PJSIP_SC_METHOD_NOT_ALLOWED,
    NotAcceptable = pjsip_sys::PJSIP_SC_NOT_ACCEPTABLE,
    ProxyAuthenticationRequired = pjsip_sys::PJSIP_SC_PROXY_AUTHENTICATION_REQUIRED,
    RequestTimeout = pjsip_sys::PJSIP_SC_REQUEST_TIMEOUT,
    Conflict = pjsip_sys::PJSIP_SC_CONFLICT,
    Gone = pjsip_sys::PJSIP_SC_GONE,
    LengthRequired = pjsip_sys::PJSIP_SC_LENGTH_REQUIRED,
    ConditionalRequestFailed = pjsip_sys::PJSIP_SC_CONDITIONAL_REQUEST_FAILED,
    RequestEntitiyTooLarge = pjsip_sys::PJSIP_SC_REQUEST_ENTITY_TOO_LARGE,
    RequestUriTooLong = pjsip_sys::PJSIP_SC_REQUEST_URI_TOO_LONG,
    UnsupportedMediaType = pjsip_sys::PJSIP_SC_UNSUPPORTED_MEDIA_TYPE,
    UnsupportedUriScheme = pjsip_sys::PJSIP_SC_UNSUPPORTED_URI_SCHEME,
    UnknownResourcePriority = pjsip_sys::PJSIP_SC_UNKNOWN_RESOURCE_PRIORITY,
    BadExtension = pjsip_sys::PJSIP_SC_BAD_EXTENSION,
    ExtensionRequired = pjsip_sys::PJSIP_SC_EXTENSION_REQUIRED,
    SessionTimerTooSmall = pjsip_sys::PJSIP_SC_SESSION_TIMER_TOO_SMALL,
    IntervalTooBrief = pjsip_sys::PJSIP_SC_INTERVAL_TOO_BRIEF,
    BadLocationInformation = pjsip_sys::PJSIP_SC_BAD_LOCATION_INFORMATION,
    UseIdentityHeader = pjsip_sys::PJSIP_SC_USE_IDENTITY_HEADER,
    ProvideReffererHeader = pjsip_sys::PJSIP_SC_PROVIDE_REFERRER_HEADER,
    FlowFailed = pjsip_sys::PJSIP_SC_FLOW_FAILED,
    AnonimityDisallowed = pjsip_sys::PJSIP_SC_ANONIMITY_DISALLOWED,
    BadIdentityInfo = pjsip_sys::PJSIP_SC_BAD_IDENTITY_INFO,
    UnsupportedCertificate = pjsip_sys::PJSIP_SC_UNSUPPORTED_CERTIFICATE,
    InvalidIdentityHeader = pjsip_sys::PJSIP_SC_INVALID_IDENTITY_HEADER,
    FirstHopLackOutboundSupport = pjsip_sys::PJSIP_SC_FIRST_HOP_LACKS_OUTBOUND_SUPPORT,
    MaxBreadthExceeded = pjsip_sys::PJSIP_SC_MAX_BREADTH_EXCEEDED,
    BadInfoPackage = pjsip_sys::PJSIP_SC_BAD_INFO_PACKAGE,
    ConsentNeeded = pjsip_sys::PJSIP_SC_CONSENT_NEEDED,
    TemporarilyUnavailable = pjsip_sys::PJSIP_SC_TEMPORARILY_UNAVAILABLE,
    TsxDeosNotExist = pjsip_sys::PJSIP_SC_CALL_TSX_DOES_NOT_EXIST,
    LoopDetected = pjsip_sys::PJSIP_SC_LOOP_DETECTED,
    TooManyHops = pjsip_sys::PJSIP_SC_TOO_MANY_HOPS,
    AddressIncomplete = pjsip_sys::PJSIP_SC_ADDRESS_INCOMPLETE,
    Ambigous = pjsip_sys::PJSIP_AC_AMBIGUOUS,
    BusyHere = pjsip_sys::PJSIP_SC_BUSY_HERE,
    RequestTerminted = pjsip_sys::PJSIP_SC_REQUEST_TERMINATED,
    NotAcceptableHere = pjsip_sys::PJSIP_SC_NOT_ACCEPTABLE_HERE,
    BadEvent = pjsip_sys::PJSIP_SC_BAD_EVENT,
    RequestUpdated = pjsip_sys::PJSIP_SC_REQUEST_UPDATED,
    RequestPending = pjsip_sys::PJSIP_SC_REQUEST_PENDING,
    Undecipherable = pjsip_sys::PJSIP_SC_UNDECIPHERABLE,
    SecurityAgreementNeeded = pjsip_sys::PJSIP_SC_SECURITY_AGREEMENT_NEEDED,
    IntenalServerError = pjsip_sys::PJSIP_SC_INTERNAL_SERVER_ERROR,
    NotImplemented = pjsip_sys::PJSIP_SC_NOT_IMPLEMENTED,
    BadGateway = pjsip_sys::PJSIP_SC_BAD_GATEWAY,
    ServiceUnavailable = pjsip_sys::PJSIP_SC_SERVICE_UNAVAILABLE,
    SeverTimeout = pjsip_sys::PJSIP_SC_SERVER_TIMEOUT,
    VersionNotSupported = pjsip_sys::PJSIP_SC_VERSION_NOT_SUPPORTED,
    MessageTooLarge = pjsip_sys::PJSIP_SC_MESSAGE_TOO_LARGE,
    PushNotificationServiceNotSupported = pjsip_sys::PJSIP_SC_PUSH_NOTIFICATION_SERVICE_NOT_SUPPORTED,
    PreconditionFailure = pjsip_sys::PJSIP_SC_PRECONDITION_FAILURE,
    BusyEverywhere = pjsip_sys::PJSIP_SC_BUSY_EVERYWHERE,
    Decline = pjsip_sys::PJSIP_SC_DECLINE,
    DoesNotExistAnywhere = pjsip_sys::PJSIP_SC_DOES_NOT_EXIST_ANYWHERE,
    NotAcceptableAnywhere = pjsip_sys::PJSIP_SC_NOT_ACCEPTABLE_ANYWHERE,
    Unwanted = pjsip_sys::PJSIP_SC_UNWANTED,
    Rejected = pjsip_sys::PJSIP_SC_REJECTED,
    // TsxTimeout = pjsip_sys::PJSIP_SC_TSX_TIMEOUT,
    // TsxTransportError = pjsip_sys::PJSIP_SC_TSX_TRANSPORT_ERROR,
    Force32Bit = pjsip_sys::PJSIP_SC__force_32bit,
}


/// pub type pjsip_msg_type_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPMsgType {
    RequestMsg = pjsip_sys::PJSIP_REQUEST_MSG,
    ResponseMsg = pjsip_sys::PJSIP_RESPONSE_MSG,
}

/// pub type pjsip_event_id_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPEventId {
    Unknown = pjsip_sys::PJSIP_EVENT_UNKNOWN,
    Timer = pjsip_sys::PJSIP_EVENT_TIMER,
    TxMsg = pjsip_sys::PJSIP_EVENT_TX_MSG,
    RxMsg = pjsip_sys::PJSIP_EVENT_RX_MSG,
    TransportError = pjsip_sys::PJSIP_EVENT_TRANSPORT_ERROR,
    TsxState = pjsip_sys::PJSIP_EVENT_TSX_STATE,
    User = pjsip_sys::PJSIP_EVENT_USER,
}

/// pub type pjsip_module_priority = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPModulePriority {
    TransportLayer = pjsip_sys::PJSIP_MOD_PRIORITY_TRANSPORT_LAYER,
    TsxLayer = pjsip_sys::PJSIP_MOD_PRIORITY_TSX_LAYER,
    UaProxyLayer = pjsip_sys::PJSIP_MOD_PRIORITY_UA_PROXY_LAYER,
    DialogUsage = pjsip_sys::PJSIP_MOD_PRIORITY_DIALOG_USAGE,
    Application = pjsip_sys::PJSIP_MOD_PRIORITY_APPLICATION,
}

/// pub type pjsip_transport_flags_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPTransportFlags {
    Reliable = pjsip_sys::PJSIP_TRANSPORT_RELIABLE,
    Secure = pjsip_sys::PJSIP_TRANSPORT_SECURE,
    Datagram = pjsip_sys::PJSIP_TRANSPORT_DATAGRAM,
}

/// pub type pjsip_tpselector_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPTpselectorType {
    None = pjsip_sys::PJSIP_TPSELECTOR_NONE,
    Transport = pjsip_sys::PJSIP_TPSELECTOR_TRANSPORT,
    Listener = pjsip_sys::PJSIP_TPSELECTOR_LISTENER,
}

/// pub type pjsip_transport_dir = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPTransportDir {
    None = pjsip_sys::PJSIP_TP_DIR_NONE,
    Outgoing = pjsip_sys::PJSIP_TP_DIR_OUTGOING,
    Incoming = pjsip_sys::PJSIP_TP_DIR_INCOMING,
}

/// pub type pjsip_transport_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPTransportState {
    Connected = pjsip_sys::PJSIP_TP_STATE_CONNECTED,
    Disconnected = pjsip_sys::PJSIP_TP_STATE_DISCONNECTED,
    Shutdown = pjsip_sys::PJSIP_TP_STATE_SHUTDOWN,
    Destroy = pjsip_sys::PJSIP_TP_STATE_DESTROY,
}

/// pub type pjsip_redirect_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPRedirectOp {
    Reject = PJSIP_REDIRECT_REJECT,
    Accept = PJSIP_REDIRECT_ACCEPT,
    AcceptReplace = PJSIP_REDIRECT_ACCEPT_REPLACE,
    Pending = PJSIP_REDIRECT_PENDING,
    Stop = PJSIP_REDIRECT_STOP,
}

/// pub type pjsip_ssl_method = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPSslMethod {
    Unspecified = pjsip_sys::PJSIP_SSL_UNSPECIFIED_METHOD,
    SslV2 = pjsip_sys::PJSIP_SSLV2_METHOD,
    SslV3 = pjsip_sys::PJSIP_SSLV3_METHOD,
    TlsV1 = pjsip_sys::PJSIP_TLSV1_METHOD,
    TlsV1_1 = pjsip_sys::PJSIP_TLSV1_1_METHOD,
    TlsV1_2 = pjsip_sys::PJSIP_TLSV1_2_METHOD,
    TlsV1_3 = pjsip_sys::PJSIP_TLSV1_3_METHOD,
    SslV23 = pjsip_sys::PJSIP_SSLV23_METHOD,
}

/// pub type pjsip_cred_data_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPCredDataType {
    PlainPasswd = pjsip_sys::PJSIP_CRED_DATA_PLAIN_PASSWD,
    Digest = pjsip_sys::PJSIP_CRED_DATA_DIGEST,
    ExtAka = pjsip_sys::PJSIP_CRED_DATA_EXT_AKA,
}

///  pub type pjsip_auth_qop_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPauthQopType {
    None = pjsip_sys::PJSIP_AUTH_QOP_NONE,
    Auth = pjsip_sys::PJSIP_AUTH_QOP_AUTH,
    AuthInt = pjsip_sys::PJSIP_AUTH_QOP_AUTH_INT,
    Unknown = pjsip_sys::PJSIP_AUTH_QOP_UNKNOWN,
}

/// pub type pjsip_tsx_state_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPTsxState {
    Null = pjsip_sys::PJSIP_TSX_STATE_NULL,
    Calling = pjsip_sys::PJSIP_TSX_STATE_CALLING,
    Trying = pjsip_sys::PJSIP_TSX_STATE_TRYING,
    Proceeding = pjsip_sys::PJSIP_TSX_STATE_PROCEEDING,
    Completed = pjsip_sys::PJSIP_TSX_STATE_COMPLETED,
    Confirmed = pjsip_sys::PJSIP_TSX_STATE_CONFIRMED,
    Terminated = pjsip_sys::PJSIP_TSX_STATE_TERMINATED,
    Destroyed = pjsip_sys::PJSIP_TSX_STATE_DESTROYED,
    Max = pjsip_sys::PJSIP_TSX_STATE_MAX,
}

/// pub type pjsip_dialog_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPDialogState {
    Null = pjsip_sys::PJSIP_DIALOG_STATE_NULL,
    Established = pjsip_sys::PJSIP_DIALOG_STATE_ESTABLISHED,
}

///  pub type pjsip_dialog_cap_status = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPDialogCapStatus {
    Unsupported = pjsip_sys::PJSIP_DIALOG_CAP_UNSUPPORTED,
    Supported = pjsip_sys::PJSIP_DIALOG_CAP_SUPPORTED,
    Unknown = pjsip_sys::PJSIP_DIALOG_CAP_UNKNOWN,
}


// function helper
// const pjsip_method * 	pjsip_get_invite_method (void)
// const pjsip_method * 	pjsip_get_cancel_method (void)
// const pjsip_method * 	pjsip_get_ack_method (void)
// const pjsip_method * 	pjsip_get_bye_method (void)
// const pjsip_method * 	pjsip_get_register_method (void)
// const pjsip_method * 	pjsip_get_options_method (void)
// void 	pjsip_method_init (pjsip_method *m, pj_pool_t *pool, const pj_str_t *str)
// void 	pjsip_method_init_np (pjsip_method *m, pj_str_t *str)
// void 	pjsip_method_set (pjsip_method *m, pjsip_method_e id)
// void 	pjsip_method_copy (pj_pool_t *pool, pjsip_method *method, const pjsip_method *rhs)
// int 	pjsip_method_cmp (const pjsip_method *m1, const pjsip_method *m2)
pub fn method_cmp(m1: &pjsip_method, m2: &pjsip_method) -> i32 {
    unsafe {
        pjsip_method_cmp(m1 as *const _, m2 as *const _)
    }
}

pub fn endpt_create(pf: *mut pj_pool_factory, name: String, endpt: &mut Box<*mut pjsip_endpoint>) -> Result<(), i32> {
    let name = CString::new(name.as_str()).expect("pjsip_endpt_create").into_raw();
    unsafe { check_status( pjsip_endpt_create(pf, name as *const _, endpt.as_mut() as *mut _)) }
}

pub fn endpt_destroy(endpt: &mut pjsip_endpoint) {
    unsafe { pjsip_endpt_destroy(endpt as *mut _); }
}

// const pj_str_t * 	pjsip_endpt_name (const pjsip_endpoint *endpt)

pub fn endpt_handle_events(endpt: &mut pjsip_endpoint, max_timeout: &mut pj_time_val) -> Result<(), i32> {
    unsafe {
        check_status( pjsip_endpt_handle_events(endpt as *mut _, max_timeout as *const _))
    }
}

pub fn endpt_handle_events2(endpt: &mut pjsip_endpoint, max_timeout: &mut pj_time_val, count: &mut u32) -> Result<(), i32> {
    unsafe {
        check_status(pjsip_endpt_handle_events2(endpt as *mut _, max_timeout as *const _, count as *mut _))}
}

// pj_status_t 	pjsip_endpt_schedule_timer (pjsip_endpoint *endpt, pj_timer_entry *entry, const pj_time_val *delay)
// pub fn enpt_schedule_timer(endpt: &mut pjsip_endpoint, entry: &mut pj_timer_entry, delay: &mut pj_time_val) -> Result<(), i32> {
//     unsafe {
//         check_status(
//             pjsip_endpt_schedule_timer_dbg(

//             )
//         )
//     }
// }

// pj_status_t 	pjsip_endpt_schedule_timer_w_grp_lock (pjsip_endpoint *endpt, pj_timer_entry *entry, const pj_time_val *delay, int id_val, pj_grp_lock_t *grp_lock)

pub fn endpt_cancel_timer(endpt: &mut pjsip_endpoint, entry: &mut pj_timer_entry) {
    unsafe { pjsip_endpt_cancel_timer(endpt as *mut _, entry as *mut _) }
}

pub fn endpt_get_timer_heap(endpt: &mut pjsip_endpoint) -> *mut pj_timer_heap_t {
    unsafe { pjsip_endpt_get_timer_heap(endpt as *mut _) }
}

pub fn endpt_register_module(endpt: *mut pjsip_endpoint, module: &mut pjsip_module) -> Result<(), i32> {
    unsafe { check_status( pjsip_endpt_register_module(endpt, module as *mut _)) }
}

pub fn endpt_unregister_module(endpt: *mut pjsip_endpoint, module: &mut pjsip_module) -> Result<(), i32> {
    unsafe { check_status(pjsip_endpt_unregister_module(endpt, module as *mut _)) }
}

// void 	pjsip_process_rdata_param_default (pjsip_process_rdata_param *p)
pub fn process_rdata_param_default(p: &mut pjsip_process_rdata_param) {
    unsafe { pjsip_process_rdata_param_default(p as *mut _); }
}

// pj_status_t 	pjsip_endpt_process_rx_data (pjsip_endpoint *endpt, pjsip_rx_data *rdata, pjsip_process_rdata_param *p, pj_bool_t *p_handled)
// pj_pool_t * 	pjsip_endpt_create_pool (pjsip_endpoint *endpt, const char *pool_name, pj_size_t initial, pj_size_t increment)
// void 	pjsip_endpt_release_pool (pjsip_endpoint *endpt, pj_pool_t *pool)
// pjsip_transaction * 	pjsip_endpt_find_tsx (pjsip_endpoint *endpt, const pj_str_t *key)

// void 	pjsip_endpt_register_tsx (pjsip_endpoint *endpt, pjsip_transaction *tsx)
// void 	pjsip_endpt_destroy_tsx (pjsip_endpoint *endpt, pjsip_transaction *tsx)

// pj_status_t 	pjsip_endpt_create_tdata (pjsip_endpoint *endpt, pjsip_tx_data **p_tdata)
pub fn endpt_create_tdata(endpt: &mut pjsip_endpoint, p_tdata: &mut Box<*mut pjsip_tx_data> ) -> Result<(), i32> {
    unsafe {
        check_status( pjsip_endpt_create_tdata( endpt as *mut _, (p_tdata.as_mut() as *mut _) as *mut _))
    }
}

// pj_status_t 	pjsip_endpt_create_resolver (pjsip_endpoint *endpt, pj_dns_resolver **p_resv)
pub fn endpt_create_resolver(endpt: &mut pjsip_endpoint, p_resv: &mut Box<*mut pjsip_tx_data>) -> Result<(), i32> {
    unsafe {
        check_status( pjsip_endpt_create_resolver( endpt as *mut _, (p_resv.as_mut() as *mut _) as *mut _))
    }
}

// pj_status_t 	pjsip_endpt_set_resolver (pjsip_endpoint *endpt, pj_dns_resolver *resv)
pub fn endpt_set_resolver(endpt: &mut pjsip_endpoint, resv: &mut pj_dns_resolver) -> Result<(), i32> {
    unsafe { check_status(pjsip_endpt_set_resolver( endpt as *mut _, resv as *mut _)) }
}

// pj_status_t 	pjsip_endpt_set_ext_resolver (pjsip_endpoint *endpt, pjsip_ext_resolver *ext_res)
pub fn endpt_set_ext_resolver(endpt: &mut pjsip_endpoint, ext_res: &mut pjsip_ext_resolver) -> Result<(), i32> {
    unsafe { check_status( pjsip_endpt_set_ext_resolver( endpt as *mut _, ext_res as *mut _))}
}

// pj_dns_resolver * 	pjsip_endpt_get_resolver (pjsip_endpoint *endpt)
// void 	pjsip_endpt_resolve (pjsip_endpoint *endpt, pj_pool_t *pool, pjsip_host_info *target, void *token, pjsip_resolver_callback *cb)
// pjsip_tpmgr * 	pjsip_endpt_get_tpmgr (pjsip_endpoint *endpt)
// pj_ioqueue_t * 	pjsip_endpt_get_ioqueue (pjsip_endpoint *endpt)
// pj_status_t 	pjsip_endpt_acquire_transport (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_transport **p_tp)
// pj_status_t 	pjsip_endpt_acquire_transport2 (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_tx_data *tdata, pjsip_transport **p_tp)
// const pjsip_hdr * 	pjsip_endpt_get_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname)
// pj_bool_t 	pjsip_endpt_has_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname, const pj_str_t *token)
// pj_status_t 	pjsip_endpt_add_capability (pjsip_endpoint *endpt, pjsip_module *mod, int htype, const pj_str_t *hname, unsigned count, const pj_str_t tags[])
// const pjsip_hdr * 	pjsip_endpt_get_request_headers (pjsip_endpoint *e)
// void 	pjsip_endpt_dump (pjsip_endpoint *endpt, pj_bool_t detail)
// pj_status_t 	pjsip_endpt_atexit (pjsip_endpoint *endpt, pjsip_endpt_exit_callback func)
