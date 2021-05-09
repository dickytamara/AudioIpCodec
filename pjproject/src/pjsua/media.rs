
use std::{convert::TryFrom, path::PathBuf};
use crate::{pjmedia::{MediaEchoFlag, MediaJbDiscardAlgo, MediaPort, MediaWavPlayerInfo}, pjnath::{IceSessTrickle, TurnTpType}, utils::{boolean_to_pjbool, check_boolean}};
use super::*;

pub trait UAMediaConfigExt {

    /// Clock rate to be applied to the conference bridge. If value is zero,
    /// default clock rate will be used (PJSUA_DEFAULT_CLOCK_RATE, which by default is 16KHz).
    fn set_clock_rate(&mut self, value: ClockRate);
    fn get_clock_rate(&self) -> ClockRate;

    /// Clock rate to be applied when opening the sound device. If value is zero,
    /// conference bridge clock rate will be used.
    fn set_snd_clock_rate(&mut self, value: ClockRate);
    fn get_snd_clock_rate(&self) -> ClockRate;

    /// Channel count be applied when opening the sound device and conference bridge.
    fn set_channel_count(&mut self, value: ConfigChannel);
    fn get_channel_count(&self) -> ConfigChannel;

    /// Specify audio frame ptime. The value here will affect the samples per frame of both the
    /// sound device and the conference bridge. Specifying lower ptime will normally reduce the latency.
    ///
    /// # Default
    /// PJSUA_DEFAULT_AUDIO_FRAME_PTIME
    fn set_audio_frame_ptime(&mut self, value: u32);
    fn get_audio_frame_ptime(&self) -> u32;

    /// Specify maximum number of media ports to be created in the conference bridge.
    /// Since all media terminate in the bridge (calls, file player, file recorder, etc),
    /// the value must be large enough to support all of them. However,
    /// the larger the value, the more computations are performed.
    ///
    /// # Default
    /// PJSUA_MAX_CONF_PORTS
    fn set_max_media_ports(&mut self, value: u32);
    fn get_max_media_ports(&self) -> u32;

    /// Specify whether the media manager should manage its own ioqueue for the RTP/RTCP sockets.
    /// If yes, ioqueue will be created and at least one worker thread will be created too.
    /// If no, the RTP/RTCP sockets will share the same ioqueue as SIP sockets,
    /// and no worker thread is needed.
    ///
    /// Normally application would say yes here, unless it wants to run everything from a single thread.
    fn set_has_ioqueue(&mut self, value: bool);
    fn get_has_ioqueue(&self) -> bool;

    /// Specify the number of worker threads to handle incoming RTP packets.
    /// A value of one is recommended for most applications.
    fn set_thread_cnt(&mut self, value: u32);
    fn get_thread_cnt(&self) -> u32;

    /// Media quality, 0-10, according to this table: 5-10:
    /// - resampling use large filter, 3-4:
    /// - resampling use small filter, 1-2:
    /// - resampling use linear.
    /// The media quality also sets speex codec quality/complexity to the number.
    ///
    /// # Default
    /// 5 (PJSUA_DEFAULT_CODEC_QUALITY).
    fn set_quality(&mut self, value: EncodingQuality);
    fn get_quality(&self) -> EncodingQuality;

    /// Specify default codec ptime.
    ///
    /// # Default
    /// 0 (codec specific)
    fn set_ptime(&mut self, value: u32);
    fn get_ptime(&self) -> u32;

    /// Disable VAD?
    ///
    /// # Default
    /// true (no (meaning VAD is enabled))
    fn set_no_vad(&mut self, value: bool);
    fn get_no_vad(&self) -> bool;

    /// iLBC mode (20 or 30).
    ///
    /// # Default
    /// 30 (PJSUA_DEFAULT_ILBC_MODE)
    fn set_ilbc_mode(&mut self, value: IlbcMode);
    fn get_ilbc_mode(&self) -> IlbcMode;

    /// Percentage of RTP packet to drop in TX direction (to simulate packet lost).
    ///
    /// # Default
    /// 0
    fn set_tx_drop_pct(&mut self, value: u32);
    fn get_tx_drop_pct(&self) -> u32;

    /// Percentage of RTP packet to drop in RX direction (to simulate packet lost).
    ///
    /// # Default
    /// 0
    fn set_rx_drop_pct(&mut self, value: u32);
    fn get_rx_drop_pct(&self) -> u32;

    /// Echo canceller options (see pjmedia_echo_create()).
    /// Specify PJMEDIA_ECHO_USE_SW_ECHO here if application wishes
    /// to use software echo canceller instead of device EC.
    ///
    /// # Default
    /// 0.
    fn set_ec_options(&mut self, value: MediaEchoFlag);
    fn get_ec_options(&self) -> MediaEchoFlag;

