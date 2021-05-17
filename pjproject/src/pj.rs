
use pj_sys::*;
use num_enum::*;

pub mod auto;
pub mod thread;


pub type PJStr = pj_str_t;
// pub type pj_timestamp__bindgen_ty_1 = pj_timestamp__bindgen_ty_1;
// pub type pj_hash_table_t = pj_hash_table_t;
// pub type pj_hash_entry = pj_hash_entry;
// pub type pj_hash_iterator_t = pj_hash_iterator_t;
// pub type pj_ioqueue_t = pj_ioqueue_t;
// pub type pj_ioqueue_key_t = pj_ioqueue_key_t;
// pub type pj_timer_heap_t = pj_timer_heap_t;
// pub type pj_atomic_t = pj_atomic_t;
// pub type PJThread = pj_thread_t;
// pub type pj_lock_t = pj_lock_t;
// pub type pj_grp_lock_t = pj_grp_lock_t;
// pub type pj_mutex_t = pj_mutex_t;
// pub type pj_sem_t = pj_sem_t;
// pub type pj_event_t = pj_event_t;
// pub type pj_pipe_t = pj_pipe_t;
// pub type pj_time_val = pj_time_val;
// pub type pj_parsed_time = pj_parsed_time;
// pub type pj_ioqueue_op_key_t = pj_ioqueue_op_key_t;
// pub type pj_ioqueue_callback = pj_ioqueue_callback;
// pub type in_addr = in_addr;
// pub type in6_addr = in6_addr;
// pub type pj_sockaddr_in = pj_sockaddr_in;
// pub type pj_sockaddr_in6 = pj_sockaddr_in6;
// pub type pj_addr_hdr = pj_addr_hdr;
// pub type pj_ip_mreq = pj_ip_mreq;
// pub type pj_sockopt_params = pj_sockopt_params;
// pub type pj_sockopt_params__bindgen_ty_1 = pj_sockopt_params__bindgen_ty_1;
// pub type pj_activesock_t = pj_activesock_t;
// pub type pj_activesock_cb = pj_activesock_cb;
// pub type pj_activesock_cfg = pj_activesock_cfg;
// pub type pj_hostent = pj_hostent;
// pub type pj_addrinfo = pj_addrinfo;
// pub type pj_exception_state_t = pj_exception_state_t;
// pub type pj_fifobuf_t = pj_fifobuf_t;
// pub type pj_file_stat = pj_file_stat;
// pub type pj_ip_route_entry__bindgen_ty_1 = pj_ip_route_entry__bindgen_ty_1;
// pub type pj_enum_ip_option = pj_enum_ip_option;
// pub type pj_list = pj_list;
// pub type pj_grp_lock_config = pj_grp_lock_config;
// pub type pj_math_stat = pj_math_stat;
// pub type pj_sys_info = pj_sys_info;
// pub type pj_symbianos_params = pj_symbianos_params;
// pub type pj_rwmutex_t = pj_rwmutex_t;
// pub type pj_pool_block = pj_pool_block;
pub type PJPool = pj_pool_t;
// pub type pj_pool_factory_policy = pj_pool_factory_policy;
// pub type pj_pool_factory = pj_pool_factory;
// pub type pj_caching_pool = pj_caching_pool;
// pub type pj_rbtree_node = pj_rbtree_node;
// pub type pj_rbtree = pj_rbtree;
// pub type pj_qos_params = pj_qos_params;
// pub type pj_fd_set_t = pj_fd_set_t;
// pub type pj_ssl_sock_t = pj_ssl_sock_t;
// pub type pj_ssl_cert_t = pj_ssl_cert_t;
// pub type pj_ssl_cert_info = pj_ssl_cert_info;
// pub type pj_ssl_cert_info__bindgen_ty_1 = pj_ssl_cert_info__bindgen_ty_1;
// pub type pj_ssl_cert_info__bindgen_ty_2 = pj_ssl_cert_info__bindgen_ty_2;
// pub type pj_ssl_cert_info__bindgen_ty_3 = pj_ssl_cert_info__bindgen_ty_3;
// pub type pj_ssl_cert_info__bindgen_ty_4 = pj_ssl_cert_info__bindgen_ty_4;
// pub type pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1 = pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1;
// pub type pj_ssl_cert_info__bindgen_ty_5 = pj_ssl_cert_info__bindgen_ty_5;
// pub type pj_ssl_sock_cb = pj_ssl_sock_cb;
// pub type pj_ssl_sock_info = pj_ssl_sock_info;
// pub type pj_ssl_sock_param = pj_ssl_sock_param;
// pub type pj_ssl_start_connect_param = pj_ssl_start_connect_param;
// pub type pj_timer_entry = pj_timer_entry;

