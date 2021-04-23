

use pj_sys::*;
use pjmedia_sys::*;
use pjsua_sys::*;

use crate::pjproject::utils::{self, AutoCreate, FromString, ToString};
use crate::pjproject::pjmedia;
use crate::pjproject::pjsua;

use std::cell::{RefCell, RefMut};
use std::os::raw::c_uint;
use std::ffi::CStr;


pub struct SIPTurnServerData {
    server: String,
    username: String,
    password: String
}

impl SIPTurnServerData {
    pub fn new(srv: String, user: String, pass: String) -> Self {
        SIPTurnServerData {
            server: srv,
            username: user,
            password: pass
        }
    }
}

// Media and sound device implementation Implementation
pub struct SIPMedia {
    ctx: RefCell<pjsua_media_config>,
    capture_dev: i32,
    playback_dev: i32,
    input_level: i32,
    output_level: i32
}

pub trait SIPMediaExt {
    /// Clock rate to be applied to the conference bridge. If value is zero,
    /// default clock rate will be used (PJSUA_DEFAULT_CLOCK_RATE, which by default is 16KHz).
    fn set_clock_rate(&self, value: u32);
    fn get_clock_rate(&self) -> u32;

    /// Clock rate to be applied when opening the sound device. If value is zero,
    /// conference bridge clock rate will be used.
    fn set_snd_clock_rate(&self, value: u32);
    fn get_snd_clock_rate(&self) -> u32;

    /// Channel count be applied when opening the sound device and conference bridge.
    fn set_channel_count(&self, value: u32);
    fn get_channel_count(&self) -> u32;

    /// Specify audio frame ptime. The value here will affect the samples per frame of both the
    /// sound device and the conference bridge. Specifying lower ptime will normally reduce the latency.
    ///
    /// # Default
    /// PJSUA_DEFAULT_AUDIO_FRAME_PTIME
    fn set_audio_frame_ptime(&self, value: u32);
    fn get_audio_frame_ptime(&self) -> u32;

    /// Specify maximum number of media ports to be created in the conference bridge.
    /// Since all media terminate in the bridge (calls, file player, file recorder, etc),
    /// the value must be large enough to support all of them. However,
    /// the larger the value, the more computations are performed.
    ///
    /// # Default
    /// PJSUA_MAX_CONF_PORTS
    fn set_max_media_ports(&self, value: u32);
    fn get_max_media_ports(&self) -> u32;

    /// Specify whether the media manager should manage its own ioqueue for the RTP/RTCP sockets.
    /// If yes, ioqueue will be created and at least one worker thread will be created too.
    /// If no, the RTP/RTCP sockets will share the same ioqueue as SIP sockets,
    /// and no worker thread is needed.
    ///
    /// Normally application would say yes here, unless it wants to run everything from a single thread.
    fn set_has_ioqueue(&self, value: bool);
    fn get_has_ioqueue(&self) -> bool;

    /// Specify the number of worker threads to handle incoming RTP packets.
    /// A value of one is recommended for most applications.
    fn set_thread_cnt(&self, value: u32);
    fn get_thread_cnt(&self) -> u32;

    /// Media quality, 0-10, according to this table: 5-10:
    /// - resampling use large filter, 3-4:
    /// - resampling use small filter, 1-2:
    /// - resampling use linear.
    /// The media quality also sets speex codec quality/complexity to the number.
    ///
    /// # Default
    /// 5 (PJSUA_DEFAULT_CODEC_QUALITY).
    fn set_quality(&self, value: u32);
    fn get_quality(&self) -> u32;

    /// Specify default codec ptime.
    ///
    /// # Default
    /// 0 (codec specific)
    fn set_ptime(&self, value: u32);
    fn get_ptime(&self) -> u32;

    /// Disable VAD?
    ///
    /// # Default
    /// true (no (meaning VAD is enabled))
    fn set_no_vad(&self, value: bool);
    fn get_no_vad(&self) -> bool;

    /// iLBC mode (20 or 30).
    ///
    /// # Default
    /// 30 (PJSUA_DEFAULT_ILBC_MODE)
    fn set_ilbc_mode(&self, value: u32);
    fn get_ilbc_mode(&self) -> u32;

    /// Percentage of RTP packet to drop in TX direction (to simulate packet lost).
    ///
    /// # Default
    /// 0
    fn set_tx_drop_pct(&self, value: u32);
    fn get_tx_drop_pct(&self) -> u32;

