
use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;

use std::os::raw::c_void;
use std::mem;

impl AutoCreate<pjsua_srtp_opt> for pjsua_srtp_opt {
    fn new() -> pjsua_srtp_opt {
        pjsua_srtp_opt {
            crypto_count: 0,
            crypto: [pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                 pjmedia_srtp_crypto::new(),
                ],
            keying_count: 0,
            keying: [0, 0],
        }
    }
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
            on_media_event: None
        }
    }
}


impl AutoCreate<pjsua_logging_config> for pjsua_logging_config {
    fn new() -> pjsua_logging_config {
        let mut config = pjsua_logging_config {
            msg_logging: pj_constants__PJ_FALSE as pj_bool_t,
            level: 0,
            console_level: 0,
            decor: 0,
            log_filename: pj_str_t::new(),
            log_file_flags: 0,
            cb: None
        };

        unsafe {
            pjsua_logging_config_default(&mut config as *mut _);
        }

        config
    }
}


impl AutoCreate<pjsua_config> for pjsua_config {
    //
    fn new () -> pjsua_config {
        let mut config = pjsua_config {
            max_calls: 0,
            thread_cnt: 0,
            nameserver_count: 0,
            nameserver: [pj_str_t::new(), pj_str_t::new(), pj_str_t::new(), pj_str_t::new()],
            force_lr: 0,
            outbound_proxy_cnt: 0,
            outbound_proxy: [pj_str_t::new(), pj_str_t::new(), pj_str_t::new(), pj_str_t::new()],
            stun_domain: pj_str_t::new(),
            stun_host: pj_str_t::new(),
            stun_srv_cnt: 0,
            stun_srv: [ pj_str_t::new(), pj_str_t::new(), pj_str_t::new(), pj_str_t::new(),
                        pj_str_t::new(), pj_str_t::new(), pj_str_t::new(), pj_str_t::new()
                    ],
            stun_try_ipv6: pj_constants__PJ_FALSE as pj_bool_t,
            stun_ignore_failure: pj_constants__PJ_FALSE as pj_bool_t,
            stun_map_use_stun2: pj_constants__PJ_FALSE as pj_bool_t,
            nat_type_in_sdp: 0,
            require_100rel: pj_constants__PJ_FALSE,
            use_timer: pj_constants__PJ_FALSE,
            enable_unsolicited_mwi: pj_constants__PJ_FALSE as pj_bool_t,
            timer_setting: pjsip_timer_setting { min_se: 0, sess_expires: 0, },
            cred_count: 0,
            cred_info: [pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new(),
                        pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new()],
            cb: pjsua_callback::new(),
            user_agent: pj_str_t::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: pj_constants__PJ_FALSE as pj_bool_t,
            srtp_opt: pjsua_srtp_opt::new(),
            hangup_forked_call: pj_constants__PJ_FALSE as pj_bool_t
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
            has_ioqueue: pj_constants__PJ_FALSE as pj_bool_t,
            thread_cnt: 0,
            quality: 0,
            ptime: 0,
            no_vad: pj_constants__PJ_FALSE as pj_bool_t,
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
            enable_ice: pj_constants__PJ_FALSE as pj_bool_t,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: pj_constants__PJ_FALSE as pj_bool_t,
            ice_always_update: pj_constants__PJ_FALSE as pj_bool_t,
            enable_turn: pj_constants__PJ_FALSE as pj_bool_t,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0 as pj_turn_tp_type,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
            snd_auto_close_time: 0,
            vid_preview_enable_native: pj_constants__PJ_FALSE as pj_bool_t,
            no_smart_media_update: pj_constants__PJ_FALSE as pj_bool_t,
            no_rtcp_sdes_bye: pj_constants__PJ_FALSE as pj_bool_t,
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
            queue_request: pj_constants__PJ_FALSE as pj_bool_t
        }
    }
}

impl AutoCreate<pjsip_auth_clt_pref> for pjsip_auth_clt_pref {
    fn new() -> pjsip_auth_clt_pref {
        pjsip_auth_clt_pref {
            initial_auth: pj_constants__PJ_FALSE as pj_bool_t,
            algorithm: pj_str_t::new()
        }
    }
}


impl AutoCreate<pjsip_timer_setting> for pjsip_timer_setting {
    fn new () -> pjsip_timer_setting {
        pjsip_timer_setting {
            min_se: 0,
            sess_expires: 0
        }
    }
}


impl AutoCreate<pjsua_transport_config> for pjsua_transport_config {
    fn new () -> pjsua_transport_config {
        let mut config = pjsua_transport_config {
            port: 0,
            port_range: 0,
            public_addr: pj_str_t::new(),
            bound_addr: pj_str_t::new(),
            tls_setting: pjsip_tls_setting::new(),
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            sockopt_params: pj_sockopt_params::new()
        };

        unsafe {
            pjsua_transport_config_default(&mut config as *mut _);
        }

        config
    }
}


