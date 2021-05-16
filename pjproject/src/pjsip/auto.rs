use pj_sys::*;
use pjsip_simple_sys::*;
use pjsip_sys::*;

use super::utils::AutoCreate;
use super::*;

impl AutoCreate<pjsip_cred_info__bindgen_ty_1__bindgen_ty_1> for pjsip_cred_info__bindgen_ty_1__bindgen_ty_1
{
fn new() -> Self {
    Self {
        k: pj_str_t::new(),
        op: pj_str_t::new(),
        amf: pj_str_t::new(),
        cb: None,
    }
}
}

impl AutoCreate<pjsip_cred_info__bindgen_ty_1> for pjsip_cred_info__bindgen_ty_1 {
fn new() -> Self {
    let mut result = Self {
        aka: pjsip_sys::__BindgenUnionField::<pjsip_cred_info__bindgen_ty_1__bindgen_ty_1>::default(),
        bindgen_union_field: [0; 7]
    };

    unsafe {
        *result.aka.as_mut() = pjsip_cred_info__bindgen_ty_1__bindgen_ty_1::new();
    }

    result
}
}

impl AutoCreate<pjsip_cred_info> for pjsip_cred_info {
fn new() -> Self {
    Self {
        realm: pj_str_t::new(),
        scheme: pj_str_t::new(),
        username: pj_str_t::new(),
        data_type: 0,
        data: pj_str_t::new(),
        ext: pjsip_cred_info__bindgen_ty_1::new(),
    }
}
}

impl AutoCreate<pjsip_hdr_vptr> for pjsip_hdr_vptr {
fn new() -> Self {
    Self {
        clone: None,
        shallow_clone: None,
        print_on: None,
    }
}
}

impl AutoCreate<pjsip_hdr> for pjsip_hdr {
fn new() -> Self {
    Self {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        type_: 0,
        name: pj_str_t::new(),
        sname: pj_str_t::new(),
        vptr: ptr::null_mut(),
    }
}
}

impl AutoCreate<pjsip_tls_setting> for pjsip_tls_setting {
fn new() -> Self {
    Self {
        ca_list_file: pj_str_t::new(),
        ca_list_path: pj_str_t::new(),
        cert_file: pj_str_t::new(),
        privkey_file: pj_str_t::new(),
        ca_buf: pj_str_t::new() as pj_ssl_cert_buffer,
        cert_buf: pj_str_t::new() as pj_ssl_cert_buffer,
        privkey_buf: pj_str_t::new() as pj_ssl_cert_buffer,
        password: pj_str_t::new(),
        method: 0,
        proto: 0,
        ciphers_num: 0,
        ciphers: ptr::null_mut(),
        curves_num: 0,
        curves: ptr::null_mut(),
        sigalgs: pj_str_t::new(),
        entropy_type: 0,
        entropy_path: pj_str_t::new(),
        verify_server: 0,
        verify_client: 0,
        require_client_cert: 0,
        timeout: pj_time_val::new(),
        reuse_addr: 0,
        qos_type: 0,
        qos_params: pj_qos_params::new(),
        qos_ignore_error: 0,
        sockopt_params: pj_sockopt_params::new(),
        sockopt_ignore_error: 0,
        on_accept_fail_cb: None,
    }
}
}

impl AutoCreate<SIPModule> for SIPModule {
fn new() -> Self {
    // unsafe { std::mem::zeroed() }
    Self {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        name: pj_str_t::new(),
        id: -1,
        priority: (PJSIP_MOD_PRIORITY_APPLICATION + 99) as i32,
        load: None,
        start: None,
        stop: None,
        unload: None,
        on_rx_request: None,
        on_rx_response: None,
        on_tx_request: None,
        on_tx_response: None,
        on_tsx_state: None,
    }
}
}

impl AutoCreate<pjsip_tx_data_op_key> for pjsip_tx_data_op_key {
fn new() -> Self {
    Self {
        key: pj_ioqueue_op_key_t::new(),
        tdata: ptr::null_mut(),
        token: ptr::null_mut(),
        callback: None,
    }
}
}

impl AutoCreate<pjsip_buffer> for pjsip_buffer {
fn new() -> Self {
    Self {
        start: ptr::null_mut(),
        cur: ptr::null_mut(),
        end: ptr::null_mut(),
    }
}
}

impl AutoCreate<pjsip_server_addresses__bindgen_ty_1> for pjsip_server_addresses__bindgen_ty_1 {
fn new() -> Self {
    Self {
        type_: 0,
        priority: 0,
        weight: 0,
        addr: pj_sockaddr::new(),
        addr_len: 0,
    }
}
}

impl AutoCreate<pjsip_server_addresses> for pjsip_server_addresses {
fn new() -> Self {
    Self {
        count: 0,
        entry: [
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
            pjsip_server_addresses__bindgen_ty_1::new(), pjsip_server_addresses__bindgen_ty_1::new(),
        ],
    }
}
}

