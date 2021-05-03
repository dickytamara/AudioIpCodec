
use pj_sys::{pj_qos_params, pj_sockaddr, pj_sockopt_params, pj_str_t, pj_time_val};
use pjmedia_sys::{pjmedia_format, pjmedia_jb_state, pjmedia_rtcp_fb_setting, pjmedia_rtcp_stat, pjmedia_vid_stream_info, pjmedia_vid_stream_rc_config, pjmedia_vid_stream_sk_config};
use pjnath_sys::{pj_ice_sess_options, pj_stun_auth_cred };
use pjsip_simple_sys::{pjrpid_element, pjsip_pres_status, pjsip_publishc_opt};
use pjsip_sys::{pjsip_auth_clt_pref, pjsip_cred_info, pjsip_hdr, pjsip_host_port, pjsip_media_type, pjsip_multipart_part, pjsip_tls_setting};
use pjsip_ua_sys::pjsip_timer_setting;

use super::utils::AutoCreate;
use super::*;

impl AutoCreate<SRTPOption> for SRTPOption {
    fn new() -> Self {
        Self {
            crypto_count: 0,
            crypto: [
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
                pjmedia_srtp_crypto::new(), pjmedia_srtp_crypto::new(),
            ],
            keying_count: 0,
            keying: [0, 0],
        }
    }
}

impl AutoCreate<UACallback> for UACallback {
    fn new() -> Self {
        Self {
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
            on_transport_state: None,
            on_call_media_transport_state: None,
            on_ice_transport_error: None,
            on_snd_dev_operation: None,
            on_call_media_event: None,
            on_create_media_transport: None,
            on_create_media_transport_srtp: None,
            on_acc_find_for_incoming: None,
            on_stun_resolution_complete: None,
            on_ip_change_progress: None,
            on_media_event: None,
        }
    }
}

impl AutoCreate<LogConfig> for LogConfig {

    fn new() -> Self {

        let mut config = Self {
            msg_logging: 0,
            level: 0,
            console_level: 0,
            decor: 0,
            log_filename: pj_str_t::new(),
            log_file_flags: 0,
            cb: None,
        };

        unsafe {
            pjsua_sys::pjsua_logging_config_default(&mut config as *mut _);
        }
        config.level = 5;
        config.console_level= 5;

        config
    }
}

