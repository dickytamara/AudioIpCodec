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






extern "C" {
    pub static mut PJ_VERSION: *const ::std::os::raw::c_char;
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
