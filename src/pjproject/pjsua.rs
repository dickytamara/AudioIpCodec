#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use super::pjdefault::AutoCreate;
use super::pjdefault::FromString;

use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsip_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;

use std::os::raw::{c_int, c_uint, c_void};
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

pub const PJSUA_INVALID_ID: i32 = -1;

#[link(name="pjsua")]
extern "C" {
    pub fn pjsua_conf_get_msignal_level(
        slot: pjsua_conf_port_id,
        tx_level_l: *mut c_uint,
        tx_level_r: *mut c_uint,
        rx_level_l: *mut c_uint,
        rx_level_r: *mut c_uint,
    ) -> pj_status_t;
}


impl AutoCreate<pjsua_srtp_opt> for pjsua_srtp_opt {
    fn new() -> pjsua_srtp_opt {
        pjsua_srtp_opt {
            crypto_count: 0,
            crypto: [pjmedia_srtp_crypto::new(); 16],
            keying_count: 0,
            keying: [0, 0],
        }
    }
}

pub trait PjsuaCallback {
    unsafe extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {}
    unsafe extern "C" fn on_incoming_call(
        acc_id: pjsua_acc_id,
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
    ) {
    }
    unsafe extern "C" fn on_call_tsx_state(
        call_id: pjsua_call_id,
        tsx: *mut pjsip_transaction,
        e: *mut pjsip_event,
    ) {
    }
    unsafe extern "C" fn on_call_media_state(call_id: pjsua_call_id) {
    }

    unsafe extern "C" fn on_call_sdp_created(
        call_id: pjsua_call_id,
        sdp: *mut pjmedia_sdp_session,
        pool: *mut pj_pool_t,
        rem_sdp: *const pjmedia_sdp_session,
    ) {

    }
    unsafe extern "C" fn on_stream_precreate(
        call_id: pjsua_call_id,
        param: *mut pjsua_on_stream_precreate_param,
    ) {

    }

