// #![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use pj_sys::*;
use pjmedia_sys::*;
use pjsip_sys::*;
use pjsip_simple_sys::*;
use pjsua_sys::*;

use super::prelude::*;
use super::utils;


use std::os::raw::{c_uint, c_void};
use std::ffi::CString;
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

pub fn conf_get_msignal_level(
    slot: pjsua_conf_port_id,
    tx_level_l: &mut u32,
    tx_level_r: &mut u32,
    rx_level_l: &mut u32,
    rx_level_r: &mut u32
) -> pj_status_t {

    unsafe {
        pjsua_conf_get_msignal_level(
            slot,
            tx_level_l as *mut _,
            tx_level_r as *mut _,
            rx_level_l as *mut _,
            rx_level_r as *mut _
        )
    }

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
        config.level = 5;
        config.console_level= 5;

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

        let mut ret = pjsua_call_setting {
            flag: 0,
            req_keyframe_method: 0,
            aud_cnt: 0,
            vid_cnt: 0,
        };

        unsafe {
            pjsua_call_setting_default(&mut ret as *mut _);
        }

        ret.aud_cnt = 1;
        ret.vid_cnt = 0;

        ret
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
        let mut ret = pjsua_msg_data {
            target_uri: pj_str_t::new(),
            hdr_list: pjsip_hdr::new(),
            content_type: pj_str_t::new(),
            msg_body: pj_str_t::new(),
            multipart_ctype: pjsip_media_type::new(),
            multipart_parts: pjsip_multipart_part::new()
        };

        unsafe {
            pjsua_msg_data_init(&mut ret as *mut _);
        }

        ret
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

impl AutoCreate<pjsua_codec_info> for pjsua_codec_info {
    fn new () -> pjsua_codec_info {
        pjsua_codec_info {
            codec_id: pj_str_t::from_string(String::new()),
            priority: 0,
            desc: pj_str_t::from_string(String::new()),
            buf_: [0; 64usize],
        }
    }
}


// function helper

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

pub fn logging_config_default(cfg: &mut pjsua_logging_config) {
    unsafe { pjsua_logging_config_default(cfg as *mut _); }
}

pub fn config_default(cfg: &mut pjsua_config) {
    unsafe { pjsua_config_default(cfg as *mut _); }
}

pub fn create () -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_create()) }
}

pub fn init (ua_cfg: &mut pjsua_config, log_cfg: &mut pjsua_logging_config, media_cfg: &mut pjsua_media_config) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjsua_init(
        ua_cfg as *const _,
        log_cfg as *const _,
        media_cfg as *const _
        );

        utils::check_status(status)
    }
}

pub fn start () -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_start()) }
}

pub fn destroy () -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_destroy()) }
}

pub fn get_state () -> pjsua_state {
    unsafe { pjsua_get_state() }
}

pub fn destroy2 (flags: u32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_destroy2(flags)) }
}

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

pub fn msg_data_init(msg_data: &mut pjsua_msg_data) {
    unsafe { pjsua_msg_data_init(msg_data as *mut _); }
}

pub fn msg_data_clone (rhs: &mut pjsua_msg_data) -> *mut pjsua_msg_data {
    unsafe {

        let pool = pool_create("tmp-pool");

        let ret = pjsua_msg_data_clone(pool, rhs as *const _ );

        pool_release(pool);

        ret
    }
}

pub fn handle_events(msec_timeout: u32) -> i32 {
    unsafe { pjsua_handle_events(msec_timeout) }
}

pub fn stop_worker_threads() {
    unsafe { pjsua_stop_worker_threads() }
}

pub fn reconfigure_logging (c: &mut pjsua_logging_config) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_reconfigure_logging(c as *const _)) }
}

pub fn get_pjsip_endpt() -> *mut pjsip_endpoint {
    unsafe { pjsua_get_pjsip_endpt() }
}

pub fn get_pjmedia_endpt() -> *mut pjmedia_endpt {
    unsafe { pjsua_get_pjmedia_endpt() }
}

pub fn get_pool_factory() -> *mut pj_pool_factory {
    unsafe { pjsua_get_pool_factory() }
}

pub fn ip_change_param_default(param: &mut pjsua_ip_change_param) {
    unsafe { pjsua_ip_change_param_default(param as *mut _) }
}

pub fn detect_nat_type () -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_detect_nat_type()) }
}

pub fn get_nat_type(type_: &mut pj_stun_nat_type) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_get_nat_type(type_ as *mut _)) }
}

pub fn update_stun_servers (count: u32, srv: &mut [pj_str_t; 8], wait: bool) -> Result<(), pj_status_t> {
    unsafe {
        // todo fix this and compare result with c code.
        let status = pjsua_update_stun_servers(
                count,
                srv.as_mut_ptr(),
                utils::boolean_to_pjbool(wait)
            );

        utils::check_status(status)
    }
}

