#![allow(dead_code)]

// default
use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;
use std::ops::Drop;
use std::ffi::CString;

pub type SIPAccount = pjsua_acc_config;
pub type SIPBuddy = pjsua_buddy_config;

pub struct SIPAudio {}
pub struct SIPIMessages {}
pub struct SIPMedia {}
pub struct SIPPressence {}
pub struct SIPCall {}



// pjsua_acc_get_count
// pjsua_acc_is_valid
// pjsua_acc_set_default
// pjsua_acc_get_default
// pjsua_acc_config_dup
// pjsua_acc_add
// pjsua_acc_add_local
// pjsua_acc_set_user_data
// pjsua_acc_get_user_data
// pjsua_acc_del
// pjsua_acc_get_config
// pjsua_acc_modify
// pjsua_acc_set_online_status
// pjsua_acc_set_online_status2
// pjsua_acc_set_registration
// pjsua_acc_get_info
// pjsua_enum_accs
// pjsua_acc_enum_info
// pjsua_acc_find_for_outgoing
// pjsua_acc_find_for_incoming
// pjsua_acc_create_request
// pjsua_acc_create_uac_contact
// pjsua_acc_create_uas_contact
// pjsua_acc_set_transport
//
trait Account {
    fn get_count() -> u32;
}

// pjsua_call_has_media
// pjsua_call_get_conf_port
// pjsua_call_get_stream_info
// pjsua_call_get_stream_stat
// pjsua_call_dial_dtmf
//
// pjsua_snd_dev_param_default
// pjsua_conf_connect_param_default
// pjsua_conf_get_max_port
// pjsua_conf_get_active_ports
// pjsua_enum_conf_ports
// pjsua_conf_get_port_info
// pjsua_conf_add_port
// pjsua_conf_remove_port
// pjsua_conf_connect
// pjsua_conf_connectpjsua_acc_info
// pjsua_conf_disconnect
// pjsua_conf_adjust_tx_level
// pjsua_conf_adjust_rx_level
// pjsua_conf_get_signal_level
// pjsua_player_create
// pjsua_playlist_create
// pjsua_player_get_conf_port
// pjsua_player_get_info
// pjsua_player_get_pos
// pjsua_player_set_pos
// pjsua_player_destroy
// pjsua_recorder_create
// pjsua_recorder_get_conf_port
// pjsua_recorder_get_port
// pjsua_recorder_destroy
//
// sound devices
// pjsua_enum_aud_devs
// pjsua_enum_snd_devs
// pjsua_set_snd_dev
// pjsua_set_snd_dev2
// pjsua_get_snd_dev
// pjsua_set_null_snd_dev
// pjsua_set_no_snd_dev
// pjsua_set_ec
// pjsua_set_ec_tail
// pjsua_get_ec_stat
// pjsua_snd_is_active
// pjsua_snd_get_setting
// pjsua_ext_snd_dev_create
// pjsua_ext_snd_dev_destroy
// pjsua_ext_snd_dev_get_snd_port
// pjsua_ext_snd_dev_get_conf_port
//
trait Audio {}

// pjsua_im_send
// pjsua_im_typing
trait IMessages {}

// pjsua_enum_codecs
// pjsua_codec_set_priority
// pjsua_codec_get_param
// pjsua_codec_set_param
trait Media {}


// pjsua_get_buddy_count
// pjsua_buddy_find
// pjsua_buddy_is_valid
// pjsua_enum_buddies
// pjsua_buddy_get_info
// pjsua_buddy_set_user_data
// pjsua_buddy_get_user_data
// pjsua_buddy_add
// pjsua_buddy_del
// pjsua_buddy_subscribe_pres
// pjsua_pres_dump
// pjsua_pres_notify
trait Pressence {}


// pjsua_call_get_max_count
// pjsua_call_get_count
// pjsua_enum_calls
// pjsua_call_setting_default
// pjsua_call_send_dtmf_param_default
// pjsua_call_make_call
// pjsua_call_is_active
// pjsua_call_get_info
// pjsua_call_remote_has_cap
// pjsua_call_set_user_data
// pjsua_call_get_user_data
// pjsua_call_get_rem_nat_type
// pjsua_call_get_med_transport_info
// pjsua_call_answer
// pjsua_call_answer2
// pjsua_call_answer_with_sdp
// pjsua_call_hangup
// pjsua_call_process_redirect
// pjsua_call_set_hold
// pjsua_call_set_hold2
// pjsua_call_reinvite
// pjsua_call_reinvite2
// pjsua_call_update
// pjsua_call_update2
// pjsua_call_xfer
// pjsua_call_xfer_replaces
// pjsua_call_send_dtmf
// pjsua_call_send_im
// pjsua_call_send_typing_ind
// pjsua_call_send_request
// pjsua_call_hangup_all
trait Call {}

