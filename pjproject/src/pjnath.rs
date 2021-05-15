
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

// pub type pj_stun_msg_hdr = pjnath_sys::pj_stun_msg_hdr;
// pub type pj_stun_attr_hdr = pjnath_sys::pj_stun_attr_hdr;
// pub type pj_stun_sockaddr_attr = pjnath_sys::pj_stun_sockaddr_attr;
// pub type pj_stun_empty_attr = pjnath_sys::pj_stun_empty_attr;
// pub type pj_stun_string_attr = pjnath_sys::pj_stun_string_attr;
// pub type pj_stun_uint_attr = pjnath_sys::pj_stun_uint_attr;
// pub type pj_stun_uint64_attr = pjnath_sys::pj_stun_uint64_attr;
// pub type pj_stun_binary_attr = pjnath_sys::pj_stun_binary_attr;
// pub type pj_stun_msgint_attr = pjnath_sys::pj_stun_msgint_attr;
// pub type pj_stun_errcode_attr = pjnath_sys::pj_stun_errcode_attr;
// pub type pj_stun_unknown_attr = pjnath_sys::pj_stun_unknown_attr;
// pub type pj_stun_msg = pjnath_sys::pj_stun_msg;
// pub type pj_stun_auth_cred = pjnath_sys::pj_stun_auth_cred;
// pub type pj_stun_auth_cred__bindgen_ty_1 = pjnath_sys::pj_stun_auth_cred__bindgen_ty_1;
// pub type pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 = pjnath_sys::pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1;
// pub type pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 = pjnath_sys::pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2;
// pub type pj_stun_req_cred_info = pjnath_sys::pj_stun_req_cred_info;
// pub type pj_stun_config = pjnath_sys::pj_stun_config;
// pub type pj_stun_client_tsx = pjnath_sys::pj_stun_client_tsx;
// pub type pj_stun_tsx_cb = pjnath_sys::pj_stun_tsx_cb;
// pub type pj_stun_session = pjnath_sys::pj_stun_session;
// pub type pj_stun_session_cb = pjnath_sys::pj_stun_session_cb;
// pub type pj_stun_rx_data = pjnath_sys::pj_stun_rx_data;
// pub type pj_stun_tx_data = pjnath_sys::pj_stun_tx_data;
// pub type pj_ice_sess_comp = pjnath_sys::pj_ice_sess_comp;
// pub type pj_ice_msg_data = pjnath_sys::pj_ice_msg_data;
// pub type pj_ice_msg_data_data_request_data = pjnath_sys::pj_ice_msg_data_data_request_data;
// pub type pj_ice_sess_cand = pjnath_sys::pj_ice_sess_cand;
// pub type pj_ice_sess_check = pjnath_sys::pj_ice_sess_check;
// pub type pj_ice_sess_checklist = pjnath_sys::pj_ice_sess_checklist;
// pub type pj_ice_sess_cb = pjnath_sys::pj_ice_sess_cb;
// pub type pj_ice_rx_check = pjnath_sys::pj_ice_rx_check;
// pub type pj_ice_sess_options = pjnath_sys::pj_ice_sess_options;
// pub type pj_ice_sess = pjnath_sys::pj_ice_sess;
// pub type pj_stun_sock = pjnath_sys::pj_stun_sock;
// pub type pj_stun_sock_cb = pjnath_sys::pj_stun_sock_cb;
// pub type pj_stun_sock_info = pjnath_sys::pj_stun_sock_info;
// pub type pj_stun_sock_cfg = pjnath_sys::pj_stun_sock_cfg;
// pub type pj_turn_session = pjnath_sys::pj_turn_session;
// pub type pj_turn_channel_data = pjnath_sys::pj_turn_channel_data;
// pub type pj_turn_session_cb = pjnath_sys::pj_turn_session_cb;
// pub type pj_turn_alloc_param = pjnath_sys::pj_turn_alloc_param;
// pub type pj_turn_session_info = pjnath_sys::pj_turn_session_info;
// pub type pj_turn_session_on_rx_pkt_param = pjnath_sys::pj_turn_session_on_rx_pkt_param;
// pub type pj_turn_sock = pjnath_sys::pj_turn_sock;
// pub type pj_turn_sock_cb = pjnath_sys::pj_turn_sock_cb;
// pub type pj_turn_sock_tls_cfg = pjnath_sys::pj_turn_sock_tls_cfg;
// pub type pj_turn_sock_cfg = pjnath_sys::pj_turn_sock_cfg;
// pub type pj_ice_strans = pjnath_sys::pj_ice_strans;
// pub type pj_ice_strans_cb = pjnath_sys::pj_ice_strans_cb;
// pub type pj_ice_strans_stun_cfg = pjnath_sys::pj_ice_strans_stun_cfg;
// pub type pj_ice_strans_turn_cfg = pjnath_sys::pj_ice_strans_turn_cfg;
// pub type pj_ice_strans_cfg = pjnath_sys::pj_ice_strans_cfg;
// pub type pj_ice_strans_cfg__bindgen_ty_1 = pjnath_sys::pj_ice_strans_cfg__bindgen_ty_1;
pub type STUNNatDetectResult = pjnath_sys::pj_stun_nat_detect_result;