/// pub type pj_ioqueue_operation_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IOQueueOperation {
    None = pj_sys::PJ_IOQUEUE_OP_NONE,
    Read = pj_sys::PJ_IOQUEUE_OP_READ,
    Recv = pj_sys::PJ_IOQUEUE_OP_RECV,
    RecvFrom = pj_sys::PJ_IOQUEUE_OP_RECV_FROM,
    Write = pj_sys::PJ_IOQUEUE_OP_WRITE,
    Send = pj_sys::PJ_IOQUEUE_OP_SEND,
    SendTo= pj_sys::PJ_IOQUEUE_OP_SEND_TO,
    Accept= pj_sys::PJ_IOQUEUE_OP_ACCEPT,
    Connect = pj_sys::PJ_IOQUEUE_OP_CONNECT,
}

/// pub type pj_socket_sd_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SocketSdType {
    Receive = pj_sys::PJ_SD_RECEIVE,
    // ShutRd = pj_sys::PJ_SHUT_RD,
    SdSend = pj_sys::PJ_SD_SEND,
    // ShutWr= pj_sys::PJ_SHUT_WR,
    SdBoth = pj_sys::PJ_SD_BOTH,
    // Rdwr = pj_sys::PJ_SHUT_RDWR,
}

/// pub type pj_log_decoration = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum LogDecoration {
    HasDayName = pj_sys::PJ_LOG_HAS_DAY_NAME,
    HasYear = pj_sys::PJ_LOG_HAS_YEAR,
    HasMonth = pj_sys::PJ_LOG_HAS_MONTH,
    HasDayOfMon = pj_sys::PJ_LOG_HAS_DAY_OF_MON,
    HasTime = pj_sys::PJ_LOG_HAS_TIME,
    HasMicroSec = pj_sys::PJ_LOG_HAS_MICRO_SEC,
    HasSender = pj_sys::PJ_LOG_HAS_SENDER,
    HasNewline = pj_sys::PJ_LOG_HAS_NEWLINE,
    HasCr = pj_sys::PJ_LOG_HAS_CR,
    HasSpace = pj_sys::PJ_LOG_HAS_SPACE,
    HasColor = pj_sys::PJ_LOG_HAS_COLOR,
    HasLevelText = pj_sys::PJ_LOG_HAS_LEVEL_TEXT,
    HasThreadId = pj_sys::PJ_LOG_HAS_THREAD_ID,
    HasThreadSwc = pj_sys::PJ_LOG_HAS_THREAD_SWC,
    HasIdent = pj_sys::PJ_LOG_HAS_INDENT,
}

/// pub type pj_file_access = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum FileAccess {
    ReadOnly = pj_sys::PJ_O_RDONLY,
    WriteOnly= pj_sys::PJ_O_WRONLY,
    ReadWrite = pj_sys::PJ_O_RDWR,
    Append = pj_sys::PJ_O_APPEND,
}

/// pub type pj_file_seek_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum FileSeekType {
    Set = pj_sys::PJ_SEEK_SET,
    Cur = pj_sys::PJ_SEEK_CUR,
    End = pj_sys::PJ_SEEK_END,
}

/// pub type pj_mutex_type_e = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MutexType {
    Default = pj_sys::PJ_MUTEX_DEFAULT,
    Simple = pj_sys::PJ_MUTEX_SIMPLE,
    Recurse = pj_sys::PJ_MUTEX_RECURSE,
}

