#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const PJ_CC_NAME: &'static [u8; 4usize] = b"gcc\0";
pub const PJ_HAS_INT64: u32 = 1;
pub const PJ_INT64_FMT: &'static [u8; 2usize] = b"L\0";
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
pub const PJ_IOQUEUE_MAX_EVENTS_IN_SINGLE_POLL: u32 = 16;
pub const PJ_IOQUEUE_MAX_CAND_EVENTS: u32 = 16;
pub const PJ_SOMAXCONN: u32 = 5;
pub const PJ_INVALID_SOCKET: i32 = -1;
pub const PJ_INET_ADDRSTRLEN: u32 = 16;
pub const PJ_INET6_ADDRSTRLEN: u32 = 46;
pub const PJ_SOCKADDR_IN_SIN_ZERO_LEN: u32 = 8;
pub const pj_hex_digits: &'static [u8; 17usize] = b"0123456789abcdef\0";
pub const PJ_ERR_MSG_SIZE: u32 = 80;
pub const PJ_PERROR_TITLE_BUF_SIZE: u32 = 120;
pub const PJ_ERRNO_START: u32 = 20000;
pub const PJ_ERRNO_SPACE_SIZE: u32 = 50000;
pub const PJ_ERRNO_START_STATUS: u32 = 70000;
pub const PJ_ERRNO_START_SYS: u32 = 120000;
pub const PJ_ERRNO_START_USER: u32 = 170000;
pub const PJ_GUID_MAX_LENGTH: u32 = 36;
pub const PJ_PI: f64 = 3.141592653589793;
pub const PJ_1_PI: f64 = 0.3183098861837907;
pub const PJ_THREAD_DESC_SIZE: u32 = 64;
pub const PJ_POOL_ALIGNMENT: u32 = 4;
pub const PJ_CACHING_POOL_ARRAY_SIZE: u32 = 16;
pub type pj_int64_t = ::std::os::raw::c_longlong;
pub type pj_uint64_t = ::std::os::raw::c_ulonglong;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
extern "C" {
    pub static mut PJ_VERSION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_get_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_dump_config();
}
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
pub const PJ_SUCCESS: pj_constants_ = 0;
pub const PJ_TRUE: pj_constants_ = 1;
pub const PJ_FALSE: pj_constants_ = 0;
pub type pj_constants_ = u32;
pub type pj_off_t = pj_int64_t;
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
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_timestamp__bindgen_ty_1 {
    pub lo: pj_uint32_t,
    pub hi: pj_uint32_t,
}
pub type pj_list_type = ::std::os::raw::c_void;
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
pub type pj_oshandle_t = *mut ::std::os::raw::c_void;
pub type pj_sock_t = ::std::os::raw::c_long;
pub type pj_sockaddr_t = ::std::os::raw::c_void;
pub type pj_color_t = ::std::os::raw::c_uint;
pub type pj_exception_id_t = ::std::os::raw::c_int;
extern "C" {
    pub fn pj_init() -> pj_status_t;
}
extern "C" {
    pub fn pj_shutdown();
}
pub type pj_exit_callback = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    pub fn pj_atexit(func: pj_exit_callback) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_time_val {
    pub sec: ::std::os::raw::c_long,
    pub msec: ::std::os::raw::c_long,
}
extern "C" {
    pub fn pj_time_val_normalize(t: *mut pj_time_val);
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
pub const PJ_TERM_COLOR_R: u32 = 2;
pub const PJ_TERM_COLOR_G: u32 = 4;
pub const PJ_TERM_COLOR_B: u32 = 1;
pub const PJ_TERM_COLOR_BRIGHT: u32 = 8;
pub type _bindgen_ty_1 = u32;
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
pub const PJ_IOQUEUE_OP_NONE: pj_ioqueue_operation_e = 0;
pub const PJ_IOQUEUE_OP_READ: pj_ioqueue_operation_e = 1;
pub const PJ_IOQUEUE_OP_RECV: pj_ioqueue_operation_e = 2;
pub const PJ_IOQUEUE_OP_RECV_FROM: pj_ioqueue_operation_e = 4;
pub const PJ_IOQUEUE_OP_WRITE: pj_ioqueue_operation_e = 8;
pub const PJ_IOQUEUE_OP_SEND: pj_ioqueue_operation_e = 16;
pub const PJ_IOQUEUE_OP_SEND_TO: pj_ioqueue_operation_e = 32;
pub const PJ_IOQUEUE_OP_ACCEPT: pj_ioqueue_operation_e = 64;
pub const PJ_IOQUEUE_OP_CONNECT: pj_ioqueue_operation_e = 128;
pub type pj_ioqueue_operation_e = u32;
extern "C" {
    pub fn pj_ioqueue_name() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_ioqueue_create(
        pool: *mut pj_pool_t,
        max_fd: pj_size_t,
        ioqueue: *mut *mut pj_ioqueue_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_destroy(ioque: *mut pj_ioqueue_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_set_lock(
        ioque: *mut pj_ioqueue_t,
        lock: *mut pj_lock_t,
        auto_delete: pj_bool_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_set_default_concurrency(
        ioqueue: *mut pj_ioqueue_t,
        allow: pj_bool_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_register_sock(
        pool: *mut pj_pool_t,
        ioque: *mut pj_ioqueue_t,
        sock: pj_sock_t,
        user_data: *mut ::std::os::raw::c_void,
        cb: *const pj_ioqueue_callback,
        key: *mut *mut pj_ioqueue_key_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_register_sock2(
        pool: *mut pj_pool_t,
        ioque: *mut pj_ioqueue_t,
        sock: pj_sock_t,
        grp_lock: *mut pj_grp_lock_t,
        user_data: *mut ::std::os::raw::c_void,
        cb: *const pj_ioqueue_callback,
        key: *mut *mut pj_ioqueue_key_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_unregister(key: *mut pj_ioqueue_key_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_get_user_data(key: *mut pj_ioqueue_key_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_ioqueue_set_user_data(
        key: *mut pj_ioqueue_key_t,
        user_data: *mut ::std::os::raw::c_void,
        old_data: *mut *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_set_concurrency(key: *mut pj_ioqueue_key_t, allow: pj_bool_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_lock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_trylock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_unlock_key(key: *mut pj_ioqueue_key_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_op_key_init(op_key: *mut pj_ioqueue_op_key_t, size: pj_size_t);
}
extern "C" {
    pub fn pj_ioqueue_is_pending(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
    ) -> pj_bool_t;
}
extern "C" {
    pub fn pj_ioqueue_post_completion(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
        bytes_status: pj_ssize_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_accept(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
        new_sock: *mut pj_sock_t,
        local: *mut pj_sockaddr_t,
        remote: *mut pj_sockaddr_t,
        addrlen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_connect(
        key: *mut pj_ioqueue_key_t,
        addr: *const pj_sockaddr_t,
        addrlen: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_poll(
        ioque: *mut pj_ioqueue_t,
        timeout: *const pj_time_val,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_ioqueue_recv(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
        buffer: *mut ::std::os::raw::c_void,
        length: *mut pj_ssize_t,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_recvfrom(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
        buffer: *mut ::std::os::raw::c_void,
        length: *mut pj_ssize_t,
        flags: pj_uint32_t,
        addr: *mut pj_sockaddr_t,
        addrlen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_send(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
        data: *const ::std::os::raw::c_void,
        length: *mut pj_ssize_t,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ioqueue_sendto(
        key: *mut pj_ioqueue_key_t,
        op_key: *mut pj_ioqueue_op_key_t,
        data: *const ::std::os::raw::c_void,
        length: *mut pj_ssize_t,
        flags: pj_uint32_t,
        addr: *const pj_sockaddr_t,
        addrlen: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint32_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
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
}
extern "C" {
    pub static PJ_AF_UNSPEC: pj_uint16_t;
}
extern "C" {
    pub static PJ_AF_UNIX: pj_uint16_t;
}
extern "C" {
    pub static PJ_AF_INET: pj_uint16_t;
}
extern "C" {
    pub static PJ_AF_INET6: pj_uint16_t;
}
extern "C" {
    pub static PJ_AF_PACKET: pj_uint16_t;
}
extern "C" {
    pub static PJ_AF_IRDA: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOCK_STREAM: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOCK_DGRAM: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOCK_RAW: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOCK_RDM: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOL_SOCKET: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOL_IP: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOL_TCP: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOL_UDP: pj_uint16_t;
}
extern "C" {
    pub static PJ_SOL_IPV6: pj_uint16_t;
}
extern "C" {
    pub static PJ_IP_TOS: pj_uint16_t;
}
extern "C" {
    pub static PJ_IPTOS_LOWDELAY: pj_uint16_t;
}
extern "C" {
    pub static PJ_IPTOS_THROUGHPUT: pj_uint16_t;
}
extern "C" {
    pub static PJ_IPTOS_RELIABILITY: pj_uint16_t;
}
extern "C" {
    pub static PJ_IPTOS_MINCOST: pj_uint16_t;
}
extern "C" {
    pub static PJ_IPV6_TCLASS: pj_uint16_t;
}
extern "C" {
    pub static PJ_SO_TYPE: pj_uint16_t;
}
extern "C" {
    pub static PJ_SO_RCVBUF: pj_uint16_t;
}
extern "C" {
    pub static PJ_SO_SNDBUF: pj_uint16_t;
}
extern "C" {
    pub static PJ_TCP_NODELAY: pj_uint16_t;
}
extern "C" {
    pub static PJ_SO_REUSEADDR: pj_uint16_t;
}
extern "C" {
    pub static PJ_SO_NOSIGPIPE: pj_uint16_t;
}
extern "C" {
    pub static PJ_SO_PRIORITY: pj_uint16_t;
}
extern "C" {
    pub static PJ_IP_MULTICAST_IF: pj_uint16_t;
}
extern "C" {
    pub static PJ_IP_MULTICAST_TTL: pj_uint16_t;
}
extern "C" {
    pub static PJ_IP_MULTICAST_LOOP: pj_uint16_t;
}
extern "C" {
    pub static PJ_IP_ADD_MEMBERSHIP: pj_uint16_t;
}
extern "C" {
    pub static PJ_IP_DROP_MEMBERSHIP: pj_uint16_t;
}
extern "C" {
    pub static PJ_MSG_OOB: ::std::os::raw::c_int;
}
extern "C" {
    pub static PJ_MSG_PEEK: ::std::os::raw::c_int;
}
extern "C" {
    pub static PJ_MSG_DONTROUTE: ::std::os::raw::c_int;
}
pub const PJ_SD_RECEIVE: pj_socket_sd_type = 0;
pub const PJ_SHUT_RD: pj_socket_sd_type = 0;
pub const PJ_SD_SEND: pj_socket_sd_type = 1;
pub const PJ_SHUT_WR: pj_socket_sd_type = 1;
pub const PJ_SD_BOTH: pj_socket_sd_type = 2;
pub const PJ_SHUT_RDWR: pj_socket_sd_type = 2;
pub type pj_socket_sd_type = u32;
pub type pj_in_addr = in_addr;
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
extern "C" {
    pub fn pj_ntohs(netshort: pj_uint16_t) -> pj_uint16_t;
}
extern "C" {
    pub fn pj_htons(hostshort: pj_uint16_t) -> pj_uint16_t;
}
extern "C" {
    pub fn pj_ntohl(netlong: pj_uint32_t) -> pj_uint32_t;
}
extern "C" {
    pub fn pj_htonl(hostlong: pj_uint32_t) -> pj_uint32_t;
}
extern "C" {
    pub fn pj_inet_ntoa(inaddr: pj_in_addr) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_inet_aton(cp: *const pj_str_t, inp: *mut pj_in_addr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_inet_pton(
        af: ::std::os::raw::c_int,
        src: *const pj_str_t,
        dst: *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_inet_ntop(
        af: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        dst: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_inet_ntop2(
        af: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        dst: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_sockaddr_print(
        addr: *const pj_sockaddr_t,
        buf: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_inet_addr(cp: *const pj_str_t) -> pj_in_addr;
}
extern "C" {
    pub fn pj_inet_addr2(cp: *const ::std::os::raw::c_char) -> pj_in_addr;
}
extern "C" {
    pub fn pj_sockaddr_in_init(
        addr: *mut pj_sockaddr_in,
        cp: *const pj_str_t,
        port: pj_uint16_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_init(
        af: ::std::os::raw::c_int,
        addr: *mut pj_sockaddr,
        cp: *const pj_str_t,
        port: pj_uint16_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_cmp(
        addr1: *const pj_sockaddr_t,
        addr2: *const pj_sockaddr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_sockaddr_get_addr(addr: *const pj_sockaddr_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_sockaddr_has_addr(addr: *const pj_sockaddr_t) -> pj_bool_t;
}
extern "C" {
    pub fn pj_sockaddr_get_addr_len(addr: *const pj_sockaddr_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_sockaddr_get_len(addr: *const pj_sockaddr_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_sockaddr_copy_addr(dst: *mut pj_sockaddr, src: *const pj_sockaddr);
}
extern "C" {
    pub fn pj_sockaddr_cp(dst: *mut pj_sockaddr_t, src: *const pj_sockaddr_t);
}
extern "C" {
    pub fn pj_sockaddr_synthesize(
        dst_af: ::std::os::raw::c_int,
        dst: *mut pj_sockaddr_t,
        src: *const pj_sockaddr_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_in_get_addr(addr: *const pj_sockaddr_in) -> pj_in_addr;
}
extern "C" {
    pub fn pj_sockaddr_in_set_addr(addr: *mut pj_sockaddr_in, hostaddr: pj_uint32_t);
}
extern "C" {
    pub fn pj_sockaddr_in_set_str_addr(
        addr: *mut pj_sockaddr_in,
        cp: *const pj_str_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_set_str_addr(
        af: ::std::os::raw::c_int,
        addr: *mut pj_sockaddr,
        cp: *const pj_str_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_get_port(addr: *const pj_sockaddr_t) -> pj_uint16_t;
}
extern "C" {
    pub fn pj_sockaddr_in_get_port(addr: *const pj_sockaddr_in) -> pj_uint16_t;
}
extern "C" {
    pub fn pj_sockaddr_set_port(addr: *mut pj_sockaddr, hostport: pj_uint16_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_in_set_port(addr: *mut pj_sockaddr_in, hostport: pj_uint16_t);
}
extern "C" {
    pub fn pj_sockaddr_parse(
        af: ::std::os::raw::c_int,
        options: ::std::os::raw::c_uint,
        str_: *const pj_str_t,
        addr: *mut pj_sockaddr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sockaddr_parse2(
        af: ::std::os::raw::c_int,
        options: ::std::os::raw::c_uint,
        str_: *const pj_str_t,
        hostpart: *mut pj_str_t,
        port: *mut pj_uint16_t,
        raf: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_gethostname() -> *const pj_str_t;
}
extern "C" {
    pub fn pj_gethostaddr() -> pj_in_addr;
}
extern "C" {
    pub fn pj_sock_socket(
        family: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        protocol: ::std::os::raw::c_int,
        sock: *mut pj_sock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_close(sockfd: pj_sock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_bind(
        sockfd: pj_sock_t,
        my_addr: *const pj_sockaddr_t,
        addrlen: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_bind_in(sockfd: pj_sock_t, addr: pj_uint32_t, port: pj_uint16_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_bind_random(
        sockfd: pj_sock_t,
        addr: *const pj_sockaddr_t,
        port_range: pj_uint16_t,
        max_try: pj_uint16_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_listen(sockfd: pj_sock_t, backlog: ::std::os::raw::c_int) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_accept(
        serverfd: pj_sock_t,
        newsock: *mut pj_sock_t,
        addr: *mut pj_sockaddr_t,
        addrlen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_connect(
        sockfd: pj_sock_t,
        serv_addr: *const pj_sockaddr_t,
        addrlen: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_getpeername(
        sockfd: pj_sock_t,
        addr: *mut pj_sockaddr_t,
        namelen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_getsockname(
        sockfd: pj_sock_t,
        addr: *mut pj_sockaddr_t,
        namelen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_getsockopt(
        sockfd: pj_sock_t,
        level: pj_uint16_t,
        optname: pj_uint16_t,
        optval: *mut ::std::os::raw::c_void,
        optlen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_setsockopt(
        sockfd: pj_sock_t,
        level: pj_uint16_t,
        optname: pj_uint16_t,
        optval: *const ::std::os::raw::c_void,
        optlen: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_setsockopt_params(
        sockfd: pj_sock_t,
        params: *const pj_sockopt_params,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_setsockopt_sobuf(
        sockfd: pj_sock_t,
        optname: pj_uint16_t,
        auto_retry: pj_bool_t,
        buf_size: *mut ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_recv(
        sockfd: pj_sock_t,
        buf: *mut ::std::os::raw::c_void,
        len: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_recvfrom(
        sockfd: pj_sock_t,
        buf: *mut ::std::os::raw::c_void,
        len: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
        from: *mut pj_sockaddr_t,
        fromlen: *mut ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_send(
        sockfd: pj_sock_t,
        buf: *const ::std::os::raw::c_void,
        len: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_sendto(
        sockfd: pj_sock_t,
        buf: *const ::std::os::raw::c_void,
        len: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
        to: *const pj_sockaddr_t,
        tolen: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_shutdown(sockfd: pj_sock_t, how: ::std::os::raw::c_int) -> pj_status_t;
}
extern "C" {
    pub fn pj_addr_str_print(
        host_str: *const pj_str_t,
        port: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_activesock_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_activesock_cb {
    pub on_data_read: ::std::option::Option<
        unsafe extern "C" fn(
            asock: *mut pj_activesock_t,
            data: *mut ::std::os::raw::c_void,
            size: pj_size_t,
            status: pj_status_t,
            remainder: *mut pj_size_t,
        ) -> pj_bool_t,
    >,
    pub on_data_recvfrom: ::std::option::Option<
        unsafe extern "C" fn(
            asock: *mut pj_activesock_t,
            data: *mut ::std::os::raw::c_void,
            size: pj_size_t,
            src_addr: *const pj_sockaddr_t,
            addr_len: ::std::os::raw::c_int,
            status: pj_status_t,
        ) -> pj_bool_t,
    >,
    pub on_data_sent: ::std::option::Option<
        unsafe extern "C" fn(
            asock: *mut pj_activesock_t,
            send_key: *mut pj_ioqueue_op_key_t,
            sent: pj_ssize_t,
        ) -> pj_bool_t,
    >,
    pub on_accept_complete: ::std::option::Option<
        unsafe extern "C" fn(
            asock: *mut pj_activesock_t,
            newsock: pj_sock_t,
            src_addr: *const pj_sockaddr_t,
            src_addr_len: ::std::os::raw::c_int,
        ) -> pj_bool_t,
    >,
    pub on_accept_complete2: ::std::option::Option<
        unsafe extern "C" fn(
            asock: *mut pj_activesock_t,
            newsock: pj_sock_t,
            src_addr: *const pj_sockaddr_t,
            src_addr_len: ::std::os::raw::c_int,
            status: pj_status_t,
        ) -> pj_bool_t,
    >,
    pub on_connect_complete: ::std::option::Option<
        unsafe extern "C" fn(asock: *mut pj_activesock_t, status: pj_status_t) -> pj_bool_t,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_activesock_cfg {
    pub grp_lock: *mut pj_grp_lock_t,
    pub async_cnt: ::std::os::raw::c_uint,
    pub concurrency: ::std::os::raw::c_int,
    pub whole_data: pj_bool_t,
}
extern "C" {
    pub fn pj_activesock_cfg_default(cfg: *mut pj_activesock_cfg);
}
extern "C" {
    pub fn pj_activesock_create(
        pool: *mut pj_pool_t,
        sock: pj_sock_t,
        sock_type: ::std::os::raw::c_int,
        opt: *const pj_activesock_cfg,
        ioqueue: *mut pj_ioqueue_t,
        cb: *const pj_activesock_cb,
        user_data: *mut ::std::os::raw::c_void,
        p_asock: *mut *mut pj_activesock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_create_udp(
        pool: *mut pj_pool_t,
        addr: *const pj_sockaddr,
        opt: *const pj_activesock_cfg,
        ioqueue: *mut pj_ioqueue_t,
        cb: *const pj_activesock_cb,
        user_data: *mut ::std::os::raw::c_void,
        p_asock: *mut *mut pj_activesock_t,
        bound_addr: *mut pj_sockaddr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_close(asock: *mut pj_activesock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_set_user_data(
        asock: *mut pj_activesock_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_get_user_data(asock: *mut pj_activesock_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_activesock_start_read(
        asock: *mut pj_activesock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_start_read2(
        asock: *mut pj_activesock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        readbuf: *mut *mut ::std::os::raw::c_void,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_start_recvfrom(
        asock: *mut pj_activesock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_start_recvfrom2(
        asock: *mut pj_activesock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        readbuf: *mut *mut ::std::os::raw::c_void,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_send(
        asock: *mut pj_activesock_t,
        send_key: *mut pj_ioqueue_op_key_t,
        data: *const ::std::os::raw::c_void,
        size: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_sendto(
        asock: *mut pj_activesock_t,
        send_key: *mut pj_ioqueue_op_key_t,
        data: *const ::std::os::raw::c_void,
        size: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
        addr: *const pj_sockaddr_t,
        addr_len: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_start_accept(
        asock: *mut pj_activesock_t,
        pool: *mut pj_pool_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_activesock_start_connect(
        asock: *mut pj_activesock_t,
        pool: *mut pj_pool_t,
        remaddr: *const pj_sockaddr_t,
        addr_len: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_hostent {
    pub h_name: *mut ::std::os::raw::c_char,
    pub h_aliases: *mut *mut ::std::os::raw::c_char,
    pub h_addrtype: ::std::os::raw::c_int,
    pub h_length: ::std::os::raw::c_int,
    pub h_addr_list: *mut *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_addrinfo {
    pub ai_canonname: [::std::os::raw::c_char; 128usize],
    pub ai_addr: pj_sockaddr,
}
extern "C" {
    pub fn pj_gethostbyname(name: *const pj_str_t, he: *mut pj_hostent) -> pj_status_t;
}
extern "C" {
    pub fn pj_gethostip(af: ::std::os::raw::c_int, addr: *mut pj_sockaddr) -> pj_status_t;
}
extern "C" {
    pub fn pj_getipinterface(
        af: ::std::os::raw::c_int,
        dst: *const pj_str_t,
        itf_addr: *mut pj_sockaddr,
        allow_resolve: pj_bool_t,
        p_dst_addr: *mut pj_sockaddr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_getdefaultipinterface(
        af: ::std::os::raw::c_int,
        addr: *mut pj_sockaddr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_getaddrinfo(
        af: ::std::os::raw::c_int,
        name: *const pj_str_t,
        count: *mut ::std::os::raw::c_uint,
        ai: *mut pj_addrinfo,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_array_insert(
        array: *mut ::std::os::raw::c_void,
        elem_size: ::std::os::raw::c_uint,
        count: ::std::os::raw::c_uint,
        pos: ::std::os::raw::c_uint,
        value: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn pj_array_erase(
        array: *mut ::std::os::raw::c_void,
        elem_size: ::std::os::raw::c_uint,
        count: ::std::os::raw::c_uint,
        pos: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn pj_array_find(
        array: *const ::std::os::raw::c_void,
        elem_size: ::std::os::raw::c_uint,
        count: ::std::os::raw::c_uint,
        matching: ::std::option::Option<
            unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> pj_status_t,
        >,
        result: *mut *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
pub type pj_os_err_type = ::std::os::raw::c_int;
pub type va_list = __builtin_va_list;
extern "C" {
    pub fn pj_get_os_error() -> pj_status_t;
}
extern "C" {
    pub fn pj_set_os_error(code: pj_status_t);
}
extern "C" {
    pub fn pj_get_netos_error() -> pj_status_t;
}
extern "C" {
    pub fn pj_set_netos_error(code: pj_status_t);
}
extern "C" {
    pub fn pj_strerror(
        statcode: pj_status_t,
        buf: *mut ::std::os::raw::c_char,
        bufsize: pj_size_t,
    ) -> pj_str_t;
}
extern "C" {
    pub fn pj_perror(
        log_level: ::std::os::raw::c_int,
        sender: *const ::std::os::raw::c_char,
        status: pj_status_t,
        title_fmt: *const ::std::os::raw::c_char,
        ...
    );
}
pub type pj_error_callback = ::std::option::Option<
    unsafe extern "C" fn(
        e: pj_status_t,
        msg: *mut ::std::os::raw::c_char,
        max: pj_size_t,
    ) -> pj_str_t,
>;
extern "C" {
    pub fn pj_register_strerror(
        start_code: pj_status_t,
        err_space: pj_status_t,
        f: pj_error_callback,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_errno_clear_handlers();
}
extern "C" {
    pub fn pj_perror_1(
        sender: *const ::std::os::raw::c_char,
        status: pj_status_t,
        title_fmt: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn pj_perror_2(
        sender: *const ::std::os::raw::c_char,
        status: pj_status_t,
        title_fmt: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn pj_perror_3(
        sender: *const ::std::os::raw::c_char,
        status: pj_status_t,
        title_fmt: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn pj_perror_4(
        sender: *const ::std::os::raw::c_char,
        status: pj_status_t,
        title_fmt: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn pj_perror_5(
        sender: *const ::std::os::raw::c_char,
        status: pj_status_t,
        title_fmt: *const ::std::os::raw::c_char,
        ...
    );
}
pub type __jmp_buf = [::std::os::raw::c_long; 8usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
    pub __saved_mask: __sigset_t,
}
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
pub type pj_log_decoration = u32;
pub type pj_log_func = ::std::option::Option<
    unsafe extern "C" fn(
        level: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ),
>;
extern "C" {
    pub fn pj_log_write(
        level: ::std::os::raw::c_int,
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn pj_log(
        sender: *const ::std::os::raw::c_char,
        level: ::std::os::raw::c_int,
        format: *const ::std::os::raw::c_char,
        marker: *mut __va_list_tag,
    );
}
extern "C" {
    pub fn pj_log_set_log_func(func: pj_log_func);
}
extern "C" {
    pub fn pj_log_get_log_func() -> pj_log_func;
}
extern "C" {
    pub fn pj_log_set_level(level: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pj_log_get_level() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_log_set_decor(decor: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn pj_log_get_decor() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_log_add_indent(indent: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pj_log_push_indent();
}
extern "C" {
    pub fn pj_log_pop_indent();
}
extern "C" {
    pub fn pj_log_set_color(level: ::std::os::raw::c_int, color: pj_color_t);
}
extern "C" {
    pub fn pj_log_get_color(level: ::std::os::raw::c_int) -> pj_color_t;
}
extern "C" {
    pub fn pj_log_init() -> pj_status_t;
}
extern "C" {
    pub fn pj_log_1(src: *const ::std::os::raw::c_char, format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn pj_log_2(src: *const ::std::os::raw::c_char, format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn pj_log_3(src: *const ::std::os::raw::c_char, format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn pj_log_4(src: *const ::std::os::raw::c_char, format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn pj_log_5(src: *const ::std::os::raw::c_char, format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn pj_exception_id_alloc(
        name: *const ::std::os::raw::c_char,
        id: *mut pj_exception_id_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_exception_id_free(id: pj_exception_id_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_exception_id_name(id: pj_exception_id_t) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_exception_state_t {
    pub state: pj_jmp_buf,
    pub prev: *mut pj_exception_state_t,
}
extern "C" {
    pub fn pj_throw_exception_(id: pj_exception_id_t);
}
extern "C" {
    pub fn pj_push_exception_handler_(rec: *mut pj_exception_state_t);
}
extern "C" {
    pub fn pj_pop_exception_handler_(rec: *mut pj_exception_state_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_fifobuf_t {
    pub first: *mut ::std::os::raw::c_char,
    pub last: *mut ::std::os::raw::c_char,
    pub ubegin: *mut ::std::os::raw::c_char,
    pub uend: *mut ::std::os::raw::c_char,
    pub full: ::std::os::raw::c_int,
}
extern "C" {
    pub fn pj_fifobuf_init(
        fb: *mut pj_fifobuf_t,
        buffer: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn pj_fifobuf_max_size(fb: *mut pj_fifobuf_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_fifobuf_alloc(
        fb: *mut pj_fifobuf_t,
        size: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_fifobuf_unalloc(
        fb: *mut pj_fifobuf_t,
        buf: *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_fifobuf_free(fb: *mut pj_fifobuf_t, buf: *mut ::std::os::raw::c_void) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_file_stat {
    pub size: pj_off_t,
    pub atime: pj_time_val,
    pub mtime: pj_time_val,
    pub ctime: pj_time_val,
}
extern "C" {
    pub fn pj_file_exists(filename: *const ::std::os::raw::c_char) -> pj_bool_t;
}
extern "C" {
    pub fn pj_file_size(filename: *const ::std::os::raw::c_char) -> pj_off_t;
}
extern "C" {
    pub fn pj_file_delete(filename: *const ::std::os::raw::c_char) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_move(
        oldname: *const ::std::os::raw::c_char,
        newname: *const ::std::os::raw::c_char,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_getstat(
        filename: *const ::std::os::raw::c_char,
        stat: *mut pj_file_stat,
    ) -> pj_status_t;
}
pub const PJ_O_RDONLY: pj_file_access = 4353;
pub const PJ_O_WRONLY: pj_file_access = 4354;
pub const PJ_O_RDWR: pj_file_access = 4355;
pub const PJ_O_APPEND: pj_file_access = 4360;
pub type pj_file_access = u32;
pub const PJ_SEEK_SET: pj_file_seek_type = 4609;
pub const PJ_SEEK_CUR: pj_file_seek_type = 4610;
pub const PJ_SEEK_END: pj_file_seek_type = 4611;
pub type pj_file_seek_type = u32;
extern "C" {
    pub fn pj_file_open(
        pool: *mut pj_pool_t,
        pathname: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_uint,
        fd: *mut pj_oshandle_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_close(fd: pj_oshandle_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_write(
        fd: pj_oshandle_t,
        data: *const ::std::os::raw::c_void,
        size: *mut pj_ssize_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_read(
        fd: pj_oshandle_t,
        data: *mut ::std::os::raw::c_void,
        size: *mut pj_ssize_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_setpos(
        fd: pj_oshandle_t,
        offset: pj_off_t,
        whence: pj_file_seek_type,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_getpos(fd: pj_oshandle_t, pos: *mut pj_off_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_file_flush(fd: pj_oshandle_t) -> pj_status_t;
}
extern "C" {
    pub static PJ_GUID_STRING_LENGTH: ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_GUID_STRING_LENGTH() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_generate_unique_string(str_: *mut pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_generate_unique_string_lower(str_: *mut pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_create_unique_string(pool: *mut pj_pool_t, str_: *mut pj_str_t);
}
extern "C" {
    pub fn pj_create_unique_string_lower(pool: *mut pj_pool_t, str_: *mut pj_str_t);
}
pub type pj_hash_entry_buf = [*mut ::std::os::raw::c_void; 4usize];
extern "C" {
    pub fn pj_hash_calc(
        hval: pj_uint32_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
    ) -> pj_uint32_t;
}
extern "C" {
    pub fn pj_hash_calc_tolower(
        hval: pj_uint32_t,
        result: *mut ::std::os::raw::c_char,
        key: *const pj_str_t,
    ) -> pj_uint32_t;
}
extern "C" {
    pub fn pj_hash_create(
        pool: *mut pj_pool_t,
        size: ::std::os::raw::c_uint,
    ) -> *mut pj_hash_table_t;
}
extern "C" {
    pub fn pj_hash_get(
        ht: *mut pj_hash_table_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
        hval: *mut pj_uint32_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_hash_get_lower(
        ht: *mut pj_hash_table_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
        hval: *mut pj_uint32_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_hash_set(
        pool: *mut pj_pool_t,
        ht: *mut pj_hash_table_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
        hval: pj_uint32_t,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn pj_hash_set_lower(
        pool: *mut pj_pool_t,
        ht: *mut pj_hash_table_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
        hval: pj_uint32_t,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn pj_hash_set_np(
        ht: *mut pj_hash_table_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
        hval: pj_uint32_t,
        entry_buf: *mut *mut ::std::os::raw::c_void,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn pj_hash_set_np_lower(
        ht: *mut pj_hash_table_t,
        key: *const ::std::os::raw::c_void,
        keylen: ::std::os::raw::c_uint,
        hval: pj_uint32_t,
        entry_buf: *mut *mut ::std::os::raw::c_void,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn pj_hash_count(ht: *mut pj_hash_table_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_hash_first(
        ht: *mut pj_hash_table_t,
        it: *mut pj_hash_iterator_t,
    ) -> *mut pj_hash_iterator_t;
}
extern "C" {
    pub fn pj_hash_next(
        ht: *mut pj_hash_table_t,
        it: *mut pj_hash_iterator_t,
    ) -> *mut pj_hash_iterator_t;
}
extern "C" {
    pub fn pj_hash_this(
        ht: *mut pj_hash_table_t,
        it: *mut pj_hash_iterator_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_str(str_: *mut ::std::os::raw::c_char) -> pj_str_t;
}
extern "C" {
    pub fn pj_strassign(dst: *mut pj_str_t, src: *mut pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strcpy(dst: *mut pj_str_t, src: *const pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strcpy2(dst: *mut pj_str_t, src: *const ::std::os::raw::c_char) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strncpy(dst: *mut pj_str_t, src: *const pj_str_t, max: pj_ssize_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strncpy_with_null(
        dst: *mut pj_str_t,
        src: *const pj_str_t,
        max: pj_ssize_t,
    ) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strdup(
        pool: *mut pj_pool_t,
        dst: *mut pj_str_t,
        src: *const pj_str_t,
    ) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strdup_with_null(
        pool: *mut pj_pool_t,
        dst: *mut pj_str_t,
        src: *const pj_str_t,
    ) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strdup2(
        pool: *mut pj_pool_t,
        dst: *mut pj_str_t,
        src: *const ::std::os::raw::c_char,
    ) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strdup2_with_null(
        pool: *mut pj_pool_t,
        dst: *mut pj_str_t,
        src: *const ::std::os::raw::c_char,
    ) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strdup3(pool: *mut pj_pool_t, src: *const ::std::os::raw::c_char) -> pj_str_t;
}
extern "C" {
    pub fn pj_strcmp(str1: *const pj_str_t, str2: *const pj_str_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_strcmp2(
        str1: *const pj_str_t,
        str2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_strncmp(
        str1: *const pj_str_t,
        str2: *const pj_str_t,
        len: pj_size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_strncmp2(
        str1: *const pj_str_t,
        str2: *const ::std::os::raw::c_char,
        len: pj_size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_stricmp(str1: *const pj_str_t, str2: *const pj_str_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_stricmp2(
        str1: *const pj_str_t,
        str2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_strnicmp(
        str1: *const pj_str_t,
        str2: *const pj_str_t,
        len: pj_size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_strnicmp2(
        str1: *const pj_str_t,
        str2: *const ::std::os::raw::c_char,
        len: pj_size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_strcat(dst: *mut pj_str_t, src: *const pj_str_t);
}
extern "C" {
    pub fn pj_strcat2(dst: *mut pj_str_t, src: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn pj_strspn(str_: *const pj_str_t, set_char: *const pj_str_t) -> pj_ssize_t;
}
extern "C" {
    pub fn pj_strspn2(str_: *const pj_str_t, set_char: *const ::std::os::raw::c_char)
        -> pj_ssize_t;
}
extern "C" {
    pub fn pj_strcspn(str_: *const pj_str_t, set_char: *const pj_str_t) -> pj_ssize_t;
}
extern "C" {
    pub fn pj_strcspn2(
        str_: *const pj_str_t,
        set_char: *const ::std::os::raw::c_char,
    ) -> pj_ssize_t;
}
extern "C" {
    pub fn pj_strtok(
        str_: *const pj_str_t,
        delim: *const pj_str_t,
        tok: *mut pj_str_t,
        start_idx: pj_size_t,
    ) -> pj_ssize_t;
}
extern "C" {
    pub fn pj_strtok2(
        str_: *const pj_str_t,
        delim: *const ::std::os::raw::c_char,
        tok: *mut pj_str_t,
        start_idx: pj_size_t,
    ) -> pj_ssize_t;
}
extern "C" {
    pub fn pj_strstr(str_: *const pj_str_t, substr: *const pj_str_t)
        -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_stristr(
        str_: *const pj_str_t,
        substr: *const pj_str_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_strltrim(str_: *mut pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strrtrim(str_: *mut pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_strtrim(str_: *mut pj_str_t) -> *mut pj_str_t;
}
extern "C" {
    pub fn pj_create_random_string(
        str_: *mut ::std::os::raw::c_char,
        length: pj_size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_strtol(str_: *const pj_str_t) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn pj_strtol2(str_: *const pj_str_t, value: *mut ::std::os::raw::c_long) -> pj_status_t;
}
extern "C" {
    pub fn pj_strtoul(str_: *const pj_str_t) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn pj_strtoul2(
        str_: *const pj_str_t,
        endptr: *mut pj_str_t,
        base: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn pj_strtoul3(
        str_: *const pj_str_t,
        value: *mut ::std::os::raw::c_ulong,
        base: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_strtof(str_: *const pj_str_t) -> f32;
}
extern "C" {
    pub fn pj_utoa(
        val: ::std::os::raw::c_ulong,
        buf: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_utoa_pad(
        val: ::std::os::raw::c_ulong,
        buf: *mut ::std::os::raw::c_char,
        min_dig: ::std::os::raw::c_int,
        pad: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pj_ip_route_entry {
    pub ipv4: pj_ip_route_entry__bindgen_ty_1,
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
    pub af: ::std::os::raw::c_int,
    pub omit_deprecated_ipv6: pj_bool_t,
}
extern "C" {
    pub fn pj_enum_ip_interface(
        af: ::std::os::raw::c_int,
        count: *mut ::std::os::raw::c_uint,
        ifs: *mut pj_sockaddr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_enum_ip_interface2(
        opt: *const pj_enum_ip_option,
        count: *mut ::std::os::raw::c_uint,
        ifs: *mut pj_sockaddr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_enum_ip_route(
        count: *mut ::std::os::raw::c_uint,
        routes: *mut pj_ip_route_entry,
    ) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_list {
    pub prev: *mut ::std::os::raw::c_void,
    pub next: *mut ::std::os::raw::c_void,
}
extern "C" {
    pub fn pj_list_insert_before(pos: *mut pj_list_type, node: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_insert_nodes_before(lst: *mut pj_list_type, nodes: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_insert_after(pos: *mut pj_list_type, node: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_insert_nodes_after(lst: *mut pj_list_type, nodes: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_merge_first(list1: *mut pj_list_type, list2: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_merge_last(list1: *mut pj_list_type, list2: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_erase(node: *mut pj_list_type);
}
extern "C" {
    pub fn pj_list_find_node(list: *mut pj_list_type, node: *mut pj_list_type)
        -> *mut pj_list_type;
}
extern "C" {
    pub fn pj_list_search(
        list: *mut pj_list_type,
        value: *mut ::std::os::raw::c_void,
        comp: ::std::option::Option<
            unsafe extern "C" fn(
                value: *mut ::std::os::raw::c_void,
                node: *const pj_list_type,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> *mut pj_list_type;
}
extern "C" {
    pub fn pj_list_size(list: *const pj_list_type) -> pj_size_t;
}
extern "C" {
    pub fn pj_lock_create_simple_mutex(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        lock: *mut *mut pj_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_create_recursive_mutex(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        lock: *mut *mut pj_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_create_null_mutex(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        lock: *mut *mut pj_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_create_semaphore(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        initial: ::std::os::raw::c_uint,
        max: ::std::os::raw::c_uint,
        lock: *mut *mut pj_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_acquire(lock: *mut pj_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_tryacquire(lock: *mut pj_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_release(lock: *mut pj_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_lock_destroy(lock: *mut pj_lock_t) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_grp_lock_config {
    pub flags: ::std::os::raw::c_uint,
}
extern "C" {
    pub fn pj_grp_lock_config_default(cfg: *mut pj_grp_lock_config);
}
extern "C" {
    pub fn pj_grp_lock_create(
        pool: *mut pj_pool_t,
        cfg: *const pj_grp_lock_config,
        p_grp_lock: *mut *mut pj_grp_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_create_w_handler(
        pool: *mut pj_pool_t,
        cfg: *const pj_grp_lock_config,
        member: *mut ::std::os::raw::c_void,
        handler: ::std::option::Option<unsafe extern "C" fn(member: *mut ::std::os::raw::c_void)>,
        p_grp_lock: *mut *mut pj_grp_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_destroy(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_replace(
        old_lock: *mut pj_grp_lock_t,
        new_lock: *mut pj_grp_lock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_acquire(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_tryacquire(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_release(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_add_handler(
        grp_lock: *mut pj_grp_lock_t,
        pool: *mut pj_pool_t,
        member: *mut ::std::os::raw::c_void,
        handler: ::std::option::Option<unsafe extern "C" fn(member: *mut ::std::os::raw::c_void)>,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_del_handler(
        grp_lock: *mut pj_grp_lock_t,
        member: *mut ::std::os::raw::c_void,
        handler: ::std::option::Option<unsafe extern "C" fn(member: *mut ::std::os::raw::c_void)>,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_add_ref(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_dec_ref(grp_lock: *mut pj_grp_lock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_get_ref(grp_lock: *mut pj_grp_lock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_grp_lock_dump(grp_lock: *mut pj_grp_lock_t);
}
extern "C" {
    pub fn pj_grp_lock_chain_lock(
        grp_lock: *mut pj_grp_lock_t,
        ext_lock: *mut pj_lock_t,
        pos: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_grp_lock_unchain_lock(
        grp_lock: *mut pj_grp_lock_t,
        ext_lock: *mut pj_lock_t,
    ) -> pj_status_t;
}
pub type pj_highprec_t = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_math_stat {
    pub n: ::std::os::raw::c_int,
    pub max: ::std::os::raw::c_int,
    pub min: ::std::os::raw::c_int,
    pub last: ::std::os::raw::c_int,
    pub mean: ::std::os::raw::c_int,
    pub fmean_: f32,
    pub m2_: pj_highprec_t,
}
pub const PJ_SYS_HAS_IOS_BG: pj_sys_info_flag = 1;
pub type pj_sys_info_flag = u32;
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
extern "C" {
    pub fn pj_get_sys_info() -> *const pj_sys_info;
}
pub const PJ_THREAD_SUSPENDED: pj_thread_create_flags = 1;
pub type pj_thread_create_flags = u32;
pub type pj_thread_proc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
pub type pj_thread_desc = [::std::os::raw::c_long; 64usize];
extern "C" {
    pub fn pj_getpid() -> pj_uint32_t;
}
extern "C" {
    pub fn pj_thread_create(
        pool: *mut pj_pool_t,
        thread_name: *const ::std::os::raw::c_char,
        proc_: pj_thread_proc,
        arg: *mut ::std::os::raw::c_void,
        stack_size: pj_size_t,
        flags: ::std::os::raw::c_uint,
        thread: *mut *mut pj_thread_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_register(
        thread_name: *const ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_long,
        thread: *mut *mut pj_thread_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_is_registered() -> pj_bool_t;
}
extern "C" {
    pub fn pj_thread_get_prio(thread: *mut pj_thread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_thread_set_prio(thread: *mut pj_thread_t, prio: ::std::os::raw::c_int)
        -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_get_prio_min(thread: *mut pj_thread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_thread_get_prio_max(thread: *mut pj_thread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_thread_get_os_handle(thread: *mut pj_thread_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_thread_get_name(thread: *mut pj_thread_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_thread_resume(thread: *mut pj_thread_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_this() -> *mut pj_thread_t;
}
extern "C" {
    pub fn pj_thread_join(thread: *mut pj_thread_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_destroy(thread: *mut pj_thread_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_sleep(msec: ::std::os::raw::c_uint) -> pj_status_t;
}
extern "C" {
    pub fn pj_symbianos_poll(
        priority: ::std::os::raw::c_int,
        ms_timeout: ::std::os::raw::c_int,
    ) -> pj_bool_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_symbianos_params {
    pub rsocketserv: *mut ::std::os::raw::c_void,
    pub rconnection: *mut ::std::os::raw::c_void,
    pub rhostresolver: *mut ::std::os::raw::c_void,
    pub rhostresolver6: *mut ::std::os::raw::c_void,
}
extern "C" {
    pub fn pj_symbianos_set_params(prm: *mut pj_symbianos_params) -> pj_status_t;
}
extern "C" {
    pub fn pj_symbianos_set_connection_status(up: pj_bool_t);
}
extern "C" {
    pub fn pj_thread_local_alloc(index: *mut ::std::os::raw::c_long) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_local_free(index: ::std::os::raw::c_long);
}
extern "C" {
    pub fn pj_thread_local_set(
        index: ::std::os::raw::c_long,
        value: *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_thread_local_get(index: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_atomic_create(
        pool: *mut pj_pool_t,
        initial: pj_atomic_value_t,
        atomic: *mut *mut pj_atomic_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_atomic_destroy(atomic_var: *mut pj_atomic_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_atomic_set(atomic_var: *mut pj_atomic_t, value: pj_atomic_value_t);
}
extern "C" {
    pub fn pj_atomic_get(atomic_var: *mut pj_atomic_t) -> pj_atomic_value_t;
}
extern "C" {
    pub fn pj_atomic_inc(atomic_var: *mut pj_atomic_t);
}
extern "C" {
    pub fn pj_atomic_inc_and_get(atomic_var: *mut pj_atomic_t) -> pj_atomic_value_t;
}
extern "C" {
    pub fn pj_atomic_dec(atomic_var: *mut pj_atomic_t);
}
extern "C" {
    pub fn pj_atomic_dec_and_get(atomic_var: *mut pj_atomic_t) -> pj_atomic_value_t;
}
extern "C" {
    pub fn pj_atomic_add(atomic_var: *mut pj_atomic_t, value: pj_atomic_value_t);
}
extern "C" {
    pub fn pj_atomic_add_and_get(
        atomic_var: *mut pj_atomic_t,
        value: pj_atomic_value_t,
    ) -> pj_atomic_value_t;
}
pub const PJ_MUTEX_DEFAULT: pj_mutex_type_e = 0;
pub const PJ_MUTEX_SIMPLE: pj_mutex_type_e = 1;
pub const PJ_MUTEX_RECURSE: pj_mutex_type_e = 2;
pub type pj_mutex_type_e = u32;
extern "C" {
    pub fn pj_mutex_create(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
        mutex: *mut *mut pj_mutex_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_create_simple(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        mutex: *mut *mut pj_mutex_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_create_recursive(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        mutex: *mut *mut pj_mutex_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_lock(mutex: *mut pj_mutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_unlock(mutex: *mut pj_mutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_trylock(mutex: *mut pj_mutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_destroy(mutex: *mut pj_mutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_mutex_is_locked(mutex: *mut pj_mutex_t) -> pj_bool_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_rwmutex_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn pj_rwmutex_create(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        mutex: *mut *mut pj_rwmutex_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_rwmutex_lock_read(mutex: *mut pj_rwmutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_rwmutex_lock_write(mutex: *mut pj_rwmutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_rwmutex_unlock_read(mutex: *mut pj_rwmutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_rwmutex_unlock_write(mutex: *mut pj_rwmutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_rwmutex_destroy(mutex: *mut pj_rwmutex_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_enter_critical_section();
}
extern "C" {
    pub fn pj_leave_critical_section();
}
extern "C" {
    pub fn pj_sem_create(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        initial: ::std::os::raw::c_uint,
        max: ::std::os::raw::c_uint,
        sem: *mut *mut pj_sem_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sem_wait(sem: *mut pj_sem_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_sem_trywait(sem: *mut pj_sem_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_sem_post(sem: *mut pj_sem_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_sem_destroy(sem: *mut pj_sem_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_create(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        manual_reset: pj_bool_t,
        initial: pj_bool_t,
        event: *mut *mut pj_event_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_wait(event: *mut pj_event_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_trywait(event: *mut pj_event_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_set(event: *mut pj_event_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_pulse(event: *mut pj_event_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_reset(event: *mut pj_event_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_event_destroy(event: *mut pj_event_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_gettimeofday(tv: *mut pj_time_val) -> pj_status_t;
}
extern "C" {
    pub fn pj_time_decode(tv: *const pj_time_val, pt: *mut pj_parsed_time) -> pj_status_t;
}
extern "C" {
    pub fn pj_time_encode(pt: *const pj_parsed_time, tv: *mut pj_time_val) -> pj_status_t;
}
extern "C" {
    pub fn pj_time_local_to_gmt(tv: *mut pj_time_val) -> pj_status_t;
}
extern "C" {
    pub fn pj_time_gmt_to_local(tv: *mut pj_time_val) -> pj_status_t;
}
extern "C" {
    pub fn pj_term_set_color(color: pj_color_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_term_get_color() -> pj_color_t;
}
extern "C" {
    pub fn pj_gettickcount(tv: *mut pj_time_val) -> pj_status_t;
}
extern "C" {
    pub fn pj_get_timestamp(ts: *mut pj_timestamp) -> pj_status_t;
}
extern "C" {
    pub fn pj_get_timestamp_freq(freq: *mut pj_timestamp) -> pj_status_t;
}
extern "C" {
    pub fn pj_elapsed_time(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_time_val;
}
extern "C" {
    pub fn pj_elapsed_msec(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
}
extern "C" {
    pub fn pj_elapsed_msec64(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint64_t;
}
extern "C" {
    pub fn pj_elapsed_usec(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
}
extern "C" {
    pub fn pj_elapsed_nanosec(start: *const pj_timestamp, stop: *const pj_timestamp)
        -> pj_uint32_t;
}
extern "C" {
    pub fn pj_elapsed_cycle(start: *const pj_timestamp, stop: *const pj_timestamp) -> pj_uint32_t;
}
pub type pj_main_func_ptr = ::std::option::Option<
    unsafe extern "C" fn(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn pj_run_app(
        main_func: pj_main_func_ptr,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_thread_init() -> pj_status_t;
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
extern "C" {
    pub fn pj_pool_create(
        factory: *mut pj_pool_factory,
        name: *const ::std::os::raw::c_char,
        initial_size: pj_size_t,
        increment_size: pj_size_t,
        callback: pj_pool_callback,
    ) -> *mut pj_pool_t;
}
extern "C" {
    pub fn pj_pool_release(pool: *mut pj_pool_t);
}
extern "C" {
    pub fn pj_pool_safe_release(ppool: *mut *mut pj_pool_t);
}
extern "C" {
    pub fn pj_pool_secure_release(ppool: *mut *mut pj_pool_t);
}
extern "C" {
    pub fn pj_pool_getobjname(pool: *const pj_pool_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_pool_reset(pool: *mut pj_pool_t);
}
extern "C" {
    pub fn pj_pool_get_capacity(pool: *mut pj_pool_t) -> pj_size_t;
}
extern "C" {
    pub fn pj_pool_get_used_size(pool: *mut pj_pool_t) -> pj_size_t;
}
extern "C" {
    pub fn pj_pool_alloc(pool: *mut pj_pool_t, size: pj_size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_pool_calloc(
        pool: *mut pj_pool_t,
        count: pj_size_t,
        elem: pj_size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_pool_alloc_from_block(
        block: *mut pj_pool_block,
        size: pj_size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_pool_allocate_find(
        pool: *mut pj_pool_t,
        size: pj_size_t,
    ) -> *mut ::std::os::raw::c_void;
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
extern "C" {
    pub static mut PJ_NO_MEMORY_EXCEPTION: ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_NO_MEMORY_EXCEPTION() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut pj_pool_factory_default_policy: pj_pool_factory_policy;
}
extern "C" {
    pub fn pj_pool_factory_get_default_policy() -> *const pj_pool_factory_policy;
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
extern "C" {
    pub fn pj_pool_create_int(
        factory: *mut pj_pool_factory,
        name: *const ::std::os::raw::c_char,
        initial_size: pj_size_t,
        increment_size: pj_size_t,
        callback: pj_pool_callback,
    ) -> *mut pj_pool_t;
}
extern "C" {
    pub fn pj_pool_init_int(
        pool: *mut pj_pool_t,
        name: *const ::std::os::raw::c_char,
        increment_size: pj_size_t,
        callback: pj_pool_callback,
    );
}
extern "C" {
    pub fn pj_pool_destroy_int(pool: *mut pj_pool_t);
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
extern "C" {
    pub fn pj_caching_pool_init(
        ch_pool: *mut pj_caching_pool,
        policy: *const pj_pool_factory_policy,
        max_capacity: pj_size_t,
    );
}
extern "C" {
    pub fn pj_caching_pool_destroy(ch_pool: *mut pj_caching_pool);
}
extern "C" {
    pub fn pj_pool_create_on_buf(
        name: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_void,
        size: pj_size_t,
    ) -> *mut pj_pool_t;
}
extern "C" {
    pub fn pj_srand(seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn pj_rand() -> ::std::os::raw::c_int;
}
pub const PJ_RBCOLOR_BLACK: pj_rbcolor_t = 0;
pub const PJ_RBCOLOR_RED: pj_rbcolor_t = 1;
pub type pj_rbcolor_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_rbtree_node {
    pub parent: *mut pj_rbtree_node,
    pub left: *mut pj_rbtree_node,
    pub right: *mut pj_rbtree_node,
    pub key: *const ::std::os::raw::c_void,
    pub user_data: *mut ::std::os::raw::c_void,
    pub color: pj_rbcolor_t,
}
pub type pj_rbtree_comp = ::std::option::Option<
    unsafe extern "C" fn(
        key1: *const ::std::os::raw::c_void,
        key2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_rbtree {
    pub null_node: pj_rbtree_node,
    pub null: *mut pj_rbtree_node,
    pub root: *mut pj_rbtree_node,
    pub size: ::std::os::raw::c_uint,
    pub comp: pj_rbtree_comp,
}
extern "C" {
    pub fn pj_rbtree_init(tree: *mut pj_rbtree, comp: pj_rbtree_comp);
}
extern "C" {
    pub fn pj_rbtree_first(tree: *mut pj_rbtree) -> *mut pj_rbtree_node;
}
extern "C" {
    pub fn pj_rbtree_last(tree: *mut pj_rbtree) -> *mut pj_rbtree_node;
}
extern "C" {
    pub fn pj_rbtree_next(tree: *mut pj_rbtree, node: *mut pj_rbtree_node) -> *mut pj_rbtree_node;
}
extern "C" {
    pub fn pj_rbtree_prev(tree: *mut pj_rbtree, node: *mut pj_rbtree_node) -> *mut pj_rbtree_node;
}
extern "C" {
    pub fn pj_rbtree_insert(
        tree: *mut pj_rbtree,
        node: *mut pj_rbtree_node,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_rbtree_find(
        tree: *mut pj_rbtree,
        key: *const ::std::os::raw::c_void,
    ) -> *mut pj_rbtree_node;
}
extern "C" {
    pub fn pj_rbtree_erase(tree: *mut pj_rbtree, node: *mut pj_rbtree_node) -> *mut pj_rbtree_node;
}
extern "C" {
    pub fn pj_rbtree_max_height(
        tree: *mut pj_rbtree,
        node: *mut pj_rbtree_node,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_rbtree_min_height(
        tree: *mut pj_rbtree,
        node: *mut pj_rbtree_node,
    ) -> ::std::os::raw::c_uint;
}
pub const PJ_QOS_TYPE_BEST_EFFORT: pj_qos_type = 0;
pub const PJ_QOS_TYPE_BACKGROUND: pj_qos_type = 1;
pub const PJ_QOS_TYPE_VIDEO: pj_qos_type = 2;
pub const PJ_QOS_TYPE_VOICE: pj_qos_type = 3;
pub const PJ_QOS_TYPE_CONTROL: pj_qos_type = 4;
pub const PJ_QOS_TYPE_SIGNALLING: pj_qos_type = 5;
pub type pj_qos_type = u32;
pub const PJ_QOS_PARAM_HAS_DSCP: pj_qos_flag = 1;
pub const PJ_QOS_PARAM_HAS_SO_PRIO: pj_qos_flag = 2;
pub const PJ_QOS_PARAM_HAS_WMM: pj_qos_flag = 4;
pub type pj_qos_flag = u32;
pub const PJ_QOS_WMM_PRIO_BULK_EFFORT: pj_qos_wmm_prio = 0;
pub const PJ_QOS_WMM_PRIO_BULK: pj_qos_wmm_prio = 1;
pub const PJ_QOS_WMM_PRIO_VIDEO: pj_qos_wmm_prio = 2;
pub const PJ_QOS_WMM_PRIO_VOICE: pj_qos_wmm_prio = 3;
pub type pj_qos_wmm_prio = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_qos_params {
    pub flags: pj_uint8_t,
    pub dscp_val: pj_uint8_t,
    pub so_prio: pj_uint8_t,
    pub wmm_prio: pj_qos_wmm_prio,
}
extern "C" {
    pub fn pj_sock_set_qos_type(sock: pj_sock_t, type_: pj_qos_type) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_get_qos_type(sock: pj_sock_t, p_type: *mut pj_qos_type) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_apply_qos(
        sock: pj_sock_t,
        qos_type: pj_qos_type,
        qos_params: *mut pj_qos_params,
        log_level: ::std::os::raw::c_uint,
        log_sender: *const ::std::os::raw::c_char,
        sock_name: *const ::std::os::raw::c_char,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_apply_qos2(
        sock: pj_sock_t,
        qos_type: pj_qos_type,
        qos_params: *const pj_qos_params,
        log_level: ::std::os::raw::c_uint,
        log_sender: *const ::std::os::raw::c_char,
        sock_name: *const ::std::os::raw::c_char,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_qos_get_params(type_: pj_qos_type, p_param: *mut pj_qos_params) -> pj_status_t;
}
extern "C" {
    pub fn pj_qos_get_type(param: *const pj_qos_params, p_type: *mut pj_qos_type) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_set_qos_params(sock: pj_sock_t, param: *mut pj_qos_params) -> pj_status_t;
}
extern "C" {
    pub fn pj_sock_get_qos_params(sock: pj_sock_t, p_param: *mut pj_qos_params) -> pj_status_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pj_fd_set_t {
    pub data: [pj_sock_t; 68usize],
}
extern "C" {
    pub fn PJ_FD_ZERO(fdsetp: *mut pj_fd_set_t);
}
extern "C" {
    pub fn PJ_FD_COUNT(fdsetp: *const pj_fd_set_t) -> pj_size_t;
}
extern "C" {
    pub fn PJ_FD_SET(fd: pj_sock_t, fdsetp: *mut pj_fd_set_t);
}
extern "C" {
    pub fn PJ_FD_CLR(fd: pj_sock_t, fdsetp: *mut pj_fd_set_t);
}
extern "C" {
    pub fn PJ_FD_ISSET(fd: pj_sock_t, fdsetp: *const pj_fd_set_t) -> pj_bool_t;
}
extern "C" {
    pub fn pj_sock_select(
        n: ::std::os::raw::c_int,
        readfds: *mut pj_fd_set_t,
        writefds: *mut pj_fd_set_t,
        exceptfds: *mut pj_fd_set_t,
        timeout: *const pj_time_val,
    ) -> ::std::os::raw::c_int;
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
pub type pj_ssl_cert_verify_flag_t = i32;
pub const PJ_SSL_CERT_NAME_UNKNOWN: pj_ssl_cert_name_type = 0;
pub const PJ_SSL_CERT_NAME_RFC822: pj_ssl_cert_name_type = 1;
pub const PJ_SSL_CERT_NAME_DNS: pj_ssl_cert_name_type = 2;
pub const PJ_SSL_CERT_NAME_URI: pj_ssl_cert_name_type = 3;
pub const PJ_SSL_CERT_NAME_IP: pj_ssl_cert_name_type = 4;
pub type pj_ssl_cert_name_type = u32;
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
extern "C" {
    pub fn pj_ssl_cert_load_from_files(
        pool: *mut pj_pool_t,
        CA_file: *const pj_str_t,
        cert_file: *const pj_str_t,
        privkey_file: *const pj_str_t,
        privkey_pass: *const pj_str_t,
        p_cert: *mut *mut pj_ssl_cert_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_cert_load_from_files2(
        pool: *mut pj_pool_t,
        CA_file: *const pj_str_t,
        CA_path: *const pj_str_t,
        cert_file: *const pj_str_t,
        privkey_file: *const pj_str_t,
        privkey_pass: *const pj_str_t,
        p_cert: *mut *mut pj_ssl_cert_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_cert_load_from_buffer(
        pool: *mut pj_pool_t,
        CA_buf: *const pj_ssl_cert_buffer,
        cert_buf: *const pj_ssl_cert_buffer,
        privkey_buf: *const pj_ssl_cert_buffer,
        privkey_pass: *const pj_str_t,
        p_cert: *mut *mut pj_ssl_cert_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_cert_info_dump(
        ci: *const pj_ssl_cert_info,
        indent: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        buf_size: pj_size_t,
    ) -> pj_ssize_t;
}
extern "C" {
    pub fn pj_ssl_cert_get_verify_status_strings(
        verify_status: pj_uint32_t,
        error_strings: *mut *const ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_cert_wipe_keys(cert: *mut pj_ssl_cert_t);
}
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
pub type pj_ssl_cipher = i32;
extern "C" {
    pub fn pj_ssl_cipher_get_availables(
        ciphers: *mut pj_ssl_cipher,
        cipher_num: *mut ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_cipher_is_supported(cipher: pj_ssl_cipher) -> pj_bool_t;
}
extern "C" {
    pub fn pj_ssl_cipher_name(cipher: pj_ssl_cipher) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_ssl_cipher_id(cipher_name: *const ::std::os::raw::c_char) -> pj_ssl_cipher;
}
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
pub type pj_ssl_curve = u32;
extern "C" {
    pub fn pj_ssl_curve_get_availables(
        curves: *mut pj_ssl_curve,
        curve_num: *mut ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_curve_is_supported(curve: pj_ssl_curve) -> pj_bool_t;
}
extern "C" {
    pub fn pj_ssl_curve_name(curve: pj_ssl_curve) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pj_ssl_curve_id(curve_name: *const ::std::os::raw::c_char) -> pj_ssl_curve;
}
pub const PJ_SSL_ENTROPY_NONE: pj_ssl_entropy = 0;
pub const PJ_SSL_ENTROPY_EGD: pj_ssl_entropy = 1;
pub const PJ_SSL_ENTROPY_RANDOM: pj_ssl_entropy = 2;
pub const PJ_SSL_ENTROPY_URANDOM: pj_ssl_entropy = 3;
pub const PJ_SSL_ENTROPY_FILE: pj_ssl_entropy = 4;
pub const PJ_SSL_ENTROPY_UNKNOWN: pj_ssl_entropy = 15;
pub type pj_ssl_entropy = u32;
pub use self::pj_ssl_entropy as pj_ssl_entropy_t;
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
pub type pj_ssl_sock_proto = u32;
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
extern "C" {
    pub fn pj_ssl_sock_param_default(param: *mut pj_ssl_sock_param);
}
extern "C" {
    pub fn pj_ssl_sock_param_copy(
        pool: *mut pj_pool_t,
        dst: *mut pj_ssl_sock_param,
        src: *const pj_ssl_sock_param,
    );
}
extern "C" {
    pub fn pj_ssl_sock_create(
        pool: *mut pj_pool_t,
        param: *const pj_ssl_sock_param,
        p_ssock: *mut *mut pj_ssl_sock_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_set_certificate(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        cert: *const pj_ssl_cert_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_close(ssock: *mut pj_ssl_sock_t) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_set_user_data(
        ssock: *mut pj_ssl_sock_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_get_user_data(ssock: *mut pj_ssl_sock_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pj_ssl_sock_get_info(
        ssock: *mut pj_ssl_sock_t,
        info: *mut pj_ssl_sock_info,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_read(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_read2(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        readbuf: *mut *mut ::std::os::raw::c_void,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_recvfrom(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_recvfrom2(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        buff_size: ::std::os::raw::c_uint,
        readbuf: *mut *mut ::std::os::raw::c_void,
        flags: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_send(
        ssock: *mut pj_ssl_sock_t,
        send_key: *mut pj_ioqueue_op_key_t,
        data: *const ::std::os::raw::c_void,
        size: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_sendto(
        ssock: *mut pj_ssl_sock_t,
        send_key: *mut pj_ioqueue_op_key_t,
        data: *const ::std::os::raw::c_void,
        size: *mut pj_ssize_t,
        flags: ::std::os::raw::c_uint,
        addr: *const pj_sockaddr_t,
        addr_len: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_accept(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        local_addr: *const pj_sockaddr_t,
        addr_len: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_accept2(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        local_addr: *const pj_sockaddr_t,
        addr_len: ::std::os::raw::c_int,
        newsock_param: *const pj_ssl_sock_param,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_connect(
        ssock: *mut pj_ssl_sock_t,
        pool: *mut pj_pool_t,
        localaddr: *const pj_sockaddr_t,
        remaddr: *const pj_sockaddr_t,
        addr_len: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_start_connect2(
        ssock: *mut pj_ssl_sock_t,
        connect_param: *mut pj_ssl_start_connect_param,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_ssl_sock_renegotiate(ssock: *mut pj_ssl_sock_t) -> pj_status_t;
}
pub type pj_timer_id_t = ::std::os::raw::c_int;
pub type pj_timer_heap_callback = ::std::option::Option<
    unsafe extern "C" fn(timer_heap: *mut pj_timer_heap_t, entry: *mut pj_timer_entry),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_timer_entry {
    pub user_data: *mut ::std::os::raw::c_void,
    pub id: ::std::os::raw::c_int,
    pub cb: pj_timer_heap_callback,
    pub _timer_id: pj_timer_id_t,
}
extern "C" {
    pub fn pj_timer_heap_mem_size(count: pj_size_t) -> pj_size_t;
}
extern "C" {
    pub fn pj_timer_heap_create(
        pool: *mut pj_pool_t,
        count: pj_size_t,
        ht: *mut *mut pj_timer_heap_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_timer_heap_destroy(ht: *mut pj_timer_heap_t);
}
extern "C" {
    pub fn pj_timer_heap_set_lock(
        ht: *mut pj_timer_heap_t,
        lock: *mut pj_lock_t,
        auto_del: pj_bool_t,
    );
}
extern "C" {
    pub fn pj_timer_heap_set_max_timed_out_per_poll(
        ht: *mut pj_timer_heap_t,
        count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_timer_entry_init(
        entry: *mut pj_timer_entry,
        id: ::std::os::raw::c_int,
        user_data: *mut ::std::os::raw::c_void,
        cb: pj_timer_heap_callback,
    ) -> *mut pj_timer_entry;
}
extern "C" {
    pub fn pj_timer_entry_running(entry: *mut pj_timer_entry) -> pj_bool_t;
}
extern "C" {
    pub fn pj_timer_heap_schedule_dbg(
        ht: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
        delay: *const pj_time_val,
        src_file: *const ::std::os::raw::c_char,
        src_line: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_timer_heap_schedule_w_grp_lock_dbg(
        ht: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
        delay: *const pj_time_val,
        id_val: ::std::os::raw::c_int,
        grp_lock: *mut pj_grp_lock_t,
        src_file: *const ::std::os::raw::c_char,
        src_line: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_timer_heap_cancel(
        ht: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_timer_heap_cancel_if_active(
        ht: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
        id_val: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pj_timer_heap_count(ht: *mut pj_timer_heap_t) -> pj_size_t;
}
extern "C" {
    pub fn pj_timer_heap_earliest_time(
        ht: *mut pj_timer_heap_t,
        timeval: *mut pj_time_val,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pj_timer_heap_poll(
        ht: *mut pj_timer_heap_t,
        next_delay: *mut pj_time_val,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn pj_timer_heap_dump(ht: *mut pj_timer_heap_t);
}
extern "C" {
    pub fn pj_ansi_to_unicode(
        str_: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        wbuf: *mut wchar_t,
        wbuf_count: ::std::os::raw::c_int,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn pj_unicode_to_ansi(
        wstr: *const wchar_t,
        len: pj_ssize_t,
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