/// pub type pj_stun_method_e = u32;
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

/// pub type pj_stun_msg_class_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunMessageClass {
    Request = pjnath_sys::PJ_STUN_REQUEST_CLASS,
    Indication = pjnath_sys::PJ_STUN_INDICATION_CLASS,
    Success = pjnath_sys::PJ_STUN_SUCCESS_CLASS,
    Error = pjnath_sys::PJ_STUN_ERROR_CLASS,
}

/// pub type pj_stun_msg_type = u32;
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

/// pub type pj_stun_attr_type = u32;
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


/// pub type pj_stun_status = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunStatus {
    TryAlternate = pjnath_sys::PJ_STUN_SC_TRY_ALTERNATE,
    BadRequest = pjnath_sys::PJ_STUN_SC_BAD_REQUEST,
    UnAuthorized = pjnath_sys::PJ_STUN_SC_UNAUTHORIZED,
    Forbiden = pjnath_sys::PJ_STUN_SC_FORBIDDEN,
    UnknownAttribute = pjnath_sys::PJ_STUN_SC_UNKNOWN_ATTRIBUTE,
    AllocationMismatch = pjnath_sys::PJ_STUN_SC_ALLOCATION_MISMATCH,
    StaleNonce = pjnath_sys::PJ_STUN_SC_STALE_NONCE,
    Transitioning = pjnath_sys::PJ_STUN_SC_TRANSITIONING,
    WrongCredentials = pjnath_sys::PJ_STUN_SC_WRONG_CREDENTIALS,
    UnsuppTransportProto = pjnath_sys::PJ_STUN_SC_UNSUPP_TRANSPORT_PROTO,
    OperTcpOnly = pjnath_sys::PJ_STUN_SC_OPER_TCP_ONLY,
    ConnectionFailure = pjnath_sys::PJ_STUN_SC_CONNECTION_FAILURE,
    ConnectionTimeout = pjnath_sys::PJ_STUN_SC_CONNECTION_TIMEOUT,
    AllocationQuotaReached = pjnath_sys::PJ_STUN_SC_ALLOCATION_QUOTA_REACHED,
    RoloConflict = pjnath_sys::PJ_STUN_SC_ROLE_CONFLICT,
    ServerError = pjnath_sys::PJ_STUN_SC_SERVER_ERROR,
    InsufficientCapacity = pjnath_sys::PJ_STUN_SC_INSUFFICIENT_CAPACITY,
    GlobalFailure = pjnath_sys::PJ_STUN_SC_GLOBAL_FAILURE,
}