/// pub type pj_qos_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum QOSType {
    BestEffort = pj_sys::PJ_QOS_TYPE_BEST_EFFORT,
    Background = pj_sys::PJ_QOS_TYPE_BACKGROUND,
    Video = pj_sys::PJ_QOS_TYPE_VIDEO,
    Voice = pj_sys::PJ_QOS_TYPE_VOICE,
    Control = pj_sys::PJ_QOS_TYPE_CONTROL,
    Signalling = pj_sys::PJ_QOS_TYPE_SIGNALLING,
}


/// pub type pj_qos_flag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum QOSFlag {
    HasDscp = pj_sys::PJ_QOS_PARAM_HAS_DSCP,
    HasSoPrio= pj_sys::PJ_QOS_PARAM_HAS_SO_PRIO,
    HasWmm = pj_sys::PJ_QOS_PARAM_HAS_WMM,
}

/// pub type pj_qos_wmm_prio = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum QOSWmmPrio {
    BulkEffort = pj_sys::PJ_QOS_WMM_PRIO_BULK_EFFORT,
    Bulk = pj_sys::PJ_QOS_WMM_PRIO_BULK,
    Video = pj_sys::PJ_QOS_WMM_PRIO_VIDEO,
    Voice = pj_sys::PJ_QOS_WMM_PRIO_VOICE,
}


/// pub type pj_ssl_cert_verify_flag_t = i32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SSLCertVerifyFlag {
    ESuccess = pj_sys::PJ_SSL_CERT_ESUCCESS,
    EIssuerNotFound = pj_sys::PJ_SSL_CERT_EISSUER_NOT_FOUND,
    EUntrusted = pj_sys::PJ_SSL_CERT_EUNTRUSTED,
    EValidityPeriod = pj_sys::PJ_SSL_CERT_EVALIDITY_PERIOD,
    EInvalidFormat = pj_sys::PJ_SSL_CERT_EINVALID_FORMAT,
    EInvalidPurpose = pj_sys::PJ_SSL_CERT_EINVALID_PURPOSE,
    EIssuerMismatch = pj_sys::PJ_SSL_CERT_EISSUER_MISMATCH,
    ECrlFailure = pj_sys::PJ_SSL_CERT_ECRL_FAILURE,
    ERevoked = pj_sys::PJ_SSL_CERT_EREVOKED,
    EChainTooLong = pj_sys::PJ_SSL_CERT_ECHAIN_TOO_LONG,
    EIdentityNotMatch = pj_sys::PJ_SSL_CERT_EIDENTITY_NOT_MATCH,
    EUnknown = pj_sys::PJ_SSL_CERT_EUNKNOWN,
}

/// pub type pj_ssl_cert_name_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SSLCertNameType {
    Unknown = pj_sys::PJ_SSL_CERT_NAME_UNKNOWN,
    Rfc822 = pj_sys::PJ_SSL_CERT_NAME_RFC822, 
    Dns = pj_sys::PJ_SSL_CERT_NAME_DNS, 
    Uri = pj_sys::PJ_SSL_CERT_NAME_URI,
    Ip = pj_sys::PJ_SSL_CERT_NAME_IP,
}