    /// Echo canceller tail length, in miliseconds.
    ///
    /// # Default
    ///  PJSUA_DEFAULT_EC_TAIL_LEN
    fn set_ec_tail_len(&mut self, value: u32);
    fn get_ec_tail_len(&self) -> u32;

    /// Audio capture buffer length, in milliseconds.
    ///
    /// # Default
    /// PJMEDIA_SND_DEFAULT_REC_LATENCY
    fn set_snd_rec_latency(&mut self, value: u32);
    fn get_snd_rec_latency(&self) -> u32;

    /// Audio playback buffer length, in milliseconds.
    ///
    /// # Default
    /// PJMEDIA_SND_DEFAULT_PLAY_LATENCY
    fn set_snd_play_latency(&mut self, value: u32);
    fn get_snd_play_latency(&self) -> u32;

    /// Jitter buffer initial prefetch delay in msec.
    /// The value must be between jb_min_pre and jb_max_pre below.
    /// If the value is 0, prefetching will be disabled.
    ///
    /// # Default
    /// -1 (to use default stream settings, currently 0)
    fn set_jb_init(&mut self, value: i32);
    fn get_jb_init(&self) -> i32;

    /// Jitter buffer minimum prefetch delay in msec.
    ///
    /// # Default
    /// -1 (to use default stream settings, currently 60 msec)
    fn set_jb_min_pre(&mut self, value: i32);
    fn get_jb_min_pre(&self) -> i32;

    /// Jitter buffer maximum prefetch delay in msec.
    ///
    /// Default
    /// -1 (to use default stream settings, currently 240 msec)
    fn set_jb_max_pre(&mut self, value: i32);
    fn get_jb_max_pre(&self) -> i32;

    /// Set maximum delay that can be accomodated by the jitter buffer msec.
    ///
    /// Default
    /// -1 (to use default stream settings, currently 360 msec)
    fn set_jb_max(&mut self, value: i32);
    fn get_jb_max(&self) -> i32;

    //     pub jb_discard_algo: pjmedia_jb_discard_algo,
    fn set_jb_discard_algo(&mut self, value: MediaJbDiscardAlgo);
    fn get_jb_discard_algo(&self) -> MediaJbDiscardAlgo;

    /// Enable ICE
    fn set_enable_ice(&mut self, value: bool);
    fn get_enable_ice(&self) -> bool;

    /// Set the maximum number of host candidates.
    ///
    /// # Default
    /// -1 (maximum not set)
    fn set_ice_max_host_cands(&mut self, value: i32);
    fn get_ice_max_host_cands(&self) -> i32;

    /// ICE session options.
    fn set_ice_opt(&mut self,
        aggresive: Option<bool>,
        nominated_check_delay: Option<u32>,
        controlled_agent_want_nom_timeout: Option<i32>,
        trickle: Option<IceSessTrickle>,
    );
    fn get_ice_opt(&self) -> (bool, u32, i32, IceSessTrickle);

    /// Disable RTCP component.
    ///
    /// # Default
    /// no
    fn set_ice_no_rtcp(&mut self, value: bool);
    fn get_ice_no_rtcp(&self) -> bool;

    /// Send re-INVITE/UPDATE every after ICE connectivity check regardless the
    /// default ICE transport address is changed or not. When this is set to PJ_FALSE,
    /// re-INVITE/UPDATE will be sent only when the default ICE transport address is changed.
    ///
    /// # Default
    /// yes
    fn set_ice_always_update(&mut self, value: bool);
    fn get_ice_always_update(&self) -> bool;

    /// Enable TURN relay candidate in ICE.
    fn set_enable_turn(&mut self, value: bool);
    fn get_enable_turn(&self) -> bool;

    /// Specify TURN domain name or host name, in in "DOMAIN:PORT" or "HOST:PORT" format.
    fn set_turn_server(&mut self, value: String);
    fn get_turn_server(&self) -> String;

    /// Specify the connection type to be used to the TURN server.
    /// Valid values are PJ_TURN_TP_UDP, PJ_TURN_TP_TCP or PJ_TURN_TP_TLS.
    ///
    /// # Default
    /// PJ_TURN_TP_UDP
    fn set_turn_conn_type(&mut self, value: TurnTpType);
    fn get_turn_conn_type(&self) -> TurnTpType;

    /// Specify the credential to authenticate with the TURN server.
    /// see pjsua api this rust api only support static credential
    fn set_turn_auth_cred(&mut self,
        realm: Option<String>,
        username: Option<String>,
        data_type: Option<CredentialInfoType>,
        data: Option<String>,
        nonce: Option<String>
    );
    fn get_turn_auth_cred(&self) -> (String, String, CredentialInfoType, String, String);

    /// This specifies TLS settings for TLS transport. It is only be used when this TLS
    /// is used to connect to the TURN server.
    fn set_turn_tls_setting(&mut self, path: Option<PathBuf>, ca: String, cert: String, privkey: String, password: String);
    fn get_turn_tls_setting(&self) -> (Option<PathBuf>, String, String, String, String);

