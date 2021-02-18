#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]


//mod pjproject_sys;
use super::pjdefault::AutoCreate;
use super::pjsua_sys::*;
use std::ptr;




pub const PJ_CC_NAME: &'static [u8; 4usize] = b"gcc\0";
pub const PJ_HAS_INT64: u32 = 1;
pub const PJ_INT64_FMT: &'static [u8; 2usize] = b"L\0";
pub const PJ_HAS_BZERO: u32 = 1;
pub const PJ_LINUX: u32 = 1;
pub const PJ_OS_NAME: &'static [u8; 6usize] = b"linux\0";
pub const PJ_HAS_ARPA_INET_H: u32 = 1;
pub const PJ_HAS_ASSERT_H: u32 = 1;
pub const PJ_HAS_CTYPE_H: u32 = 1;
pub const PJ_HAS_ERRNO_H: u32 = 1;
pub const PJ_HAS_LINUX_SOCKET_H: u32 = 0;
pub const PJ_HAS_MALLOC_H: u32 = 1;
pub const PJ_HAS_NETDB_H: u32 = 1;
pub const PJ_HAS_NETINET_IN_H: u32 = 1;
pub const PJ_HAS_SETJMP_H: u32 = 1;
pub const PJ_HAS_STDARG_H: u32 = 1;
pub const PJ_HAS_STDDEF_H: u32 = 1;
pub const PJ_HAS_STDIO_H: u32 = 1;
pub const PJ_HAS_STDLIB_H: u32 = 1;
pub const PJ_HAS_STRING_H: u32 = 1;
pub const PJ_HAS_SYS_IOCTL_H: u32 = 1;
pub const PJ_HAS_SYS_SELECT_H: u32 = 1;
pub const PJ_HAS_SYS_SOCKET_H: u32 = 1;
pub const PJ_HAS_SYS_TIME_H: u32 = 1;
pub const PJ_HAS_SYS_TIMEB_H: u32 = 1;
pub const PJ_HAS_SYS_TYPES_H: u32 = 1;
pub const PJ_HAS_TIME_H: u32 = 1;
pub const PJ_HAS_UNISTD_H: u32 = 1;
pub const PJ_HAS_SEMAPHORE_H: u32 = 1;
pub const PJ_HAS_MSWSOCK_H: u32 = 0;
pub const PJ_HAS_WINSOCK_H: u32 = 0;
pub const PJ_HAS_WINSOCK2_H: u32 = 0;
pub const PJ_HAS_LOCALTIME_R: u32 = 1;
pub const PJ_SOCK_HAS_INET_ATON: u32 = 1;
pub const PJ_SOCKADDR_HAS_LEN: u32 = 0;
pub const PJ_SELECT_NEEDS_NFDS: u32 = 0;
pub const PJ_HAS_ERRNO_VAR: u32 = 1;
pub const PJ_HAS_SO_ERROR: u32 = 1;
pub const PJ_HAS_THREADS: u32 = 1;
pub const PJ_HAS_HIGH_RES_TIMER: u32 = 1;
pub const PJ_HAS_MALLOC: u32 = 1;
pub const PJ_OS_HAS_CHECK_STACK: u32 = 0;
pub const PJ_NATIVE_STRING_IS_UNICODE: u32 = 0;
pub const PJ_EMULATE_RWMUTEX: u32 = 0;
pub const PJ_THREAD_SET_STACK_SIZE: u32 = 0;
pub const PJ_THREAD_ALLOCATE_STACK: u32 = 0;
pub const PJ_HAS_SOCKLEN_T: u32 = 1;
pub const PJ_M_X86_64: u32 = 1;
pub const PJ_M_NAME: &'static [u8; 7usize] = b"x86_64\0";
pub const PJ_HAS_PENTIUM: u32 = 1;
pub const PJ_IS_LITTLE_ENDIAN: u32 = 1;
pub const PJ_IS_BIG_ENDIAN: u32 = 0;
pub const PJ_DEBUG: u32 = 1;
pub const PJ_DEBUG_MUTEX: u32 = 0;
pub const PJ_FUNCTIONS_ARE_INLINED: u32 = 0;
pub const PJ_HAS_FLOATING_POINT: u32 = 1;
pub const PJ_LOG_MAX_LEVEL: u32 = 5;
pub const PJ_LOG_MAX_SIZE: u32 = 4000;
pub const PJ_LOG_USE_STACK_BUFFER: u32 = 1;
pub const PJ_LOG_ENABLE_INDENT: u32 = 1;
pub const PJ_LOG_INDENT_SIZE: u32 = 1;
pub const PJ_LOG_INDENT_CHAR: u8 = 46u8;
pub const PJ_LOG_SENDER_WIDTH: u32 = 14;
pub const PJ_LOG_THREAD_WIDTH: u32 = 12;
pub const PJ_TERM_HAS_COLOR: u32 = 1;
pub const PJ_SAFE_POOL: u32 = 0;
pub const PJ_POOL_DEBUG: u32 = 0;
pub const PJ_POOL_RELEASE_WIPE_DATA: u32 = 0;
pub const PJ_TIMER_DEBUG: u32 = 1;
pub const PJ_TIMER_USE_COPY: u32 = 1;
pub const PJ_TIMER_USE_LINKED_LIST: u32 = 0;
pub const PJ_GRP_LOCK_DEBUG: u32 = 0;
pub const PJ_THREAD_DEFAULT_STACK_SIZE: u32 = 8192;
pub const PJ_HAS_POOL_ALT_API: u32 = 0;
pub const PJ_HAS_TCP: u32 = 1;
pub const PJ_HAS_IPV6: u32 = 0;
pub const PJ_MAX_HOSTNAME: u32 = 128;
pub const PJ_ACTIVESOCK_MAX_CONSECUTIVE_ACCEPT_ERROR: u32 = 50;
pub const PJ_IOQUEUE_MAX_HANDLES: u32 = 64;
pub const PJ_IOQUEUE_HAS_SAFE_UNREG: u32 = 1;
pub const PJ_IOQUEUE_DEFAULT_ALLOW_CONCURRENCY: u32 = 1;
pub const PJ_IOQUEUE_KEY_FREE_DELAY: u32 = 500;
pub const PJ_FD_SETSIZE_SETABLE: u32 = 0;
pub const PJ_IP_HELPER_IGNORE_LOOPBACK_IF: u32 = 1;
pub const PJ_HAS_SEMAPHORE: u32 = 1;
pub const PJ_SEMAPHORE_USE_DISPATCH_SEM: u32 = 0;
pub const PJ_HAS_EVENT_OBJ: u32 = 1;
pub const PJ_MAXPATH: u32 = 260;
pub const PJ_ENABLE_EXTRA_CHECK: u32 = 1;
pub const PJ_HAS_EXCEPTION_NAMES: u32 = 1;
pub const PJ_MAX_EXCEPTION_ID: u32 = 16;
pub const PJ_EXCEPTION_USE_WIN32_SEH: u32 = 0;
pub const PJ_TIMESTAMP_USE_RDTSC: u32 = 0;
pub const PJ_NATIVE_ERR_POSITIVE: u32 = 1;
pub const PJ_HAS_ERROR_STRING: u32 = 1;
pub const PJ_HAS_STRICMP_ALNUM: u32 = 0;
pub const PJ_QOS_DUMMY: u32 = 1;
pub const PJ_QOS_BSD: u32 = 2;
pub const PJ_QOS_WM: u32 = 3;
pub const PJ_QOS_SYMBIAN: u32 = 4;
pub const PJ_QOS_DARWIN: u32 = 5;
pub const PJ_HAS_SSL_SOCK: u32 = 0;
pub const PJ_SSL_SOCK_IMP_NONE: u32 = 0;
pub const PJ_SSL_SOCK_IMP_OPENSSL: u32 = 1;
pub const PJ_SSL_SOCK_IMP_GNUTLS: u32 = 2;
pub const PJ_SSL_SOCK_IMP_DARWIN: u32 = 3;
pub const PJ_SSL_SOCK_IMP_APPLE: u32 = 4;
pub const PJ_SSL_SOCK_IMP: u32 = 0;
pub const PJ_SSL_SOCK_MAX_CIPHERS: u32 = 256;
pub const PJ_SSL_SOCK_OSSL_CIPHERS: &'static [u8; 26usize] = b"HIGH:-COMPLEMENTOFDEFAULT\0";
pub const PJ_SSL_SOCK_MAX_CURVES: u32 = 32;
pub const PJ_SSL_SOCK_OSSL_USE_THREAD_CB: u32 = 1;
pub const PJ_SOCK_DISABLE_WSAECONNRESET: u32 = 1;
pub const PJ_MAX_SOCKOPT_PARAMS: u32 = 4;
pub const PJ_VERSION_NUM_MAJOR: u32 = 2;
pub const PJ_VERSION_NUM_MINOR: u32 = 10;
pub const PJ_VERSION_NUM_REV: u32 = 0;
pub const PJ_VERSION_NUM_EXTRA: &'static [u8; 5usize] = b"-dev\0";
pub const PJ_VERSION_NUM: u32 = 34209792;
pub const PJ_MAX_OBJ_NAME: u32 = 32;
pub const PJ_ERR_MSG_SIZE: u32 = 80;
pub const PJ_PERROR_TITLE_BUF_SIZE: u32 = 120;
pub const PJ_ERRNO_START: u32 = 20000;
pub const PJ_ERRNO_SPACE_SIZE: u32 = 50000;
pub const PJ_ERRNO_START_STATUS: u32 = 70000;
pub const PJ_ERRNO_START_SYS: u32 = 120000;
pub const PJ_ERRNO_START_USER: u32 = 170000;
pub const PJ_DNS_MAX_IP_IN_A_REC: u32 = 8;
pub const PJ_DNS_SRV_MAX_ADDR: u32 = 8;
pub const PJ_DNS_MAX_NAMES_IN_NAMETABLE: u32 = 16;
pub const PJ_DNS_RESOLVER_MAX_NS: u32 = 16;
pub const PJ_DNS_RESOLVER_QUERY_RETRANSMIT_DELAY: u32 = 2000;
pub const PJ_DNS_RESOLVER_QUERY_RETRANSMIT_COUNT: u32 = 5;
pub const PJ_DNS_RESOLVER_MAX_TTL: u32 = 300;
pub const PJ_DNS_RESOLVER_INVALID_TTL: u32 = 60;
pub const PJ_DNS_RESOLVER_GOOD_NS_TTL: u32 = 600;
pub const PJ_DNS_RESOLVER_BAD_NS_TTL: u32 = 60;
pub const PJ_DNS_RESOLVER_MAX_UDP_SIZE: u32 = 512;
pub const PJ_DNS_RESOLVER_RES_BUF_SIZE: u32 = 512;
pub const PJ_DNS_RESOLVER_TMP_BUF_SIZE: u32 = 4000;
pub const PJ_SCANNER_USE_BITWISE: u32 = 1;
pub const PJSTUN_MAX_ATTR: u32 = 16;
pub const PJ_STUN_MAX_ATTR: u32 = 16;
pub const PJ_CRC32_HAS_TABLES: u32 = 1;
pub const PJ_HTTP_DEFAULT_TIMEOUT: u32 = 60000;
pub const PJ_CLI_POOL_SIZE: u32 = 1024;
pub const PJ_CLI_POOL_INC: u32 = 512;
pub const PJ_CLI_MAX_CMDBUF: u32 = 512;
pub const PJ_CLI_MAX_ARGS: u32 = 8;
pub const PJ_CLI_MAX_HINTS: u32 = 32;
pub const PJ_CLI_MAX_SHORTCUTS: u32 = 4;
pub const PJ_CLI_CONSOLE_POOL_SIZE: u32 = 256;
pub const PJ_CLI_CONSOLE_POOL_INC: u32 = 256;
pub const PJ_CLI_TELNET_POOL_SIZE: u32 = 1024;
pub const PJ_CLI_TELNET_POOL_INC: u32 = 512;
pub const PJ_CLI_MAX_CHOICE_VAL: u32 = 64;
pub const PJ_CLI_MAX_CMD_HISTORY: u32 = 16;
pub const PJ_SOMAXCONN: u32 = 5;
pub const PJ_INVALID_SOCKET: i32 = -1;
pub const PJ_INET_ADDRSTRLEN: u32 = 16;
pub const PJ_INET6_ADDRSTRLEN: u32 = 46;
pub const PJ_SOCKADDR_IN_SIN_ZERO_LEN: u32 = 8;
pub const PJ_IOQUEUE_MAX_EVENTS_IN_SINGLE_POLL: u32 = 16;
pub const PJ_IOQUEUE_MAX_CAND_EVENTS: u32 = 16;
pub const PJ_POOL_ALIGNMENT: u32 = 4;
pub const PJ_CACHING_POOL_ARRAY_SIZE: u32 = 16;
pub const PJ_PI: f64 = 3.141592653589793;
pub const PJ_1_PI: f64 = 0.3183098861837907;
pub const PJ_THREAD_DESC_SIZE: u32 = 64;
pub const PJNATH_ERROR_LEVEL: u32 = 1;
pub const PJ_STUN_RTO_VALUE: u32 = 100;
pub const PJ_STUN_TIMEOUT_VALUE: u32 = 1600;
pub const PJ_STUN_MAX_TRANSMIT_COUNT: u32 = 7;
pub const PJ_STUN_RES_CACHE_DURATION: u32 = 10000;
pub const PJ_STUN_MAX_PKT_LEN: u32 = 800;
pub const PJ_STUN_PORT: u32 = 3478;
pub const PJ_STUN_STRING_ATTR_PAD_CHR: u32 = 0;
pub const PJ_STUN_OLD_STYLE_MI_FINGERPRINT: u32 = 0;
pub const PJ_STUN_SOCK_PKT_LEN: u32 = 2000;
pub const PJ_STUN_KEEP_ALIVE_SEC: u32 = 15;
pub const PJ_TURN_MAX_DNS_SRV_CNT: u32 = 4;
pub const PJ_TURN_MAX_PKT_LEN: u32 = 3000;
pub const PJ_TURN_PERM_TIMEOUT: u32 = 300;
pub const PJ_TURN_CHANNEL_TIMEOUT: u32 = 600;
pub const PJ_TURN_REFRESH_SEC_BEFORE: u32 = 60;
pub const PJ_TURN_KEEP_ALIVE_SEC: u32 = 15;
pub const PJ_TURN_MAX_TCP_CONN_CNT: u32 = 8;
pub const PJ_ICE_MAX_CAND: u32 = 16;
pub const PJ_ICE_ST_MAX_CAND: u32 = 8;
pub const PJ_ICE_MAX_STUN: u32 = 2;
pub const PJ_ICE_MAX_TURN: u32 = 3;
pub const PJ_ICE_COMP_BITS: u32 = 1;
pub const PJ_ICE_MAX_COMP: u32 = 2;
pub const PJNATH_ICE_PRIO_STD: u32 = 1;
pub const PJ_ICE_CAND_TYPE_PREF_BITS: u32 = 8;
pub const PJ_ICE_LOCAL_PREF_BITS: u32 = 0;
pub const PJ_ICE_MAX_CHECKS: u32 = 32;
pub const PJ_ICE_TA_VAL: u32 = 20;
pub const PJ_ICE_CANCEL_ALL: u32 = 1;
pub const PJ_ICE_NOMINATED_CHECK_DELAY: u32 = 400;
pub const PJ_ICE_SESS_KEEP_ALIVE_MIN: u32 = 20;
pub const PJ_ICE_SESS_KEEP_ALIVE_MAX_RAND: u32 = 5;
pub const PJ_ICE_UFRAG_LEN: u32 = 8;
pub const PJ_ICE_PWD_LEN: u32 = 24;
pub const PJNATH_POOL_LEN_ICE_SESS: u32 = 512;
pub const PJNATH_POOL_INC_ICE_SESS: u32 = 512;
pub const PJNATH_POOL_LEN_ICE_STRANS: u32 = 1000;
pub const PJNATH_POOL_INC_ICE_STRANS: u32 = 512;
pub const PJNATH_POOL_LEN_NATCK: u32 = 512;
pub const PJNATH_POOL_INC_NATCK: u32 = 512;
pub const PJNATH_POOL_LEN_STUN_SESS: u32 = 1000;
pub const PJNATH_POOL_INC_STUN_SESS: u32 = 1000;
pub const PJNATH_POOL_LEN_STUN_TDATA: u32 = 1000;
pub const PJNATH_POOL_INC_STUN_TDATA: u32 = 1000;
pub const PJNATH_POOL_LEN_TURN_SESS: u32 = 1000;
pub const PJNATH_POOL_INC_TURN_SESS: u32 = 1000;
pub const PJNATH_POOL_LEN_TURN_SOCK: u32 = 1000;
pub const PJNATH_POOL_INC_TURN_SOCK: u32 = 1000;
pub const PJ_TURN_INVALID_CHANNEL: u32 = 65535;
pub const PJ_STUN_MAGIC: u32 = 554869826;
pub const PJ_STUN_SUCCESS_RESPONSE_BIT: u32 = 256;
pub const PJ_STUN_ERROR_RESPONSE_BIT: u32 = 272;
pub const PJ_STUN_INDICATION_BIT: u32 = 16;
pub const PJNATH_ERRNO_START: u32 = 370000;
pub const PJNATH_EINSTUNMSG: u32 = 370001;
pub const PJNATH_EINSTUNMSGLEN: u32 = 370002;
pub const PJNATH_EINSTUNMSGTYPE: u32 = 370003;
pub const PJNATH_ESTUNTIMEDOUT: u32 = 370004;
pub const PJNATH_ESTUNTOOMANYATTR: u32 = 370021;
pub const PJNATH_ESTUNINATTRLEN: u32 = 370022;
pub const PJNATH_ESTUNDUPATTR: u32 = 370023;
pub const PJNATH_ESTUNFINGERPRINT: u32 = 370030;
pub const PJNATH_ESTUNMSGINTPOS: u32 = 370031;
pub const PJNATH_ESTUNFINGERPOS: u32 = 370033;
pub const PJNATH_ESTUNNOMAPPEDADDR: u32 = 370040;
pub const PJNATH_ESTUNIPV6NOTSUPP: u32 = 370041;
pub const PJNATH_EINVAF: u32 = 370042;
pub const PJNATH_ESTUNINSERVER: u32 = 370050;
pub const PJNATH_ESTUNDESTROYED: u32 = 370060;
pub const PJNATH_ENOICE: u32 = 370080;
pub const PJNATH_EICEINPROGRESS: u32 = 370081;
pub const PJNATH_EICEFAILED: u32 = 370082;
pub const PJNATH_EICEMISMATCH: u32 = 370083;
pub const PJNATH_EICEINCOMPID: u32 = 370086;
pub const PJNATH_EICEINCANDID: u32 = 370087;
pub const PJNATH_EICEINSRCADDR: u32 = 370088;
pub const PJNATH_EICEMISSINGSDP: u32 = 370090;
pub const PJNATH_EICEINCANDSDP: u32 = 370091;
pub const PJNATH_EICENOHOSTCAND: u32 = 370092;
pub const PJNATH_EICENOMTIMEOUT: u32 = 370093;
pub const PJNATH_ETURNINTP: u32 = 370120;
pub const PJLIB_UTIL_ERRNO_START: u32 = 320000;
pub const PJLIB_UTIL_ESTUNRESOLVE: u32 = 320001;
pub const PJLIB_UTIL_ESTUNINMSGTYPE: u32 = 320002;
pub const PJLIB_UTIL_ESTUNINMSGLEN: u32 = 320003;
pub const PJLIB_UTIL_ESTUNINATTRLEN: u32 = 320004;
pub const PJLIB_UTIL_ESTUNINATTRTYPE: u32 = 320005;
pub const PJLIB_UTIL_ESTUNININDEX: u32 = 320006;
pub const PJLIB_UTIL_ESTUNNOBINDRES: u32 = 320007;
pub const PJLIB_UTIL_ESTUNRECVERRATTR: u32 = 320008;
pub const PJLIB_UTIL_ESTUNNOMAP: u32 = 320009;
pub const PJLIB_UTIL_ESTUNNOTRESPOND: u32 = 320010;
pub const PJLIB_UTIL_ESTUNSYMMETRIC: u32 = 320011;
pub const PJLIB_UTIL_ESTUNNOTMAGIC: u32 = 320012;
pub const PJLIB_UTIL_ESTUNFINGERPRINT: u32 = 320013;
pub const PJLIB_UTIL_EINXML: u32 = 320020;
pub const PJLIB_UTIL_EINJSON: u32 = 320030;
pub const PJLIB_UTIL_EDNSQRYTOOSMALL: u32 = 320040;
pub const PJLIB_UTIL_EDNSINSIZE: u32 = 320041;
pub const PJLIB_UTIL_EDNSINCLASS: u32 = 320042;
pub const PJLIB_UTIL_EDNSINNAMEPTR: u32 = 320043;
pub const PJLIB_UTIL_EDNSINNSADDR: u32 = 320044;
pub const PJLIB_UTIL_EDNSNONS: u32 = 320045;
pub const PJLIB_UTIL_EDNSNOWORKINGNS: u32 = 320046;
pub const PJLIB_UTIL_EDNSNOANSWERREC: u32 = 320047;
pub const PJLIB_UTIL_EDNSINANSWER: u32 = 320048;
pub const PJLIB_UTIL_DNS_RCODE_START: u32 = 320050;
pub const PJLIB_UTIL_ESTUNTOOMANYATTR: u32 = 320110;
pub const PJLIB_UTIL_ESTUNUNKNOWNATTR: u32 = 320111;
pub const PJLIB_UTIL_ESTUNINADDRLEN: u32 = 320112;
pub const PJLIB_UTIL_ESTUNIPV6NOTSUPP: u32 = 320113;
pub const PJLIB_UTIL_ESTUNNOTRESPONSE: u32 = 320114;
pub const PJLIB_UTIL_ESTUNINVALIDID: u32 = 320115;
pub const PJLIB_UTIL_ESTUNNOHANDLER: u32 = 320116;
pub const PJLIB_UTIL_ESTUNMSGINTPOS: u32 = 320118;
pub const PJLIB_UTIL_ESTUNFINGERPOS: u32 = 320119;
pub const PJLIB_UTIL_ESTUNNOUSERNAME: u32 = 320120;
pub const PJLIB_UTIL_ESTUNUSERNAME: u32 = 320121;
pub const PJLIB_UTIL_ESTUNMSGINT: u32 = 320122;
pub const PJLIB_UTIL_ESTUNDUPATTR: u32 = 320123;
pub const PJLIB_UTIL_ESTUNNOREALM: u32 = 320124;
pub const PJLIB_UTIL_ESTUNNONCE: u32 = 320125;
pub const PJLIB_UTIL_ESTUNTSXFAILED: u32 = 320126;
pub const PJLIB_UTIL_EHTTPINURL: u32 = 320151;
pub const PJLIB_UTIL_EHTTPINPORT: u32 = 320152;
pub const PJLIB_UTIL_EHTTPINCHDR: u32 = 320153;
pub const PJLIB_UTIL_EHTTPINSBUF: u32 = 320154;
pub const PJLIB_UTIL_EHTTPLOST: u32 = 320155;
pub const PJ_CLI_EEXIT: u32 = 320201;
pub const PJ_CLI_EMISSINGARG: u32 = 320202;
pub const PJ_CLI_ETOOMANYARGS: u32 = 320203;
pub const PJ_CLI_EINVARG: u32 = 320204;
pub const PJ_CLI_EBADNAME: u32 = 320205;
pub const PJ_CLI_EBADID: u32 = 320206;
pub const PJ_CLI_EBADXML: u32 = 320207;
pub const PJ_CLI_EAMBIGUOUS: u32 = 320208;
pub const PJ_CLI_ETELNETLOST: u32 = 320211;
pub const PJ_SHA1_DIGEST_SIZE: u32 = 20;
pub const PJ_HTTP_HEADER_SIZE: u32 = 32;
pub const PJ_CLI_CONSOLE_LOG_LEVEL: u32 = 5;
pub const PJ_CLI_TELNET_LOG_LEVEL: u32 = 4;
pub const PJ_CLI_TELNET_PORT: u32 = 0;
pub const pj_hex_digits: &'static [u8; 17usize] = b"0123456789abcdef\0";
pub const PJ_GUID_MAX_LENGTH: u32 = 36;
pub const PJ_SCAN_AUTOSKIP_WS: ::std::os::raw::c_uint = 1;
pub const PJ_SCAN_AUTOSKIP_WS_HEADER: ::std::os::raw::c_uint = 3;
pub const PJ_SCAN_AUTOSKIP_NEWLINE: ::std::os::raw::c_uint = 4;
pub type pj_int32_t = ::std::os::raw::c_int;
pub type pj_uint32_t = ::std::os::raw::c_uint;
pub type pj_int16_t = ::std::os::raw::c_short;
pub type pj_uint16_t = ::std::os::raw::c_ushort;
pub type pj_int8_t = ::std::os::raw::c_schar;
pub type pj_uint8_t = ::std::os::raw::c_uchar;
pub type pj_size_t = size_t;
pub type pj_ssize_t = ::std::os::raw::c_long;
pub type pj_status_t = ::std::os::raw::c_int;
pub type pj_bool_t = ::std::os::raw::c_int;
pub type pj_char_t = ::std::os::raw::c_char;
pub type pj_int64_t = ::std::os::raw::c_longlong;
pub type pj_uint64_t = ::std::os::raw::c_ulonglong;
pub type size_t = ::std::os::raw::c_ulong;

