
use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;


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
        pjsua_logging_config {
            msg_logging: pj_constants__PJ_FALSE as pj_bool_t,
            level: 0,
            console_level: 0,
            decor: 0,
            log_filename: pj_str_t::new(),
            log_file_flags: 0,
            cb: None
        }
    }
}


impl AutoCreate<pjsua_config> for pjsua_config {
    //
    fn new () -> pjsua_config {
        pjsua_config {
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
        }
    }
}