    /// Specify idle time of sound device before it is automatically closed,
    /// in seconds. Use value -1 to disable the auto-close feature of sound device
    ///
    /// # Default
    /// 1
    fn set_snd_auto_close_time(&mut self, value: i32);
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
    fn set_no_smart_media_update(&mut self, value: bool);
    fn get_no_smart_media_update(&self) -> bool;

    /// Omit RTCP SDES and BYE in outgoing RTCP packet, this setting will be applied
    /// for both audio and video streams. Note that, when RTCP SDES and BYE are set
    /// to be omitted, RTCP SDES will still be sent once when the stream starts/stops
    /// and RTCP BYE will be sent once when the stream stops.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_no_rtcp_sdes_bye(&mut self, value: bool);
    fn get_no_rtcp_sdes_bye(&self) -> bool;

    // TODO implement callback
    //     pub on_aud_prev_play_frame: Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
    //     pub on_aud_prev_rec_frame: Option<unsafe extern "C" fn(frame: *mut pjmedia_frame)>,
}

pub trait UACodecInfoExt {

    /// Codec unique identification.
    fn get_codec_id (&self) -> String;


    /// Codec priority (integer 0-255).
    fn get_priority (&self) -> u8;

    ///Codec description.
    fn get_desc (&self) -> String;
    // fn get_buf_ (&self) -> [::std::os::raw::c_char; 64usize],
}

pub trait UAConfPortInfoExt {

    fn get_slot_id (&self) -> i32;
    fn get_name (&self) -> String;
    fn get_format (&self) -> &pjmedia_sys::pjmedia_format;
    fn get_clock_rate (&self) -> ClockRate;
    fn get_channel_count (&self) -> ConfigChannel;
    fn get_samples_per_frame (&self) -> u32;
    fn get_bits_per_sample (&self) -> u32;
    fn get_tx_level_adj (&self) -> f32;
    fn get_rx_level_adj (&self) -> f32;
    fn get_listener_cnt (&self) -> u32;
    // fn get_listeners (&self) -> [pjsua_conf_port_id; 254usize];

}

pub trait UAMediaTransportExt {

    fn set_skinfo (&mut self, value: pjmedia_sys::pjmedia_sock_info);
    fn get_skinfo (&self) -> &pjmedia_sys::pjmedia_sock_info;

    // fn set_transport (&mut self, value: pjmedia_sys::pjmedia_transport);
    fn get_transport (&self) -> *mut pjmedia_sys::pjmedia_transport;

}

pub trait UASndDevParamExt {

    fn set_capture_dev(&mut self, value: i32);
    fn get_capture_dev(&self) -> i32;

    fn set_playback_dev(&mut self, value: i32);
    fn get_playback_dev(&self) -> i32;

    fn set_mode(&mut self, value: u32);
    fn get_mode(&self) -> u32;

}

pub trait UAConfConectParamExt {

    fn set_level(&mut self, value: f32);
    fn get_level(&self) -> f32;

}



impl UAMediaConfigExt for UAMediaConfig {

    fn set_clock_rate(&mut self, value: ClockRate) {
        self.clock_rate = value.into();
    }

    fn get_clock_rate(&self) -> ClockRate {
        ClockRate::try_from(self.clock_rate)
        .expect("Error MediaConfig get clock_rate")
    }

    fn set_snd_clock_rate(&mut self, value: ClockRate) {
        self.snd_clock_rate = value.into();
    }

    fn get_snd_clock_rate(&self) -> ClockRate {
        ClockRate::try_from(self.snd_clock_rate)
        .expect("Error MediaConfig get snd_clock_rate")
    }

    fn set_channel_count(&mut self, value: ConfigChannel) {
        self.channel_count = value.into();
    }

    fn get_channel_count(&self) -> ConfigChannel {
        ConfigChannel::try_from(self.channel_count)
        .expect("Error MediaConfig get channel_count")
    }

    fn set_audio_frame_ptime(&mut self, value: u32) {
        self.audio_frame_ptime = value;
    }

    fn get_audio_frame_ptime(&self) -> u32 {
        self.audio_frame_ptime
    }

    fn set_max_media_ports(&mut self, value: u32) {
        self.max_media_ports = value;
    }

    fn get_max_media_ports(&self) -> u32 {
        self.max_media_ports
    }

    fn set_has_ioqueue(&mut self, value: bool) {
        self.has_ioqueue = boolean_to_pjbool(value);
    }

    fn get_has_ioqueue(&self) -> bool {
        check_boolean(self.has_ioqueue)
    }

    fn set_thread_cnt(&mut self, value: u32) {
        self.thread_cnt = value;
    }

    fn get_thread_cnt(&self) -> u32 {
        self.thread_cnt
    }