    /// Percentage of RTP packet to drop in RX direction (to simulate packet lost).
    ///
    /// # Default
    /// 0
    fn set_rx_drop_pct(&self, value: u32);
    fn get_rx_drop_pct(&self) -> u32;

    /// Echo canceller options (see pjmedia_echo_create()).
    /// Specify PJMEDIA_ECHO_USE_SW_ECHO here if application wishes
    /// to use software echo canceller instead of device EC.
    ///
    /// # Default
    /// 0.
    fn set_ec_options(&self, value: u32);
    fn get_ec_options(&self) -> u32;

    /// Echo canceller tail length, in miliseconds.
    ///
    /// # Default
    ///  PJSUA_DEFAULT_EC_TAIL_LEN
    fn set_ec_tail_len(&self, value: u32);
    fn get_ec_tail_len(&self) -> u32;

    /// Audio capture buffer length, in milliseconds.
    ///
    /// # Default
    /// PJMEDIA_SND_DEFAULT_REC_LATENCY
    fn set_snd_rec_latency(&self, value: u32);
    fn get_snd_rec_latency(&self) -> u32;

    /// Audio playback buffer length, in milliseconds.
    ///
    /// # Default
    /// PJMEDIA_SND_DEFAULT_PLAY_LATENCY
    fn set_snd_play_latency(&self, value: u32);
    fn get_snd_paly_latency(&self) -> u32;

    /// Jitter buffer initial prefetch delay in msec.
    /// The value must be between jb_min_pre and jb_max_pre below.
    /// If the value is 0, prefetching will be disabled.
    ///
    /// # Default
    /// -1 (to use default stream settings, currently 0)
    fn set_jb_init(&self, value: i32);
    fn get_jb_init(&self) -> i32;

    /// Jitter buffer minimum prefetch delay in msec.
    ///
    /// # Default
    /// -1 (to use default stream settings, currently 60 msec)
    fn set_jb_min_pre(&self, value: i32);
    fn get_jb_min_pre(&self) -> i32;

    /// Jitter buffer maximum prefetch delay in msec.
    ///
    /// Default
    /// -1 (to use default stream settings, currently 240 msec)
    fn set_jb_max_pre(&self, value: i32);
    fn get_jb_max_pre(&self) -> i32;

    /// Set maximum delay that can be accomodated by the jitter buffer msec.
    ///
    /// Default
    /// -1 (to use default stream settings, currently 360 msec)
    fn set_jb_max(&self, value: i32);
    fn get_jb_max(&self) -> i32;

    //     pub jb_discard_algo: pjmedia_jb_discard_algo,
    fn set_jb_discard_algo(&self, value: pjmedia_jb_discard_algo);
    fn get_jb_discard_algo(&self) -> pjmedia_jb_discard_algo;

    /// Enable ICE
    fn set_enable_ice(&self, value: bool);
    fn get_enable_ice(&self) -> bool;

    /// Set the maximum number of host candidates.
    ///
    /// # Default
    /// -1 (maximum not set)
    fn set_ice_max_host_cands(&self, value: i32);
    fn get_ice_max_host_cands(&self) -> i32;

    /// ICE session options.
    fn set_ice_opt(&self, value: pj_ice_sess_options);
    fn get_ice_opt(&self) -> pj_ice_sess_options;

    /// Disable RTCP component.
    ///
    /// # Default
    /// no
    fn set_ice_no_rtcp(&self, value: bool);
    fn get_ice_no_rtcp(&self) -> bool;

    /// Send re-INVITE/UPDATE every after ICE connectivity check regardless the
    /// default ICE transport address is changed or not. When this is set to PJ_FALSE,
    /// re-INVITE/UPDATE will be sent only when the default ICE transport address is changed.
    ///
    /// # Default
    /// yes
    fn set_ice_always_update(&self, value: bool);
    fn get_ice_always_update(&self) -> bool;

    /// Enable TURN relay candidate in ICE.
    fn set_enable_turn(&self, value: bool);
    fn get_enable_turn(&self) -> bool;

    /// Specify TURN domain name or host name, in in "DOMAIN:PORT" or "HOST:PORT" format.
    fn set_turn_server(&self, value: SIPTurnServerData);
    fn get_turn_server(&self) -> String;

    /// Specify the connection type to be used to the TURN server.
    /// Valid values are PJ_TURN_TP_UDP, PJ_TURN_TP_TCP or PJ_TURN_TP_TLS.
    ///
    /// # Default
    /// PJ_TURN_TP_UDP
    fn set_turn_conn_type(&self, value: pj_turn_tp_type);
    fn get_turn_conn_type(&self) -> pj_turn_tp_type;