impl AutoCreate<pjsip_tx_data__bindgen_ty_1> for pjsip_tx_data__bindgen_ty_1 {
fn new() -> Self {
    Self {
        name: pj_str_t::new(),
        addr: pjsip_server_addresses::new(),
        cur_addr: 0x00000000,
    }
}
}

impl AutoCreate<pjsip_tx_data__bindgen_ty_2> for pjsip_tx_data__bindgen_ty_2 {
fn new() -> Self {
    Self {
        transport: ptr::null_mut(),
        dst_addr: pj_sockaddr::new(),
        dst_addr_len: 0,
        dst_name: [0x0; 46],
        dst_port: 0,
    }
}
}

impl AutoCreate<pjsip_tpselector__bindgen_ty_1> for pjsip_tpselector__bindgen_ty_1 {
fn new() -> Self {
    Self {
        ptr: ptr::null_mut(),
    }
}
}

impl AutoCreate<pjsip_tpselector> for pjsip_tpselector {
fn new() -> Self {
    Self {
        type_: 0,
        disable_connection_reuse: PJ_FALSE as pj_bool_t,
        u: pjsip_tpselector__bindgen_ty_1::new(),
    }
}
}

impl AutoCreate<pjsip_host_port> for pjsip_host_port {
fn new() -> Self {
    Self {
        host: pj_str_t::new(),
        port: 0,
    }
}
}

impl AutoCreate<pjsip_tx_data> for pjsip_tx_data {
fn new() -> Self {
    Self {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        pool: ptr::null_mut(),
        obj_name: [0x00; 32],
        info: ptr::null_mut(),
        rx_timestamp: pj_time_val::new(),
        mgr: ptr::null_mut(),
        op_key: pjsip_tx_data_op_key::new(),
        lock: ptr::null_mut(),
        msg: ptr::null_mut(),
        saved_strict_route: ptr::null_mut(),
        buf: pjsip_buffer::new(),
        ref_cnt: ptr::null_mut(),
        is_pending: 0,
        token: ptr::null_mut(),
        cb: None,
        dest_info: pjsip_tx_data__bindgen_ty_1::new(),
        tp_info: pjsip_tx_data__bindgen_ty_2::new(),
        tp_sel: pjsip_tpselector::new(),
        auth_retry: 0,
        mod_data: [ptr::null_mut(); 32],
        via_addr: pjsip_host_port::new(),
        via_tp: ptr::null_mut(),
    }
}
}

impl AutoCreate<pjsip_generic_string_hdr> for pjsip_generic_string_hdr {
fn new() -> Self {
    Self {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        type_: 0,
        name: pj_str_t::new(),
        sname: pj_str_t::new(),
        vptr: ptr::null_mut(),
        hvalue: pj_str_t::new(),
    }
}
}

impl AutoCreate<pjsip_pres_status__bindgen_ty_1> for pjsip_pres_status__bindgen_ty_1 {
fn new() -> Self {
    Self {
        basic_open: PJ_FALSE as pj_bool_t,
        rpid: pjrpid_element::new(),
        id: pj_str_t::new(),
        contact: pj_str_t::new(),
        tuple_node: ptr::null_mut(),
    }
}
}

impl AutoCreate<pjsip_pres_status> for pjsip_pres_status {
fn new() -> Self {
    Self {
        info_cnt: 0,
        info: [
            pjsip_pres_status__bindgen_ty_1::new(), pjsip_pres_status__bindgen_ty_1::new(),
            pjsip_pres_status__bindgen_ty_1::new(), pjsip_pres_status__bindgen_ty_1::new(),
            pjsip_pres_status__bindgen_ty_1::new(), pjsip_pres_status__bindgen_ty_1::new(),
            pjsip_pres_status__bindgen_ty_1::new(), pjsip_pres_status__bindgen_ty_1::new(),
        ],
        _is_valid: PJ_FALSE as pj_bool_t,
    }
}
}

impl AutoCreate<pjsip_param> for pjsip_param {
fn new() -> Self {
    Self {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        name: pj_str_t::new(),
        value: pj_str_t::new(),
    }
}
}

impl AutoCreate<pjsip_multipart_part> for pjsip_multipart_part {
fn new() -> Self {
    Self {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        hdr: pjsip_hdr::new(),
        body: ptr::null_mut(),
    }
}
}

impl AutoCreate<pjsip_media_type> for pjsip_media_type {
fn new() -> Self {
    Self {
        type_: pj_str_t::new(),
        subtype: pj_str_t::new(),
        param: pjsip_param::new(),
    }
}
}

impl AutoCreate<pjsip_publishc_opt> for pjsip_publishc_opt {
fn new() -> Self {
    Self {
        queue_request: 0,
    }
}
}

impl AutoCreate<pjsip_auth_clt_pref> for pjsip_auth_clt_pref {
fn new() -> pjsip_auth_clt_pref {
    pjsip_auth_clt_pref {
        initial_auth: 0,
        algorithm: pj_str_t::new(),
    }
}
}