impl AutoCreate<UAConfig> for UAConfig {
    fn new() -> Self {
        let mut config = Self {
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
            stun_try_ipv6: 0,
            stun_ignore_failure: 0,
            stun_map_use_stun2: 0,
            nat_type_in_sdp: 0,
            require_100rel: 0,
            use_timer: 0,
            enable_unsolicited_mwi: 0,
            timer_setting: pjsip_timer_setting {
                min_se: 0,
                sess_expires: 0,
            },
            cred_count: 0,
            cred_info: [
                pjsip_cred_info::new(), pjsip_cred_info::new(),
                pjsip_cred_info::new(), pjsip_cred_info::new(),
                pjsip_cred_info::new(), pjsip_cred_info::new(),
                pjsip_cred_info::new(), pjsip_cred_info::new(),
            ],
            cb: UACallback::new(),
            user_agent: pj_str_t::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: 0,
            srtp_opt: pjsua_sys::pjsua_srtp_opt::new(),
            hangup_forked_call: 0,
        };

        // set with pjsua default
        unsafe {
            pjsua_sys::pjsua_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<MediaConfig> for MediaConfig {
    fn new() -> Self {
        let mut config = Self {
            clock_rate: 0,
            snd_clock_rate: 0,
            channel_count: 0,
            audio_frame_ptime: 0,
            max_media_ports: 0,
            has_ioqueue: 0,
            thread_cnt: 0,
            quality: 0,
            ptime: 0,
            no_vad: 0,
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
            enable_ice: 0,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: 0,
            ice_always_update: 0,
            enable_turn: 0,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
            snd_auto_close_time: 0,
            vid_preview_enable_native: 0,
            no_smart_media_update: 0,
            no_rtcp_sdes_bye: 0,
            on_aud_prev_play_frame: None,
            on_aud_prev_rec_frame: None,
        };

        unsafe {
            pjsua_sys::pjsua_media_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<TransportConfig> for TransportConfig {
    fn new() -> Self {
        let mut config = Self {
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
            pjsua_sys::pjsua_transport_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<ICEConfig> for ICEConfig {
    fn new() -> Self {
        Self {
            enable_ice: 0,
            ice_max_host_cands: 0,
            ice_opt: pj_ice_sess_options::new(),
            ice_no_rtcp: 0,
            ice_always_update: 0,
        }
    }
}

impl AutoCreate<TurnConfig> for TurnConfig {
    fn new() -> Self {
        Self {
            enable_turn: 0,
            turn_server: pj_str_t::new(),
            turn_conn_type: 0,
            turn_auth_cred: pj_stun_auth_cred::new(),
            turn_tls_setting: pj_turn_sock_tls_cfg::new(),
        }
    }
}

impl AutoCreate<IPChangeAccountConfig> for IPChangeAccountConfig {
    fn new() -> Self {
        Self {
            shutdown_tp: 0,
            hangup_calls: 0,
            reinvite_flags: 0,
        }
    }
}

impl AutoCreate<AccountConfig> for AccountConfig {
    fn new() -> Self {
        let mut config = Self {
            user_data: ptr::null_mut(),
            priority: 0,
            id: pj_str_t::new(),
            reg_uri: pj_str_t::new(),
            reg_hdr_list: pjsip_hdr::new(),
            reg_contact_params: pj_str_t::new(),
            sub_hdr_list: pjsip_hdr::new(),
            mwi_enabled: 0,
            mwi_expires: 0,
            publish_enabled: 0,
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
            cred_info: [
                pjsip_cred_info::new(), pjsip_cred_info::new(),
                pjsip_cred_info::new(), pjsip_cred_info::new(),
                pjsip_cred_info::new(), pjsip_cred_info::new(),
                pjsip_cred_info::new(), pjsip_cred_info::new(),
            ],
            transport_id: 0,
            allow_contact_rewrite: 0,
            contact_rewrite_method: 0,
            contact_use_src_port: 0,
            allow_via_rewrite: 0,
            allow_sdp_nat_rewrite: 0,
            use_rfc5626: 0,
            rfc5626_instance_id: pj_str_t::new(),
            rfc5626_reg_id: pj_str_t::new(),
            ka_interval: 0,
            ka_data: pj_str_t::new(),
            vid_in_auto_show: 0,
            vid_out_auto_transmit: 0,
            vid_wnd_flags: 0,
            vid_cap_dev: 0,
            vid_rend_dev: 0,
            vid_stream_rc_cfg: pjmedia_vid_stream_rc_config::new(),
            vid_stream_sk_cfg: pjmedia_vid_stream_sk_config::new(),
            rtp_cfg: TransportConfig::new(),
            nat64_opt: 0,
            ipv6_media_use: 0,
            sip_stun_use: 0,
            media_stun_use: 0,
            use_loop_med_tp: 0,
            enable_loopback: 0,
            ice_cfg_use: 0,
            ice_cfg: ICEConfig::new(),
            turn_cfg_use: 0,
            turn_cfg: TurnConfig::new(),
            use_srtp: 0,
            srtp_secure_signaling: 0,
            srtp_optional_dup_offer: 0,
            srtp_opt: SRTPOption::new(),
            reg_retry_interval: 0,
            reg_first_retry_interval: 0,
            reg_retry_random_interval: 0,
            drop_calls_on_reg_fail: 0,
            reg_use_proxy: 0,
            call_hold_type: 0,
            register_on_acc_add: 0,
            ip_change_cfg: IPChangeAccountConfig::new(),
            enable_rtcp_mux: 0,
            rtcp_fb_cfg: pjmedia_rtcp_fb_setting::new(),
        };

        unsafe {
            pjsua_sys::pjsua_acc_config_default(&mut config as *mut _);


        config.cred_count = 1;
        config.reg_retry_interval = 300;
        config.reg_first_retry_interval = 60;
        config.cred_info[0].data_type = 0;
        config.cred_info[0].scheme = pj_str_t::from_string(String::from("Digest"));

        config

        }
    }
}

impl AutoCreate<BuddyConfig> for BuddyConfig {
    fn new() -> Self {
        let mut config = Self {
            uri: pj_str_t::new(),
            subscribe: 0,
            user_data: ptr::null_mut(),
        };

        unsafe {
            pjsua_sys::pjsua_buddy_config_default(&mut config as *mut _);
        }

        config
    }
}

impl AutoCreate<TransportInfo> for TransportInfo {
    fn new() -> Self {
        Self {
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

impl AutoCreate<AccountInfo> for AccountInfo {
    fn new() -> Self {
        Self {
            id: -1,
            is_default: 0,
            acc_uri: pj_str_t::new(),
            has_registration: 0,
            expires: 0,
            status: 0,
            reg_last_err: 0,
            status_text: pj_str_t::new(),
            online_status: 0,
            online_status_text: pj_str_t::new(),
            rpid: pjrpid_element::new(),
            buf_: [0; 80],
        }
    }
}

impl AutoCreate<CallSetting> for CallSetting {
    fn new() -> Self {

        let mut ret = Self {
            flag: 0,
            req_keyframe_method: 0,
            aud_cnt: 0,
            vid_cnt: 0,
        };

        unsafe {
            pjsua_sys::pjsua_call_setting_default(&mut ret as *mut _);
        }

        ret.aud_cnt = 1;
        ret.vid_cnt = 0;

        ret
    }
}

impl AutoCreate<pjsua_sys::pjsua_call_info__bindgen_ty_1> for pjsua_sys::pjsua_call_info__bindgen_ty_1 {
    fn new() -> Self {
        Self {
            local_info: [0; 256],
            local_contact: [0; 256],
            remote_info: [0; 256],
            remote_contact: [0; 256],
            call_id: [0; 128],
            last_status_text: [0; 128],
        }
    }
}

impl AutoCreate<pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1> for pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1 {
    fn new() -> Self {
        Self { conf_slot: 0 }
    }
}

impl AutoCreate<pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_2> for pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_2 {
    fn new() -> Self {
        Self {
            win_in: -1,
            dec_slot: -1,
            enc_slot: -1,
            cap_dev: -1,
        }
    }
}

impl AutoCreate<pjsua_sys::pjsua_call_media_info__bindgen_ty_1> for pjsua_sys::pjsua_call_media_info__bindgen_ty_1 {
    fn new() -> Self {
        let mut result = Self {
            aud: pjsua_sys::__BindgenUnionField::<pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1>::default(),
            vid: pjsua_sys::__BindgenUnionField::<pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_2>::default(),
            bindgen_union_field: [0; 4usize]
        };

        unsafe {
            *result.aud.as_mut() = pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_1::new();
            *result.vid.as_mut() = pjsua_sys::pjsua_call_media_info__bindgen_ty_1__bindgen_ty_2::new();
        }

        result
    }
}

impl AutoCreate<CallMediaInfo> for CallMediaInfo {
    fn new() -> Self {
        Self {
            index: 0,
            type_: 0,
            dir: 0,
            status: 0,
            stream: pjsua_sys::pjsua_call_media_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<CallInfo> for CallInfo {
    fn new() -> Self {
        Self {
            id: -1,
            role: 0,
            acc_id: -1,
            local_info: pj_str_t::new(),
            local_contact: pj_str_t::new(),
            remote_info: pj_str_t::new(),
            remote_contact: pj_str_t::new(),
            call_id: pj_str_t::new(),
            setting: CallSetting::new(),
            state: 0,
            state_text: pj_str_t::new(),
            last_status: 0,
            last_status_text: pj_str_t::new(),
            media_status: 0,
            media_dir: 0,
            conf_slot: -1,
            media_cnt: 0,
            media: [
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
            ],
            prov_media_cnt: 0,
            prov_media: [
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
                CallMediaInfo::new(), CallMediaInfo::new(),
            ],
            connect_duration: pj_time_val::new(),
            total_duration: pj_time_val::new(),
            rem_offerer: 0,
            rem_aud_cnt: 0,
            rem_vid_cnt: 0,
            buf_: pjsua_sys::pjsua_call_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<BuddyInfo> for BuddyInfo {
    fn new() -> Self {
        Self {
            id: -1,
            uri: pj_str_t::new(),
            contact: pj_str_t::new(),
            status: 0,
            status_text: pj_str_t::new(),
            monitor_pres: 0,
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

impl AutoCreate<MessageData> for MessageData {
    fn new () -> Self {
        let mut ret = Self {
            target_uri: pj_str_t::new(),
            hdr_list: pjsip_hdr::new(),
            content_type: pj_str_t::new(),
            msg_body: pj_str_t::new(),
            multipart_ctype: pjsip_media_type::new(),
            multipart_parts: pjsip_multipart_part::new()
        };

        unsafe {
            pjsua_sys::pjsua_msg_data_init(&mut ret as *mut _);
        }

        ret
    }
}

impl AutoCreate<ConferencePortInfo> for ConferencePortInfo {
    fn new () -> Self {
        Self {
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

impl AutoCreate<pjsua_sys::pjsua_stream_info__bindgen_ty_1> for pjsua_sys::pjsua_stream_info__bindgen_ty_1 {
    fn new() -> pjsua_sys::pjsua_stream_info__bindgen_ty_1 {
        let mut result = Self {
            aud: pjsua_sys::__BindgenUnionField::<pjmedia_stream_info>::default(),
            vid: pjsua_sys::__BindgenUnionField::<pjmedia_vid_stream_info>::default(),
            bindgen_union_field: [0; 277usize],
        };

        unsafe {
            *result.aud.as_mut() = pjmedia_stream_info::new();
            *result.vid.as_mut() = pjmedia_vid_stream_info::new();
        }

        result
    }
}

impl AutoCreate<StreamInfo> for StreamInfo {

    fn new() -> Self {
        Self {
            type_: 0,
            info: pjsua_sys::pjsua_stream_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<StreamStatus> for StreamStatus {
    fn new () -> Self {
        Self {
            rtcp: pjmedia_rtcp_stat::new(),
            jbuf: pjmedia_jb_state::new(),
        }
    }
}

impl AutoCreate<CodecInfo> for CodecInfo {
    fn new () -> Self {
        CodecInfo {
            codec_id: pj_str_t::from_string(String::new()),
            priority: 0,
            desc: pj_str_t::from_string(String::new()),
            buf_: [0; 64usize],
        }
    }
}