///  pub type pj_ssl_cipher = i32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SSLCipher {
    UnknownCipher = pj_sys::PJ_TLS_UNKNOWN_CIPHER,
    NullWithNullNull = pj_sys::PJ_TLS_NULL_WITH_NULL_NULL,
    RsaWithNullMd5 = pj_sys::PJ_TLS_RSA_WITH_NULL_MD5,
    RsaWithNullSha= pj_sys::PJ_TLS_RSA_WITH_NULL_SHA,
    RsaWithNullSha256 = pj_sys::PJ_TLS_RSA_WITH_NULL_SHA256,
    RsaWithRc4_128Md5 = pj_sys::PJ_TLS_RSA_WITH_RC4_128_MD5,
    RsaWithRc4_128Sha = pj_sys::PJ_TLS_RSA_WITH_RC4_128_SHA,
    RsaWith3desEdeCbcSha = pj_sys::PJ_TLS_RSA_WITH_3DES_EDE_CBC_SHA,
    RsaWithAes128CbcSha = pj_sys::PJ_TLS_RSA_WITH_AES_128_CBC_SHA,
    RsaWithAes256CbcSha = pj_sys::PJ_TLS_RSA_WITH_AES_256_CBC_SHA,
    RsaWithAes128CbcSha256 = pj_sys::PJ_TLS_RSA_WITH_AES_128_CBC_SHA256,
    RsaWithAes256CbcSha256 = pj_sys::PJ_TLS_RSA_WITH_AES_256_CBC_SHA256,
    DhDssWith3desEdeCbcSha = pj_sys::PJ_TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA,
    DhRsaWith3desEdeCbcSha = pj_sys::PJ_TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA,
    DheDssWith3desEdeCbcSha = pj_sys::PJ_TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA,
    DheRsaWith3desEdeCbcSha = pj_sys::PJ_TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA,
    DhDssWithAes128CbcSha = pj_sys::PJ_TLS_DH_DSS_WITH_AES_128_CBC_SHA,
    DhRsaWithAes128CbcSha = pj_sys::PJ_TLS_DH_RSA_WITH_AES_128_CBC_SHA,
    DheDssWithAes128CbcSha = pj_sys::PJ_TLS_DHE_DSS_WITH_AES_128_CBC_SHA,
    DheRsaWithAes128CbcSha = pj_sys::PJ_TLS_DHE_RSA_WITH_AES_128_CBC_SHA,
    DhDssWithAes256CbcSha = pj_sys::PJ_TLS_DH_DSS_WITH_AES_256_CBC_SHA,
    DhRsaWithAes256CbcSha = pj_sys::PJ_TLS_DH_RSA_WITH_AES_256_CBC_SHA,
    DheDssWithAes256CbcSha = pj_sys::PJ_TLS_DHE_DSS_WITH_AES_256_CBC_SHA,
    DheRsaWithAes256CbcSha = pj_sys::PJ_TLS_DHE_RSA_WITH_AES_256_CBC_SHA,
    DhDssWithAes128CbcSha256 = pj_sys::PJ_TLS_DH_DSS_WITH_AES_128_CBC_SHA256,
    DhRsaWithAes128CbcSha256 = pj_sys::PJ_TLS_DH_RSA_WITH_AES_128_CBC_SHA256,
    DheDssWithAes128CbcSha256 = pj_sys::PJ_TLS_DHE_DSS_WITH_AES_128_CBC_SHA256,
    DheRsaWithAes128CbcSha256 = pj_sys::PJ_TLS_DHE_RSA_WITH_AES_128_CBC_SHA256,
    DhDssWithAes256CbcSha256 = pj_sys::PJ_TLS_DH_DSS_WITH_AES_256_CBC_SHA256,
    DhRsaWithAes256CbcSha256 = pj_sys::PJ_TLS_DH_RSA_WITH_AES_256_CBC_SHA256,
    DheDssWithAes256CbcSha256 = pj_sys::PJ_TLS_DHE_DSS_WITH_AES_256_CBC_SHA256,
    DheRsaWithAes256CbcSha256 = pj_sys::PJ_TLS_DHE_RSA_WITH_AES_256_CBC_SHA256,
    DhAnonWithRc4_128Md5 = pj_sys::PJ_TLS_DH_anon_WITH_RC4_128_MD5,
    DhAnonWith3desEdeCbcSha = pj_sys::PJ_TLS_DH_anon_WITH_3DES_EDE_CBC_SHA,
    DhAnonWithAes128CbcSha = pj_sys::PJ_TLS_DH_anon_WITH_AES_128_CBC_SHA,
    DhAnonWithAes256CbcSha = pj_sys::PJ_TLS_DH_anon_WITH_AES_256_CBC_SHA,
    DhAnonWithAes128CbcSha256 = pj_sys::PJ_TLS_DH_anon_WITH_AES_128_CBC_SHA256,
    DhAnonWithAes256CbcSha256 = pj_sys::PJ_TLS_DH_anon_WITH_AES_256_CBC_SHA256,
    RsaExportWithRc4_40Md5 = pj_sys::PJ_TLS_RSA_EXPORT_WITH_RC4_40_MD5,
    RsaExportWithRc2Cbc40Md5 = pj_sys::PJ_TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5,
    RsaWithIdeaCbcSha = pj_sys::PJ_TLS_RSA_WITH_IDEA_CBC_SHA,
    RsaExportWithDes40CbcSha = pj_sys::PJ_TLS_RSA_EXPORT_WITH_DES40_CBC_SHA,
    RsaWithDesCbcSha = pj_sys::PJ_TLS_RSA_WITH_DES_CBC_SHA,
    DhDssExportWithDes40CbcSha = pj_sys::PJ_TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA,
    DhDssWithDesCbcSha = pj_sys::PJ_TLS_DH_DSS_WITH_DES_CBC_SHA,
    DhRsaExportWithDes40CbcSha = pj_sys::PJ_TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA,
    DhRsaWithDesCbcSha = pj_sys::PJ_TLS_DH_RSA_WITH_DES_CBC_SHA,
    DheDssExportWithDes40CbcSha = pj_sys::PJ_TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA,
    DheDssWithDesCbcSha = pj_sys::PJ_TLS_DHE_DSS_WITH_DES_CBC_SHA,
    DheRsaExportWithDes40CbcSha = pj_sys::PJ_TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA,
    DheRsaWithDesCbcSha = pj_sys::PJ_TLS_DHE_RSA_WITH_DES_CBC_SHA,
    DhAnonExportWithRc4_40Md5 = pj_sys::PJ_TLS_DH_anon_EXPORT_WITH_RC4_40_MD5,
    DhAnonExportWithDes40CbcSha = pj_sys::PJ_TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA,
    DhAnonWithDesCbcSha = pj_sys::PJ_TLS_DH_anon_WITH_DES_CBC_SHA,
    FortezzaKeaWithNullSha = pj_sys::PJ_SSL_FORTEZZA_KEA_WITH_NULL_SHA,
    FortezzaKeaWithFortezzaCbcSha = pj_sys::PJ_SSL_FORTEZZA_KEA_WITH_FORTEZZA_CBC_SHA,
    FortezzaKeaWithRc4_128Sha = pj_sys::PJ_SSL_FORTEZZA_KEA_WITH_RC4_128_SHA,
    CkRc4_128WithMd5 = pj_sys::PJ_SSL_CK_RC4_128_WITH_MD5,
    CkRc4_128Export40WithMd5 = pj_sys::PJ_SSL_CK_RC4_128_EXPORT40_WITH_MD5,
    CkRc2_128CbcWithMd5 = pj_sys::PJ_SSL_CK_RC2_128_CBC_WITH_MD5,
    CkRc2_128CbcExport40WithMd5 = pj_sys::PJ_SSL_CK_RC2_128_CBC_EXPORT40_WITH_MD5,
    CkIdea128CbcWithMd5 = pj_sys::PJ_SSL_CK_IDEA_128_CBC_WITH_MD5,
    CkDes64CbcWithMd5 = pj_sys::PJ_SSL_CK_DES_64_CBC_WITH_MD5,
    CkDes192Ede3CbcWithMd5 = pj_sys::PJ_SSL_CK_DES_192_EDE3_CBC_WITH_MD5,
}

