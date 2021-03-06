#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use super::pjdefault::AutoCreate;
use super::pjdefault::ToString;
use super::pj_sys::*;

use std::ffi::CStr;
use std::ptr;

impl AutoCreate<pj_str_t> for pj_str_t {
    fn new() -> pj_str_t {
        pj_str_t {
            ptr: ptr::null_mut(),
            slen: 0,
        }
    }
}

impl ToString for pj_str_t {
    fn to_string(&self) -> String {

        let mut tmp: Vec<i8> = Vec::new();

        let ret: String = String::new();

        unsafe {
            let pointer = self.ptr;
            for i in 0..self.slen {
                tmp.push(*pointer.offset(i as isize).clone());
            }
            let ret = CStr::from_ptr(tmp.as_ptr()).to_str().expect("error").clone().to_string();
        }
        ret
    }
}

impl AutoCreate<pj_ice_sess_options> for pj_ice_sess_options {
    fn new() -> pj_ice_sess_options {
        pj_ice_sess_options {
            aggressive: PJ_FALSE as pj_bool_t,
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
            whole_data: PJ_FALSE as pj_bool_t,
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
            verify_peer: PJ_FALSE as pj_bool_t,
            require_client_cert: PJ_FALSE as pj_bool_t,
            server_name: pj_str_t::new(),
            reuse_addr: PJ_FALSE as pj_bool_t,
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            qos_ignore_error: PJ_FALSE as pj_bool_t,
            sockopt_params: pj_sockopt_params::new(),
            sockopt_ignore_error: PJ_FALSE as pj_bool_t,
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


