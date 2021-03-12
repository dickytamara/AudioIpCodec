#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_uint, c_short, c_ushort,
    c_void, c_ulong, c_ulonglong, c_long,
    c_longlong, c_uchar, c_schar};

// pub const PJ_CC_NAME: &'static [u8; 4usize] = b"gcc\0";
// pub const PJ_INT64_FMT: &'static [u8; 2usize] = b"L\0";
// pub const PJ_LINUX: u32 = 1;
// pub const PJ_OS_NAME: &'static [u8; 6usize] = b"linux\0";
// pub const PJ_SOCK_HAS_INET_ATON: u32 = 1;
// pub const PJ_SOCKADDR_HAS_LEN: u32 = 0;
// pub const PJ_SELECT_NEEDS_NFDS: u32 = 0;
// pub const PJ_OS_HAS_CHECK_STACK: u32 = 0;
// pub const PJ_NATIVE_STRING_IS_UNICODE: u32 = 0;
// pub const PJ_EMULATE_RWMUTEX: u32 = 0;
// pub const PJ_THREAD_SET_STACK_SIZE: u32 = 0;
// pub const PJ_THREAD_ALLOCATE_STACK: u32 = 0;
// pub const PJ_M_X86_64: u32 = 1;
// pub const PJ_M_NAME: &'static [u8; 7usize] = b"x86_64\0";
// pub const PJ_IS_LITTLE_ENDIAN: u32 = 1;
// pub const PJ_IS_BIG_ENDIAN: u32 = 0;
// pub const PJ_DEBUG: u32 = 1;
// pub const PJ_DEBUG_MUTEX: u32 = 0;
// pub const PJ_FUNCTIONS_ARE_INLINED: u32 = 0;
// pub const PJ_LOG_MAX_LEVEL: u32 = 5;
// pub const PJ_LOG_MAX_SIZE: u32 = 4000;
// pub const PJ_LOG_USE_STACK_BUFFER: u32 = 1;
// pub const PJ_LOG_ENABLE_INDENT: u32 = 1;
// pub const PJ_LOG_INDENT_SIZE: u32 = 1;
// pub const PJ_LOG_INDENT_CHAR: u8 = 46u8;
// pub const PJ_LOG_SENDER_WIDTH: u32 = 14;
// pub const PJ_LOG_THREAD_WIDTH: u32 = 12;
// pub const PJ_TERM_HAS_COLOR: u32 = 1;
// pub const PJ_SAFE_POOL: u32 = 0;
// pub const PJ_POOL_DEBUG: u32 = 0;
// pub const PJ_POOL_RELEASE_WIPE_DATA: u32 = 0;
// pub const PJ_TIMER_DEBUG: u32 = 1;
// pub const PJ_TIMER_USE_COPY: u32 = 1;
// pub const PJ_TIMER_USE_LINKED_LIST: u32 = 0;
// pub const PJ_GRP_LOCK_DEBUG: u32 = 0;
// pub const PJ_THREAD_DEFAULT_STACK_SIZE: u32 = 8192;
// pub const PJ_MAX_HOSTNAME: u32 = 128;
// pub const PJ_ACTIVESOCK_MAX_CONSECUTIVE_ACCEPT_ERROR: u32 = 50;
// pub const PJ_IOQUEUE_MAX_HANDLES: u32 = 64;
// pub const PJ_IOQUEUE_HAS_SAFE_UNREG: u32 = 1;
// pub const PJ_IOQUEUE_DEFAULT_ALLOW_CONCURRENCY: u32 = 1;
// pub const PJ_IOQUEUE_KEY_FREE_DELAY: u32 = 500;
// pub const PJ_FD_SETSIZE_SETABLE: u32 = 0;
// pub const PJ_IP_HELPER_IGNORE_LOOPBACK_IF: u32 = 1;
// pub const PJ_SEMAPHORE_USE_DISPATCH_SEM: u32 = 0;
// pub const PJ_MAXPATH: u32 = 260;
// pub const PJ_ENABLE_EXTRA_CHECK: u32 = 1;
// pub const PJ_MAX_EXCEPTION_ID: u32 = 16;
// pub const PJ_EXCEPTION_USE_WIN32_SEH: u32 = 0;
// pub const PJ_TIMESTAMP_USE_RDTSC: u32 = 0;
// pub const PJ_NATIVE_ERR_POSITIVE: u32 = 1;

// pub const PJ_QOS_DUMMY: u32 = 1;
// pub const PJ_QOS_BSD: u32 = 2;
// pub const PJ_QOS_WM: u32 = 3;
// pub const PJ_QOS_SYMBIAN: u32 = 4;
// pub const PJ_QOS_DARWIN: u32 = 5;

// pub const PJ_SSL_SOCK_IMP_NONE: u32 = 0;
// pub const PJ_SSL_SOCK_IMP_OPENSSL: u32 = 1;
// pub const PJ_SSL_SOCK_IMP_GNUTLS: u32 = 2;
// pub const PJ_SSL_SOCK_IMP_DARWIN: u32 = 3;
// pub const PJ_SSL_SOCK_IMP_APPLE: u32 = 4;
// pub const PJ_SSL_SOCK_IMP: u32 = 0;
// pub const PJ_SSL_SOCK_MAX_CIPHERS: u32 = 256;
// pub const PJ_SSL_SOCK_OSSL_CIPHERS: &'static [u8; 26usize] = b"HIGH:-COMPLEMENTOFDEFAULT\0";
// pub const PJ_SSL_SOCK_MAX_CURVES: u32 = 32;
// pub const PJ_SSL_SOCK_OSSL_USE_THREAD_CB: u32 = 1;

// pub const PJ_SOCK_DISABLE_WSAECONNRESET: u32 = 1;
// pub const PJ_MAX_SOCKOPT_PARAMS: u32 = 4;
// pub const PJ_VERSION_NUM_MAJOR: u32 = 2;
// pub const PJ_VERSION_NUM_MINOR: u32 = 10;
// pub const PJ_VERSION_NUM_REV: u32 = 0;
// pub const PJ_VERSION_NUM_EXTRA: &'static [u8; 5usize] = b"-dev\0";
// pub const PJ_VERSION_NUM: u32 = 34209792;
// pub const PJ_MAX_OBJ_NAME: u32 = 32;
// pub const PJ_ERR_MSG_SIZE: u32 = 80;
// pub const PJ_PERROR_TITLE_BUF_SIZE: u32 = 120;
// pub const PJ_ERRNO_START: u32 = 20000;
// pub const PJ_ERRNO_SPACE_SIZE: u32 = 50000;
// pub const PJ_ERRNO_START_STATUS: u32 = 70000;
// pub const PJ_ERRNO_START_SYS: u32 = 120000;
// pub const PJ_ERRNO_START_USER: u32 = 170000;

// pub const PJ_DNS_MAX_IP_IN_A_REC: u32 = 8;
// pub const PJ_DNS_SRV_MAX_ADDR: u32 = 8;
// pub const PJ_DNS_MAX_NAMES_IN_NAMETABLE: u32 = 16;
// pub const PJ_DNS_RESOLVER_MAX_NS: u32 = 16;
// pub const PJ_DNS_RESOLVER_QUERY_RETRANSMIT_DELAY: u32 = 2000;
// pub const PJ_DNS_RESOLVER_QUERY_RETRANSMIT_COUNT: u32 = 5;
// pub const PJ_DNS_RESOLVER_MAX_TTL: u32 = 300;
// pub const PJ_DNS_RESOLVER_INVALID_TTL: u32 = 60;
// pub const PJ_DNS_RESOLVER_GOOD_NS_TTL: u32 = 600;
// pub const PJ_DNS_RESOLVER_BAD_NS_TTL: u32 = 60;
// pub const PJ_DNS_RESOLVER_MAX_UDP_SIZE: u32 = 512;
// pub const PJ_DNS_RESOLVER_RES_BUF_SIZE: u32 = 512;
// pub const PJ_DNS_RESOLVER_TMP_BUF_SIZE: u32 = 4000;
// pub const PJ_SCANNER_USE_BITWISE: u32 = 1;
// pub const PJSTUN_MAX_ATTR: u32 = 16;
// pub const PJ_STUN_MAX_ATTR: u32 = 16;
// pub const PJ_CRC32_HAS_TABLES: u32 = 1;
// pub const PJ_HTTP_DEFAULT_TIMEOUT: u32 = 60000;
// pub const PJ_CLI_POOL_SIZE: u32 = 1024;
// pub const PJ_CLI_POOL_INC: u32 = 512;
// pub const PJ_CLI_MAX_CMDBUF: u32 = 512;
// pub const PJ_CLI_MAX_ARGS: u32 = 8;
// pub const PJ_CLI_MAX_HINTS: u32 = 32;
// pub const PJ_CLI_MAX_SHORTCUTS: u32 = 4;
// pub const PJ_CLI_CONSOLE_POOL_SIZE: u32 = 256;
// pub const PJ_CLI_CONSOLE_POOL_INC: u32 = 256;
// pub const PJ_CLI_TELNET_POOL_SIZE: u32 = 1024;
// pub const PJ_CLI_TELNET_POOL_INC: u32 = 512;
// pub const PJ_CLI_MAX_CHOICE_VAL: u32 = 64;
// pub const PJ_CLI_MAX_CMD_HISTORY: u32 = 16;
// pub const PJ_SOMAXCONN: u32 = 5;
// pub const PJ_INVALID_SOCKET: i32 = -1;
// pub const PJ_INET_ADDRSTRLEN: u32 = 16;
// pub const PJ_INET6_ADDRSTRLEN: u32 = 46;
// pub const PJ_SOCKADDR_IN_SIN_ZERO_LEN: u32 = 8;
// pub const PJ_IOQUEUE_MAX_EVENTS_IN_SINGLE_POLL: u32 = 16;
// pub const PJ_IOQUEUE_MAX_CAND_EVENTS: u32 = 16;
// pub const PJ_POOL_ALIGNMENT: u32 = 4;
// pub const PJ_CACHING_POOL_ARRAY_SIZE: u32 = 16;
// pub const PJ_PI: f64 = 3.141592653589793;
// pub const PJ_1_PI: f64 = 0.3183098861837907;
// pub const PJ_THREAD_DESC_SIZE: u32 = 64;
// pub const PJNATH_ERROR_LEVEL: u32 = 1;
// pub const PJ_STUN_RTO_VALUE: u32 = 100;
// pub const PJ_STUN_TIMEOUT_VALUE: u32 = 1600;
// pub const PJ_STUN_MAX_TRANSMIT_COUNT: u32 = 7;
// pub const PJ_STUN_RES_CACHE_DURATION: u32 = 10000;
// pub const PJ_STUN_MAX_PKT_LEN: u32 = 800;
// pub const PJ_STUN_PORT: u32 = 3478;
// pub const PJ_STUN_STRING_ATTR_PAD_CHR: u32 = 0;
// pub const PJ_STUN_OLD_STYLE_MI_FINGERPRINT: u32 = 0;
// pub const PJ_STUN_SOCK_PKT_LEN: u32 = 2000;
// pub const PJ_STUN_KEEP_ALIVE_SEC: u32 = 15;
// pub const PJ_TURN_MAX_DNS_SRV_CNT: u32 = 4;
// pub const PJ_TURN_MAX_PKT_LEN: u32 = 3000;
// pub const PJ_TURN_PERM_TIMEOUT: u32 = 300;
// pub const PJ_TURN_CHANNEL_TIMEOUT: u32 = 600;
// pub const PJ_TURN_REFRESH_SEC_BEFORE: u32 = 60;
// pub const PJ_TURN_KEEP_ALIVE_SEC: u32 = 15;
// pub const PJ_TURN_MAX_TCP_CONN_CNT: u32 = 8;
// pub const PJ_ICE_MAX_CAND: u32 = 16;
// pub const PJ_ICE_ST_MAX_CAND: u32 = 8;
// pub const PJ_ICE_MAX_STUN: u32 = 2;
// pub const PJ_ICE_MAX_TURN: u32 = 3;
// pub const PJ_ICE_COMP_BITS: u32 = 1;
// pub const PJ_ICE_MAX_COMP: u32 = 2;
// pub const PJNATH_ICE_PRIO_STD: u32 = 1;
// pub const PJ_ICE_CAND_TYPE_PREF_BITS: u32 = 8;
// pub const PJ_ICE_LOCAL_PREF_BITS: u32 = 0;
// pub const PJ_ICE_MAX_CHECKS: u32 = 32;
// pub const PJ_ICE_TA_VAL: u32 = 20;
// pub const PJ_ICE_CANCEL_ALL: u32 = 1;
// pub const PJ_ICE_NOMINATED_CHECK_DELAY: u32 = 400;
// pub const PJ_ICE_SESS_KEEP_ALIVE_MIN: u32 = 20;
// pub const PJ_ICE_SESS_KEEP_ALIVE_MAX_RAND: u32 = 5;
// pub const PJ_ICE_UFRAG_LEN: u32 = 8;
// pub const PJ_ICE_PWD_LEN: u32 = 24;
// pub const PJNATH_POOL_LEN_ICE_SESS: u32 = 512;
// pub const PJNATH_POOL_INC_ICE_SESS: u32 = 512;
// pub const PJNATH_POOL_LEN_ICE_STRANS: u32 = 1000;
// pub const PJNATH_POOL_INC_ICE_STRANS: u32 = 512;
// pub const PJNATH_POOL_LEN_NATCK: u32 = 512;
// pub const PJNATH_POOL_INC_NATCK: u32 = 512;
// pub const PJNATH_POOL_LEN_STUN_SESS: u32 = 1000;
// pub const PJNATH_POOL_INC_STUN_SESS: u32 = 1000;
// pub const PJNATH_POOL_LEN_STUN_TDATA: u32 = 1000;
// pub const PJNATH_POOL_INC_STUN_TDATA: u32 = 1000;
// pub const PJNATH_POOL_LEN_TURN_SESS: u32 = 1000;
// pub const PJNATH_POOL_INC_TURN_SESS: u32 = 1000;
// pub const PJNATH_POOL_LEN_TURN_SOCK: u32 = 1000;
// pub const PJNATH_POOL_INC_TURN_SOCK: u32 = 1000;
// pub const PJ_TURN_INVALID_CHANNEL: u32 = 65535;
// pub const PJ_STUN_MAGIC: u32 = 554869826;
// pub const PJ_STUN_SUCCESS_RESPONSE_BIT: u32 = 256;
// pub const PJ_STUN_ERROR_RESPONSE_BIT: u32 = 272;
// pub const PJ_STUN_INDICATION_BIT: u32 = 16;
// pub const PJNATH_ERRNO_START: u32 = 370000;
// pub const PJNATH_EINSTUNMSG: u32 = 370001;
// pub const PJNATH_EINSTUNMSGLEN: u32 = 370002;
// pub const PJNATH_EINSTUNMSGTYPE: u32 = 370003;
// pub const PJNATH_ESTUNTIMEDOUT: u32 = 370004;
// pub const PJNATH_ESTUNTOOMANYATTR: u32 = 370021;
// pub const PJNATH_ESTUNINATTRLEN: u32 = 370022;
// pub const PJNATH_ESTUNDUPATTR: u32 = 370023;
// pub const PJNATH_ESTUNFINGERPRINT: u32 = 370030;
// pub const PJNATH_ESTUNMSGINTPOS: u32 = 370031;
// pub const PJNATH_ESTUNFINGERPOS: u32 = 370033;
// pub const PJNATH_ESTUNNOMAPPEDADDR: u32 = 370040;
// pub const PJNATH_ESTUNIPV6NOTSUPP: u32 = 370041;
// pub const PJNATH_EINVAF: u32 = 370042;
// pub const PJNATH_ESTUNINSERVER: u32 = 370050;
// pub const PJNATH_ESTUNDESTROYED: u32 = 370060;
// pub const PJNATH_ENOICE: u32 = 370080;
// pub const PJNATH_EICEINPROGRESS: u32 = 370081;
// pub const PJNATH_EICEFAILED: u32 = 370082;
// pub const PJNATH_EICEMISMATCH: u32 = 370083;
// pub const PJNATH_EICEINCOMPID: u32 = 370086;
// pub const PJNATH_EICEINCANDID: u32 = 370087;
// pub const PJNATH_EICEINSRCADDR: u32 = 370088;
// pub const PJNATH_EICEMISSINGSDP: u32 = 370090;
// pub const PJNATH_EICEINCANDSDP: u32 = 370091;
// pub const PJNATH_EICENOHOSTCAND: u32 = 370092;
// pub const PJNATH_EICENOMTIMEOUT: u32 = 370093;
// pub const PJNATH_ETURNINTP: u32 = 370120;

// pub const PJLIB_UTIL_ERRNO_START: u32 = 320000;
// pub const PJLIB_UTIL_ESTUNRESOLVE: u32 = 320001;
// pub const PJLIB_UTIL_ESTUNINMSGTYPE: u32 = 320002;
// pub const PJLIB_UTIL_ESTUNINMSGLEN: u32 = 320003;
// pub const PJLIB_UTIL_ESTUNINATTRLEN: u32 = 320004;
// pub const PJLIB_UTIL_ESTUNINATTRTYPE: u32 = 320005;
// pub const PJLIB_UTIL_ESTUNININDEX: u32 = 320006;
// pub const PJLIB_UTIL_ESTUNNOBINDRES: u32 = 320007;
// pub const PJLIB_UTIL_ESTUNRECVERRATTR: u32 = 320008;
// pub const PJLIB_UTIL_ESTUNNOMAP: u32 = 320009;
// pub const PJLIB_UTIL_ESTUNNOTRESPOND: u32 = 320010;
// pub const PJLIB_UTIL_ESTUNSYMMETRIC: u32 = 320011;
// pub const PJLIB_UTIL_ESTUNNOTMAGIC: u32 = 320012;
// pub const PJLIB_UTIL_ESTUNFINGERPRINT: u32 = 320013;
// pub const PJLIB_UTIL_EINXML: u32 = 320020;
// pub const PJLIB_UTIL_EINJSON: u32 = 320030;
// pub const PJLIB_UTIL_EDNSQRYTOOSMALL: u32 = 320040;
// pub const PJLIB_UTIL_EDNSINSIZE: u32 = 320041;
// pub const PJLIB_UTIL_EDNSINCLASS: u32 = 320042;
// pub const PJLIB_UTIL_EDNSINNAMEPTR: u32 = 320043;
// pub const PJLIB_UTIL_EDNSINNSADDR: u32 = 320044;
// pub const PJLIB_UTIL_EDNSNONS: u32 = 320045;
// pub const PJLIB_UTIL_EDNSNOWORKINGNS: u32 = 320046;
// pub const PJLIB_UTIL_EDNSNOANSWERREC: u32 = 320047;
// pub const PJLIB_UTIL_EDNSINANSWER: u32 = 320048;
// pub const PJLIB_UTIL_DNS_RCODE_START: u32 = 320050;
// pub const PJLIB_UTIL_ESTUNTOOMANYATTR: u32 = 320110;
// pub const PJLIB_UTIL_ESTUNUNKNOWNATTR: u32 = 320111;
// pub const PJLIB_UTIL_ESTUNINADDRLEN: u32 = 320112;
// pub const PJLIB_UTIL_ESTUNIPV6NOTSUPP: u32 = 320113;
// pub const PJLIB_UTIL_ESTUNNOTRESPONSE: u32 = 320114;
// pub const PJLIB_UTIL_ESTUNINVALIDID: u32 = 320115;
// pub const PJLIB_UTIL_ESTUNNOHANDLER: u32 = 320116;
// pub const PJLIB_UTIL_ESTUNMSGINTPOS: u32 = 320118;
// pub const PJLIB_UTIL_ESTUNFINGERPOS: u32 = 320119;
// pub const PJLIB_UTIL_ESTUNNOUSERNAME: u32 = 320120;
// pub const PJLIB_UTIL_ESTUNUSERNAME: u32 = 320121;
// pub const PJLIB_UTIL_ESTUNMSGINT: u32 = 320122;
// pub const PJLIB_UTIL_ESTUNDUPATTR: u32 = 320123;
// pub const PJLIB_UTIL_ESTUNNOREALM: u32 = 320124;
// pub const PJLIB_UTIL_ESTUNNONCE: u32 = 320125;
// pub const PJLIB_UTIL_ESTUNTSXFAILED: u32 = 320126;
// pub const PJLIB_UTIL_EHTTPINURL: u32 = 320151;
// pub const PJLIB_UTIL_EHTTPINPORT: u32 = 320152;
// pub const PJLIB_UTIL_EHTTPINCHDR: u32 = 320153;
// pub const PJLIB_UTIL_EHTTPINSBUF: u32 = 320154;
// pub const PJLIB_UTIL_EHTTPLOST: u32 = 320155;

// pub const PJ_CLI_EEXIT: u32 = 320201;
// pub const PJ_CLI_EMISSINGARG: u32 = 320202;
// pub const PJ_CLI_ETOOMANYARGS: u32 = 320203;
// pub const PJ_CLI_EINVARG: u32 = 320204;
// pub const PJ_CLI_EBADNAME: u32 = 320205;
// pub const PJ_CLI_EBADID: u32 = 320206;
// pub const PJ_CLI_EBADXML: u32 = 320207;
// pub const PJ_CLI_EAMBIGUOUS: u32 = 320208;
// pub const PJ_CLI_ETELNETLOST: u32 = 320211;
pub const PJ_SHA1_DIGEST_SIZE: u32 = 20;
pub const PJ_HTTP_HEADER_SIZE: u32 = 32;

pub const PJ_CLI_CONSOLE_LOG_LEVEL: u32 = 5;
pub const PJ_CLI_TELNET_LOG_LEVEL: u32 = 4;
pub const PJ_CLI_TELNET_PORT: u32 = 0;

pub const pj_hex_digits: &'static [u8; 17usize] = b"0123456789abcdef\0";
pub const PJ_GUID_MAX_LENGTH: u32 = 36;
pub const PJ_SCAN_AUTOSKIP_WS: c_uint = 1;
pub const PJ_SCAN_AUTOSKIP_WS_HEADER: c_uint = 3;
pub const PJ_SCAN_AUTOSKIP_NEWLINE: c_uint = 4;

pub type pj_int32_t = c_int;
pub type pj_uint32_t = c_uint;
pub type pj_int16_t = c_short;
pub type pj_uint16_t = c_ushort;
pub type pj_int8_t = c_schar;
pub type pj_uint8_t = c_uchar;
pub type pj_size_t = size_t;
pub type pj_ssize_t = c_long;
pub type pj_status_t = c_int;
pub type pj_bool_t = c_int;
pub type pj_char_t = c_char;
pub type pj_int64_t = c_longlong;
pub type pj_uint64_t = c_ulonglong;
pub type size_t = c_ulong;

pub const PJ_SUCCESS: pj_constants_ = 0;
pub const PJ_TRUE: pj_constants_ = 1;
pub const PJ_FALSE: pj_constants_ = 0;

pub type _bindgen_ty_1 = c_uint;
pub type _bindgen_ty_2 = c_uint;
pub type _bindgen_ty_3 = c_uint;
pub type _bindgen_ty_4 = c_uint;
pub type _bindgen_ty_15 = c_uint;

pub type __uint8_t = c_uchar;
pub type __uint16_t = c_ushort;
pub type __uint32_t = c_uint;

pub type pj_syn_err_func_ptr = Option<unsafe extern "C" fn(scanner: *mut pj_scanner)>;
pub const PJ_TERM_COLOR_R: c_uint = 2;
pub const PJ_TERM_COLOR_G: c_uint = 4;
pub const PJ_TERM_COLOR_B: c_uint = 1;
pub const PJ_TERM_COLOR_BRIGHT: c_uint = 8;

pub type pj_os_err_type = c_int;
pub type va_list = __builtin_va_list;
pub type pj_list_type = c_void;
pub type wchar_t = c_int;
pub type pj_constants_ = c_uint;
pub type pj_off_t = pj_int64_t;
pub type pj_oshandle_t = *mut c_void;
pub type pj_sock_t = c_long;
pub type pj_sockaddr_t = c_void;
pub type pj_color_t = c_uint;
pub type pj_exception_id_t = c_int;
pub type pj_cis_elem_t = pj_uint32_t;

pub const pj_socket_sd_type_PJ_SD_RECEIVE: pj_socket_sd_type = 0;
pub const pj_socket_sd_type_PJ_SHUT_RD: pj_socket_sd_type = 0;
pub const pj_socket_sd_type_PJ_SD_SEND: pj_socket_sd_type = 1;
pub const pj_socket_sd_type_PJ_SHUT_WR: pj_socket_sd_type = 1;
pub const pj_socket_sd_type_PJ_SD_BOTH: pj_socket_sd_type = 2;
pub const pj_socket_sd_type_PJ_SHUT_RDWR: pj_socket_sd_type = 2;
pub type pj_socket_sd_type = c_uint;
pub type pj_in_addr = in_addr;

pub const PJ_DNS_CLASS_IN: c_uint = 1;
pub type _bindgen_ty_14 = c_uint;
pub const pj_dns_type_PJ_DNS_TYPE_A: pj_dns_type = 1;
pub const pj_dns_type_PJ_DNS_TYPE_NS: pj_dns_type = 2;
pub const pj_dns_type_PJ_DNS_TYPE_MD: pj_dns_type = 3;
pub const pj_dns_type_PJ_DNS_TYPE_MF: pj_dns_type = 4;
pub const pj_dns_type_PJ_DNS_TYPE_CNAME: pj_dns_type = 5;
pub const pj_dns_type_PJ_DNS_TYPE_SOA: pj_dns_type = 6;
pub const pj_dns_type_PJ_DNS_TYPE_MB: pj_dns_type = 7;
pub const pj_dns_type_PJ_DNS_TYPE_MG: pj_dns_type = 8;
pub const pj_dns_type_PJ_DNS_TYPE_MR: pj_dns_type = 9;
pub const pj_dns_type_PJ_DNS_TYPE_NULL: pj_dns_type = 10;
pub const pj_dns_type_PJ_DNS_TYPE_WKS: pj_dns_type = 11;
pub const pj_dns_type_PJ_DNS_TYPE_PTR: pj_dns_type = 12;
pub const pj_dns_type_PJ_DNS_TYPE_HINFO: pj_dns_type = 13;
pub const pj_dns_type_PJ_DNS_TYPE_MINFO: pj_dns_type = 14;
pub const pj_dns_type_PJ_DNS_TYPE_MX: pj_dns_type = 15;
pub const pj_dns_type_PJ_DNS_TYPE_TXT: pj_dns_type = 16;
pub const pj_dns_type_PJ_DNS_TYPE_RP: pj_dns_type = 17;
pub const pj_dns_type_PJ_DNS_TYPE_AFSB: pj_dns_type = 18;
pub const pj_dns_type_PJ_DNS_TYPE_X25: pj_dns_type = 19;
pub const pj_dns_type_PJ_DNS_TYPE_ISDN: pj_dns_type = 20;
pub const pj_dns_type_PJ_DNS_TYPE_RT: pj_dns_type = 21;
pub const pj_dns_type_PJ_DNS_TYPE_NSAP: pj_dns_type = 22;
pub const pj_dns_type_PJ_DNS_TYPE_NSAP_PTR: pj_dns_type = 23;
pub const pj_dns_type_PJ_DNS_TYPE_SIG: pj_dns_type = 24;
pub const pj_dns_type_PJ_DNS_TYPE_KEY: pj_dns_type = 25;
pub const pj_dns_type_PJ_DNS_TYPE_PX: pj_dns_type = 26;
pub const pj_dns_type_PJ_DNS_TYPE_GPOS: pj_dns_type = 27;
pub const pj_dns_type_PJ_DNS_TYPE_AAAA: pj_dns_type = 28;
pub const pj_dns_type_PJ_DNS_TYPE_LOC: pj_dns_type = 29;
pub const pj_dns_type_PJ_DNS_TYPE_NXT: pj_dns_type = 30;
pub const pj_dns_type_PJ_DNS_TYPE_EID: pj_dns_type = 31;
pub const pj_dns_type_PJ_DNS_TYPE_NIMLOC: pj_dns_type = 32;
pub const pj_dns_type_PJ_DNS_TYPE_SRV: pj_dns_type = 33;
pub const pj_dns_type_PJ_DNS_TYPE_ATMA: pj_dns_type = 34;
pub const pj_dns_type_PJ_DNS_TYPE_NAPTR: pj_dns_type = 35;
pub const pj_dns_type_PJ_DNS_TYPE_KX: pj_dns_type = 36;
pub const pj_dns_type_PJ_DNS_TYPE_CERT: pj_dns_type = 37;
pub const pj_dns_type_PJ_DNS_TYPE_A6: pj_dns_type = 38;
pub const pj_dns_type_PJ_DNS_TYPE_DNAME: pj_dns_type = 39;
pub const pj_dns_type_PJ_DNS_TYPE_OPT: pj_dns_type = 41;
pub const pj_dns_type_PJ_DNS_TYPE_APL: pj_dns_type = 42;
pub const pj_dns_type_PJ_DNS_TYPE_DS: pj_dns_type = 43;
pub const pj_dns_type_PJ_DNS_TYPE_SSHFP: pj_dns_type = 44;
pub const pj_dns_type_PJ_DNS_TYPE_IPSECKEY: pj_dns_type = 45;
pub const pj_dns_type_PJ_DNS_TYPE_RRSIG: pj_dns_type = 46;
pub const pj_dns_type_PJ_DNS_TYPE_NSEC: pj_dns_type = 47;
pub const pj_dns_type_PJ_DNS_TYPE_DNSKEY: pj_dns_type = 48;
pub type pj_dns_type = c_uint;

