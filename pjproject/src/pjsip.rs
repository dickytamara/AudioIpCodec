#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]


use pj_sys::*;
use pjlib_util_sys::*;
use pjsip_simple_sys::*;
use pjsip_sys::*;

use super::{prelude::*, utils::check_status};

use std::{ffi::CString, ptr};


impl AutoCreate<pjsip_cred_info__bindgen_ty_1__bindgen_ty_1>
    for pjsip_cred_info__bindgen_ty_1__bindgen_ty_1
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

impl AutoCreate<pjsip_module> for pjsip_module {
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

// function helper
// const pjsip_method * 	pjsip_get_invite_method (void)
// const pjsip_method * 	pjsip_get_cancel_method (void)
// const pjsip_method * 	pjsip_get_ack_method (void)
// const pjsip_method * 	pjsip_get_bye_method (void)
// const pjsip_method * 	pjsip_get_register_method (void)
// const pjsip_method * 	pjsip_get_options_method (void)
// void 	pjsip_method_init (pjsip_method *m, pj_pool_t *pool, const pj_str_t *str)
// void 	pjsip_method_init_np (pjsip_method *m, pj_str_t *str)
// void 	pjsip_method_set (pjsip_method *m, pjsip_method_e id)
// void 	pjsip_method_copy (pj_pool_t *pool, pjsip_method *method, const pjsip_method *rhs)
// int 	pjsip_method_cmp (const pjsip_method *m1, const pjsip_method *m2)
pub fn method_cmp(m1: &pjsip_method, m2: &pjsip_method) -> i32 {
    unsafe {
        pjsip_method_cmp(m1 as *const _, m2 as *const _)
    }
}

pub fn endpt_create(pf: *mut pj_pool_factory, name: String, endpt: &mut Box<*mut pjsip_endpoint>) -> Result<(), i32> {
    let name = CString::new(name.as_str()).expect("pjsip_endpt_create").into_raw();
    unsafe { check_status( pjsip_endpt_create(pf, name as *const _, endpt.as_mut() as *mut _)) }
}

pub fn endpt_destroy(endpt: &mut pjsip_endpoint) {
    unsafe { pjsip_endpt_destroy(endpt as *mut _); }
}

// const pj_str_t * 	pjsip_endpt_name (const pjsip_endpoint *endpt)

pub fn endpt_handle_events(endpt: &mut pjsip_endpoint, max_timeout: &mut pj_time_val) -> Result<(), i32> {
    unsafe {
        check_status( pjsip_endpt_handle_events(endpt as *mut _, max_timeout as *const _))
    }
}

pub fn endpt_handle_events2(endpt: &mut pjsip_endpoint, max_timeout: &mut pj_time_val, count: &mut u32) -> Result<(), i32> {
    unsafe {
        check_status(pjsip_endpt_handle_events2(endpt as *mut _, max_timeout as *const _, count as *mut _))}
}

// pj_status_t 	pjsip_endpt_schedule_timer (pjsip_endpoint *endpt, pj_timer_entry *entry, const pj_time_val *delay)
// pub fn enpt_schedule_timer(endpt: &mut pjsip_endpoint, entry: &mut pj_timer_entry, delay: &mut pj_time_val) -> Result<(), i32> {
//     unsafe {
//         check_status(
//             pjsip_endpt_schedule_timer_dbg(

//             )
//         )
//     }
// }

// pj_status_t 	pjsip_endpt_schedule_timer_w_grp_lock (pjsip_endpoint *endpt, pj_timer_entry *entry, const pj_time_val *delay, int id_val, pj_grp_lock_t *grp_lock)

pub fn endpt_cancel_timer(endpt: &mut pjsip_endpoint, entry: &mut pj_timer_entry) {
    unsafe { pjsip_endpt_cancel_timer(endpt as *mut _, entry as *mut _) }
}

pub fn endpt_get_timer_heap(endpt: &mut pjsip_endpoint) -> *mut pj_timer_heap_t {
    unsafe { pjsip_endpt_get_timer_heap(endpt as *mut _) }
}

pub fn endpt_register_module(endpt: *mut pjsip_endpoint, module: &mut pjsip_module) -> Result<(), i32> {
    unsafe { check_status( pjsip_endpt_register_module(endpt, module as *mut _)) }
}

pub fn endpt_unregister_module(endpt: *mut pjsip_endpoint, module: &mut pjsip_module) -> Result<(), i32> {
    unsafe { check_status(pjsip_endpt_unregister_module(endpt, module as *mut _)) }
}

// void 	pjsip_process_rdata_param_default (pjsip_process_rdata_param *p)
pub fn process_rdata_param_default(p: &mut pjsip_process_rdata_param) {
    unsafe { pjsip_process_rdata_param_default(p as *mut _); }
}

// pj_status_t 	pjsip_endpt_process_rx_data (pjsip_endpoint *endpt, pjsip_rx_data *rdata, pjsip_process_rdata_param *p, pj_bool_t *p_handled)
// pj_pool_t * 	pjsip_endpt_create_pool (pjsip_endpoint *endpt, const char *pool_name, pj_size_t initial, pj_size_t increment)
// void 	pjsip_endpt_release_pool (pjsip_endpoint *endpt, pj_pool_t *pool)
// pjsip_transaction * 	pjsip_endpt_find_tsx (pjsip_endpoint *endpt, const pj_str_t *key)

// void 	pjsip_endpt_register_tsx (pjsip_endpoint *endpt, pjsip_transaction *tsx)
// void 	pjsip_endpt_destroy_tsx (pjsip_endpoint *endpt, pjsip_transaction *tsx)

// pj_status_t 	pjsip_endpt_create_tdata (pjsip_endpoint *endpt, pjsip_tx_data **p_tdata)
pub fn endpt_create_tdata(endpt: &mut pjsip_endpoint, p_tdata: &mut Box<*mut pjsip_tx_data> ) -> Result<(), i32> {
    unsafe {
        check_status( pjsip_endpt_create_tdata( endpt as *mut _, (p_tdata.as_mut() as *mut _) as *mut _))
    }
}

// pj_status_t 	pjsip_endpt_create_resolver (pjsip_endpoint *endpt, pj_dns_resolver **p_resv)
pub fn endpt_create_resolver(endpt: &mut pjsip_endpoint, p_resv: &mut Box<*mut pjsip_tx_data>) -> Result<(), i32> {
    unsafe {
        check_status( pjsip_endpt_create_resolver( endpt as *mut _, (p_resv.as_mut() as *mut _) as *mut _))
    }
}

// pj_status_t 	pjsip_endpt_set_resolver (pjsip_endpoint *endpt, pj_dns_resolver *resv)
pub fn endpt_set_resolver(endpt: &mut pjsip_endpoint, resv: &mut pj_dns_resolver) -> Result<(), i32> {
    unsafe { check_status(pjsip_endpt_set_resolver( endpt as *mut _, resv as *mut _)) }
}

// pj_status_t 	pjsip_endpt_set_ext_resolver (pjsip_endpoint *endpt, pjsip_ext_resolver *ext_res)
pub fn endpt_set_ext_resolver(endpt: &mut pjsip_endpoint, ext_res: &mut pjsip_ext_resolver) -> Result<(), i32> {
    unsafe { check_status( pjsip_endpt_set_ext_resolver( endpt as *mut _, ext_res as *mut _))}
}

// pj_dns_resolver * 	pjsip_endpt_get_resolver (pjsip_endpoint *endpt)
// void 	pjsip_endpt_resolve (pjsip_endpoint *endpt, pj_pool_t *pool, pjsip_host_info *target, void *token, pjsip_resolver_callback *cb)
// pjsip_tpmgr * 	pjsip_endpt_get_tpmgr (pjsip_endpoint *endpt)
// pj_ioqueue_t * 	pjsip_endpt_get_ioqueue (pjsip_endpoint *endpt)
// pj_status_t 	pjsip_endpt_acquire_transport (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_transport **p_tp)
// pj_status_t 	pjsip_endpt_acquire_transport2 (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_tx_data *tdata, pjsip_transport **p_tp)
// const pjsip_hdr * 	pjsip_endpt_get_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname)
// pj_bool_t 	pjsip_endpt_has_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname, const pj_str_t *token)
// pj_status_t 	pjsip_endpt_add_capability (pjsip_endpoint *endpt, pjsip_module *mod, int htype, const pj_str_t *hname, unsigned count, const pj_str_t tags[])
// const pjsip_hdr * 	pjsip_endpt_get_request_headers (pjsip_endpoint *e)
// void 	pjsip_endpt_dump (pjsip_endpoint *endpt, pj_bool_t detail)
// pj_status_t 	pjsip_endpt_atexit (pjsip_endpoint *endpt, pjsip_endpt_exit_callback func)