pub const pj_constants__PJ_SUCCESS: pj_constants_ = 0;
pub const pj_constants__PJ_TRUE: pj_constants_ = 1;
pub const pj_constants__PJ_FALSE: pj_constants_ = 0;

pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
pub type _bindgen_ty_15 = ::std::os::raw::c_uint;

pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint32_t = ::std::os::raw::c_uint;

pub type pj_syn_err_func_ptr = ::std::option::Option<unsafe extern "C" fn(scanner: *mut pj_scanner)>;
pub const PJ_TERM_COLOR_R: ::std::os::raw::c_uint = 2;
pub const PJ_TERM_COLOR_G: ::std::os::raw::c_uint = 4;
pub const PJ_TERM_COLOR_B: ::std::os::raw::c_uint = 1;
pub const PJ_TERM_COLOR_BRIGHT: ::std::os::raw::c_uint = 8;
pub type pj_os_err_type = ::std::os::raw::c_int;
pub type va_list = __builtin_va_list;
pub type pj_list_type = ::std::os::raw::c_void;
pub type wchar_t = ::std::os::raw::c_int;
pub type pj_constants_ = ::std::os::raw::c_uint;
pub type pj_off_t = pj_int64_t;
pub type pj_oshandle_t = *mut ::std::os::raw::c_void;
pub type pj_sock_t = ::std::os::raw::c_long;
pub type pj_sockaddr_t = ::std::os::raw::c_void;
pub type pj_color_t = ::std::os::raw::c_uint;
pub type pj_exception_id_t = ::std::os::raw::c_int;
pub type pj_cis_elem_t = pj_uint32_t;