// pj_status_t 	pjsua_resolve_stun_servers (unsigned count, pj_str_t srv[], pj_bool_t wait, void *token, pj_stun_resolve_cb cb)
pub fn resolve_stun_servers<T> (
    count: u32,
    srv: &mut [pj_str_t; 8],
    wait: bool,
    token: Option<&mut T>,
    cb: pj_stun_resolve_cb
) -> Result<(), pj_status_t> {
        // todo check token
    unsafe {

        let token = match token {
            Some(value) => (value as *mut _) as *mut c_void,
            None => ptr::null_mut()
        };

        let status = pjsua_resolve_stun_servers(
            count,
            srv.as_mut_ptr(),
            utils::boolean_to_pjbool(wait),
            token,
            cb
        );

        utils::check_status(status)
    }
}

// pj_status_t 	pjsua_cancel_stun_resolution (void *token, pj_bool_t notify_cb)
pub fn cancel_stun_resolution<T> (token: Option<&mut T>, notify_cb: bool) -> Result<(), pj_status_t> {
    unsafe {

        let token = match token {
            Some(val) => (val as *mut _) as *mut c_void,
            None => ptr::null_mut()
        };

        let status = pjsua_cancel_stun_resolution (
            token,
            utils::boolean_to_pjbool(notify_cb)
        );

        utils::check_status(status)
    }
}

pub fn verify_sip_url(url: String) -> Result<(), pj_status_t> {
    let url: *const i8 = CString::new(url).expect("pjsua_verify_sip_url").into_raw();
    unsafe { utils::check_status(pjsua_verify_sip_url( url )) }
}

pub fn verify_url (url: String) -> Result<(), pj_status_t> {
    let url: *const i8 = CString::new(url).expect("pjsua_verify_url").into_raw();
    unsafe {
        utils::check_status(pjsua_verify_url ( url ))
    }
}

pub fn schedule_timer (entry: &mut pj_timer_entry, delay: &mut pj_time_val) -> Result<(), pj_status_t> {
    unsafe {
        // because we use debug pjsua
        // will provide timer with debug suport
        let status = pjsua_schedule_timer_dbg(
            entry as *mut _,
            delay as *const _,
            ptr::null_mut(),
            0
        );

        utils::check_status(status)
     }
}

// pj_status_t 	pjsua_schedule_timer2 (void(*cb)(void *user_data), void *user_data, unsigned msec_delay)

pub fn cancel_timer(entry: &mut pj_timer_entry) {
    unsafe { pjsua_cancel_timer(entry as *mut _) }
}

pub fn perror(sender: String, title: String, status: pj_status_t) {
    let sender: *const i8 = CString::new(sender.as_str()).expect("pjsua_perror").into_raw();
    let title: *const i8 = CString::new(title.as_str()).expect("pjusa_perror").into_raw();

    unsafe { pjsua_perror( sender, title, status ); }
}

pub fn dump(detail: bool) {
    unsafe { pjsua_dump(utils::boolean_to_pjbool(detail)); }
}

pub fn handle_ip_change(param: &mut pjsua_ip_change_param) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_handle_ip_change( param as *const _ )) }
}


// call helper function

pub fn call_setting_default (opt: &mut pjsua_call_setting) {
    unsafe { pjsua_call_setting_default(opt as * mut _) }
}

pub fn call_send_dtmf_param_default (param: &mut pjsua_call_send_dtmf_param) {
    unsafe { pjsua_call_send_dtmf_param_default(param as *mut _) }
}

pub fn call_get_max_count () -> u32 {
    unsafe { pjsua_call_get_max_count() }
}

pub fn call_get_count () -> u32 {
    unsafe { pjsua_call_get_count() }
}

pub fn enum_calls (ids: &mut [pjsua_call_id; PJSUA_MAX_CALLS as usize], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_enum_calls( ids.as_mut_ptr(), count as *mut _))
    }
}

// pj_status_t 	pjsua_call_make_call (pjsua_acc_id acc_id, const pj_str_t *dst_uri, const pjsua_call_setting *opt, void *user_data, const pjsua_msg_data *msg_data, pjsua_call_id *p_call_id)
pub fn call_make_call (
    acc_id: pjsua_acc_id,
    dst_uri: String,
    opt: Option<&mut pjsua_call_setting>,
    msg_data: Option<&mut pjsua_msg_data>,
    p_call_id: Option<&mut pjsua_call_id>
) -> Result<(), pj_status_t> {

    let mut dst_uri = pj_str_t::from_string(dst_uri);

    let opt = match opt {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    let p_call_id = match p_call_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };


    unsafe {

        let status = pjsua_call_make_call(
            acc_id,
            &mut dst_uri as *const _,
            opt,
            ptr::null_mut(),
            msg_data,
            p_call_id
        );

        utils::check_status(status)
    }
}