    /// Specify the credential to authenticate with the TURN server.
    fn set_turn_auth_cred(&self, value: pj_stun_auth_cred);
    fn get_turn_auth_cred(&self) -> pj_stun_auth_cred;

    /// This specifies TLS settings for TLS transport. It is only be used when this TLS
    /// is used to connect to the TURN server.
    fn set_turn_tls_setting(&self, value: pj_turn_sock_tls_cfg);
    fn get_turn_tls_setting(&self) -> pj_turn_sock_tls_cfg;

    /// Specify idle time of sound device before it is automatically closed,
    /// in seconds. Use value -1 to disable the auto-close feature of sound device
    ///
    /// # Default
    /// 1
    fn set_snd_auto_close_time(&self, value: i32);
    fn get_snd_auto_close_time(&self) -> i32;

    // skiped
    //     pub vid_preview_enable_native: pj_bool_t,

    /// Disable smart media update (ticket #1568). The smart media update will check
    /// for any changes in the media properties after a successful SDP negotiation
    /// and the media will only be reinitialized when any change is found. When it is disabled,
    /// media streams will always be reinitialized after a successful SDP negotiation.
    ///
    /// Note for third party media, the smart media update requires stream info
    /// retrieval capability, see PJSUA_THIRD_PARTY_STREAM_HAS_GET_INFO.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_no_smart_media_update(&self, value: bool);
    fn get_no_smart_media_update(&self) -> bool;

    /// Omit RTCP SDES and BYE in outgoing RTCP packet, this setting will be applied
    /// for both audio and video streams. Note that, when RTCP SDES and BYE are set
    /// to be omitted, RTCP SDES will still be sent once when the stream starts/stops
    /// and RTCP BYE will be sent once when the stream stops.
    ///
    /// # Default
    /// PJ_FALSE
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
        cfg.set_clock_rate(48000);
        cfg.set_snd_clock_rate(48000);
        cfg.set_channel_count(2);

        // media encoding and decoding quality
        cfg.set_quality(10);

        // disable voice activity detection
        cfg.set_no_vad(false);

        // disable echo cancelar
        cfg.set_ec_tail_len(0);
        cfg.set_ec_options(0);

        //ptime
        cfg.set_ptime(20);
        cfg.set_jb_max(3840);
        cfg.set_jb_discard_algo(0);

