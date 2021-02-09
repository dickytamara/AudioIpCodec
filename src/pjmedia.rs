

use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;

impl AutoCreate<pjmedia_srtp_crypto>  for pjmedia_srtp_crypto {
    fn new() -> pjmedia_srtp_crypto {
        pjmedia_srtp_crypto {
            key: pj_str_t::new(),
            name: pj_str_t::new(),
            flags: 0
        }
    }
}


// impl AutoCreate<pjmedia_config> for pjmedia_config {
//     fn new() -> pjmedia_config {

//     pjsua_media_config {
//         clock_rate: c_uint,
//         snd_clock_rate: c_uint,
//         channel_count: c_uint,
//         audio_frame_ptime: c_uint,
//         max_media_ports: c_uint,
//         has_ioqueue: pj_bool_t,
//         thread_cnt: c_uint,
//         quality: c_uint,
//         ptime: c_uint,
//         no_vad: pj_bool_t,
//         ilbc_mode: c_uint,
//         tx_drop_pct: c_uint,
//         rx_drop_pct: c_uint,
//         ec_options: c_uint,
//         ec_tail_len: c_uint,
//         snd_rec_latency: c_uint,
//         snd_play_latency: c_uint,
//         jb_init: c_int,
//         jb_min_pre: c_int,
//         jb_max_pre: c_int,
//         jb_max: c_int,
//         jb_discard_algo: pjmedia_jb_discard_algo,
//         enable_ice: pj_bool_t,
//         ice_max_host_cands: c_int,
//         ice_opt: pj_ice_sess_options,
//         ice_no_rtcp: pj_bool_t,
//         ice_always_update: pj_bool_t,
//         enable_turn: pj_bool_t,
//         turn_server: pj_str_t,
//         turn_conn_type: pj_turn_tp_type,
//         turn_auth_cred: pj_stun_auth_cred,
//         turn_tls_setting: pj_turn_sock_tls_cfg,
//         snd_auto_close_time: c_int,
//         vid_preview_enable_native: pj_bool_t,
//         no_smart_media_update: pj_bool_t,
//         no_rtcp_sdes_bye: pj_bool_t,
//         on_aud_prev_play_frame: Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
//         on_aud_prev_rec_frame: Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>
//     }


//     }
// }