    fn set_quality(&mut self, value: EncodingQuality) {
        self.quality = value.into();
    }

    fn get_quality(&self) -> EncodingQuality {
        EncodingQuality::try_from(self.quality)
        .expect("Error MediaConfig get quality")
    }

    fn set_ptime(&mut self, value: u32) {
        self.ptime = value;
    }

    fn get_ptime(&self) -> u32 {
        self.ptime
    }

    fn set_no_vad(&mut self, value: bool) {
        self.no_vad = boolean_to_pjbool(value);
    }

    fn get_no_vad(&self) -> bool {
        check_boolean(self.no_vad)
    }

    fn set_ilbc_mode(&mut self, value: IlbcMode) {
        self.ilbc_mode = value.into();
    }

    fn get_ilbc_mode(&self) -> IlbcMode {
        IlbcMode::try_from(self.ilbc_mode)
        .expect("Error MediaConfig get ilbc_mode")
    }

    fn set_tx_drop_pct(&mut self, value: u32) {
        self.tx_drop_pct = value;
    }

    fn get_tx_drop_pct(&self) -> u32 {
        self.tx_drop_pct
    }

    fn set_rx_drop_pct(&mut self, value: u32) {
        self.rx_drop_pct = value;
    }

    fn get_rx_drop_pct(&self) -> u32 {
        self.rx_drop_pct
    }

    fn set_ec_options(&mut self, value: MediaEchoFlag) {
        self.ec_options = value.into();
    }

    fn get_ec_options(&self) -> MediaEchoFlag {
        MediaEchoFlag::try_from(self.ec_options)
        .expect("Error MediaConfig get ec_options")
    }

    fn set_ec_tail_len(&mut self, value: u32) {
        self.ec_tail_len = value;
    }

    fn get_ec_tail_len(&self) -> u32 {
        self.ec_tail_len
    }

    fn set_snd_rec_latency(&mut self, value: u32) {
        self.snd_rec_latency = value;
    }

    fn get_snd_rec_latency(&self) -> u32 {
        self.snd_rec_latency
    }

    fn set_snd_play_latency(&mut self, value: u32) {
        self.snd_play_latency = value;
    }

    fn get_snd_play_latency(&self) -> u32 {
        self.snd_play_latency
    }

    fn set_jb_init(&mut self, value: i32) {
        self.jb_init = value;
    }

    fn get_jb_init(&self) -> i32 {
        self.jb_init
    }

    fn set_jb_min_pre(&mut self, value: i32) {
        self.jb_min_pre = value;
    }

    fn get_jb_min_pre(&self) -> i32 {
        self.jb_min_pre
    }

    fn set_jb_max_pre(&mut self, value: i32) {
        self.jb_max_pre = value;
    }

    fn get_jb_max_pre(&self) -> i32 {
        self.jb_max_pre
    }

    fn set_jb_max(&mut self, value: i32) {
        self.jb_max = value;
    }

    fn get_jb_max(&self) -> i32 {
        self.jb_max
    }

    fn set_jb_discard_algo(&mut self, value: MediaJbDiscardAlgo) {
        self.jb_discard_algo = value.into();
    }

    fn get_jb_discard_algo(&self) -> MediaJbDiscardAlgo {
        MediaJbDiscardAlgo::try_from(self.jb_discard_algo)
        .expect("Error MediaConfig get jb_discard_algo")
    }

    fn set_enable_ice(&mut self, value: bool) {
        self.enable_ice = boolean_to_pjbool(value);
    }

    fn get_enable_ice(&self) -> bool {
        check_boolean(self.enable_ice)
    }

    fn set_ice_max_host_cands(&mut self, value: i32) {
        self.ice_max_host_cands = value;
    }

    fn get_ice_max_host_cands(&self) -> i32 {
        self.ice_max_host_cands
    }


    fn set_ice_opt(&mut self,
        aggresive: Option<bool>,
        nominated_check_delay: Option<u32>,
        controlled_agent_want_nom_timeout: Option<i32>,
        trickle: Option<IceSessTrickle>,
    ) {

        if aggresive.is_some() {
            self.ice_opt.aggressive = boolean_to_pjbool(aggresive.unwrap());
        }

        if nominated_check_delay.is_some() {
            self.ice_opt.nominated_check_delay = nominated_check_delay.unwrap();
        }

        if controlled_agent_want_nom_timeout.is_some() {
            self.ice_opt.controlled_agent_want_nom_timeout =
            controlled_agent_want_nom_timeout.unwrap();
        }

        if trickle.is_some() {
            self.ice_opt.trickle = trickle.unwrap().into();
        }

    }

    fn get_ice_opt(&self) -> (bool, u32, i32, IceSessTrickle) {
        (
            check_boolean(self.ice_opt.aggressive),
            self.ice_opt.nominated_check_delay,
            self.ice_opt.controlled_agent_want_nom_timeout,
            IceSessTrickle::try_from(self.ice_opt.trickle)
            .expect("Error MediaConfig get ice_opt.trickle")
        )
    }

