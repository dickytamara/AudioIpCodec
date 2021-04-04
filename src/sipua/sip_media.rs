

use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjsua::*;
use super::pjdefault::*;

use super::pjmedia;
use super::pjsua;

use std::{cell::{RefCell, RefMut}, os::raw::c_uint};
use std::ffi::CStr;


// Media and sound device implementation Implementation
pub struct SIPMedia {
    ctx: RefCell<pjsua_media_config>,
    capture_dev: i32,
    playback_dev: i32,
    input_level: i32,
    output_level: i32
}

pub trait SIPMediaExt {
    //     pub clock_rate: c_uint,
    fn set_clock_rate(&self, value: u32);
    fn get_clock_rate(&self) -> u32;
    //     pub snd_clock_rate: c_uint,
    fn set_snd_clock_rate(&self, value: u32);
    fn get_snd_clock_rate(&self) -> u32;
    //     pub channel_count: c_uint,
    fn set_channel_count(&self, value: u32);
    fn get_channel_count(&self) -> u32;
    //     pub audio_frame_ptime: c_uint,
    fn set_audio_frame_ptime(&self, value: u32);
    fn get_audio_frame_ptime(&self) -> u32;
    //     pub max_media_ports: c_uint,
    fn set_max_media_ports(&self, value: u32);
    fn get_max_media_ports(&self) -> u32;
    //     pub has_ioqueue: pj_bool_t,
    fn set_has_ioqueue(&self, value: bool);
    fn get_has_ioqueue(&self) -> bool;
    //     pub thread_cnt: c_uint,
    fn set_thread_cnt(&self, value: u32);
    fn get_thread_cnt(&self);
    //     pub quality: c_uint,
    fn set_quality(&self, value: u32);
    fn get_quality(&self) -> u32;
    //     pub ptime: c_uint,
    fn set_ptime(&self, value: u32);
    fn get_ptime(&self) -> u32;
    //     pub no_vad: pj_bool_t,
    fn set_no_vad(&self, value: bool);
    fn get_no_vad(&self) -> bool;
    //     pub ilbc_mode: c_uint,
    fn set_ilbc_mode(&self, value: bool);
    fn get_ilbc_mode(&self) -> bool;
    //     pub tx_drop_pct: c_uint,
    fn set_tx_drop_pct(&self, value: u32);
    fn get_tx_drop_pct(&self) -> u32;
    //     pub rx_drop_pct: c_uint,
    fn set_rx_drop_pct(&self, value: u32);
    fn get_rx_drop_pct(&self) -> u32;
    //     pub ec_options: c_uint,
    fn set_ec_options(&self, value: u32);
    fn get_ec_options(&self) -> u32;
    //     pub ec_tail_len: c_uint,
    fn set_ec_tail_len(&self, value: u32);
    fn get_ec_tail_len(&self) -> u32;
    //     pub snd_rec_latency: c_uint,
    fn set_snd_rec_latency(&self, value: u32);
    fn get_snd_rec_latency(&self) -> u32;
    //     pub snd_play_latency: c_uint,
    fn set_snd_play_latency(&self, value: u32);
    fn get_snd_paly_latency(&self) -> u32;
    //     pub jb_init: c_int,
    fn set_jb_init(&self, value: i32);
    fn get_jb_init(&self) -> i32;
    //     pub jb_min_pre: c_int,
    fn set_jb_min_pre(&self, value: i32);
    fn get_jb_min_pre(&self) -> i32;
    //     pub jb_max_pre: c_int,
    fn set_jb_max_pre(&self, value: i32);
    fn get_jb_max_pre(&self) -> i32;
    //     pub jb_max: c_int,
    fn set_jb_max(&self, value: i32);
    fn get_jb_max(&self) -> i32;
    //     pub jb_discard_algo: pjmedia_jb_discard_algo,
    fn set_jb_discard_algo(&self, value: pjmedia_jb_discard_algo);
    fn get_jb_discard_algo(&self) -> pjmedia_jb_discard_algo;
    //     pub enable_ice: pj_bool_t,
    fn set_enable_ice(&self, value: bool);
    fn get_enable_ice(&self) -> bool;
    //     pub ice_max_host_cands: c_int,
    fn set_ice_max_host_cands(&self, value: bool);
    fn get_ice_max_host_cands(&self) -> bool;
    //     pub ice_opt: pj_ice_sess_options,
    fn set_ice_opt(&self, value: pj_ice_sess_options);
    fn get_ice_opt(&self) -> pj_ice_sess_options;
    //     pub ice_no_rtcp: pj_bool_t,
    fn set_ice_no_rtcp(&self, value: bool);
    fn get_ice_no_rtcp(&self) -> bool;
    //     pub ice_always_update: pj_bool_t,
    fn set_ice_always_update(&self, value: bool);
    fn get_ice_always_update(&self) -> bool;
    //     pub enable_turn: pj_bool_t,
    fn set_enable_turn(&self, value: bool);
    fn get_enable_turn(&self) -> bool;
    //     pub turn_server: pj_str_t,
    fn set_turn_server(&self, value: String);
    fn get_turn_server(&self) -> String;
    //     pub turn_conn_type: pj_turn_tp_type,
    fn set_turn_conn_type(&self, value: String);
    fn get_turn_conn_type(&self) -> String;
    //     pub turn_auth_cred: pj_stun_auth_cred,
    fn set_turn_auth_cred(&self, value: pj_stun_auth_cred);
    fn get_turn_auth_cred(&self) -> pj_stun_auth_cred;
    //     pub turn_tls_setting: pj_turn_sock_tls_cfg,
    fn set_turn_tls_setting(&self, value: pj_turn_sock_tls_cfg);
    fn get_turn_tls_setting(&self) -> pj_turn_sock_tls_cfg;
    //     pub snd_auto_close_time: c_int,
    fn set_snd_auto_close_time(&self, value: i32);
    fn get_snd_auto_close_time(&self) -> i32;

