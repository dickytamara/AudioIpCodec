#![allow(non_camel_case_types, unused_variables)]

use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;
use std::os::raw::{c_char, c_void};
use std::mem;
use std::ptr;


impl AutoCreate<pjsip_cred_info> for pjsip_cred_info {
    fn new() -> pjsip_cred_info {
        let extmem: pjsip_cred_info__bindgen_ty_1 = unsafe { mem::zeroed() };
        pjsip_cred_info {
            realm: pj_str_t::new(),
            scheme: pj_str_t::new(),
            username: pj_str_t::new(),
            data_type: 0,
            data: pj_str_t::new(),
            ext: extmem,
        }
    }
}

impl AutoCreate<pjsip_hdr_vptr> for pjsip_hdr_vptr {
    fn new () -> pjsip_hdr_vptr {
        pjsip_hdr_vptr {
            clone: None,
            shallow_clone: None,
            print_on: None
        }
    }
}

impl AutoCreate<pjsip_hdr> for pjsip_hdr {
    fn new () -> pjsip_hdr {
        unsafe {
            let mut pjsip_hdr_z: pjsip_hdr = mem::zeroed();
            let mut pjsip_hdr_vptr_z: pjsip_hdr_vptr = mem::zeroed();
            pjsip_hdr {
                prev: &mut pjsip_hdr_z as *mut _,
                next: &mut pjsip_hdr_z as *mut _,
                type_: 0,
                name: pj_str_t::new(),
                sname: pj_str_t::new(),
                vptr: &mut pjsip_hdr_vptr_z as *mut _,
            }
        }
    }
}


impl AutoCreate<pjsip_tls_setting> for pjsip_tls_setting {
    fn new () -> pjsip_tls_setting {
        unsafe {
            let mut pj_ssl_cipher_z: pj_ssl_cipher = mem::zeroed();
            let mut pj_ssl_curve_z: pj_ssl_curve = mem::zeroed();
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
                ciphers: &mut pj_ssl_cipher_z as *mut _,
                curves_num: 0,
                curves: &mut pj_ssl_curve_z as *mut _,
                sigalgs: pj_str_t::new(),
                entropy_type: 0,
                entropy_path: pj_str_t::new(),
                verify_server: pj_constants__PJ_FALSE as pj_bool_t,
                verify_client: pj_constants__PJ_FALSE as pj_bool_t,
                require_client_cert: pj_constants__PJ_FALSE as pj_bool_t,
                timeout: pj_time_val::new(),
                reuse_addr: pj_constants__PJ_FALSE as pj_bool_t,
                qos_type: 0,
                qos_params: pj_qos_params::new(),
                qos_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
                sockopt_params: pj_sockopt_params::new(),
                sockopt_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
                on_accept_fail_cb: None
            }
        }
    }
}


pub trait PjsipModuleCallback {
    unsafe extern "C" fn start () -> pj_status_t { 0 }
    unsafe extern "C" fn stop () -> pj_status_t { 0 }
    unsafe extern "C" fn unload () -> pj_status_t { 0 }
    unsafe extern "C" fn on_rx_request (rdata: *mut pjsip_rx_data) -> pj_bool_t { 0 }
    unsafe extern "C" fn on_rx_response (rdata: *mut pjsip_rx_data) -> pj_bool_t { 0 }
    unsafe extern "C" fn on_tx_request (tdata: *mut pjsip_tx_data) -> pj_status_t { 0 }
    unsafe extern "C" fn on_tx_response (tdata: *mut pjsip_tx_data) -> pj_status_t { 0 }
    unsafe extern "C" fn on_tsx_state (tsx: *mut pjsip_transaction, event: *mut pjsip_event) {  }
}


