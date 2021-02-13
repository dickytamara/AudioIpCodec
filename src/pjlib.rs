

//mod pjproject_sys;
use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;
use std::os::raw::{c_char, c_void};
use std::mem;


impl AutoCreate<pj_str_t>  for pj_str_t {
    fn new () -> pj_str_t {
        unsafe {
            let zeromem: *mut c_char  = mem::zeroed() ;
            pj_str(zeromem)
        }
    }
}


impl AutoCreate<pj_ice_sess_options> for pj_ice_sess_options {
    fn new () -> pj_ice_sess_options {
        pj_ice_sess_options{
            aggressive: pj_constants__PJ_FALSE  as pj_bool_t,
            nominated_check_delay: 0,
            controlled_agent_want_nom_timeout: 0,
            trickle: 0 as pj_ice_sess_trickle,
        }
    }
}


impl AutoCreate<pj_time_val> for pj_time_val {
    fn new () -> pj_time_val {
        pj_time_val {
            sec: 0,
            msec: 0,
        }
    }
}

impl AutoCreate<pj_qos_params> for pj_qos_params {
    fn new () -> pj_qos_params {
        pj_qos_params {
            flags: 0,
            dscp_val: 0,
            so_prio: 0,
            wmm_prio: 0,
        }
    }
}


impl AutoCreate<pj_sockopt_params__bindgen_ty_1> for pj_sockopt_params__bindgen_ty_1 {
    fn new () -> pj_sockopt_params__bindgen_ty_1 {
        unsafe {
            let mut void: c_void = mem::zeroed();
            pj_sockopt_params__bindgen_ty_1 {
                level: 0,
                optname: 0,
                optval: &mut void as *mut _,
                optlen: 0
            }
        }
    }
}

impl AutoCreate<pj_sockopt_params> for pj_sockopt_params {
    fn new () -> pj_sockopt_params {
        pj_sockopt_params {
            cnt: 0,
            options: [pj_sockopt_params__bindgen_ty_1::new(); 4]
                     
        }
    }
}

impl AutoCreate<pj_ssl_sock_cb> for pj_ssl_sock_cb {
    fn new () -> pj_ssl_sock_cb {
        pj_ssl_sock_cb {
            on_data_read: None,
                on_data_recvfrom: None,
                on_data_sent: None,
                on_accept_complete: None,
                on_accept_complete2: None,
                on_connect_complete: None
        }
    }
}


impl AutoCreate<pj_ssl_sock_param> for pj_ssl_sock_param {
    fn new () -> pj_ssl_sock_param {
        unsafe {
        let mut pj_grp_lock_z: pj_grp_lock_t = mem::zeroed();
        let mut pj_ioqueue_z: pj_ioqueue_t = mem::zeroed();
        let mut pj_timer_heap_z: pj_timer_heap_t = mem::zeroed();
        let mut pj_ssl_cipher_z: pj_ssl_cipher = mem::zeroed();
        let mut pj_ssl_curve_z: pj_ssl_curve = mem::zeroed();
        let mut void: c_void = mem::zeroed();
        pj_ssl_sock_param {
            grp_lock:  &mut pj_grp_lock_z as *mut _,
            sock_af: 0,
            sock_type: 0,
            ioqueue:  &mut pj_ioqueue_z as *mut _,
            timer_heap: &mut pj_timer_heap_z as *mut _,
            cb: pj_ssl_sock_cb::new(),
            user_data: &mut void as *mut _,
            proto: 0 as pj_uint32_t,
            async_cnt: 0,
            concurrency: 0,
            whole_data: pj_constants__PJ_FALSE as pj_bool_t,
            send_buffer_size: 0,
            read_buffer_size: 0,
            ciphers_num: 0,
            ciphers: &mut pj_ssl_cipher_z as *mut _,
            curves_num: 0,
            curves: &mut pj_ssl_curve_z as *mut _,
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
        }}
    }
}

impl AutoCreate<pj_turn_sock_tls_cfg> for pj_turn_sock_tls_cfg {
    fn new () -> pj_turn_sock_tls_cfg {
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

impl AutoCreate<pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1> for pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 {
    fn new () -> pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 {
        pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1 {
            realm: pj_str_t::new(),
            username: pj_str_t::new(),
            data_type: 0,
            data: pj_str_t::new(),
            nonce: pj_str_t::new()
        }
    }
}

impl AutoCreate<pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2> for pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 {
    fn new () -> pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 {
        unsafe {
            let mut void = mem::zeroed();
            pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2 {
                user_data: &mut void as *mut _,
                get_auth: None,
                get_cred: None,
                get_password: None,
                verify_nonce: None
            }
        }
    }
}

impl AutoCreate<pj_stun_auth_cred__bindgen_ty_1> for pj_stun_auth_cred__bindgen_ty_1 {
    fn new () -> pj_stun_auth_cred__bindgen_ty_1 {
        pj_stun_auth_cred__bindgen_ty_1 {
            static_cred: pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pj_stun_auth_cred> for pj_stun_auth_cred {
    fn new () -> pj_stun_auth_cred {
        pj_stun_auth_cred {
            type_ : 0,
            data : pj_stun_auth_cred__bindgen_ty_1::new()
        }
    }
}


impl AutoCreate<pj_ioqueue_op_key_t> for pj_ioqueue_op_key_t {
    fn new () -> pj_ioqueue_op_key_t {
        unsafe {
            let mut c: c_void = mem::zeroed();
            pj_ioqueue_op_key_t {
                internal__: [ &mut c as *mut _; 32],
                activesock_data: &mut c as *mut _,
                user_data: &mut c as *mut _,
            }
        }
    }
}

impl AutoCreate<pj_sockaddr_in> for pj_sockaddr_in {
    fn new () -> pj_sockaddr_in {
        pj_sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr{ s_addr: 0 } as  pj_in_addr,
            sin_zero_pad: [0; 8]
        }
    }
}

impl AutoCreate<pj_sockaddr> for pj_sockaddr {
    fn new () -> pj_sockaddr {
        pj_sockaddr {
            ipv4: pj_sockaddr_in::new()
        }
    }
}

impl AutoCreate<pjrpid_element> for pjrpid_element {
    fn new () -> pjrpid_element {
        pjrpid_element{
            type_: 0,
            id: pj_str_t::new(),
            activity: 0,
            note: pj_str_t::new(),
        }
    }
}