        cfg
    }

    pub fn init(&self) {
        pjsua::set_snd_dev(self.capture_dev, self.playback_dev)
        .expect("SIPMedia::pjsua_set_snd_dev");

    }

    pub fn get_input_device_list(&self) -> Vec<String> {
        let dev_count = pjmedia::aud_dev_count();
        let mut result: Vec<String> = Vec::new();

        for idx in 0..dev_count {
            let mut info = pjmedia_aud_dev_info::new();

            pjmedia::aud_dev_get_info(idx as i32, &mut info)
            .expect("SIPMedia::pjsua_aud_dev_get_info");

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

            pjmedia::aud_dev_get_info(idx as i32, &mut info)
            .expect("SIPMedia::pjsua_aud_dev_get_info");

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
        pjsua::conf_adjust_rx_level(0, (self.input_level as f32 / 100.0) as f32)
        .expect("SIPMedia::pjsua_conf_adjust_rx_level");
    }

    pub fn set_output_level(&mut self, value: i32) {
        self.output_level = value;
        pjsua::conf_adjust_tx_level(0, (self.output_level as f32 / 100.0) as f32)
        .expect("SIPMedia::pjsua_confg_adjust_tx_level");
    }

    pub fn input_mute(&self, is_mute: bool) {
        if is_mute {
            pjsua::conf_adjust_rx_level(0, 0f32)
            .expect("SIPMedia::pjsua_conf_adjust_rx_level");
        } else {
            pjsua::conf_adjust_rx_level(0, (self.input_level as f32 / 100.0) as f32)
            .expect("SIPMedia::pjsua_conf_adjust_rx_level");
        }
    }

    pub fn output_mute(&self, is_mute: bool) {
        if is_mute {
            pjsua::conf_adjust_tx_level(0, 0f32)
            .expect("SIPMedia::pjsua_conf_adjust_tx_level");
        } else {
            pjsua::conf_adjust_tx_level(0, (self.output_level as f32 / 100.0) as f32)
            .expect("SIPMedia::pjsua_conf_adjust_tx_level");
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

impl SIPMediaExt for SIPMedia {

    fn set_clock_rate(&self, value: u32) {
        self.ctx.borrow_mut().clock_rate = value;
    }

    fn get_clock_rate(&self) -> u32 {
        self.ctx.borrow().clock_rate
    }

    fn set_snd_clock_rate(&self, value: u32) {
        self.ctx.borrow_mut().snd_clock_rate = value;
    }

    fn get_snd_clock_rate(&self) -> u32 {
        self.ctx.borrow().snd_clock_rate
    }

    fn set_channel_count(&self, value: u32) {
        self.ctx.borrow_mut().channel_count = value;
    }

    fn get_channel_count(&self) -> u32 {
        self.ctx.borrow().channel_count
    }

    fn set_audio_frame_ptime(&self, value: u32) {
        self.ctx.borrow_mut().audio_frame_ptime = value;
    }

    fn get_audio_frame_ptime(&self) -> u32 {
        self.ctx.borrow().audio_frame_ptime
    }

    fn set_max_media_ports(&self, value: u32) {
        self.ctx.borrow_mut().max_media_ports = value;
    }

    fn get_max_media_ports(&self) -> u32 {
        self.ctx.borrow().max_media_ports
    }

    fn set_has_ioqueue(&self, value: bool) {
        self.ctx.borrow_mut().has_ioqueue = utils::boolean_to_pjbool(value);
    }

    fn get_has_ioqueue(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().has_ioqueue)
    }

    fn set_thread_cnt(&self, value: u32) {
        self.ctx.borrow_mut().thread_cnt = value;
    }

    fn get_thread_cnt(&self) -> u32 {
        self.ctx.borrow().thread_cnt
    }

    fn set_quality(&self, value: u32) {
        self.ctx.borrow_mut().quality = value;
    }

    fn get_quality(&self) -> u32 {
        self.ctx.borrow().quality
    }

    fn set_ptime(&self, value: u32) {
        self.ctx.borrow_mut().ptime = value;
    }

    fn get_ptime(&self) -> u32 {
        self.ctx.borrow().ptime
    }

    fn set_no_vad(&self, value: bool) {
        self.ctx.borrow_mut().no_vad = utils::boolean_to_pjbool(value);
    }

    fn get_no_vad(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().no_vad)
    }

    fn set_ilbc_mode(&self, value: u32) {
        self.ctx.borrow_mut().ilbc_mode = value;
    }

    fn get_ilbc_mode(&self) -> u32 {
        self.ctx.borrow().ilbc_mode
    }

    fn set_tx_drop_pct(&self, value: u32) {
        self.ctx.borrow_mut().tx_drop_pct = value;
    }

    fn get_tx_drop_pct(&self) -> u32 {
        self.ctx.borrow().tx_drop_pct
    }

    fn set_rx_drop_pct(&self, value: u32) {
        self.ctx.borrow_mut().rx_drop_pct = value;
    }

    fn get_rx_drop_pct(&self) -> u32 {
        self.ctx.borrow().rx_drop_pct
    }

    fn set_ec_options(&self, value: u32) {
        self.ctx.borrow_mut().ec_options = value;
    }

    fn get_ec_options(&self) -> u32 {
        self.ctx.borrow().ec_options
    }

    fn set_ec_tail_len(&self, value: u32) {
        self.ctx.borrow_mut().ec_tail_len = value;
    }

    fn get_ec_tail_len(&self) -> u32 {
        self.ctx.borrow().ec_tail_len
    }

    fn set_snd_rec_latency(&self, value: u32) {
        self.ctx.borrow_mut().snd_rec_latency = value;
    }

    fn get_snd_rec_latency(&self) -> u32 {
        self.ctx.borrow().snd_rec_latency
    }

    fn set_snd_play_latency(&self, value: u32) {
        self.ctx.borrow_mut().snd_play_latency = value;
    }

    fn get_snd_paly_latency(&self) -> u32 {
        self.ctx.borrow().snd_play_latency
    }

    fn set_jb_init(&self, value: i32) {
        self.ctx.borrow_mut().jb_init = value;
    }

    fn get_jb_init(&self) -> i32 {
        self.ctx.borrow().jb_init
    }

    fn set_jb_min_pre(&self, value: i32) {
        self.ctx.borrow_mut().jb_min_pre = value;
    }

    fn get_jb_min_pre(&self) -> i32 {
        self.ctx.borrow().jb_min_pre
    }

    fn set_jb_max_pre(&self, value: i32) {
        self.ctx.borrow_mut().jb_max_pre = value;
    }

    fn get_jb_max_pre(&self) -> i32 {
        self.ctx.borrow().jb_max_pre
    }

    fn set_jb_max(&self, value: i32) {
        self.ctx.borrow_mut().jb_max = value;
    }

    fn get_jb_max(&self) -> i32 {
        self.ctx.borrow().jb_max
    }

    fn set_jb_discard_algo(&self, value: pjmedia_jb_discard_algo) {
        self.ctx.borrow_mut().jb_discard_algo = value;
    }

    fn get_jb_discard_algo(&self) -> pjmedia_jb_discard_algo {
        self.ctx.borrow().jb_discard_algo
    }

    fn set_enable_ice(&self, value: bool) {
        self.ctx.borrow_mut().enable_ice = utils::boolean_to_pjbool(value);
    }

    fn get_enable_ice(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().enable_ice)
    }

    fn set_ice_max_host_cands(&self, value: i32) {
        self.ctx.borrow_mut().ice_max_host_cands = value;
    }

    fn get_ice_max_host_cands(&self) -> i32 {
        self.ctx.borrow().ice_max_host_cands
    }

    fn set_ice_opt(&self, value: pj_ice_sess_options) {
        self.ctx.borrow_mut().ice_opt = value;
    }

    fn get_ice_opt(&self) -> pj_ice_sess_options {
        self.ctx.borrow().ice_opt
    }

    fn set_ice_no_rtcp(&self, value: bool) {
        self.ctx.borrow_mut().ice_no_rtcp = utils::boolean_to_pjbool(value);
    }

    fn get_ice_no_rtcp(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().ice_no_rtcp)
    }

    fn set_ice_always_update(&self, value: bool) {
        self.ctx.borrow_mut().ice_always_update = utils::boolean_to_pjbool(value);
    }

    fn get_ice_always_update(&self) -> bool {
        utils::check_boolean(self.ctx.borrow_mut().ice_always_update)
    }

    fn set_enable_turn(&self, value: bool) {
        self.ctx.borrow_mut().enable_turn = utils::boolean_to_pjbool(value);
    }

    fn get_enable_turn(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().enable_turn)
    }

    fn set_turn_server(&self, value: SIPTurnServerData) {
        if !value.server.is_empty() {
            self.ctx.borrow_mut().turn_server = pj_str_t::from_string(value.server);
            if !value.username.is_empty() & !value.password.is_empty() {
                self.ctx.borrow_mut().turn_auth_cred.data.static_cred.username = pj_str_t::from_string(value.username);
                self.ctx.borrow_mut().turn_auth_cred.data.static_cred.data = pj_str_t::from_string(value.password);
            }
        }
    }

    fn get_turn_server(&self) -> String {
        self.ctx.borrow().turn_server.to_string()
    }

    fn set_turn_conn_type(&self, value: pj_turn_tp_type) {
        self.ctx.borrow_mut().turn_conn_type = value;
    }

    fn get_turn_conn_type(&self) -> pj_turn_tp_type {
        self.ctx.borrow().turn_conn_type
    }

    fn set_turn_auth_cred(&self, value: pj_stun_auth_cred) {
        self.ctx.borrow_mut().turn_auth_cred = value;
    }

    fn get_turn_auth_cred(&self) -> pj_stun_auth_cred {
        self.ctx.borrow().turn_auth_cred
    }

    fn set_turn_tls_setting(&self, value: pj_turn_sock_tls_cfg) {
        self.ctx.borrow_mut().turn_tls_setting = value;
    }

    fn get_turn_tls_setting(&self) -> pj_turn_sock_tls_cfg {
        self.ctx.borrow().turn_tls_setting
    }

    fn set_snd_auto_close_time(&self, value: i32) {
        self.ctx.borrow_mut().snd_auto_close_time = value;
    }

    fn get_snd_auto_close_time(&self) -> i32 {
        self.ctx.borrow().snd_auto_close_time
    }

    fn set_no_smart_media_update(&self, value: bool) {
        self.ctx.borrow_mut().no_smart_media_update = utils::boolean_to_pjbool(value);
    }

    fn get_no_smart_media_update(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().no_smart_media_update)
    }

    fn set_no_rtcp_sdes_bye(&self, value: bool) {
        self.ctx.borrow_mut().no_rtcp_sdes_bye = utils::boolean_to_pjbool(value);
    }

    fn get_no_rtcp_sdes_bye(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().no_rtcp_sdes_bye)
    }
}
