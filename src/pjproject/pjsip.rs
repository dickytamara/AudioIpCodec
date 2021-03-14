#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use super::pjdefault::AutoCreate;
use super::pj_sys::*;
use super::pjsip_sys::*;

use std::ptr;


impl AutoCreate<pjsip_cred_info__bindgen_ty_1__bindgen_ty_1>
    for pjsip_cred_info__bindgen_ty_1__bindgen_ty_1
{
    fn new() -> pjsip_cred_info__bindgen_ty_1__bindgen_ty_1 {
        pjsip_cred_info__bindgen_ty_1__bindgen_ty_1 {
            k: pj_str_t::new(),
            op: pj_str_t::new(),
            amf: pj_str_t::new(),
            cb: None,
        }
    }
}

impl AutoCreate<pjsip_cred_info__bindgen_ty_1> for pjsip_cred_info__bindgen_ty_1 {
    fn new() -> pjsip_cred_info__bindgen_ty_1 {
        pjsip_cred_info__bindgen_ty_1 {
            aka: pjsip_cred_info__bindgen_ty_1__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsip_cred_info> for pjsip_cred_info {
    fn new() -> pjsip_cred_info {
        pjsip_cred_info {
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
    fn new() -> pjsip_hdr_vptr {
        pjsip_hdr_vptr {
            clone: None,
            shallow_clone: None,
            print_on: None,
        }
    }
}

impl AutoCreate<pjsip_hdr> for pjsip_hdr {
    fn new() -> pjsip_hdr {
        pjsip_hdr {
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
    fn new() -> pjsip_tls_setting {
        pjsip_tls_setting {
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
            verify_server: PJ_FALSE as pj_bool_t,
            verify_client: PJ_FALSE as pj_bool_t,
            require_client_cert: PJ_FALSE as pj_bool_t,
            timeout: pj_time_val::new(),
            reuse_addr: PJ_FALSE as pj_bool_t,
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            qos_ignore_error: PJ_FALSE as pj_bool_t,
            sockopt_params: pj_sockopt_params::new(),
            sockopt_ignore_error: PJ_FALSE as pj_bool_t,
            on_accept_fail_cb: None,
        }
    }
}

pub trait PjsipModuleCallback {
    unsafe extern "C" fn start() -> pj_status_t {
        0
    }
    unsafe extern "C" fn stop() -> pj_status_t {
        0
    }
    unsafe extern "C" fn unload() -> pj_status_t {
        0
    }
    unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_bool_t {
        0
    }
    unsafe extern "C" fn on_rx_response(rdata: *mut pjsip_rx_data) -> pj_bool_t {
        0
    }
    unsafe extern "C" fn on_tx_request(tdata: *mut pjsip_tx_data) -> pj_status_t {
        0
    }
    unsafe extern "C" fn on_tx_response(tdata: *mut pjsip_tx_data) -> pj_status_t {
        0
    }
    unsafe extern "C" fn on_tsx_state(tsx: *mut pjsip_transaction, event: *mut pjsip_event) {}
}

impl AutoCreate<pjsip_module> for pjsip_module {
    fn new() -> pjsip_module {
        pjsip_module {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            name: pj_str_t::new(),
            id: 0,
            priority: 0,
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
    fn new() -> pjsip_tx_data_op_key {
        pjsip_tx_data_op_key {
            key: pj_ioqueue_op_key_t::new(),
            tdata: ptr::null_mut(),
            token: ptr::null_mut(),
            callback: None,
        }
    }
}

impl AutoCreate<pjsip_buffer> for pjsip_buffer {
    fn new() -> pjsip_buffer {
        pjsip_buffer {
            start: ptr::null_mut(),
            cur: ptr::null_mut(),
            end: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_server_addresses__bindgen_ty_1> for pjsip_server_addresses__bindgen_ty_1 {
    fn new() -> pjsip_server_addresses__bindgen_ty_1 {
        pjsip_server_addresses__bindgen_ty_1 {
            type_: 0,
            priority: 0,
            weight: 0,
            addr: pj_sockaddr::new(),
            addr_len: 0,
        }
    }
}

impl AutoCreate<pjsip_server_addresses> for pjsip_server_addresses {
    fn new() -> pjsip_server_addresses {
        pjsip_server_addresses {
            count: 0,
            entry: [pjsip_server_addresses__bindgen_ty_1::new(); 16],
        }
    }
}

impl AutoCreate<pjsip_tx_data__bindgen_ty_1> for pjsip_tx_data__bindgen_ty_1 {
    fn new() -> pjsip_tx_data__bindgen_ty_1 {
        pjsip_tx_data__bindgen_ty_1 {
            name: pj_str_t::new(),
            addr: pjsip_server_addresses::new(),
            cur_addr: 0x00000000,
        }
    }
}

impl AutoCreate<pjsip_tx_data__bindgen_ty_2> for pjsip_tx_data__bindgen_ty_2 {
    fn new() -> pjsip_tx_data__bindgen_ty_2 {
        pjsip_tx_data__bindgen_ty_2 {
            transport: ptr::null_mut(),
            dst_addr: pj_sockaddr::new(),
            dst_addr_len: 0,
            dst_name: [0x0; 46],
            dst_port: 0,
        }
    }
}

impl AutoCreate<pjsip_tpselector__bindgen_ty_1> for pjsip_tpselector__bindgen_ty_1 {
    fn new() -> pjsip_tpselector__bindgen_ty_1 {
        pjsip_tpselector__bindgen_ty_1 {
            ptr: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_tpselector> for pjsip_tpselector {
    fn new() -> pjsip_tpselector {
        pjsip_tpselector {
            type_: 0,
            disable_connection_reuse: PJ_FALSE as pj_bool_t,
            u: pjsip_tpselector__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsip_host_port> for pjsip_host_port {
    fn new() -> pjsip_host_port {
        pjsip_host_port {
            host: pj_str_t::new(),
            port: 0,
        }
    }
}

impl AutoCreate<pjsip_tx_data> for pjsip_tx_data {
    fn new() -> pjsip_tx_data {
        pjsip_tx_data {
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
            auth_retry: PJ_FALSE as pj_bool_t,
            mod_data: [ptr::null_mut(); 32],
            via_addr: pjsip_host_port::new(),
            via_tp: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_generic_string_hdr> for pjsip_generic_string_hdr {
    fn new() -> pjsip_generic_string_hdr {
        pjsip_generic_string_hdr {
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
    fn new() -> pjsip_pres_status__bindgen_ty_1 {
        pjsip_pres_status__bindgen_ty_1 {
            basic_open: PJ_FALSE as pj_bool_t,
            rpid: pjrpid_element::new(),
            id: pj_str_t::new(),
            contact: pj_str_t::new(),
            tuple_node: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_pres_status> for pjsip_pres_status {
    fn new() -> pjsip_pres_status {
        pjsip_pres_status {
            info_cnt: 0,
            info: [pjsip_pres_status__bindgen_ty_1::new(); 8],
            _is_valid: PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pjsip_param> for pjsip_param {
    fn new() -> pjsip_param {
        pjsip_param {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            name: pj_str_t::new(),
            value: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjsip_multipart_part> for pjsip_multipart_part {
    fn new() -> pjsip_multipart_part {
        pjsip_multipart_part {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            hdr: pjsip_hdr::new(),
            body: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_media_type> for pjsip_media_type {
    fn new() -> pjsip_media_type {
        pjsip_media_type {
            type_: pj_str_t::new(),
            subtype: pj_str_t::new(),
            param: pjsip_param::new(),
        }
    }
}