pub fn call_is_active (call_id: pjsua_call_id) -> bool {
    unsafe { utils::check_boolean(pjsua_call_is_active(call_id)) }
}

pub fn call_has_media (call_id: pjsua_call_id) -> bool {
    unsafe { utils::check_boolean(pjsua_call_has_media(call_id)) }
}

pub fn call_get_conf_port (call_id: pjsua_call_id) -> pjsua_conf_port_id {
    unsafe { pjsua_call_get_conf_port(call_id) }
}

pub fn call_get_info (call_id: pjsua_call_id, info: &mut pjsua_call_info) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_call_get_info (call_id, info as *mut _)) }
}

pub fn call_remote_has_cap (call_id: pjsua_call_id, htype: i32, hname: String, token: String) -> pjsip_dialog_cap_status {
    let hname: *const pj_str_t = &mut pj_str_t::from_string(hname) as *const _;
    let token: *const pj_str_t = &mut pj_str_t::from_string(token) as *const _;

    unsafe {
        pjsua_call_remote_has_cap(
            call_id,
            htype,
            hname,
            token
        )
    }
}

// unused function
// pj_status_t 	pjsua_call_set_user_data (pjsua_call_id call_id, void *user_data)
// void * 	pjsua_call_get_user_data (pjsua_call_id call_id)

pub fn call_get_rem_nat_type (call_id: pjsua_call_id, p_type: &mut pj_stun_nat_type) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_call_get_rem_nat_type ( call_id, p_type as *mut _))
    }
}

pub fn call_answer (call_id: pjsua_call_id, code: u32, reason: Option<String>, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const pj_str_t,
        None => ptr::null_mut(),
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_answer( call_id, code, reason, msg_data)) }
}

pub fn call_answer2 (
    call_id: pjsua_call_id,
    opt: &mut pjsua_call_setting,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut pjsua_msg_data>
) -> Result<(), pj_status_t> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const _ ,
        None => ptr::null_mut()
    };

    let msg_data: *const pjsua_msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_answer2(call_id, opt, code, reason, msg_data)) }
}

pub fn call_answer_with_sdp(
    call_id: pjsua_call_id,
    sdp: &mut pjmedia_sdp_session,
    opt: &mut pjsua_call_setting,
    code: u32,
    reason: Option<String>,
    msg_data: Option<&mut pjsua_msg_data>
) -> Result<(), pj_status_t> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value),
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_call_answer_with_sdp(
            call_id, sdp as *const _, opt as *const _,
            code, reason, msg_data))
    }
}

pub fn call_hangup(
    call_id: pjsua_call_id,
    code: c_uint,
    reason: Option<String>,
    msg_data: Option<&mut pjsua_msg_data>
) -> Result<(), pj_status_t> {

    let reason = match reason {
        Some(value) => &mut pj_str_t::from_string(value) as *const _,
        None => ptr::null_mut()
    };

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_hangup(call_id, code, reason, msg_data)) }
}

pub fn call_process_redirect (call_id: pjsua_call_id, cmd: pjsip_redirect_op) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_call_process_redirect(call_id, cmd)) }
}

pub fn call_set_hold (call_id: pjsua_call_id, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_set_hold( call_id, msg_data)) }
}

pub fn call_set_hold2 (call_id: pjsua_call_id, options: u32, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_set_hold2(call_id, options, msg_data)) }
}

pub fn call_reinvite(call_id: pjsua_call_id, options: u32, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_reinvite( call_id, options, msg_data)) }
}

pub fn call_reinvite2(call_id: pjsua_call_id, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data> ) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_reinvite2( call_id, opt as *const _, msg_data )) }
}

pub fn call_update (call_id: pjsua_call_id, options: u32, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe { utils::check_status(pjsua_call_update( call_id, options, msg_data)) }
}

pub fn call_update2 (call_id: pjsua_call_id, opt: &mut pjsua_call_setting, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_call_update2( call_id, opt as *const _, msg_data))
    }
}

pub fn call_xfer (call_id: pjsua_call_id, dest: String, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let mut dest = pj_str_t::from_string(dest);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_call_xfer(call_id,&mut dest as *const _,msg_data))
    }
}

pub fn call_xfer_replaces (call_id: pjsua_call_id, dest_call_id: pjsua_call_id, options: u32, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_call_xfer_replaces (call_id, dest_call_id, options, msg_data))
    }
}

pub fn call_dial_dtmf (call_id: pjsua_call_id, digits: String) -> Result<(), pj_status_t> {

    let mut digits = pj_str_t::from_string(digits);

    unsafe {
        utils::check_status(pjsua_call_dial_dtmf(call_id, &mut digits as *const _))
    }

}

pub fn call_send_dtmf (call_id: pjsua_call_id, param: &mut pjsua_call_send_dtmf_param) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_call_send_dtmf (call_id, param as *const _))
    }
}

