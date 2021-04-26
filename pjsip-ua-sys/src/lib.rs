/* automatically generated by rust-bindgen 0.58.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate pj_sys;
extern crate pjsip_sys;
extern crate pjmedia_sys;
use pj_sys::*;
use pjsip_sys::*;
use pjmedia_sys::*;

pub const PJSIP_REDIRECT_REJECT: pjsip_redirect_op = 0;
pub const PJSIP_REDIRECT_ACCEPT: pjsip_redirect_op = 1;
pub const PJSIP_REDIRECT_ACCEPT_REPLACE: pjsip_redirect_op = 2;
pub const PJSIP_REDIRECT_PENDING: pjsip_redirect_op = 3;
pub const PJSIP_REDIRECT_STOP: pjsip_redirect_op = 4;
pub type pjsip_redirect_op = u32;
pub const PJSIP_INV_STATE_NULL: pjsip_inv_state = 0;
pub const PJSIP_INV_STATE_CALLING: pjsip_inv_state = 1;
pub const PJSIP_INV_STATE_INCOMING: pjsip_inv_state = 2;
pub const PJSIP_INV_STATE_EARLY: pjsip_inv_state = 3;
pub const PJSIP_INV_STATE_CONNECTING: pjsip_inv_state = 4;
pub const PJSIP_INV_STATE_CONFIRMED: pjsip_inv_state = 5;
pub const PJSIP_INV_STATE_DISCONNECTED: pjsip_inv_state = 6;
pub type pjsip_inv_state = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_inv_on_rx_offer_cb_param {
    pub offer: *const pjmedia_sdp_session,
    pub rdata: *const pjsip_rx_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_inv_callback {
    pub on_state_changed: ::std::option::Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, e: *mut pjsip_event),
    >,
    pub on_new_session: ::std::option::Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, e: *mut pjsip_event),
    >,
    pub on_tsx_state_changed: ::std::option::Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            tsx: *mut pjsip_transaction,
            e: *mut pjsip_event,
        ),
    >,
    pub on_rx_offer: ::std::option::Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, offer: *const pjmedia_sdp_session),
    >,
    pub on_rx_offer2: ::std::option::Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            param: *mut pjsip_inv_on_rx_offer_cb_param,
        ),
    >,
    pub on_rx_reinvite: ::std::option::Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            offer: *const pjmedia_sdp_session,
            rdata: *mut pjsip_rx_data,
        ) -> pj_status_t,
    >,
    pub on_create_offer: ::std::option::Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, p_offer: *mut *mut pjmedia_sdp_session),
    >,
    pub on_media_update: ::std::option::Option<
        unsafe extern "C" fn(inv_ses: *mut pjsip_inv_session, status: pj_status_t),
    >,
    pub on_send_ack: ::std::option::Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, rdata: *mut pjsip_rx_data),
    >,
    pub on_redirected: ::std::option::Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            target: *const pjsip_uri,
            e: *const pjsip_event,
        ) -> pjsip_redirect_op,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_timer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct pjsip_inv_session {
    pub obj_name: [::std::os::raw::c_char; 32usize],
    pub pool: *mut pj_pool_t,
    pub pool_prov: *mut pj_pool_t,
    pub pool_active: *mut pj_pool_t,
    pub state: pjsip_inv_state,
    pub cancelling: pj_bool_t,
    pub pending_cancel: pj_bool_t,
    pub pending_bye: *mut pjsip_tx_data,
    pub cause: pjsip_status_code,
    pub cause_text: pj_str_t,
    pub notify: pj_bool_t,
    pub cb_called: ::std::os::raw::c_uint,
    pub dlg: *mut pjsip_dialog,
    pub role: pjsip_role_e,
    pub options: ::std::os::raw::c_uint,
    pub neg: *mut pjmedia_sdp_neg,
    pub sdp_neg_flags: ::std::os::raw::c_uint,
    pub invite_tsx: *mut pjsip_transaction,
    pub invite_req: *mut pjsip_tx_data,
    pub last_answer: *mut pjsip_tx_data,
    pub last_ack: *mut pjsip_tx_data,
    pub last_ack_cseq: pj_int32_t,
    pub mod_data: [*mut ::std::os::raw::c_void; 32usize],
    pub timer: *mut pjsip_timer,
    pub following_fork: pj_bool_t,
    pub ref_cnt: *mut pj_atomic_t,
    pub updated_sdp_answer: pj_bool_t,
}
#[repr(C)]
pub struct pjsip_rdata_sdp_info {
    pub body: pj_str_t,
    pub sdp_err: pj_status_t,
    pub sdp: *mut pjmedia_sdp_session,
}
extern "C" {
    pub fn pjsip_inv_usage_init(
        endpt: *mut pjsip_endpoint,
        cb: *const pjsip_inv_callback,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_usage_instance() -> *mut pjsip_module;
}
extern "C" {
    pub fn pjsip_inv_create_uac(
        dlg: *mut pjsip_dialog,
        local_sdp: *const pjmedia_sdp_session,
        options: ::std::os::raw::c_uint,
        p_inv: *mut *mut pjsip_inv_session,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_verify_request(
        rdata: *mut pjsip_rx_data,
        options: *mut ::std::os::raw::c_uint,
        sdp: *const pjmedia_sdp_session,
        dlg: *mut pjsip_dialog,
        endpt: *mut pjsip_endpoint,
        tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_verify_request2(
        rdata: *mut pjsip_rx_data,
        options: *mut ::std::os::raw::c_uint,
        offer: *const pjmedia_sdp_session,
        answer: *const pjmedia_sdp_session,
        dlg: *mut pjsip_dialog,
        endpt: *mut pjsip_endpoint,
        tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_verify_request3(
        rdata: *mut pjsip_rx_data,
        tmp_pool: *mut pj_pool_t,
        options: *mut ::std::os::raw::c_uint,
        offer: *const pjmedia_sdp_session,
        answer: *const pjmedia_sdp_session,
        dlg: *mut pjsip_dialog,
        endpt: *mut pjsip_endpoint,
        tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_create_uas(
        dlg: *mut pjsip_dialog,
        rdata: *mut pjsip_rx_data,
        local_sdp: *const pjmedia_sdp_session,
        options: ::std::os::raw::c_uint,
        p_inv: *mut *mut pjsip_inv_session,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_add_ref(inv: *mut pjsip_inv_session) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_dec_ref(inv: *mut pjsip_inv_session) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_terminate(
        inv: *mut pjsip_inv_session,
        st_code: ::std::os::raw::c_int,
        notify: pj_bool_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_uac_restart(inv: *mut pjsip_inv_session, new_offer: pj_bool_t) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_process_redirect(
        inv: *mut pjsip_inv_session,
        cmd: pjsip_redirect_op,
        e: *mut pjsip_event,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_invite(
        inv: *mut pjsip_inv_session,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_initial_answer(
        inv: *mut pjsip_inv_session,
        rdata: *mut pjsip_rx_data,
        st_code: ::std::os::raw::c_int,
        st_text: *const pj_str_t,
        sdp: *const pjmedia_sdp_session,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_answer(
        inv: *mut pjsip_inv_session,
        st_code: ::std::os::raw::c_int,
        st_text: *const pj_str_t,
        local_sdp: *const pjmedia_sdp_session,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_set_local_sdp(
        inv: *mut pjsip_inv_session,
        sdp: *const pjmedia_sdp_session,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_set_sdp_answer(
        inv: *mut pjsip_inv_session,
        sdp: *const pjmedia_sdp_session,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_end_session(
        inv: *mut pjsip_inv_session,
        st_code: ::std::os::raw::c_int,
        st_text: *const pj_str_t,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_cancel_reinvite(
        inv: *mut pjsip_inv_session,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_reinvite(
        inv: *mut pjsip_inv_session,
        new_contact: *const pj_str_t,
        new_offer: *const pjmedia_sdp_session,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_update(
        inv: *mut pjsip_inv_session,
        new_contact: *const pj_str_t,
        offer: *const pjmedia_sdp_session,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_create_ack(
        inv: *mut pjsip_inv_session,
        cseq: ::std::os::raw::c_int,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_inv_send_msg(
        inv: *mut pjsip_inv_session,
        tdata: *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_dlg_get_inv_session(dlg: *mut pjsip_dialog) -> *mut pjsip_inv_session;
}
extern "C" {
    pub fn pjsip_inv_state_name(state: pjsip_inv_state) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn pjsip_create_sdp_body(
        pool: *mut pj_pool_t,
        sdp: *mut pjmedia_sdp_session,
        p_body: *mut *mut pjsip_msg_body,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_rdata_get_sdp_info(rdata: *mut pjsip_rx_data) -> *mut pjsip_rdata_sdp_info;
}
extern "C" {
    pub fn pjsip_rdata_get_sdp_info2(
        rdata: *mut pjsip_rx_data,
        med_type: *const pjsip_media_type,
    ) -> *mut pjsip_rdata_sdp_info;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_regc {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct pjsip_regc_cbparam {
    pub regc: *mut pjsip_regc,
    pub token: *mut ::std::os::raw::c_void,
    pub status: pj_status_t,
    pub code: ::std::os::raw::c_int,
    pub reason: pj_str_t,
    pub rdata: *mut pjsip_rx_data,
    pub expiration: ::std::os::raw::c_uint,
    pub contact_cnt: ::std::os::raw::c_int,
    pub contact: [*mut pjsip_contact_hdr; 10usize],
    pub is_unreg: pj_bool_t,
}
pub type pjsip_regc_cb =
    ::std::option::Option<unsafe extern "C" fn(param: *mut pjsip_regc_cbparam)>;
#[repr(C)]
pub struct pjsip_regc_tsx_cb_param {
    pub cbparam: pjsip_regc_cbparam,
    pub contact_cnt: ::std::os::raw::c_int,
    pub contact: [pj_str_t; 10usize],
}
pub type pjsip_regc_tsx_cb =
    ::std::option::Option<unsafe extern "C" fn(param: *mut pjsip_regc_tsx_cb_param)>;
#[repr(C)]
pub struct pjsip_regc_info {
    pub server_uri: pj_str_t,
    pub client_uri: pj_str_t,
    pub is_busy: pj_bool_t,
    pub auto_reg: pj_bool_t,
    pub interval: ::std::os::raw::c_uint,
    pub next_reg: ::std::os::raw::c_uint,
    pub transport: *mut pjsip_transport,
}
extern "C" {
    pub fn pjsip_regc_create(
        endpt: *mut pjsip_endpoint,
        token: *mut ::std::os::raw::c_void,
        cb: pjsip_regc_cb,
        p_regc: *mut *mut pjsip_regc,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_destroy(regc: *mut pjsip_regc) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_get_info(regc: *mut pjsip_regc, info: *mut pjsip_regc_info) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_get_pool(regc: *mut pjsip_regc) -> *mut pj_pool_t;
}
extern "C" {
    pub fn pjsip_regc_init(
        regc: *mut pjsip_regc,
        srv_url: *const pj_str_t,
        from_url: *const pj_str_t,
        to_url: *const pj_str_t,
        ccnt: ::std::os::raw::c_int,
        contact: *const pj_str_t,
        expires: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_add_ref(regc: *mut pjsip_regc);
}
extern "C" {
    pub fn pjsip_regc_dec_ref(regc: *mut pjsip_regc) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_reg_tsx_cb(
        regc: *mut pjsip_regc,
        tsx_cb: pjsip_regc_tsx_cb,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_via_sent_by(
        regc: *mut pjsip_regc,
        via_addr: *mut pjsip_host_port,
        via_tp: *mut pjsip_transport,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_delay_before_refresh(
        regc: *mut pjsip_regc,
        delay: pj_uint32_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_credentials(
        regc: *mut pjsip_regc,
        count: ::std::os::raw::c_int,
        cred: *const pjsip_cred_info,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_prefs(
        regc: *mut pjsip_regc,
        pref: *const pjsip_auth_clt_pref,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_route_set(
        regc: *mut pjsip_regc,
        route_set: *const pjsip_route_hdr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_set_transport(
        regc: *mut pjsip_regc,
        sel: *const pjsip_tpselector,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_release_transport(regc: *mut pjsip_regc) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_add_headers(regc: *mut pjsip_regc, hdr_list: *const pjsip_hdr)
        -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_register(
        regc: *mut pjsip_regc,
        autoreg: pj_bool_t,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_unregister(
        regc: *mut pjsip_regc,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_unregister_all(
        regc: *mut pjsip_regc,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_update_contact(
        regc: *mut pjsip_regc,
        ccnt: ::std::os::raw::c_int,
        contact: *const pj_str_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_update_expires(regc: *mut pjsip_regc, expires: pj_uint32_t) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_regc_send(regc: *mut pjsip_regc, tdata: *mut pjsip_tx_data) -> pj_status_t;
}
#[repr(C)]
pub struct pjsip_replaces_hdr {
    pub prev: *mut pjsip_replaces_hdr,
    pub next: *mut pjsip_replaces_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub call_id: pj_str_t,
    pub to_tag: pj_str_t,
    pub from_tag: pj_str_t,
    pub early_only: pj_bool_t,
    pub other_param: pjsip_param,
}
extern "C" {
    pub fn pjsip_replaces_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_replaces_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_replaces_hdr;
}
extern "C" {
    pub fn pjsip_replaces_verify_request(
        rdata: *mut pjsip_rx_data,
        p_dlg: *mut *mut pjsip_dialog,
        lock_dlg: pj_bool_t,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_evsub {
    _unused: [u8; 0],
}
pub const PJSIP_EVSUB_STATE_NULL: pjsip_evsub_state = 0;
pub const PJSIP_EVSUB_STATE_SENT: pjsip_evsub_state = 1;
pub const PJSIP_EVSUB_STATE_ACCEPTED: pjsip_evsub_state = 2;
pub const PJSIP_EVSUB_STATE_PENDING: pjsip_evsub_state = 3;
pub const PJSIP_EVSUB_STATE_ACTIVE: pjsip_evsub_state = 4;
pub const PJSIP_EVSUB_STATE_TERMINATED: pjsip_evsub_state = 5;
pub const PJSIP_EVSUB_STATE_UNKNOWN: pjsip_evsub_state = 6;
pub type pjsip_evsub_state = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_evsub_user {
    pub on_evsub_state:
        ::std::option::Option<unsafe extern "C" fn(sub: *mut pjsip_evsub, event: *mut pjsip_event)>,
    pub on_tsx_state: ::std::option::Option<
        unsafe extern "C" fn(
            sub: *mut pjsip_evsub,
            tsx: *mut pjsip_transaction,
            event: *mut pjsip_event,
        ),
    >,
    pub on_rx_refresh: ::std::option::Option<
        unsafe extern "C" fn(
            sub: *mut pjsip_evsub,
            rdata: *mut pjsip_rx_data,
            p_st_code: *mut ::std::os::raw::c_int,
            p_st_text: *mut *mut pj_str_t,
            res_hdr: *mut pjsip_hdr,
            p_body: *mut *mut pjsip_msg_body,
        ),
    >,
    pub on_rx_notify: ::std::option::Option<
        unsafe extern "C" fn(
            sub: *mut pjsip_evsub,
            rdata: *mut pjsip_rx_data,
            p_st_code: *mut ::std::os::raw::c_int,
            p_st_text: *mut *mut pj_str_t,
            res_hdr: *mut pjsip_hdr,
            p_body: *mut *mut pjsip_msg_body,
        ),
    >,
    pub on_client_refresh: ::std::option::Option<unsafe extern "C" fn(sub: *mut pjsip_evsub)>,
    pub on_server_timeout: ::std::option::Option<unsafe extern "C" fn(sub: *mut pjsip_evsub)>,
}
extern "C" {
    pub static pjsip_refer_method: pjsip_method;
}
extern "C" {
    pub fn pjsip_get_refer_method() -> *const pjsip_method;
}
extern "C" {
    pub fn pjsip_xfer_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_create_uac(
        dlg: *mut pjsip_dialog,
        user_cb: *const pjsip_evsub_user,
        p_evsub: *mut *mut pjsip_evsub,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_create_uas(
        dlg: *mut pjsip_dialog,
        user_cb: *const pjsip_evsub_user,
        rdata: *mut pjsip_rx_data,
        p_evsub: *mut *mut pjsip_evsub,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_initiate(
        sub: *mut pjsip_evsub,
        refer_to_uri: *const pj_str_t,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_accept(
        sub: *mut pjsip_evsub,
        rdata: *mut pjsip_rx_data,
        st_code: ::std::os::raw::c_int,
        hdr_list: *const pjsip_hdr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_notify(
        sub: *mut pjsip_evsub,
        state: pjsip_evsub_state,
        xfer_st_code: ::std::os::raw::c_int,
        xfer_st_text: *const pj_str_t,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_current_notify(
        sub: *mut pjsip_evsub,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_xfer_send_request(sub: *mut pjsip_evsub, tdata: *mut pjsip_tx_data)
        -> pj_status_t;
}
extern "C" {
    pub static pjsip_prack_method: pjsip_method;
}
extern "C" {
    pub fn pjsip_get_prack_method() -> *const pjsip_method;
}
extern "C" {
    pub fn pjsip_100rel_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_100rel_attach(inv: *mut pjsip_inv_session) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_100rel_is_reliable(rdata: *mut pjsip_rx_data) -> pj_bool_t;
}
extern "C" {
    pub fn pjsip_100rel_create_prack(
        inv: *mut pjsip_inv_session,
        rdata: *mut pjsip_rx_data,
        p_tdata: *mut *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_100rel_send_prack(
        inv: *mut pjsip_inv_session,
        tdata: *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_100rel_on_rx_prack(
        inv: *mut pjsip_inv_session,
        rdata: *mut pjsip_rx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_100rel_tx_response(
        inv: *mut pjsip_inv_session,
        tdata: *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_100rel_end_session(inv: *mut pjsip_inv_session) -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_timer_setting {
    pub min_se: ::std::os::raw::c_uint,
    pub sess_expires: ::std::os::raw::c_uint,
}
#[repr(C)]
pub struct pjsip_sess_expires_hdr {
    pub prev: *mut pjsip_sess_expires_hdr,
    pub next: *mut pjsip_sess_expires_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub sess_expires: ::std::os::raw::c_uint,
    pub refresher: pj_str_t,
    pub other_param: pjsip_param,
}
#[repr(C)]
pub struct pjsip_min_se_hdr {
    pub prev: *mut pjsip_min_se_hdr,
    pub next: *mut pjsip_min_se_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub min_se: ::std::os::raw::c_uint,
    pub other_param: pjsip_param,
}
extern "C" {
    pub fn pjsip_timer_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_setting_default(setting: *mut pjsip_timer_setting) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_init_session(
        inv: *mut pjsip_inv_session,
        setting: *const pjsip_timer_setting,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_sess_expires_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_sess_expires_hdr;
}
extern "C" {
    pub fn pjsip_min_se_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_min_se_hdr;
}
extern "C" {
    pub fn pjsip_timer_update_req(
        inv: *mut pjsip_inv_session,
        tdata: *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_process_resp(
        inv: *mut pjsip_inv_session,
        rdata: *const pjsip_rx_data,
        st_code: *mut pjsip_status_code,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_handle_refresh_error(
        inv: *mut pjsip_inv_session,
        event: *mut pjsip_event,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_process_req(
        inv: *mut pjsip_inv_session,
        rdata: *const pjsip_rx_data,
        st_code: *mut pjsip_status_code,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_update_resp(
        inv: *mut pjsip_inv_session,
        tdata: *mut pjsip_tx_data,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjsip_timer_end_session(inv: *mut pjsip_inv_session) -> pj_status_t;
}