// pjsua_call_dump
trait Dump {}


// pjsua_get_var
// pjsua_perror
// pjsua_logging_config_default
// pjsua_logging_config_dup
// pjsua_config_default
// pjsua_config_dup
// pjsua_msg_data_init
// pjsua_msg_data_clone
// pjsua_transport_config_default
// pjsua_transport_config_dup
// pjsua_ice_config_from_media_config
// pjsua_ice_config_dup
// pjsua_turn_config_from_media_config
// pjsua_turn_config_dup
// pjsua_srtp_opt_default
// pjsua_srtp_opt_dup
// pjsua_acc_config_default
// pjsua_buddy_config_default
// pjsua_media_config_default
// pjsua_reconfigure_logging
// pjsua_stop_worker_threads
// pjsua_create
// pjsua_init
// pjsua_update_stun_servers
// pjsua_resolve_stun_servers
// pjsua_cancel_stun_resolution
// pjsua_destroy2
// pjsua_destroy
// pjsua_start
// pjsua_handle_events
// pjsua_pool_create
// pjsua_get_pjsip_endpt
// pjsua_get_pjmedia_endpt
// pjsua_get_pool_factory
// pjsua_transport_create
// pjsua_transport_register
// pjsua_tpfactory_register
// pjsua_enum_transports
// pjsua_transport_get_info
// pjsua_transport_set_enable
// pjsua_transport_close
// pjsua_transport_lis_start
// pjsua_ip_change_param_default
// pjsua_detect_nat_type
// pjsua_get_nat_type
// pjsua_verify_url
// pjsua_verify_sip_url
// pjsua_schedule_timer_dbg
// pjsua_schedule_timer
// pjsua_schedule_timer2_dbg
// pjsua_schedule_timer2
// pjsua_cancel_timer
// pjsua_dump
// pjsua_handle_ip_change
pub struct SIPUserAgent {
    /// hold internal pjsua data
    pool: *mut pj_pool_t,
    app_config: pjsua_config,
    log_config: pjsua_logging_config,
    media_config: pjsua_media_config,
    udp_config: pjsua_transport_config,
    rtp_config: pjsua_transport_config,
    account: SIPAccount, // for now just set to 1 account
    buddy_list: Vec<SIPBuddy>,
    redir_op: pjsip_redirect_op,
    wav_id: pjsua_player_id,
    rec_id: pjsua_recorder_id,
    wav_port: pjsua_conf_port_id,
    rec_port: pjsua_conf_port_id,
    input_level: f32,
    output_level: f32,
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    ringback_slot: i32,
    ring_slot: i32,
}

//type SIPConfig = pjsua_config;

pub const PJSUA_INVALID_ID: i32 = -1;



impl SIPUserAgent {

    /// create sip user sip user agent with default value
    ///
    pub fn new() -> SIPUserAgent {
        // create default data
        let ctx: *mut pj_pool_t;
        unsafe {
            pjsua_create();
            let pool_name = CString::new("ipcodec").expect("pool_name fail.");
            ctx = pjsua_pool_create(pool_name.as_ptr(), 1000, 1000);
        }

        let mut udp = pjsua_transport_config::new();
        let mut rtp = pjsua_transport_config::new();
        udp.port = 5060;
        rtp.port = 4000;


        SIPUserAgent{
            pool: ctx,
            app_config: pjsua_config::new(),
            log_config: pjsua_logging_config::new(),
            media_config: pjsua_media_config::new(),
            udp_config: udp,
            rtp_config: rtp,
            account: SIPAccount::new(),
            buddy_list: Vec::<SIPBuddy>::new(),
            redir_op: pjsip_redirect_op_PJSIP_REDIRECT_ACCEPT_REPLACE,
            wav_id: PJSUA_INVALID_ID,
            rec_id: PJSUA_INVALID_ID,
            wav_port: PJSUA_INVALID_ID,
            rec_port: PJSUA_INVALID_ID,
            input_level: 1.0,
            output_level: 1.0,
            input_dev: PJSUA_INVALID_ID,
            output_dev: PJSUA_INVALID_ID,
            input_latency: 100,
            output_latency: 140,
            ringback_slot: PJSUA_INVALID_ID,
            ring_slot: PJSUA_INVALID_ID,
        }
    }

    /// start application
    pub fn start(&mut self) {
        unsafe {
            pjsua_init(&mut self.app_config as *mut _, &mut self.log_config as *mut _,
                       &mut self.media_config as *mut _);
            // pjsip_endpt_register_module(pjsua_get_pjsip_endpt())

        }
    }
}

//binding clike code patern with destructor
impl Drop for SIPUserAgent {
    fn drop(&mut self) {
        unsafe {
            pj_pool_safe_release(&mut self.pool as *mut _);
            pjsua_destroy();
        }
    }
}