pub const pj_socket_sd_type_PJ_SD_RECEIVE: pj_socket_sd_type = 0;
pub const pj_socket_sd_type_PJ_SHUT_RD: pj_socket_sd_type = 0;
pub const pj_socket_sd_type_PJ_SD_SEND: pj_socket_sd_type = 1;
pub const pj_socket_sd_type_PJ_SHUT_WR: pj_socket_sd_type = 1;
pub const pj_socket_sd_type_PJ_SD_BOTH: pj_socket_sd_type = 2;
pub const pj_socket_sd_type_PJ_SHUT_RDWR: pj_socket_sd_type = 2;
pub type pj_socket_sd_type = ::std::os::raw::c_uint;
pub type pj_in_addr = in_addr;

pub const PJ_DNS_CLASS_IN: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_14 = ::std::os::raw::c_uint;
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
pub type pj_dns_type = ::std::os::raw::c_uint;

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
pub type pj_dns_rcode = ::std::os::raw::c_uint;

pub const pj_dns_dup_options_PJ_DNS_NO_QD: pj_dns_dup_options = 1;
pub const pj_dns_dup_options_PJ_DNS_NO_ANS: pj_dns_dup_options = 2;
pub const pj_dns_dup_options_PJ_DNS_NO_NS: pj_dns_dup_options = 4;
pub const pj_dns_dup_options_PJ_DNS_NO_AR: pj_dns_dup_options = 8;
pub type pj_dns_dup_options = ::std::os::raw::c_uint;

pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_NONE: pj_ioqueue_operation_e = 0;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_READ: pj_ioqueue_operation_e = 1;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_RECV: pj_ioqueue_operation_e = 2;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_RECV_FROM: pj_ioqueue_operation_e = 4;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_WRITE: pj_ioqueue_operation_e = 8;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_SEND: pj_ioqueue_operation_e = 16;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_SEND_TO: pj_ioqueue_operation_e = 32;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_ACCEPT: pj_ioqueue_operation_e = 64;
pub const pj_ioqueue_operation_e_PJ_IOQUEUE_OP_CONNECT: pj_ioqueue_operation_e = 128;
pub type pj_ioqueue_operation_e = ::std::os::raw::c_uint;

pub const pj_qos_type_PJ_QOS_TYPE_BEST_EFFORT: pj_qos_type = 0;
pub const pj_qos_type_PJ_QOS_TYPE_BACKGROUND: pj_qos_type = 1;
pub const pj_qos_type_PJ_QOS_TYPE_VIDEO: pj_qos_type = 2;
pub const pj_qos_type_PJ_QOS_TYPE_VOICE: pj_qos_type = 3;
pub const pj_qos_type_PJ_QOS_TYPE_CONTROL: pj_qos_type = 4;
pub const pj_qos_type_PJ_QOS_TYPE_SIGNALLING: pj_qos_type = 5;
pub type pj_qos_type = ::std::os::raw::c_uint;
pub const pj_qos_flag_PJ_QOS_PARAM_HAS_DSCP: pj_qos_flag = 1;
pub const pj_qos_flag_PJ_QOS_PARAM_HAS_SO_PRIO: pj_qos_flag = 2;
pub const pj_qos_flag_PJ_QOS_PARAM_HAS_WMM: pj_qos_flag = 4;
pub type pj_qos_flag = ::std::os::raw::c_uint;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_BULK_EFFORT: pj_qos_wmm_prio = 0;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_BULK: pj_qos_wmm_prio = 1;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_VIDEO: pj_qos_wmm_prio = 2;
pub const pj_qos_wmm_prio_PJ_QOS_WMM_PRIO_VOICE: pj_qos_wmm_prio = 3;
pub type pj_qos_wmm_prio = ::std::os::raw::c_uint;

pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_ESUCCESS: pj_ssl_cert_verify_flag_t = 0;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EISSUER_NOT_FOUND: pj_ssl_cert_verify_flag_t = 1;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EUNTRUSTED: pj_ssl_cert_verify_flag_t = 2;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EVALIDITY_PERIOD: pj_ssl_cert_verify_flag_t = 4;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EINVALID_FORMAT: pj_ssl_cert_verify_flag_t = 8;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EINVALID_PURPOSE: pj_ssl_cert_verify_flag_t = 16;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EISSUER_MISMATCH: pj_ssl_cert_verify_flag_t = 32;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_ECRL_FAILURE: pj_ssl_cert_verify_flag_t = 64;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EREVOKED: pj_ssl_cert_verify_flag_t = 128;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_ECHAIN_TOO_LONG: pj_ssl_cert_verify_flag_t = 256;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EIDENTITY_NOT_MATCH: pj_ssl_cert_verify_flag_t = 1073741824;
pub const pj_ssl_cert_verify_flag_t_PJ_SSL_CERT_EUNKNOWN: pj_ssl_cert_verify_flag_t = -2147483648;
pub type pj_ssl_cert_verify_flag_t = ::std::os::raw::c_int;

pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_UNKNOWN: pj_ssl_cert_name_type = 0;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_RFC822: pj_ssl_cert_name_type = 1;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_DNS: pj_ssl_cert_name_type = 2;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_URI: pj_ssl_cert_name_type = 3;
pub const pj_ssl_cert_name_type_PJ_SSL_CERT_NAME_IP: pj_ssl_cert_name_type = 4;
pub type pj_ssl_cert_name_type = ::std::os::raw::c_uint;

pub const pj_ssl_cipher_PJ_TLS_UNKNOWN_CIPHER: pj_ssl_cipher = -1;
pub const pj_ssl_cipher_PJ_TLS_NULL_WITH_NULL_NULL: pj_ssl_cipher = 0;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_NULL_MD5: pj_ssl_cipher = 1;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_NULL_SHA: pj_ssl_cipher = 2;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_NULL_SHA256: pj_ssl_cipher = 59;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_RC4_128_MD5: pj_ssl_cipher = 4;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_RC4_128_SHA: pj_ssl_cipher = 5;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 10;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 47;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 53;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 60;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 61;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 13;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 16;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 19;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 22;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 48;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 49;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 50;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 51;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 54;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 55;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 56;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 57;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 62;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 63;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 64;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 103;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 104;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 105;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 106;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 107;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_RC4_128_MD5: pj_ssl_cipher = 24;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_3DES_EDE_CBC_SHA: pj_ssl_cipher = 27;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_AES_128_CBC_SHA: pj_ssl_cipher = 52;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_AES_256_CBC_SHA: pj_ssl_cipher = 58;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_AES_128_CBC_SHA256: pj_ssl_cipher = 108;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_AES_256_CBC_SHA256: pj_ssl_cipher = 109;
pub const pj_ssl_cipher_PJ_TLS_RSA_EXPORT_WITH_RC4_40_MD5: pj_ssl_cipher = 3;
pub const pj_ssl_cipher_PJ_TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5: pj_ssl_cipher = 6;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_IDEA_CBC_SHA: pj_ssl_cipher = 7;
pub const pj_ssl_cipher_PJ_TLS_RSA_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 8;
pub const pj_ssl_cipher_PJ_TLS_RSA_WITH_DES_CBC_SHA: pj_ssl_cipher = 9;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 11;
pub const pj_ssl_cipher_PJ_TLS_DH_DSS_WITH_DES_CBC_SHA: pj_ssl_cipher = 12;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 14;
pub const pj_ssl_cipher_PJ_TLS_DH_RSA_WITH_DES_CBC_SHA: pj_ssl_cipher = 15;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 17;
pub const pj_ssl_cipher_PJ_TLS_DHE_DSS_WITH_DES_CBC_SHA: pj_ssl_cipher = 18;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 20;
pub const pj_ssl_cipher_PJ_TLS_DHE_RSA_WITH_DES_CBC_SHA: pj_ssl_cipher = 21;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_EXPORT_WITH_RC4_40_MD5: pj_ssl_cipher = 23;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA: pj_ssl_cipher = 25;
pub const pj_ssl_cipher_PJ_TLS_DH_anon_WITH_DES_CBC_SHA: pj_ssl_cipher = 26;
pub const pj_ssl_cipher_PJ_SSL_FORTEZZA_KEA_WITH_NULL_SHA: pj_ssl_cipher = 28;
pub const pj_ssl_cipher_PJ_SSL_FORTEZZA_KEA_WITH_FORTEZZA_CBC_SHA: pj_ssl_cipher = 29;
pub const pj_ssl_cipher_PJ_SSL_FORTEZZA_KEA_WITH_RC4_128_SHA: pj_ssl_cipher = 30;
pub const pj_ssl_cipher_PJ_SSL_CK_RC4_128_WITH_MD5: pj_ssl_cipher = 65664;
pub const pj_ssl_cipher_PJ_SSL_CK_RC4_128_EXPORT40_WITH_MD5: pj_ssl_cipher = 131200;
pub const pj_ssl_cipher_PJ_SSL_CK_RC2_128_CBC_WITH_MD5: pj_ssl_cipher = 196736;
pub const pj_ssl_cipher_PJ_SSL_CK_RC2_128_CBC_EXPORT40_WITH_MD5: pj_ssl_cipher = 262272;
pub const pj_ssl_cipher_PJ_SSL_CK_IDEA_128_CBC_WITH_MD5: pj_ssl_cipher = 327808;
pub const pj_ssl_cipher_PJ_SSL_CK_DES_64_CBC_WITH_MD5: pj_ssl_cipher = 393280;
pub const pj_ssl_cipher_PJ_SSL_CK_DES_192_EDE3_CBC_WITH_MD5: pj_ssl_cipher = 458944;
pub type pj_ssl_cipher = ::std::os::raw::c_int;

pub const pj_ssl_curve_PJ_TLS_UNKNOWN_CURVE: pj_ssl_curve = 0;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT163K1: pj_ssl_curve = 1;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT163R1: pj_ssl_curve = 2;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT163R2: pj_ssl_curve = 3;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT193R1: pj_ssl_curve = 4;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT193R2: pj_ssl_curve = 5;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT233K1: pj_ssl_curve = 6;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT233R1: pj_ssl_curve = 7;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT239K1: pj_ssl_curve = 8;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT283K1: pj_ssl_curve = 9;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT283R1: pj_ssl_curve = 10;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT409K1: pj_ssl_curve = 11;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT409R1: pj_ssl_curve = 12;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT571K1: pj_ssl_curve = 13;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECT571R1: pj_ssl_curve = 14;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP160K1: pj_ssl_curve = 15;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP160R1: pj_ssl_curve = 16;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP160R2: pj_ssl_curve = 17;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP192K1: pj_ssl_curve = 18;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP192R1: pj_ssl_curve = 19;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP224K1: pj_ssl_curve = 20;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP224R1: pj_ssl_curve = 21;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP256K1: pj_ssl_curve = 22;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP256R1: pj_ssl_curve = 23;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP384R1: pj_ssl_curve = 24;
pub const pj_ssl_curve_PJ_TLS_CURVE_SECP521R1: pj_ssl_curve = 25;
pub const pj_ssl_curve_PJ_TLS_CURVE_BRAINPOOLP256R1: pj_ssl_curve = 26;
pub const pj_ssl_curve_PJ_TLS_CURVE_BRAINPOOLP384R1: pj_ssl_curve = 27;
pub const pj_ssl_curve_PJ_TLS_CURVE_BRAINPOOLP512R1: pj_ssl_curve = 28;
pub const pj_ssl_curve_PJ_TLS_CURVE_ARBITRARY_EXPLICIT_PRIME_CURVES: pj_ssl_curve = 65281;
pub const pj_ssl_curve_PJ_TLS_CURVE_ARBITRARY_EXPLICIT_CHAR2_CURVES: pj_ssl_curve = 65282;
pub type pj_ssl_curve = ::std::os::raw::c_uint;