pub fn call_send_im (call_id: pjsua_call_id, mime_type: String, content: String, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let mut mime_type = pj_str_t::from_string(mime_type);
    let mut content = pj_str_t::from_string(content);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_call_send_im(
            call_id,
            &mut mime_type as *const _,
            &mut content as *const _,
            msg_data,
            ptr::null_mut()
        );
        utils::check_status(status)
    }
}

pub fn call_send_typing_ind (call_id: pjsua_call_id, is_typing: bool, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_call_send_typing_ind(
            call_id,
            utils::boolean_to_pjbool(is_typing),
            msg_data
        );

        utils::check_status(status)
    }
}

pub fn call_send_request (call_id: pjsua_call_id, method: String, msg_data: Option<&mut pjsua_msg_data>) -> Result<(), pj_status_t> {

    let mut method = pj_str_t::from_string(method);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_call_send_request ( call_id, &mut method as *const _, msg_data ))
    }
}

pub fn call_hangup_all () {
    unsafe { pjsua_call_hangup_all() }
}

pub fn call_dump(
    call_id: pjsua_call_id,
    with_media: bool,
    buffer: String,
    maxlen: u32,
    indent: String,
) -> Result<(), pj_status_t> {

    let buffer: *mut i8 = CString::new(buffer.as_str()).expect("CString::pjsua_call_dump fail.").into_raw();
    let indent: *const i8 = CString::new(indent.as_str()).expect("CString::pjsua_call_dump fail.").into_raw();

    unsafe {

        let status = pjsua_call_dump(
            call_id,
            utils::boolean_to_pjbool(with_media),
            buffer,
            maxlen,
            indent as *const _
        );

        utils::check_status(status)
    }
}

pub fn call_get_stream_info (call_id: pjsua_call_id, med_idx: u32, psi: &mut pjsua_stream_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_call_get_stream_info (call_id, med_idx, psi as *mut _))
    }
}

pub fn call_get_stream_stat (call_id: pjsua_call_id, med_idx: u32, stat: &mut pjsua_stream_stat) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_call_get_stream_stat( call_id, med_idx, stat as *mut _))
    }
}

// pj_status_t 	pjsua_call_get_med_transport_info (pjsua_call_id call_id, unsigned med_idx, pjmedia_transport_info *t)
pub fn call_get_med_transport_info (call_id: pjsua_call_id, med_idx: u32, t: &mut pjmedia_transport_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_call_get_med_transport_info( call_id, med_idx, t as *mut _))
    }
}

// void 	pjsua_call_vid_strm_op_param_default (pjsua_call_vid_strm_op_param *param)


// pjsua_vid_win_id 	pjsua_call_get_vid_win (pjsua_call_id call_id)
// pjsua_conf_port_id 	pjsua_call_get_vid_conf_port (pjsua_call_id call_id, pjmedia_dir dir)
// pj_status_t 	pjsua_call_set_vid_strm (pjsua_call_id call_id, pjsua_call_vid_strm_op op, const pjsua_call_vid_strm_op_param *param)
// pj_bool_t 	pjsua_call_vid_stream_is_running (pjsua_call_id call_id, int med_idx, pjmedia_dir dir)
// int 	pjsua_call_get_vid_stream_idx (pjsua_call_id call_id)



// pjsua sound and media device function helper

pub fn media_config_default(cfg: &mut pjsua_media_config) {
    unsafe { pjsua_media_config_default(cfg as *mut _); }
}

pub fn snd_dev_param_default (prm: &mut pjsua_snd_dev_param) {
    unsafe { pjsua_snd_dev_param_default(prm as *mut _); }
}

pub fn conf_connect_param_defautl(prm: &mut pjsua_conf_connect_param) {
    unsafe { pjsua_conf_connect_param_default(prm as *mut _); }
}

pub fn conf_get_max_ports() -> u32 {
    unsafe { pjsua_conf_get_max_ports() }
}

pub fn conf_get_active_ports() -> u32 {
    unsafe { pjsua_conf_get_active_ports() }
}

pub fn enum_conf_ports(id: &mut [pjsua_conf_port_id; PJSUA_MAX_CONF_PORTS as usize], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_enum_conf_ports( id.as_mut_ptr(), count as *mut _))
    }
}

pub fn conf_get_port_info (port_id: pjsua_conf_port_id, info: &mut pjsua_conf_port_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_conf_get_port_info( port_id, info as *mut _ ))
    }
}

pub fn conf_add_port(port: *mut pjmedia_port, p_id: Option<&mut pjsua_conf_port_id>) -> Result<(), pj_status_t> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };


    unsafe {
        let pool = pool_create("tmp-pool");

        // let aport = port.as_mut().as_ptr();
        let result = pjsua_conf_add_port(
            pool,
            port,
            p_id
        );

        pool_release(pool);

        if result == PJ_SUCCESS as pj_status_t {
            Ok(())
        } else {
            Err(result)
        }
    }
}