    fn set_ice_no_rtcp(&mut self, value: bool) {
        self.ice_no_rtcp = boolean_to_pjbool(value);
    }

    fn get_ice_no_rtcp(&self) -> bool {
        check_boolean(self.ice_no_rtcp)
    }

    fn set_ice_always_update(&mut self, value: bool) {
        self.ice_always_update = boolean_to_pjbool(value);
    }

    fn get_ice_always_update(&self) -> bool {
        check_boolean(self.ice_always_update)
    }

    fn set_enable_turn(&mut self, value: bool) {
        self.enable_turn = boolean_to_pjbool(value);
    }

    fn get_enable_turn(&self) -> bool {
        check_boolean(self.enable_turn)
    }

    fn set_turn_server(&mut self, value: String) {
        self.turn_server = pj_str_t::from_string(value);
    }

    fn get_turn_server(&self) -> String {
        self.turn_server.to_string()
    }

    fn set_turn_conn_type(&mut self, value: TurnTpType) {
        self.turn_conn_type = value.into();
    }

    fn get_turn_conn_type(&self) -> TurnTpType {
        TurnTpType::try_from(self.turn_conn_type)
        .expect("Error MediaConfig get turn_conn_type")
    }

    fn set_turn_auth_cred(&mut self,
        realm: Option<String>,
        username: Option<String>,
        data_type: Option<CredentialInfoType>,
        data: Option<String>,
        nonce: Option<String>
    ) {
        // always set in static
        self.turn_auth_cred.type_ = 0;

        if realm.is_some() {
            unsafe {
                self.turn_auth_cred.data.static_cred.as_mut().realm =
                pj_str_t::from_string(realm.unwrap());
            }
        }

        if username.is_some() {
            unsafe {
                self.turn_auth_cred.data.static_cred.as_mut().username =
                pj_str_t::from_string(username.unwrap());
            }
        }

        if data_type.is_some() {
            unsafe {
                // rust-analyzer unable to detect this
                // probably bug in rust-analyzer or rust AST Parser.
                let ctype: i32 = data_type.unwrap().into();
                self.turn_auth_cred.data.static_cred.as_mut().data_type = ctype as u32;
            }
        }

        if data.is_some() {
            unsafe {
                self.turn_auth_cred.data.static_cred.as_mut().data =
                pj_str_t::from_string(data.unwrap());
            }
        }

        if nonce.is_some() {
            unsafe {
                self.turn_auth_cred.data.static_cred.as_mut().nonce =
                pj_str_t::from_string(nonce.unwrap());
            }
        }

    }

    fn get_turn_auth_cred(&self) -> (String, String, CredentialInfoType, String, String) {
        unsafe {
            (
                self.turn_auth_cred.data.static_cred.as_ref().realm.to_string(),
                self.turn_auth_cred.data.static_cred.as_ref().username.to_string(),
                CredentialInfoType::try_from(self.turn_auth_cred.data.static_cred.as_ref().data_type.clone() as i32)
                .expect("Error MediaConfig get turn_auth_cred.data_type"),
                self.turn_auth_cred.data.static_cred.as_ref().data.to_string(),
                self.turn_auth_cred.data.static_cred.as_ref().nonce.to_string(),
            )
        }
    }

    fn set_turn_tls_setting(&mut self,
        path: Option<PathBuf>,
        ca: String,
        cert: String,
        privkey: String,
        password: String
    )
    {
        match path {
            Some(path) => {
                self.turn_tls_setting.ca_list_path = pj_str_t::from_string(
                    String::from(path.to_str().unwrap())
                );
                self.turn_tls_setting.ca_list_file = pj_str_t::from_string(ca);
                self.turn_tls_setting.cert_file = pj_str_t::from_string(cert);
                self.turn_tls_setting.privkey_file = pj_str_t::from_string(privkey);
                self.turn_tls_setting.password = pj_str_t::from_string(password);
            },
            None => {
                self.turn_tls_setting.ca_buf = pj_str_t::from_string(ca);
                self.turn_tls_setting.cert_buf = pj_str_t::from_string(cert);
                self.turn_tls_setting.privkey_buf = pj_str_t::from_string(privkey);
                self.turn_tls_setting.password = pj_str_t::from_string(password);
            }
        }
    }

    fn get_turn_tls_setting(&self) -> (Option<PathBuf>, String, String, String, String) {
        // return based on ca_list_path
        let path = self.turn_tls_setting.ca_list_path.to_string();
        if path.is_empty() {
            (
                None,
                self.turn_tls_setting.ca_buf.to_string(),
                self.turn_tls_setting.cert_buf.to_string(),
                self.turn_tls_setting.privkey_buf.to_string(),
                self.turn_tls_setting.password.to_string()
            )
        } else {
            (
                Some(PathBuf::from(path)),
                self.turn_tls_setting.ca_list_file.to_string(),
                self.turn_tls_setting.cert_file.to_string(),
                self.turn_tls_setting.privkey_file.to_string(),
                self.turn_tls_setting.password.to_string()
            )
        }
    }