pub const pj_ssl_entropy_PJ_SSL_ENTROPY_NONE: pj_ssl_entropy = 0;
pub const pj_ssl_entropy_PJ_SSL_ENTROPY_EGD: pj_ssl_entropy = 1;
pub const pj_ssl_entropy_PJ_SSL_ENTROPY_RANDOM: pj_ssl_entropy = 2;
pub const pj_ssl_entropy_PJ_SSL_ENTROPY_URANDOM: pj_ssl_entropy = 3;
pub const pj_ssl_entropy_PJ_SSL_ENTROPY_FILE: pj_ssl_entropy = 4;
pub const pj_ssl_entropy_PJ_SSL_ENTROPY_UNKNOWN: pj_ssl_entropy = 15;
pub type pj_ssl_entropy = ::std::os::raw::c_uint;
pub use self::pj_ssl_entropy as pj_ssl_entropy_t;

pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_DEFAULT: pj_ssl_sock_proto = 0;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_SSL2: pj_ssl_sock_proto = 1;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_SSL3: pj_ssl_sock_proto = 2;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_TLS1: pj_ssl_sock_proto = 4;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_TLS1_1: pj_ssl_sock_proto = 8;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_TLS1_2: pj_ssl_sock_proto = 16;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_TLS1_3: pj_ssl_sock_proto = 32;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_SSL23: pj_ssl_sock_proto = 65535;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_ALL: pj_ssl_sock_proto = 65535;
pub const pj_ssl_sock_proto_PJ_SSL_SOCK_PROTO_DTLS1: pj_ssl_sock_proto = 65536;
pub type pj_ssl_sock_proto = ::std::os::raw::c_uint;

pub const pjsip_ssl_method_PJSIP_SSL_UNSPECIFIED_METHOD: pjsip_ssl_method = 0;
pub const pjsip_ssl_method_PJSIP_SSLV2_METHOD: pjsip_ssl_method = 20;
pub const pjsip_ssl_method_PJSIP_SSLV3_METHOD: pjsip_ssl_method = 30;
pub const pjsip_ssl_method_PJSIP_TLSV1_METHOD: pjsip_ssl_method = 31;
pub const pjsip_ssl_method_PJSIP_TLSV1_1_METHOD: pjsip_ssl_method = 32;
pub const pjsip_ssl_method_PJSIP_TLSV1_2_METHOD: pjsip_ssl_method = 33;
pub const pjsip_ssl_method_PJSIP_TLSV1_3_METHOD: pjsip_ssl_method = 34;
pub const pjsip_ssl_method_PJSIP_SSLV23_METHOD: pjsip_ssl_method = 23;
pub type pjsip_ssl_method = ::std::os::raw::c_uint;



pub type pj_exit_callback = ::std::option::Option<unsafe extern "C" fn()>;
pub type pj_error_callback = ::std::option::Option<
    unsafe extern "C" fn(
        e: pj_status_t,
        msg: *mut ::std::os::raw::c_char,
        max: pj_size_t,
    ) -> pj_str_t,
>;




