#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate pj_sys;
extern crate pjmedia_sys;
extern crate pjsip_sys;

use pj_sys::*;
use pjsip_sys::*;

use std::os::raw::{c_uint, c_int, c_char, c_void};



pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_NULL: pjsip_evsub_state = 0;
pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_SENT: pjsip_evsub_state = 1;
pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_ACCEPTED: pjsip_evsub_state = 2;
pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_PENDING: pjsip_evsub_state = 3;
pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_ACTIVE: pjsip_evsub_state = 4;
pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_TERMINATED: pjsip_evsub_state = 5;
pub const pjsip_evsub_state_PJSIP_EVSUB_STATE_UNKNOWN: pjsip_evsub_state = 6;
pub type pjsip_evsub_state = c_uint;
pub const PJSIP_EVSUB_NO_EVENT_ID: c_uint = 1;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_evsub_user {
    pub on_evsub_state:
    Option<unsafe extern "C" fn(sub: *mut pjsip_evsub, event: *mut pjsip_event)>,
    pub on_tsx_state: Option<
    unsafe extern "C" fn(
        sub: *mut pjsip_evsub,
        tsx: *mut pjsip_transaction,
        event: *mut pjsip_event,
        ),
        >,
    pub on_rx_refresh: Option<
    unsafe extern "C" fn(
        sub: *mut pjsip_evsub,
            rdata: *mut pjsip_rx_data,
            p_st_code: *mut c_int,
            p_st_text: *mut *mut pj_str_t,
            res_hdr: *mut pjsip_hdr,
            p_body: *mut *mut pjsip_msg_body,
        ),
    >,
    pub on_rx_notify: Option<
    unsafe extern "C" fn(
            sub: *mut pjsip_evsub,
            rdata: *mut pjsip_rx_data,
            p_st_code: *mut c_int,
            p_st_text: *mut *mut pj_str_t,
            res_hdr: *mut pjsip_hdr,
            p_body: *mut *mut pjsip_msg_body,
        ),
        >,
    pub on_client_refresh: Option<unsafe extern "C" fn(sub: *mut pjsip_evsub)>,
    pub on_server_timeout: Option<unsafe extern "C" fn(sub: *mut pjsip_evsub)>,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_evsub {
    _unused: [u8; 0],
}



#[link(name="pjsip-simple")]
extern "C" {

    pub fn pjsip_evsub_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_evsub_instance() -> *mut pjsip_module;
    pub fn pjsip_evsub_register_pkg( pkg_mod: *mut pjsip_module, event_name: *const pj_str_t, expires: c_uint, accept_cnt: c_uint, accept: *const pj_str_t, ) -> pj_status_t;
    pub fn pjsip_evsub_get_allow_events_hdr(m: *mut pjsip_module) -> *const pjsip_hdr;
    pub fn pjsip_evsub_create_uac( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, event: *const pj_str_t, option: c_uint, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_evsub_create_uas( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, rdata: *mut pjsip_rx_data, option: c_uint, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_evsub_terminate(sub: *mut pjsip_evsub, notify: pj_bool_t) -> pj_status_t;
    pub fn pjsip_evsub_get_state(sub: *mut pjsip_evsub) -> pjsip_evsub_state;
    pub fn pjsip_evsub_get_state_name(sub: *mut pjsip_evsub) -> *const c_char;
    pub fn pjsip_evsub_get_termination_reason(sub: *mut pjsip_evsub) -> *const pj_str_t;
    pub fn pjsip_evsub_initiate( sub: *mut pjsip_evsub, method: *const pjsip_method, expires: pj_uint32_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_evsub_add_header(sub: *mut pjsip_evsub, hdr_list: *const pjsip_hdr) -> pj_status_t;
    pub fn pjsip_evsub_accept( sub: *mut pjsip_evsub, rdata: *mut pjsip_rx_data, st_code: c_int, hdr_list: *const pjsip_hdr, ) -> pj_status_t;
    pub fn pjsip_evsub_notify( sub: *mut pjsip_evsub, state: pjsip_evsub_state, state_str: *const pj_str_t, reason: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_evsub_current_notify( sub: *mut pjsip_evsub, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_evsub_send_request( sub: *mut pjsip_evsub, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_evsub_set_mod_data( sub: *mut pjsip_evsub, mod_id: c_uint, data: *mut c_void, );
    pub fn pjsip_evsub_get_mod_data( sub: *mut pjsip_evsub, mod_id: c_uint, ) -> *mut c_void;
    pub fn pjsip_evsub_add_ref(sub: *mut pjsip_evsub) -> pj_status_t;
    pub fn pjsip_evsub_dec_ref(sub: *mut pjsip_evsub) -> pj_status_t;
    pub fn pjsip_evsub_uas_set_timeout(sub: *mut pjsip_evsub, seconds: pj_uint32_t);

    pub fn pjsip_replaces_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_replaces_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_replaces_hdr;
    pub fn pjsip_replaces_verify_request( rdata: *mut pjsip_rx_data, p_dlg: *mut *mut pjsip_dialog, lock_dlg: pj_bool_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;

    pub fn pjsip_pres_init_module( endpt: *mut pjsip_endpoint, mod_evsub: *mut pjsip_module, ) -> pj_status_t;
    pub fn pjsip_pres_instance() -> *mut pjsip_module;
    pub fn pjsip_pres_create_uac( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, options: c_uint, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_pres_create_uas( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, rdata: *mut pjsip_rx_data, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_pres_terminate(sub: *mut pjsip_evsub, notify: pj_bool_t) -> pj_status_t;
    pub fn pjsip_pres_initiate( sub: *mut pjsip_evsub, expires: pj_uint32_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_pres_add_header(sub: *mut pjsip_evsub, hdr_list: *const pjsip_hdr) -> pj_status_t;
    pub fn pjsip_pres_accept( sub: *mut pjsip_evsub, rdata: *mut pjsip_rx_data, st_code: c_int, hdr_list: *const pjsip_hdr, ) -> pj_status_t;
    pub fn pjsip_pres_notify( sub: *mut pjsip_evsub, state: pjsip_evsub_state, state_str: *const pj_str_t, reason: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_pres_current_notify( sub: *mut pjsip_evsub, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_pres_send_request(sub: *mut pjsip_evsub, tdata: *mut pjsip_tx_data) -> pj_status_t;
    pub fn pjsip_pres_get_status( sub: *mut pjsip_evsub, status: *mut pjsip_pres_status, ) -> pj_status_t;
    pub fn pjsip_pres_set_status( sub: *mut pjsip_evsub, status: *const pjsip_pres_status, ) -> pj_status_t;
    pub fn pjsip_pres_create_pidf( pool: *mut pj_pool_t, status: *const pjsip_pres_status, entity: *const pj_str_t, p_body: *mut *mut pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_pres_create_xpidf( pool: *mut pj_pool_t, status: *const pjsip_pres_status, entity: *const pj_str_t, p_body: *mut *mut pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_pres_parse_pidf( rdata: *mut pjsip_rx_data, pool: *mut pj_pool_t, status: *mut pjsip_pres_status, ) -> pj_status_t;
    pub fn pjsip_pres_parse_pidf2( body: *mut c_char, body_len: c_uint, pool: *mut pj_pool_t, status: *mut pjsip_pres_status, ) -> pj_status_t;
    pub fn pjsip_pres_parse_xpidf( rdata: *mut pjsip_rx_data, pool: *mut pj_pool_t, status: *mut pjsip_pres_status, ) -> pj_status_t;
    pub fn pjsip_pres_parse_xpidf2( body: *mut c_char, body_len: c_uint, pool: *mut pj_pool_t, status: *mut pjsip_pres_status, ) -> pj_status_t;

    pub fn pjsip_xfer_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_xfer_create_uac( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_xfer_create_uas( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, rdata: *mut pjsip_rx_data, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_xfer_initiate( sub: *mut pjsip_evsub, refer_to_uri: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_xfer_accept( sub: *mut pjsip_evsub, rdata: *mut pjsip_rx_data, st_code: c_int, hdr_list: *const pjsip_hdr, ) -> pj_status_t;
    pub fn pjsip_xfer_notify( sub: *mut pjsip_evsub, state: pjsip_evsub_state, xfer_st_code: c_int, xfer_st_text: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_xfer_current_notify( sub: *mut pjsip_evsub, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_xfer_send_request(sub: *mut pjsip_evsub, tdata: *mut pjsip_tx_data) -> pj_status_t;

    pub fn pjsip_mwi_init_module( endpt: *mut pjsip_endpoint, mod_evsub: *mut pjsip_module, ) -> pj_status_t;
    pub fn pjsip_mwi_instance() -> *mut pjsip_module;
    pub fn pjsip_mwi_create_uac( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, options: c_uint, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_mwi_create_uas( dlg: *mut pjsip_dialog, user_cb: *const pjsip_evsub_user, rdata: *mut pjsip_rx_data, p_evsub: *mut *mut pjsip_evsub, ) -> pj_status_t;
    pub fn pjsip_mwi_terminate(sub: *mut pjsip_evsub, notify: pj_bool_t) -> pj_status_t;
    pub fn pjsip_mwi_initiate( sub: *mut pjsip_evsub, expires: pj_uint32_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_mwi_accept( sub: *mut pjsip_evsub, rdata: *mut pjsip_rx_data, st_code: c_int, hdr_list: *const pjsip_hdr, ) -> pj_status_t;
    pub fn pjsip_mwi_notify( sub: *mut pjsip_evsub, state: pjsip_evsub_state, state_str: *const pj_str_t, reason: *const pj_str_t, mime_type: *const pjsip_media_type, body: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_mwi_current_notify( sub: *mut pjsip_evsub, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_mwi_send_request(sub: *mut pjsip_evsub, tdata: *mut pjsip_tx_data) -> pj_status_t;

    pub fn pjsip_tsx_get_evsub(tsx: *mut pjsip_transaction) -> *mut pjsip_evsub;

    pub fn pjsip_evsub_init_parser();
    pub fn pjsip_iscomposing_create_xml( pool: *mut pj_pool_t, is_composing: pj_bool_t, lst_actv: *const pj_time_val, content_tp: *const pj_str_t, refresh: c_int, ) -> *mut pj_xml_node;
    pub fn pjsip_iscomposing_create_body( pool: *mut pj_pool_t, is_composing: pj_bool_t, lst_actv: *const pj_time_val, content_tp: *const pj_str_t, refresh: c_int, ) -> *mut pjsip_msg_body;
    pub fn pjsip_iscomposing_parse( pool: *mut pj_pool_t, msg: *mut c_char, len: pj_size_t, p_is_composing: *mut pj_bool_t, p_last_active: *mut *mut pj_str_t, p_content_type: *mut *mut pj_str_t, p_refresh: *mut c_int, ) -> pj_status_t;

    pub static mut pjpidf_op: pjpidf_op_desc;
    pub fn pjpidf_create(pool: *mut pj_pool_t, entity: *const pj_str_t) -> *mut pjpidf_pres;
    pub fn pjpidf_parse( pool: *mut pj_pool_t, text: *mut c_char, len: c_int, ) -> *mut pjpidf_pres;
    pub fn pjpidf_print( pres: *const pjpidf_pres, buf: *mut c_char, len: c_int, ) -> c_int;
    pub fn pjpidf_pres_construct( pool: *mut pj_pool_t, pres: *mut pjpidf_pres, entity: *const pj_str_t, );
    pub fn pjpidf_pres_add_tuple( pool: *mut pj_pool_t, pres: *mut pjpidf_pres, id: *const pj_str_t, ) -> *mut pjpidf_tuple;
    pub fn pjpidf_pres_get_first_tuple(pres: *mut pjpidf_pres) -> *mut pjpidf_tuple;
    pub fn pjpidf_pres_get_next_tuple( pres: *mut pjpidf_pres, t: *mut pjpidf_tuple, ) -> *mut pjpidf_tuple;
    pub fn pjpidf_pres_find_tuple(pres: *mut pjpidf_pres, id: *const pj_str_t) -> *mut pjpidf_tuple;
    pub fn pjpidf_pres_remove_tuple(pres: *mut pjpidf_pres, arg1: *mut pjpidf_tuple);
    pub fn pjpidf_pres_add_note( pool: *mut pj_pool_t, pres: *mut pjpidf_pres, text: *const pj_str_t, ) -> *mut pjpidf_note;
    pub fn pjpidf_pres_get_first_note(pres: *mut pjpidf_pres) -> *mut pjpidf_note;
    pub fn pjpidf_pres_get_next_note( arg1: *mut pjpidf_pres, arg2: *mut pjpidf_note, ) -> *mut pjpidf_note;
    pub fn pjpidf_tuple_construct(pool: *mut pj_pool_t, t: *mut pjpidf_tuple, id: *const pj_str_t);
    pub fn pjpidf_tuple_get_id(t: *const pjpidf_tuple) -> *const pj_str_t;
    pub fn pjpidf_tuple_set_id(pool: *mut pj_pool_t, t: *mut pjpidf_tuple, id: *const pj_str_t);
    pub fn pjpidf_tuple_get_status(t: *mut pjpidf_tuple) -> *mut pjpidf_status;
    pub fn pjpidf_tuple_get_contact(t: *const pjpidf_tuple) -> *const pj_str_t;
    pub fn pjpidf_tuple_set_contact( pool: *mut pj_pool_t, t: *mut pjpidf_tuple, contact: *const pj_str_t, );
    pub fn pjpidf_tuple_set_contact_prio( pool: *mut pj_pool_t, t: *mut pjpidf_tuple, prio: *const pj_str_t, );
    pub fn pjpidf_tuple_get_contact_prio(t: *const pjpidf_tuple) -> *const pj_str_t;
    pub fn pjpidf_tuple_add_note( pool: *mut pj_pool_t, t: *mut pjpidf_tuple, text: *const pj_str_t, ) -> *mut pjpidf_note;
    pub fn pjpidf_tuple_get_first_note(t: *mut pjpidf_tuple) -> *mut pjpidf_note;
    pub fn pjpidf_tuple_get_next_note( t: *mut pjpidf_tuple, n: *mut pjpidf_note, ) -> *mut pjpidf_note;
    pub fn pjpidf_tuple_get_timestamp(t: *const pjpidf_tuple) -> *const pj_str_t;
    pub fn pjpidf_tuple_set_timestamp( pool: *mut pj_pool_t, t: *mut pjpidf_tuple, ts: *const pj_str_t, );
    pub fn pjpidf_tuple_set_timestamp_np( arg1: *mut pj_pool_t, t: *mut pjpidf_tuple, ts: *mut pj_str_t, );
    pub fn pjpidf_status_construct(arg1: *mut pj_pool_t, arg2: *mut pjpidf_status);
    pub fn pjpidf_status_is_basic_open(arg1: *const pjpidf_status) -> pj_bool_t;
    pub fn pjpidf_status_set_basic_open(arg1: *mut pjpidf_status, arg2: pj_bool_t);

    pub static pjsip_publish_method: pjsip_method;
    pub fn pjsip_publishc_opt_default(opt: *mut pjsip_publishc_opt);
    pub fn pjsip_publishc_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_publishc_create( endpt: *mut pjsip_endpoint, opt: *const pjsip_publishc_opt, token: *mut c_void, cb: pjsip_publishc_cb, p_pubc: *mut *mut pjsip_publishc, ) -> pj_status_t;
    pub fn pjsip_publishc_destroy(pubc: *mut pjsip_publishc) -> pj_status_t;
    pub fn pjsip_publishc_get_pool(pubc: *mut pjsip_publishc) -> *mut pj_pool_t;
    pub fn pjsip_publishc_init( pubc: *mut pjsip_publishc, event: *const pj_str_t, target_uri: *const pj_str_t, from_uri: *const pj_str_t, to_uri: *const pj_str_t, expires: pj_uint32_t, ) -> pj_status_t;
    pub fn pjsip_publishc_set_credentials( pubc: *mut pjsip_publishc, count: c_int, c: *const pjsip_cred_info, ) -> pj_status_t;
    pub fn pjsip_publishc_set_route_set( pubc: *mut pjsip_publishc, rs: *const pjsip_route_hdr, ) -> pj_status_t;
    pub fn pjsip_publishc_set_headers( pubc: *mut pjsip_publishc, hdr_list: *const pjsip_hdr, ) -> pj_status_t;
    pub fn pjsip_publishc_set_via_sent_by( pubc: *mut pjsip_publishc, via_addr: *mut pjsip_host_port, via_tp: *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_publishc_publish( pubc: *mut pjsip_publishc, auto_refresh: pj_bool_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_publishc_unpublish( pubc: *mut pjsip_publishc, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_publishc_update_expires( pubc: *mut pjsip_publishc, expires: pj_uint32_t, ) -> pj_status_t;
    pub fn pjsip_publishc_send(pubc: *mut pjsip_publishc, tdata: *mut pjsip_tx_data) -> pj_status_t;

    pub fn pjrpid_element_dup( pool: *mut pj_pool_t, dst: *mut pjrpid_element, src: *const pjrpid_element, );
    pub fn pjrpid_add_element( pres: *mut pjpidf_pres, pool: *mut pj_pool_t, options: c_uint, elem: *const pjrpid_element, ) -> pj_status_t;
    pub fn pjrpid_get_element( pres: *const pjpidf_pres, pool: *mut pj_pool_t, elem: *mut pjrpid_element, ) -> pj_status_t;

    pub fn pjxpidf_create(pool: *mut pj_pool_t, uri: *const pj_str_t) -> *mut pjxpidf_pres;
    pub fn pjxpidf_parse( pool: *mut pj_pool_t, text: *mut c_char, len: pj_size_t, ) -> *mut pjxpidf_pres;
    pub fn pjxpidf_print( pres: *mut pjxpidf_pres, text: *mut c_char, len: pj_size_t, ) -> c_int;
    pub fn pjxpidf_get_uri(pres: *mut pjxpidf_pres) -> *mut pj_str_t;
    pub fn pjxpidf_set_uri( pool: *mut pj_pool_t, pres: *mut pjxpidf_pres, uri: *const pj_str_t, ) -> pj_status_t;
    pub fn pjxpidf_get_status(pres: *mut pjxpidf_pres) -> pj_bool_t;
    pub fn pjxpidf_set_status(pres: *mut pjxpidf_pres, status: pj_bool_t) -> pj_status_t;

}