pub fn conf_remove_port (port_id: pjsua_conf_port_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_conf_remove_port(port_id)) }
}

pub fn conf_connect(source: pjsua_conf_port_id, sink: pjsua_conf_port_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_conf_connect(source, sink)) }
}

pub fn conf_connect2 (source: pjsua_conf_port_id, sink: pjsua_conf_port_id, prm: &mut pjsua_conf_connect_param) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_conf_connect2 ( source, sink, prm as *const _ ))
    }
}

pub fn conf_disconnect(source: pjsua_conf_port_id, sink: pjsua_conf_port_id) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_conf_disconnect(source, sink))
    }
}

pub fn conf_adjust_tx_level (slot: pjsua_conf_port_id, level: f32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_conf_adjust_tx_level(slot, level))
    }
}

pub fn conf_adjust_rx_level (slot: pjsua_conf_port_id, level: f32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_conf_adjust_rx_level(slot, level)) }
}

pub fn conf_get_signal_level (slot: pjsua_conf_port_id, tx_level: &mut u32, rx_level: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_conf_get_signal_level (slot, tx_level as *mut _, rx_level as *mut _))
    }
}

pub fn player_create(filename: String, options: u32, p_id: &mut pjsua_player_id) -> Result<(), pj_status_t> {

    let filename: *const pj_str_t = &mut pj_str_t::from_string(filename) as *const _;

    unsafe {
        utils::check_status(pjsua_player_create( filename, options, p_id as *mut _))
    }
}

// pj_status_t 	pjsua_playlist_create (const pj_str_t file_names[], unsigned file_count, const pj_str_t *label, unsigned options, pjsua_player_id *p_id)

pub fn player_get_conf_port(id: pjsua_player_id) -> pjsua_conf_port_id {
    unsafe { pjsua_player_get_conf_port(id) }
}

pub fn player_get_port(id: pjsua_player_id, p_port: &mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_player_get_port(id, &mut (p_port as *mut _) as *mut _))
    }
}

pub fn player_get_info(id: pjsua_player_id, info: &mut pjmedia_wav_player_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_player_get_info( id, info as *mut _))
    }
}

pub fn player_get_pos(id: pjsua_player_id) -> pj_ssize_t {
    unsafe { pjsua_player_get_pos(id) }
}

pub fn player_set_pos(id: pjsua_player_id, samples: pj_uint32_t) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_player_set_pos(id, samples)) }
}

pub fn player_destroy (id: pjsua_player_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_player_destroy(id)) }
}

// skiped function

// pj_status_t 	pjsua_recorder_create (const pj_str_t *filename, unsigned enc_type, void *enc_param, pj_ssize_t max_size, unsigned options, pjsua_recorder_id *p_id)
// pjsua_conf_port_id 	pjsua_recorder_get_conf_port (pjsua_recorder_id id)
// pj_status_t 	pjsua_recorder_get_port (pjsua_recorder_id id, pjmedia_port **p_port)
// pj_status_t 	pjsua_recorder_destroy (pjsua_recorder_id id)

pub fn enum_aud_devs(info: &mut [pjmedia_aud_dev_info; 256], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_enum_aud_devs( info.as_mut_ptr(), count as *mut _)) }
}

pub fn enum_snd_devs(info: &mut [pjmedia_snd_dev_info; 256], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_enum_snd_devs( info.as_mut_ptr(), count as *mut _))
    }
}

pub fn get_snd_dev(capture_dev: &mut i32, playback_dev: &mut i32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_get_snd_dev( capture_dev as *mut _, playback_dev as *mut _ )) }
}

pub fn set_snd_dev(capture_dev: i32, playback_dev: i32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_set_snd_dev(capture_dev, playback_dev)) }
}

pub fn set_snd_dev2(snd_param: &mut pjsua_snd_dev_param) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_set_snd_dev2( snd_param as *mut _)) }
}

pub fn set_null_snd_dev() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_set_null_snd_dev()) }
}

pub fn set_no_snd_dev() -> *mut pjmedia_port {
    unsafe { pjsua_set_no_snd_dev() }
}

pub fn set_ec(tail_ms: u32, options: u32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_set_ec(tail_ms, options)) }
}

pub fn get_ec_tail(p_tail_ms: &mut u32) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_get_ec_tail(p_tail_ms)) }
}

pub fn get_ec_stat(p_stat: &mut pjmedia_echo_stat) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_get_ec_stat( p_stat as *mut _ )) }
}

pub fn snd_is_active() -> bool {
    unsafe { utils::check_boolean(pjsua_snd_is_active()) }
}


// skiped function for detailed audio dev setting
// pj_status_t 	pjsua_snd_set_setting (pjmedia_aud_dev_cap cap, const void *pval, pj_bool_t keep)
// pj_status_t 	pjsua_snd_get_setting (pjmedia_aud_dev_cap cap, void *pval)