// pub type pj_ssl_curve = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SSLCurve {
    UnknownCurve = pj_sys::PJ_TLS_UNKNOWN_CURVE,
    Sect163K1 = pj_sys::PJ_TLS_CURVE_SECT163K1,
    Sect163R1 = pj_sys::PJ_TLS_CURVE_SECT163R1,
    Sect163R2 = pj_sys::PJ_TLS_CURVE_SECT163R2,
    Sect193R1 = pj_sys::PJ_TLS_CURVE_SECT193R1,
    Sect193R2 = pj_sys::PJ_TLS_CURVE_SECT193R2,
    Sect233K1 = pj_sys::PJ_TLS_CURVE_SECT233K1,
    Sect233R1 = pj_sys::PJ_TLS_CURVE_SECT233R1,
    Sect239K1 = pj_sys::PJ_TLS_CURVE_SECT239K1,
    Sect283K1 = pj_sys::PJ_TLS_CURVE_SECT283K1,
    Sect283R1 = pj_sys::PJ_TLS_CURVE_SECT283R1,
    Sect409K1 = pj_sys::PJ_TLS_CURVE_SECT409K1,
    Sect409R1 = pj_sys::PJ_TLS_CURVE_SECT409R1, 
    Sect571K1 = pj_sys::PJ_TLS_CURVE_SECT571K1, 
    Sect571R1 = pj_sys::PJ_TLS_CURVE_SECT571R1,
    Secp160K1 = pj_sys::PJ_TLS_CURVE_SECP160K1,
    Secp160R1 = pj_sys::PJ_TLS_CURVE_SECP160R1,
    Secp160R2 = pj_sys::PJ_TLS_CURVE_SECP160R2,
    Secp192K1 = pj_sys::PJ_TLS_CURVE_SECP192K1,
    Secp192R1 = pj_sys::PJ_TLS_CURVE_SECP192R1,
    Secp224K1 = pj_sys::PJ_TLS_CURVE_SECP224K1,
    Secp224R1 = pj_sys::PJ_TLS_CURVE_SECP224R1, 
    Secp256K1 = pj_sys::PJ_TLS_CURVE_SECP256K1,
    Secp256R1 = pj_sys::PJ_TLS_CURVE_SECP256R1, 
    Secp384R1 = pj_sys::PJ_TLS_CURVE_SECP384R1,
    Secp521R1 = pj_sys::PJ_TLS_CURVE_SECP521R1,
    BrainPoolP256R1 = pj_sys::PJ_TLS_CURVE_BRAINPOOLP256R1,
    BrainPoolP384R1 = pj_sys::PJ_TLS_CURVE_BRAINPOOLP384R1, 
    BrainPoolP512R1 = pj_sys::PJ_TLS_CURVE_BRAINPOOLP512R1, 
    ArbitraryExplicitPrimeCurves = pj_sys::PJ_TLS_CURVE_ARBITRARY_EXPLICIT_PRIME_CURVES,
    ArbitraryExplicitChar2Curves = pj_sys::PJ_TLS_CURVE_ARBITRARY_EXPLICIT_CHAR2_CURVES,
}