pub const pj_dns_rcode_PJ_DNS_RCODE_FORMERR: pj_dns_rcode = 1;
pub const pj_dns_rcode_PJ_DNS_RCODE_SERVFAIL: pj_dns_rcode = 2;
pub const pj_dns_rcode_PJ_DNS_RCODE_NXDOMAIN: pj_dns_rcode = 3;
pub const pj_dns_rcode_PJ_DNS_RCODE_NOTIMPL: pj_dns_rcode = 4;
pub const pj_dns_rcode_PJ_DNS_RCODE_REFUSED: pj_dns_rcode = 5;
pub const pj_dns_rcode_PJ_DNS_RCODE_YXDOMAIN: pj_dns_rcode = 6;
pub const pj_dns_rcode_PJ_DNS_RCODE_YXRRSET: pj_dns_rcode = 7;
pub const pj_dns_rcode_PJ_DNS_RCODE_NXRRSET: pj_dns_rcode = 8;
pub const pj_dns_rcode_PJ_DNS_RCODE_NOTAUTH: pj_dns_rcode = 9;
pub const pj_dns_rcode_PJ_DNS_RCODE_NOTZONE: pj_dns_rcode = 10;
pub type pj_dns_rcode = c_uint;

pub const pj_dns_dup_options_PJ_DNS_NO_QD: pj_dns_dup_options = 1;
pub const pj_dns_dup_options_PJ_DNS_NO_ANS: pj_dns_dup_options = 2;
pub const pj_dns_dup_options_PJ_DNS_NO_NS: pj_dns_dup_options = 4;
pub const pj_dns_dup_options_PJ_DNS_NO_AR: pj_dns_dup_options = 8;
pub type pj_dns_dup_options = c_uint;

pub const PJ_IOQUEUE_OP_NONE: pj_ioqueue_operation_e = 0;
pub const PJ_IOQUEUE_OP_READ: pj_ioqueue_operation_e = 1;
pub const PJ_IOQUEUE_OP_RECV: pj_ioqueue_operation_e = 2;
pub const PJ_IOQUEUE_OP_RECV_FROM: pj_ioqueue_operation_e = 4;
pub const PJ_IOQUEUE_OP_WRITE: pj_ioqueue_operation_e = 8;
pub const PJ_IOQUEUE_OP_SEND: pj_ioqueue_operation_e = 16;
pub const PJ_IOQUEUE_OP_SEND_TO: pj_ioqueue_operation_e = 32;
pub const PJ_IOQUEUE_OP_ACCEPT: pj_ioqueue_operation_e = 64;
pub const PJ_IOQUEUE_OP_CONNECT: pj_ioqueue_operation_e = 128;
pub type pj_ioqueue_operation_e = c_uint;

pub const PJ_QOS_TYPE_BEST_EFFORT: pj_qos_type = 0;
pub const PJ_QOS_TYPE_BACKGROUND: pj_qos_type = 1;
pub const PJ_QOS_TYPE_VIDEO: pj_qos_type = 2;
pub const PJ_QOS_TYPE_VOICE: pj_qos_type = 3;
pub const PJ_QOS_TYPE_CONTROL: pj_qos_type = 4;
pub const PJ_QOS_TYPE_SIGNALLING: pj_qos_type = 5;
pub type pj_qos_type = c_uint;

pub const pj_qos_flag_PJ_QOS_PARAM_HAS_DSCP: pj_qos_flag = 1;
pub const pj_qos_flag_PJ_QOS_PARAM_HAS_SO_PRIO: pj_qos_flag = 2;
pub const pj_qos_flag_PJ_QOS_PARAM_HAS_WMM: pj_qos_flag = 4;
pub type pj_qos_flag = c_uint;

pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_BULK_EFFORT: pj_qos_wmm_prio = 0;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_BULK: pj_qos_wmm_prio = 1;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_VIDEO: pj_qos_wmm_prio = 2;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_VOICE: pj_qos_wmm_prio = 3;
pub type pj_qos_wmm_prio = c_uint;

pub const PJ_SSL_CERT_ESUCCESS: pj_ssl_cert_verify_flag_t = 0;
pub const PJ_SSL_CERT_EISSUER_NOT_FOUND: pj_ssl_cert_verify_flag_t = 1;
pub const PJ_SSL_CERT_EUNTRUSTED: pj_ssl_cert_verify_flag_t = 2;
pub const PJ_SSL_CERT_EVALIDITY_PERIOD: pj_ssl_cert_verify_flag_t = 4;
pub const PJ_SSL_CERT_EINVALID_FORMAT: pj_ssl_cert_verify_flag_t = 8;
pub const PJ_SSL_CERT_EINVALID_PURPOSE: pj_ssl_cert_verify_flag_t = 16;
pub const PJ_SSL_CERT_EISSUER_MISMATCH: pj_ssl_cert_verify_flag_t = 32;
pub const PJ_SSL_CERT_ECRL_FAILURE: pj_ssl_cert_verify_flag_t = 64;
pub const PJ_SSL_CERT_EREVOKED: pj_ssl_cert_verify_flag_t = 128;
pub const PJ_SSL_CERT_ECHAIN_TOO_LONG: pj_ssl_cert_verify_flag_t = 256;
pub const PJ_SSL_CERT_EIDENTITY_NOT_MATCH: pj_ssl_cert_verify_flag_t = 1073741824;
pub const PJ_SSL_CERT_EUNKNOWN: pj_ssl_cert_verify_flag_t = -2147483648;
pub type pj_ssl_cert_verify_flag_t = c_int;

pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_UNKNOWN: pj_ssl_cert_name_type = 0;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_RFC822: pj_ssl_cert_name_type = 1;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_DNS: pj_ssl_cert_name_type = 2;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_URI: pj_ssl_cert_name_type = 3;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_IP: pj_ssl_cert_name_type = 4;
pub type pj_ssl_cert_name_type = c_uint;

pub const PJ_TLS_UNKNOWN_CIPHER: pj_ssl_cipher = -1;
pub const PJ_TLS_NULL_WITH_NULL_NULL: pj_ssl_cipher = 0;
pub const PJ_TLS_RSA_WITH_NULL_MD5: pj_ssl_cipher = 1;
pub const PJ_TLS_RSA_WITH_NULL_SHA: pj_ssl_cipher = 2;
pub const PJ_TLS_RSA_WITH_NULL_SHA256: pj_ssl_cipher = 59;
pub const PJ_TLS_RSA_WITH_RC4_128_MD5: pj_ssl_cipher = 4;
pub const PJ_TLS_RSA_WITH_RC4_128_SHA: pj_ssl_cipher = 5;
pub const PJ_TLS_RSA_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 10;
pub const PJ_TLS_RSA_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 47;
pub const PJ_TLS_RSA_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 53;
pub const PJ_TLS_RSA_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 60;
pub const PJ_TLS_RSA_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 61;
pub const PJ_TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 13;
pub const PJ_TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 16;
pub const PJ_TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 19;
pub const PJ_TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 22;
pub const PJ_TLS_DH_DSS_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 48;
pub const PJ_TLS_DH_RSA_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 49;
pub const PJ_TLS_DHE_DSS_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 50;
pub const PJ_TLS_DHE_RSA_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 51;
pub const PJ_TLS_DH_DSS_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 54;
pub const PJ_TLS_DH_RSA_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 55;
pub const PJ_TLS_DHE_DSS_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 56;
pub const PJ_TLS_DHE_RSA_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 57;
pub const PJ_TLS_DH_DSS_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 62;
pub const PJ_TLS_DH_RSA_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 63;
pub const PJ_TLS_DHE_DSS_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 64;
pub const PJ_TLS_DHE_RSA_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 103;
pub const PJ_TLS_DH_DSS_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 104;
pub const PJ_TLS_DH_RSA_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 105;
pub const PJ_TLS_DHE_DSS_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 106;
pub const PJ_TLS_DHE_RSA_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 107;
pub const PJ_TLS_DH_anon_WITH_RC4_128_MD5: pj_ssl_cipher = 24;
pub const PJ_TLS_DH_anon_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 27;
pub const PJ_TLS_DH_anon_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 52;
pub const PJ_TLS_DH_anon_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 58;
pub const PJ_TLS_DH_anon_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 108;
pub const PJ_TLS_DH_anon_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 109;
pub const PJ_TLS_RSA_EXPORT_WITH_RC4_40_MD5: pj_ssl_cipher = 3;
pub const PJ_TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5: pj_ssl_cipher = 6;
pub const PJ_TLS_RSA_WITH_IDEA_CBC_SHA: pj_ssl_cipher = 7;
pub const PJ_TLS_RSA_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 8;
pub const PJ_TLS_RSA_WITH_DES_CBC_SHA: pj_ssl_cipher = 9;
pub const PJ_TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 11;
pub const PJ_TLS_DH_DSS_WITH_DES_CBC_SHA: pj_ssl_cipher = 12;
pub const PJ_TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 14;
pub const PJ_TLS_DH_RSA_WITH_DES_CBC_SHA: pj_ssl_cipher = 15;
pub const PJ_TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 17;
pub const PJ_TLS_DHE_DSS_WITH_DES_CBC_SHA: pj_ssl_cipher = 18;
pub const PJ_TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 20;
pub const PJ_TLS_DHE_RSA_WITH_DES_CBC_SHA: pj_ssl_cipher = 21;
pub const PJ_TLS_DH_anon_EXPORT_WITH_RC4_40_MD5: pj_ssl_cipher = 23;
pub const PJ_TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 25;
pub const PJ_TLS_DH_anon_WITH_DES_CBC_SHA: pj_ssl_cipher = 26;
pub const PJ_SSL_FORTEZZA_KEA_WITH_NULL_SHA: pj_ssl_cipher = 28;
pub const PJ_SSL_FORTEZZA_KEA_WITH_FORTEZZA_CBC_SHA: pj_ssl_cipher = 29;
pub const PJ_SSL_FORTEZZA_KEA_WITH_RC4_128_SHA: pj_ssl_cipher = 30;
pub const PJ_SSL_CK_RC4_128_WITH_MD5: pj_ssl_cipher = 65664;
pub const PJ_SSL_CK_RC4_128_EXPORT40_WITH_MD5: pj_ssl_cipher = 131200;
pub const PJ_SSL_CK_RC2_128_CBC_WITH_MD5: pj_ssl_cipher = 196736;
pub const PJ_SSL_CK_RC2_128_CBC_EXPORT40_WITH_MD5: pj_ssl_cipher = 262272;
pub const PJ_SSL_CK_IDEA_128_CBC_WITH_MD5: pj_ssl_cipher = 327808;
pub const PJ_SSL_CK_DES_64_CBC_WITH_MD5: pj_ssl_cipher = 393280;
pub const PJ_SSL_CK_DES_192_EDE3_CBC_WITH_MD5: pj_ssl_cipher = 458944;
pub type pj_ssl_cipher = c_int;

pub const PJ_TLS_UNKNOWN_CURVE: pj_ssl_curve = 0;
pub const PJ_TLS_CURVE_SECT163K1: pj_ssl_curve = 1;
pub const PJ_TLS_CURVE_SECT163R1: pj_ssl_curve = 2;
pub const PJ_TLS_CURVE_SECT163R2: pj_ssl_curve = 3;
pub const PJ_TLS_CURVE_SECT193R1: pj_ssl_curve = 4;
pub const PJ_TLS_CURVE_SECT193R2: pj_ssl_curve = 5;
pub const PJ_TLS_CURVE_SECT233K1: pj_ssl_curve = 6;
pub const PJ_TLS_CURVE_SECT233R1: pj_ssl_curve = 7;
pub const PJ_TLS_CURVE_SECT239K1: pj_ssl_curve = 8;
pub const PJ_TLS_CURVE_SECT283K1: pj_ssl_curve = 9;
pub const PJ_TLS_CURVE_SECT283R1: pj_ssl_curve = 10;
pub const PJ_TLS_CURVE_SECT409K1: pj_ssl_curve = 11;
pub const PJ_TLS_CURVE_SECT409R1: pj_ssl_curve = 12;
pub const PJ_TLS_CURVE_SECT571K1: pj_ssl_curve = 13;
pub const PJ_TLS_CURVE_SECT571R1: pj_ssl_curve = 14;
pub const PJ_TLS_CURVE_SECP160K1: pj_ssl_curve = 15;
pub const PJ_TLS_CURVE_SECP160R1: pj_ssl_curve = 16;
pub const PJ_TLS_CURVE_SECP160R2: pj_ssl_curve = 17;
pub const PJ_TLS_CURVE_SECP192K1: pj_ssl_curve = 18;
pub const PJ_TLS_CURVE_SECP192R1: pj_ssl_curve = 19;
pub const PJ_TLS_CURVE_SECP224K1: pj_ssl_curve = 20;
pub const PJ_TLS_CURVE_SECP224R1: pj_ssl_curve = 21;
pub const PJ_TLS_CURVE_SECP256K1: pj_ssl_curve = 22;
pub const PJ_TLS_CURVE_SECP256R1: pj_ssl_curve = 23;
pub const PJ_TLS_CURVE_SECP384R1: pj_ssl_curve = 24;
pub const PJ_TLS_CURVE_SECP521R1: pj_ssl_curve = 25;
pub const PJ_TLS_CURVE_BRAINPOOLP256R1: pj_ssl_curve = 26;
pub const PJ_TLS_CURVE_BRAINPOOLP384R1: pj_ssl_curve = 27;
pub const PJ_TLS_CURVE_BRAINPOOLP512R1: pj_ssl_curve = 28;
pub const PJ_TLS_CURVE_ARBITRARY_EXPLICIT_PRIME_CURVES: pj_ssl_curve = 65281;
pub const PJ_TLS_CURVE_ARBITRARY_EXPLICIT_CHAR2_CURVES: pj_ssl_curve = 65282;
pub type pj_ssl_curve = c_uint;

pub const PJ_SSL_ENTROPY_NONE: pj_ssl_entropy = 0;
pub const PJ_SSL_ENTROPY_EGD: pj_ssl_entropy = 1;
pub const PJ_SSL_ENTROPY_RANDOM: pj_ssl_entropy = 2;
pub const PJ_SSL_ENTROPY_URANDOM: pj_ssl_entropy = 3;
pub const PJ_SSL_ENTROPY_FILE: pj_ssl_entropy = 4;
pub const PJ_SSL_ENTROPY_UNKNOWN: pj_ssl_entropy = 15;
pub type pj_ssl_entropy = c_uint;
pub use self::pj_ssl_entropy as pj_ssl_entropy_t;

pub const PJ_SSL_SOCK_PROTO_DEFAULT: pj_ssl_sock_proto = 0;
pub const PJ_SSL_SOCK_PROTO_SSL2: pj_ssl_sock_proto = 1;
pub const PJ_SSL_SOCK_PROTO_SSL3: pj_ssl_sock_proto = 2;
pub const PJ_SSL_SOCK_PROTO_TLS1: pj_ssl_sock_proto = 4;
pub const PJ_SSL_SOCK_PROTO_TLS1_1: pj_ssl_sock_proto = 8;
pub const PJ_SSL_SOCK_PROTO_TLS1_2: pj_ssl_sock_proto = 16;
pub const PJ_SSL_SOCK_PROTO_TLS1_3: pj_ssl_sock_proto = 32;
pub const PJ_SSL_SOCK_PROTO_SSL23: pj_ssl_sock_proto = 65535;
pub const PJ_SSL_SOCK_PROTO_ALL: pj_ssl_sock_proto = 65535;
pub const PJ_SSL_SOCK_PROTO_DTLS1: pj_ssl_sock_proto = 65536;
pub type pj_ssl_sock_proto = c_uint;

pub const PJSIP_SSL_UNSPECIFIED_METHOD: pjsip_ssl_method = 0;
pub const PJSIP_SSLV2_METHOD: pjsip_ssl_method = 20;
pub const PJSIP_SSLV3_METHOD: pjsip_ssl_method = 30;
pub const PJSIP_TLSV1_METHOD: pjsip_ssl_method = 31;
pub const PJSIP_TLSV1_1_METHOD: pjsip_ssl_method = 32;
pub const PJSIP_TLSV1_2_METHOD: pjsip_ssl_method = 33;
pub const PJSIP_TLSV1_3_METHOD: pjsip_ssl_method = 34;
pub const PJSIP_SSLV23_METHOD: pjsip_ssl_method = 23;
pub type pjsip_ssl_method = c_uint;

pub const pj_sys_info_flag_PJ_SYS_HAS_IOS_BG: pj_sys_info_flag = 1;
pub type pj_sys_info_flag = c_uint;

pub const pj_thread_create_flags_PJ_THREAD_SUSPENDED: pj_thread_create_flags = 1;
pub type pj_thread_create_flags = c_uint;
pub type pj_thread_proc = Option<
unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
>;
pub type pj_thread_desc = [c_long; 64usize];


pub const PJ_MUTEX_DEFAULT: pj_mutex_type_e = 0;
pub const PJ_MUTEX_SIMPLE: pj_mutex_type_e = 1;
pub const PJ_MUTEX_RECURSE: pj_mutex_type_e = 2;
pub type pj_mutex_type_e = c_uint;


pub type pj_exit_callback = Option<unsafe extern "C" fn()>;
pub type pj_error_callback = Option<
unsafe extern "C" fn(
e: pj_status_t,
msg: *mut c_char,
max: pj_size_t,
) -> pj_str_t,
>;

pub const PJ_STUN_BINDING_METHOD: pj_stun_method_e = 1;
pub const PJ_STUN_SHARED_SECRET_METHOD: pj_stun_method_e = 2;
pub const PJ_STUN_ALLOCATE_METHOD: pj_stun_method_e = 3;
pub const PJ_STUN_REFRESH_METHOD: pj_stun_method_e = 4;
pub const PJ_STUN_SEND_METHOD: pj_stun_method_e = 6;
pub const PJ_STUN_DATA_METHOD: pj_stun_method_e = 7;
pub const PJ_STUN_CREATE_PERM_METHOD: pj_stun_method_e = 8;
pub const PJ_STUN_CHANNEL_BIND_METHOD: pj_stun_method_e = 9;
pub const PJ_STUN_CONNECT_METHOD: pj_stun_method_e = 10;
pub const PJ_STUN_CONNECTION_BIND_METHOD: pj_stun_method_e = 11;
pub const PJ_STUN_CONNECTION_ATTEMPT_METHOD: pj_stun_method_e = 12;
pub const PJ_STUN_METHOD_MAX: pj_stun_method_e = 13;
pub type pj_stun_method_e = c_uint;

pub const PJ_STUN_REQUEST_CLASS: pj_stun_msg_class_e = 0;
pub const PJ_STUN_INDICATION_CLASS: pj_stun_msg_class_e = 1;
pub const PJ_STUN_SUCCESS_CLASS: pj_stun_msg_class_e = 2;
pub const PJ_STUN_ERROR_CLASS: pj_stun_msg_class_e = 3;
pub type pj_stun_msg_class_e = c_uint;

pub const PJ_STUN_BINDING_REQUEST: pj_stun_msg_type = 1;
pub const PJ_STUN_BINDING_RESPONSE: pj_stun_msg_type = 257;
pub const PJ_STUN_BINDING_ERROR_RESPONSE: pj_stun_msg_type = 273;
pub const PJ_STUN_BINDING_INDICATION: pj_stun_msg_type = 17;
pub const PJ_STUN_SHARED_SECRET_REQUEST: pj_stun_msg_type = 2;
pub const PJ_STUN_SHARED_SECRET_RESPONSE: pj_stun_msg_type = 258;
pub const PJ_STUN_SHARED_SECRET_ERROR_RESPONSE: pj_stun_msg_type = 274;
pub const PJ_STUN_ALLOCATE_REQUEST: pj_stun_msg_type = 3;
pub const PJ_STUN_ALLOCATE_RESPONSE: pj_stun_msg_type = 259;
pub const PJ_STUN_ALLOCATE_ERROR_RESPONSE: pj_stun_msg_type = 275;
pub const PJ_STUN_REFRESH_REQUEST: pj_stun_msg_type = 4;
pub const PJ_STUN_REFRESH_RESPONSE: pj_stun_msg_type = 260;
pub const PJ_STUN_REFRESH_ERROR_RESPONSE: pj_stun_msg_type = 276;
pub const PJ_STUN_SEND_INDICATION: pj_stun_msg_type = 22;
pub const PJ_STUN_DATA_INDICATION: pj_stun_msg_type = 23;
pub const PJ_STUN_CREATE_PERM_REQUEST: pj_stun_msg_type = 8;
pub const PJ_STUN_CREATE_PERM_RESPONSE: pj_stun_msg_type = 264;
pub const PJ_STUN_CREATE_PERM_ERROR_RESPONSE: pj_stun_msg_type = 280;
pub const PJ_STUN_CHANNEL_BIND_REQUEST: pj_stun_msg_type = 9;
pub const PJ_STUN_CHANNEL_BIND_RESPONSE: pj_stun_msg_type = 265;
pub const PJ_STUN_CHANNEL_BIND_ERROR_RESPONSE: pj_stun_msg_type = 281;
pub const PJ_STUN_CONNECTION_BIND_REQUEST: pj_stun_msg_type = 11;
pub const PJ_STUN_CONNECTION_ATTEMPT_INDICATION: pj_stun_msg_type = 28;
pub type pj_stun_msg_type = c_uint;

pub const PJ_STUN_ATTR_MAPPED_ADDR: pj_stun_attr_type = 1;
pub const PJ_STUN_ATTR_RESPONSE_ADDR: pj_stun_attr_type = 2;
pub const PJ_STUN_ATTR_CHANGE_REQUEST: pj_stun_attr_type = 3;
pub const PJ_STUN_ATTR_SOURCE_ADDR: pj_stun_attr_type = 4;
pub const PJ_STUN_ATTR_CHANGED_ADDR: pj_stun_attr_type = 5;
pub const PJ_STUN_ATTR_USERNAME: pj_stun_attr_type = 6;
pub const PJ_STUN_ATTR_PASSWORD: pj_stun_attr_type = 7;
pub const PJ_STUN_ATTR_MESSAGE_INTEGRITY: pj_stun_attr_type = 8;
pub const PJ_STUN_ATTR_ERROR_CODE: pj_stun_attr_type = 9;
pub const PJ_STUN_ATTR_UNKNOWN_ATTRIBUTES: pj_stun_attr_type = 10;
pub const PJ_STUN_ATTR_REFLECTED_FROM: pj_stun_attr_type = 11;
pub const PJ_STUN_ATTR_CHANNEL_NUMBER: pj_stun_attr_type = 12;
pub const PJ_STUN_ATTR_LIFETIME: pj_stun_attr_type = 13;
pub const PJ_STUN_ATTR_MAGIC_COOKIE: pj_stun_attr_type = 15;
pub const PJ_STUN_ATTR_BANDWIDTH: pj_stun_attr_type = 16;
pub const PJ_STUN_ATTR_XOR_PEER_ADDR: pj_stun_attr_type = 18;
pub const PJ_STUN_ATTR_DATA: pj_stun_attr_type = 19;
pub const PJ_STUN_ATTR_REALM: pj_stun_attr_type = 20;
pub const PJ_STUN_ATTR_NONCE: pj_stun_attr_type = 21;
pub const PJ_STUN_ATTR_XOR_RELAYED_ADDR: pj_stun_attr_type = 22;
pub const PJ_STUN_ATTR_REQ_ADDR_TYPE: pj_stun_attr_type = 23;
pub const PJ_STUN_ATTR_REQ_ADDR_FAMILY: pj_stun_attr_type = 23;
pub const PJ_STUN_ATTR_EVEN_PORT: pj_stun_attr_type = 24;
pub const PJ_STUN_ATTR_REQ_TRANSPORT: pj_stun_attr_type = 25;
pub const PJ_STUN_ATTR_DONT_FRAGMENT: pj_stun_attr_type = 26;
pub const PJ_STUN_ATTR_XOR_MAPPED_ADDR: pj_stun_attr_type = 32;
pub const PJ_STUN_ATTR_TIMER_VAL: pj_stun_attr_type = 33;
pub const PJ_STUN_ATTR_RESERVATION_TOKEN: pj_stun_attr_type = 34;
pub const PJ_STUN_ATTR_XOR_REFLECTED_FROM: pj_stun_attr_type = 35;
pub const PJ_STUN_ATTR_PRIORITY: pj_stun_attr_type = 36;
pub const PJ_STUN_ATTR_USE_CANDIDATE: pj_stun_attr_type = 37;
pub const PJ_STUN_ATTR_CONNECTION_ID: pj_stun_attr_type = 42;
pub const PJ_STUN_ATTR_ICMP: pj_stun_attr_type = 48;
pub const PJ_STUN_ATTR_END_MANDATORY_ATTR: pj_stun_attr_type = 49;
pub const PJ_STUN_ATTR_START_EXTENDED_ATTR: pj_stun_attr_type = 32801;
pub const PJ_STUN_ATTR_SOFTWARE: pj_stun_attr_type = 32802;
pub const PJ_STUN_ATTR_ALTERNATE_SERVER: pj_stun_attr_type = 32803;
pub const PJ_STUN_ATTR_REFRESH_INTERVAL: pj_stun_attr_type = 32804;
pub const PJ_STUN_ATTR_FINGERPRINT: pj_stun_attr_type = 32808;
pub const PJ_STUN_ATTR_ICE_CONTROLLED: pj_stun_attr_type = 32809;
pub const PJ_STUN_ATTR_ICE_CONTROLLING: pj_stun_attr_type = 32810;
pub const PJ_STUN_ATTR_END_EXTENDED_ATTR: pj_stun_attr_type = 32811;
pub type pj_stun_attr_type = c_uint;

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
pub type pj_stun_status = c_uint;

pub type pj_stun_fingerprint_attr = pj_stun_uint_attr;
pub type pj_stun_realm_attr = pj_stun_string_attr;
pub type pj_stun_nonce_attr = pj_stun_string_attr;
pub type pj_stun_mapped_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_xor_mapped_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_software_attr = pj_stun_string_attr;
pub type pj_stun_alt_server_attr = pj_stun_sockaddr_attr;
pub type pj_stun_refresh_interval_attr = pj_stun_uint_attr;
pub type pj_stun_response_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_changed_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_change_request_attr = pj_stun_uint_attr;
pub type pj_stun_src_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_reflected_from_attr = pj_stun_sockaddr_attr;
pub type pj_stun_username_attr = pj_stun_string_attr;
pub type pj_stun_password_attr = pj_stun_string_attr;
pub type pj_stun_channel_number_attr = pj_stun_uint_attr;
pub type pj_stun_lifetime_attr = pj_stun_uint_attr;
pub type pj_stun_bandwidth_attr = pj_stun_uint_attr;
pub type pj_stun_xor_peer_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_data_attr = pj_stun_binary_attr;
pub type pj_stun_xor_relayed_addr_attr = pj_stun_sockaddr_attr;
pub type pj_stun_req_addr_type_attr = pj_stun_uint_attr;
pub type pj_stun_even_port_attr = pj_stun_uint_attr;
pub type pj_stun_req_transport_attr = pj_stun_uint_attr;
pub type pj_stun_dont_fragment_attr = pj_stun_empty_attr;
pub type pj_stun_res_token_attr = pj_stun_uint64_attr;
pub type pj_stun_xor_reflected_from_attr = pj_stun_sockaddr_attr;
pub type pj_stun_priority_attr = pj_stun_uint_attr;
pub type pj_stun_use_candidate_attr = pj_stun_empty_attr;
pub type pj_stun_timer_val_attr = pj_stun_uint_attr;
pub type pj_stun_ice_controlling_attr = pj_stun_uint64_attr;
pub type pj_stun_ice_controlled_attr = pj_stun_uint64_attr;
pub type pj_stun_icmp_attr = pj_stun_uint_attr;

pub const PJ_STUN_IS_DATAGRAM: pj_stun_decode_options = 1;
pub const PJ_STUN_CHECK_PACKET: pj_stun_decode_options = 2;
pub const PJ_STUN_NO_AUTHENTICATE: pj_stun_decode_options = 4;
pub const PJ_STUN_NO_FINGERPRINT_CHECK: pj_stun_decode_options = 8;
pub type pj_stun_decode_options = c_uint;

pub const PJ_STUN_AUTH_NONE: pj_stun_auth_type = 0;
pub const PJ_STUN_AUTH_SHORT_TERM: pj_stun_auth_type = 1;
pub const PJ_STUN_AUTH_LONG_TERM: pj_stun_auth_type = 2;
pub type pj_stun_auth_type = c_uint;