    fn set_snd_auto_close_time(&mut self, value: i32) {
        self.snd_auto_close_time = value;
    }

    fn get_snd_auto_close_time(&self) -> i32 {
        self.snd_auto_close_time
    }

    fn set_no_smart_media_update(&mut self, value: bool) {
        self.no_smart_media_update = boolean_to_pjbool(value);
    }

    fn get_no_smart_media_update(&self) -> bool {
        check_boolean(self.no_smart_media_update)
    }

    fn set_no_rtcp_sdes_bye(&mut self, value: bool) {
        self.no_rtcp_sdes_bye = boolean_to_pjbool(value);
    }

    fn get_no_rtcp_sdes_bye(&self) -> bool {
        check_boolean(self.no_rtcp_sdes_bye)
    }
}

impl UACodecInfoExt for UACodecInfo  {
    fn get_codec_id (&self) -> String {
        self.codec_id.to_string()
    }
    fn get_priority (&self) -> u8 {
        self.priority
    }
    fn get_desc (&self) -> String {
        self.desc.to_string()
    }
}

impl UAConfPortInfoExt for UAConfPortInfo {

    fn get_slot_id (&self) -> i32 {
        self.slot_id
    }

    fn get_name (&self) -> String {
        self.name.to_string()
    }

    fn get_format (&self) -> &pjmedia_sys::pjmedia_format {
        &self.format
    }

    fn get_clock_rate (&self) -> ClockRate {
        ClockRate::try_from(self.clock_rate)
        .expect("Error UAConfPortInfo get clock_rate")
    }

    fn get_channel_count (&self) -> ConfigChannel {
        ConfigChannel::try_from(self.channel_count)
        .expect("Error UAConfPortInfo get channel_count")
    }

    fn get_samples_per_frame (&self) -> u32 {
        self.samples_per_frame
    }

    fn get_bits_per_sample (&self) -> u32 {
        self.bits_per_sample
    }

    fn get_tx_level_adj (&self) -> f32 {
        self.tx_level_adj
    }

    fn get_rx_level_adj (&self) -> f32 {
        self.rx_level_adj
    }

    fn get_listener_cnt (&self) -> u32 {
        self.listener_cnt
    }

}

impl UAMediaTransportExt for UAMediaTransport {

    fn set_skinfo (&mut self, value: pjmedia_sys::pjmedia_sock_info) {
        self.skinfo = value;
    }

    fn get_skinfo (&self) -> &pjmedia_sys::pjmedia_sock_info {
        &self.skinfo
    }

    fn get_transport (&self) -> *mut pjmedia_sys::pjmedia_transport {
        self.transport
    }

}

impl UASndDevParamExt for UASndDevParam {

    fn set_capture_dev(&mut self, value: i32) {
        self.capture_dev = value;
    }

    fn get_capture_dev(&self) -> i32 {
        self.capture_dev
    }

    fn set_playback_dev(&mut self, value: i32) {
        self.playback_dev = value;
    }

    fn get_playback_dev(&self) -> i32 {
        self.playback_dev
    }

    fn set_mode(&mut self, value: u32) {
        self.mode = value;
    }

    fn get_mode(&self) -> u32 {
        self.mode
    }

}

impl UAConfConectParamExt for UAConfConectParam {

    fn set_level(&mut self, value: f32) {
        self.level = value;
    }

    fn get_level(&self) -> f32 {
        self.level
    }

}

pub struct UAConf {}

impl Default for UAConf {
    fn default() -> Self {
        Self {}
    }
}

impl UAConf {

    pub fn conf_connect_param_default(prm: &mut pjsua_conf_connect_param) {
        unsafe { pjsua_sys::pjsua_conf_connect_param_default(prm as *mut _); }
    }

    pub fn get_max_ports() -> u32 {
        unsafe { pjsua_sys::pjsua_conf_get_max_ports() }
    }

    pub fn get_active_ports() -> u32 {
        unsafe { pjsua_sys::pjsua_conf_get_active_ports() }
    }