// TODO check this create and destroy
pub fn ext_snd_dev_create(param: &mut pjmedia_snd_port_param, p_snd: &mut pjsua_ext_snd_dev) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_ext_snd_dev_create( param as *mut _, &mut (p_snd as *mut _) as *mut _ ))
    }
}

pub fn ext_snd_dev_destroy(snd: &mut pjsua_ext_snd_dev) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_ext_snd_dev_destroy(snd as *mut _)) }
}

pub fn ext_snd_dev_get_snd_port(snd: &mut pjsua_ext_snd_dev) -> *mut pjmedia_snd_port {
    unsafe { pjsua_ext_snd_dev_get_snd_port( snd as *mut _) }
}

pub fn ext_snd_dev_get_conf_port(snd: &mut pjsua_ext_snd_dev) -> pjsua_conf_port_id {
    unsafe { pjsua_ext_snd_dev_get_conf_port( snd as *mut _ ) }
}

pub fn enum_codecs(id: &mut [pjsua_codec_info; 32], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_enum_codecs( id.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn codec_set_priority(codec_id: String, priority: u8) -> Result<(), pj_status_t> {
    let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;
    unsafe { utils::check_status(pjsua_codec_set_priority( codec_id, priority)) }
}

pub fn codec_get_param(codec_id: String, param: &mut pjmedia_codec_param) -> Result<(), pj_status_t> {

    let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;

    unsafe {
        utils::check_status(pjsua_codec_get_param( codec_id, param as *mut _))
    }
}

pub fn codec_set_param(codec_id: String, param: &mut pjmedia_codec_param) -> Result<(), pj_status_t> {
    let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;
    unsafe {
        utils::check_status(pjsua_codec_set_param( codec_id, param as *const _ ))
    }
}


// pjsua account helper

pub fn ice_config_from_media_config(dst: &mut pjsua_ice_config, src: &mut pjsua_media_config) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_ice_config_from_media_config(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn ice_config_dup(dst: &mut pjsua_ice_config, src: &mut pjsua_ice_config) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_ice_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn turn_config_from_media_config(dst: &mut pjsua_turn_config, src: &mut pjsua_media_config) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_turn_config_from_media_config(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn turn_config_dup(dst: &mut pjsua_turn_config, src: &mut pjsua_turn_config) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_turn_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn srtp_opt_default(cfg: &mut pjsua_srtp_opt) {
    unsafe {
        pjsua_srtp_opt_default(
            cfg as *mut _
        )
    }
}

pub fn srtp_opt_dup(dst: &mut pjsua_srtp_opt, src: &mut pjsua_srtp_opt, check_str: bool) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_srtp_opt_dup(
            pool,
            dst as *mut _,
            src as *const _,
            utils::boolean_to_pjbool(check_str)
        )
    }

    pool_release(pool);
}

pub fn acc_config_default (cfg: &mut pjsua_acc_config) {
    unsafe { pjsua_acc_config_default(cfg as *mut _); }
}

pub fn acc_config_dup (dst: &mut pjsua_acc_config, src: &mut pjsua_acc_config) {
    unsafe {
        let pool = pool_create("tmp-pool");

        pjsua_acc_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool);
    }
}

pub fn acc_get_count() -> u32 {
    unsafe { pjsua_acc_get_count() }
}

pub fn acc_is_valid(acc_id: pjsua_acc_id) -> bool {
    unsafe { utils::check_boolean(pjsua_acc_is_valid(acc_id)) }
}

pub fn acc_set_default(acc_id: pjsua_acc_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_acc_set_default(acc_id)) }
}

pub fn acc_get_default() -> pjsua_acc_id {
    unsafe { pjsua_acc_get_default() }
}

pub fn acc_add(acc_cfg: &mut pjsua_acc_config, is_default: bool, p_acc_id: &mut pjsua_acc_id) -> Result<(), pj_status_t> {
    unsafe {

        let status = pjsua_acc_add(
            acc_cfg as *const _,
            utils::boolean_to_pjbool(is_default),
            p_acc_id as *mut _
        );

        utils::check_status(status)
    }
}

pub fn acc_add_local(tid: pjsua_transport_id, is_default: bool, p_acc_id: &mut pjsua_acc_id) -> Result<(), pj_status_t> {
    unsafe {

        let status = pjsua_acc_add_local(
            tid,
            utils::boolean_to_pjbool(is_default),
            p_acc_id as *mut _
        );

        utils::check_status(status)
    }
}

// pj_status_t 	pjsua_acc_set_user_data (pjsua_acc_id acc_id, void *user_data)
// void * 	pjsua_acc_get_user_data (pjsua_acc_id acc_id)

pub fn acc_del(acc_id: pjsua_acc_id) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_acc_del(acc_id))
    }
}