pub const PJ_STUN_AUTH_CRED_STATIC: pj_stun_auth_cred_type = 0;
pub const PJ_STUN_AUTH_CRED_DYNAMIC: pj_stun_auth_cred_type = 1;
pub type pj_stun_auth_cred_type = c_uint;

pub const PJ_STUN_PASSWD_PLAIN: pj_stun_passwd_type = 0;
pub const PJ_STUN_PASSWD_HASHED: pj_stun_passwd_type = 1;
pub type pj_stun_passwd_type = c_uint;

pub const PJ_STUN_SESS_LOG_TX_REQ: pj_stun_sess_msg_log_flag = 1;
pub const PJ_STUN_SESS_LOG_TX_RES: pj_stun_sess_msg_log_flag = 2;
pub const PJ_STUN_SESS_LOG_TX_IND: pj_stun_sess_msg_log_flag = 4;
pub const PJ_STUN_SESS_LOG_RX_REQ: pj_stun_sess_msg_log_flag = 8;
pub const PJ_STUN_SESS_LOG_RX_RES: pj_stun_sess_msg_log_flag = 16;
pub const PJ_STUN_SESS_LOG_RX_IND: pj_stun_sess_msg_log_flag = 32;
pub type pj_stun_sess_msg_log_flag = c_uint;

pub const PJ_ICE_CAND_TYPE_HOST: pj_ice_cand_type = 0;
pub const PJ_ICE_CAND_TYPE_SRFLX: pj_ice_cand_type = 1;
pub const PJ_ICE_CAND_TYPE_PRFLX: pj_ice_cand_type = 2;
pub const PJ_ICE_CAND_TYPE_RELAYED: pj_ice_cand_type = 3;
pub const PJ_ICE_CAND_TYPE_MAX: pj_ice_cand_type = 4;
pub type pj_ice_cand_type = c_uint;

pub const PJ_ICE_SESS_CHECK_STATE_FROZEN: pj_ice_sess_check_state = 0;
pub const PJ_ICE_SESS_CHECK_STATE_WAITING: pj_ice_sess_check_state = 1;
pub const PJ_ICE_SESS_CHECK_STATE_IN_PROGRESS: pj_ice_sess_check_state = 2;
pub const PJ_ICE_SESS_CHECK_STATE_SUCCEEDED: pj_ice_sess_check_state = 3;
pub const PJ_ICE_SESS_CHECK_STATE_FAILED: pj_ice_sess_check_state = 4;
pub type pj_ice_sess_check_state = c_uint;

pub const PJ_ICE_SESS_CHECKLIST_ST_IDLE: pj_ice_sess_checklist_state = 0;
pub const PJ_ICE_SESS_CHECKLIST_ST_RUNNING: pj_ice_sess_checklist_state = 1;
pub const PJ_ICE_SESS_CHECKLIST_ST_COMPLETED: pj_ice_sess_checklist_state = 2;
pub type pj_ice_sess_checklist_state = c_uint;

pub const PJ_ICE_SESS_ROLE_UNKNOWN: pj_ice_sess_role = 0;
pub const PJ_ICE_SESS_ROLE_CONTROLLED: pj_ice_sess_role = 1;
pub const PJ_ICE_SESS_ROLE_CONTROLLING: pj_ice_sess_role = 2;
pub type pj_ice_sess_role = c_uint;

pub const PJ_ICE_SESS_TRICKLE_DISABLED: pj_ice_sess_trickle = 0;
pub const PJ_ICE_SESS_TRICKLE_HALF: pj_ice_sess_trickle = 1;
pub const PJ_ICE_SESS_TRICKLE_FULL: pj_ice_sess_trickle = 2;
pub type pj_ice_sess_trickle = c_uint;

pub const PJ_STUN_SOCK_DNS_OP: pj_stun_sock_op = 1;
pub const PJ_STUN_SOCK_BINDING_OP: pj_stun_sock_op = 2;
pub const PJ_STUN_SOCK_KEEP_ALIVE_OP: pj_stun_sock_op = 3;
pub const PJ_STUN_SOCK_MAPPED_ADDR_CHANGE: pj_stun_sock_op = 4;
pub type pj_stun_sock_op = c_uint;

pub const PJ_TURN_TP_UDP: pj_turn_tp_type = 17;
pub const PJ_TURN_TP_TCP: pj_turn_tp_type = 6;
pub const PJ_TURN_TP_TLS: pj_turn_tp_type = 56;
pub type pj_turn_tp_type = c_uint;

pub const PJ_TURN_STATE_NULL: pj_turn_state_t = 0;
pub const PJ_TURN_STATE_RESOLVING: pj_turn_state_t = 1;
pub const PJ_TURN_STATE_RESOLVED: pj_turn_state_t = 2;
pub const PJ_TURN_STATE_ALLOCATING: pj_turn_state_t = 3;
pub const PJ_TURN_STATE_READY: pj_turn_state_t = 4;
pub const PJ_TURN_STATE_DEALLOCATING: pj_turn_state_t = 5;
pub const PJ_TURN_STATE_DEALLOCATED: pj_turn_state_t = 6;
pub const PJ_TURN_STATE_DESTROYING: pj_turn_state_t = 7;
pub type pj_turn_state_t = c_uint;

pub const PJ_ICE_STRANS_OP_INIT: pj_ice_strans_op = 0;
pub const PJ_ICE_STRANS_OP_NEGOTIATION: pj_ice_strans_op = 1;
pub const PJ_ICE_STRANS_OP_KEEP_ALIVE: pj_ice_strans_op = 2;
pub const PJ_ICE_STRANS_OP_ADDR_CHANGE: pj_ice_strans_op = 3;
pub type pj_ice_strans_op = c_uint;

pub const PJ_ICE_STRANS_STATE_NULL: pj_ice_strans_state = 0;
pub const PJ_ICE_STRANS_STATE_INIT: pj_ice_strans_state = 1;
pub const PJ_ICE_STRANS_STATE_READY: pj_ice_strans_state = 2;
pub const PJ_ICE_STRANS_STATE_SESS_READY: pj_ice_strans_state = 3;
pub const PJ_ICE_STRANS_STATE_NEGO: pj_ice_strans_state = 4;
pub const PJ_ICE_STRANS_STATE_RUNNING: pj_ice_strans_state = 5;
pub const PJ_ICE_STRANS_STATE_FAILED: pj_ice_strans_state = 6;
pub type pj_ice_strans_state = c_uint;

pub type pj_main_func_ptr = Option<
unsafe extern "C" fn(
argc: c_int,
argv: *mut *mut c_char,
) -> c_int,
>;

pub const PJ_STUN_NAT_TYPE_UNKNOWN: pj_stun_nat_type = 0;
pub const PJ_STUN_NAT_TYPE_ERR_UNKNOWN: pj_stun_nat_type = 1;
pub const PJ_STUN_NAT_TYPE_OPEN: pj_stun_nat_type = 2;
pub const PJ_STUN_NAT_TYPE_BLOCKED: pj_stun_nat_type = 3;
pub const PJ_STUN_NAT_TYPE_SYMMETRIC_UDP: pj_stun_nat_type = 4;
pub const PJ_STUN_NAT_TYPE_FULL_CONE: pj_stun_nat_type = 5;
pub const PJ_STUN_NAT_TYPE_SYMMETRIC: pj_stun_nat_type = 6;
pub const PJ_STUN_NAT_TYPE_RESTRICTED: pj_stun_nat_type = 7;
pub const PJ_STUN_NAT_TYPE_PORT_RESTRICTED: pj_stun_nat_type = 8;
pub type pj_stun_nat_type = c_uint;

pub const PJ_DNS_SRV_FALLBACK_A: pj_dns_srv_option = 1;
pub const PJ_DNS_SRV_FALLBACK_AAAA: pj_dns_srv_option = 2;
pub const PJ_DNS_SRV_RESOLVE_AAAA: pj_dns_srv_option = 4;
pub const PJ_DNS_SRV_RESOLVE_AAAA_ONLY: pj_dns_srv_option = 8;
pub type pj_dns_srv_option = c_uint;

pub const PJ_JSON_VAL_NULL: pj_json_val_type = 0;
pub const PJ_JSON_VAL_BOOL: pj_json_val_type = 1;
pub const PJ_JSON_VAL_NUMBER: pj_json_val_type = 2;
pub const PJ_JSON_VAL_STRING: pj_json_val_type = 3;
pub const PJ_JSON_VAL_ARRAY: pj_json_val_type = 4;
pub const PJ_JSON_VAL_OBJ: pj_json_val_type = 5;
pub type pj_json_val_type = c_uint;

pub const PJSTUN_BINDING_REQUEST: pjstun_msg_type = 1;
pub const PJSTUN_BINDING_RESPONSE: pjstun_msg_type = 257;
pub const PJSTUN_BINDING_ERROR_RESPONSE: pjstun_msg_type = 273;
pub const PJSTUN_SHARED_SECRET_REQUEST: pjstun_msg_type = 2;
pub const PJSTUN_SHARED_SECRET_RESPONSE: pjstun_msg_type = 258;
pub const PJSTUN_SHARED_SECRET_ERROR_RESPONSE: pjstun_msg_type = 274;
pub type pjstun_msg_type = c_uint;


pub const PJSTUN_ATTR_MAPPED_ADDR: pjstun_attr_type = 1;
pub const PJSTUN_ATTR_RESPONSE_ADDR: pjstun_attr_type = 2;
pub const PJSTUN_ATTR_CHANGE_REQUEST: pjstun_attr_type = 3;
pub const PJSTUN_ATTR_SOURCE_ADDR: pjstun_attr_type = 4;
pub const PJSTUN_ATTR_CHANGED_ADDR: pjstun_attr_type = 5;
pub const PJSTUN_ATTR_USERNAME: pjstun_attr_type = 6;
pub const PJSTUN_ATTR_PASSWORD: pjstun_attr_type = 7;
pub const PJSTUN_ATTR_MESSAGE_INTEGRITY: pjstun_attr_type = 8;
pub const PJSTUN_ATTR_ERROR_CODE: pjstun_attr_type = 9;
pub const PJSTUN_ATTR_UNKNOWN_ATTRIBUTES: pjstun_attr_type = 10;
pub const PJSTUN_ATTR_REFLECTED_FROM: pjstun_attr_type = 11;
pub const PJSTUN_ATTR_XOR_MAPPED_ADDR: pjstun_attr_type = 32;
pub type pjstun_attr_type = c_uint;

pub const pj_pcap_link_type_PJ_PCAP_LINK_TYPE_ETH: pj_pcap_link_type = 1;
pub type pj_pcap_link_type = c_uint;

pub const pj_pcap_proto_type_PJ_PCAP_PROTO_TYPE_UDP: pj_pcap_proto_type = 17;
pub type pj_pcap_proto_type = c_uint;

pub type pjstun_response_addr_attr = pjstun_mapped_addr_attr;
pub type pjstun_changed_addr_attr = pjstun_mapped_addr_attr;
pub type pjstun_src_addr_attr = pjstun_mapped_addr_attr;
pub type pjstun_reflected_form_attr = pjstun_mapped_addr_attr;

pub const PJ_CLI_CONSOLE_FRONT_END: pj_cli_front_end_type = 0;
pub const PJ_CLI_TELNET_FRONT_END: pj_cli_front_end_type = 1;
pub const PJ_CLI_HTTP_FRONT_END: pj_cli_front_end_type = 2;
pub const PJ_CLI_GUI_FRONT_END: pj_cli_front_end_type = 3;
pub type pj_cli_front_end_type = c_uint;

pub type pj_cli_get_dyn_choice = Option<unsafe extern "C" fn(param: *mut pj_cli_dyn_choice_param)>;
pub type pj_cli_cmd_handler = Option<unsafe extern "C" fn(cval: *mut pj_cli_cmd_val) -> pj_status_t>;

pub type pj_cli_telnet_on_started = Option<unsafe extern "C" fn(status: pj_status_t)>;

pub type __jmp_buf = [c_long; 8usize];

pub type jmp_buf = [__jmp_buf_tag; 1usize];
pub type pj_jmp_buf = jmp_buf;

pub const PJ_LOG_HAS_DAY_NAME: pj_log_decoration = 1;
pub const PJ_LOG_HAS_YEAR: pj_log_decoration = 2;
pub const PJ_LOG_HAS_MONTH: pj_log_decoration = 4;
pub const PJ_LOG_HAS_DAY_OF_MON: pj_log_decoration = 8;
pub const PJ_LOG_HAS_TIME: pj_log_decoration = 16;
pub const PJ_LOG_HAS_MICRO_SEC: pj_log_decoration = 32;
pub const PJ_LOG_HAS_SENDER: pj_log_decoration = 64;
pub const PJ_LOG_HAS_NEWLINE: pj_log_decoration = 128;
pub const PJ_LOG_HAS_CR: pj_log_decoration = 256;
pub const PJ_LOG_HAS_SPACE: pj_log_decoration = 512;
pub const PJ_LOG_HAS_COLOR: pj_log_decoration = 1024;
pub const PJ_LOG_HAS_LEVEL_TEXT: pj_log_decoration = 2048;
pub const PJ_LOG_HAS_THREAD_ID: pj_log_decoration = 4096;
pub const PJ_LOG_HAS_THREAD_SWC: pj_log_decoration = 8192;
pub const PJ_LOG_HAS_INDENT: pj_log_decoration = 16384;
pub type pj_log_decoration = c_uint;

pub type pj_log_func = Option<
unsafe extern "C" fn(
level: c_int,
data: *const c_char,
len: c_int,
),
>;

pub const PJ_O_RDONLY: pj_file_access = 4353;
pub const PJ_O_WRONLY: pj_file_access = 4354;
pub const PJ_O_RDWR: pj_file_access = 4355;
pub const PJ_O_APPEND: pj_file_access = 4360;
pub type pj_file_access = c_uint;

pub const PJ_SEEK_SET: pj_file_seek_type = 4609;
pub const PJ_SEEK_CUR: pj_file_seek_type = 4610;
pub const PJ_SEEK_END: pj_file_seek_type = 4611;
pub type pj_file_seek_type = c_uint;

pub type pj_hash_entry_buf = [*mut c_void; 4usize];

pub type pjstun_password_attr = pjstun_username_attr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
pub gp_offset: c_uint,
pub fp_offset: c_uint,
pub overflow_arg_area: *mut c_void,
pub reg_save_area: *mut c_void,
}