    pub fn enum_conf_ports(id: &mut [i32; pjsua_sys::PJSUA_MAX_CONF_PORTS as usize], count: &mut u32) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_enum_conf_ports( id.as_mut_ptr(), count as *mut _))
        }
    }

    pub fn get_port_info (port_id: i32, info: &mut UAConfPortInfo) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_conf_get_port_info( port_id, info as *mut _ ))
        }
    }

    pub fn add_port(port: *mut pjmedia_port, p_id: Option<&mut i32>) -> Result<(), i32> {
    
        let p_id = match p_id {
            Some(value) => value as *mut _,
            None => ptr::null_mut()
        };

    
        unsafe {
            let pool = pool_create("tmp-pool");
    
            let result = pjsua_sys::pjsua_conf_add_port(pool, port, p_id);
    
            pool_release(pool);
    
            if result == PJ_SUCCESS as i32 {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn remove_port (port_id: i32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_conf_remove_port(port_id)) }
    }

    pub fn connect(source: i32, sink: i32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_conf_connect(source, sink)) }
    }

    pub fn connect2 (source: i32, sink: i32, prm: &mut pjsua_conf_connect_param) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_conf_connect2( source, sink, prm as *const _ ))
        }
    }

    pub fn disconnect(source: i32, sink: i32) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_conf_disconnect(source, sink))
        }
    }

    pub fn adjust_tx_level (slot: i32, level: f32) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_conf_adjust_tx_level(slot, level))
        }
    }

    pub fn adjust_rx_level (slot: i32, level: f32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_conf_adjust_rx_level(slot, level)) }
    }

    pub fn get_signal_level (slot: i32, tx_level: &mut u32, rx_level: &mut u32) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_conf_get_signal_level (slot, tx_level as *mut _, rx_level as *mut _))
        }
    }

}


pub struct UAPlayer { id: i32 }

impl From<i32> for UAPlayer {
    fn from(id: i32) -> Self {
        Self { id }
    }
}

impl UAPlayer {

    // TODO: set the new function
    // overiding player_create
    pub fn new(filename: PathBuf, playloop: bool) -> Self {
        UAPlayer::player_create(
            filename,
            boolean_to_pjbool(!playloop) as u32
        ).unwrap()
    }


    pub fn player_create(filename: PathBuf, options: u32) -> Result<UAPlayer, i32> {
        unsafe {
            let filename = &mut pj_str_t::from_string(filename.to_str().unwrap().to_string()) as *const _;
            let mut p_id = -1_i32;
            let status = pjsua_sys::pjsua_player_create( filename, options, &mut p_id as *mut _);
            match utils::check_status(status) {
                Ok(()) => { return Ok(UAPlayer::from(p_id)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    // i32 	pjsua_playlist_create (const pj_str_t file_names[], unsigned file_count, const pj_str_t *label,
    // unsigned options, pjsua_player_id *p_id)

    pub fn get_conf_port(&self) -> i32 {
        unsafe { pjsua_sys::pjsua_player_get_conf_port(self.id) }
    }

    pub fn get_port(&self) -> Result<MediaPort, i32> {
        unsafe {
            let mut p_port = MediaPort::new();
            let status = pjsua_sys::pjsua_player_get_port(self.id, &mut (&mut p_port as *mut _) as *mut _);
            match utils::check_status(status) {
                Ok(()) => { return Ok(p_port) },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn get_info(&self) -> Result<MediaWavPlayerInfo, i32> {
        unsafe {
            let mut info: MediaWavPlayerInfo = std::mem::zeroed();
            let status = pjsua_sys::pjsua_player_get_info(self.id, &mut info as *mut _);
            match utils::check_status(status) {
                Ok(()) => { return Ok(info); },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn get_pos(&self) -> i64 {
        unsafe { pjsua_sys::pjsua_player_get_pos(self.id) }
    }

    pub fn set_pos(&self, samples: u32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_player_set_pos(self.id, samples)) }
    }

    pub fn player_destroy (self) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_player_destroy(self.id)) }
    }

}


pub struct UAEchoCancelar {}

impl Default for UAEchoCancelar {
    fn default() -> Self {
        Self {}
    }
}

impl UAEchoCancelar {

    pub fn set_ec(&self, tail_ms: u32, options: u32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_set_ec(tail_ms, options)) }
    }

    pub fn get_ec_tail(&self, p_tail_ms: &mut u32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_get_ec_tail(p_tail_ms)) }
    }

    pub fn get_ec_stat(&self, p_stat: &mut pjmedia_echo_stat) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_get_ec_stat( p_stat as *mut _ )) }
    }

}

pub struct UASound {  }

impl Default for UASound {
    fn default() -> Self {
        Self {}
    }
}

impl UASound {

    pub fn snd_is_active(&self) -> bool {
        unsafe { utils::check_boolean(pjsua_sys::pjsua_snd_is_active()) }
    }