pub fn acc_get_config (acc_id: pjsua_acc_id, acc_cfg: &mut pjsua_acc_config) -> Result<(), pj_status_t> {
    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_acc_get_config(acc_id, pool, acc_cfg as *mut _);

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_modify(acc_id: pjsua_acc_id, acc_cfg: &mut pjsua_acc_config) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_acc_modify( acc_id, acc_cfg as *const _ ))
    }
}

pub fn acc_set_online_status(acc_id: pjsua_acc_id, is_online: bool) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_acc_set_online_status( acc_id, utils::boolean_to_pjbool(is_online)))
    }
}

pub fn acc_set_online_status2(acc_id: pjsua_acc_id, is_online: bool, pr: &mut  pjrpid_element) -> Result<(), pj_status_t> {

    unsafe {
        let status = pjsua_acc_set_online_status2(
            acc_id,
            utils::boolean_to_pjbool(is_online),
            pr as *const _
        );
        utils::check_status(status)
    }
}

pub fn acc_set_registration(acc_id: pjsua_acc_id, renew: bool) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_acc_set_registration( acc_id, utils::boolean_to_pjbool(renew)))
    }
}

pub fn acc_get_info (acc_id: pjsua_acc_id, info: &mut pjsua_acc_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_acc_get_info(acc_id, info as *mut _))
    }
}

pub fn enum_accs(ids: &mut [pjsua_acc_id; PJSUA_MAX_ACC as usize], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_enum_accs( ids.as_mut_ptr(), count as *mut _))
    }
}

pub fn acc_enum_info(info: &mut [pjsua_acc_info; PJSUA_MAX_ACC as usize], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_acc_enum_info( info.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn acc_find_for_outgoing(url: String) -> pjsua_acc_id {

    let mut url = pj_str_t::from_string(url);

    unsafe {
        pjsua_acc_find_for_outgoing(
            &mut url as *const _
        )
    }

}

pub fn acc_find_for_incoming(rdata: &mut pjsip_rx_data) -> pjsua_acc_id {

    unsafe {
        pjsua_acc_find_for_incoming(
            rdata as *mut _
        )
    }
}

pub fn acc_create_request(acc_id: pjsua_acc_id, method: &mut pjsip_method, target: String, p_tdata: &mut pjsip_tx_data) -> Result<(), pj_status_t> {

    let mut target = pj_str_t::from_string(target);

    unsafe {
        let status = pjsua_acc_create_request(
            acc_id,
            method as *const _,
            &mut target as *const _,
            (p_tdata as *mut _) as *mut _
        );

        utils::check_status(status)
    }
}

pub fn acc_create_uac_contact(contact: String, acc_id: pjsua_acc_id, uri: String) -> Result<(), pj_status_t> {

    let mut contact = pj_str_t::from_string(contact);
    let mut uri = pj_str_t::from_string(uri);

    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_acc_create_uac_contact(
            pool,
            &mut contact as *mut _,
            acc_id,
            &mut uri as *mut _
        );

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_create_uas_contact(contact: String, acc_id: pjsua_acc_id, rdata: &mut pjsip_rx_data) -> Result<(), pj_status_t> {

    let mut contact = pj_str_t::from_string(contact);

    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_acc_create_uas_contact(
            pool,
            &mut contact as *mut _,
            acc_id,
            rdata as *mut _
        );

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_set_transport(acc_id: pjsua_acc_id, tp_id: pjsua_transport_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_acc_set_transport( acc_id, tp_id )) }
}


// JSUA-API Buddy, Presence, and Instant Messaging

pub fn buddy_config_default(cfg: &mut pjsua_buddy_config) {
    unsafe {
        pjsua_buddy_config_default(
            cfg as *mut _
        )
    }
}

pub fn get_buddy_count() -> u32 {
    unsafe { pjsua_get_buddy_count() }
}

pub fn buddy_is_valid(buddy_id: pjsua_buddy_id) -> bool {
    unsafe {utils::check_boolean(pjsua_buddy_is_valid(buddy_id)) }
}

pub fn enum_buddies(ids: &mut [pjsua_buddy_id; PJSUA_MAX_BUDDIES as usize], count: &mut u32) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_enum_buddies( ids.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn buddy_find(uri: String) -> pjsua_buddy_id {
    let uri: *const pj_str_t = &mut pj_str_t::from_string(uri) as *const _;
    unsafe { pjsua_buddy_find( uri ) }
}

pub fn buddy_get_info(buddy_id: pjsua_buddy_id, info: &mut pjsua_buddy_info) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_buddy_get_info( buddy_id, info as *mut _ )) }
}

// skiped function this mosly for trasfer data
// pj_status_t 	pjsua_buddy_set_user_data (pjsua_buddy_id buddy_id, void *user_data)
// void * 	pjsua_buddy_get_user_data (pjsua_buddy_id buddy_id)