pub type __builtin_va_list = [__va_list_tag; 1usize];

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
pub storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
#[inline]
pub const fn new(storage: Storage) -> Self {
Self { storage }
}
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
Storage: AsRef<[u8]> + AsMut<[u8]>,
{
#[inline]
pub fn get_bit(&self, index: usize) -> bool {
debug_assert!(index / 8 < self.storage.as_ref().len());
let byte_index = index / 8;
let byte = self.storage.as_ref()[byte_index];
let bit_index = if cfg!(target_endian = "big") {
7 - (index % 8)
} else {
index % 8
};
let mask = 1 << bit_index;
byte & mask == mask
}
#[inline]
pub fn set_bit(&mut self, index: usize, val: bool) {
debug_assert!(index / 8 < self.storage.as_ref().len());
let byte_index = index / 8;
let byte = &mut self.storage.as_mut()[byte_index];
let bit_index = if cfg!(target_endian = "big") {
7 - (index % 8)
} else {
index % 8
};
let mask = 1 << bit_index;
if val {
*byte |= mask;
} else {
*byte &= !mask;
}
}
#[inline]
pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
debug_assert!(bit_width <= 64);
debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
let mut val = 0;
for i in 0..(bit_width as usize) {
if self.get_bit(i + bit_offset) {
 let index = if cfg!(target_endian = "big") {
     bit_width as usize - 1 - i
 } else {
     i
 };
 val |= 1 << index;
}
}
val
}
#[inline]
pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
debug_assert!(bit_width <= 64);
debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
for i in 0..(bit_width as usize) {
let mask = 1 << i;
let val_bit_is_set = val & mask == mask;
let index = if cfg!(target_endian = "big") {
 bit_width as usize - 1 - i
} else {
 i
};
self.set_bit(index + bit_offset, val_bit_is_set);
}
}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
pub __val: [c_ulong; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_str_t {
pub ptr: *mut c_char,
pub slen: pj_ssize_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_timestamp {
pub u32_: pj_timestamp__bindgen_ty_1,
pub u64_: pj_uint64_t,
_bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_timestamp__bindgen_ty_1 {
pub lo: pj_uint32_t,
pub hi: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_hash_table_t {
_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_hash_entry {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_hash_iterator_t {
pub index: pj_uint32_t,
pub entry: *mut pj_hash_entry,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ioqueue_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ioqueue_key_t {
_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_timer_heap_t {
_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_atomic_t {
_unused: [u8; 0],
}
pub type pj_atomic_value_t = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_thread_t {
pub _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_lock_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_grp_lock_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_mutex_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_sem_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_event_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pipe_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_time_val {
pub sec: c_long,
pub msec: c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_parsed_time {
pub wday: c_int,
pub day: c_int,
pub mon: c_int,
pub year: c_int,
pub sec: c_int,
pub min: c_int,
pub hour: c_int,
pub msec: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_list {
pub prev: *mut c_void,
pub next: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_cis_buf_t {
pub cis_buf: [pj_cis_elem_t; 256usize],
pub use_mask: pj_cis_elem_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cis_t {
pub cis_buf: *mut pj_cis_elem_t,
pub cis_id: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_scanner {
pub begin: *mut c_char,
pub end: *mut c_char,
pub curptr: *mut c_char,
pub line: c_int,
pub start_line: *mut c_char,
pub skip_ws: c_int,
pub callback: pj_syn_err_func_ptr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_scan_state {
pub curptr: *mut c_char,
pub line: c_int,
pub start_line: *mut c_char,
}


pub type pj_timer_id_t = c_int;
pub type pj_timer_heap_callback = Option<
unsafe extern "C" fn(timer_heap: *mut pj_timer_heap_t, entry: *mut pj_timer_entry),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_grp_lock_config {
pub flags: c_uint,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_timer_entry {
pub user_data: *mut c_void,
pub id: c_int,
pub cb: pj_timer_heap_callback,
pub _timer_id: pj_timer_id_t,
}


pub type in_addr_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
pub s_addr: in_addr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
pub __in6_u: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
pub __u6_addr8: [u8; 16usize],
pub __u6_addr16: [u16; 8usize],
pub __u6_addr32: [u32; 4usize],
_bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_sockaddr_in {
pub sin_family: pj_uint16_t,
pub sin_port: pj_uint16_t,
pub sin_addr: pj_in_addr,
pub sin_zero_pad: [c_char; 8usize],
}

pub type pj_in6_addr = in6_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_sockaddr_in6 {
pub sin6_family: pj_uint16_t,
pub sin6_port: pj_uint16_t,
pub sin6_flowinfo: pj_uint32_t,
pub sin6_addr: pj_in6_addr,
pub sin6_scope_id: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_addr_hdr {
pub sa_family: pj_uint16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_sockaddr {
pub addr: pj_addr_hdr,
pub ipv4: pj_sockaddr_in,
pub ipv6: pj_sockaddr_in6,
_bindgen_union_align: [u32; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ip_mreq {
pub imr_multiaddr: pj_in_addr,
pub imr_interface: pj_in_addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_sockopt_params {
pub cnt: c_uint,
pub options: [pj_sockopt_params__bindgen_ty_1; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_sockopt_params__bindgen_ty_1 {
pub level: c_int,
pub optname: c_int,
pub optval: *mut c_void,
pub optlen: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_hdr {
pub id: pj_uint16_t,
pub flags: pj_uint16_t,
pub qdcount: pj_uint16_t,
pub anscount: pj_uint16_t,
pub nscount: pj_uint16_t,
pub arcount: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_query {
pub name: pj_str_t,
pub type_: pj_uint16_t,
pub dnsclass: pj_uint16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_parsed_rr {
pub name: pj_str_t,
pub type_: pj_uint16_t,
pub dnsclass: pj_uint16_t,
pub ttl: pj_uint32_t,
pub rdlength: pj_uint16_t,
pub data: *mut c_void,
pub rdata: pj_dns_parsed_rr_rdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_dns_parsed_rr_rdata {
pub srv: pj_dns_parsed_rr_rdata_srv,
pub cname: pj_dns_parsed_rr_rdata_cname,
pub ns: pj_dns_parsed_rr_rdata_ns,
pub ptr: pj_dns_parsed_rr_rdata_ptr,
pub a: pj_dns_parsed_rr_rdata_a,
pub aaaa: pj_dns_parsed_rr_rdata_aaaa,
_bindgen_union_align: [u64; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_rr_rdata_srv {
pub prio: pj_uint16_t,
pub weight: pj_uint16_t,
pub port: pj_uint16_t,
pub target: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_rr_rdata_cname {
pub name: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_rr_rdata_ns {
pub name: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_rr_rdata_ptr {
pub name: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_rr_rdata_a {
pub ip_addr: pj_in_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_parsed_rr_rdata_aaaa {
pub ip_addr: pj_in6_addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_parsed_packet {
pub hdr: pj_dns_hdr,
pub q: *mut pj_dns_parsed_query,
pub ans: *mut pj_dns_parsed_rr,
pub ns: *mut pj_dns_parsed_rr,
pub arr: *mut pj_dns_parsed_rr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_resolver {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_async_query {
_unused: [u8; 0],
}
pub type pj_dns_callback = Option<
unsafe extern "C" fn(
user_data: *mut c_void,
status: pj_status_t,
response: *mut pj_dns_parsed_packet,
),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_settings {
pub options: c_uint,
pub qretr_delay: c_uint,
pub qretr_count: c_uint,
pub cache_max_ttl: c_uint,
pub good_ns_ttl: c_uint,
pub bad_ns_ttl: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_a_record {
pub name: pj_str_t,
pub alias: pj_str_t,
pub addr_count: c_uint,
pub addr: [pj_in_addr; 8usize],
pub buf_: [c_char; 128usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_addr_record {
pub name: pj_str_t,
pub alias: pj_str_t,
pub addr_count: c_uint,
pub addr: [pj_dns_addr_record__bindgen_ty_1; 8usize],
pub buf_: [c_char; 128usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_addr_record__bindgen_ty_1 {
pub af: c_int,
pub ip: pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1 {
pub v4: pj_in_addr,
pub v6: pj_in6_addr,
_bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ioqueue_op_key_t {
pub internal__: [*mut c_void; 32usize],
pub activesock_data: *mut c_void,
pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ioqueue_callback {
pub on_read_complete: Option<
unsafe extern "C" fn(
key: *mut pj_ioqueue_key_t,
op_key: *mut pj_ioqueue_op_key_t,
bytes_read: pj_ssize_t,
),
>,
pub on_write_complete: Option<
unsafe extern "C" fn(
key: *mut pj_ioqueue_key_t,
op_key: *mut pj_ioqueue_op_key_t,
bytes_sent: pj_ssize_t,
),
>,
pub on_accept_complete: Option<
unsafe extern "C" fn(
key: *mut pj_ioqueue_key_t,
op_key: *mut pj_ioqueue_op_key_t,
sock: pj_sock_t,
status: pj_status_t,
),
>,
pub on_connect_complete: Option<
unsafe extern "C" fn(key: *mut pj_ioqueue_key_t, status: pj_status_t),
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_qos_params {
pub flags: pj_uint8_t,
pub dscp_val: pj_uint8_t,
pub so_prio: pj_uint8_t,
pub wmm_prio: pj_qos_wmm_prio,
}

pub type pj_pool_callback =
Option<unsafe extern "C" fn(pool: *mut pj_pool_t, size: pj_size_t)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_block {
pub prev: *mut pj_pool_block,
pub next: *mut pj_pool_block,
pub buf: *mut c_uchar,
pub cur: *mut c_uchar,
pub end: *mut c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_t {
pub prev: *mut pj_pool_t,
pub next: *mut pj_pool_t,
pub obj_name: [c_char; 32usize],
pub factory: *mut pj_pool_factory,
pub factory_data: *mut c_void,
pub capacity: pj_size_t,
pub increment_size: pj_size_t,
pub block_list: pj_pool_block,
pub callback: pj_pool_callback,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_factory_policy {
pub block_alloc: Option<
unsafe extern "C" fn(
factory: *mut pj_pool_factory,
size: pj_size_t,
) -> *mut c_void,
>,
pub block_free: Option<
unsafe extern "C" fn(
factory: *mut pj_pool_factory,
mem: *mut c_void,
size: pj_size_t,
),
>,
pub callback: pj_pool_callback,
pub flags: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_factory {
pub policy: pj_pool_factory_policy,
pub create_pool: Option<
unsafe extern "C" fn(
factory: *mut pj_pool_factory,
name: *const c_char,
initial_size: pj_size_t,
increment_size: pj_size_t,
callback: pj_pool_callback,
) -> *mut pj_pool_t,
>,
pub release_pool: Option<
unsafe extern "C" fn(factory: *mut pj_pool_factory, pool: *mut pj_pool_t),
>,
pub dump_status: Option<
unsafe extern "C" fn(factory: *mut pj_pool_factory, detail: pj_bool_t),
>,
pub on_block_alloc: Option<
unsafe extern "C" fn(factory: *mut pj_pool_factory, size: pj_size_t) -> pj_bool_t,
>,
pub on_block_free:
Option<unsafe extern "C" fn(factory: *mut pj_pool_factory, size: pj_size_t)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_caching_pool {
pub factory: pj_pool_factory,
pub capacity: pj_size_t,
pub max_capacity: pj_size_t,
pub used_count: pj_size_t,
pub used_size: pj_size_t,
pub peak_used_size: pj_size_t,
pub free_list: [pj_list; 16usize],
pub used_list: pj_list,
pub pool_buf: [c_char; 512usize],
pub lock: *mut pj_lock_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_sock_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info {
pub version: c_uint,
pub serial_no: [pj_uint8_t; 20usize],
pub subject: pj_ssl_cert_info__bindgen_ty_1,
pub issuer: pj_ssl_cert_info__bindgen_ty_2,
pub validity: pj_ssl_cert_info__bindgen_ty_3,
pub subj_alt_name: pj_ssl_cert_info__bindgen_ty_4,
pub raw: pj_str_t,
pub raw_chain: pj_ssl_cert_info__bindgen_ty_5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info__bindgen_ty_1 {
pub cn: pj_str_t,
pub info: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info__bindgen_ty_2 {
pub cn: pj_str_t,
pub info: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info__bindgen_ty_3 {
pub start: pj_time_val,
pub end: pj_time_val,
pub gmt: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info__bindgen_ty_4 {
pub cnt: c_uint,
pub entry: *mut pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1 {
pub type_: pj_ssl_cert_name_type,
pub name: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_cert_info__bindgen_ty_5 {
pub cnt: c_uint,
pub cert_raw: *mut pj_str_t,
}

pub type pj_ssl_cert_buffer = pj_str_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_sock_cb {
pub on_data_read: Option<
unsafe extern "C" fn(
ssock: *mut pj_ssl_sock_t,
data: *mut c_void,
size: pj_size_t,
status: pj_status_t,
remainder: *mut pj_size_t,
) -> pj_bool_t,
>,
pub on_data_recvfrom: Option<
unsafe extern "C" fn(
ssock: *mut pj_ssl_sock_t,
data: *mut c_void,
size: pj_size_t,
src_addr: *const pj_sockaddr_t,
addr_len: c_int,
status: pj_status_t,
) -> pj_bool_t,
>,
pub on_data_sent: Option<
unsafe extern "C" fn(
ssock: *mut pj_ssl_sock_t,
send_key: *mut pj_ioqueue_op_key_t,
sent: pj_ssize_t,
) -> pj_bool_t,
>,
pub on_accept_complete: Option<
unsafe extern "C" fn(
ssock: *mut pj_ssl_sock_t,
newsock: *mut pj_ssl_sock_t,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_int,
) -> pj_bool_t,
>,
pub on_accept_complete2: Option<
unsafe extern "C" fn(
ssock: *mut pj_ssl_sock_t,
newsock: *mut pj_ssl_sock_t,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_int,
status: pj_status_t,
) -> pj_bool_t,
>,
pub on_connect_complete: Option<
unsafe extern "C" fn(ssock: *mut pj_ssl_sock_t, status: pj_status_t) -> pj_bool_t,
>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ssl_sock_info {
pub established: pj_bool_t,
pub proto: pj_uint32_t,
pub cipher: pj_ssl_cipher,
pub local_addr: pj_sockaddr,
pub remote_addr: pj_sockaddr,
pub local_cert_info: *mut pj_ssl_cert_info,
pub remote_cert_info: *mut pj_ssl_cert_info,
pub verify_status: pj_uint32_t,
pub last_native_err: c_ulong,
pub grp_lock: *mut pj_grp_lock_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_sock_param {
pub grp_lock: *mut pj_grp_lock_t,
pub sock_af: c_int,
pub sock_type: c_int,
pub ioqueue: *mut pj_ioqueue_t,
pub timer_heap: *mut pj_timer_heap_t,
pub cb: pj_ssl_sock_cb,
pub user_data: *mut c_void,
pub proto: pj_uint32_t,
pub async_cnt: c_uint,
pub concurrency: c_int,
pub whole_data: pj_bool_t,
pub send_buffer_size: pj_size_t,
pub read_buffer_size: pj_size_t,
pub ciphers_num: c_uint,
pub ciphers: *mut pj_ssl_cipher,
pub curves_num: c_uint,
pub curves: *mut pj_ssl_curve,
pub sigalgs: pj_str_t,
pub entropy_type: pj_ssl_entropy_t,
pub entropy_path: pj_str_t,
pub timeout: pj_time_val,
pub verify_peer: pj_bool_t,
pub require_client_cert: pj_bool_t,
pub server_name: pj_str_t,
pub reuse_addr: pj_bool_t,
pub qos_type: pj_qos_type,
pub qos_params: pj_qos_params,
pub qos_ignore_error: pj_bool_t,
pub sockopt_params: pj_sockopt_params,
pub sockopt_ignore_error: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_start_connect_param {
pub pool: *mut pj_pool_t,
pub localaddr: *const pj_sockaddr_t,
pub local_port_range: pj_uint16_t,
pub remaddr: *const pj_sockaddr_t,
pub addr_len: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tls_on_accept_fail_param {
pub local_addr: *const pj_sockaddr_t,
pub remote_addr: *const pj_sockaddr_t,
pub status: pj_status_t,
pub last_native_err: pj_status_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tls_setting {
pub ca_list_file: pj_str_t,
pub ca_list_path: pj_str_t,
pub cert_file: pj_str_t,
pub privkey_file: pj_str_t,
pub ca_buf: pj_ssl_cert_buffer,
pub cert_buf: pj_ssl_cert_buffer,
pub privkey_buf: pj_ssl_cert_buffer,
pub password: pj_str_t,
pub method: pjsip_ssl_method,
pub proto: pj_uint32_t,
pub ciphers_num: c_uint,
pub ciphers: *mut pj_ssl_cipher,
pub curves_num: c_uint,
pub curves: *mut pj_ssl_curve,
pub sigalgs: pj_str_t,
pub entropy_type: pj_ssl_entropy_t,
pub entropy_path: pj_str_t,
pub verify_server: pj_bool_t,
pub verify_client: pj_bool_t,
pub require_client_cert: pj_bool_t,
pub timeout: pj_time_val,
pub reuse_addr: pj_bool_t,
pub qos_type: pj_qos_type,
pub qos_params: pj_qos_params,
pub qos_ignore_error: pj_bool_t,
pub sockopt_params: pj_sockopt_params,
pub sockopt_ignore_error: pj_bool_t,
pub on_accept_fail_cb:
Option<unsafe extern "C" fn(param: *const pjsip_tls_on_accept_fail_param)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_sys_info {
pub machine: pj_str_t,
pub os_name: pj_str_t,
pub os_ver: pj_uint32_t,
pub sdk_name: pj_str_t,
pub sdk_ver: pj_uint32_t,
pub info: pj_str_t,
pub flags: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_symbianos_params {
pub rsocketserv: *mut c_void,
pub rconnection: *mut c_void,
pub rhostresolver: *mut c_void,
pub rhostresolver6: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_rwmutex_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_msg_hdr {
pub type_: pj_uint16_t,
pub length: pj_uint16_t,
pub magic: pj_uint32_t,
pub tsx_id: [pj_uint8_t; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_attr_hdr {
pub type_: pj_uint16_t,
pub length: pj_uint16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_stun_sockaddr_attr {
pub hdr: pj_stun_attr_hdr,
pub xor_ed: pj_bool_t,
pub sockaddr: pj_sockaddr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_empty_attr {
pub hdr: pj_stun_attr_hdr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_string_attr {
pub hdr: pj_stun_attr_hdr,
pub value: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_uint_attr {
pub hdr: pj_stun_attr_hdr,
pub value: pj_uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_stun_uint64_attr {
pub hdr: pj_stun_attr_hdr,
pub value: pj_timestamp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_binary_attr {
pub hdr: pj_stun_attr_hdr,
pub magic: pj_uint32_t,
pub length: c_uint,
pub data: *mut pj_uint8_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_msgint_attr {
pub hdr: pj_stun_attr_hdr,
pub hmac: [pj_uint8_t; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_errcode_attr {
pub hdr: pj_stun_attr_hdr,
pub err_code: c_int,
pub reason: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]

pub struct pj_stun_unknown_attr {
pub hdr: pj_stun_attr_hdr,
pub attr_count: c_uint,
pub attrs: [pj_uint16_t; 16usize],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_msg {
pub hdr: pj_stun_msg_hdr,
pub attr_count: c_uint,
pub attr: [*mut pj_stun_attr_hdr; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_stun_auth_cred {
pub type_: pj_stun_auth_cred_type,
pub data: pj_stun_auth_cred__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_stun_auth_cred__bindgen_ty_1 {
pub static_cred: pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1,
pub dyn_cred: pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2,
_bindgen_union_align: [u64; 9usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 {
pub realm: pj_str_t,
pub username: pj_str_t,
pub data_type: pj_stun_passwd_type,
pub data: pj_str_t,
pub nonce: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 {
pub user_data: *mut c_void,
pub get_auth: Option<
unsafe extern "C" fn(
user_data: *mut c_void,
pool: *mut pj_pool_t,
realm: *mut pj_str_t,
nonce: *mut pj_str_t,
) -> pj_status_t,
>,
pub get_cred: Option<
unsafe extern "C" fn(
msg: *const pj_stun_msg,
user_data: *mut c_void,
pool: *mut pj_pool_t,
realm: *mut pj_str_t,
username: *mut pj_str_t,
nonce: *mut pj_str_t,
data_type: *mut pj_stun_passwd_type,
data: *mut pj_str_t,
) -> pj_status_t,
>,
pub get_password: Option<
unsafe extern "C" fn(
msg: *const pj_stun_msg,
user_data: *mut c_void,
realm: *const pj_str_t,
username: *const pj_str_t,
pool: *mut pj_pool_t,
data_type: *mut pj_stun_passwd_type,
data: *mut pj_str_t,
) -> pj_status_t,
>,
pub verify_nonce: Option<
unsafe extern "C" fn(
msg: *const pj_stun_msg,
user_data: *mut c_void,
realm: *const pj_str_t,
username: *const pj_str_t,
nonce: *const pj_str_t,
) -> pj_bool_t,
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_req_cred_info {
pub realm: pj_str_t,
pub username: pj_str_t,
pub nonce: pj_str_t,
pub auth_key: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_config {
pub pf: *mut pj_pool_factory,
pub ioqueue: *mut pj_ioqueue_t,
pub timer_heap: *mut pj_timer_heap_t,
pub options: c_uint,
pub rto_msec: c_uint,
pub res_cache_msec: c_uint,
pub software_name: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_client_tsx {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_tsx_cb {
pub on_complete: Option<
unsafe extern "C" fn(
tsx: *mut pj_stun_client_tsx,
status: pj_status_t,
response: *const pj_stun_msg,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_uint,
),
>,
pub on_send_msg: Option<
unsafe extern "C" fn(
tsx: *mut pj_stun_client_tsx,
stun_pkt: *const c_void,
pkt_size: pj_size_t,
) -> pj_status_t,
>,
pub on_destroy: Option<unsafe extern "C" fn(tsx: *mut pj_stun_client_tsx)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_session {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_session_cb {
pub on_send_msg: Option<
unsafe extern "C" fn(
sess: *mut pj_stun_session,
token: *mut c_void,
pkt: *const c_void,
pkt_size: pj_size_t,
dst_addr: *const pj_sockaddr_t,
addr_len: c_uint,
) -> pj_status_t,
>,
pub on_rx_request: Option<
unsafe extern "C" fn(
sess: *mut pj_stun_session,
pkt: *const pj_uint8_t,
pkt_len: c_uint,
rdata: *const pj_stun_rx_data,
token: *mut c_void,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_uint,
) -> pj_status_t,
>,
pub on_request_complete: Option<
unsafe extern "C" fn(
sess: *mut pj_stun_session,
status: pj_status_t,
token: *mut c_void,
tdata: *mut pj_stun_tx_data,
response: *const pj_stun_msg,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_uint,
),
>,
pub on_rx_indication: Option<
unsafe extern "C" fn(
sess: *mut pj_stun_session,
pkt: *const pj_uint8_t,
pkt_len: c_uint,
msg: *const pj_stun_msg,
token: *mut c_void,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_uint,
) -> pj_status_t,
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_rx_data {
pub msg: *mut pj_stun_msg,
pub info: pj_stun_req_cred_info,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_tx_data {
pub prev: *mut pj_stun_tx_data,
pub next: *mut pj_stun_tx_data,
pub pool: *mut pj_pool_t,
pub sess: *mut pj_stun_session,
pub msg: *mut pj_stun_msg,
pub token: *mut c_void,
pub client_tsx: *mut pj_stun_client_tsx,
pub retransmit: pj_bool_t,
pub msg_magic: pj_uint32_t,
pub msg_key: [pj_uint8_t; 12usize],
pub grp_lock: *mut pj_grp_lock_t,
pub auth_info: pj_stun_req_cred_info,
pub pkt: *mut c_void,
pub max_len: c_uint,
pub pkt_size: pj_size_t,
pub addr_len: c_uint,
pub dst_addr: *const pj_sockaddr_t,
pub res_timer: pj_timer_entry,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_sess_comp {
pub valid_check: *mut pj_ice_sess_check,
pub nominated_check: *mut pj_ice_sess_check,
pub stun_sess: *mut pj_stun_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_msg_data {
pub transport_id: c_uint,
pub has_req_data: pj_bool_t,
pub data: pj_ice_msg_data_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_ice_msg_data_data {
pub req: pj_ice_msg_data_data_request_data,
_bindgen_union_align: [u64; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_msg_data_data_request_data {
pub ice: *mut pj_ice_sess,
pub clist: *mut pj_ice_sess_checklist,
pub ckid: c_uint,
pub lcand: *mut pj_ice_sess_cand,
pub rcand: *mut pj_ice_sess_cand,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_sess_cand {
pub id: c_uint,
pub type_: pj_ice_cand_type,
pub status: pj_status_t,
pub comp_id: pj_uint8_t,
pub transport_id: pj_uint8_t,
pub local_pref: pj_uint16_t,
pub foundation: pj_str_t,
pub prio: pj_uint32_t,
pub addr: pj_sockaddr,
pub base_addr: pj_sockaddr,
pub rel_addr: pj_sockaddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_sess_check {
pub lcand: *mut pj_ice_sess_cand,
pub rcand: *mut pj_ice_sess_cand,
pub foundation_idx: c_int,
pub prio: pj_timestamp,
pub state: pj_ice_sess_check_state,
pub tdata: *mut pj_stun_tx_data,
pub nominated: pj_bool_t,
pub err_code: pj_status_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_sess_checklist {
pub state: pj_ice_sess_checklist_state,
pub count: c_uint,
pub checks: [pj_ice_sess_check; 32usize],
pub foundation_cnt: c_uint,
pub foundation: [pj_str_t; 32usize],
pub timer: pj_timer_entry,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_sess_cb {
pub on_valid_pair: Option<unsafe extern "C" fn(ice: *mut pj_ice_sess)>,
pub on_ice_complete:
Option<unsafe extern "C" fn(ice: *mut pj_ice_sess, status: pj_status_t)>,
pub on_tx_pkt: Option<
unsafe extern "C" fn(
ice: *mut pj_ice_sess,
comp_id: c_uint,
transport_id: c_uint,
pkt: *const c_void,
size: pj_size_t,
dst_addr: *const pj_sockaddr_t,
dst_addr_len: c_uint,
) -> pj_status_t,
>,
pub on_rx_data: Option<
unsafe extern "C" fn(
ice: *mut pj_ice_sess,
comp_id: c_uint,
transport_id: c_uint,
pkt: *mut c_void,
size: pj_size_t,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_uint,
),
>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_rx_check {
pub prev: *mut pj_ice_rx_check,
pub next: *mut pj_ice_rx_check,
pub comp_id: c_uint,
pub transport_id: c_uint,
pub src_addr: pj_sockaddr,
pub src_addr_len: c_uint,
pub use_candidate: pj_bool_t,
pub priority: pj_uint32_t,
pub role_attr: *mut pj_stun_uint64_attr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_sess_options {
pub aggressive: pj_bool_t,
pub nominated_check_delay: c_uint,
pub controlled_agent_want_nom_timeout: c_int,
pub trickle: pj_ice_sess_trickle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_sess {
pub obj_name: [c_char; 32usize],
pub pool: *mut pj_pool_t,
pub user_data: *mut c_void,
pub grp_lock: *mut pj_grp_lock_t,
pub role: pj_ice_sess_role,
pub opt: pj_ice_sess_options,
pub tie_breaker: pj_timestamp,
pub prefs: *mut pj_uint8_t,
pub is_nominating: pj_bool_t,
pub is_complete: pj_bool_t,
pub is_destroying: pj_bool_t,
pub valid_pair_found: pj_bool_t,
pub is_trickling: pj_bool_t,
pub ice_status: pj_status_t,
pub timer: pj_timer_entry,
pub cb: pj_ice_sess_cb,
pub stun_cfg: pj_stun_config,
pub tx_ufrag: pj_str_t,
pub tx_uname: pj_str_t,
pub tx_pass: pj_str_t,
pub rx_ufrag: pj_str_t,
pub rx_uname: pj_str_t,
pub rx_pass: pj_str_t,
pub comp_cnt: c_uint,
pub comp: [pj_ice_sess_comp; 2usize],
pub comp_ka: c_uint,
pub lcand_cnt: c_uint,
pub lcand: [pj_ice_sess_cand; 16usize],
pub lcand_paired: c_uint,
pub rcand_cnt: c_uint,
pub rcand: [pj_ice_sess_cand; 16usize],
pub rcand_paired: c_uint,
pub tp_data: [pj_ice_msg_data; 5usize],
pub early_check: pj_ice_rx_check,
pub clist: pj_ice_sess_checklist,
pub valid_list: pj_ice_sess_checklist,
pub tmp: pj_ice_sess__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_ice_sess__bindgen_ty_1 {
pub txt: [c_char; 128usize],
pub errmsg: [c_char; 80usize],
_bindgen_union_align: [u8; 128usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_sock {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_sock_cb {
pub on_rx_data: Option<
unsafe extern "C" fn(
stun_sock: *mut pj_stun_sock,
pkt: *mut c_void,
pkt_len: c_uint,
src_addr: *const pj_sockaddr_t,
addr_len: c_uint,
) -> pj_bool_t,
>,
pub on_data_sent: Option<
unsafe extern "C" fn(
stun_sock: *mut pj_stun_sock,
send_key: *mut pj_ioqueue_op_key_t,
sent: pj_ssize_t,
) -> pj_bool_t,
>,
pub on_status: Option<
unsafe extern "C" fn(
stun_sock: *mut pj_stun_sock,
op: pj_stun_sock_op,
status: pj_status_t,
) -> pj_bool_t,
>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_stun_sock_info {
pub bound_addr: pj_sockaddr,
pub srv_addr: pj_sockaddr,
pub mapped_addr: pj_sockaddr,
pub alias_cnt: c_uint,
pub aliases: [pj_sockaddr; 8usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_stun_sock_cfg {
pub grp_lock: *mut pj_grp_lock_t,
pub max_pkt_size: c_uint,
pub async_cnt: c_uint,
pub bound_addr: pj_sockaddr,
pub port_range: pj_uint16_t,
pub ka_interval: c_int,
pub qos_type: pj_qos_type,
pub qos_params: pj_qos_params,
pub qos_ignore_error: pj_bool_t,
pub so_rcvbuf_size: c_uint,
pub so_sndbuf_size: c_uint,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_session {
_unused: [u8; 0],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_channel_data {
pub ch_number: pj_uint16_t,
pub length: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_session_cb {
pub on_send_pkt: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
pkt: *const pj_uint8_t,
pkt_len: c_uint,
dst_addr: *const pj_sockaddr_t,
addr_len: c_uint,
) -> pj_status_t,
>,
pub on_stun_send_pkt: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
pkt: *const pj_uint8_t,
pkt_len: c_uint,
dst_addr: *const pj_sockaddr_t,
addr_len: c_uint,
) -> pj_status_t,
>,
pub on_channel_bound: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
ch_num: c_uint,
),
>,
pub on_rx_data: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
pkt: *mut c_void,
pkt_len: c_uint,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
),
>,
pub on_state: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
old_state: pj_turn_state_t,
new_state: pj_turn_state_t,
),
>,
pub on_connection_attempt: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
conn_id: pj_uint32_t,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
),
>,
pub on_connection_bind_status: Option<
unsafe extern "C" fn(
sess: *mut pj_turn_session,
status: pj_status_t,
conn_id: pj_uint32_t,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
),
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_alloc_param {
pub bandwidth: c_int,
pub lifetime: c_int,
pub ka_interval: c_int,
pub af: c_int,
pub peer_conn_type: pj_turn_tp_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_turn_session_info {
pub state: pj_turn_state_t,
pub last_status: pj_status_t,
pub conn_type: pj_turn_tp_type,
pub server: pj_sockaddr,
pub mapped_addr: pj_sockaddr,
pub relay_addr: pj_sockaddr,
pub lifetime: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_session_on_rx_pkt_param {
pub pkt: *mut c_void,
pub pkt_len: pj_size_t,
pub parsed_len: pj_size_t,
pub src_addr: *const pj_sockaddr_t,
pub src_addr_len: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_sock {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_sock_cb {
pub on_rx_data: Option<
unsafe extern "C" fn(
turn_sock: *mut pj_turn_sock,
pkt: *mut c_void,
pkt_len: c_uint,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
),
>,
pub on_data_sent: Option<
unsafe extern "C" fn(sock: *mut pj_turn_sock, sent: pj_ssize_t) -> pj_bool_t,
>,
pub on_state: Option<
unsafe extern "C" fn(
turn_sock: *mut pj_turn_sock,
old_state: pj_turn_state_t,
new_state: pj_turn_state_t,
),
>,
pub on_connection_attempt: Option<
unsafe extern "C" fn(
turn_sock: *mut pj_turn_sock,
conn_id: pj_uint32_t,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
) -> pj_status_t,
>,
pub on_connection_status: Option<
unsafe extern "C" fn(
turn_sock: *mut pj_turn_sock,
status: pj_status_t,
conn_id: pj_uint32_t,
peer_addr: *const pj_sockaddr_t,
addr_len: c_uint,
),
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_turn_sock_tls_cfg {
pub ca_list_file: pj_str_t,
pub ca_list_path: pj_str_t,
pub cert_file: pj_str_t,
pub privkey_file: pj_str_t,
pub ca_buf: pj_ssl_cert_buffer,
pub cert_buf: pj_ssl_cert_buffer,
pub privkey_buf: pj_ssl_cert_buffer,
pub password: pj_str_t,
pub ssock_param: pj_ssl_sock_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_turn_sock_cfg {
pub grp_lock: *mut pj_grp_lock_t,
pub max_pkt_size: c_uint,
pub qos_type: pj_qos_type,
pub qos_params: pj_qos_params,
pub qos_ignore_error: pj_bool_t,
pub bound_addr: pj_sockaddr,
pub port_range: pj_uint16_t,
pub so_rcvbuf_size: c_uint,
pub so_sndbuf_size: c_uint,
pub tls_cfg: pj_turn_sock_tls_cfg,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_strans {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_strans_cb {
pub on_rx_data: Option<
unsafe extern "C" fn(
ice_st: *mut pj_ice_strans,
comp_id: c_uint,
pkt: *mut c_void,
size: pj_size_t,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_uint,
),
>,
pub on_data_sent:
Option<unsafe extern "C" fn(sock: *mut pj_ice_strans, sent: pj_ssize_t)>,
pub on_valid_pair: Option<unsafe extern "C" fn(ice_st: *mut pj_ice_strans)>,
pub on_ice_complete: Option<
unsafe extern "C" fn(ice_st: *mut pj_ice_strans, op: pj_ice_strans_op, status: pj_status_t),
>,
pub on_new_candidate: Option<
unsafe extern "C" fn(
ice_st: *mut pj_ice_strans,
cand: *const pj_ice_sess_cand,
end_of_cand: pj_bool_t,
),
>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_strans_stun_cfg {
pub af: c_int,
pub cfg: pj_stun_sock_cfg,
pub max_host_cands: c_uint,
pub loop_addr: pj_bool_t,
pub server: pj_str_t,
pub port: pj_uint16_t,
pub ignore_stun_error: pj_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_strans_turn_cfg {
pub af: c_int,
pub cfg: pj_turn_sock_cfg,
pub server: pj_str_t,
pub port: pj_uint16_t,
pub conn_type: pj_turn_tp_type,
pub auth_cred: pj_stun_auth_cred,
pub alloc_param: pj_turn_alloc_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_ice_strans_cfg {
pub af: c_int,
pub stun_cfg: pj_stun_config,
pub resolver: *mut pj_dns_resolver,
pub opt: pj_ice_sess_options,
pub stun: pj_ice_strans_stun_cfg,
pub stun_tp_cnt: c_uint,
pub stun_tp: [pj_ice_strans_stun_cfg; 2usize],
pub turn: pj_ice_strans_turn_cfg,
pub turn_tp_cnt: c_uint,
pub turn_tp: [pj_ice_strans_turn_cfg; 3usize],
pub num_send_buf: c_uint,
pub send_buf_size: c_uint,
pub comp: [pj_ice_strans_cfg__bindgen_ty_1; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ice_strans_cfg__bindgen_ty_1 {
pub qos_type: pj_qos_type,
pub qos_params: pj_qos_params,
pub so_rcvbuf_size: c_uint,
pub so_sndbuf_size: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_stun_nat_detect_result {
pub status: pj_status_t,
pub status_text: *const c_char,
pub nat_type: pj_stun_nat_type,
pub nat_type_name: *const c_char,
}
pub type pj_stun_nat_detect_cb = Option<
unsafe extern "C" fn(
user_data: *mut c_void,
res: *const pj_stun_nat_detect_result,
),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_getopt_option {
pub name: *const c_char,
pub has_arg: c_int,
pub flag: *mut c_int,
pub val: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_crc32_context {
pub crc_state: pj_uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_md5_context {
pub buf: [pj_uint32_t; 4usize],
pub bits: [pj_uint32_t; 2usize],
pub in_: [pj_uint8_t; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_hmac_md5_context {
pub context: pj_md5_context,
pub k_opad: [pj_uint8_t; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_sha1_context {
pub state: [pj_uint32_t; 5usize],
pub count: [pj_uint32_t; 2usize],
pub buffer: [pj_uint8_t; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_hmac_sha1_context {
pub context: pj_sha1_context,
pub k_opad: [pj_uint8_t; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_srv_record {
pub count: c_uint,
pub entry: [pj_dns_srv_record__bindgen_ty_1; 8usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_srv_record__bindgen_ty_1 {
pub priority: c_uint,
pub weight: c_uint,
pub port: pj_uint16_t,
pub server: pj_dns_addr_record,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_srv_async_query {
_unused: [u8; 0],
}

pub type pj_dns_srv_resolver_cb = Option<
unsafe extern "C" fn(
user_data: *mut c_void,
status: pj_status_t,
rec: *const pj_dns_srv_record,
),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_server {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_json_list {
pub prev: *mut pj_json_elem,
pub next: *mut pj_json_elem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_json_elem {
pub prev: *mut pj_json_elem,
pub next: *mut pj_json_elem,
pub name: pj_str_t,
pub type_: pj_json_val_type,
pub value: pj_json_elem__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_json_elem__bindgen_ty_1 {
pub is_true: pj_bool_t,
pub num: f32,
pub str_: pj_str_t,
pub children: pj_json_list,
_bindgen_union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_json_err_info {
pub line: c_uint,
pub col: c_uint,
pub err_char: c_int,
}

pub type pj_json_writer = Option<
unsafe extern "C" fn(
s: *const c_char,
size: c_uint,
user_data: *mut c_void,
) -> pj_status_t,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_msg_hdr {
pub type_: pj_uint16_t,
pub length: pj_uint16_t,
pub tsx: [pj_uint32_t; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_attr_hdr {
pub type_: pj_uint16_t,
pub length: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_mapped_addr_attr {
pub hdr: pjstun_attr_hdr,
pub ignored: pj_uint8_t,
pub family: pj_uint8_t,
pub port: pj_uint16_t,
pub addr: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_change_request_attr {
pub hdr: pjstun_attr_hdr,
pub value: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_username_attr {
pub hdr: pjstun_attr_hdr,
pub value: [pj_uint32_t; 1usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_error_code_attr {
pub hdr: pjstun_attr_hdr,
pub ignored: pj_uint16_t,
pub err_class: pj_uint8_t,
pub number: pj_uint8_t,
pub reason: [c_char; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_msg {
pub hdr: *mut pjstun_msg_hdr,
pub attr_count: c_int,
pub attr: [*mut pjstun_attr_hdr; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjstun_setting {
pub use_stun2: pj_bool_t,
pub af: c_int,
pub srv1: pj_str_t,
pub port1: c_int,
pub srv2: pj_str_t,
pub port2: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pcap_udp_hdr {
pub src_port: pj_uint16_t,
pub dst_port: pj_uint16_t,
pub len: pj_uint16_t,
pub csum: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pcap_filter {
pub link: pj_pcap_link_type,
pub proto: pj_pcap_proto_type,
pub ip_src: pj_uint32_t,
pub ip_dst: pj_uint32_t,
pub src_port: pj_uint16_t,
pub dst_port: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pcap_file {
_unused: [u8; 0],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_activesock_t {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_activesock_cb {
pub on_data_read: Option<
unsafe extern "C" fn(
asock: *mut pj_activesock_t,
data: *mut c_void,
size: pj_size_t,
status: pj_status_t,
remainder: *mut pj_size_t,
) -> pj_bool_t,
>,
pub on_data_recvfrom: Option<
unsafe extern "C" fn(
asock: *mut pj_activesock_t,
data: *mut c_void,
size: pj_size_t,
src_addr: *const pj_sockaddr_t,
addr_len: c_int,
status: pj_status_t,
) -> pj_bool_t,
>,
pub on_data_sent: Option<
unsafe extern "C" fn(
asock: *mut pj_activesock_t,
send_key: *mut pj_ioqueue_op_key_t,
sent: pj_ssize_t,
) -> pj_bool_t,
>,
pub on_accept_complete: Option<
unsafe extern "C" fn(
asock: *mut pj_activesock_t,
newsock: pj_sock_t,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_int,
) -> pj_bool_t,
>,
pub on_accept_complete2: Option<
unsafe extern "C" fn(
asock: *mut pj_activesock_t,
newsock: pj_sock_t,
src_addr: *const pj_sockaddr_t,
src_addr_len: c_int,
status: pj_status_t,
) -> pj_bool_t,
>,
pub on_connect_complete: Option<
unsafe extern "C" fn(asock: *mut pj_activesock_t, status: pj_status_t) -> pj_bool_t,
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_activesock_cfg {
pub grp_lock: *mut pj_grp_lock_t,
pub async_cnt: c_uint,
pub concurrency: c_int,
pub whole_data: pj_bool_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_req {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_header_elmt {
pub name: pj_str_t,
pub value: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_headers {
pub count: c_uint,
pub header: [pj_http_header_elmt; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_auth_cred {
pub scheme: pj_str_t,
pub realm: pj_str_t,
pub username: pj_str_t,
pub data_type: c_uint,
pub data: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_req_param {
pub addr_family: c_int,
pub method: pj_str_t,
pub version: pj_str_t,
pub timeout: pj_time_val,
pub user_data: *mut c_void,
pub headers: pj_http_headers,
pub reqdata: pj_http_req_param_pj_http_reqdata,
pub auth_cred: pj_http_auth_cred,
pub source_port_range_start: pj_uint16_t,
pub source_port_range_size: pj_uint16_t,
pub max_retries: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_req_param_pj_http_reqdata {
pub data: *mut c_void,
pub size: pj_size_t,
pub total_size: pj_size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_auth_chal {
pub scheme: pj_str_t,
pub realm: pj_str_t,
pub domain: pj_str_t,
pub nonce: pj_str_t,
pub opaque: pj_str_t,
pub stale: c_int,
pub algorithm: pj_str_t,
pub qop: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_resp {
pub version: pj_str_t,
pub status_code: pj_uint16_t,
pub reason: pj_str_t,
pub headers: pj_http_headers,
pub auth_chal: pj_http_auth_chal,
pub content_length: pj_int32_t,
pub data: *mut c_void,
pub size: pj_size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_url {
pub username: pj_str_t,
pub passwd: pj_str_t,
pub protocol: pj_str_t,
pub host: pj_str_t,
pub port: pj_uint16_t,
pub path: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_http_req_callback {
pub on_response: Option<
unsafe extern "C" fn(http_req: *mut pj_http_req, resp: *const pj_http_resp),
>,
pub on_send_data: Option<
unsafe extern "C" fn(
http_req: *mut pj_http_req,
data: *mut *mut c_void,
size: *mut pj_size_t,
),
>,
pub on_data_read: Option<
unsafe extern "C" fn(
http_req: *mut pj_http_req,
data: *mut c_void,
size: pj_size_t,
),
>,
pub on_complete: Option<
unsafe extern "C" fn(
http_req: *mut pj_http_req,
status: pj_status_t,
resp: *const pj_http_resp,
),
>,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_t {
_unused: [u8; 0],
}
pub type pj_cli_cmd_id = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_cfg {
pub name: pj_str_t,
pub title: pj_str_t,
pub pf: *mut pj_pool_factory,
}

pub type pj_cli_arg_id = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_cmd_spec {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_arg_spec {
_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_cmd_val {
pub sess: *mut pj_cli_sess,
pub cmd: *const pj_cli_cmd_spec,
pub argc: c_int,
pub argv: [pj_str_t; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_hint_info {
pub name: pj_str_t,
pub type_: pj_str_t,
pub desc: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_exec_info {
pub err_pos: c_int,
pub cmd_id: pj_cli_cmd_id,
pub cmd_ret: pj_status_t,
pub hint_cnt: c_uint,
pub hint: [pj_cli_hint_info; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_arg_choice_val {
pub value: pj_str_t,
pub desc: pj_str_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_cli_dyn_choice_param {
pub sess: *mut pj_cli_sess,
pub cmd: *mut pj_cli_cmd_spec,
pub arg_id: pj_cli_arg_id,
pub max_cnt: c_uint,
pub pool: *mut pj_pool_t,
pub cnt: c_uint,
pub choice: [pj_cli_arg_choice_val; 64usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_front_end_op {
pub on_write_log: Option<
unsafe extern "C" fn(
fe: *mut pj_cli_front_end,
level: c_int,
data: *const c_char,
len: pj_size_t,
),
>,
pub on_quit: Option<
unsafe extern "C" fn(fe: *mut pj_cli_front_end, req: *mut pj_cli_sess),
>,
pub on_destroy: Option<unsafe extern "C" fn(fe: *mut pj_cli_front_end)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_front_end {
pub prev: *mut pj_cli_front_end,
pub next: *mut pj_cli_front_end,
pub type_: pj_cli_front_end_type,
pub cli: *mut pj_cli_t,
pub op: *mut pj_cli_front_end_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_sess_op {
pub destroy: Option<unsafe extern "C" fn(sess: *mut pj_cli_sess)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_sess {
pub prev: *mut pj_cli_sess,
pub next: *mut pj_cli_sess,
pub fe: *mut pj_cli_front_end,
pub op: *mut pj_cli_sess_op,
pub info: pj_str_t,
pub log_level: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_console_cfg {
pub log_level: c_int,
pub prompt_str: pj_str_t,
pub quit_command: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_telnet_info {
pub ip_address: pj_str_t,
pub port: pj_uint16_t,
pub buf_: [c_char; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_cli_telnet_cfg {
pub port: pj_uint16_t,
pub ioqueue: *mut pj_ioqueue_t,
pub log_level: c_int,
pub passwd: pj_str_t,
pub welcome_msg: pj_str_t,
pub prompt_str: pj_str_t,
pub on_started: pj_cli_telnet_on_started,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_hostent {
pub h_name: *mut c_char,
pub h_aliases: *mut *mut c_char,
pub h_addrtype: c_int,
pub h_length: c_int,
pub h_addr_list: *mut *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_addrinfo {
pub ai_canonname: [c_char; 128usize],
pub ai_addr: pj_sockaddr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
pub __jmpbuf: __jmp_buf,
pub __mask_was_saved: c_int,
pub __saved_mask: __sigset_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_exception_state_t {
pub state: pj_jmp_buf,
pub prev: *mut pj_exception_state_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_fifobuf_t {
pub first: *mut c_char,
pub last: *mut c_char,
pub ubegin: *mut c_char,
pub uend: *mut c_char,
pub full: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_file_stat {
pub size: pj_off_t,
pub atime: pj_time_val,
pub mtime: pj_time_val,
pub ctime: pj_time_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_ip_route_entry {
pub ipv4: pj_ip_route_entry__bindgen_ty_1,
_bindgen_union_align: [u32; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ip_route_entry__bindgen_ty_1 {
pub if_addr: pj_in_addr,
pub dst_addr: pj_in_addr,
pub mask: pj_in_addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_enum_ip_option {
pub af: c_int,
pub omit_deprecated_ipv6: pj_bool_t,
}

pub const pj_rbcolor_t_PJ_RBCOLOR_BLACK: pj_rbcolor_t = 0;
pub const pj_rbcolor_t_PJ_RBCOLOR_RED: pj_rbcolor_t = 1;
pub type pj_rbcolor_t = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_rbtree_node {
pub parent: *mut pj_rbtree_node,
pub left: *mut pj_rbtree_node,
pub right: *mut pj_rbtree_node,
pub key: *const c_void,
pub user_data: *mut c_void,
pub color: pj_rbcolor_t,
}

pub type pj_rbtree_comp = Option<
unsafe extern "C" fn(
key1: *const c_void,
key2: *const c_void,
) -> c_int,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_rbtree {
pub null_node: pj_rbtree_node,
pub null: *mut pj_rbtree_node,
pub root: *mut pj_rbtree_node,
pub size: c_uint,
pub comp: pj_rbtree_comp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_fd_set_t {
pub data: [pj_sock_t; 68usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_xml_attr {
pub prev: *mut pj_xml_attr,
pub next: *mut pj_xml_attr,
pub name: pj_str_t,
pub value: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_xml_node_head {
pub prev: *mut pj_xml_node,
pub next: *mut pj_xml_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_xml_node {
pub prev: *mut pj_xml_node,
pub next: *mut pj_xml_node,
pub name: pj_str_t,
pub attr_head: pj_xml_attr,
pub node_head: pj_xml_node_head,
pub content: pj_str_t,
}

pub type pjpidf_pres = pj_xml_node;
pub type pjpidf_tuple = pj_xml_node;
pub type pjpidf_status = pj_xml_node;
pub type pjpidf_note = pj_xml_node;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjpidf_status_op {
pub construct:
Option<unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_status)>,
pub is_basic_open:
Option<unsafe extern "C" fn(arg1: *const pjpidf_status) -> pj_bool_t>,
pub set_basic_open:
Option<unsafe extern "C" fn(arg1: *mut pjpidf_status, arg2: pj_bool_t)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjpidf_tuple_op {
pub construct: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_tuple, arg3: *const pj_str_t),
>,
pub get_id:
Option<unsafe extern "C" fn(arg1: *const pjpidf_tuple) -> *const pj_str_t>,
pub set_id: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_tuple, arg3: *const pj_str_t),
>,
pub get_status:
Option<unsafe extern "C" fn(arg1: *mut pjpidf_tuple) -> *mut pjpidf_status>,
pub get_contact:
Option<unsafe extern "C" fn(arg1: *const pjpidf_tuple) -> *const pj_str_t>,
pub set_contact: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_tuple, arg3: *const pj_str_t),
>,
pub set_contact_prio: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_tuple, arg3: *const pj_str_t),
>,
pub get_contact_prio:
Option<unsafe extern "C" fn(arg1: *const pjpidf_tuple) -> *const pj_str_t>,
pub add_note: Option<
unsafe extern "C" fn(
arg1: *mut pj_pool_t,
arg2: *mut pjpidf_tuple,
arg3: *const pj_str_t,
) -> *mut pjpidf_note,
>,
pub get_first_note:
Option<unsafe extern "C" fn(arg1: *mut pjpidf_tuple) -> *mut pjpidf_note>,
pub get_next_note: Option<
unsafe extern "C" fn(arg1: *mut pjpidf_tuple, arg2: *mut pjpidf_note) -> *mut pjpidf_note,
>,
pub get_timestamp:
Option<unsafe extern "C" fn(arg1: *const pjpidf_tuple) -> *const pj_str_t>,
pub set_timestamp: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_tuple, arg3: *const pj_str_t),
>,
pub set_timestamp_np: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_tuple, arg3: *mut pj_str_t),
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjpidf_pres_op {
pub construct: Option<
unsafe extern "C" fn(arg1: *mut pj_pool_t, arg2: *mut pjpidf_pres, arg3: *const pj_str_t),
>,
pub add_tuple: Option<
unsafe extern "C" fn(
arg1: *mut pj_pool_t,
arg2: *mut pjpidf_pres,
arg3: *const pj_str_t,
) -> *mut pjpidf_tuple,
>,
pub get_first_tuple:
Option<unsafe extern "C" fn(arg1: *mut pjpidf_pres) -> *mut pjpidf_tuple>,
pub get_next_tuple: Option<
unsafe extern "C" fn(arg1: *mut pjpidf_pres, arg2: *mut pjpidf_tuple) -> *mut pjpidf_tuple,
>,
pub find_tuple: Option<
unsafe extern "C" fn(arg1: *mut pjpidf_pres, arg2: *const pj_str_t) -> *mut pjpidf_tuple,
>,
pub remove_tuple: Option<
unsafe extern "C" fn(arg1: *mut pjpidf_pres, arg2: *mut pjpidf_tuple),
>,
pub add_note: Option<
unsafe extern "C" fn(
arg1: *mut pj_pool_t,
arg2: *mut pjpidf_pres,
arg3: *const pj_str_t,
) -> *mut pjpidf_note,
>,
pub get_first_note:
Option<unsafe extern "C" fn(arg1: *mut pjpidf_pres) -> *mut pjpidf_note>,
pub get_next_note: Option<
unsafe extern "C" fn(arg1: *mut pjpidf_pres, arg2: *mut pjpidf_note) -> *mut pjpidf_note,
>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjpidf_op_desc {
pub pres: pjpidf_pres_op,
pub tuple: pjpidf_tuple_op,
pub status: pjpidf_status_op,
}

pub type pjxpidf_pres = pj_xml_node;

pub const pjrpid_activity_PJRPID_ACTIVITY_UNKNOWN: pjrpid_activity = 0;
pub const pjrpid_activity_PJRPID_ACTIVITY_AWAY: pjrpid_activity = 1;
pub const pjrpid_activity_PJRPID_ACTIVITY_BUSY: pjrpid_activity = 2;
pub type pjrpid_activity = c_uint;
pub const pjrpid_element_type_PJRPID_ELEMENT_TYPE_PERSON: pjrpid_element_type = 0;
pub type pjrpid_element_type = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjrpid_element {
pub type_: pjrpid_element_type,
pub id: pj_str_t,
pub activity: pjrpid_activity,
pub note: pj_str_t,
}

pub type pj_highprec_t = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_math_stat {
    pub n: c_int,
    pub max: c_int,
    pub min: c_int,
    pub last: c_int,
    pub mean: c_int,
    pub fmean_: f32,
    pub m2_: pj_highprec_t,
}

#[link(name = "pj")]
extern "C" {
pub static mut PJ_VERSION: *const c_char;
pub static PJ_AF_UNSPEC: pj_uint16_t;
pub static PJ_AF_UNIX: pj_uint16_t;
pub static PJ_AF_INET: pj_uint16_t;
pub static PJ_AF_INET6: pj_uint16_t;
pub static PJ_AF_PACKET: pj_uint16_t;
pub static PJ_AF_IRDA: pj_uint16_t;
pub static PJ_SOCK_STREAM: pj_uint16_t;
pub static PJ_SOCK_DGRAM: pj_uint16_t;
pub static PJ_SOCK_RAW: pj_uint16_t;
pub static PJ_SOCK_RDM: pj_uint16_t;
pub static PJ_SOL_SOCKET: pj_uint16_t;
pub static PJ_SOL_IP: pj_uint16_t;
pub static PJ_SOL_TCP: pj_uint16_t;
pub static PJ_SOL_UDP: pj_uint16_t;
pub static PJ_SOL_IPV6: pj_uint16_t;
pub static PJ_IP_TOS: pj_uint16_t;
pub static PJ_IPTOS_LOWDELAY: pj_uint16_t;
pub static PJ_IPTOS_THROUGHPUT: pj_uint16_t;
pub static PJ_IPTOS_RELIABILITY: pj_uint16_t;
pub static PJ_IPTOS_MINCOST: pj_uint16_t;
pub static PJ_IPV6_TCLASS: pj_uint16_t;
pub static PJ_SO_TYPE: pj_uint16_t;
pub static PJ_SO_RCVBUF: pj_uint16_t;
pub static PJ_SO_SNDBUF: pj_uint16_t;
pub static PJ_TCP_NODELAY: pj_uint16_t;
pub static PJ_SO_REUSEADDR: pj_uint16_t;
pub static PJ_SO_NOSIGPIPE: pj_uint16_t;
pub static PJ_SO_PRIORITY: pj_uint16_t;
pub static PJ_IP_MULTICAST_IF: pj_uint16_t;
pub static PJ_IP_MULTICAST_TTL: pj_uint16_t;
pub static PJ_IP_MULTICAST_LOOP: pj_uint16_t;
pub static PJ_IP_ADD_MEMBERSHIP: pj_uint16_t;
pub static PJ_IP_DROP_MEMBERSHIP: pj_uint16_t;
pub static PJ_MSG_OOB: c_int;
pub static PJ_MSG_PEEK: c_int;
pub static PJ_MSG_DONTROUTE: c_int;

pub fn pj_get_version() -> *const c_char;
pub fn pj_dump_config();
pub fn pj_init() -> pj_status_t;
pub fn pj_shutdown();
pub fn pj_atexit(func: pj_exit_callback) -> pj_status_t;
pub fn pj_get_os_error() -> pj_status_t;
pub fn pj_set_os_error(code: pj_status_t);
pub fn pj_get_netos_error() -> pj_status_t;
pub fn pj_set_netos_error(code: pj_status_t);
pub fn pj_strerror(statcode: pj_status_t,buf: *mut c_char,bufsize: pj_size_t,) -> pj_str_t;
pub fn pj_perror( log_level: c_int, sender: *const c_char, status: pj_status_t, title_fmt: *const c_char, ... );
pub fn pj_register_strerror(start_code: pj_status_t,err_space: pj_status_t,f: pj_error_callback, ) -> pj_status_t;
pub fn pj_errno_clear_handlers();
pub fn pj_perror_1( sender: *const c_char, status: pj_status_t, title_fmt: *const c_char, ... );
pub fn pj_perror_2( sender: *const c_char, status: pj_status_t, title_fmt: *const c_char, ... );
pub fn pj_perror_3( sender: *const c_char, status: pj_status_t, title_fmt: *const c_char, ... );
pub fn pj_perror_4( sender: *const c_char, status: pj_status_t, title_fmt: *const c_char, ... );
pub fn pj_perror_5( sender: *const c_char, status: pj_status_t, title_fmt: *const c_char, ...);
pub fn pjsip_strerror( status: pj_status_t, buffer: *mut c_char, bufsize: pj_size_t, ) -> pj_str_t;
pub fn pj_time_val_normalize(t: *mut pj_time_val);
pub fn pj_list_insert_before(pos: *mut pj_list_type, node: *mut pj_list_type);
pub fn pj_list_insert_nodes_before(lst: *mut pj_list_type, nodes: *mut pj_list_type);
pub fn pj_list_insert_after(pos: *mut pj_list_type, node: *mut pj_list_type);
pub fn pj_list_insert_nodes_after(lst: *mut pj_list_type, nodes: *mut pj_list_type);
pub fn pj_list_merge_first(list1: *mut pj_list_type, list2: *mut pj_list_type);
pub fn pj_list_merge_last(list1: *mut pj_list_type, list2: *mut pj_list_type);
pub fn pj_list_erase(node: *mut pj_list_type);
pub fn pj_list_find_node(list: *mut pj_list_type, node: *mut pj_list_type) -> *mut pj_list_type;
pub fn pj_list_search( list: *mut pj_list_type, value: *mut c_void, comp: Option< unsafe extern "C" fn( value: *mut c_void, node: *const pj_list_type, ) -> c_int, >, ) -> *mut pj_list_type;
pub fn pj_list_size(list: *const pj_list_type) -> pj_size_t;
pub fn pjlib_util_init() -> pj_status_t;
pub fn pj_cis_buf_init(cs_buf: *mut pj_cis_buf_t);
pub fn pj_cis_init(cs_buf: *mut pj_cis_buf_t, cis: *mut pj_cis_t) -> pj_status_t;
pub fn pj_cis_dup(new_cis: *mut pj_cis_t, existing: *mut pj_cis_t) -> pj_status_t;
pub fn pj_cis_add_range( cis: *mut pj_cis_t, cstart: c_int, cend: c_int, );
pub fn pj_cis_add_alpha(cis: *mut pj_cis_t);
pub fn pj_cis_add_num(cis: *mut pj_cis_t);
pub fn pj_cis_add_str(cis: *mut pj_cis_t, str_: *const c_char);
pub fn pj_cis_add_cis(cis: *mut pj_cis_t, rhs: *const pj_cis_t);
pub fn pj_cis_del_range( cis: *mut pj_cis_t, cstart: c_int, cend: c_int, );
pub fn pj_cis_del_str(cis: *mut pj_cis_t, str_: *const c_char);
pub fn pj_cis_invert(cis: *mut pj_cis_t);
pub fn pj_scan_init( scanner: *mut pj_scanner, bufstart: *mut c_char, buflen: pj_size_t, options: c_uint, callback: pj_syn_err_func_ptr, );
pub fn pj_scan_fini(scanner: *mut pj_scanner);
pub fn pj_scan_peek( scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t, ) -> c_int;
pub fn pj_scan_peek_n( scanner: *mut pj_scanner, len: pj_size_t, out: *mut pj_str_t,) -> c_int;
pub fn pj_scan_peek_until( scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t, ) -> c_int;
pub fn pj_scan_get(scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t);
pub fn pj_scan_get_unescape( scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t, );
pub fn pj_scan_get_quote( scanner: *mut pj_scanner, begin_quote: c_int, end_quote: c_int, out: *mut pj_str_t,);
pub fn pj_scan_get_quotes( scanner: *mut pj_scanner, begin_quotes: *const c_char, end_quotes: *const c_char, qsize: c_int, out: *mut pj_str_t, );
pub fn pj_scan_get_n(scanner: *mut pj_scanner, N: c_uint, out: *mut pj_str_t);
pub fn pj_scan_get_char(scanner: *mut pj_scanner) -> c_int;
pub fn pj_scan_get_until(scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t);
pub fn pj_scan_get_until_ch( scanner: *mut pj_scanner, until_char: c_int, out: *mut pj_str_t, );
pub fn pj_scan_get_until_chr( scanner: *mut pj_scanner, until_spec: *const c_char, out: *mut pj_str_t, );
pub fn pj_scan_advance_n(scanner: *mut pj_scanner, N: c_uint, skip: pj_bool_t);
pub fn pj_scan_strcmp( scanner: *mut pj_scanner, s: *const c_char, len: c_int, ) -> c_int;
pub fn pj_scan_stricmp( scanner: *mut pj_scanner, s: *const c_char, len: c_int,) -> c_int;
pub fn pj_scan_stricmp_alnum( scanner: *mut pj_scanner, s: *const c_char, len: c_int,) -> c_int;
pub fn pj_scan_get_newline(scanner: *mut pj_scanner);
pub fn pj_scan_skip_whitespace(scanner: *mut pj_scanner);
pub fn pj_scan_skip_line(scanner: *mut pj_scanner);
pub fn pj_scan_save_state(scanner: *const pj_scanner, state: *mut pj_scan_state);
pub fn pj_scan_restore_state(scanner: *mut pj_scanner, state: *mut pj_scan_state);
pub fn pj_lock_create_simple_mutex( pool: *mut pj_pool_t, name: *const c_char, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
pub fn pj_lock_create_recursive_mutex( pool: *mut pj_pool_t, name: *const c_char, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
pub fn pj_lock_create_null_mutex( pool: *mut pj_pool_t, name: *const c_char, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
pub fn pj_lock_create_semaphore( pool: *mut pj_pool_t, name: *const c_char, initial: c_uint, max: c_uint, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
pub fn pj_lock_acquire(lock: *mut pj_lock_t) -> pj_status_t;
pub fn pj_lock_tryacquire(lock: *mut pj_lock_t) -> pj_status_t;
pub fn pj_lock_release(lock: *mut pj_lock_t) -> pj_status_t;
pub fn pj_lock_destroy(lock: *mut pj_lock_t) -> pj_status_t;
pub fn pj_grp_lock_config_default(cfg: *mut pj_grp_lock_config);
pub fn pj_grp_lock_create( pool: *mut pj_pool_t, cfg: *const pj_grp_lock_config, p_grp_lock: *mut *mut pj_grp_lock_t, ) -> pj_status_t;
pub fn pj_grp_lock_create_w_handler( pool: *mut pj_pool_t, cfg: *const pj_grp_lock_config, member: *mut c_void, handler: Option<unsafe extern "C" fn(member: *mut c_void)>, p_grp_lock: *mut *mut pj_grp_lock_t, ) -> pj_status_t;
pub fn pj_grp_lock_destroy(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
pub fn pj_grp_lock_replace( old_lock: *mut pj_grp_lock_t, new_lock: *mut pj_grp_lock_t, ) -> pj_status_t;
pub fn pj_grp_lock_acquire(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
pub fn pj_grp_lock_tryacquire(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
pub fn pj_grp_lock_release(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
pub fn pj_grp_lock_add_handler( grp_lock: *mut pj_grp_lock_t, pool: *mut pj_pool_t, member: *mut c_void, handler: Option<unsafe extern "C" fn(member: *mut c_void)>, ) -> pj_status_t;
pub fn pj_grp_lock_del_handler( grp_lock: *mut pj_grp_lock_t, member: *mut c_void, handler: Option<unsafe extern "C" fn(member: *mut c_void)>, ) -> pj_status_t;
pub fn pj_grp_lock_add_ref(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
pub fn pj_grp_lock_dec_ref(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
pub fn pj_grp_lock_get_ref(grp_lock: *mut pj_grp_lock_t) -> c_int;
pub fn pj_grp_lock_dump(grp_lock: *mut pj_grp_lock_t);
pub fn pj_grp_lock_chain_lock( grp_lock: *mut pj_grp_lock_t, ext_lock: *mut pj_lock_t, pos: c_int, ) -> pj_status_t;
pub fn pj_grp_lock_unchain_lock( grp_lock: *mut pj_grp_lock_t, ext_lock: *mut pj_lock_t, ) -> pj_status_t;
pub fn pj_timer_heap_mem_size(count: pj_size_t) -> pj_size_t;
pub fn pj_timer_heap_create( pool: *mut pj_pool_t, count: pj_size_t, ht: *mut *mut pj_timer_heap_t, ) -> pj_status_t;
pub fn pj_timer_heap_destroy(ht: *mut pj_timer_heap_t);
pub fn pj_timer_heap_set_lock( ht: *mut pj_timer_heap_t, lock: *mut pj_lock_t, auto_del: pj_bool_t, );
pub fn pj_timer_heap_set_max_timed_out_per_poll( ht: *mut pj_timer_heap_t, count: c_uint, ) -> c_uint;
pub fn pj_timer_entry_init( entry: *mut pj_timer_entry, id: c_int, user_data: *mut c_void, cb: pj_timer_heap_callback, ) -> *mut pj_timer_entry;
pub fn pj_timer_entry_running(entry: *mut pj_timer_entry) -> pj_bool_t;
pub fn pj_timer_heap_schedule_dbg( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, delay: *const pj_time_val, src_file: *const c_char, src_line: c_int, ) -> pj_status_t;
pub fn pj_timer_heap_schedule_w_grp_lock_dbg( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, delay: *const pj_time_val, id_val: c_int, grp_lock: *mut pj_grp_lock_t, src_file: *const c_char, src_line: c_int, ) -> pj_status_t;
pub fn pj_timer_heap_cancel( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, ) -> c_int;
pub fn pj_timer_heap_cancel_if_active( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, id_val: c_int, ) -> c_int;
pub fn pj_timer_heap_count(ht: *mut pj_timer_heap_t) -> pj_size_t;
pub fn pj_timer_heap_earliest_time( ht: *mut pj_timer_heap_t, timeval: *mut pj_time_val, ) -> pj_status_t;
pub fn pj_timer_heap_poll( ht: *mut pj_timer_heap_t, next_delay: *mut pj_time_val, ) -> c_uint;
pub fn pj_timer_heap_dump(ht: *mut pj_timer_heap_t);
pub fn pj_ntohs(netshort: pj_uint16_t) -> pj_uint16_t;
pub fn pj_htons(hostshort: pj_uint16_t) -> pj_uint16_t;
pub fn pj_ntohl(netlong: pj_uint32_t) -> pj_uint32_t;
pub fn pj_htonl(hostlong: pj_uint32_t) -> pj_uint32_t;
pub fn pj_inet_ntoa(inaddr: pj_in_addr) -> *mut c_char;
pub fn pj_inet_aton(cp: *const pj_str_t, inp: *mut pj_in_addr) -> c_int;
pub fn pj_inet_pton( af: c_int, src: *const pj_str_t, dst: *mut c_void,) -> pj_status_t;
pub fn pj_inet_ntop( af: c_int, src: *const c_void, dst: *mut c_char, size: c_int,) -> pj_status_t;
pub fn pj_inet_ntop2( af: c_int, src: *const c_void, dst: *mut c_char, size: c_int, ) -> *mut c_char;
pub fn pj_sockaddr_print( addr: *const pj_sockaddr_t, buf: *mut c_char, size: c_int, flags: c_uint, )-> *mut c_char;
pub fn pj_inet_addr(cp: *const pj_str_t) -> pj_in_addr;
pub fn pj_inet_addr2(cp: *const c_char) -> pj_in_addr;

pub fn pj_sockaddr_in_init( addr: *mut pj_sockaddr_in, cp: *const pj_str_t, port: pj_uint16_t, ) -> pj_status_t;
pub fn pj_sockaddr_init( af: c_int, addr: *mut pj_sockaddr, cp: *const pj_str_t, port: pj_uint16_t,) -> pj_status_t;
pub fn pj_sockaddr_cmp( addr1: *const pj_sockaddr_t, addr2: *const pj_sockaddr_t, ) -> c_int;
pub fn pj_sockaddr_get_addr(addr: *const pj_sockaddr_t) -> *mut c_void;
pub fn pj_sockaddr_has_addr(addr: *const pj_sockaddr_t) -> pj_bool_t;
pub fn pj_sockaddr_get_addr_len(addr: *const pj_sockaddr_t) -> c_uint;
pub fn pj_sockaddr_get_len(addr: *const pj_sockaddr_t) -> c_uint;
pub fn pj_sockaddr_copy_addr(dst: *mut pj_sockaddr, src: *const pj_sockaddr);
pub fn pj_sockaddr_cp(dst: *mut pj_sockaddr_t, src: *const pj_sockaddr_t);
pub fn pj_sockaddr_synthesize( dst_af: c_int, dst: *mut pj_sockaddr_t, src: *const pj_sockaddr_t, ) -> pj_status_t;
pub fn pj_sockaddr_in_get_addr(addr: *const pj_sockaddr_in) -> pj_in_addr;
pub fn pj_sockaddr_in_set_addr(addr: *mut pj_sockaddr_in, hostaddr: pj_uint32_t);
pub fn pj_sockaddr_in_set_str_addr(addr: *mut pj_sockaddr_in,cp: *const pj_str_t,) -> pj_status_t;
pub fn pj_sockaddr_set_str_addr( af: c_int, addr: *mut pj_sockaddr, cp: *const pj_str_t, ) -> pj_status_t;
pub fn pj_sockaddr_get_port(addr: *const pj_sockaddr_t) -> pj_uint16_t;
pub fn pj_sockaddr_in_get_port(addr: *const pj_sockaddr_in) -> pj_uint16_t;
pub fn pj_sockaddr_set_port(addr: *mut pj_sockaddr, hostport: pj_uint16_t) -> pj_status_t;
pub fn pj_sockaddr_in_set_port(addr: *mut pj_sockaddr_in, hostport: pj_uint16_t);
pub fn pj_sockaddr_parse( af: c_int, options: c_uint, str_: *const pj_str_t, addr: *mut pj_sockaddr,) -> pj_status_t;
pub fn pj_sockaddr_parse2( af: c_int, options: c_uint, str_: *const pj_str_t, hostpart: *mut pj_str_t, port: *mut pj_uint16_t, raf: *mut c_int, ) -> pj_status_t;

pub fn pj_gethostname() -> *const pj_str_t;
pub fn pj_gethostaddr() -> pj_in_addr;

pub fn pj_sock_socket( family: c_int, type_: c_int, protocol: c_int, sock: *mut pj_sock_t, ) -> pj_status_t;
pub fn pj_sock_close(sockfd: pj_sock_t) -> pj_status_t;
pub fn pj_sock_bind( sockfd: pj_sock_t, my_addr: *const pj_sockaddr_t, addrlen: c_int, ) -> pj_status_t;
pub fn pj_sock_bind_in(sockfd: pj_sock_t, addr: pj_uint32_t, port: pj_uint16_t) -> pj_status_t;
pub fn pj_sock_bind_random( sockfd: pj_sock_t, addr: *const pj_sockaddr_t, port_range: pj_uint16_t, max_try: pj_uint16_t,) -> pj_status_t;
pub fn pj_sock_listen(sockfd: pj_sock_t, backlog: c_int) -> pj_status_t;
pub fn pj_sock_accept( serverfd: pj_sock_t, newsock: *mut pj_sock_t, addr: *mut pj_sockaddr_t, addrlen: *mut c_int,) -> pj_status_t;
pub fn pj_sock_connect( sockfd: pj_sock_t, serv_addr: *const pj_sockaddr_t, addrlen: c_int, ) -> pj_status_t;
pub fn pj_sock_getpeername( sockfd: pj_sock_t, addr: *mut pj_sockaddr_t, namelen: *mut c_int, ) -> pj_status_t;
pub fn pj_sock_getsockname( sockfd: pj_sock_t, addr: *mut pj_sockaddr_t, namelen: *mut c_int, ) -> pj_status_t;
pub fn pj_sock_getsockopt( sockfd: pj_sock_t, level: pj_uint16_t, optname: pj_uint16_t, optval: *mut c_void, optlen: *mut c_int, ) -> pj_status_t;
pub fn pj_sock_setsockopt( sockfd: pj_sock_t, level: pj_uint16_t, optname: pj_uint16_t, optval: *const c_void, optlen: c_int, ) -> pj_status_t;
pub fn pj_sock_setsockopt_params( sockfd: pj_sock_t, params: *const pj_sockopt_params, ) -> pj_status_t;
pub fn pj_sock_setsockopt_sobuf( sockfd: pj_sock_t, optname: pj_uint16_t, auto_retry: pj_bool_t, buf_size: *mut c_uint, ) -> pj_status_t;
pub fn pj_sock_recv( sockfd: pj_sock_t, buf: *mut c_void, len: *mut pj_ssize_t, flags: c_uint, ) -> pj_status_t;
pub fn pj_sock_recvfrom( sockfd: pj_sock_t, buf: *mut c_void, len: *mut pj_ssize_t, flags: c_uint, from: *mut pj_sockaddr_t, fromlen: *mut c_int, ) -> pj_status_t;
pub fn pj_sock_send( sockfd: pj_sock_t, buf: *const c_void, len: *mut pj_ssize_t, flags: c_uint, ) -> pj_status_t;
pub fn pj_sock_sendto( sockfd: pj_sock_t, buf: *const c_void, len: *mut pj_ssize_t, flags: c_uint, to: *const pj_sockaddr_t, tolen: c_int, ) -> pj_status_t;
pub fn pj_sock_shutdown(sockfd: pj_sock_t, how: c_int) -> pj_status_t;
pub fn pj_addr_str_print( host_str: *const pj_str_t, port: c_int, buf: *mut c_char, size: c_int, flag: c_uint, ) -> *mut c_char;

pub fn pj_dns_make_query( packet: *mut c_void, size: *mut c_uint, id: pj_uint16_t, qtype: c_int, name: *const pj_str_t, ) -> pj_status_t;
pub fn pj_dns_parse_packet( pool: *mut pj_pool_t, packet: *const c_void, size: c_uint, p_res: *mut *mut pj_dns_parsed_packet, ) -> pj_status_t;
pub fn pj_dns_packet_dup( pool: *mut pj_pool_t, p: *const pj_dns_parsed_packet, options: c_uint, p_dst: *mut *mut pj_dns_parsed_packet, );
pub fn pj_dns_get_type_name(type_: c_int) -> *const c_char;
pub fn pj_dns_init_srv_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: c_uint, ttl: c_uint, prio: c_uint, weight: c_uint, port: c_uint, target: *const pj_str_t, );
pub fn pj_dns_init_cname_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: c_uint, ttl: c_uint, name: *const pj_str_t, );
pub fn pj_dns_init_a_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: c_uint, ttl: c_uint, ip_addr: *const pj_in_addr, );
pub fn pj_dns_init_aaaa_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: c_uint, ttl: c_uint, ip_addr: *const pj_in6_addr, );
pub fn pj_dns_dump_packet(res: *const pj_dns_parsed_packet);
pub fn pj_dns_settings_default(s: *mut pj_dns_settings);
pub fn pj_dns_resolver_create( pf: *mut pj_pool_factory, name: *const c_char, options: c_uint, timer: *mut pj_timer_heap_t, ioqueue: *mut pj_ioqueue_t, p_resolver: *mut *mut pj_dns_resolver, ) -> pj_status_t;
pub fn pj_dns_resolver_set_ns( resolver: *mut pj_dns_resolver, count: c_uint, servers: *const pj_str_t, ports: *const pj_uint16_t, ) -> pj_status_t;
pub fn pj_dns_resolver_get_settings( resolver: *mut pj_dns_resolver, st: *mut pj_dns_settings,) -> pj_status_t;
pub fn pj_dns_resolver_set_settings( resolver: *mut pj_dns_resolver, st: *const pj_dns_settings, ) -> pj_status_t;
pub fn pj_dns_resolver_handle_events( resolver: *mut pj_dns_resolver, timeout: *const pj_time_val, );
pub fn pj_dns_resolver_destroy( resolver: *mut pj_dns_resolver, notify: pj_bool_t, ) -> pj_status_t;
pub fn pj_dns_resolver_start_query( resolver: *mut pj_dns_resolver, name: *const pj_str_t, type_: c_int, options: c_uint, cb: pj_dns_callback, user_data: *mut c_void, p_query: *mut *mut pj_dns_async_query, ) -> pj_status_t;
pub fn pj_dns_resolver_cancel_query( query: *mut pj_dns_async_query, notify: pj_bool_t, ) -> pj_status_t;
pub fn pj_dns_parse_a_response( pkt: *const pj_dns_parsed_packet, rec: *mut pj_dns_a_record, ) -> pj_status_t;
pub fn pj_dns_parse_addr_response( pkt: *const pj_dns_parsed_packet, rec: *mut pj_dns_addr_record, ) -> pj_status_t;
pub fn pj_dns_resolver_add_entry( resolver: *mut pj_dns_resolver, pkt: *const pj_dns_parsed_packet, set_ttl: pj_bool_t, ) -> pj_status_t;
pub fn pj_dns_resolver_get_cached_count( resolver: *mut pj_dns_resolver, ) -> c_uint;
pub fn pj_dns_resolver_dump(resolver: *mut pj_dns_resolver, detail: pj_bool_t);

pub fn pj_ioqueue_name() -> *const c_char;
pub fn pj_ioqueue_create( pool: *mut pj_pool_t, max_fd: pj_size_t, ioqueue: *mut *mut pj_ioqueue_t, ) -> pj_status_t;
pub fn pj_ioqueue_destroy(ioque: *mut pj_ioqueue_t) -> pj_status_t;
pub fn pj_ioqueue_set_lock( ioque: *mut pj_ioqueue_t, lock: *mut pj_lock_t, auto_delete: pj_bool_t, ) -> pj_status_t;
pub fn pj_ioqueue_set_default_concurrency( ioqueue: *mut pj_ioqueue_t, allow: pj_bool_t, ) -> pj_status_t;
pub fn pj_ioqueue_register_sock( pool: *mut pj_pool_t, ioque: *mut pj_ioqueue_t, sock: pj_sock_t, user_data: *mut c_void, cb: *const pj_ioqueue_callback, key: *mut *mut pj_ioqueue_key_t, ) -> pj_status_t;
pub fn pj_ioqueue_register_sock2( pool: *mut pj_pool_t, ioque: *mut pj_ioqueue_t, sock: pj_sock_t, grp_lock: *mut pj_grp_lock_t, user_data: *mut c_void, cb: *const pj_ioqueue_callback, key: *mut *mut pj_ioqueue_key_t, ) -> pj_status_t;
pub fn pj_ioqueue_unregister(key: *mut pj_ioqueue_key_t) -> pj_status_t;
pub fn pj_ioqueue_get_user_data(key: *mut pj_ioqueue_key_t) -> *mut c_void;
pub fn pj_ioqueue_set_user_data( key: *mut pj_ioqueue_key_t, user_data: *mut c_void, old_data: *mut *mut c_void, ) -> pj_status_t;
pub fn pj_ioqueue_set_concurrency(key: *mut pj_ioqueue_key_t, allow: pj_bool_t) -> pj_status_t;
pub fn pj_ioqueue_lock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
pub fn pj_ioqueue_trylock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
pub fn pj_ioqueue_unlock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
pub fn pj_ioqueue_op_key_init(op_key: *mut pj_ioqueue_op_key_t, size: pj_size_t);
pub fn pj_ioqueue_is_pending( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, ) -> pj_bool_t;
pub fn pj_ioqueue_post_completion( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, bytes_status: pj_ssize_t, ) -> pj_status_t;
pub fn pj_ioqueue_accept( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, new_sock: *mut pj_sock_t, local: *mut pj_sockaddr_t, remote: *mut pj_sockaddr_t, addrlen: *mut c_int, ) -> pj_status_t;
pub fn pj_ioqueue_connect( key: *mut pj_ioqueue_key_t, addr: *const pj_sockaddr_t, addrlen: c_int, ) -> pj_status_t;
pub fn pj_ioqueue_poll( ioque: *mut pj_ioqueue_t, timeout: *const pj_time_val, ) -> c_int;
pub fn pj_ioqueue_recv( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, buffer: *mut c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_ioqueue_recvfrom( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, buffer: *mut c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, addr: *mut pj_sockaddr_t, addrlen: *mut c_int, ) -> pj_status_t;
pub fn pj_ioqueue_send( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, data: *const c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_ioqueue_sendto( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, data: *const c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, addr: *const pj_sockaddr_t, addrlen: c_int, ) -> pj_status_t;

pub fn pj_sock_set_qos_type(sock: pj_sock_t, type_: pj_qos_type) -> pj_status_t;
pub fn pj_sock_get_qos_type(sock: pj_sock_t, p_type: *mut pj_qos_type) -> pj_status_t;
pub fn pj_sock_apply_qos( sock: pj_sock_t, qos_type: pj_qos_type, qos_params: *mut pj_qos_params, log_level: c_uint, log_sender: *const c_char, sock_name: *const c_char, ) -> pj_status_t;
pub fn pj_sock_apply_qos2( sock: pj_sock_t, qos_type: pj_qos_type, qos_params: *const pj_qos_params, log_level: c_uint, log_sender: *const c_char, sock_name: *const c_char, ) -> pj_status_t;
pub fn pj_qos_get_params(type_: pj_qos_type, p_param: *mut pj_qos_params) -> pj_status_t;
pub fn pj_qos_get_type(param: *const pj_qos_params, p_type: *mut pj_qos_type) -> pj_status_t;
pub fn pj_sock_set_qos_params(sock: pj_sock_t, param: *mut pj_qos_params) -> pj_status_t;
pub fn pj_sock_get_qos_params(sock: pj_sock_t, p_param: *mut pj_qos_params) -> pj_status_t;
pub static mut PJ_NO_MEMORY_EXCEPTION: c_int;
pub fn pj_NO_MEMORY_EXCEPTION() -> c_int;
pub static mut pj_pool_factory_default_policy: pj_pool_factory_policy;

pub fn pj_pool_factory_get_default_policy() -> *const pj_pool_factory_policy;
pub fn pj_pool_create( factory: *mut pj_pool_factory, name: *const c_char, initial_size: pj_size_t, increment_size: pj_size_t, callback: pj_pool_callback, ) -> *mut pj_pool_t;
pub fn pj_pool_release(pool: *mut pj_pool_t);
pub fn pj_pool_safe_release(ppool: *mut *mut pj_pool_t);
pub fn pj_pool_secure_release(ppool: *mut *mut pj_pool_t);
pub fn pj_pool_getobjname(pool: *const pj_pool_t) -> *const c_char;
pub fn pj_pool_reset(pool: *mut pj_pool_t);
pub fn pj_pool_get_capacity(pool: *mut pj_pool_t) -> pj_size_t;
pub fn pj_pool_get_used_size(pool: *mut pj_pool_t) -> pj_size_t;
pub fn pj_pool_alloc(pool: *mut pj_pool_t, size: pj_size_t) -> *mut c_void;
pub fn pj_pool_calloc( pool: *mut pj_pool_t, count: pj_size_t, elem: pj_size_t, ) -> *mut c_void;
pub fn pj_pool_alloc_from_block( block: *mut pj_pool_block, size: pj_size_t,) -> *mut c_void;
pub fn pj_pool_allocate_find( pool: *mut pj_pool_t, size: pj_size_t, ) -> *mut c_void;
pub fn pj_pool_create_int( factory: *mut pj_pool_factory, name: *const c_char, initial_size: pj_size_t, increment_size: pj_size_t, callback: pj_pool_callback, ) -> *mut pj_pool_t;
pub fn pj_pool_init_int( pool: *mut pj_pool_t, name: *const c_char, increment_size: pj_size_t, callback: pj_pool_callback, );
pub fn pj_pool_destroy_int(pool: *mut pj_pool_t);
pub fn pj_caching_pool_init( ch_pool: *mut pj_caching_pool, policy: *const pj_pool_factory_policy, max_capacity: pj_size_t, );
pub fn pj_caching_pool_destroy(ch_pool: *mut pj_caching_pool);

pub fn pj_ssl_cert_load_from_files( pool: *mut pj_pool_t, CA_file: *const pj_str_t, cert_file: *const pj_str_t, privkey_file: *const pj_str_t, privkey_pass: *const pj_str_t, p_cert: *mut *mut pj_ssl_cert_t, ) -> pj_status_t;
pub fn pj_ssl_cert_load_from_files2( pool: *mut pj_pool_t, CA_file: *const pj_str_t, CA_path: *const pj_str_t, cert_file: *const pj_str_t, privkey_file: *const pj_str_t, privkey_pass: *const pj_str_t, p_cert: *mut *mut pj_ssl_cert_t, ) -> pj_status_t;
pub fn pj_ssl_cert_load_from_buffer( pool: *mut pj_pool_t, CA_buf: *const pj_ssl_cert_buffer, cert_buf: *const pj_ssl_cert_buffer, privkey_buf: *const pj_ssl_cert_buffer, privkey_pass: *const pj_str_t, p_cert: *mut *mut pj_ssl_cert_t, ) -> pj_status_t;
pub fn pj_ssl_cert_info_dump( ci: *const pj_ssl_cert_info, indent: *const c_char, buf: *mut c_char, buf_size: pj_size_t, ) -> pj_ssize_t;
pub fn pj_ssl_cert_get_verify_status_strings( verify_status: pj_uint32_t, error_strings: *mut *const c_char, count: *mut c_uint, ) -> pj_status_t;
pub fn pj_ssl_cert_wipe_keys(cert: *mut pj_ssl_cert_t);
pub fn pj_ssl_cipher_get_availables( ciphers: *mut pj_ssl_cipher, cipher_num: *mut c_uint, ) -> pj_status_t;
pub fn pj_ssl_cipher_is_supported(cipher: pj_ssl_cipher) -> pj_bool_t;
pub fn pj_ssl_cipher_name(cipher: pj_ssl_cipher) -> *const c_char;
pub fn pj_ssl_cipher_id(cipher_name: *const c_char) -> pj_ssl_cipher;
pub fn pj_ssl_curve_get_availables( curves: *mut pj_ssl_curve, curve_num: *mut c_uint, ) -> pj_status_t;
pub fn pj_ssl_curve_is_supported(curve: pj_ssl_curve) -> pj_bool_t;
pub fn pj_ssl_curve_name(curve: pj_ssl_curve) -> *const c_char;
pub fn pj_ssl_curve_id(curve_name: *const c_char) -> pj_ssl_curve;
pub fn pj_ssl_sock_param_default(param: *mut pj_ssl_sock_param);
pub fn pj_ssl_sock_param_copy( pool: *mut pj_pool_t, dst: *mut pj_ssl_sock_param, src: *const pj_ssl_sock_param, );
pub fn pj_ssl_sock_create( pool: *mut pj_pool_t, param: *const pj_ssl_sock_param, p_ssock: *mut *mut pj_ssl_sock_t, ) -> pj_status_t;
pub fn pj_ssl_sock_set_certificate( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, cert: *const pj_ssl_cert_t, ) -> pj_status_t;
pub fn pj_ssl_sock_close(ssock: *mut pj_ssl_sock_t) -> pj_status_t;
pub fn pj_ssl_sock_set_user_data( ssock: *mut pj_ssl_sock_t, user_data: *mut c_void, ) -> pj_status_t;
pub fn pj_ssl_sock_get_user_data(ssock: *mut pj_ssl_sock_t) -> *mut c_void;
pub fn pj_ssl_sock_get_info( ssock: *mut pj_ssl_sock_t, info: *mut pj_ssl_sock_info,) -> pj_status_t;
pub fn pj_ssl_sock_start_read( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: c_uint, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_ssl_sock_start_read2( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: c_uint, readbuf: *mut *mut c_void, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_ssl_sock_start_recvfrom( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: c_uint, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_ssl_sock_start_recvfrom2( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: c_uint, readbuf: *mut *mut c_void, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_ssl_sock_send( ssock: *mut pj_ssl_sock_t, send_key: *mut pj_ioqueue_op_key_t, data: *const c_void, size: *mut pj_ssize_t, flags: c_uint, ) -> pj_status_t;
pub fn pj_ssl_sock_sendto( ssock: *mut pj_ssl_sock_t, send_key: *mut pj_ioqueue_op_key_t, data: *const c_void, size: *mut pj_ssize_t, flags: c_uint, addr: *const pj_sockaddr_t, addr_len: c_int, ) -> pj_status_t;
pub fn pj_ssl_sock_start_accept( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, local_addr: *const pj_sockaddr_t, addr_len: c_int, ) -> pj_status_t;
pub fn pj_ssl_sock_start_accept2( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, local_addr: *const pj_sockaddr_t, addr_len: c_int, newsock_param: *const pj_ssl_sock_param, ) -> pj_status_t;
pub fn pj_ssl_sock_start_connect( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, localaddr: *const pj_sockaddr_t, remaddr: *const pj_sockaddr_t, addr_len: c_int, ) -> pj_status_t;
pub fn pj_ssl_sock_start_connect2( ssock: *mut pj_ssl_sock_t, connect_param: *mut pj_ssl_start_connect_param, ) -> pj_status_t;
pub fn pj_ssl_sock_renegotiate(ssock: *mut pj_ssl_sock_t) -> pj_status_t;

pub fn pj_str(str_: *mut c_char) -> pj_str_t;

pub fn pj_strassign(dst: *mut pj_str_t, src: *mut pj_str_t) -> *mut pj_str_t;
pub fn pj_strcpy(dst: *mut pj_str_t, src: *const pj_str_t) -> *mut pj_str_t;
pub fn pj_strcpy2(dst: *mut pj_str_t, src: *const c_char) -> *mut pj_str_t;
pub fn pj_strncpy(dst: *mut pj_str_t, src: *const pj_str_t, max: pj_ssize_t) -> *mut pj_str_t;
pub fn pj_strncpy_with_null( dst: *mut pj_str_t, src: *const pj_str_t, max: pj_ssize_t, ) -> *mut pj_str_t;
pub fn pj_strdup( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const pj_str_t, ) -> *mut pj_str_t;
pub fn pj_strdup_with_null( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const pj_str_t, ) -> *mut pj_str_t;
pub fn pj_strdup2( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const c_char, ) -> *mut pj_str_t;
pub fn pj_strdup2_with_null( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const c_char, ) -> *mut pj_str_t;
pub fn pj_strdup3(pool: *mut pj_pool_t, src: *const c_char) -> pj_str_t;
pub fn pj_strcmp(str1: *const pj_str_t, str2: *const pj_str_t) -> c_int;
pub fn pj_strcmp2( str1: *const pj_str_t, str2: *const c_char, ) -> c_int;
pub fn pj_strncmp( str1: *const pj_str_t, str2: *const pj_str_t, len: pj_size_t, ) -> c_int;
pub fn pj_strncmp2( str1: *const pj_str_t, str2: *const c_char, len: pj_size_t, ) -> c_int;
pub fn pj_stricmp(str1: *const pj_str_t, str2: *const pj_str_t) -> c_int;
pub fn pj_stricmp2( str1: *const pj_str_t, str2: *const c_char, ) -> c_int;
pub fn pj_strnicmp( str1: *const pj_str_t, str2: *const pj_str_t, len: pj_size_t )-> c_int;
pub fn pj_strnicmp2( str1: *const pj_str_t, str2: *const c_char, len: pj_size_t, ) -> c_int;
pub fn pj_strcat(dst: *mut pj_str_t, src: *const pj_str_t);
pub fn pj_strcat2(dst: *mut pj_str_t, src: *const c_char);
pub fn pj_strspn(str_: *const pj_str_t, set_char: *const pj_str_t) -> pj_ssize_t;
pub fn pj_strspn2(str_: *const pj_str_t, set_char: *const c_char) -> pj_ssize_t;
pub fn pj_strcspn(str_: *const pj_str_t, set_char: *const pj_str_t) -> pj_ssize_t;
pub fn pj_strcspn2( str_: *const pj_str_t, set_char: *const c_char, ) -> pj_ssize_t;
pub fn pj_strtok( str_: *const pj_str_t, delim: *const pj_str_t, tok: *mut pj_str_t, start_idx: pj_size_t, ) -> pj_ssize_t;
pub fn pj_strtok2( str_: *const pj_str_t, delim: *const c_char, tok: *mut pj_str_t, start_idx: pj_size_t, ) -> pj_ssize_t;
pub fn pj_strstr(str_: *const pj_str_t, substr: *const pj_str_t) -> *mut c_char;
pub fn pj_stristr( str_: *const pj_str_t, substr: *const pj_str_t, ) -> *mut c_char;
pub fn pj_strltrim(str_: *mut pj_str_t) -> *mut pj_str_t;
pub fn pj_strrtrim(str_: *mut pj_str_t) -> *mut pj_str_t;
pub fn pj_strtrim(str_: *mut pj_str_t) -> *mut pj_str_t;
pub fn pj_create_random_string( str_: *mut c_char, length: pj_size_t, ) -> *mut c_char;
pub fn pj_strtol(str_: *const pj_str_t) -> c_long;
pub fn pj_strtol2(str_: *const pj_str_t, value: *mut c_long) -> pj_status_t;
pub fn pj_strtoul(str_: *const pj_str_t) -> c_ulong;
pub fn pj_strtoul2( str_: *const pj_str_t, endptr: *mut pj_str_t, base: c_uint, ) -> c_ulong;
pub fn pj_strtoul3( str_: *const pj_str_t, value: *mut c_ulong, base: c_uint, ) -> pj_status_t;
pub fn pj_strtof(str_: *const pj_str_t) -> f32;
pub fn pj_utoa( val: c_ulong, buf: *mut c_char, ) -> c_int;
pub fn pj_utoa_pad( val: c_ulong, buf: *mut c_char, min_dig: c_int, pad: c_int, ) -> c_int;
pub fn pj_get_sys_info() -> *const pj_sys_info;
pub fn pj_getpid() -> pj_uint32_t;
pub fn pj_thread_create( pool: *mut pj_pool_t, thread_name: *const c_char, proc_: pj_thread_proc, arg: *mut c_void, stack_size: pj_size_t, flags: c_uint, thread: *mut *mut pj_thread_t, ) -> pj_status_t;
pub fn pj_thread_register( thread_name: *const c_char, desc: *mut c_long, thread: *mut *mut pj_thread_t, ) -> pj_status_t;
pub fn pj_thread_is_registered() -> pj_bool_t;
pub fn pj_thread_get_prio(thread: *mut pj_thread_t) -> c_int;
pub fn pj_thread_set_prio(thread: *mut pj_thread_t, prio: c_int) -> pj_status_t;
pub fn pj_thread_get_prio_min(thread: *mut pj_thread_t) -> c_int;
pub fn pj_thread_get_prio_max(thread: *mut pj_thread_t) -> c_int;
pub fn pj_thread_get_os_handle(thread: *mut pj_thread_t) -> *mut c_void;
pub fn pj_thread_get_name(thread: *mut pj_thread_t) -> *const c_char;
pub fn pj_thread_resume(thread: *mut pj_thread_t) -> pj_status_t;
pub fn pj_thread_this() -> *mut pj_thread_t;
pub fn pj_thread_join(thread: *mut pj_thread_t) -> pj_status_t;
pub fn pj_thread_destroy(thread: *mut pj_thread_t) -> pj_status_t;
pub fn pj_thread_sleep(msec: c_uint) -> pj_status_t;
pub fn pj_symbianos_poll( priority: c_int, ms_timeout: c_int, ) -> pj_bool_t;
pub fn pj_symbianos_set_params(prm: *mut pj_symbianos_params) -> pj_status_t;
pub fn pj_symbianos_set_connection_status(up: pj_bool_t);
pub fn pj_thread_local_alloc(index: *mut c_long) -> pj_status_t;
pub fn pj_thread_local_free(index: c_long);
pub fn pj_thread_local_set( index: c_long, value: *mut c_void, ) -> pj_status_t;
pub fn pj_thread_local_get(index: c_long) -> *mut c_void;
pub fn pj_atomic_create( pool: *mut pj_pool_t, initial: pj_atomic_value_t, atomic: *mut *mut pj_atomic_t, ) -> pj_status_t;
pub fn pj_atomic_destroy(atomic_var: *mut pj_atomic_t) -> pj_status_t;
pub fn pj_atomic_set(atomic_var: *mut pj_atomic_t, value: pj_atomic_value_t);
pub fn pj_atomic_get(atomic_var: *mut pj_atomic_t) -> pj_atomic_value_t;
pub fn pj_atomic_inc(atomic_var: *mut pj_atomic_t);
pub fn pj_atomic_inc_and_get(atomic_var: *mut pj_atomic_t) -> pj_atomic_value_t;
pub fn pj_atomic_dec(atomic_var: *mut pj_atomic_t);
pub fn pj_atomic_dec_and_get(atomic_var: *mut pj_atomic_t) -> pj_atomic_value_t;
pub fn pj_atomic_add(atomic_var: *mut pj_atomic_t, value: pj_atomic_value_t);
pub fn pj_atomic_add_and_get( atomic_var: *mut pj_atomic_t, value: pj_atomic_value_t, ) -> pj_atomic_value_t;
pub fn pj_mutex_create( pool: *mut pj_pool_t, name: *const c_char, type_: c_int, mutex: *mut *mut pj_mutex_t, ) -> pj_status_t;
pub fn pj_mutex_create_simple( pool: *mut pj_pool_t, name: *const c_char, mutex: *mut *mut pj_mutex_t, ) -> pj_status_t;
pub fn pj_mutex_create_recursive( pool: *mut pj_pool_t, name: *const c_char, mutex: *mut *mut pj_mutex_t, ) -> pj_status_t;
pub fn pj_mutex_lock(mutex: *mut pj_mutex_t) -> pj_status_t;
pub fn pj_mutex_unlock(mutex: *mut pj_mutex_t) -> pj_status_t;
pub fn pj_mutex_trylock(mutex: *mut pj_mutex_t) -> pj_status_t;
pub fn pj_mutex_destroy(mutex: *mut pj_mutex_t) -> pj_status_t;
pub fn pj_mutex_is_locked(mutex: *mut pj_mutex_t) -> pj_bool_t;
pub fn pj_rwmutex_create( pool: *mut pj_pool_t, name: *const c_char, mutex: *mut *mut pj_rwmutex_t, ) -> pj_status_t;
pub fn pj_rwmutex_lock_read(mutex: *mut pj_rwmutex_t) -> pj_status_t;
pub fn pj_rwmutex_lock_write(mutex: *mut pj_rwmutex_t) -> pj_status_t;
pub fn pj_rwmutex_unlock_read(mutex: *mut pj_rwmutex_t) -> pj_status_t;
pub fn pj_rwmutex_unlock_write(mutex: *mut pj_rwmutex_t) -> pj_status_t;
pub fn pj_rwmutex_destroy(mutex: *mut pj_rwmutex_t) -> pj_status_t;
pub fn pj_enter_critical_section();
pub fn pj_leave_critical_section();
pub fn pj_sem_create( pool: *mut pj_pool_t, name: *const c_char, initial: c_uint, max: c_uint, sem: *mut *mut pj_sem_t, ) -> pj_status_t;
pub fn pj_sem_wait(sem: *mut pj_sem_t) -> pj_status_t;
pub fn pj_sem_trywait(sem: *mut pj_sem_t) -> pj_status_t;
pub fn pj_sem_post(sem: *mut pj_sem_t) -> pj_status_t;
pub fn pj_sem_destroy(sem: *mut pj_sem_t) -> pj_status_t;
pub fn pj_event_create( pool: *mut pj_pool_t, name: *const c_char, manual_reset: pj_bool_t, initial: pj_bool_t, event: *mut *mut pj_event_t, ) -> pj_status_t;
pub fn pj_event_wait(event: *mut pj_event_t) -> pj_status_t;
pub fn pj_event_trywait(event: *mut pj_event_t) -> pj_status_t;
pub fn pj_event_set(event: *mut pj_event_t) -> pj_status_t;
pub fn pj_event_pulse(event: *mut pj_event_t) -> pj_status_t;
pub fn pj_event_reset(event: *mut pj_event_t) -> pj_status_t;
pub fn pj_event_destroy(event: *mut pj_event_t) -> pj_status_t;
pub fn pj_gettimeofday(tv: *mut pj_time_val) -> pj_status_t;
pub fn pj_time_decode(tv: *const pj_time_val, pt: *mut pj_parsed_time) -> pj_status_t;
pub fn pj_time_encode(pt: *const pj_parsed_time, tv: *mut pj_time_val) -> pj_status_t;
pub fn pj_time_local_to_gmt(tv: *mut pj_time_val) -> pj_status_t;
pub fn pj_time_gmt_to_local(tv: *mut pj_time_val) -> pj_status_t;
pub fn pj_term_set_color(color: pj_color_t) -> pj_status_t;
pub fn pj_term_get_color() -> pj_color_t;
pub fn pj_gettickcount(tv: *mut pj_time_val) -> pj_status_t;
pub fn pj_get_timestamp(ts: *mut pj_timestamp) -> pj_status_t;
pub fn pj_get_timestamp_freq(freq: *mut pj_timestamp) -> pj_status_t;
pub fn pj_elapsed_time(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_time_val;
pub fn pj_elapsed_msec(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
pub fn pj_elapsed_msec64(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint64_t;
pub fn pj_elapsed_usec(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
pub fn pj_elapsed_nanosec(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
pub fn pj_elapsed_cycle(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
pub fn pj_run_app( main_func: pj_main_func_ptr, argc: c_int, argv: *mut *mut c_char, flags: c_uint, ) -> c_int;
pub fn pj_thread_init() -> pj_status_t;

pub fn pj_ice_get_cand_type_name(type_: pj_ice_cand_type) -> *const c_char;
pub fn pj_ice_sess_role_name(role: pj_ice_sess_role) -> *const c_char;
pub fn pj_ice_calc_foundation( pool: *mut pj_pool_t, foundation: *mut pj_str_t, type_: pj_ice_cand_type, base_addr: *const pj_sockaddr, );
pub fn pj_ice_sess_options_default(opt: *mut pj_ice_sess_options);
pub fn pj_ice_sess_create( stun_cfg: *mut pj_stun_config, name: *const c_char, role: pj_ice_sess_role, comp_cnt: c_uint, cb: *const pj_ice_sess_cb, local_ufrag: *const pj_str_t, local_passwd: *const pj_str_t, grp_lock: *mut pj_grp_lock_t, p_ice: *mut *mut pj_ice_sess, ) -> pj_status_t;
pub fn pj_ice_sess_get_options( ice: *mut pj_ice_sess, opt: *mut pj_ice_sess_options, ) -> pj_status_t;
pub fn pj_ice_sess_set_options( ice: *mut pj_ice_sess, opt: *const pj_ice_sess_options, ) -> pj_status_t;
pub fn pj_ice_sess_destroy(ice: *mut pj_ice_sess) -> pj_status_t;
pub fn pj_ice_sess_change_role( ice: *mut pj_ice_sess, new_role: pj_ice_sess_role, ) -> pj_status_t;
pub fn pj_ice_sess_set_prefs(ice: *mut pj_ice_sess, prefs: *const pj_uint8_t) -> pj_status_t;
pub fn pj_ice_sess_add_cand( ice: *mut pj_ice_sess, comp_id: c_uint, transport_id: c_uint, type_: pj_ice_cand_type, local_pref: pj_uint16_t, foundation: *const pj_str_t, addr: *const pj_sockaddr_t, base_addr: *const pj_sockaddr_t, rel_addr: *const pj_sockaddr_t, addr_len: c_int, p_cand_id: *mut c_uint, ) -> pj_status_t;
pub fn pj_ice_sess_find_default_cand( ice: *mut pj_ice_sess, comp_id: c_uint, p_cand_id: *mut c_int, ) -> pj_status_t;
pub fn pj_ice_sess_create_check_list( ice: *mut pj_ice_sess, rem_ufrag: *const pj_str_t, rem_passwd: *const pj_str_t, rem_cand_cnt: c_uint, rem_cand: *const pj_ice_sess_cand, ) -> pj_status_t;
pub fn pj_ice_sess_update_check_list( ice: *mut pj_ice_sess, rem_ufrag: *const pj_str_t, rem_passwd: *const pj_str_t, rem_cand_cnt: c_uint, rem_cand: *const pj_ice_sess_cand, trickle_done: pj_bool_t, ) -> pj_status_t;
pub fn pj_ice_sess_start_check(ice: *mut pj_ice_sess) -> pj_status_t;
pub fn pj_ice_sess_send_data( ice: *mut pj_ice_sess, comp_id: c_uint, data: *const c_void, data_len: pj_size_t, ) -> pj_status_t;
pub fn pj_ice_sess_on_rx_pkt( ice: *mut pj_ice_sess, comp_id: c_uint, transport_id: c_uint, pkt: *mut c_void, pkt_size: pj_size_t, src_addr: *const pj_sockaddr_t, src_addr_len: c_int, ) -> pj_status_t;

pub fn pj_stun_sock_op_name(op: pj_stun_sock_op) -> *const c_char;
pub fn pj_stun_sock_cfg_default(cfg: *mut pj_stun_sock_cfg);
pub fn pj_stun_sock_create( stun_cfg: *mut pj_stun_config, name: *const c_char, af: c_int, cb: *const pj_stun_sock_cb, cfg: *const pj_stun_sock_cfg, user_data: *mut c_void, p_sock: *mut *mut pj_stun_sock, ) -> pj_status_t;
pub fn pj_stun_sock_start( stun_sock: *mut pj_stun_sock, domain: *const pj_str_t, default_port: pj_uint16_t, resolver: *mut pj_dns_resolver, ) -> pj_status_t;
pub fn pj_stun_sock_destroy(sock: *mut pj_stun_sock) -> pj_status_t;
pub fn pj_stun_sock_set_user_data( stun_sock: *mut pj_stun_sock, user_data: *mut c_void, ) -> pj_status_t;
pub fn pj_stun_sock_get_user_data(stun_sock: *mut pj_stun_sock) -> *mut c_void;
pub fn pj_stun_sock_get_grp_lock(stun_sock: *mut pj_stun_sock) -> *mut pj_grp_lock_t;
pub fn pj_stun_sock_get_info( stun_sock: *mut pj_stun_sock, info: *mut pj_stun_sock_info, ) -> pj_status_t;
pub fn pj_stun_sock_sendto( stun_sock: *mut pj_stun_sock, send_key: *mut pj_ioqueue_op_key_t, pkt: *const c_void, pkt_len: c_uint, flag: c_uint, dst_addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_get_method_name( msg_type: c_uint, ) -> *const c_char;
pub fn pj_stun_get_class_name( msg_type: c_uint, ) -> *const c_char;
pub fn pj_stun_get_attr_name( attr_type: c_uint, ) -> *const c_char;
pub fn pj_stun_get_err_reason(err_code: c_int) -> pj_str_t;
pub fn pj_stun_set_padding_char(chr: c_int) -> c_int;
pub fn pj_stun_msg_init( msg: *mut pj_stun_msg, msg_type: c_uint, magic: pj_uint32_t, tsx_id: *const pj_uint8_t, ) -> pj_status_t;
pub fn pj_stun_msg_create( pool: *mut pj_pool_t, msg_type: c_uint, magic: pj_uint32_t, tsx_id: *const pj_uint8_t, p_msg: *mut *mut pj_stun_msg, ) -> pj_status_t;
pub fn pj_stun_msg_clone(pool: *mut pj_pool_t, msg: *const pj_stun_msg) -> *mut pj_stun_msg;
pub fn pj_stun_msg_create_response( pool: *mut pj_pool_t, req_msg: *const pj_stun_msg, err_code: c_uint, err_msg: *const pj_str_t, p_response: *mut *mut pj_stun_msg, ) -> pj_status_t;
pub fn pj_stun_msg_add_attr(msg: *mut pj_stun_msg, attr: *mut pj_stun_attr_hdr) -> pj_status_t;
pub fn pj_stun_msg_encode( msg: *mut pj_stun_msg, pkt_buf: *mut pj_uint8_t, buf_size: pj_size_t, options: c_uint, key: *const pj_str_t, p_msg_len: *mut pj_size_t, ) -> pj_status_t;
pub fn pj_stun_msg_check( pdu: *const pj_uint8_t, pdu_len: pj_size_t, options: c_uint, ) -> pj_status_t;
pub fn pj_stun_msg_decode( pool: *mut pj_pool_t, pdu: *const pj_uint8_t, pdu_len: pj_size_t, options: c_uint, p_msg: *mut *mut pj_stun_msg, p_parsed_len: *mut pj_size_t, p_response: *mut *mut pj_stun_msg, ) -> pj_status_t;
pub fn pj_stun_msg_dump( msg: *const pj_stun_msg, buffer: *mut c_char, length: c_uint, printed_len: *mut c_uint, ) -> *mut c_char;
pub fn pj_stun_msg_find_attr( msg: *const pj_stun_msg, attr_type: c_int, start_index: c_uint, ) -> *mut pj_stun_attr_hdr;
pub fn pj_stun_attr_clone( pool: *mut pj_pool_t, attr: *const pj_stun_attr_hdr, ) -> *mut pj_stun_attr_hdr;
pub fn pj_stun_sockaddr_attr_init( attr: *mut pj_stun_sockaddr_attr, attr_type: c_int, xor_ed: pj_bool_t, addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_sockaddr_attr_create( pool: *mut pj_pool_t, attr_type: c_int, xor_ed: pj_bool_t, addr: *const pj_sockaddr_t, addr_len: c_uint, p_attr: *mut *mut pj_stun_sockaddr_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_sockaddr_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_type: c_int, xor_ed: pj_bool_t, addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_string_attr_init( attr: *mut pj_stun_string_attr, pool: *mut pj_pool_t, attr_type: c_int, value: *const pj_str_t, ) -> pj_status_t;
pub fn pj_stun_string_attr_create( pool: *mut pj_pool_t, attr_type: c_int, value: *const pj_str_t, p_attr: *mut *mut pj_stun_string_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_string_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_type: c_int, value: *const pj_str_t, ) -> pj_status_t;
pub fn pj_stun_uint_attr_create( pool: *mut pj_pool_t, attr_type: c_int, value: pj_uint32_t, p_attr: *mut *mut pj_stun_uint_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_uint_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_type: c_int, value: pj_uint32_t, ) -> pj_status_t;
pub fn pj_stun_uint64_attr_create( pool: *mut pj_pool_t, attr_type: c_int, value: *const pj_timestamp, p_attr: *mut *mut pj_stun_uint64_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_uint64_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_type: c_int, value: *const pj_timestamp, ) -> pj_status_t;
pub fn pj_stun_msgint_attr_create( pool: *mut pj_pool_t, p_attr: *mut *mut pj_stun_msgint_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_msgint_attr(pool: *mut pj_pool_t, msg: *mut pj_stun_msg) -> pj_status_t;
pub fn pj_stun_errcode_attr_create( pool: *mut pj_pool_t, err_code: c_int, err_reason: *const pj_str_t, p_attr: *mut *mut pj_stun_errcode_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_errcode_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, err_code: c_int, err_reason: *const pj_str_t, ) -> pj_status_t;
pub fn pj_stun_unknown_attr_create( pool: *mut pj_pool_t, attr_cnt: c_uint, attr: *const pj_uint16_t, p_attr: *mut *mut pj_stun_unknown_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_unknown_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_cnt: c_uint, attr: *const pj_uint16_t, ) -> pj_status_t;
pub fn pj_stun_binary_attr_init( attr: *mut pj_stun_binary_attr, pool: *mut pj_pool_t, attr_type: c_int, data: *const pj_uint8_t, length: c_uint, ) -> pj_status_t;
pub fn pj_stun_binary_attr_create( pool: *mut pj_pool_t, attr_type: c_int, data: *const pj_uint8_t, length: c_uint, p_attr: *mut *mut pj_stun_binary_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_binary_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_type: c_int, data: *const pj_uint8_t, length: c_uint, ) -> pj_status_t;
pub fn pj_stun_empty_attr_create( pool: *mut pj_pool_t, attr_type: c_int, p_attr: *mut *mut pj_stun_empty_attr, ) -> pj_status_t;
pub fn pj_stun_msg_add_empty_attr( pool: *mut pj_pool_t, msg: *mut pj_stun_msg, attr_type: c_int, ) -> pj_status_t;
pub fn pj_stun_auth_cred_dup( pool: *mut pj_pool_t, dst: *mut pj_stun_auth_cred, src: *const pj_stun_auth_cred, );
pub fn pj_stun_req_cred_info_dup( pool: *mut pj_pool_t, dst: *mut pj_stun_req_cred_info, src: *const pj_stun_req_cred_info, );
pub fn pj_stun_create_key( pool: *mut pj_pool_t, key: *mut pj_str_t, realm: *const pj_str_t, username: *const pj_str_t, data_type: pj_stun_passwd_type, data: *const pj_str_t, );
pub fn pj_stun_authenticate_request( pkt: *const pj_uint8_t, pkt_len: c_uint, msg: *const pj_stun_msg, cred: *mut pj_stun_auth_cred, pool: *mut pj_pool_t, info: *mut pj_stun_req_cred_info, p_response: *mut *mut pj_stun_msg, ) -> pj_status_t;
pub fn pj_stun_auth_valid_for_msg(msg: *const pj_stun_msg) -> pj_bool_t;
pub fn pj_stun_authenticate_response( pkt: *const pj_uint8_t, pkt_len: c_uint, msg: *const pj_stun_msg, key: *const pj_str_t, ) -> pj_status_t;
pub fn pj_stun_client_tsx_create( cfg: *mut pj_stun_config, pool: *mut pj_pool_t, grp_lock: *mut pj_grp_lock_t, cb: *const pj_stun_tsx_cb, p_tsx: *mut *mut pj_stun_client_tsx, ) -> pj_status_t;
pub fn pj_stun_client_tsx_schedule_destroy( tsx: *mut pj_stun_client_tsx, delay: *const pj_time_val, ) -> pj_status_t;
pub fn pj_stun_client_tsx_destroy(tsx: *mut pj_stun_client_tsx) -> pj_status_t;
pub fn pj_stun_client_tsx_stop(tsx: *mut pj_stun_client_tsx) -> pj_status_t;
pub fn pj_stun_client_tsx_is_complete(tsx: *mut pj_stun_client_tsx) -> pj_bool_t;
pub fn pj_stun_client_tsx_set_data( tsx: *mut pj_stun_client_tsx, data: *mut c_void, ) -> pj_status_t;
pub fn pj_stun_client_tsx_get_data(tsx: *mut pj_stun_client_tsx) -> *mut c_void;
pub fn pj_stun_client_tsx_send_msg( tsx: *mut pj_stun_client_tsx, retransmit: pj_bool_t, pkt: *mut c_void, pkt_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_client_tsx_retransmit( tsx: *mut pj_stun_client_tsx, mod_count: pj_bool_t, ) -> pj_status_t;
pub fn pj_stun_client_tsx_on_rx_msg( tsx: *mut pj_stun_client_tsx, msg: *const pj_stun_msg, src_addr: *const pj_sockaddr_t, src_addr_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_session_create( cfg: *mut pj_stun_config, name: *const c_char, cb: *const pj_stun_session_cb, fingerprint: pj_bool_t, grp_lock: *mut pj_grp_lock_t, p_sess: *mut *mut pj_stun_session, ) -> pj_status_t;
pub fn pj_stun_session_destroy(sess: *mut pj_stun_session) -> pj_status_t;
pub fn pj_stun_session_set_user_data( sess: *mut pj_stun_session, user_data: *mut c_void, ) -> pj_status_t;
pub fn pj_stun_session_get_user_data(sess: *mut pj_stun_session) -> *mut c_void;
pub fn pj_stun_session_get_grp_lock(sess: *mut pj_stun_session) -> *mut pj_grp_lock_t;
pub fn pj_stun_session_set_software_name( sess: *mut pj_stun_session, sw: *const pj_str_t, ) -> pj_status_t;
pub fn pj_stun_session_set_credential( sess: *mut pj_stun_session, auth_type: pj_stun_auth_type, cred: *const pj_stun_auth_cred, ) -> pj_status_t;
pub fn pj_stun_session_set_log(sess: *mut pj_stun_session, flags: c_uint);
pub fn pj_stun_session_use_fingerprint( sess: *mut pj_stun_session, use_: pj_bool_t, ) -> pj_bool_t;
pub fn pj_stun_session_create_req( sess: *mut pj_stun_session, msg_type: c_int, magic: pj_uint32_t, tsx_id: *const pj_uint8_t, p_tdata: *mut *mut pj_stun_tx_data, ) -> pj_status_t;
pub fn pj_stun_session_create_ind( sess: *mut pj_stun_session, msg_type: c_int, p_tdata: *mut *mut pj_stun_tx_data, ) -> pj_status_t;
pub fn pj_stun_session_create_res( sess: *mut pj_stun_session, rdata: *const pj_stun_rx_data, err_code: c_uint, err_msg: *const pj_str_t, p_tdata: *mut *mut pj_stun_tx_data, ) -> pj_status_t;
pub fn pj_stun_session_send_msg( sess: *mut pj_stun_session, token: *mut c_void, cache_res: pj_bool_t, retransmit: pj_bool_t, dst_addr: *const pj_sockaddr_t, addr_len: c_uint, tdata: *mut pj_stun_tx_data, ) -> pj_status_t;
pub fn pj_stun_session_respond( sess: *mut pj_stun_session, rdata: *const pj_stun_rx_data, code: c_uint, err_msg: *const c_char, token: *mut c_void, cache: pj_bool_t, dst_addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_session_cancel_req( sess: *mut pj_stun_session, tdata: *mut pj_stun_tx_data, notify: pj_bool_t, status: pj_status_t, ) -> pj_status_t;
pub fn pj_stun_session_retransmit_req( sess: *mut pj_stun_session, tdata: *mut pj_stun_tx_data, mod_count: pj_bool_t, ) -> pj_status_t;
pub fn pj_stun_session_on_rx_pkt( sess: *mut pj_stun_session, packet: *const c_void, pkt_size: pj_size_t, options: c_uint, token: *mut c_void, parsed_len: *mut pj_size_t, src_addr: *const pj_sockaddr_t, src_addr_len: c_uint, ) -> pj_status_t;
pub fn pj_stun_msg_destroy_tdata(sess: *mut pj_stun_session, tdata: *mut pj_stun_tx_data);

pub fn pj_turn_alloc_param_default(prm: *mut pj_turn_alloc_param);
pub fn pj_turn_alloc_param_copy( pool: *mut pj_pool_t, dst: *mut pj_turn_alloc_param, src: *const pj_turn_alloc_param, );
pub fn pj_turn_state_name(state: pj_turn_state_t) -> *const c_char;
pub fn pj_turn_session_create( cfg: *const pj_stun_config, name: *const c_char, af: c_int, conn_type: pj_turn_tp_type, grp_lock: *mut pj_grp_lock_t, cb: *const pj_turn_session_cb, options: c_uint, user_data: *mut c_void, p_sess: *mut *mut pj_turn_session, ) -> pj_status_t;
pub fn pj_turn_session_shutdown(sess: *mut pj_turn_session) -> pj_status_t;
pub fn pj_turn_session_destroy( sess: *mut pj_turn_session, last_err: pj_status_t, ) -> pj_status_t;
pub fn pj_turn_session_get_info( sess: *mut pj_turn_session, info: *mut pj_turn_session_info, ) -> pj_status_t;
pub fn pj_turn_session_set_user_data( sess: *mut pj_turn_session, user_data: *mut c_void, ) -> pj_status_t;
pub fn pj_turn_session_get_user_data(sess: *mut pj_turn_session) -> *mut c_void;
pub fn pj_turn_session_get_grp_lock(sess: *mut pj_turn_session) -> *mut pj_grp_lock_t;
pub fn pj_turn_session_set_log(sess: *mut pj_turn_session, flags: c_uint);
pub fn pj_turn_session_set_software_name( sess: *mut pj_turn_session, sw: *const pj_str_t, ) -> pj_status_t;
pub fn pj_turn_session_set_server( sess: *mut pj_turn_session, domain: *const pj_str_t, default_port: c_int, resolver: *mut pj_dns_resolver, ) -> pj_status_t;
pub fn pj_turn_session_set_credential( sess: *mut pj_turn_session, cred: *const pj_stun_auth_cred, ) -> pj_status_t;
pub fn pj_turn_session_alloc( sess: *mut pj_turn_session, param: *const pj_turn_alloc_param, ) -> pj_status_t;
pub fn pj_turn_session_set_perm( sess: *mut pj_turn_session, addr_cnt: c_uint, addr: *const pj_sockaddr, options: c_uint, ) -> pj_status_t;
pub fn pj_turn_session_sendto( sess: *mut pj_turn_session, pkt: *const pj_uint8_t, pkt_len: c_uint, peer_addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_turn_session_bind_channel( sess: *mut pj_turn_session, peer: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_turn_session_on_rx_pkt( sess: *mut pj_turn_session, pkt: *mut c_void, pkt_len: pj_size_t, parsed_len: *mut pj_size_t, ) -> pj_status_t;
pub fn pj_turn_session_on_rx_pkt2( sess: *mut pj_turn_session, prm: *mut pj_turn_session_on_rx_pkt_param, ) -> pj_status_t;
pub fn pj_turn_session_connection_bind( sess: *mut pj_turn_session, pool: *mut pj_pool_t, conn_id: pj_uint32_t, peer_addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_turn_sock_tls_cfg_default(tls_cfg: *mut pj_turn_sock_tls_cfg);
pub fn pj_turn_sock_tls_cfg_dup( pool: *mut pj_pool_t, dst: *mut pj_turn_sock_tls_cfg, src: *const pj_turn_sock_tls_cfg, );
pub fn pj_turn_sock_tls_cfg_wipe_keys(tls_cfg: *mut pj_turn_sock_tls_cfg);
pub fn pj_turn_sock_cfg_default(cfg: *mut pj_turn_sock_cfg);
pub fn pj_turn_sock_create( cfg: *mut pj_stun_config, af: c_int, conn_type: pj_turn_tp_type, cb: *const pj_turn_sock_cb, setting: *const pj_turn_sock_cfg, user_data: *mut c_void, p_turn_sock: *mut *mut pj_turn_sock, ) -> pj_status_t;
pub fn pj_turn_sock_destroy(turn_sock: *mut pj_turn_sock);
pub fn pj_turn_sock_set_user_data( turn_sock: *mut pj_turn_sock, user_data: *mut c_void, ) -> pj_status_t;
pub fn pj_turn_sock_get_user_data(turn_sock: *mut pj_turn_sock) -> *mut c_void;
pub fn pj_turn_sock_get_grp_lock(turn_sock: *mut pj_turn_sock) -> *mut pj_grp_lock_t;
pub fn pj_turn_sock_get_info( turn_sock: *mut pj_turn_sock, info: *mut pj_turn_session_info, ) -> pj_status_t;
pub fn pj_turn_sock_lock(turn_sock: *mut pj_turn_sock) -> pj_status_t;
pub fn pj_turn_sock_unlock(turn_sock: *mut pj_turn_sock) -> pj_status_t;
pub fn pj_turn_sock_set_log(turn_sock: *mut pj_turn_sock, flags: c_uint);
pub fn pj_turn_sock_set_software_name( turn_sock: *mut pj_turn_sock, sw: *const pj_str_t, ) -> pj_status_t;
pub fn pj_turn_sock_alloc( turn_sock: *mut pj_turn_sock, domain: *const pj_str_t, default_port: c_int, resolver: *mut pj_dns_resolver, cred: *const pj_stun_auth_cred, param: *const pj_turn_alloc_param, ) -> pj_status_t;
pub fn pj_turn_sock_set_perm( turn_sock: *mut pj_turn_sock, addr_cnt: c_uint, addr: *const pj_sockaddr, options: c_uint, ) -> pj_status_t;
pub fn pj_turn_sock_sendto( turn_sock: *mut pj_turn_sock, pkt: *const pj_uint8_t, pkt_len: c_uint, peer_addr: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;
pub fn pj_turn_sock_bind_channel( turn_sock: *mut pj_turn_sock, peer: *const pj_sockaddr_t, addr_len: c_uint, ) -> pj_status_t;

pub fn pj_ice_strans_cfg_default(cfg: *mut pj_ice_strans_cfg);
pub fn pj_ice_strans_stun_cfg_default(cfg: *mut pj_ice_strans_stun_cfg);
pub fn pj_ice_strans_turn_cfg_default(cfg: *mut pj_ice_strans_turn_cfg);
pub fn pj_ice_strans_cfg_copy( pool: *mut pj_pool_t, dst: *mut pj_ice_strans_cfg, src: *const pj_ice_strans_cfg, );
pub fn pj_ice_strans_create( name: *const c_char, cfg: *const pj_ice_strans_cfg, comp_cnt: c_uint, user_data: *mut c_void, cb: *const pj_ice_strans_cb, p_ice_st: *mut *mut pj_ice_strans, ) -> pj_status_t;
pub fn pj_ice_strans_get_state(ice_st: *mut pj_ice_strans) -> pj_ice_strans_state;
pub fn pj_ice_strans_state_name(state: pj_ice_strans_state) -> *const c_char;
pub fn pj_ice_strans_destroy(ice_st: *mut pj_ice_strans) -> pj_status_t;
pub fn pj_ice_strans_get_user_data(ice_st: *mut pj_ice_strans) -> *mut c_void;
pub fn pj_ice_strans_get_options( ice_st: *mut pj_ice_strans, opt: *mut pj_ice_sess_options, ) -> pj_status_t;
pub fn pj_ice_strans_set_options( ice_st: *mut pj_ice_strans, opt: *const pj_ice_sess_options, ) -> pj_status_t;
pub fn pj_ice_strans_update_comp_cnt( ice_st: *mut pj_ice_strans, comp_cnt: c_uint, ) -> pj_status_t;
pub fn pj_ice_strans_get_grp_lock(ice_st: *mut pj_ice_strans) -> *mut pj_grp_lock_t;
pub fn pj_ice_strans_init_ice( ice_st: *mut pj_ice_strans, role: pj_ice_sess_role, local_ufrag: *const pj_str_t, local_passwd: *const pj_str_t, ) -> pj_status_t;
pub fn pj_ice_strans_has_sess(ice_st: *mut pj_ice_strans) -> pj_bool_t;
pub fn pj_ice_strans_sess_is_running(ice_st: *mut pj_ice_strans) -> pj_bool_t;
pub fn pj_ice_strans_sess_is_complete(ice_st: *mut pj_ice_strans) -> pj_bool_t;
pub fn pj_ice_strans_get_running_comp_cnt(ice_st: *mut pj_ice_strans) -> c_uint;
pub fn pj_ice_strans_get_ufrag_pwd( ice_st: *mut pj_ice_strans, loc_ufrag: *mut pj_str_t, loc_pwd: *mut pj_str_t, rem_ufrag: *mut pj_str_t, rem_pwd: *mut pj_str_t, ) -> pj_status_t;
pub fn pj_ice_strans_get_cands_count( ice_st: *mut pj_ice_strans, comp_id: c_uint, ) -> c_uint;
pub fn pj_ice_strans_enum_cands( ice_st: *mut pj_ice_strans, comp_id: c_uint, count: *mut c_uint, cand: *mut pj_ice_sess_cand, ) -> pj_status_t;
pub fn pj_ice_strans_get_def_cand( ice_st: *mut pj_ice_strans, comp_id: c_uint, cand: *mut pj_ice_sess_cand, ) -> pj_status_t;
pub fn pj_ice_strans_get_role(ice_st: *mut pj_ice_strans) -> pj_ice_sess_role;
pub fn pj_ice_strans_change_role( ice_st: *mut pj_ice_strans, new_role: pj_ice_sess_role, ) -> pj_status_t;
pub fn pj_ice_strans_start_ice( ice_st: *mut pj_ice_strans, rem_ufrag: *const pj_str_t, rem_passwd: *const pj_str_t, rcand_cnt: c_uint, rcand: *const pj_ice_sess_cand, ) -> pj_status_t;
pub fn pj_ice_strans_update_check_list( ice_st: *mut pj_ice_strans, rem_ufrag: *const pj_str_t, rem_passwd: *const pj_str_t, rcand_cnt: c_uint, rcand: *const pj_ice_sess_cand, rcand_end: pj_bool_t, ) -> pj_status_t;
pub fn pj_ice_strans_get_valid_pair( ice_st: *const pj_ice_strans, comp_id: c_uint, ) -> *const pj_ice_sess_check;
pub fn pj_ice_strans_stop_ice(ice_st: *mut pj_ice_strans) -> pj_status_t;
pub fn pj_ice_strans_sendto( ice_st: *mut pj_ice_strans, comp_id: c_uint, data: *const c_void, data_len: pj_size_t, dst_addr: *const pj_sockaddr_t, dst_addr_len: c_int, ) -> pj_status_t;
pub fn pj_ice_strans_sendto2( ice_st: *mut pj_ice_strans, comp_id: c_uint, data: *const c_void, data_len: pj_size_t, dst_addr: *const pj_sockaddr_t, dst_addr_len: c_int, ) -> pj_status_t;
pub fn pj_stun_get_nat_name(type_: pj_stun_nat_type) -> *const c_char;
pub fn pj_stun_detect_nat_type( server: *const pj_sockaddr_in, stun_cfg: *mut pj_stun_config, user_data: *mut c_void, cb: pj_stun_nat_detect_cb, ) -> pj_status_t;
pub fn pj_stun_detect_nat_type2( server: *const pj_sockaddr, stun_cfg: *mut pj_stun_config, user_data: *mut c_void, cb: pj_stun_nat_detect_cb, ) -> pj_status_t;
pub static mut pj_optarg: *mut c_char;
pub static mut pj_optind: c_int;
pub static mut pj_optopt: c_int;
pub fn pj_getopt( argc: c_int, argv: *const *mut c_char, shortopts: *const c_char, ) -> c_int;
pub fn pj_getopt_long( argc: c_int, argv: *const *mut c_char, options: *const c_char, longopts: *const pj_getopt_option, longind: *mut c_int, ) -> c_int;
pub fn pj_getopt_long_only( argc: c_int, argv: *const *mut c_char, shortopts: *const c_char, longopts: *const pj_getopt_option, longind: *mut c_int, ) -> c_int;
pub fn pj_base64_encode( input: *const pj_uint8_t, in_len: c_int, output: *mut c_char, out_len: *mut c_int, ) -> pj_status_t;
pub fn pj_base64_decode( input: *const pj_str_t, out: *mut pj_uint8_t, out_len: *mut c_int, ) -> pj_status_t;
pub fn pj_crc32_init(ctx: *mut pj_crc32_context);
pub fn pj_crc32_update( ctx: *mut pj_crc32_context, data: *const pj_uint8_t, nbytes: pj_size_t, ) -> pj_uint32_t;
pub fn pj_crc32_final(ctx: *mut pj_crc32_context) -> pj_uint32_t;
pub fn pj_crc32_calc(data: *const pj_uint8_t, nbytes: pj_size_t) -> pj_uint32_t;
pub fn pj_md5_init(pms: *mut pj_md5_context);
pub fn pj_md5_update( pms: *mut pj_md5_context, data: *const pj_uint8_t, nbytes: c_uint, );
pub fn pj_md5_final(pms: *mut pj_md5_context, digest: *mut pj_uint8_t);
pub fn pj_hmac_md5( input: *const pj_uint8_t, input_len: c_uint, key: *const pj_uint8_t, key_len: c_uint, digest: *mut pj_uint8_t,);
pub fn pj_hmac_md5_init( hctx: *mut pj_hmac_md5_context, key: *const pj_uint8_t, key_len: c_uint,);
pub fn pj_hmac_md5_update( hctx: *mut pj_hmac_md5_context, input: *const pj_uint8_t, input_len: c_uint,);
pub fn pj_hmac_md5_final(hctx: *mut pj_hmac_md5_context, digest: *mut pj_uint8_t);
pub fn pj_sha1_init(ctx: *mut pj_sha1_context);
pub fn pj_sha1_update(ctx: *mut pj_sha1_context, data: *const pj_uint8_t, nbytes: pj_size_t);
pub fn pj_sha1_final(ctx: *mut pj_sha1_context, digest: *mut pj_uint8_t);
pub fn pj_hmac_sha1( input: *const pj_uint8_t, input_len: c_uint, key: *const pj_uint8_t, key_len: c_uint, digest: *mut pj_uint8_t, );
pub fn pj_hmac_sha1_init( hctx: *mut pj_hmac_sha1_context, key: *const pj_uint8_t, key_len: c_uint, );
pub fn pj_hmac_sha1_update( hctx: *mut pj_hmac_sha1_context, input: *const pj_uint8_t, input_len: c_uint,);
pub fn pj_hmac_sha1_final(hctx: *mut pj_hmac_sha1_context, digest: *mut pj_uint8_t);
pub fn pj_dns_srv_resolve( domain_name: *const pj_str_t, res_name: *const pj_str_t, def_port: c_uint, pool: *mut pj_pool_t, resolver: *mut pj_dns_resolver, option: c_uint, token: *mut c_void, cb: pj_dns_srv_resolver_cb, p_query: *mut *mut pj_dns_srv_async_query, ) -> pj_status_t;
pub fn pj_dns_srv_cancel_query( query: *mut pj_dns_srv_async_query, notify: pj_bool_t, ) -> pj_status_t;
pub fn pj_dns_server_create( pf: *mut pj_pool_factory, ioqueue: *mut pj_ioqueue_t, af: c_int, port: c_uint, flags: c_uint, p_srv: *mut *mut pj_dns_server, ) -> pj_status_t;
pub fn pj_dns_server_destroy(srv: *mut pj_dns_server) -> pj_status_t;
pub fn pj_dns_server_add_rec( srv: *mut pj_dns_server, count: c_uint, rr: *const pj_dns_parsed_rr, ) -> pj_status_t;
pub fn pj_dns_server_del_rec( srv: *mut pj_dns_server, dns_class: c_int, type_: pj_dns_type, name: *const pj_str_t, ) -> pj_status_t;
pub fn pj_str_unescape(pool: *mut pj_pool_t, src: *const pj_str_t) -> pj_str_t;
pub fn pj_strcpy_unescape(dst: *mut pj_str_t, src: *const pj_str_t) -> *mut pj_str_t;
pub fn pj_strncpy_escape( dst: *mut pj_str_t, src: *const pj_str_t, max: pj_ssize_t, unres: *const pj_cis_t, ) -> *mut pj_str_t;
pub fn pj_strncpy2_escape( dst: *mut c_char, src: *const pj_str_t, max: pj_ssize_t, unres: *const pj_cis_t, ) -> pj_ssize_t;


// json utility
pub fn pj_json_elem_null(el: *mut pj_json_elem, name: *mut pj_str_t);
pub fn pj_json_elem_bool(el: *mut pj_json_elem, name: *mut pj_str_t, val: pj_bool_t);
pub fn pj_json_elem_number(el: *mut pj_json_elem, name: *mut pj_str_t, val: f32);
pub fn pj_json_elem_string(el: *mut pj_json_elem, name: *mut pj_str_t, val: *mut pj_str_t);
pub fn pj_json_elem_array(el: *mut pj_json_elem, name: *mut pj_str_t);
pub fn pj_json_elem_obj(el: *mut pj_json_elem, name: *mut pj_str_t);
pub fn pj_json_elem_add(el: *mut pj_json_elem, child: *mut pj_json_elem);
pub fn pj_json_parse( pool: *mut pj_pool_t, buffer: *mut c_char, size: *mut c_uint, err_info: *mut pj_json_err_info, ) -> *mut pj_json_elem;
pub fn pj_json_write( elem: *const pj_json_elem, buffer: *mut c_char, size: *mut c_uint, ) -> pj_status_t;
pub fn pj_json_writef( elem: *const pj_json_elem, writer: pj_json_writer, user_data: *mut c_void,) -> pj_status_t;

// stun
pub fn pjstun_create_bind_req( pool: *mut pj_pool_t, msg: *mut *mut c_void, len: *mut pj_size_t, id_hi: pj_uint32_t, id_lo: pj_uint32_t, ) -> pj_status_t;
pub fn pjstun_parse_msg( buf: *mut c_void, len: pj_size_t, msg: *mut pjstun_msg, ) -> pj_status_t;
pub fn pjstun_msg_find_attr( msg: *mut pjstun_msg, t: pjstun_attr_type, ) -> *mut c_void;
pub fn pjstun_get_mapped_addr( pf: *mut pj_pool_factory, sock_cnt: c_int, sock: *mut pj_sock_t, srv1: *const pj_str_t, port1: c_int, srv2: *const pj_str_t, port2: c_int, mapped_addr: *mut pj_sockaddr_in, ) -> pj_status_t;
pub fn pjstun_get_mapped_addr2( pf: *mut pj_pool_factory, opt: *const pjstun_setting, sock_cnt: c_int, sock: *mut pj_sock_t, mapped_addr: *mut pj_sockaddr_in, ) -> pj_status_t;

// pub fn pj_pcap_filter_default(filter: *mut pj_pcap_filter);
// pub fn pj_pcap_open( pool: *mut pj_pool_t, path: *const c_char, p_file: *mut *mut pj_pcap_file, ) -> pj_status_t;
// pub fn pj_pcap_close(file: *mut pj_pcap_file) -> pj_status_t;
// pub fn pj_pcap_set_filter( file: *mut pj_pcap_file, filter: *const pj_pcap_filter, ) -> pj_status_t;
// pub fn pj_pcap_read_udp( file: *mut pj_pcap_file, udp_hdr: *mut pj_pcap_udp_hdr, udp_payload: *mut pj_uint8_t, udp_payload_size: *mut pj_size_t, ) -> pj_status_t;

pub fn pj_activesock_cfg_default(cfg: *mut pj_activesock_cfg);
pub fn pj_activesock_create( pool: *mut pj_pool_t, sock: pj_sock_t, sock_type: c_int, opt: *const pj_activesock_cfg, ioqueue: *mut pj_ioqueue_t, cb: *const pj_activesock_cb, user_data: *mut c_void, p_asock: *mut *mut pj_activesock_t, ) -> pj_status_t;
pub fn pj_activesock_create_udp( pool: *mut pj_pool_t, addr: *const pj_sockaddr, opt: *const pj_activesock_cfg, ioqueue: *mut pj_ioqueue_t, cb: *const pj_activesock_cb, user_data: *mut c_void, p_asock: *mut *mut pj_activesock_t, bound_addr: *mut pj_sockaddr, ) -> pj_status_t;
pub fn pj_activesock_close(asock: *mut pj_activesock_t) -> pj_status_t;
pub fn pj_activesock_set_user_data( asock: *mut pj_activesock_t, user_data: *mut c_void, ) -> pj_status_t;
pub fn pj_activesock_get_user_data(asock: *mut pj_activesock_t) -> *mut c_void;
pub fn pj_activesock_start_read( asock: *mut pj_activesock_t, pool: *mut pj_pool_t, buff_size: c_uint, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_activesock_start_read2( asock: *mut pj_activesock_t, pool: *mut pj_pool_t, buff_size: c_uint, readbuf: *mut *mut c_void, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_activesock_start_recvfrom( asock: *mut pj_activesock_t, pool: *mut pj_pool_t, buff_size: c_uint, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_activesock_start_recvfrom2( asock: *mut pj_activesock_t, pool: *mut pj_pool_t, buff_size: c_uint, readbuf: *mut *mut c_void, flags: pj_uint32_t, ) -> pj_status_t;
pub fn pj_activesock_send( asock: *mut pj_activesock_t, send_key: *mut pj_ioqueue_op_key_t, data: *const c_void, size: *mut pj_ssize_t, flags: c_uint, ) -> pj_status_t;
pub fn pj_activesock_sendto( asock: *mut pj_activesock_t, send_key: *mut pj_ioqueue_op_key_t, data: *const c_void, size: *mut pj_ssize_t, flags: c_uint, addr: *const pj_sockaddr_t, addr_len: c_int, ) -> pj_status_t;
pub fn pj_activesock_start_accept( asock: *mut pj_activesock_t, pool: *mut pj_pool_t, ) -> pj_status_t;
pub fn pj_activesock_start_connect( asock: *mut pj_activesock_t, pool: *mut pj_pool_t, remaddr: *const pj_sockaddr_t, addr_len: c_int, ) -> pj_status_t;

// http utility
// pub fn pj_http_req_param_default(param: *mut pj_http_req_param);
// pub fn pj_http_headers_add_elmt( headers: *mut pj_http_headers, name: *mut pj_str_t, val: *mut pj_str_t, ) -> pj_status_t;
// pub fn pj_http_headers_add_elmt2( headers: *mut pj_http_headers, name: *mut c_char, val: *mut c_char, ) -> pj_status_t;
// pub fn pj_http_req_parse_url(url: *const pj_str_t, hurl: *mut pj_http_url) -> pj_status_t;
// pub fn pj_http_req_create( pool: *mut pj_pool_t, url: *const pj_str_t, timer: *mut pj_timer_heap_t, ioqueue: *mut pj_ioqueue_t, param: *const pj_http_req_param, hcb: *const pj_http_req_callback, http_req: *mut *mut pj_http_req, ) -> pj_status_t;
// pub fn pj_http_req_set_timeout(http_req: *mut pj_http_req, timeout: *const pj_time_val);
// pub fn pj_http_req_start(http_req: *mut pj_http_req) -> pj_status_t;
// pub fn pj_http_req_cancel(http_req: *mut pj_http_req, notify: pj_bool_t) -> pj_status_t;
// pub fn pj_http_req_destroy(http_req: *mut pj_http_req) -> pj_status_t;
// pub fn pj_http_req_is_running(http_req: *const pj_http_req) -> pj_bool_t;
// pub fn pj_http_req_get_user_data(http_req: *mut pj_http_req) -> *mut c_void;

// telnet utility
// pub fn pj_cli_write_log( cli: *mut pj_cli_t, level: c_int, buffer: *const c_char, len: c_int, );
// pub fn pj_cli_create(cfg: *mut pj_cli_cfg, p_cli: *mut *mut pj_cli_t) -> pj_status_t;
// pub fn pj_cli_get_cmd_id(cmd: *const pj_cli_cmd_spec) -> pj_cli_cmd_id;
// pub fn pj_cli_get_param(cli: *mut pj_cli_t) -> *mut pj_cli_cfg;
// pub fn pj_cli_quit(cli: *mut pj_cli_t, req: *mut pj_cli_sess, restart: pj_bool_t);
// pub fn pj_cli_is_quitting(cli: *mut pj_cli_t) -> pj_bool_t;
// pub fn pj_cli_is_restarting(cli: *mut pj_cli_t) -> pj_bool_t;
// pub fn pj_cli_destroy(cli: *mut pj_cli_t);
// pub fn pj_cli_cfg_default(param: *mut pj_cli_cfg);
// pub fn pj_cli_register_front_end(cli: *mut pj_cli_t, fe: *mut pj_cli_front_end);
// pub fn pj_cli_add_cmd_from_xml( cli: *mut pj_cli_t, group: *mut pj_cli_cmd_spec, xml: *const pj_str_t, handler: pj_cli_cmd_handler, p_cmd: *mut *mut pj_cli_cmd_spec, get_choice: pj_cli_get_dyn_choice, ) -> pj_status_t;
// pub fn pj_cli_exec_info_default(param: *mut pj_cli_exec_info);
// pub fn pj_cli_sess_write_msg( sess: *mut pj_cli_sess, buffer: *const c_char, len: pj_size_t, );
// pub fn pj_cli_sess_parse( sess: *mut pj_cli_sess, cmdline: *mut c_char, val: *mut pj_cli_cmd_val, pool: *mut pj_pool_t, info: *mut pj_cli_exec_info, ) -> pj_status_t;
// pub fn pj_cli_sess_end_session(sess: *mut pj_cli_sess);
// pub fn pj_cli_sess_exec( sess: *mut pj_cli_sess, cmdline: *mut c_char, pool: *mut pj_pool_t, info: *mut pj_cli_exec_info, ) -> pj_status_t;
// pub fn pj_cli_console_cfg_default(param: *mut pj_cli_console_cfg);
// pub fn pj_cli_console_create( cli: *mut pj_cli_t, param: *const pj_cli_console_cfg, p_sess: *mut *mut pj_cli_sess, p_fe: *mut *mut pj_cli_front_end, ) -> pj_status_t;
// pub fn pj_cli_console_process( sess: *mut pj_cli_sess, buf: *mut c_char, maxlen: c_uint, ) -> pj_status_t;
// pub fn pj_cli_telnet_cfg_default(param: *mut pj_cli_telnet_cfg);
// pub fn pj_cli_telnet_create( cli: *mut pj_cli_t, param: *mut pj_cli_telnet_cfg, p_fe: *mut *mut pj_cli_front_end, ) -> pj_status_t;
// pub fn pj_cli_telnet_get_info( fe: *mut pj_cli_front_end, info: *mut pj_cli_telnet_info, ) -> pj_status_t;

pub fn pj_gethostbyname(name: *const pj_str_t, he: *mut pj_hostent) -> pj_status_t;
pub fn pj_gethostip(af: c_int, addr: *mut pj_sockaddr) -> pj_status_t;
pub fn pj_getipinterface( af: c_int, dst: *const pj_str_t, itf_addr: *mut pj_sockaddr, allow_resolve: pj_bool_t, p_dst_addr: *mut pj_sockaddr, ) -> pj_status_t;
pub fn pj_getdefaultipinterface( af: c_int, addr: *mut pj_sockaddr, ) -> pj_status_t;
pub fn pj_getaddrinfo( af: c_int, name: *const pj_str_t, count: *mut c_uint, ai: *mut pj_addrinfo, ) -> pj_status_t;
pub fn pj_array_insert( array: *mut c_void, elem_size: c_uint, count: c_uint, pos: c_uint, value: *const c_void, );
pub fn pj_array_erase( array: *mut c_void, elem_size: c_uint, count: c_uint, pos: c_uint, );
pub fn pj_array_find( array: *const c_void, elem_size: c_uint, count: c_uint, matching: Option< unsafe extern "C" fn(value: *const c_void) -> pj_status_t, >, result: *mut *mut c_void, ) -> pj_status_t;
pub fn pj_log_write( level: c_int, buffer: *const c_char, len: c_int, );
pub fn pj_log( sender: *const c_char, level: c_int, format: *const c_char, marker: *mut __va_list_tag, );
pub fn pj_log_set_log_func(func: pj_log_func);
pub fn pj_log_get_log_func() -> pj_log_func;
pub fn pj_log_set_level(level: c_int);
pub fn pj_log_get_level() -> c_int;
pub fn pj_log_set_decor(decor: c_uint);
pub fn pj_log_get_decor() -> c_uint;
pub fn pj_log_add_indent(indent: c_int);
pub fn pj_log_push_indent();
pub fn pj_log_pop_indent();
pub fn pj_log_set_color(level: c_int, color: pj_color_t);
pub fn pj_log_get_color(level: c_int) -> pj_color_t;
pub fn pj_log_init() -> pj_status_t;
pub fn pj_log_1(src: *const c_char, format: *const c_char, ...);
pub fn pj_log_2(src: *const c_char, format: *const c_char, ...);
pub fn pj_log_3(src: *const c_char, format: *const c_char, ...);
pub fn pj_log_4(src: *const c_char, format: *const c_char, ...);
pub fn pj_log_5(src: *const c_char, format: *const c_char, ...);
pub fn pj_exception_id_alloc( name: *const c_char, id: *mut pj_exception_id_t, ) -> pj_status_t;
pub fn pj_exception_id_free(id: pj_exception_id_t) -> pj_status_t;
pub fn pj_exception_id_name(id: pj_exception_id_t) -> *const c_char;
pub fn pj_throw_exception_(id: pj_exception_id_t);
pub fn pj_push_exception_handler_(rec: *mut pj_exception_state_t);
pub fn pj_pop_exception_handler_(rec: *mut pj_exception_state_t);
pub fn pj_fifobuf_init( fb: *mut pj_fifobuf_t, buffer: *mut c_void, size: c_uint, );
pub fn pj_fifobuf_max_size(fb: *mut pj_fifobuf_t) -> c_uint;
pub fn pj_fifobuf_alloc( fb: *mut pj_fifobuf_t, size: c_uint)-> *mut c_void;
pub fn pj_fifobuf_unalloc( fb: *mut pj_fifobuf_t, buf: *mut c_void, ) -> pj_status_t;
pub fn pj_fifobuf_free(fb: *mut pj_fifobuf_t, buf: *mut c_void) -> pj_status_t;
pub fn pj_file_exists(filename: *const c_char) -> pj_bool_t;
pub fn pj_file_size(filename: *const c_char) -> pj_off_t;
pub fn pj_file_delete(filename: *const c_char) -> pj_status_t;
pub fn pj_file_move( oldname: *const c_char, newname: *const c_char, ) -> pj_status_t;
pub fn pj_file_getstat( filename: *const c_char, stat: *mut pj_file_stat, ) -> pj_status_t;
pub fn pj_file_open( pool: *mut pj_pool_t, pathname: *const c_char, flags: c_uint, fd: *mut pj_oshandle_t, ) -> pj_status_t;
pub fn pj_file_close(fd: pj_oshandle_t) -> pj_status_t;
pub fn pj_file_write( fd: pj_oshandle_t, data: *const c_void, size: *mut pj_ssize_t, ) -> pj_status_t;
pub fn pj_file_read( fd: pj_oshandle_t, data: *mut c_void, size: *mut pj_ssize_t, ) -> pj_status_t;
pub fn pj_file_setpos( fd: pj_oshandle_t, offset: pj_off_t, whence: pj_file_seek_type, ) -> pj_status_t;
pub fn pj_file_getpos(fd: pj_oshandle_t, pos: *mut pj_off_t) -> pj_status_t;
pub fn pj_file_flush(fd: pj_oshandle_t) -> pj_status_t;
pub static PJ_GUID_STRING_LENGTH: c_uint;
pub fn pj_GUID_STRING_LENGTH() -> c_uint;
pub fn pj_generate_unique_string(str_: *mut pj_str_t) -> *mut pj_str_t;
pub fn pj_generate_unique_string_lower(str_: *mut pj_str_t) -> *mut pj_str_t;
pub fn pj_create_unique_string(pool: *mut pj_pool_t, str_: *mut pj_str_t);
pub fn pj_create_unique_string_lower(pool: *mut pj_pool_t, str_: *mut pj_str_t);

pub fn pj_hash_calc( hval: pj_uint32_t, key: *const c_void, keylen: c_uint, ) -> pj_uint32_t;
pub fn pj_hash_calc_tolower( hval: pj_uint32_t, result: *mut c_char, key: *const pj_str_t, ) -> pj_uint32_t;
pub fn pj_hash_create( pool: *mut pj_pool_t, size: c_uint, ) -> *mut pj_hash_table_t;
pub fn pj_hash_get( ht: *mut pj_hash_table_t, key: *const c_void, keylen: c_uint, hval: *mut pj_uint32_t , ) -> *mut c_void;
pub fn pj_hash_get_lower( ht: *mut pj_hash_table_t, key: *const c_void, keylen: c_uint, hval: *mut pj_uint32_t )-> *mut c_void;
pub fn pj_hash_set( pool: *mut pj_pool_t, ht: *mut pj_hash_table_t, key: *const c_void, keylen: c_uint, hval: pj_uint32_t, value: *mut c_void, );
pub fn pj_hash_set_lower( pool: *mut pj_pool_t, ht: *mut pj_hash_table_t, key: *const c_void, keylen: c_uint, hval: pj_uint32_t, value: *mut c_void,);
pub fn pj_hash_set_np( ht: *mut pj_hash_table_t, key: *const c_void, keylen: c_uint, hval: pj_uint32_t, entry_buf: *mut *mut c_void, value: *mut c_void, );
pub fn pj_hash_set_np_lower( ht: *mut pj_hash_table_t, key: *const c_void, keylen: c_uint, hval: pj_uint32_t, entry_buf: *mut *mut c_void, value: *mut c_void, );
pub fn pj_hash_count(ht: *mut pj_hash_table_t) -> c_uint;
pub fn pj_hash_first( ht: *mut pj_hash_table_t, it: *mut pj_hash_iterator_t, ) -> *mut pj_hash_iterator_t;
pub fn pj_hash_next( ht: *mut pj_hash_table_t, it: *mut pj_hash_iterator_t, ) -> *mut pj_hash_iterator_t;
pub fn pj_hash_this( ht: *mut pj_hash_table_t, it: *mut pj_hash_iterator_t, ) -> *mut c_void;

pub fn pj_enum_ip_interface( af: c_int, count: *mut c_uint, ifs: *mut pj_sockaddr, ) -> pj_status_t;
pub fn pj_enum_ip_interface2( opt: *const pj_enum_ip_option, count: *mut c_uint, ifs: *mut pj_sockaddr, ) -> pj_status_t;
pub fn pj_enum_ip_route( count: *mut c_uint, routes: *mut pj_ip_route_entry, ) -> pj_status_t;
pub fn pj_pool_create_on_buf( name: *const c_char, buf: *mut c_void, size: pj_size_t, ) -> *mut pj_pool_t;
pub fn pj_srand(seed: c_uint);
pub fn pj_rand() -> c_int;
pub fn pj_rbtree_init(tree: *mut pj_rbtree, comp: pj_rbtree_comp);
pub fn pj_rbtree_first(tree: *mut pj_rbtree) -> *mut pj_rbtree_node;
pub fn pj_rbtree_last(tree: *mut pj_rbtree) -> *mut pj_rbtree_node;
pub fn pj_rbtree_next(tree: *mut pj_rbtree, node: *mut pj_rbtree_node) -> *mut pj_rbtree_node;
pub fn pj_rbtree_prev(tree: *mut pj_rbtree, node: *mut pj_rbtree_node) -> *mut pj_rbtree_node;
pub fn pj_rbtree_insert( tree: *mut pj_rbtree, node: *mut pj_rbtree_node, ) -> c_int;
pub fn pj_rbtree_find( tree: *mut pj_rbtree, key: *const c_void, ) -> *mut pj_rbtree_node;
pub fn pj_rbtree_erase(tree: *mut pj_rbtree, node: *mut pj_rbtree_node) -> *mut pj_rbtree_node;
pub fn pj_rbtree_max_height( tree: *mut pj_rbtree, node: *mut pj_rbtree_node, ) -> c_uint;
pub fn pj_rbtree_min_height( tree: *mut pj_rbtree, node: *mut pj_rbtree_node, ) -> c_uint;
pub fn PJ_FD_ZERO(fdsetp: *mut pj_fd_set_t);
pub fn PJ_FD_COUNT(fdsetp: *const pj_fd_set_t) -> pj_size_t;
pub fn PJ_FD_SET(fd: pj_sock_t, fdsetp: *mut pj_fd_set_t);
pub fn PJ_FD_CLR(fd: pj_sock_t, fdsetp: *mut pj_fd_set_t);
pub fn PJ_FD_ISSET(fd: pj_sock_t, fdsetp: *const pj_fd_set_t) -> pj_bool_t;
pub fn pj_sock_select( n: c_int, readfds: *mut pj_fd_set_t, writefds: *mut pj_fd_set_t, exceptfds: *mut pj_fd_set_t, timeout: *const pj_time_val, ) -> c_int;
pub fn pj_ansi_to_unicode( str_: *const c_char, len: c_int, wbuf: *mut wchar_t, wbuf_count: c_int, ) -> *mut wchar_t;
pub fn pj_unicode_to_ansi( wstr: *const wchar_t, len: pj_ssize_t, buf: *mut c_char, buf_size: c_int, ) -> *mut c_char;

// xml
pub fn pj_xml_parse( pool: *mut pj_pool_t, msg: *mut c_char, len: pj_size_t, ) -> *mut pj_xml_node;
pub fn pj_xml_print( node: *const pj_xml_node, buf: *mut c_char, len: pj_size_t, prolog: pj_bool_t, ) -> c_int;
pub fn pj_xml_clone(pool: *mut pj_pool_t, rhs: *const pj_xml_node) -> *mut pj_xml_node;
pub fn pj_xml_node_new(pool: *mut pj_pool_t, name: *const pj_str_t) -> *mut pj_xml_node;
pub fn pj_xml_attr_new( pool: *mut pj_pool_t, name: *const pj_str_t, value: *const pj_str_t, ) -> *mut pj_xml_attr;
pub fn pj_xml_add_node(parent: *mut pj_xml_node, node: *mut pj_xml_node);
pub fn pj_xml_add_attr(node: *mut pj_xml_node, attr: *mut pj_xml_attr);
pub fn pj_xml_find_node(parent: *const pj_xml_node, name: *const pj_str_t) -> *mut pj_xml_node;
pub fn pj_xml_find_next_node( parent: *const pj_xml_node, node: *const pj_xml_node, name: *const pj_str_t, ) -> *mut pj_xml_node;
pub fn pj_xml_find_node_rec( parent: *const pj_xml_node, name: *const pj_str_t, ) -> *mut pj_xml_node;
pub fn pj_xml_find_attr( node: *const pj_xml_node, name: *const pj_str_t, value: *const pj_str_t, ) -> *mut pj_xml_attr;
pub fn pj_xml_find( parent: *const pj_xml_node, name: *const pj_str_t, data: *const c_void, match_: Option< unsafe extern "C" fn( arg1: *const pj_xml_node, arg2: *const c_void, ) -> pj_bool_t, >, ) -> *mut pj_xml_node;
pub fn pj_xml_find_rec( parent: *const pj_xml_node, name: *const pj_str_t, data: *const c_void, match_: Option< unsafe extern "C" fn( arg1: *const pj_xml_node, arg2: *const c_void, ) -> pj_bool_t, >, ) -> *mut pj_xml_node;

pub fn pjnath_init() -> pj_status_t;
pub fn pjnath_perror( sender: *const c_char, title: *const c_char, status: pj_status_t, );
}