#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use super::pjdefault::AutoCreate;

use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsip_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;

use std::os::raw::{c_int, c_uint, c_void};
use std::ptr;



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
        }

        config
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

impl AutoCreate<pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1>
    for pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1
{
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