///  pub type pj_stun_decode_options = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunDecodeOptions {
    IsDatagram = pjnath_sys::PJ_STUN_IS_DATAGRAM,
    CheckPacket = pjnath_sys::PJ_STUN_CHECK_PACKET,
    Authenticate = pjnath_sys::PJ_STUN_NO_AUTHENTICATE,
    NoFingerprintCheck = pjnath_sys::PJ_STUN_NO_FINGERPRINT_CHECK,
}

/// pub type pj_stun_auth_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunAuthType {
    None = pjnath_sys::PJ_STUN_AUTH_NONE,
    ShortTerm = pjnath_sys::PJ_STUN_AUTH_SHORT_TERM,
    LongTerm = pjnath_sys::PJ_STUN_AUTH_LONG_TERM,
}

/// pub type pj_stun_auth_cred_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunAuthCredType {
    Static = pjnath_sys::PJ_STUN_AUTH_CRED_STATIC,
    CredDynamic = pjnath_sys::PJ_STUN_AUTH_CRED_DYNAMIC,
}

/// pub type pj_stun_passwd_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunPasswdType {
    Plain = pjnath_sys::PJ_STUN_PASSWD_PLAIN,
    Hashed = pjnath_sys::PJ_STUN_PASSWD_HASHED,
}

/// pub type pj_stun_sess_msg_log_flag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunSessMsgLogFlag {
    TxReq = pjnath_sys::PJ_STUN_SESS_LOG_TX_REQ,
    TxRes = pjnath_sys::PJ_STUN_SESS_LOG_TX_RES,
    TxInd = pjnath_sys::PJ_STUN_SESS_LOG_TX_IND,
    RxReq = pjnath_sys::PJ_STUN_SESS_LOG_RX_REQ,
    RxRes = pjnath_sys::PJ_STUN_SESS_LOG_RX_RES,
    RxInd = pjnath_sys::PJ_STUN_SESS_LOG_RX_IND,
}

/// pub type pj_ice_cand_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceCandType {
    Host = pjnath_sys::PJ_ICE_CAND_TYPE_HOST,
    Srflx = pjnath_sys::PJ_ICE_CAND_TYPE_SRFLX,
    Prflx = pjnath_sys::PJ_ICE_CAND_TYPE_PRFLX,
    Relayed = pjnath_sys::PJ_ICE_CAND_TYPE_RELAYED,
    Max = pjnath_sys::PJ_ICE_CAND_TYPE_MAX,
}

/// pub type pj_ice_sess_check_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceSessCheckState {
    Frozen = pjnath_sys::PJ_ICE_SESS_CHECK_STATE_FROZEN,
    Waiting = pjnath_sys::PJ_ICE_SESS_CHECK_STATE_WAITING,
    InProgress = pjnath_sys::PJ_ICE_SESS_CHECK_STATE_IN_PROGRESS,
    Succeded = pjnath_sys::PJ_ICE_SESS_CHECK_STATE_SUCCEEDED,
    Failed = pjnath_sys::PJ_ICE_SESS_CHECK_STATE_FAILED,
}

/// pub type pj_ice_sess_checklist_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceSessChecklistState {
    Idle = pjnath_sys::PJ_ICE_SESS_CHECKLIST_ST_IDLE,
    Running  = pjnath_sys::PJ_ICE_SESS_CHECKLIST_ST_RUNNING,
    Completed = pjnath_sys::PJ_ICE_SESS_CHECKLIST_ST_COMPLETED,
}

/// pub type pj_ice_sess_role = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceSessRole {
    Unknown = pjnath_sys::PJ_ICE_SESS_ROLE_UNKNOWN,
    Controlled = pjnath_sys::PJ_ICE_SESS_ROLE_CONTROLLED,
    Controllin = pjnath_sys::PJ_ICE_SESS_ROLE_CONTROLLING,
}