pub fn buddy_add(buddy_cfg: &mut pjsua_buddy_config, p_buddy_id: *mut pjsua_buddy_id) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjsua_buddy_add (
            buddy_cfg as *const _,
            p_buddy_id as *mut _
        );
        utils::check_status(status)
    }
}

pub fn buddy_del(buddy_id: pjsua_buddy_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_buddy_del(buddy_id)) }
}

pub fn buddy_subscribe_pres(buddy_id: pjsua_buddy_id, subscribe: bool) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_buddy_subscribe_pres(buddy_id, utils::boolean_to_pjbool(subscribe)))
    }
}

pub fn buddy_update_pres(buddy_id: pjsua_buddy_id) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_buddy_update_pres(buddy_id)) }
}

pub fn pres_notify(
    acc_id: pjsua_acc_id,
    srv_pres: &mut pjsua_srv_pres,
    state: pjsip_evsub_state,
    state_str: String,
    reason: String,
    with_body: bool,
    msg_data: Option<&mut pjsua_msg_data>
) -> Result<(), pj_status_t> {

    let mut state_str = pj_str_t::from_string(state_str);
    let mut reason = pj_str_t::from_string(reason);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_pres_notify(
            acc_id,
            srv_pres,
            state,
            &mut state_str as *const _,
            &mut reason as *const _,
            utils::boolean_to_pjbool(with_body),
            msg_data
        );

        utils::check_status(status)
    }
}

pub fn pres_dump(verbose: bool) {
    unsafe { pjsua_pres_dump ( utils::boolean_to_pjbool(verbose))}
}

pub fn im_send(
    acc_id: pjsua_acc_id,
    to: String,
    mime_type: String,
    content: String,
    msg_data: Option<&mut pjsua_msg_data>
) -> Result<(), pj_status_t> {

    let mut to = pj_str_t::from_string(to);
    let mut mime_type = pj_str_t::from_string(mime_type);
    let mut content = pj_str_t::from_string(content);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_im_send(
            acc_id,
            &mut to as *const _,
            &mut mime_type as *const _,
            &mut content as *const _,
            msg_data,
            ptr::null_mut()
        );
        utils::check_status(status)
    }

}

pub fn im_typing(
    acc_id: pjsua_acc_id,
    to:String,
    is_typing: bool,
    msg_data: Option<&mut pjsua_msg_data>
) -> Result<(), pj_status_t> {

    let mut to = pj_str_t::from_string(to);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_im_typing(
            acc_id,
            &mut to as *const _,
            utils::boolean_to_pjbool(is_typing),
            msg_data
        );

        utils::check_status(status)
    }

}

// PJSUA-API Signaling Transport

pub fn transport_config_default(cfg: &mut pjsua_transport_config) {
    unsafe { pjsua_transport_config_default(cfg as *mut _) }
}

pub fn transport_config_dup(dst: &mut pjsua_transport_config, src: &mut pjsua_transport_config) {
    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_transport_config_dup(
            pool,
            dst as *mut _,
            src as *mut _
        );
    }

    pool_release(pool)
}

pub fn transport_create(type_: pjsip_transport_type_e, cfg: &mut pjsua_transport_config, p_id: Option<&mut pjsua_transport_id>) -> Result<(), pj_status_t> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_transport_create(
            type_,
            cfg as *const _,
            p_id
        );
        utils::check_status(status)
    }
}

pub fn transport_register(tp: &mut pjsip_transport, p_id: Option<&mut pjsua_transport_id>) -> Result<(), pj_status_t> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_transport_register( tp as *mut _, p_id))
    }
}

pub fn tpfactory_register(tf: &mut pjsip_tpfactory, p_id: Option<&mut pjsua_transport_id>) -> Result<(), pj_status_t> {

    let p_id = match p_id {
        Some(value) => value as *mut _,
        None => ptr::null_mut()
    };

    unsafe {
        utils::check_status(pjsua_tpfactory_register( tf as *mut _, p_id ))
    }
}

pub fn enum_transports(id: &mut [pjsua_transport_id; PJSIP_MAX_TRANSPORTS as usize], count: &mut u32) -> Result<(), pj_status_t> {

    unsafe {
        utils::check_status(pjsua_enum_transports( id.as_mut_ptr(), count as *mut _))
    }
}

pub fn transport_get_info(id: pjsua_transport_id, info: &mut pjsua_transport_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_transport_get_info ( id, info as *mut _))
    }
}

pub fn transport_set_enable(id: pjsua_transport_id, enabled: bool) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_transport_set_enable( id, utils::boolean_to_pjbool(enabled) ))
    }
}

pub fn transport_close (id: pjsua_transport_id, force: bool) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjsua_transport_close ( id, utils::boolean_to_pjbool(force)))
    }
}

pub fn transport_lis_start(id: pjsua_transport_id, cfg: &mut pjsua_transport_config) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjsua_transport_lis_start( id, cfg as *const _)) }
}