#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
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
    pub __val: [::std::os::raw::c_ulong; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_str_t {
    pub ptr: *mut ::std::os::raw::c_char,
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
pub type pj_atomic_value_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_thread_t {
    _unused: [u8; 0],
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
    pub sec: ::std::os::raw::c_long,
    pub msec: ::std::os::raw::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_parsed_time {
    pub wday: ::std::os::raw::c_int,
    pub day: ::std::os::raw::c_int,
    pub mon: ::std::os::raw::c_int,
    pub year: ::std::os::raw::c_int,
    pub sec: ::std::os::raw::c_int,
    pub min: ::std::os::raw::c_int,
    pub hour: ::std::os::raw::c_int,
    pub msec: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_list {
    pub prev: *mut ::std::os::raw::c_void,
    pub next: *mut ::std::os::raw::c_void,
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
    pub cis_id: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_scanner {
    pub begin: *mut ::std::os::raw::c_char,
    pub end: *mut ::std::os::raw::c_char,
    pub curptr: *mut ::std::os::raw::c_char,
    pub line: ::std::os::raw::c_int,
    pub start_line: *mut ::std::os::raw::c_char,
    pub skip_ws: ::std::os::raw::c_int,
    pub callback: pj_syn_err_func_ptr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_scan_state {
    pub curptr: *mut ::std::os::raw::c_char,
    pub line: ::std::os::raw::c_int,
    pub start_line: *mut ::std::os::raw::c_char,
}


pub type pj_timer_id_t = ::std::os::raw::c_int;
pub type pj_timer_heap_callback = ::std::option::Option<
    unsafe extern "C" fn(timer_heap: *mut pj_timer_heap_t, entry: *mut pj_timer_entry),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_grp_lock_config {
    pub flags: ::std::os::raw::c_uint,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_timer_entry {
    pub user_data: *mut ::std::os::raw::c_void,
    pub id: ::std::os::raw::c_int,
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
    pub sin_zero_pad: [::std::os::raw::c_char; 8usize],
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
    pub cnt: ::std::os::raw::c_uint,
    pub options: [pj_sockopt_params__bindgen_ty_1; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_sockopt_params__bindgen_ty_1 {
    pub level: ::std::os::raw::c_int,
    pub optname: ::std::os::raw::c_int,
    pub optval: *mut ::std::os::raw::c_void,
    pub optlen: ::std::os::raw::c_int,
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
    pub data: *mut ::std::os::raw::c_void,
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
pub type pj_dns_callback = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        status: pj_status_t,
        response: *mut pj_dns_parsed_packet,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_dns_settings {
    pub options: ::std::os::raw::c_uint,
    pub qretr_delay: ::std::os::raw::c_uint,
    pub qretr_count: ::std::os::raw::c_uint,
    pub cache_max_ttl: ::std::os::raw::c_uint,
    pub good_ns_ttl: ::std::os::raw::c_uint,
    pub bad_ns_ttl: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_a_record {
    pub name: pj_str_t,
    pub alias: pj_str_t,
    pub addr_count: ::std::os::raw::c_uint,
    pub addr: [pj_in_addr; 8usize],
    pub buf_: [::std::os::raw::c_char; 128usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_addr_record {
    pub name: pj_str_t,
    pub alias: pj_str_t,
    pub addr_count: ::std::os::raw::c_uint,
    pub addr: [pj_dns_addr_record__bindgen_ty_1; 8usize],
    pub buf_: [::std::os::raw::c_char; 128usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_dns_addr_record__bindgen_ty_1 {
    pub af: ::std::os::raw::c_int,
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
    pub internal__: [*mut ::std::os::raw::c_void; 32usize],
    pub activesock_data: *mut ::std::os::raw::c_void,
    pub user_data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ioqueue_callback {
    pub on_read_complete: ::std::option::Option<
        unsafe extern "C" fn(
            key: *mut pj_ioqueue_key_t,
            op_key: *mut pj_ioqueue_op_key_t,
            bytes_read: pj_ssize_t,
        ),
    >,
    pub on_write_complete: ::std::option::Option<
        unsafe extern "C" fn(
            key: *mut pj_ioqueue_key_t,
            op_key: *mut pj_ioqueue_op_key_t,
            bytes_sent: pj_ssize_t,
        ),
    >,
    pub on_accept_complete: ::std::option::Option<
        unsafe extern "C" fn(
            key: *mut pj_ioqueue_key_t,
            op_key: *mut pj_ioqueue_op_key_t,
            sock: pj_sock_t,
            status: pj_status_t,
        ),
    >,
    pub on_connect_complete: ::std::option::Option<
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
    ::std::option::Option<unsafe extern "C" fn(pool: *mut pj_pool_t, size: pj_size_t)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_block {
    pub prev: *mut pj_pool_block,
    pub next: *mut pj_pool_block,
    pub buf: *mut ::std::os::raw::c_uchar,
    pub cur: *mut ::std::os::raw::c_uchar,
    pub end: *mut ::std::os::raw::c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_t {
    pub prev: *mut pj_pool_t,
    pub next: *mut pj_pool_t,
    pub obj_name: [::std::os::raw::c_char; 32usize],
    pub factory: *mut pj_pool_factory,
    pub factory_data: *mut ::std::os::raw::c_void,
    pub capacity: pj_size_t,
    pub increment_size: pj_size_t,
    pub block_list: pj_pool_block,
    pub callback: pj_pool_callback,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_factory_policy {
    pub block_alloc: ::std::option::Option<
        unsafe extern "C" fn(
            factory: *mut pj_pool_factory,
            size: pj_size_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub block_free: ::std::option::Option<
        unsafe extern "C" fn(
            factory: *mut pj_pool_factory,
            mem: *mut ::std::os::raw::c_void,
            size: pj_size_t,
        ),
    >,
    pub callback: pj_pool_callback,
    pub flags: ::std::os::raw::c_uint,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_pool_factory {
    pub policy: pj_pool_factory_policy,
    pub create_pool: ::std::option::Option<
        unsafe extern "C" fn(
            factory: *mut pj_pool_factory,
            name: *const ::std::os::raw::c_char,
            initial_size: pj_size_t,
            increment_size: pj_size_t,
            callback: pj_pool_callback,
        ) -> *mut pj_pool_t,
    >,
    pub release_pool: ::std::option::Option<
        unsafe extern "C" fn(factory: *mut pj_pool_factory, pool: *mut pj_pool_t),
    >,
    pub dump_status: ::std::option::Option<
        unsafe extern "C" fn(factory: *mut pj_pool_factory, detail: pj_bool_t),
    >,
    pub on_block_alloc: ::std::option::Option<
        unsafe extern "C" fn(factory: *mut pj_pool_factory, size: pj_size_t) -> pj_bool_t,
    >,
    pub on_block_free:
        ::std::option::Option<unsafe extern "C" fn(factory: *mut pj_pool_factory, size: pj_size_t)>,
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
    pub pool_buf: [::std::os::raw::c_char; 512usize],
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
    pub version: ::std::os::raw::c_uint,
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
    pub cnt: ::std::os::raw::c_uint,
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
    pub cnt: ::std::os::raw::c_uint,
    pub cert_raw: *mut pj_str_t,
}

pub type pj_ssl_cert_buffer = pj_str_t;




#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_sock_cb {
    pub on_data_read: ::std::option::Option<
        unsafe extern "C" fn(
            ssock: *mut pj_ssl_sock_t,
            data: *mut ::std::os::raw::c_void,
            size: pj_size_t,
            status: pj_status_t,
            remainder: *mut pj_size_t,
        ) -> pj_bool_t,
    >,
    pub on_data_recvfrom: ::std::option::Option<
        unsafe extern "C" fn(
            ssock: *mut pj_ssl_sock_t,
            data: *mut ::std::os::raw::c_void,
            size: pj_size_t,
            src_addr: *const pj_sockaddr_t,
            addr_len: ::std::os::raw::c_int,
            status: pj_status_t,
        ) -> pj_bool_t,
    >,
    pub on_data_sent: ::std::option::Option<
        unsafe extern "C" fn(
            ssock: *mut pj_ssl_sock_t,
            send_key: *mut pj_ioqueue_op_key_t,
            sent: pj_ssize_t,
        ) -> pj_bool_t,
    >,
    pub on_accept_complete: ::std::option::Option<
        unsafe extern "C" fn(
            ssock: *mut pj_ssl_sock_t,
            newsock: *mut pj_ssl_sock_t,
            src_addr: *const pj_sockaddr_t,
            src_addr_len: ::std::os::raw::c_int,
        ) -> pj_bool_t,
    >,
    pub on_accept_complete2: ::std::option::Option<
        unsafe extern "C" fn(
            ssock: *mut pj_ssl_sock_t,
            newsock: *mut pj_ssl_sock_t,
            src_addr: *const pj_sockaddr_t,
            src_addr_len: ::std::os::raw::c_int,
            status: pj_status_t,
        ) -> pj_bool_t,
    >,
    pub on_connect_complete: ::std::option::Option<
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
    pub last_native_err: ::std::os::raw::c_ulong,
    pub grp_lock: *mut pj_grp_lock_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_ssl_sock_param {
    pub grp_lock: *mut pj_grp_lock_t,
    pub sock_af: ::std::os::raw::c_int,
    pub sock_type: ::std::os::raw::c_int,
    pub ioqueue: *mut pj_ioqueue_t,
    pub timer_heap: *mut pj_timer_heap_t,
    pub cb: pj_ssl_sock_cb,
    pub user_data: *mut ::std::os::raw::c_void,
    pub proto: pj_uint32_t,
    pub async_cnt: ::std::os::raw::c_uint,
    pub concurrency: ::std::os::raw::c_int,
    pub whole_data: pj_bool_t,
    pub send_buffer_size: pj_size_t,
    pub read_buffer_size: pj_size_t,
    pub ciphers_num: ::std::os::raw::c_uint,
    pub ciphers: *mut pj_ssl_cipher,
    pub curves_num: ::std::os::raw::c_uint,
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
    pub addr_len: ::std::os::raw::c_int,
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
    pub ciphers_num: ::std::os::raw::c_uint,
    pub ciphers: *mut pj_ssl_cipher,
    pub curves_num: ::std::os::raw::c_uint,
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
        ::std::option::Option<unsafe extern "C" fn(param: *const pjsip_tls_on_accept_fail_param)>,
}

extern "C" {
    pub static mut PJ_VERSION: *const ::std::os::raw::c_char;
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
    pub static PJ_MSG_OOB: ::std::os::raw::c_int;
    pub static PJ_MSG_PEEK: ::std::os::raw::c_int;
    pub static PJ_MSG_DONTROUTE: ::std::os::raw::c_int;

    pub fn pj_get_version() -> *const ::std::os::raw::c_char;
    pub fn pj_dump_config();
    pub fn pj_init() -> pj_status_t;
    pub fn pj_shutdown();
    pub fn pj_atexit(func: pj_exit_callback) -> pj_status_t;
    pub fn pj_get_os_error() -> pj_status_t;
    pub fn pj_set_os_error(code: pj_status_t);
    pub fn pj_get_netos_error() -> pj_status_t;
    pub fn pj_set_netos_error(code: pj_status_t);
    pub fn pj_strerror(statcode: pj_status_t,buf: *mut ::std::os::raw::c_char,bufsize: pj_size_t,) -> pj_str_t;
    pub fn pj_perror( log_level: ::std::os::raw::c_int, sender: *const ::std::os::raw::c_char, status: pj_status_t, title_fmt: *const ::std::os::raw::c_char, ... );
    pub fn pj_register_strerror(start_code: pj_status_t,err_space: pj_status_t,f: pj_error_callback, ) -> pj_status_t;
    pub fn pj_errno_clear_handlers();
    pub fn pj_perror_1( sender: *const ::std::os::raw::c_char, status: pj_status_t, title_fmt: *const ::std::os::raw::c_char, ... );
    pub fn pj_perror_2( sender: *const ::std::os::raw::c_char, status: pj_status_t, title_fmt: *const ::std::os::raw::c_char, ... );
    pub fn pj_perror_3( sender: *const ::std::os::raw::c_char, status: pj_status_t, title_fmt: *const ::std::os::raw::c_char, ... );
    pub fn pj_perror_4( sender: *const ::std::os::raw::c_char, status: pj_status_t, title_fmt: *const ::std::os::raw::c_char, ... );
    pub fn pj_perror_5( sender: *const ::std::os::raw::c_char, status: pj_status_t, title_fmt: *const ::std::os::raw::c_char, ...);
    pub fn pjsip_strerror( status: pj_status_t, buffer: *mut ::std::os::raw::c_char, bufsize: pj_size_t, ) -> pj_str_t;
    pub fn pj_time_val_normalize(t: *mut pj_time_val);
    pub fn pj_list_insert_before(pos: *mut pj_list_type, node: *mut pj_list_type);
    pub fn pj_list_insert_nodes_before(lst: *mut pj_list_type, nodes: *mut pj_list_type);
    pub fn pj_list_insert_after(pos: *mut pj_list_type, node: *mut pj_list_type);
    pub fn pj_list_insert_nodes_after(lst: *mut pj_list_type, nodes: *mut pj_list_type);
    pub fn pj_list_merge_first(list1: *mut pj_list_type, list2: *mut pj_list_type);
    pub fn pj_list_merge_last(list1: *mut pj_list_type, list2: *mut pj_list_type);
    pub fn pj_list_erase(node: *mut pj_list_type);
    pub fn pj_list_find_node(list: *mut pj_list_type, node: *mut pj_list_type) -> *mut pj_list_type;
    pub fn pj_list_search( list: *mut pj_list_type, value: *mut ::std::os::raw::c_void, comp: ::std::option::Option< unsafe extern "C" fn( value: *mut ::std::os::raw::c_void, node: *const pj_list_type, ) -> ::std::os::raw::c_int, >, ) -> *mut pj_list_type;
    pub fn pj_list_size(list: *const pj_list_type) -> pj_size_t;
    pub fn pjlib_util_init() -> pj_status_t;
    pub fn pj_cis_buf_init(cs_buf: *mut pj_cis_buf_t);
    pub fn pj_cis_init(cs_buf: *mut pj_cis_buf_t, cis: *mut pj_cis_t) -> pj_status_t;
    pub fn pj_cis_dup(new_cis: *mut pj_cis_t, existing: *mut pj_cis_t) -> pj_status_t;
    pub fn pj_cis_add_range( cis: *mut pj_cis_t, cstart: ::std::os::raw::c_int, cend: ::std::os::raw::c_int, );
    pub fn pj_cis_add_alpha(cis: *mut pj_cis_t);
    pub fn pj_cis_add_num(cis: *mut pj_cis_t);
    pub fn pj_cis_add_str(cis: *mut pj_cis_t, str_: *const ::std::os::raw::c_char);
    pub fn pj_cis_add_cis(cis: *mut pj_cis_t, rhs: *const pj_cis_t);
    pub fn pj_cis_del_range( cis: *mut pj_cis_t, cstart: ::std::os::raw::c_int, cend: ::std::os::raw::c_int, );
    pub fn pj_cis_del_str(cis: *mut pj_cis_t, str_: *const ::std::os::raw::c_char);
    pub fn pj_cis_invert(cis: *mut pj_cis_t);
    pub fn pj_scan_init( scanner: *mut pj_scanner, bufstart: *mut ::std::os::raw::c_char, buflen: pj_size_t, options: ::std::os::raw::c_uint, callback: pj_syn_err_func_ptr, );
    pub fn pj_scan_fini(scanner: *mut pj_scanner);
    pub fn pj_scan_peek( scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t, ) -> ::std::os::raw::c_int;
    pub fn pj_scan_peek_n( scanner: *mut pj_scanner, len: pj_size_t, out: *mut pj_str_t,) -> ::std::os::raw::c_int;
    pub fn pj_scan_peek_until( scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t, ) -> ::std::os::raw::c_int;
    pub fn pj_scan_get(scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t);
    pub fn pj_scan_get_unescape( scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t, );
    pub fn pj_scan_get_quote( scanner: *mut pj_scanner, begin_quote: ::std::os::raw::c_int, end_quote: ::std::os::raw::c_int, out: *mut pj_str_t,);
    pub fn pj_scan_get_quotes( scanner: *mut pj_scanner, begin_quotes: *const ::std::os::raw::c_char, end_quotes: *const ::std::os::raw::c_char, qsize: ::std::os::raw::c_int, out: *mut pj_str_t, );
    pub fn pj_scan_get_n(scanner: *mut pj_scanner, N: ::std::os::raw::c_uint, out: *mut pj_str_t);
    pub fn pj_scan_get_char(scanner: *mut pj_scanner) -> ::std::os::raw::c_int;
    pub fn pj_scan_get_until(scanner: *mut pj_scanner, spec: *const pj_cis_t, out: *mut pj_str_t);
    pub fn pj_scan_get_until_ch( scanner: *mut pj_scanner, until_char: ::std::os::raw::c_int, out: *mut pj_str_t, );
    pub fn pj_scan_get_until_chr( scanner: *mut pj_scanner, until_spec: *const ::std::os::raw::c_char, out: *mut pj_str_t, );
    pub fn pj_scan_advance_n(scanner: *mut pj_scanner, N: ::std::os::raw::c_uint, skip: pj_bool_t);
    pub fn pj_scan_strcmp( scanner: *mut pj_scanner, s: *const ::std::os::raw::c_char, len: ::std::os::raw::c_int, ) -> ::std::os::raw::c_int;
    pub fn pj_scan_stricmp( scanner: *mut pj_scanner, s: *const ::std::os::raw::c_char, len: ::std::os::raw::c_int,) -> ::std::os::raw::c_int;
    pub fn pj_scan_stricmp_alnum( scanner: *mut pj_scanner, s: *const ::std::os::raw::c_char, len: ::std::os::raw::c_int,) -> ::std::os::raw::c_int;
    pub fn pj_scan_get_newline(scanner: *mut pj_scanner);
    pub fn pj_scan_skip_whitespace(scanner: *mut pj_scanner);
    pub fn pj_scan_skip_line(scanner: *mut pj_scanner);
    pub fn pj_scan_save_state(scanner: *const pj_scanner, state: *mut pj_scan_state);
    pub fn pj_scan_restore_state(scanner: *mut pj_scanner, state: *mut pj_scan_state);
    pub fn pj_lock_create_simple_mutex( pool: *mut pj_pool_t, name: *const ::std::os::raw::c_char, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
    pub fn pj_lock_create_recursive_mutex( pool: *mut pj_pool_t, name: *const ::std::os::raw::c_char, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
    pub fn pj_lock_create_null_mutex( pool: *mut pj_pool_t, name: *const ::std::os::raw::c_char, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
    pub fn pj_lock_create_semaphore( pool: *mut pj_pool_t, name: *const ::std::os::raw::c_char, initial: ::std::os::raw::c_uint, max: ::std::os::raw::c_uint, lock: *mut *mut pj_lock_t, ) -> pj_status_t;
    pub fn pj_lock_acquire(lock: *mut pj_lock_t) -> pj_status_t;
    pub fn pj_lock_tryacquire(lock: *mut pj_lock_t) -> pj_status_t;
    pub fn pj_lock_release(lock: *mut pj_lock_t) -> pj_status_t;
    pub fn pj_lock_destroy(lock: *mut pj_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_config_default(cfg: *mut pj_grp_lock_config);
    pub fn pj_grp_lock_create( pool: *mut pj_pool_t, cfg: *const pj_grp_lock_config, p_grp_lock: *mut *mut pj_grp_lock_t, ) -> pj_status_t;
    pub fn pj_grp_lock_create_w_handler( pool: *mut pj_pool_t, cfg: *const pj_grp_lock_config, member: *mut ::std::os::raw::c_void, handler: ::std::option::Option<unsafe extern "C" fn(member: *mut ::std::os::raw::c_void)>, p_grp_lock: *mut *mut pj_grp_lock_t, ) -> pj_status_t;
    pub fn pj_grp_lock_destroy(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_replace( old_lock: *mut pj_grp_lock_t, new_lock: *mut pj_grp_lock_t, ) -> pj_status_t;
    pub fn pj_grp_lock_acquire(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_tryacquire(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_release(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_add_handler( grp_lock: *mut pj_grp_lock_t, pool: *mut pj_pool_t, member: *mut ::std::os::raw::c_void, handler: ::std::option::Option<unsafe extern "C" fn(member: *mut ::std::os::raw::c_void)>, ) -> pj_status_t;
    pub fn pj_grp_lock_del_handler( grp_lock: *mut pj_grp_lock_t, member: *mut ::std::os::raw::c_void, handler: ::std::option::Option<unsafe extern "C" fn(member: *mut ::std::os::raw::c_void)>, ) -> pj_status_t;
    pub fn pj_grp_lock_add_ref(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_dec_ref(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
    pub fn pj_grp_lock_get_ref(grp_lock: *mut pj_grp_lock_t) -> ::std::os::raw::c_int;
    pub fn pj_grp_lock_dump(grp_lock: *mut pj_grp_lock_t);
    pub fn pj_grp_lock_chain_lock( grp_lock: *mut pj_grp_lock_t, ext_lock: *mut pj_lock_t, pos: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_grp_lock_unchain_lock( grp_lock: *mut pj_grp_lock_t, ext_lock: *mut pj_lock_t, ) -> pj_status_t;
    pub fn pj_timer_heap_mem_size(count: pj_size_t) -> pj_size_t;
    pub fn pj_timer_heap_create( pool: *mut pj_pool_t, count: pj_size_t, ht: *mut *mut pj_timer_heap_t, ) -> pj_status_t;
    pub fn pj_timer_heap_destroy(ht: *mut pj_timer_heap_t);
    pub fn pj_timer_heap_set_lock( ht: *mut pj_timer_heap_t, lock: *mut pj_lock_t, auto_del: pj_bool_t, );
    pub fn pj_timer_heap_set_max_timed_out_per_poll( ht: *mut pj_timer_heap_t, count: ::std::os::raw::c_uint, ) -> ::std::os::raw::c_uint;
    pub fn pj_timer_entry_init( entry: *mut pj_timer_entry, id: ::std::os::raw::c_int, user_data: *mut ::std::os::raw::c_void, cb: pj_timer_heap_callback, ) -> *mut pj_timer_entry;
    pub fn pj_timer_entry_running(entry: *mut pj_timer_entry) -> pj_bool_t;
    pub fn pj_timer_heap_schedule_dbg( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, delay: *const pj_time_val, src_file: *const ::std::os::raw::c_char, src_line: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_timer_heap_schedule_w_grp_lock_dbg( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, delay: *const pj_time_val, id_val: ::std::os::raw::c_int, grp_lock: *mut pj_grp_lock_t, src_file: *const ::std::os::raw::c_char, src_line: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_timer_heap_cancel( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, ) -> ::std::os::raw::c_int;
    pub fn pj_timer_heap_cancel_if_active( ht: *mut pj_timer_heap_t, entry: *mut pj_timer_entry, id_val: ::std::os::raw::c_int, ) -> ::std::os::raw::c_int;
    pub fn pj_timer_heap_count(ht: *mut pj_timer_heap_t) -> pj_size_t;
    pub fn pj_timer_heap_earliest_time( ht: *mut pj_timer_heap_t, timeval: *mut pj_time_val, ) -> pj_status_t;
    pub fn pj_timer_heap_poll( ht: *mut pj_timer_heap_t, next_delay: *mut pj_time_val, ) -> ::std::os::raw::c_uint;
    pub fn pj_timer_heap_dump(ht: *mut pj_timer_heap_t);
    pub fn pj_ntohs(netshort: pj_uint16_t) -> pj_uint16_t;
    pub fn pj_htons(hostshort: pj_uint16_t) -> pj_uint16_t;
    pub fn pj_ntohl(netlong: pj_uint32_t) -> pj_uint32_t;
    pub fn pj_htonl(hostlong: pj_uint32_t) -> pj_uint32_t;
    pub fn pj_inet_ntoa(inaddr: pj_in_addr) -> *mut ::std::os::raw::c_char;
    pub fn pj_inet_aton(cp: *const pj_str_t, inp: *mut pj_in_addr) -> ::std::os::raw::c_int;
    pub fn pj_inet_pton( af: ::std::os::raw::c_int, src: *const pj_str_t, dst: *mut ::std::os::raw::c_void,) -> pj_status_t;
    pub fn pj_inet_ntop( af: ::std::os::raw::c_int, src: *const ::std::os::raw::c_void, dst: *mut ::std::os::raw::c_char, size: ::std::os::raw::c_int,) -> pj_status_t;
    pub fn pj_inet_ntop2( af: ::std::os::raw::c_int, src: *const ::std::os::raw::c_void, dst: *mut ::std::os::raw::c_char, size: ::std::os::raw::c_int, ) -> *mut ::std::os::raw::c_char;
    pub fn pj_sockaddr_print( addr: *const pj_sockaddr_t, buf: *mut ::std::os::raw::c_char, size: ::std::os::raw::c_int, flags: ::std::os::raw::c_uint, )-> *mut ::std::os::raw::c_char;
    pub fn pj_inet_addr(cp: *const pj_str_t) -> pj_in_addr;
    pub fn pj_inet_addr2(cp: *const ::std::os::raw::c_char) -> pj_in_addr;
    pub fn pj_sockaddr_in_init( addr: *mut pj_sockaddr_in, cp: *const pj_str_t, port: pj_uint16_t, ) -> pj_status_t;
    pub fn pj_sockaddr_init( af: ::std::os::raw::c_int, addr: *mut pj_sockaddr, cp: *const pj_str_t, port: pj_uint16_t,) -> pj_status_t;
    pub fn pj_sockaddr_cmp( addr1: *const pj_sockaddr_t, addr2: *const pj_sockaddr_t, ) -> ::std::os::raw::c_int;
    pub fn pj_sockaddr_get_addr(addr: *const pj_sockaddr_t) -> *mut ::std::os::raw::c_void;
    pub fn pj_sockaddr_has_addr(addr: *const pj_sockaddr_t) -> pj_bool_t;
    pub fn pj_sockaddr_get_addr_len(addr: *const pj_sockaddr_t) -> ::std::os::raw::c_uint;
    pub fn pj_sockaddr_get_len(addr: *const pj_sockaddr_t) -> ::std::os::raw::c_uint;
    pub fn pj_sockaddr_copy_addr(dst: *mut pj_sockaddr, src: *const pj_sockaddr);
    pub fn pj_sockaddr_cp(dst: *mut pj_sockaddr_t, src: *const pj_sockaddr_t);
    pub fn pj_sockaddr_synthesize( dst_af: ::std::os::raw::c_int, dst: *mut pj_sockaddr_t, src: *const pj_sockaddr_t, ) -> pj_status_t;
    pub fn pj_sockaddr_in_get_addr(addr: *const pj_sockaddr_in) -> pj_in_addr;
    pub fn pj_sockaddr_in_set_addr(addr: *mut pj_sockaddr_in, hostaddr: pj_uint32_t);
    pub fn pj_sockaddr_in_set_str_addr(addr: *mut pj_sockaddr_in,cp: *const pj_str_t,) -> pj_status_t;
    pub fn pj_sockaddr_set_str_addr( af: ::std::os::raw::c_int, addr: *mut pj_sockaddr, cp: *const pj_str_t, ) -> pj_status_t;
    pub fn pj_sockaddr_get_port(addr: *const pj_sockaddr_t) -> pj_uint16_t;
    pub fn pj_sockaddr_in_get_port(addr: *const pj_sockaddr_in) -> pj_uint16_t;
    pub fn pj_sockaddr_set_port(addr: *mut pj_sockaddr, hostport: pj_uint16_t) -> pj_status_t;
    pub fn pj_sockaddr_in_set_port(addr: *mut pj_sockaddr_in, hostport: pj_uint16_t);
    pub fn pj_sockaddr_parse( af: ::std::os::raw::c_int, options: ::std::os::raw::c_uint, str_: *const pj_str_t, addr: *mut pj_sockaddr,) -> pj_status_t;
    pub fn pj_sockaddr_parse2( af: ::std::os::raw::c_int, options: ::std::os::raw::c_uint, str_: *const pj_str_t, hostpart: *mut pj_str_t, port: *mut pj_uint16_t, raf: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_gethostname() -> *const pj_str_t;
    pub fn pj_gethostaddr() -> pj_in_addr;
    pub fn pj_sock_socket( family: ::std::os::raw::c_int, type_: ::std::os::raw::c_int, protocol: ::std::os::raw::c_int, sock: *mut pj_sock_t, ) -> pj_status_t;
    pub fn pj_sock_close(sockfd: pj_sock_t) -> pj_status_t;
    pub fn pj_sock_bind( sockfd: pj_sock_t, my_addr: *const pj_sockaddr_t, addrlen: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_bind_in(sockfd: pj_sock_t, addr: pj_uint32_t, port: pj_uint16_t) -> pj_status_t;
    pub fn pj_sock_bind_random( sockfd: pj_sock_t, addr: *const pj_sockaddr_t, port_range: pj_uint16_t, max_try: pj_uint16_t,) -> pj_status_t;
    pub fn pj_sock_listen(sockfd: pj_sock_t, backlog: ::std::os::raw::c_int) -> pj_status_t;
    pub fn pj_sock_accept( serverfd: pj_sock_t, newsock: *mut pj_sock_t, addr: *mut pj_sockaddr_t, addrlen: *mut ::std::os::raw::c_int,) -> pj_status_t;
    pub fn pj_sock_connect( sockfd: pj_sock_t, serv_addr: *const pj_sockaddr_t, addrlen: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_getpeername( sockfd: pj_sock_t, addr: *mut pj_sockaddr_t, namelen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_getsockname( sockfd: pj_sock_t, addr: *mut pj_sockaddr_t, namelen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_getsockopt( sockfd: pj_sock_t, level: pj_uint16_t, optname: pj_uint16_t, optval: *mut ::std::os::raw::c_void, optlen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_setsockopt( sockfd: pj_sock_t, level: pj_uint16_t, optname: pj_uint16_t, optval: *const ::std::os::raw::c_void, optlen: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_setsockopt_params( sockfd: pj_sock_t, params: *const pj_sockopt_params, ) -> pj_status_t;
    pub fn pj_sock_setsockopt_sobuf( sockfd: pj_sock_t, optname: pj_uint16_t, auto_retry: pj_bool_t, buf_size: *mut ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_sock_recv( sockfd: pj_sock_t, buf: *mut ::std::os::raw::c_void, len: *mut pj_ssize_t, flags: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_sock_recvfrom( sockfd: pj_sock_t, buf: *mut ::std::os::raw::c_void, len: *mut pj_ssize_t, flags: ::std::os::raw::c_uint, from: *mut pj_sockaddr_t, fromlen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_send( sockfd: pj_sock_t, buf: *const ::std::os::raw::c_void, len: *mut pj_ssize_t, flags: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_sock_sendto( sockfd: pj_sock_t, buf: *const ::std::os::raw::c_void, len: *mut pj_ssize_t, flags: ::std::os::raw::c_uint, to: *const pj_sockaddr_t, tolen: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_shutdown(sockfd: pj_sock_t, how: ::std::os::raw::c_int) -> pj_status_t;
    pub fn pj_addr_str_print( host_str: *const pj_str_t, port: ::std::os::raw::c_int, buf: *mut ::std::os::raw::c_char, size: ::std::os::raw::c_int, flag: ::std::os::raw::c_u t, ) -> *mut ::std::os::raw::c_char;
    pub fn pj_dns_make_query( packet: *mut ::std::os::raw::c_void, size: *mut ::std::os::raw::c_uint, id: pj_uint16_t, qtype: ::std::os::raw::c_int, name: *const pj_str_t, ) -> pj_status_t;
    pub fn pj_dns_parse_packet( pool: *mut pj_pool_t, packet: *const ::std::os::raw::c_void, size: ::std::os::raw::c_uint, p_res: *mut *mut pj_dns_parsed_packet, ) -> pj_status_t;
    pub fn pj_dns_packet_dup( pool: *mut pj_pool_t, p: *const pj_dns_parsed_packet, options: ::std::os::raw::c_uint, p_dst: *mut *mut pj_dns_parsed_packet, );
    pub fn pj_dns_get_type_name(type_: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn pj_dns_init_srv_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: ::std::os::raw::c_uint, ttl: ::std::os::raw::c_uint, prio: ::std::os::raw::c_uint, weight: ::std::os::raw::c_uint, port: ::std::os::raw::c_uint, target: *const pj_str_t, );
    pub fn pj_dns_init_cname_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: ::std::os::raw::c_uint, ttl: ::std::os::raw::c_uint, name: *const pj_str_t, );
    pub fn pj_dns_init_a_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: ::std::os::raw::c_uint, ttl: ::std::os::raw::c_uint, ip_addr: *const pj_in_addr, );
    pub fn pj_dns_init_aaaa_rr( rec: *mut pj_dns_parsed_rr, res_name: *const pj_str_t, dnsclass: ::std::os::raw::c_uint, ttl: ::std::os::raw::c_uint, ip_addr: *const pj_in6_addr, );
    pub fn pj_dns_dump_packet(res: *const pj_dns_parsed_packet);
    pub fn pj_dns_settings_default(s: *mut pj_dns_settings);
    pub fn pj_dns_resolver_create( pf: *mut pj_pool_factory, name: *const ::std::os::raw::c_char, options: ::std::os::raw::c_uint, timer: *mut pj_timer_heap_t, ioqueue: *mut pj_ioqueue_t, p_resolver: *mut *mut pj_dns_resolver, ) -> pj_status_t;
    pub fn pj_dns_resolver_set_ns( resolver: *mut pj_dns_resolver, count: ::std::os::raw::c_uint, servers: *const pj_str_t, ports: *const pj_uint16_t, ) -> pj_status_t;
    pub fn pj_dns_resolver_get_settings( resolver: *mut pj_dns_resolver, st: *mut pj_dns_settings,) -> pj_status_t;
    pub fn pj_dns_resolver_set_settings( resolver: *mut pj_dns_resolver, st: *const pj_dns_settings, ) -> pj_status_t;
    pub fn pj_dns_resolver_handle_events( resolver: *mut pj_dns_resolver, timeout: *const pj_time_val, );
    pub fn pj_dns_resolver_destroy( resolver: *mut pj_dns_resolver, notify: pj_bool_t, ) -> pj_status_t;
    pub fn pj_dns_resolver_start_query( resolver: *mut pj_dns_resolver, name: *const pj_str_t, type_: ::std::os::raw::c_int, options: ::std::os::raw::c_uint, cb: pj_dns_callback, user_data: *mut ::std::os::raw::c_void, p_query: *mut *mut pj_dns_async_query, ) -> pj_status_t;
    pub fn pj_dns_resolver_cancel_query( query: *mut pj_dns_async_query, notify: pj_bool_t, ) -> pj_status_t;
    pub fn pj_dns_parse_a_response( pkt: *const pj_dns_parsed_packet, rec: *mut pj_dns_a_record, ) -> pj_status_t;
    pub fn pj_dns_parse_addr_response( pkt: *const pj_dns_parsed_packet, rec: *mut pj_dns_addr_record, ) -> pj_status_t;
    pub fn pj_dns_resolver_add_entry( resolver: *mut pj_dns_resolver, pkt: *const pj_dns_parsed_packet, set_ttl: pj_bool_t, ) -> pj_status_t;
    pub fn pj_dns_resolver_get_cached_count( resolver: *mut pj_dns_resolver, ) -> ::std::os::raw::c_uint;
    pub fn pj_dns_resolver_dump(resolver: *mut pj_dns_resolver, detail: pj_bool_t);
    pub fn pj_ioqueue_name() -> *const ::std::os::raw::c_char;
    pub fn pj_ioqueue_create( pool: *mut pj_pool_t, max_fd: pj_size_t, ioqueue: *mut *mut pj_ioqueue_t, ) -> pj_status_t;
    pub fn pj_ioqueue_destroy(ioque: *mut pj_ioqueue_t) -> pj_status_t;
    pub fn pj_ioqueue_set_lock( ioque: *mut pj_ioqueue_t, lock: *mut pj_lock_t, auto_delete: pj_bool_t, ) -> pj_status_t;
    pub fn pj_ioqueue_set_default_concurrency( ioqueue: *mut pj_ioqueue_t, allow: pj_bool_t, ) -> pj_status_t;
    pub fn pj_ioqueue_register_sock( pool: *mut pj_pool_t, ioque: *mut pj_ioqueue_t, sock: pj_sock_t, user_data: *mut ::std::os::raw::c_void, cb: *const pj_ioqueue_callback, key: *mut *mut pj_ioqueue_key_t, ) -> pj_status_t;
    pub fn pj_ioqueue_register_sock2( pool: *mut pj_pool_t, ioque: *mut pj_ioqueue_t, sock: pj_sock_t, grp_lock: *mut pj_grp_lock_t, user_data: *mut ::std::os::raw::c_void, cb: *const pj_ioqueue_callback, key: *mut *mut pj_ioqueue_key_t, ) -> pj_status_t;
    pub fn pj_ioqueue_unregister(key: *mut pj_ioqueue_key_t) -> pj_status_t;
    pub fn pj_ioqueue_get_user_data(key: *mut pj_ioqueue_key_t) -> *mut ::std::os::raw::c_void;
    pub fn pj_ioqueue_set_user_data( key: *mut pj_ioqueue_key_t, user_data: *mut ::std::os::raw::c_void, old_data: *mut *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pj_ioqueue_set_concurrency(key: *mut pj_ioqueue_key_t, allow: pj_bool_t) -> pj_status_t;
    pub fn pj_ioqueue_lock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
    pub fn pj_ioqueue_trylock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
    pub fn pj_ioqueue_unlock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
    pub fn pj_ioqueue_op_key_init(op_key: *mut pj_ioqueue_op_key_t, size: pj_size_t);
    pub fn pj_ioqueue_is_pending( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, ) -> pj_bool_t;
    pub fn pj_ioqueue_post_completion( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, bytes_status: pj_ssize_t, ) -> pj_status_t;
    pub fn pj_ioqueue_accept( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, new_sock: *mut pj_sock_t, local: *mut pj_sockaddr_t, remote: *mut pj_sockaddr_t, addrlen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_ioqueue_connect( key: *mut pj_ioqueue_key_t, addr: *const pj_sockaddr_t, addrlen: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_ioqueue_poll( ioque: *mut pj_ioqueue_t, timeout: *const pj_time_val, ) -> ::std::os::raw::c_int;
    pub fn pj_ioqueue_recv( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, buffer: *mut ::std::os::raw::c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, ) -> pj_status_t;
    pub fn pj_ioqueue_recvfrom( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, buffer: *mut ::std::os::raw::c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, addr: *mut pj_sockaddr_t, addrlen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_ioqueue_send( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, data: *const ::std::os::raw::c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, ) -> pj_status_t;
    pub fn pj_ioqueue_sendto( key: *mut pj_ioqueue_key_t, op_key: *mut pj_ioqueue_op_key_t, data: *const ::std::os::raw::c_void, length: *mut pj_ssize_t, flags: pj_uint32_t, addr: *const pj_sockaddr_t, addrlen: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_sock_set_qos_type(sock: pj_sock_t, type_: pj_qos_type) -> pj_status_t;
    pub fn pj_sock_get_qos_type(sock: pj_sock_t, p_type: *mut pj_qos_type) -> pj_status_t;
    pub fn pj_sock_apply_qos( sock: pj_sock_t, qos_type: pj_qos_type, qos_params: *mut pj_qos_params, log_level: ::std::os::raw::c_uint, log_sender: *const ::std::os::raw::c_char, sock_name: *const ::std::os::raw::c_char, ) -> pj_status_t;
    pub fn pj_sock_apply_qos2( sock: pj_sock_t, qos_type: pj_qos_type, qos_params: *const pj_qos_params, log_level: ::std::os::raw::c_uint, log_sender: *const ::std::os::raw::c_char, sock_name: *const ::std::os::raw::c_char, ) -> pj_status_t;
    pub fn pj_qos_get_params(type_: pj_qos_type, p_param: *mut pj_qos_params) -> pj_status_t;
    pub fn pj_qos_get_type(param: *const pj_qos_params, p_type: *mut pj_qos_type) -> pj_status_t;
    pub fn pj_sock_set_qos_params(sock: pj_sock_t, param: *mut pj_qos_params) -> pj_status_t;
    pub fn pj_sock_get_qos_params(sock: pj_sock_t, p_param: *mut pj_qos_params) -> pj_status_t;
    pub static mut PJ_NO_MEMORY_EXCEPTION: ::std::os::raw::c_int;
    pub fn pj_NO_MEMORY_EXCEPTION() -> ::std::os::raw::c_int;
    pub static mut pj_pool_factory_default_policy: pj_pool_factory_policy;
    pub fn pj_pool_factory_get_default_policy() -> *const pj_pool_factory_policy;
    pub fn pj_pool_create( factory: *mut pj_pool_factory, name: *const ::std::os::raw::c_char, initial_size: pj_size_t, increment_size: pj_size_t, callback: pj_pool_callback, ) -> *mut pj_pool_t;
    pub fn pj_pool_release(pool: *mut pj_pool_t);
    pub fn pj_pool_safe_release(ppool: *mut *mut pj_pool_t);
    pub fn pj_pool_secure_release(ppool: *mut *mut pj_pool_t);
    pub fn pj_pool_getobjname(pool: *const pj_pool_t) -> *const ::std::os::raw::c_char;
    pub fn pj_pool_reset(pool: *mut pj_pool_t);
    pub fn pj_pool_get_capacity(pool: *mut pj_pool_t) -> pj_size_t;
    pub fn pj_pool_get_used_size(pool: *mut pj_pool_t) -> pj_size_t;
    pub fn pj_pool_alloc(pool: *mut pj_pool_t, size: pj_size_t) -> *mut ::std::os::raw::c_void;
    pub fn pj_pool_calloc( pool: *mut pj_pool_t, count: pj_size_t, elem: pj_size_t, ) -> *mut ::std::os::raw::c_void;
    pub fn pj_pool_alloc_from_block( block: *mut pj_pool_block, size: pj_size_t,) -> *mut ::std::os::raw::c_void;
    pub fn pj_pool_allocate_find( pool: *mut pj_pool_t, size: pj_size_t, ) -> *mut ::std::os::raw::c_void;
    pub fn pj_pool_create_int( factory: *mut pj_pool_factory, name: *const ::std::os::raw::c_char, initial_size: pj_size_t, increment_size: pj_size_t, callback: pj_pool_callback, ) -> *mut pj_pool_t;
    pub fn pj_pool_init_int( pool: *mut pj_pool_t, name: *const ::std::os::raw::c_char, increment_size: pj_size_t, callback: pj_pool_callback, );
    pub fn pj_pool_destroy_int(pool: *mut pj_pool_t);
    pub fn pj_caching_pool_init( ch_pool: *mut pj_caching_pool, policy: *const pj_pool_factory_policy, max_capacity: pj_size_t, );
    pub fn pj_caching_pool_destroy(ch_pool: *mut pj_caching_pool);
    pub fn pj_ssl_cert_load_from_files( pool: *mut pj_pool_t, CA_file: *const pj_str_t, cert_file: *const pj_str_t, privkey_file: *const pj_str_t, privkey_pass: *const pj_str_t, p_cert: *mut *mut pj_ssl_cert_t, ) -> pj_status_t;
    pub fn pj_ssl_cert_load_from_files2( pool: *mut pj_pool_t, CA_file: *const pj_str_t, CA_path: *const pj_str_t, cert_file: *const pj_str_t, privkey_file: *const pj_str_t, privkey_pass: *const pj_str_t, p_cert: *mut *mut pj_ssl_cert_t, ) -> pj_status_t;
    pub fn pj_ssl_cert_load_from_buffer( pool: *mut pj_pool_t, CA_buf: *const pj_ssl_cert_buffer, cert_buf: *const pj_ssl_cert_buffer, privkey_buf: *const pj_ssl_cert_buffer, privkey_pass: *const pj_str_t, p_cert: *mut *mut pj_ssl_cert_t, ) -> pj_status_t;
    pub fn pj_ssl_cert_info_dump( ci: *const pj_ssl_cert_info, indent: *const ::std::os::raw::c_char, buf: *mut ::std::os::raw::c_char, buf_size: pj_size_t, ) -> pj_ssize_t;
    pub fn pj_ssl_cert_get_verify_status_strings( verify_status: pj_uint32_t, error_strings: *mut *const ::std::os::raw::c_char, count: *mut ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_ssl_cert_wipe_keys(cert: *mut pj_ssl_cert_t);
    pub fn pj_ssl_cipher_get_availables( ciphers: *mut pj_ssl_cipher, cipher_num: *mut ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_ssl_cipher_is_supported(cipher: pj_ssl_cipher) -> pj_bool_t;
    pub fn pj_ssl_cipher_name(cipher: pj_ssl_cipher) -> *const ::std::os::raw::c_char;
    pub fn pj_ssl_cipher_id(cipher_name: *const ::std::os::raw::c_char) -> pj_ssl_cipher;
    pub fn pj_ssl_curve_get_availables( curves: *mut pj_ssl_curve, curve_num: *mut ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_ssl_curve_is_supported(curve: pj_ssl_curve) -> pj_bool_t;
    pub fn pj_ssl_curve_name(curve: pj_ssl_curve) -> *const ::std::os::raw::c_char;
    pub fn pj_ssl_curve_id(curve_name: *const ::std::os::raw::c_char) -> pj_ssl_curve;
    pub fn pj_ssl_sock_param_default(param: *mut pj_ssl_sock_param);
    pub fn pj_ssl_sock_param_copy( pool: *mut pj_pool_t, dst: *mut pj_ssl_sock_param, src: *const pj_ssl_sock_param, );
    pub fn pj_ssl_sock_create( pool: *mut pj_pool_t, param: *const pj_ssl_sock_param, p_ssock: *mut *mut pj_ssl_sock_t, ) -> pj_status_t;
    pub fn pj_ssl_sock_set_certificate( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, cert: *const pj_ssl_cert_t, ) -> pj_status_t;
    pub fn pj_ssl_sock_close(ssock: *mut pj_ssl_sock_t) -> pj_status_t;
    pub fn pj_ssl_sock_set_user_data( ssock: *mut pj_ssl_sock_t, user_data: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pj_ssl_sock_get_user_data(ssock: *mut pj_ssl_sock_t) -> *mut ::std::os::raw::c_void;
    pub fn pj_ssl_sock_get_info( ssock: *mut pj_ssl_sock_t, info: *mut pj_ssl_sock_info,) -> pj_status_t;
    pub fn pj_ssl_sock_start_read( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: ::std::os::raw::c_uint, flags: pj_uint32_t, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_read2( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: ::std::os::raw::c_uint, readbuf: *mut *mut ::std::os::raw::c_void, flags: pj_uint32_t, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_recvfrom( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: ::std::os::raw::c_uint, flags: pj_uint32_t, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_recvfrom2( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, buff_size: ::std::os::raw::c_uint, readbuf: *mut *mut ::std::os::raw::c_void, flags: pj_uint32_t, ) -> pj_status_t;
    pub fn pj_ssl_sock_send( ssock: *mut pj_ssl_sock_t, send_key: *mut pj_ioqueue_op_key_t, data: *const ::std::os::raw::c_void, size: *mut pj_ssize_t, flags: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_ssl_sock_sendto( ssock: *mut pj_ssl_sock_t, send_key: *mut pj_ioqueue_op_key_t, data: *const ::std::os::raw::c_void, size: *mut pj_ssize_t, flags: ::std::os::raw::c_uint, addr: *const pj_sockaddr_t, addr_len: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_accept( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, local_addr: *const pj_sockaddr_t, addr_len: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_accept2( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, local_addr: *const pj_sockaddr_t, addr_len: ::std::os::raw::c_int, newsock_param: *const pj_ssl_sock_param, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_connect( ssock: *mut pj_ssl_sock_t, pool: *mut pj_pool_t, localaddr: *const pj_sockaddr_t, remaddr: *const pj_sockaddr_t, addr_len: ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pj_ssl_sock_start_connect2( ssock: *mut pj_ssl_sock_t, connect_param: *mut pj_ssl_start_connect_param, ) -> pj_status_t;
    pub fn pj_ssl_sock_renegotiate(ssock: *mut pj_ssl_sock_t) -> pj_status_t;
    pub fn pj_str(str_: *mut ::std::os::raw::c_char) -> pj_str_t;
    pub fn pj_strassign(dst: *mut pj_str_t, src: *mut pj_str_t) -> *mut pj_str_t;
    pub fn pj_strcpy(dst: *mut pj_str_t, src: *const pj_str_t) -> *mut pj_str_t;
    pub fn pj_strcpy2(dst: *mut pj_str_t, src: *const ::std::os::raw::c_char) -> *mut pj_str_t;
    pub fn pj_strncpy(dst: *mut pj_str_t, src: *const pj_str_t, max: pj_ssize_t) -> *mut pj_str_t;
    pub fn pj_strncpy_with_null( dst: *mut pj_str_t, src: *const pj_str_t, max: pj_ssize_t, ) -> *mut pj_str_t;
    pub fn pj_strdup( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const pj_str_t, ) -> *mut pj_str_t;
    pub fn pj_strdup_with_null( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const pj_str_t, ) -> *mut pj_str_t;
    pub fn pj_strdup2( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const ::std::os::raw::c_char, ) -> *mut pj_str_t;
    pub fn pj_strdup2_with_null( pool: *mut pj_pool_t, dst: *mut pj_str_t, src: *const ::std::os::raw::c_char, ) -> *mut pj_str_t;
    pub fn pj_strdup3(pool: *mut pj_pool_t, src: *const ::std::os::raw::c_char) -> pj_str_t;
    pub fn pj_strcmp(str1: *const pj_str_t, str2: *const pj_str_t) -> ::std::os::raw::c_int;
    pub fn pj_strcmp2( str1: *const pj_str_t, str2: *const ::std::os::raw::c_char, ) -> ::std::os::raw::c_int;
    pub fn pj_strncmp( str1: *const pj_str_t, str2: *const pj_str_t, len: pj_size_t, ) -> ::std::os::raw::c_int;
    pub fn pj_strncmp2( str1: *const pj_str_t, str2: *const ::std::os::raw::c_char, len: pj_size_t, ) -> ::std::os::raw::c_int;
    pub fn pj_stricmp(str1: *const pj_str_t, str2: *const pj_str_t) -> ::std::os::raw::c_int;
    pub fn pj_stricmp2( str1: *const pj_str_t, str2: *const ::std::os::raw::c_char, ) -> ::std::os::raw::c_int;
    pub fn pj_strnicmp( str1: *const pj_str_t, str2: *const pj_str_t, len: pj_size_t )-> ::std::os::raw::c_int;
    pub fn pj_strnicmp2( str1: *const pj_str_t, str2: *const ::std::os::raw::c_char, len: pj_size_t, ) -> ::std::os::raw::c_int;
    pub fn pj_strcat(dst: *mut pj_str_t, src: *const pj_str_t);
    pub fn pj_strcat2(dst: *mut pj_str_t, src: *const ::std::os::raw::c_char);
    pub fn pj_strspn(str_: *const pj_str_t, set_char: *const pj_str_t) -> pj_ssize_t;
    pub fn pj_strspn2(str_: *const pj_str_t, set_char: *const ::std::os::raw::c_char) -> pj_ssize_t;
    pub fn pj_strcspn(str_: *const pj_str_t, set_char: *const pj_str_t) -> pj_ssize_t;
    pub fn pj_strcspn2( str_: *const pj_str_t, set_char: *const ::std::os::raw::c_char, ) -> pj_ssize_t;
    pub fn pj_strtok( str_: *const pj_str_t, delim: *const pj_str_t, tok: *mut pj_str_t, start_idx: pj_size_t, ) -> pj_ssize_t;
    pub fn pj_strtok2( str_: *const pj_str_t, delim: *const ::std::os::raw::c_char, tok: *mut pj_str_t, start_idx: pj_size_t, ) -> pj_ssize_t;
    pub fn pj_strstr(str_: *const pj_str_t, substr: *const pj_str_t) -> *mut ::std::os::raw::c_char;
    pub fn pj_stristr( str_: *const pj_str_t, substr: *const pj_str_t, ) -> *mut ::std::os::raw::c_char;
    pub fn pj_strltrim(str_: *mut pj_str_t) -> *mut pj_str_t;
    pub fn pj_strrtrim(str_: *mut pj_str_t) -> *mut pj_str_t;
    pub fn pj_strtrim(str_: *mut pj_str_t) -> *mut pj_str_t;
    pub fn pj_create_random_string( str_: *mut ::std::os::raw::c_char, length: pj_size_t, ) -> *mut ::std::os::raw::c_char;
    pub fn pj_strtol(str_: *const pj_str_t) -> ::std::os::raw::c_long;
    pub fn pj_strtol2(str_: *const pj_str_t, value: *mut ::std::os::raw::c_long) -> pj_status_t;
    pub fn pj_strtoul(str_: *const pj_str_t) -> ::std::os::raw::c_ulong;
    pub fn pj_strtoul2( str_: *const pj_str_t, endptr: *mut pj_str_t, base: ::std::os::raw::c_uint, ) -> ::std::os::raw::c_ulong;
    pub fn pj_strtoul3( str_: *const pj_str_t, value: *mut ::std::os::raw::c_ulong, base: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pj_strtof(str_: *const pj_str_t) -> f32;
    pub fn pj_utoa( val: ::std::os::raw::c_ulong, buf: *mut ::std::os::raw::c_char, ) -> ::std::os::raw::c_int;
    pub fn pj_utoa_pad( val: ::std::os::raw::c_ulong, buf: *mut ::std::os::raw::c_char, min_dig: ::std::os::raw::c_int, pad: ::std::os::raw::c_int, ) -> ::std::os::raw::c_int;
}





impl AutoCreate<pj_str_t> for pj_str_t {
    fn new() -> pj_str_t {
        pj_str_t {
            ptr: ptr::null_mut(),
            slen: 0,
        }
    }
}

impl AutoCreate<pj_ice_sess_options> for pj_ice_sess_options {
    fn new() -> pj_ice_sess_options {
        pj_ice_sess_options {
            aggressive: pj_constants__PJ_FALSE as pj_bool_t,
            nominated_check_delay: 0,
            controlled_agent_want_nom_timeout: 0,
            trickle: 0 as pj_ice_sess_trickle,
        }
    }
}

impl AutoCreate<pj_time_val> for pj_time_val {
    fn new() -> pj_time_val {
        pj_time_val { sec: 0, msec: 0 }
    }
}

impl AutoCreate<pj_qos_params> for pj_qos_params {
    fn new() -> pj_qos_params {
        pj_qos_params {
            flags: 0,
            dscp_val: 0,
            so_prio: 0,
            wmm_prio: 0,
        }
    }
}

impl AutoCreate<pj_sockopt_params__bindgen_ty_1> for pj_sockopt_params__bindgen_ty_1 {
    fn new() -> pj_sockopt_params__bindgen_ty_1 {
        pj_sockopt_params__bindgen_ty_1 {
            level: 0,
            optname: 0,
            optval: ptr::null_mut(),
            optlen: 0,
        }
    }
}

impl AutoCreate<pj_sockopt_params> for pj_sockopt_params {
    fn new() -> pj_sockopt_params {
        pj_sockopt_params {
            cnt: 0,
            options: [pj_sockopt_params__bindgen_ty_1::new(); 4],
        }
    }
}

impl AutoCreate<pj_ssl_sock_cb> for pj_ssl_sock_cb {
    fn new() -> pj_ssl_sock_cb {
        pj_ssl_sock_cb {
            on_data_read: None,
            on_data_recvfrom: None,
            on_data_sent: None,
            on_accept_complete: None,
            on_accept_complete2: None,
            on_connect_complete: None,
        }
    }
}

impl AutoCreate<pj_ssl_sock_param> for pj_ssl_sock_param {
    fn new() -> pj_ssl_sock_param {
        pj_ssl_sock_param {
            grp_lock: ptr::null_mut(),
            sock_af: 0,
            sock_type: 0,
            ioqueue: ptr::null_mut(),
            timer_heap: ptr::null_mut(),
            cb: pj_ssl_sock_cb::new(),
            user_data: ptr::null_mut(),
            proto: 0 as pj_uint32_t,
            async_cnt: 0,
            concurrency: 0,
            whole_data: pj_constants__PJ_FALSE as pj_bool_t,
            send_buffer_size: 0,
            read_buffer_size: 0,
            ciphers_num: 0,
            ciphers: ptr::null_mut(),
            curves_num: 0,
            curves: ptr::null_mut(),
            sigalgs: pj_str_t::new(),
            entropy_type: 0,
            entropy_path: pj_str_t::new(),
            timeout: pj_time_val::new(),
            verify_peer: pj_constants__PJ_FALSE as pj_bool_t,
            require_client_cert: pj_constants__PJ_FALSE as pj_bool_t,
            server_name: pj_str_t::new(),
            reuse_addr: pj_constants__PJ_FALSE as pj_bool_t,
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            qos_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
            sockopt_params: pj_sockopt_params::new(),
            sockopt_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pj_turn_sock_tls_cfg> for pj_turn_sock_tls_cfg {
    fn new() -> pj_turn_sock_tls_cfg {
        pj_turn_sock_tls_cfg {
            ca_list_file: pj_str_t::new(),
            ca_list_path: pj_str_t::new(),
            cert_file: pj_str_t::new(),
            privkey_file: pj_str_t::new(),
            ca_buf: pj_str_t::new() as pj_ssl_cert_buffer,
            cert_buf: pj_str_t::new() as pj_ssl_cert_buffer,
            privkey_buf: pj_str_t::new() as pj_ssl_cert_buffer,
            password: pj_str_t::new(),
            ssock_param: pj_ssl_sock_param::new(),
        }
    }
}

impl AutoCreate<pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1>
    for pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1
{
    fn new() -> pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 {
        pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 {
            realm: pj_str_t::new(),
            username: pj_str_t::new(),
            data_type: 0,
            data: pj_str_t::new(),
            nonce: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2>
    for pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2
{
    fn new() -> pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 {
        pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 {
            user_data: ptr::null_mut(),
            get_auth: None,
            get_cred: None,
            get_password: None,
            verify_nonce: None,
        }
    }
}

impl AutoCreate<pj_stun_auth_cred__bindgen_ty_1> for pj_stun_auth_cred__bindgen_ty_1 {
    fn new() -> pj_stun_auth_cred__bindgen_ty_1 {
        pj_stun_auth_cred__bindgen_ty_1 {
            static_cred: pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pj_stun_auth_cred> for pj_stun_auth_cred {
    fn new() -> pj_stun_auth_cred {
        pj_stun_auth_cred {
            type_: 0,
            data: pj_stun_auth_cred__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pj_ioqueue_op_key_t> for pj_ioqueue_op_key_t {
    fn new() -> pj_ioqueue_op_key_t {
        pj_ioqueue_op_key_t {
            internal__: [ptr::null_mut(); 32],
            activesock_data: ptr::null_mut(),
            user_data: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pj_sockaddr_in> for pj_sockaddr_in {
    fn new() -> pj_sockaddr_in {
        pj_sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 } as pj_in_addr,
            sin_zero_pad: [0; 8],
        }
    }
}

impl AutoCreate<pj_sockaddr> for pj_sockaddr {
    fn new() -> pj_sockaddr {
        pj_sockaddr {
            ipv4: pj_sockaddr_in::new(),
        }
    }
}

impl AutoCreate<pjrpid_element> for pjrpid_element {
    fn new() -> pjrpid_element {
        pjrpid_element {
            type_: 0,
            id: pj_str_t::new(),
            activity: 0,
            note: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pj_timer_entry> for pj_timer_entry {
    fn new() -> pj_timer_entry {
        pj_timer_entry {
            user_data: ptr::null_mut(),
            id: -1,
            cb: None,
            _timer_id: -1,
        }
    }
}

pub trait PjTimerEntry {
    unsafe extern "C" fn pj_timer_heap_callback(
        timer_heap: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
    );
}