/// pub type pj_ice_sess_trickle = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceSessTrickle {
    Disabled = pjnath_sys::PJ_ICE_SESS_TRICKLE_DISABLED,
    Half = pjnath_sys::PJ_ICE_SESS_TRICKLE_HALF,
    Full = pjnath_sys::PJ_ICE_SESS_TRICKLE_FULL,
}

/// pub type pj_stun_sock_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunSockOp {
    Dns = pjnath_sys::PJ_STUN_SOCK_DNS_OP,
    Binding = pjnath_sys::PJ_STUN_SOCK_BINDING_OP,
    KeepAlive = pjnath_sys::PJ_STUN_SOCK_KEEP_ALIVE_OP,
    MappedAddrChange = pjnath_sys::PJ_STUN_SOCK_MAPPED_ADDR_CHANGE,
}

/// pub type pj_turn_tp_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum TurnTpType {
    Udp = pjnath_sys::PJ_TURN_TP_UDP,
    Tcp = pjnath_sys::PJ_TURN_TP_TCP,
    Tls = pjnath_sys::PJ_TURN_TP_TLS,
}

/// pub type pj_turn_state_t = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum TurnState {
    Null = pjnath_sys::PJ_TURN_STATE_NULL,
    Resolving = pjnath_sys::PJ_TURN_STATE_RESOLVING,
    Resolved = pjnath_sys::PJ_TURN_STATE_RESOLVED,
    Allocating = pjnath_sys::PJ_TURN_STATE_ALLOCATING,
    Ready = pjnath_sys::PJ_TURN_STATE_READY,
    Deallocating = pjnath_sys::PJ_TURN_STATE_DEALLOCATING,
    Deallocated = pjnath_sys::PJ_TURN_STATE_DEALLOCATED,
    Destroying = pjnath_sys::PJ_TURN_STATE_DESTROYING,
}

/// pub type pj_ice_strans_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceStransOp {
    Init = pjnath_sys::PJ_ICE_STRANS_OP_INIT,
    Negotiation = pjnath_sys::PJ_ICE_STRANS_OP_NEGOTIATION,
    KeepAlive = pjnath_sys::PJ_ICE_STRANS_OP_KEEP_ALIVE,
    AddrChange = pjnath_sys::PJ_ICE_STRANS_OP_ADDR_CHANGE,
}

/// pub type pj_ice_strans_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IceStransState {
    Null = pjnath_sys::PJ_ICE_STRANS_STATE_NULL,
    Init = pjnath_sys::PJ_ICE_STRANS_STATE_INIT,
    Ready = pjnath_sys::PJ_ICE_STRANS_STATE_READY,
    SessReady = pjnath_sys::PJ_ICE_STRANS_STATE_SESS_READY,
    Nego = pjnath_sys::PJ_ICE_STRANS_STATE_NEGO,
    Running = pjnath_sys::PJ_ICE_STRANS_STATE_RUNNING,
    Failed = pjnath_sys::PJ_ICE_STRANS_STATE_FAILED,
}

/// pub type pj_stun_nat_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StunNatType {
    Unknown = pjnath_sys::PJ_STUN_NAT_TYPE_UNKNOWN,
    ErrUnknown = pjnath_sys::PJ_STUN_NAT_TYPE_ERR_UNKNOWN,
    Open = pjnath_sys::PJ_STUN_NAT_TYPE_OPEN,
    Blocked = pjnath_sys::PJ_STUN_NAT_TYPE_BLOCKED,
    SymmetricUdp = pjnath_sys::PJ_STUN_NAT_TYPE_SYMMETRIC_UDP,
    FullCone = pjnath_sys::PJ_STUN_NAT_TYPE_FULL_CONE,
    Symmetric = pjnath_sys::PJ_STUN_NAT_TYPE_SYMMETRIC,
    Restricted = pjnath_sys::PJ_STUN_NAT_TYPE_RESTRICTED,
    PortRestricted = pjnath_sys::PJ_STUN_NAT_TYPE_PORT_RESTRICTED,
}