    unsafe extern "C" fn on_stream_created(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
        p_port: *mut *mut pjmedia_port,
    ) {
    }
    unsafe extern "C" fn on_stream_created2(
        call_id: pjsua_call_id,
        param: *mut pjsua_on_stream_created_param,
    ) {
    }
    unsafe extern "C" fn on_stream_destroyed(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
    ) {
    }
    unsafe extern "C" fn on_dtmf_digit(call_id: pjsua_call_id, digit: c_int) {}
    unsafe extern "C" fn on_dtmf_digit2(call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {}
    unsafe extern "C" fn on_dtmf_event(call_id: pjsua_call_id, event: *const pjsua_dtmf_event) {}
    unsafe extern "C" fn on_call_transfer_request(
        call_id: pjsua_call_id,
        dst: *const pj_str_t,
        code: *mut pjsip_status_code,
    ) {
    }
    unsafe extern "C" fn on_call_transfer_request2(
        call_id: pjsua_call_id,
        dst: *const pj_str_t,
        code: *mut pjsip_status_code,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_transfer_status(
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
    }
    unsafe extern "C" fn on_call_replace_request(
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
        st_code: *mut c_int,
        st_text: *mut pj_str_t,
    ) {
    }
    unsafe extern "C" fn on_call_replace_request2(
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
        st_code: *mut c_int,
        st_text: *mut pj_str_t,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_replaced(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id) {}
    unsafe extern "C" fn on_call_rx_offer(
        call_id: pjsua_call_id,
        offer: *const pjmedia_sdp_session,
        reserved: *mut c_void,
        code: *mut pjsip_status_code,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_rx_reinvite(
        call_id: pjsua_call_id,
        offer: *const pjmedia_sdp_session,
        rdata: *mut pjsip_rx_data,
        reserved: *mut c_void,
        async_: *mut pj_bool_t,
        code: *mut pjsip_status_code,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_call_tx_offer(
        call_id: pjsua_call_id,
        reserved: *mut c_void,
        opt: *mut pjsua_call_setting,
    ) {
    }
    unsafe extern "C" fn on_reg_started(acc_id: pjsua_acc_id, renew: pj_bool_t) {}
    unsafe extern "C" fn on_reg_started2(acc_id: pjsua_acc_id, info: *mut pjsua_reg_info) {}
    unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {}
    unsafe extern "C" fn on_reg_state2(acc_id: pjsua_acc_id, info: *mut pjsua_reg_info) {}
    unsafe extern "C" fn on_incoming_subscribe(
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data,
    ) {
    }

    unsafe extern "C" fn on_srv_subscribe_state(
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        remote_uri: *const pj_str_t,
        state: pjsip_evsub_state,
        event: *mut pjsip_event,
    ) {
    }

    unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {}

    unsafe extern "C" fn on_buddy_evsub_state(
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
    }

    unsafe extern "C" fn on_pager(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
    ) {
    }

    unsafe extern "C" fn on_pager2(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        acc_id: pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_pager_status(
        call_id: pjsua_call_id,
        to: *const pj_str_t,
        body: *const pj_str_t,
        user_data: *mut c_void,
        status: pjsip_status_code,
        reason: *const pj_str_t,
    ) {
    }

    unsafe extern "C" fn on_pager_status2(
        call_id: pjsua_call_id,
        to: *const pj_str_t,
        body: *const pj_str_t,
        user_data: *mut c_void,
        status: pjsip_status_code,
        reason: *const pj_str_t,
        tdata: *mut pjsip_tx_data,
        rdata: *mut pjsip_rx_data,
        acc_id: pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_typing(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
    }

    unsafe extern "C" fn on_typing2(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
        rdata: *mut pjsip_rx_data,
        acc_id: pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_nat_detect(res: *const pj_stun_nat_detect_result) {}

    unsafe extern "C" fn on_call_redirected(
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        0
    }

    unsafe extern "C" fn on_mwi_state(acc_id: pjsua_acc_id, evsub: *mut pjsip_evsub) {}

    unsafe extern "C" fn on_mwi_info(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {}

    unsafe extern "C" fn on_transport_state(
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
    }

    unsafe extern "C" fn on_call_media_transport_state(
        call_id: pjsua_call_id,
        info: *const pjsua_med_tp_state_info,
    ) -> pj_status_t {
        0
    }

    unsafe extern "C" fn on_ice_transport_error(
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
    }

    unsafe extern "C" fn on_snd_dev_operation(operation: c_int) -> pj_status_t {
        0
    }

    unsafe extern "C" fn on_call_media_event(
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event,
    ) {
        //println!("OnCallMediaEvent");
    }

    unsafe extern "C" fn on_create_media_transport(
        call_id: pjsua_call_id,
        media_idx: c_uint,
        base_tp: *mut pjmedia_transport,
        flags: c_uint,
    ) -> *mut pjmedia_transport {
        //println!("OnCreateMediaTransport");
        let mut transport: pjmedia_transport = pjmedia_transport::new();
        &mut transport as *mut _
    }

    unsafe extern "C" fn on_create_media_transport_srtp(
        call_id: pjsua_call_id,
        media_idx: c_uint,
        srtp_opt: *mut pjmedia_srtp_setting,
    ) {
    }

    unsafe extern "C" fn on_acc_find_for_incoming(
        rdata: *const pjsip_rx_data,
        acc_id: *mut pjsua_acc_id,
    ) {
    }

    unsafe extern "C" fn on_stun_resolution_complete(result: *const pj_stun_resolve_result) {}

    unsafe extern "C" fn on_ip_change_progress(
        op: pjsua_ip_change_op,
        status: pj_status_t,
        info: *const pjsua_ip_change_op_info,
    ) {
    }
    unsafe extern "C" fn on_media_event(event: *mut pjmedia_event) {}
}

impl AutoCreate<pjsua_callback> for pjsua_callback {
    fn new() -> pjsua_callback {
        pjsua_callback {
            on_call_state: None,
            on_incoming_call: None,
            on_call_tsx_state: None,
            on_call_media_state: None,
            on_call_sdp_created: None,
            on_stream_precreate: None,
            on_stream_created: None,
            on_stream_created2: None,
            on_stream_destroyed: None,
            on_dtmf_digit: None,
            on_dtmf_digit2: None,
            on_dtmf_event: None,
            on_call_transfer_request: None,
            on_call_transfer_request2: None,
            on_call_transfer_status: None,
            on_call_replace_request: None,
            on_call_replace_request2: None,
            on_call_replaced: None,
            on_call_rx_offer: None,
            on_call_rx_reinvite: None,
            on_call_tx_offer: None,
            on_reg_started: None,
            on_reg_started2: None,
            on_reg_state: None,
            on_reg_state2: None,
            on_incoming_subscribe: None,
            on_srv_subscribe_state: None,
            on_buddy_state: None,
            on_buddy_evsub_state: None,
            on_pager: None,
            on_pager2: None,
            on_pager_status: None,
            on_pager_status2: None,
            on_typing: None,
            on_typing2: None,
            on_nat_detect: None,
            on_call_redirected: None,
            on_mwi_state: None,
            on_mwi_info: None,
            on_transport_state: None as pjsip_tp_state_callback,
            on_call_media_transport_state: None as pjsua_med_tp_state_cb,
            on_ice_transport_error: None,
            on_snd_dev_operation: None,
            on_call_media_event: None,
            on_create_media_transport: None,
            on_create_media_transport_srtp: None,
            on_acc_find_for_incoming: None,
            on_stun_resolution_complete: None as pj_stun_resolve_cb,
            on_ip_change_progress: None,
            on_media_event: None,
        }
    }
}

impl AutoCreate<pjsua_logging_config> for pjsua_logging_config {

    fn new() -> pjsua_logging_config {

        let mut config = pjsua_logging_config {
            msg_logging: PJ_FALSE as pj_bool_t,
            level: 0,
            console_level: 0,
            decor: 0,
            log_filename: pj_str_t::new(),
            log_file_flags: 0,
            cb: None,
        };

        unsafe {
            pjsua_logging_config_default(&mut config as *mut _);
        }
        config.level = 0;
        config.console_level= 0;

        config
    }
}

impl AutoCreate<pjsua_config> for pjsua_config {

    fn new() -> pjsua_config {

        let mut config = pjsua_config {
            max_calls: 0,
            thread_cnt: 0,
            nameserver_count: 0,
            nameserver: [pj_str_t::new(); 4],
            force_lr: 0,
            outbound_proxy_cnt: 0,
            outbound_proxy: [pj_str_t::new(); 4],
            stun_domain: pj_str_t::new(),
            stun_host: pj_str_t::new(),
            stun_srv_cnt: 0,
            stun_srv: [pj_str_t::new(); 8],
            stun_try_ipv6: PJ_FALSE as pj_bool_t,
            stun_ignore_failure: PJ_FALSE as pj_bool_t,
            stun_map_use_stun2: PJ_FALSE as pj_bool_t,
            nat_type_in_sdp: 0,
            require_100rel: PJ_FALSE,
            use_timer: PJ_FALSE,
            enable_unsolicited_mwi: PJ_FALSE as pj_bool_t,
            timer_setting: pjsip_timer_setting {
                min_se: 0,
                sess_expires: 0,
            },
            cred_count: 0,
            cred_info: [pjsip_cred_info::new(); 8],
            cb: pjsua_callback::new(),
            user_agent: pj_str_t::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: PJ_FALSE as pj_bool_t,
            srtp_opt: pjsua_srtp_opt::new(),
            hangup_forked_call: PJ_FALSE as pj_bool_t,
        };

        // set with pjsua default
        unsafe {
            pjsua_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_media_config> for pjsua_media_config {
    fn new() -> pjsua_media_config {
        let mut config = pjsua_media_config {
            clock_rate: 0,
            snd_clock_rate: 0,
            channel_count: 0,
            audio_frame_ptime: 0,
            max_media_ports: 0,
            has_ioqueue: PJ_FALSE as pj_bool_t,
            thread_cnt: 0,
            quality: 0,
            ptime: 0,
            no_vad: PJ_FALSE as pj_bool_t,
            ilbc_mode: 0,
            tx_drop_pct: 0,
            rx_drop_pct: 0,
            ec_options: 0,
            ec_tail_len: 0,
            snd_rec_latency: 0,
            snd_play_latency: 0,
            jb_init: 0,
            jb_min_pre: 0,
            jb_max_pre: 0,
            jb_max: 0,
            jb_discard_algo: 0,
            enable_ice: PJ_FALSE as pj_bool_t,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: PJ_FALSE as pj_bool_t,
            ice_always_update: PJ_FALSE as pj_bool_t,
            enable_turn: PJ_FALSE as pj_bool_t,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0 as pj_turn_tp_type,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
            snd_auto_close_time: 0,
            vid_preview_enable_native: PJ_FALSE as pj_bool_t,
            no_smart_media_update: PJ_FALSE as pj_bool_t,
            no_rtcp_sdes_bye: PJ_FALSE as pj_bool_t,
            on_aud_prev_play_frame: None,
            on_aud_prev_rec_frame: None,
        };

        unsafe {
            pjsua_media_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsip_publishc_opt> for pjsip_publishc_opt {
    fn new() -> pjsip_publishc_opt {
        pjsip_publishc_opt {
            queue_request: PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pjsip_auth_clt_pref> for pjsip_auth_clt_pref {
    fn new() -> pjsip_auth_clt_pref {
        pjsip_auth_clt_pref {
            initial_auth: PJ_FALSE as pj_bool_t,
            algorithm: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjsip_timer_setting> for pjsip_timer_setting {
    fn new() -> pjsip_timer_setting {
        pjsip_timer_setting {
            min_se: 0,
            sess_expires: 0,
        }
    }
}

impl AutoCreate<pjsua_transport_config> for pjsua_transport_config {
    fn new() -> pjsua_transport_config {
        let mut config = pjsua_transport_config {
            port: 0,
            port_range: 0,
            public_addr: pj_str_t::new(),
            bound_addr: pj_str_t::new(),
            tls_setting: pjsip_tls_setting::new(),
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            sockopt_params: pj_sockopt_params::new(),
        };

        unsafe {
            pjsua_transport_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_ice_config> for pjsua_ice_config {
    fn new() -> pjsua_ice_config {
        pjsua_ice_config {
            enable_ice: PJ_FALSE as pj_bool_t,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: PJ_FALSE as pj_bool_t,
            ice_always_update: PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pjsua_turn_config> for pjsua_turn_config {
    fn new() -> pjsua_turn_config {
        pjsua_turn_config {
            enable_turn: PJ_FALSE as pj_bool_t,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
        }
    }
}

impl AutoCreate<pjsua_ip_change_acc_cfg> for pjsua_ip_change_acc_cfg {
    fn new() -> pjsua_ip_change_acc_cfg {
        pjsua_ip_change_acc_cfg {
            shutdown_tp: PJ_FALSE as pj_bool_t,
            hangup_calls: PJ_FALSE as pj_bool_t,
            reinvite_flags: 0,
        }
    }
}

impl AutoCreate<pjsua_acc_config> for pjsua_acc_config {
    fn new() -> pjsua_acc_config {
        let mut config = pjsua_acc_config {
            user_data: ptr::null_mut(),
            priority: 0,
            id: pj_str_t::new(),
            reg_uri: pj_str_t::new(),
            reg_hdr_list: pjsip_hdr::new(),
            reg_contact_params: pj_str_t::new(),
            sub_hdr_list: pjsip_hdr::new(),
            mwi_enabled: PJ_FALSE as pj_bool_t,
            mwi_expires: 0,
            publish_enabled: PJ_FALSE as pj_bool_t,
            publish_opt: pjsip_publishc_opt::new(),
            unpublish_max_wait_time_msec: 0,
            auth_pref: pjsip_auth_clt_pref::new(),
            pidf_tuple_id: pj_str_t::new(),
            force_contact: pj_str_t::new(),
            contact_params: pj_str_t::new(),
            contact_uri_params: pj_str_t::new(),
            require_100rel: 0,
            use_timer: 0,
            timer_setting: pjsip_timer_setting::new(),
            proxy_cnt: 0,
            proxy: [pj_str_t::new(); 8],
            lock_codec: 0,
            reg_timeout: 0,
            reg_delay_before_refresh: 0,
            unreg_timeout: 0,
            cred_count: 0,
            cred_info: [pjsip_cred_info::new(); 8],
            transport_id: 0,
            allow_contact_rewrite: PJ_FALSE as pj_bool_t,
            contact_rewrite_method: 0,
            contact_use_src_port: PJ_FALSE as pj_bool_t,
            allow_via_rewrite: PJ_FALSE as pj_bool_t,
            allow_sdp_nat_rewrite: PJ_FALSE as pj_bool_t,
            use_rfc5626: 0,
            rfc5626_instance_id: pj_str_t::new(),
            rfc5626_reg_id: pj_str_t::new(),
            ka_interval: 0,
            ka_data: pj_str_t::new(),
            vid_in_auto_show: PJ_FALSE as pj_bool_t,
            vid_out_auto_transmit: PJ_FALSE as pj_bool_t,
            vid_wnd_flags: 0,
            vid_cap_dev: 0,
            vid_rend_dev: 0,
            vid_stream_rc_cfg: pjmedia_vid_stream_rc_config::new(),
            vid_stream_sk_cfg: pjmedia_vid_stream_sk_config::new(),
            rtp_cfg: pjsua_transport_config::new(),
            nat64_opt: 0,
            ipv6_media_use: 0,
            sip_stun_use: 0,
            media_stun_use: 0,
            use_loop_med_tp: PJ_FALSE as pj_bool_t,
            enable_loopback: PJ_FALSE as pj_bool_t,
            ice_cfg_use: 0,
            ice_cfg: pjsua_ice_config::new(),
            turn_cfg_use: 0,
            turn_cfg: pjsua_turn_config::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: PJ_FALSE as pj_bool_t,
            srtp_opt: pjsua_srtp_opt::new(),
            reg_retry_interval: 0,
            reg_first_retry_interval: 0,
            reg_retry_random_interval: 0,
            drop_calls_on_reg_fail: PJ_FALSE as pj_bool_t,
            reg_use_proxy: 0,
            call_hold_type: 0,
            register_on_acc_add: PJ_FALSE as pj_bool_t,
            ip_change_cfg: pjsua_ip_change_acc_cfg::new(),
            enable_rtcp_mux: PJ_FALSE as pj_bool_t,
            rtcp_fb_cfg: pjmedia_rtcp_fb_setting::new(),
        };

        unsafe {
            pjsua_acc_config_default(&mut config as *mut _);


        config.cred_count = 1;
        config.reg_retry_interval = 300;
        config.reg_first_retry_interval = 60;
        config.cred_info[0].data_type = 0;
        config.cred_info[0].scheme = pj_str_t::from_string(String::from("Digest"));

        config

        }
    }
}

impl AutoCreate<pjsua_buddy_config> for pjsua_buddy_config {
    fn new() -> pjsua_buddy_config {
        let mut config = pjsua_buddy_config {
            uri: pj_str_t::new(),
            subscribe: PJ_FALSE as pj_bool_t,
            user_data: ptr::null_mut(),
        };

        unsafe {
            pjsua_buddy_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<pjsua_transport_info> for pjsua_transport_info {
    fn new() -> pjsua_transport_info {
        pjsua_transport_info {
            id: -1,
            type_: 0,
            type_name: pj_str_t::new(),
            info: pj_str_t::new(),
            flag: 0,
            addr_len: 0,
            local_addr: pj_sockaddr::new(),
            local_name: pjsip_host_port::new(),
            usage_count: 0,
        }
    }
}

impl AutoCreate<pjsua_acc_info> for pjsua_acc_info {
    fn new() -> pjsua_acc_info {
        pjsua_acc_info {
            id: -1,
            is_default: PJ_FALSE as pj_bool_t,
            acc_uri: pj_str_t::new(),
            has_registration: PJ_FALSE as pj_bool_t,
            expires: 0,
            status: 0,
            reg_last_err: PJ_FALSE as pj_status_t,
            status_text: pj_str_t::new(),
            online_status: PJ_FALSE as pj_bool_t,
            online_status_text: pj_str_t::new(),
            rpid: pjrpid_element::new(),
            buf_: [0; 80],
        }
    }
}

impl AutoCreate<pjsua_call_setting> for pjsua_call_setting {
    fn new() -> pjsua_call_setting {
        unsafe {

            let mut call_setting = pjsua_call_setting {
                flag: 0,
                req_keyframe_method: 0,
                aud_cnt: 0,
                vid_cnt: 0,
            };

            pjsua_call_setting_default(&mut call_setting as *mut _);
            call_setting
        }
    }
}

impl AutoCreate<pjsua_call_info__bindgen_ty_1> for pjsua_call_info__bindgen_ty_1 {
    fn new() -> pjsua_call_info__bindgen_ty_1 {
        pjsua_call_info__bindgen_ty_1 {
            local_info: [0; 256],
            local_contact: [0; 256],
            remote_info: [0; 256],
            remote_contact: [0; 256],
            call_id: [0; 128],
            last_status_text: [0; 128],
        }
    }
}

impl AutoCreate<pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1> for pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 {
    fn new() -> pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 {
        pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 { conf_slot: 0 }
    }
}

impl AutoCreate<pjsua_call_media_info__bindgen_ty_1> for pjsua_call_media_info__bindgen_ty_1 {
    fn new() -> pjsua_call_media_info__bindgen_ty_1 {
        pjsua_call_media_info__bindgen_ty_1 {
            aud: pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_call_media_info> for pjsua_call_media_info {
    fn new() -> pjsua_call_media_info {
        pjsua_call_media_info {
            index: 0,
            type_: 0,
            dir: 0,
            status: 0,
            stream: pjsua_call_media_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_call_info> for pjsua_call_info {
    fn new() -> pjsua_call_info {
        pjsua_call_info {
            id: -1,
            role: 0,
            acc_id: -1,
            local_info: pj_str_t::new(),
            local_contact: pj_str_t::new(),
            remote_info: pj_str_t::new(),
            remote_contact: pj_str_t::new(),
            call_id: pj_str_t::new(),
            setting: pjsua_call_setting::new(),
            state: 0,
            state_text: pj_str_t::new(),
            last_status: 0,
            last_status_text: pj_str_t::new(),
            media_status: 0,
            media_dir: 0,
            conf_slot: -1,
            media_cnt: 0,
            media: [pjsua_call_media_info::new(); 16],
            prov_media_cnt: 0,
            prov_media: [pjsua_call_media_info::new(); 16],
            connect_duration: pj_time_val::new(),
            total_duration: pj_time_val::new(),
            rem_offerer: PJ_FALSE as pj_bool_t,
            rem_aud_cnt: 0,
            rem_vid_cnt: 0,
            buf_: pjsua_call_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_buddy_info> for pjsua_buddy_info {
    fn new() -> pjsua_buddy_info {
        pjsua_buddy_info {
            id: -1,
            uri: pj_str_t::new(),
            contact: pj_str_t::new(),
            status: 0,
            status_text: pj_str_t::new(),
            monitor_pres: PJ_FALSE as pj_bool_t,
            sub_state: 0,
            sub_state_name: ptr::null_mut(),
            sub_term_code: 0,
            sub_term_reason: pj_str_t::new(),
            rpid: pjrpid_element::new(),
            pres_status: pjsip_pres_status::new(),
            buf_: [0; 512],
        }
    }
}

impl AutoCreate<pjsua_msg_data> for pjsua_msg_data {
    fn new () -> pjsua_msg_data{
        pjsua_msg_data {
            target_uri: pj_str_t::new(),
            hdr_list: pjsip_hdr::new(),
            content_type: pj_str_t::new(),
            msg_body: pj_str_t::new(),
            multipart_ctype: pjsip_media_type::new(),
            multipart_parts: pjsip_multipart_part::new()
        }
    }
}

impl AutoCreate<pjsua_conf_port_info> for pjsua_conf_port_info {
    fn new () -> pjsua_conf_port_info {
        pjsua_conf_port_info {
            slot_id: 0,
            name: pj_str_t::new(),
            format: pjmedia_format::new(),
            clock_rate: 0,
            channel_count: 0,
            samples_per_frame: 0,
            bits_per_sample: 0,
            tx_level_adj: 0.0,
            rx_level_adj: 0.0,
            listener_cnt: 0,
            listeners: [-1; 254usize],
        }
    }
}

impl AutoCreate<pjsua_stream_info__bindgen_ty_1> for pjsua_stream_info__bindgen_ty_1 {
    fn new() -> pjsua_stream_info__bindgen_ty_1 {
        pjsua_stream_info__bindgen_ty_1 {
            aud: pjmedia_stream_info::new(),
        }
    }
}

impl AutoCreate<pjsua_stream_info> for pjsua_stream_info {

    fn new() -> pjsua_stream_info {
        pjsua_stream_info {
            type_: 0,
            info: pjsua_stream_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsua_stream_stat> for pjsua_stream_stat {
    fn new () -> pjsua_stream_stat {
        pjsua_stream_stat {
            rtcp: pjmedia_rtcp_stat::new(),
            jbuf: pjmedia_jb_state::new(),
        }
    }
}


// function helper

// pj_pool_t * 	pjsua_pool_create (const char *name, pj_size_t init_size, pj_size_t increment)
pub fn pool_create(pool_name: &str) -> *mut pj_pool_t {
    unsafe {

        let ret = pjsua_pool_create(
            CString::new(pool_name)
            .expect("String error create pool_name")
            .into_raw(),
            1000,
            1000
        );

        assert_ne!(ret.is_null(), true);
        ret
    }
}

pub fn pool_release(pool: *mut pj_pool_t) {
    unsafe {
        pj_pool_release(pool);
    }
}

pub fn pool_safe_release(ppool: *mut *mut pj_pool_t) {
    unsafe {
        pj_pool_safe_release(ppool);
    }
}

// void 	pjsua_logging_config_default (pjsua_logging_config *cfg)
pub fn logging_config_default(cfg: &mut pjsua_logging_config) {
    unsafe { pjsua_logging_config_default(cfg as *mut _); }
}

// void 	pjsua_config_default (pjsua_config *cfg)
pub fn config_derfault(cfg: &mut pjsua_config) {
    unsafe { pjsua_config_default(cfg as *mut _); }
}

// pj_status_t 	pjsua_create (void)
pub fn create () -> pj_status_t {
    unsafe { pjsua_create() }
}

// pj_status_t 	pjsua_init (const pjsua_config *ua_cfg, const pjsua_logging_config *log_cfg, const pjsua_media_config *media_cfg)
pub fn init (ua_cfg: &mut pjsua_config, log_cfg: &mut pjsua_logging_config, media_cfg: &mut pjsua_media_config) -> pj_status_t {
    unsafe { pjsua_init(
        ua_cfg as *const _,
        log_cfg as *const _,
        media_cfg as *const _
    ) }
}

// pj_status_t 	pjsua_start (void)
pub fn start () -> pj_status_t {
    unsafe { pjsua_start() }
}

// pj_status_t 	pjsua_destroy (void)
pub fn destroy () -> pj_status_t {
    unsafe { pjsua_destroy() }
}

// pjsua_state 	pjsua_get_state (void)
pub fn get_state () -> pjsua_state {
    unsafe { pjsua_get_state() }
}

// pj_status_t 	pjsua_destroy2 (unsigned flags)
pub fn destroy2 (flags: u32) -> pj_status_t {
    unsafe { pjsua_destroy2(flags) }
}

// void 	pjsua_logging_config_dup (pj_pool_t *pool, pjsua_logging_config *dst, const pjsua_logging_config *src)
pub fn logging_config_dup (dst: &mut pjsua_logging_config, src: &mut pjsua_logging_config) {
    unsafe {

        let pool = pool_create("tmp-pool");

        pjsua_logging_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool)
    }
}

// void 	pjsua_config_dup (pj_pool_t *pool, pjsua_config *dst, const pjsua_config *src)
pub fn config_dup (dst: &mut pjsua_config, src: &mut pjsua_config) {
    unsafe {

        let pool = pool_create("tmp-pool");

        pjsua_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool);
    }
}

// void 	pjsua_msg_data_init (pjsua_msg_data *msg_data)
pub fn msg_data_init(msg_data: &mut pjsua_msg_data) {
    unsafe { pjsua_msg_data_init(msg_data as *mut _); }
}

// pjsua_msg_data * 	pjsua_msg_data_clone (pj_pool_t *pool, const pjsua_msg_data *rhs)
pub fn msg_data_clone (rhs: &mut pjsua_msg_data) -> *mut pjsua_msg_data {
    unsafe {

        let pool = pool_create("tmp-pool");

        let ret = pjsua_msg_data_clone(pool, rhs as *const _ );

        pool_release(pool);

        ret
    }
}

// int 	pjsua_handle_events (unsigned msec_timeout)
pub fn handle_events(msec_timeout: u32) -> i32 {
    unsafe { pjsua_handle_events(msec_timeout) }
}

// void 	pjsua_stop_worker_threads (void)
pub fn stop_worker_threads() {
    unsafe { pjsua_stop_worker_threads() }
}

// pj_status_t 	pjsua_reconfigure_logging (const pjsua_logging_config *c)
pub fn reconfigure_logging (c: &mut pjsua_logging_config) -> pj_status_t {
    unsafe {
        pjsua_reconfigure_logging(c as *const _)
    }
}

// pjsip_endpoint * 	pjsua_get_pjsip_endpt (void)
pub fn get_pjsip_endpt() -> *mut pjsip_endpoint {
    unsafe { pjsua_get_pjsip_endpt() }
}

// pjmedia_endpt * 	pjsua_get_pjmedia_endpt (void)
pub fn get_pjmedia_endpt() -> *mut pjmedia_endpt {
    unsafe { pjsua_get_pjmedia_endpt() }
}

// pj_pool_factory * 	pjsua_get_pool_factory (void)
pub fn get_pool_factory() -> *mut pj_pool_factory {
    unsafe { pjsua_get_pool_factory() }
}

// void 	pjsua_ip_change_param_default (pjsua_ip_change_param *param)
pub fn ip_change_param_default(param: &mut pjsua_ip_change_param) {
    unsafe { pjsua_ip_change_param_default(param as *mut _) }
}

// pj_status_t 	pjsua_detect_nat_type (void)
pub fn detect_nat_type () -> pj_status_t {
    unsafe { pjsua_detect_nat_type() }
}

// pj_status_t 	pjsua_get_nat_type (pj_stun_nat_type *type)
pub fn get_nat_type(type_: &mut pj_stun_nat_type) -> pj_status_t {
    unsafe { pjsua_get_nat_type(type_ as *mut _) }
}

// pj_status_t 	pjsua_update_stun_servers (unsigned count, pj_str_t srv[], pj_bool_t wait)
pub fn update_stun_servers (count: u32, srv: &mut [pj_str_t; 8], wait: bool) -> pj_status_t {
    unsafe {
        let mut a_wait = PJ_FALSE as pj_bool_t;

        if wait {
            a_wait = PJ_TRUE as pj_bool_t;
        }

        // todo fix this and compare result with c code.
        pjsua_update_stun_servers(
                count,
                srv.as_mut_ptr(),
                a_wait
            )

    }
}

// pj_status_t 	pjsua_resolve_stun_servers (unsigned count, pj_str_t srv[], pj_bool_t wait, void *token, pj_stun_resolve_cb cb)
pub fn resolve_stun_servers (
    count: u32,
    srv: &mut [pj_str_t; 8],
    wait: bool, ) {
        // todo
}

// this function mosly unsued
// pj_status_t 	pjsua_cancel_stun_resolution (void *token, pj_bool_t notify_cb)

// pj_status_t 	pjsua_verify_sip_url (const char *url)
pub fn verify_sip_url(url: String) -> pj_status_t {
    unsafe {
        pjsua_verify_sip_url(
            CString::new(url).expect("error url").into_raw()
        )
    }
}

// pj_status_t 	pjsua_verify_url (const char *url)
pub fn verify_url (url: String) -> pj_status_t {
    unsafe {
        pjsua_verify_url (
            CString::new(url).expect("error url").into_raw()
        )
    }
}

// pj_status_t 	pjsua_schedule_timer (pj_timer_entry *entry, const pj_time_val *delay)
pub fn schedule_timer (entry: &mut pj_timer_entry, delay: &mut pj_time_val) -> pj_status_t {
    unsafe {
        // because we use debug pjsua
        // will provide timer with debug suport
        pjsua_schedule_timer_dbg(
            entry as *mut _,
            delay as *const _,
            ptr::null_mut(),
            0
        )
     }
}

// pj_status_t 	pjsua_schedule_timer2 (void(*cb)(void *user_data), void *user_data, unsigned msec_delay)

// void 	pjsua_cancel_timer (pj_timer_entry *entry)
pub fn cancel_timer(entry: &mut pj_timer_entry) {
    unsafe { pjsua_cancel_timer(entry as *mut _) }
}

// void 	pjsua_perror (const char *sender, const char *title, pj_status_t status)
pub fn perror(sender: &str, title: &str, status: pj_status_t) {
    unsafe {
        pjsua_perror(
                CString::new(sender).expect("error sender string").into_raw() as *const _,
                CString::new(title).expect("error title string").into_raw() as *const _,
                status
            );
    }
}

// void 	pjsua_dump (pj_bool_t detail)
pub fn dump(detail: bool) {

    let mut adetail = PJ_FALSE as pj_bool_t;

    if detail {
        adetail = PJ_TRUE as pj_bool_t;
    }

    unsafe { pjsua_dump(adetail); }
}

// pj_status_t 	pjsua_handle_ip_change (const pjsua_ip_change_param *param)
pub fn handle_ip_change(param: &mut pjsua_ip_change_param) -> pj_status_t {
    unsafe { pjsua_handle_ip_change( param as *const _ ) }
}


// conference helper
pub fn conf_connect(source: pjsua_conf_port_id, sink: pjsua_conf_port_id) -> pj_status_t {
    unsafe {
        pjsua_conf_connect(source, sink)
    }
}



// account helper function
pub fn acc_get_info(acc_id: pjsua_acc_id, info: &mut pjsua_acc_info) -> pj_status_t {
    unsafe {
        pjsua_acc_get_info(acc_id, info as *mut _)
    }
}

pub fn acc_get_config(acc_id: pjsua_acc_id, acc_cfg: &mut pjsua_acc_config) -> pj_status_t {
    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_acc_get_config(acc_id, pool, acc_cfg as *mut _);

        pool_release(pool);

        status
    }
}

pub fn acc_config_default(cfg: &mut pjsua_acc_config) {
    unsafe { pjsua_acc_config_default(cfg as *mut _); }
}

pub fn acc_set_default(acc_id: pjsua_acc_id) -> pj_status_t {
    unsafe { pjsua_acc_set_default(acc_id) }
}


// call helper function

// void 	pjsua_call_setting_default (pjsua_call_setting *opt)
pub fn call_setting_default(opt: &mut pjsua_call_setting) {
    unsafe { pjsua_call_setting_default(opt as * mut _) }
}

// void 	pjsua_call_send_dtmf_param_default (pjsua_call_send_dtmf_param *param)
pub fn call_send_dtmf_param_default(param: &mut pjsua_call_send_dtmf_param) {
    unsafe { pjsua_call_send_dtmf_param_default(param as *mut _) }
}

// unsigned 	pjsua_call_get_max_count (void)
pub fn call_get_max_count () -> u32 {
    unsafe { pjsua_call_get_max_count() }
}

// unsigned 	pjsua_call_get_count (void)
// pj_status_t 	pjsua_enum_calls (pjsua_call_id ids[], unsigned *count)
// pj_status_t 	pjsua_call_make_call (pjsua_acc_id acc_id, const pj_str_t *dst_uri, const pjsua_call_setting *opt, void *user_data, const pjsua_msg_data *msg_data, pjsua_call_id *p_call_id)
// pj_bool_t 	pjsua_call_is_active (pjsua_call_id call_id)
// pj_bool_t 	pjsua_call_has_media (pjsua_call_id call_id)
// pjsua_conf_port_id 	pjsua_call_get_conf_port (pjsua_call_id call_id)
// pj_status_t 	pjsua_call_get_info (pjsua_call_id call_id, pjsua_call_info *info)
// pjsip_dialog_cap_status 	pjsua_call_remote_has_cap (pjsua_call_id call_id, int htype, const pj_str_t *hname, const pj_str_t *token)
// pj_status_t 	pjsua_call_set_user_data (pjsua_call_id call_id, void *user_data)
// void * 	pjsua_call_get_user_data (pjsua_call_id call_id)
// pj_status_t 	pjsua_call_get_rem_nat_type (pjsua_call_id call_id, pj_stun_nat_type *p_type)
// pj_status_t 	pjsua_call_answer (pjsua_call_id call_id, unsigned code, const pj_str_t *reason, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_answer2 (pjsua_call_id call_id, const pjsua_call_setting *opt, unsigned code, const pj_str_t *reason, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_answer_with_sdp (pjsua_call_id call_id, const pjmedia_sdp_session *sdp, const pjsua_call_setting *opt, unsigned code, const pj_str_t *reason, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_hangup (pjsua_call_id call_id, unsigned code, const pj_str_t *reason, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_process_redirect (pjsua_call_id call_id, pjsip_redirect_op cmd)
// pj_status_t 	pjsua_call_set_hold (pjsua_call_id call_id, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_set_hold2 (pjsua_call_id call_id, unsigned options, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_reinvite (pjsua_call_id call_id, unsigned options, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_reinvite2 (pjsua_call_id call_id, const pjsua_call_setting *opt, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_update (pjsua_call_id call_id, unsigned options, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_update2 (pjsua_call_id call_id, const pjsua_call_setting *opt, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_xfer (pjsua_call_id call_id, const pj_str_t *dest, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_xfer_replaces (pjsua_call_id call_id, pjsua_call_id dest_call_id, unsigned options, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_dial_dtmf (pjsua_call_id call_id, const pj_str_t *digits)
// pj_status_t 	pjsua_call_send_dtmf (pjsua_call_id call_id, const pjsua_call_send_dtmf_param *param)
// pj_status_t 	pjsua_call_send_im (pjsua_call_id call_id, const pj_str_t *mime_type, const pj_str_t *content, const pjsua_msg_data *msg_data, void *user_data)
// pj_status_t 	pjsua_call_send_typing_ind (pjsua_call_id call_id, pj_bool_t is_typing, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_call_send_request (pjsua_call_id call_id, const pj_str_t *method, const pjsua_msg_data *msg_data)
// void 	pjsua_call_hangup_all (void)
// pj_status_t 	pjsua_call_dump (pjsua_call_id call_id, pj_bool_t with_media, char *buffer, unsigned maxlen, const char *indent)
// int 	pjsua_call_get_vid_stream_idx (pjsua_call_id call_id)
// pj_status_t 	pjsua_call_get_stream_info (pjsua_call_id call_id, unsigned med_idx, pjsua_stream_info *psi)
// pj_status_t 	pjsua_call_get_stream_stat (pjsua_call_id call_id, unsigned med_idx, pjsua_stream_stat *stat)
// pj_status_t 	pjsua_call_get_med_transport_info (pjsua_call_id call_id, unsigned med_idx, pjmedia_transport_info *t)

// void 	pjsua_call_vid_strm_op_param_default (pjsua_call_vid_strm_op_param *param)
// pjsua_vid_win_id 	pjsua_call_get_vid_win (pjsua_call_id call_id)
// pjsua_conf_port_id 	pjsua_call_get_vid_conf_port (pjsua_call_id call_id, pjmedia_dir dir)
// pj_bool_t 	pjsua_call_vid_stream_is_running (pjsua_call_id call_id, int med_idx, pjmedia_dir dir)
// pj_status_t 	pjsua_call_set_vid_strm (pjsua_call_id call_id, pjsua_call_vid_strm_op op, const pjsua_call_vid_strm_op_param *param)

pub fn call_answer2(
    call_id: pjsua_call_id,
    opt: &mut pjsua_call_setting,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut pjsua_msg_data>
) -> pj_status_t {

    let reason: *const pj_str_t = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const _ ,
        None => ptr::null_mut()
    };

    let msg_data: *const pjsua_msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        pjsua_call_answer2(
            call_id,
            opt,
            code,
            reason,
            msg_data
        )
    }
}

pub fn call_hangup(
    call_id: pjsua_call_id,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut pjsua_msg_data>
) -> pj_status_t {

    let reason: *const pj_str_t = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const _,
        None => ptr::null_mut()
    };

    let msg_data: *const pjsua_msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        pjsua_call_hangup(call_id, code, reason, msg_data)
    }
}