impl AutoCreate<pjsua_ice_config> for pjsua_ice_config {
    fn new () -> pjsua_ice_config {
        pjsua_ice_config {
            enable_ice: pj_constants__PJ_FALSE as pj_bool_t,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: pj_constants__PJ_FALSE as pj_bool_t,
            ice_always_update: pj_constants__PJ_FALSE as pj_bool_t
        }
    }
}


impl AutoCreate<pjsua_turn_config> for pjsua_turn_config {
    fn new() -> pjsua_turn_config {
        pjsua_turn_config {
            enable_turn: pj_constants__PJ_FALSE as pj_bool_t,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
        }
    }
}


impl AutoCreate<pjsua_ip_change_acc_cfg> for pjsua_ip_change_acc_cfg {
    fn new () -> pjsua_ip_change_acc_cfg {
        pjsua_ip_change_acc_cfg {
            shutdown_tp: pj_constants__PJ_FALSE as pj_bool_t,
            hangup_calls: pj_constants__PJ_FALSE as pj_bool_t,
            reinvite_flags: 0
        }
    }
}


impl AutoCreate<pjsua_acc_config> for pjsua_acc_config {
    fn new () -> pjsua_acc_config {
        unsafe {
            let mut void_z: c_void = mem::zeroed();
            let mut config = pjsua_acc_config {
                user_data: &mut void_z as *mut _,
                priority: 0,
                id: pj_str_t::new(),
                reg_uri: pj_str_t::new(),
                reg_hdr_list: pjsip_hdr::new(),
                reg_contact_params: pj_str_t::new(),
                sub_hdr_list: pjsip_hdr::new(),
                mwi_enabled: pj_constants__PJ_FALSE as pj_bool_t,
                mwi_expires: 0,
                publish_enabled: pj_constants__PJ_FALSE as pj_bool_t,
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
                proxy: [pj_str_t::new(), pj_str_t::new(), pj_str_t::new(), pj_str_t::new(),
                        pj_str_t::new(), pj_str_t::new(), pj_str_t::new(), pj_str_t::new()],
                lock_codec: 0,
                reg_timeout: 0,
                reg_delay_before_refresh: 0,
                unreg_timeout: 0,
                cred_count: 0,
                cred_info: [pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new(),
                            pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new(), pjsip_cred_info::new()],
                transport_id: 0,
                allow_contact_rewrite: pj_constants__PJ_FALSE as pj_bool_t,
                contact_rewrite_method: 0,
                contact_use_src_port: pj_constants__PJ_FALSE as pj_bool_t,
                allow_via_rewrite: pj_constants__PJ_FALSE as pj_bool_t,
                allow_sdp_nat_rewrite: pj_constants__PJ_FALSE as pj_bool_t,
                use_rfc5626: 0,
                rfc5626_instance_id: pj_str_t::new(),
                rfc5626_reg_id: pj_str_t::new(),
                ka_interval: 0,
                ka_data: pj_str_t::new(),
                vid_in_auto_show: pj_constants__PJ_FALSE as pj_bool_t,
                vid_out_auto_transmit: pj_constants__PJ_FALSE as pj_bool_t,
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
                use_loop_med_tp: pj_constants__PJ_FALSE as pj_bool_t,
                enable_loopback: pj_constants__PJ_FALSE as pj_bool_t,
                ice_cfg_use: 0,
                ice_cfg: pjsua_ice_config::new(),
                turn_cfg_use: 0,
                turn_cfg: pjsua_turn_config::new(),
                use_srtp: 0,
                srtp_secure_signaling: 0,
                srtp_optional_dup_offer: pj_constants__PJ_FALSE as pj_bool_t,
                srtp_opt: pjsua_srtp_opt::new(),
                reg_retry_interval: 0,
                reg_first_retry_interval: 0,
                reg_retry_random_interval: 0,
                drop_calls_on_reg_fail: pj_constants__PJ_FALSE as pj_bool_t,
                reg_use_proxy: 0,
                call_hold_type: 0,
                register_on_acc_add: pj_constants__PJ_FALSE as pj_bool_t,
                ip_change_cfg: pjsua_ip_change_acc_cfg::new(),
                enable_rtcp_mux: pj_constants__PJ_FALSE as pj_bool_t,
                rtcp_fb_cfg: pjmedia_rtcp_fb_setting::new(),
            };

            pjsua_acc_config_default(&mut config as *mut _);
            config
        }
    }
}

impl AutoCreate<pjsua_buddy_config> for pjsua_buddy_config {
    fn new () -> pjsua_buddy_config {
        unsafe {
            let mut void_z: c_void = mem::zeroed();
            let mut config = pjsua_buddy_config {
                uri: pj_str_t::new(),
                subscribe: pj_constants__PJ_FALSE as pj_bool_t,
                user_data: &mut void_z as *mut _,
            };

            pjsua_buddy_config_default(&mut config as *mut _);
            config
        }
    }
}
