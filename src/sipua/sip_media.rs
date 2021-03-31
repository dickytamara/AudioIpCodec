

use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjsua::*;
use super::pjdefault::*;

use super::pjmedia;
use super::pjsua;

use std::os::raw::c_uint;
use std::ffi::CStr;


// Media Implementation
pub struct SIPMedia {
    ctx: pjsua_media_config,
    capture_dev: i32,
    playback_dev: i32,
    input_level: i32,
    output_level: i32
}

impl SIPMedia {

    // Create new SIP Media.
    pub fn new() -> Self {
        let mut cfg = SIPMedia {
            ctx: pjsua_media_config::new(),
            capture_dev: -1,
            playback_dev: -2,
            input_level: 100,
            output_level: 100
        };

        // spesific tune for AudioIpCodec
        cfg.ctx.clock_rate = 48000;
        cfg.ctx.snd_clock_rate = 48000;
        cfg.ctx.channel_count = 2;

        // media encoding and decoding quality
        cfg.ctx.quality = 10;

        // disable voice activity detection
        cfg.ctx.no_vad = PJ_TRUE as pj_bool_t;

        // disable echo cancelar
        cfg.ctx.ec_tail_len = 0;
        cfg.ctx.ec_options = 0;

        //ptime
        cfg.ctx.ptime = 10;
        cfg.ctx.jb_max = 3840;
        cfg.ctx.jb_discard_algo = 0;

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

    pub fn get_context(&self) -> pjsua_media_config {
        self.ctx.clone()
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