impl AutoCreate<pjsip_module> for pjsip_module {
    fn new () -> pjsip_module {
        unsafe {
            let mut endpt: pjsip_module = mem::zeroed();
            pjsip_module {
                prev: &mut endpt as *mut _,
                next: &mut endpt as *mut _,
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
}


impl AutoCreate<pjsip_tx_data_op_key> for pjsip_tx_data_op_key {
    fn new () -> pjsip_tx_data_op_key {
        unsafe {
            let mut tx_data: pjsip_tx_data = mem::zeroed();
            let mut void: c_void = mem::zeroed();

            pjsip_tx_data_op_key {
                key: pj_ioqueue_op_key_t::new(),
                tdata: &mut tx_data as *mut _,
                token: &mut void as *mut _,
                callback: None
            }
        }
    }
}


impl AutoCreate<pjsip_buffer> for pjsip_buffer {
    fn new () -> pjsip_buffer {
        unsafe {
            let mut c: c_char = mem::zeroed();
            pjsip_buffer {
                start: &mut c as *mut _,
                cur: &mut c as *mut _,
                end: &mut c as *mut _
            }
        }
    }
}

impl AutoCreate<pjsip_server_addresses__bindgen_ty_1> for pjsip_server_addresses__bindgen_ty_1 {
    fn new () -> pjsip_server_addresses__bindgen_ty_1 {
        pjsip_server_addresses__bindgen_ty_1 {
            type_: 0,
            priority: 0,
            weight: 0,
            addr: pj_sockaddr::new(),
            addr_len: 0
        }
    }
}

impl AutoCreate<pjsip_server_addresses> for pjsip_server_addresses {
    fn new () -> pjsip_server_addresses {
        pjsip_server_addresses {
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
    fn new () -> pjsip_tx_data__bindgen_ty_1 {
        pjsip_tx_data__bindgen_ty_1 {
            name: pj_str_t::new(),
            addr: pjsip_server_addresses::new(),
            cur_addr: 0x00000000,
        }
    }
}


impl AutoCreate<pjsip_tx_data__bindgen_ty_2> for pjsip_tx_data__bindgen_ty_2 {
    fn new () -> pjsip_tx_data__bindgen_ty_2 {
        unsafe {
            let mut transport: pjsip_transport = mem::zeroed();
            pjsip_tx_data__bindgen_ty_2 {
                transport: &mut transport as *mut _,
                dst_addr: pj_sockaddr::new(),
                dst_addr_len: 0,
                dst_name: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                           0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                           0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                           0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                           0x0, 0x0, 0x0, 0x0, 0x0, 0x0
                ],
                dst_port: 0

            }
        }
    }
}


impl AutoCreate<pjsip_tpselector__bindgen_ty_1> for pjsip_tpselector__bindgen_ty_1 {
    fn new () -> pjsip_tpselector__bindgen_ty_1 {
        unsafe {
            let mut void: c_void = mem::zeroed();
            pjsip_tpselector__bindgen_ty_1 {
                ptr: &mut void as *mut _,
            }
        }
    }
}


impl AutoCreate<pjsip_tpselector> for pjsip_tpselector {
    fn new () -> pjsip_tpselector {
        pjsip_tpselector {
            type_: 0,
            disable_connection_reuse: pj_constants__PJ_FALSE as pj_bool_t,
            u: pjsip_tpselector__bindgen_ty_1::new(),
        }
    }
}


impl AutoCreate<pjsip_host_port> for pjsip_host_port {
    fn new() -> pjsip_host_port {
        pjsip_host_port {
            host: pj_str_t::new(),
            port: 0
        }
    }
}

impl AutoCreate<pjsip_tx_data> for pjsip_tx_data {
    fn new () -> pjsip_tx_data {
        unsafe {
            let mut tx_data: pjsip_tx_data = mem::zeroed();
            let mut pool: pj_pool_t = mem::zeroed();
            let mut c: c_char = mem::zeroed();
            let mut v: c_void = mem::zeroed();
            let mut tpmgr: pjsip_tpmgr = mem::zeroed();
            let mut lock: pj_lock_t = mem::zeroed();
            let mut msg: pjsip_msg = mem::zeroed();
            let mut route_hdr: pjsip_route_hdr = mem::zeroed();
            let mut atomic: pj_atomic_t = mem::zeroed();

            pjsip_tx_data {
                prev: &mut tx_data as *mut _,
                next: &mut tx_data as *mut _,
                pool: &mut pool as *mut _,
                obj_name: [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,0x00,0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,0x00,0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,0x00,0x00,
                            0x00, 0x00 ],
                info: &mut c as * mut _,
                rx_timestamp: pj_time_val::new(),
                mgr: &mut tpmgr as *mut _,
                op_key: pjsip_tx_data_op_key::new(),
                lock: &mut lock as *mut _,
                msg: &mut msg as *mut _,
                saved_strict_route: &mut route_hdr as *mut _,
                buf: pjsip_buffer::new(),
                ref_cnt: &mut atomic as *mut _,
                is_pending: 0,
                token: &mut v as *mut _,
                cb: None,
                dest_info: pjsip_tx_data__bindgen_ty_1::new(),
                tp_info: pjsip_tx_data__bindgen_ty_2::new(),
                tp_sel: pjsip_tpselector::new(),
                auth_retry: pj_constants__PJ_FALSE as pj_bool_t,
                mod_data: [&mut v as *mut _, &mut v as *mut _,&mut v as *mut _,&mut v as *mut _,&mut v as *mut _,
                           &mut v as *mut _, &mut v as *mut _,&mut v as *mut _,&mut v as *mut _,&mut v as *mut _,
                           &mut v as *mut _, &mut v as *mut _,&mut v as *mut _,&mut v as *mut _,&mut v as *mut _,
                           &mut v as *mut _, &mut v as *mut _,&mut v as *mut _,&mut v as *mut _,&mut v as *mut _,
                           &mut v as *mut _, &mut v as *mut _,&mut v as *mut _,&mut v as *mut _,&mut v as *mut _,
                           &mut v as *mut _, &mut v as *mut _,&mut v as *mut _,&mut v as *mut _,&mut v as *mut _,
                           &mut v as *mut _, &mut v as *mut _],
                via_addr: pjsip_host_port::new(),
                via_tp: &mut v as *mut _
            }
        }
    }
}


impl AutoCreate<pjsip_generic_string_hdr> for pjsip_generic_string_hdr {
    fn new () -> pjsip_generic_string_hdr {
        unsafe {
            let generic_string_hdr: *mut pjsip_generic_string_hdr = mem::zeroed();
            let hdr: *const  pjsip_hdr = ptr::null();
            pjsip_generic_string_hdr {
              prev: generic_string_hdr,
              next: generic_string_hdr,
              type_: 0,
              name: pj_str_t::new(),
              sname: pj_str_t::new(),
              vptr: hdr as *mut _,
              hvalue: pj_str_t::new()
            }
        }
    }
}