    pub fn set_snd_dev(&self, capture_dev: i32, playback_dev: i32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_set_snd_dev(capture_dev, playback_dev)) }
    }

    pub fn get_snd_dev(&self, capture_dev: &mut i32, playback_dev: &mut i32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_get_snd_dev( capture_dev as *mut _, playback_dev as *mut _ )) }
    }

    // TODO: fix with Result<pjsua_snd_dev_param, i32>
    pub fn snd_dev_param_default (prm: &mut pjsua_snd_dev_param) {
        unsafe { pjsua_sys::pjsua_snd_dev_param_default(prm as *mut _); }
    }

    // current stable api
    // TODO: fix with Result<Vec<pjmedia_aud_dev_info>>
    pub fn enum_aud_devs(info: &mut [pjmedia_aud_dev_info; 256], count: &mut u32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_enum_aud_devs( info.as_mut_ptr(), count as *mut _)) }
    }

    // old api
    // TODO: fix with Result<Vec<pjmedia_snd_dev_info>>
    pub fn enum_snd_devs(info: &mut [pjmedia_snd_dev_info; 256], count: &mut u32) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_enum_snd_devs( info.as_mut_ptr(), count as *mut _))
        }
    }

    pub fn set_snd_dev2(snd_param: &mut pjsua_snd_dev_param) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_set_snd_dev2( snd_param as *mut _)) }
    }
    
    pub fn set_null_snd_dev() -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_set_null_snd_dev()) }
    }
    
    pub fn set_no_snd_dev() -> *mut pjmedia_port {
        unsafe { pjsua_sys::pjsua_set_no_snd_dev() }
    }

}

pub struct UAExtSound { }

impl UAExtSound {

    // TODO check this create and destroy
    pub fn ext_snd_dev_create(param: &mut pjmedia_snd_port_param, p_snd: &mut pjsua_ext_snd_dev) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_ext_snd_dev_create( param as *mut _, &mut (p_snd as *mut _) as *mut _ ))
        }
    }

    pub fn ext_snd_dev_destroy(snd: &mut pjsua_ext_snd_dev) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_ext_snd_dev_destroy(snd as *mut _)) }
    }

    pub fn ext_snd_dev_get_snd_port(snd: &mut pjsua_ext_snd_dev) -> *mut pjmedia_snd_port {
        unsafe { pjsua_sys::pjsua_ext_snd_dev_get_snd_port( snd as *mut _) }
    }

    pub fn ext_snd_dev_get_conf_port(snd: &mut pjsua_ext_snd_dev) -> i32 {
        unsafe { pjsua_sys::pjsua_ext_snd_dev_get_conf_port( snd as *mut _ ) }
    }

}

pub struct UACodecManager {}

impl Default for UACodecManager {
    fn default() -> Self {
        Self {}
    }
}


impl UACodecManager {

    pub fn enum_codecs(id: &mut [UACodecInfo; 32], count: &mut u32) -> Result<Vec<UACodecInfo>, i32> {
        unsafe {
            let mut id: [UACodecInfo; 32] = std::mem::zeroed();
            let mut count = 0_u32;
            let status = pjsua_sys::pjsua_enum_codecs( id.as_mut_ptr(),  &mut count as *mut _);
            match utils::check_status(status) {
                Ok(()) => {
                    let vec = Vec::<UACodecInfo>::new();

                    for i in 0..count as usize {
                        todo!();
                        // vec.push(id[i]);
                    }

                    return Ok(vec);
                },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn set_priority(codec_id: String, priority: u8) -> Result<(), i32> {
        let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;
        unsafe { utils::check_status(pjsua_sys::pjsua_codec_set_priority( codec_id, priority)) }
    }

    pub fn get_param(codec_id: String, param: &mut pjmedia_codec_param) -> Result<(), i32> {

        let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;

        unsafe {
            utils::check_status(pjsua_sys::pjsua_codec_get_param( codec_id, param as *mut _))
        }
    }

    pub fn set_param(codec_id: String, param: &mut pjmedia_codec_param) -> Result<(), i32> {
        let codec_id: *const pj_str_t = &mut pj_str_t::from_string(codec_id) as *const _;
        unsafe {
            utils::check_status(pjsua_sys::pjsua_codec_set_param( codec_id, param as *const _ ))
        }
    }

}

// pjsua sound and media device function helper
pub fn media_config_default(cfg: &mut UAMediaConfig) {
    unsafe { pjsua_sys::pjsua_media_config_default(cfg as *mut _); }
}
// skiped function

// i32 	pjsua_recorder_create (const pj_str_t *filename, unsigned enc_type, void *enc_param, pj_ssize_t max_size, unsigned options, pjsua_recorder_id *p_id)
// pjsua_conf_port_id 	pjsua_recorder_get_conf_port (pjsua_recorder_id id)
// i32 	pjsua_recorder_get_port (pjsua_recorder_id id, pjmedia_port **p_port)
// i32 	pjsua_recorder_destroy (pjsua_recorder_id id)



// skiped function for detailed audio dev setting
// i32 	pjsua_snd_set_setting (pjmedia_aud_dev_cap cap, const void *pval, pj_bool_t keep)
// i32 	pjsua_snd_get_setting (pjmedia_aud_dev_cap cap, void *pval)