/// pub type pj_ssl_entropy = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SSLEntropy {
    None = pj_sys::PJ_SSL_ENTROPY_NONE,
    Egd = pj_sys::PJ_SSL_ENTROPY_EGD,
    Random = pj_sys::PJ_SSL_ENTROPY_RANDOM,
    Urandom = pj_sys::PJ_SSL_ENTROPY_URANDOM,
    File = pj_sys::PJ_SSL_ENTROPY_FILE,
    Unknown = pj_sys::PJ_SSL_ENTROPY_UNKNOWN,
}

/// pub type pj_ssl_sock_proto = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SSLSockProto {
    DEFAULT = pj_sys::PJ_SSL_SOCK_PROTO_DEFAULT,
    SSL2 = pj_sys::PJ_SSL_SOCK_PROTO_SSL2,
    SSL3 = pj_sys::PJ_SSL_SOCK_PROTO_SSL3,
    TLS1 = pj_sys::PJ_SSL_SOCK_PROTO_TLS1,
    TLS1_1 = pj_sys::PJ_SSL_SOCK_PROTO_TLS1_1,
    TLS1_2 = pj_sys::PJ_SSL_SOCK_PROTO_TLS1_2,
    TLS1_3 = pj_sys::PJ_SSL_SOCK_PROTO_TLS1_3,
    // SSL23 = pj_sys::PJ_SSL_SOCK_PROTO_SSL23,
    ALL = pj_sys::PJ_SSL_SOCK_PROTO_ALL,
    DTLS1 = pj_sys::PJ_SSL_SOCK_PROTO_DTLS1,
}