    // skiped
    //     pub vid_preview_enable_native: pj_bool_t,

    //     pub no_smart_media_update: pj_bool_t,
    fn set_no_smart_media_update(&self, value: bool);
    fn get_no_smart_media_update(&self) -> bool;
    //     pub no_rtcp_sdes_bye: pj_bool_t,
    fn set_no_rtcp_sdes_bye(&self, value: bool);
    fn get_no_rtcp_sdes_bye(&self) -> bool;

    // TODO implement callback
    //     pub on_aud_prev_play_frame: Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
    //     pub on_aud_prev_rec_frame: Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
}

impl SIPMedia {

    // Create new SIP Media.
    pub fn new() -> Self {
        let cfg = SIPMedia {
            ctx: RefCell::new(pjsua_media_config::new()),
            capture_dev: -1,
            playback_dev: -2,
            input_level: 100,
            output_level: 100
        };

        // spesific tune for AudioIpCodec
        cfg.ctx.borrow_mut().clock_rate = 48000;
        cfg.ctx.borrow_mut().snd_clock_rate = 48000;
        cfg.ctx.borrow_mut().channel_count = 2;

        // media encoding and decoding quality
        cfg.ctx.borrow_mut().quality = 10;

        // disable voice activity detection
        cfg.ctx.borrow_mut().no_vad = PJ_TRUE as pj_bool_t;

        // disable echo cancelar
        cfg.ctx.borrow_mut().ec_tail_len = 0;
        cfg.ctx.borrow_mut().ec_options = 0;

        //ptime
        cfg.ctx.borrow_mut().ptime = 10;
        cfg.ctx.borrow_mut().jb_max = 3840;
        cfg.ctx.borrow_mut().jb_discard_algo = 0;

        cfg
    }

    pub fn init(&self) {
        if let Err(_) = pjsua::set_snd_dev(self.capture_dev, self.playback_dev) {
            panic!("cant set audio device");
        }
    }

    pub fn get_input_device_list(&self) -> Vec<String> {
        let dev_count = pjmedia::aud_dev_count();
        let mut result: Vec<String> = Vec::new();

        for idx in 0..dev_count {
            let mut info = pjmedia_aud_dev_info::new();

            if let Err(_) = pjmedia::aud_dev_get_info(idx as i32, &mut info) {
                panic!("can't enumerate input audio device");
            }

            unsafe {
                let dev_name = format!("{} (in:{}, out:{})",
                    CStr::from_ptr(info.name.as_ptr()).to_owned().into_string().expect("error"),
                    info.input_count, info.output_count);

                result.push(dev_name);
            }
        }

        result
    }

    pub fn get_output_device_list(&self) -> Vec<String> {
        let dev_count = pjmedia::aud_dev_count();
        let mut result: Vec<String> = Vec::new();

        for idx in 0..dev_count {
            let mut info: pjmedia_aud_dev_info = pjmedia_aud_dev_info::new();

            if let Err(_) = pjmedia::aud_dev_get_info(idx as i32, &mut info) {
                panic!("can't enumerate output audio device");
            }

            unsafe {
                let dev_name = format!("{} (in:{},out:{})",
                    CStr::from_ptr(info.name.as_ptr()).to_owned().into_string().expect("error"),
                    info.input_count, info.output_count);

                    result.push(dev_name);
            }
        }

        result
    }

    pub fn get_context(&self) -> RefMut<pjsua_media_config> {
        self.ctx.borrow_mut()
    }

    pub fn media_list () { }

    pub fn get_input_level(&self) -> i32 {
        self.input_level
    }

    pub fn get_output_level(&self) -> i32 {
        self.output_level
    }

    pub fn set_input_level(&mut self, value: i32) {
        self.input_level = value;
        pjsua::conf_adjust_rx_level(0, (self.input_level as f32 / 100.0) as f32).unwrap();
    }

    pub fn set_output_level(&mut self, value: i32) {
        self.output_level = value;
        pjsua::conf_adjust_tx_level(0, (self.output_level as f32 / 100.0) as f32).unwrap();
    }

    pub fn input_mute(&self, is_mute: bool) {
        if is_mute {
            pjsua::conf_adjust_rx_level(0, 0f32).unwrap();
        } else {
            pjsua::conf_adjust_rx_level(0, (self.input_level as f32 / 100.0) as f32).unwrap();
        }
    }

    pub fn output_mute(&self, is_mute: bool) {
        if is_mute {
            pjsua::conf_adjust_tx_level(0, 0f32).unwrap();
        } else {
            pjsua::conf_adjust_tx_level(0, (self.output_level as f32 / 100.0) as f32).unwrap();
        }
    }

    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        // transmit/receive signal level
        let mut tx_l: c_uint = 0;
        let mut tx_r: c_uint = 0;
        let mut rx_l: c_uint = 0;
        let mut rx_r: c_uint = 0;

        pjsua::conf_get_msignal_level(
            0,
            &mut tx_l,
            &mut tx_r,
            &mut rx_l,
            &mut rx_r
        );

        (tx_l, tx_r, rx_l, rx_r)
    }
}
