#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]

// default
use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsip_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;


use super::pjdefault::AutoCreate;
use super::pjlib::PjTimerEntry;
use super::pjsip::PjsipModuleCallback;
use super::pjsua::PjsuaCallback;

use super::pjlib::*;
use super::pjmedia::*;
use super::pjsip::*;
use super::pjsua::*;

use std::ffi::CString;
use std::ffi::CStr;
use std::fmt::format;
use std::mem;
use std::ops::Drop;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

pub struct SIPAccount {
    id: i32,
    ctx: pjsua_acc_config,
}

impl SIPAccount {
    pub fn new(rtp_config: pjsua_transport_config) -> SIPAccount {
        let mut acc = SIPAccount {
            id: -1,
            ctx: pjsua_acc_config::new(),
        };

        acc.ctx.rtp_cfg = rtp_config;
        acc.ctx.reg_retry_interval = 300;
        acc.ctx.reg_first_retry_interval = 60;

        unsafe {
            let mut status = pjsua_acc_add(
                &mut acc.ctx as *const _,
                PJ_TRUE as pj_bool_t,
                &mut acc.id as *mut _,
            );

            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPAccount");
            }

            status = pjsua_acc_set_online_status(
                pjsua_acc_get_default(),
                PJ_TRUE as pj_bool_t,
            );
            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPAccount");
            }
        }
        assert_ne!(acc.id, -1);

        acc
    }
}

// true interface for managing accounts
pub struct SIPAccounts {
    acc_list: Vec<SIPAccount>
}

impl SIPAccounts {

  pub fn new() -> SIPAccounts {
        SIPAccounts {
            acc_list: Vec::<SIPAccount>::new(),
        }
    }

    pub fn add() {}

    pub fn delete() {}

    pub fn modify() {}

    pub fn register() {}

    pub fn unregister() {}

    pub fn next() {}

    pub fn prev() {}

    pub fn show() {}
}

pub struct SIPBuddy {
    id: i32,
    ctx: pjsua_buddy_config,
}

impl SIPBuddy {
    pub fn new() -> SIPBuddy {
        SIPBuddy {
            id: -1,
            ctx: pjsua_buddy_config::new(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let status = pjsua_buddy_add(&mut self.ctx as *const _, &mut self.id as *mut _);

            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPBuddy");
            }

            assert_ne!(self.id, -1);
        }
    }
}

pub struct SIPPresence {
    buddy_list: Vec<SIPBuddy>,
}

impl SIPPresence {
    pub fn new() -> SIPPresence {
        SIPPresence {
            buddy_list: Vec::<SIPBuddy>::new(),
        }
    }

    pub fn add_buddy() {}

    pub fn delete() {}

    pub fn send_im() {}

    pub fn subscribe() {}

    pub fn unsubscribe() {}

    pub fn toggle_state() {}

    pub fn text() {}

    pub fn list() {}
}

pub struct SIPIMessages {}

pub struct SIPWavPlayer {
    id: pjsua_player_id,
    files: String,
    play_opt: u32,
    play_hangup: bool,
    play_timer: pj_timer_entry,
    port: *mut pjmedia_port,
}

trait SIPWavPlayerDone {
    unsafe extern "C" fn eof(port: *mut pjmedia_port, usr_data: *mut c_void);
}

// wav player
impl SIPWavPlayer {
    pub fn new(file_path: String, play_options: u32, auto_play_hangup: bool) -> SIPWavPlayer {
        let mut player = SIPWavPlayer {
            id: -1,
            files: file_path,
            play_opt: play_options,
            play_hangup: auto_play_hangup,
            play_timer: pj_timer_entry::new(),
            port: ptr::null_mut(),
        };

        player.play_opt |= player.play_opt;
        unsafe {
            let mut files_str = pj_str(
                CString::new(player.files.clone())
                    .expect("error")
                    .into_raw(),
            );
            pjsua_player_create(
                &mut files_str as *const _,
                player.play_opt,
                &mut player.id as *mut _,
            );

            let conf_port = pjsua_player_get_conf_port(player.id);
            pjsua_player_get_port(conf_port, player.port as *mut _);

            if player.play_hangup {
                let status = pjmedia_wav_player_set_eof_cb2(
                    player.port,
                    ptr::null_mut(),
                    Some(SIPWavPlayer::eof),
                );

                if status != PJ_SUCCESS as i32 {
                    panic!("Panic set pjmedia_wav_player_set_eof_cb2");
                }

                pj_timer_entry_init(
                    player.port as *mut _,
                    0,
                    ptr::null_mut(),
                    Some(SIPWavPlayer::pj_timer_heap_callback),
                );
            }
        }

        player
    }

    pub fn get_conf_port(&self) -> i32 {
        unsafe { pjsua_player_get_conf_port(self.id) }
    }
}

impl SIPWavPlayerDone for SIPWavPlayer {
    unsafe extern "C" fn eof(port: *mut pjmedia_port, usr_data: *mut c_void) {
        println!("");
    }
}

impl PjTimerEntry for SIPWavPlayer {
    unsafe extern "C" fn pj_timer_heap_callback(
        timer_heap: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
    ) {
    }
}

impl Drop for SIPWavPlayer {
    fn drop(&mut self) {
        // destroy player
        // TODO event
        unsafe {
            pjsua_player_destroy(self.id);
        }
    }
}

// wav recorder for log
pub struct SIPWavRecorder {
    id: i32,
    files: String,
    port: *mut pjmedia_port,
}

impl SIPWavRecorder {
    pub fn new() -> SIPWavRecorder {
        SIPWavRecorder {
            id: -1,
            files: String::from("rec.wav"),
            port: ptr::null_mut(),
        }
    }

    pub fn get_conf_port(&self) -> pjsua_conf_port_id {
        unsafe { pjsua_recorder_get_conf_port(self.id) }
    }
}

impl Drop for SIPWavRecorder {
    fn drop(&mut self) {
        unsafe {
            pjsua_recorder_destroy(self.id);
        }
    }
}

// Optional
#[derive(Clone, Copy)]
pub struct SIPTones {
    slot: i32,
    tones: pjmedia_tone_desc,
    port: *mut pjmedia_port,
}

impl SIPTones {
    pub fn new() -> SIPTones {
        SIPTones {
            slot: -1,
            tones: pjmedia_tone_desc::new(),
            port: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, pool: *mut pj_pool_t, freq1: u16, freq2: u16) {
        unsafe {
            assert_ne!(pool.is_null(), true);

            let mut slabel = pj_str(
                CString::new(format!("tone-{}-{}", freq1, freq2))
                    .expect("error")
                    .into_raw(),
            );

            let mut status = pjmedia_tonegen_create2(
                pool,
                &mut slabel as *const _,
                8000,
                1,
                160,
                16,
                PJMEDIA_TONEGEN_LOOP,
                &mut self.port as *mut _,
            );

            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPTones");
            }

            status = pjsua_conf_add_port(pool, self.port, &mut self.slot as *mut _);

            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPTones");
            }

            status = pjmedia_tonegen_play(self.port, 1, &mut self.tones as *mut _, 0);
            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPTones");
            }

            assert_ne!(self.port.is_null(), true);
            assert_ne!(self.slot, -1);

            // println!(
            //     "SIPTones init slot {}, freq {} and {}",
            //     self.slot, freq1, freq2
            // );
        }
    }
}

//  Ringback tone
pub struct SIPRingback {
    tones: pjmedia_tone_desc,
    slot: i32,
    port: *mut pjmedia_port,
}

impl SIPRingback {
    pub fn new() -> SIPRingback {
        SIPRingback {
            tones: pjmedia_tone_desc::new(),
            slot: -1,
            port: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, pool: *mut pj_pool_t, media_config: pjsua_media_config) {
        unsafe {
            let samples_per_frame = media_config.audio_frame_ptime
                * media_config.clock_rate
                * media_config.channel_count
                / 1000;

            self.tones.freq1 = 440;
            self.tones.freq2 = 480;
            self.tones.on_msec = 2000;
            self.tones.off_msec = 4000;

            let mut name = pj_str(CString::new("ringback").expect("error").into_raw());
            let mut status = pjmedia_tonegen_create2(
                pool,
                &mut name as *const _,
                media_config.clock_rate,
                media_config.channel_count,
                samples_per_frame,
                16,
                PJMEDIA_TONEGEN_LOOP,
                &mut self.port as *mut _,
            );

            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPRingback");
            }

            status = pjsua_conf_add_port(pool, self.port, &mut self.slot as *mut _);
            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPRingback");
            }

            status = pjmedia_tonegen_play(
                self.port,
                1,
                &mut self.tones as *mut _,
                PJMEDIA_TONEGEN_LOOP,
            );

            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPRingback");
            }

            assert_ne!(self.port.is_null(), true);
            assert_ne!(self.slot, -1);
            // println!("SIPRingback init with slot {}", self.slot);
        }
    }
}

impl Drop for SIPRingback {
    fn drop(&mut self) {
        unsafe {
            pjmedia_tonegen_stop(self.port);
        }
    }
}

// this tone gen will alert on incoming call
pub struct SIPRingtone {
    tones: [pjmedia_tone_desc; 3],
    slot: i32,
    port: *mut pjmedia_port,
}

impl SIPRingtone {
    pub fn new() -> SIPRingtone {
        SIPRingtone {
            tones: [pjmedia_tone_desc::new(); 3],
            slot: -1,
            port: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, pool: *mut pj_pool_t, media_config: pjsua_media_config) {
        unsafe {
            assert_ne!(pool.is_null(), true);

            let samples_per_frame = media_config.audio_frame_ptime
                * media_config.clock_rate
                * media_config.channel_count
                / 1000;

            for tone in self.tones.iter_mut() {
                tone.freq1 = 800;
                tone.freq2 = 640;
                tone.on_msec = 200;
                tone.off_msec = 100;
            }

            self.tones[2].off_msec = 3000;

            let mut name = pj_str(CString::new("ringtone").expect("error").into_raw());
            let mut status = pjmedia_tonegen_create2(
                pool,
                &mut name as *const _,
                media_config.clock_rate,
                media_config.channel_count,
                samples_per_frame,
                16,
                PJMEDIA_TONEGEN_LOOP,
                &mut self.port as *mut _,
            );

            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPRingtone");
            }

            status = pjsua_conf_add_port(pool, self.port, &mut self.slot as *mut _);
            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPRingtone");
            }

            status = pjmedia_tonegen_play(
                self.port,
                3,
                self.tones.as_ptr() as *mut pjmedia_tone_desc,
                PJMEDIA_TONEGEN_LOOP,
            );

            if status != PJ_SUCCESS as i32 {
                panic!("cant init SIPRingtone");
            }

            assert_ne!(self.port.is_null(), true);
            assert_ne!(self.slot, -1);
            println!("SIPRingtone init with slot {}", self.slot);
        }
    }
}

impl Drop for SIPRingtone {
    fn drop(&mut self) {
        unsafe {
            pjmedia_tonegen_stop(self.port);
        }
    }
}

// Media Implementation
pub struct SIPMedia {
    ctx: pjsua_media_config,
    capture_dev: i32,
    playback_dev: i32,
    input_level: i32,
    output_level: i32
}

impl SIPMedia {

    pub fn new() -> SIPMedia {
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
        unsafe {
            let status = pjsua_set_snd_dev(self.capture_dev, self.playback_dev);
            if status != PJ_SUCCESS as pj_status_t {
                panic!("cant set audio device");
            }
        }
    }

    pub fn get_input_device_list(&self) -> Vec<String> {
        unsafe{
            let dev_count = pjmedia_aud_dev_count();
            let mut result: Vec<String> = Vec::new();

            for idx in 0..dev_count {
                let mut info = pjmedia_aud_dev_info::new();

                let status = pjmedia_aud_dev_get_info(idx as i32,
                    &mut info as *mut _);
                if status != PJ_SUCCESS as pj_status_t {
                    panic!("can't enumerate input audio device");
                }

                let dev_name = format!("{} (in:{}, out:{})",
                    CStr::from_ptr(info.name.as_ptr()).to_owned().into_string().expect("error"),
                    info.input_count, info.output_count);

                result.push(dev_name);
            }

            result
        }
    }

    pub fn get_output_device_list(&self) -> Vec<String> {
        unsafe{
            let dev_count = pjmedia_aud_dev_count();
            let mut result: Vec<String> = Vec::new();

            for idx in 0..dev_count {
                let mut info: pjmedia_aud_dev_info = pjmedia_aud_dev_info::new();

                let status = pjmedia_aud_dev_get_info(idx as i32,
                    &mut info as *mut _);
                if status != PJ_SUCCESS as pj_status_t {
                    panic!("can't enumerate output audio device");
                }

                let dev_name = format!("{} (in:{},out:{})",
                    CStr::from_ptr(info.name.as_ptr()).to_owned().into_string().expect("error"),
                    info.input_count, info.output_count);

                result.push(dev_name);
            }

            result
        }

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
        unsafe {
            pjsua_conf_adjust_rx_level(0, (self.input_level as f32 / 100.0) as f32);
        }
    }

    pub fn set_output_level(&mut self, value: i32) {
        self.output_level = value;
        unsafe {
            pjsua_conf_adjust_tx_level(0, (self.output_level as f32 / 100.0) as f32);
        }
    }

    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        // transmit/receive signal level
        let mut tx_l: c_uint = 0;
        let mut tx_r: c_uint = 0;
        let mut rx_l: c_uint = 0;
        let mut rx_r: c_uint = 0;

        unsafe {
            pjsua_conf_get_msignal_level(0,
                &mut tx_l as *mut _,
                &mut tx_r as *mut _,
                &mut rx_l as *mut _,
                &mut rx_r as *mut _
            );
        }

        (tx_l, tx_r, rx_l, rx_r)
    }
}

// Transport wrapper
pub struct SIPTransport {
    id: i32,
    acc_id: i32,
}

impl SIPTransport {
    pub fn new() -> SIPTransport {
        SIPTransport { id: -1, acc_id: -1 }
    }

    // start create the transport
    pub fn init(
        &mut self,
        type_: pjsip_transport_type_e,
        config: *const pjsua_transport_config,
        rtp_config: *const pjsua_transport_config,
    ) {
        unsafe {
            assert_ne!(config.is_null(), true);

            let mut status = pjsua_transport_create(type_, config, &mut self.id as &mut _);

            if status != PJ_SUCCESS as i32 {
                panic!("cant create transport.")
            }

            assert_ne!(self.id, -1);

            status = pjsua_acc_add_local(
                self.id,
                PJ_TRUE as i32,
                &mut self.acc_id as *mut _,
            );

            if status != PJ_SUCCESS as i32 {
                panic!("cant init transport");
            }

            assert_ne!(self.acc_id, -1);

            let pool = pjsua_pool_create(
                CString::new("tmp-pool").expect("error").into_raw(),
                1000,
                1000,
            );

            let mut acc_cfg = pjsua_acc_config::new();
            pjsua_acc_get_config(self.acc_id, pool, &mut acc_cfg as *mut _);

            acc_cfg.rtp_cfg = *rtp_config;
            if type_ == pjsip_transport_type_e_PJSIP_TRANSPORT_TCP6
                || type_ == pjsip_transport_type_e_PJSIP_TRANSPORT_UDP6
            {
                acc_cfg.ipv6_media_use = pjsua_ipv6_use_PJSUA_IPV6_ENABLED;
            }

            pjsua_acc_modify(self.acc_id, &mut acc_cfg as *const _);
            pj_pool_release(pool);
            pjsua_acc_set_online_status(pjsua_acc_get_default(), self.acc_id);
        }
    }

    pub fn get_info(&self) -> Result<*const pjsua_transport_info, i32> {
        unsafe {
            let info: *mut pjsua_transport_info = ptr::null_mut();
            let status: pj_status_t = pjsua_transport_get_info(self.id, info);

            if status != PJ_SUCCESS as i32 {
                return Err(status);
            }

            Ok(info)
        }
    }

    pub fn set_enable(&self, enabled: bool) {
        unsafe {
            let mut e = PJ_FALSE;
            if enabled {
                e = PJ_TRUE;
            }

            let status = pjsua_transport_set_enable(self.id, e as i32);

            if status != PJ_SUCCESS as i32 {
                panic!("cant set enable transport");
            }
        }
    }
}

impl Drop for SIPTransport {
    fn drop(&mut self) {
        unsafe {
            pjsua_transport_close(self.id, PJ_TRUE as i32);
        }
    }
}

pub struct SIPTransports {
    transport_list: Vec<SIPTransport>,
    udp_cfg: pjsua_transport_config,
    rtp_cfg: pjsua_transport_config,
}

impl SIPTransports {
    pub fn new() -> SIPTransports {
        let mut sip_transports = SIPTransports {
            transport_list: Vec::<SIPTransport>::new(),
            udp_cfg: pjsua_transport_config::new(),
            rtp_cfg: pjsua_transport_config::new(),
        };

        sip_transports.udp_cfg.port = 5060;
        sip_transports.rtp_cfg.port = 4000;

        sip_transports
    }

    pub fn add(&mut self, transport_type: u32) {
        let mut transport = SIPTransport::new();
        transport.init(
            transport_type,
            &mut self.udp_cfg as *const _,
            &mut self.rtp_cfg as *const _,
        );
        self.transport_list.push(transport);
    }

    pub fn delete(&mut self, transport_id: i32) {
        // TODO
    }

    pub fn set_ca_list_file(&mut self, file_path: &str) {
        unsafe {
            self.udp_cfg.tls_setting.ca_list_file =
                pj_str(CString::new(file_path).expect("error").into_raw());
        }
    }

    pub fn set_cert_file(&mut self, file_path: &str) {
        unsafe {
            self.udp_cfg.tls_setting.cert_file =
                pj_str(CString::new(file_path).expect("error").into_raw());
        }
    }

    pub fn set_privkey_file(&mut self, file_path: &str) {
        unsafe {
            self.udp_cfg.tls_setting.privkey_file =
                pj_str(CString::new(file_path).expect("error").into_raw());
        }
    }

    pub fn set_password(&mut self, password: &str) {
        unsafe {
            self.udp_cfg.tls_setting.password =
                pj_str(CString::new(password).expect("error").into_raw());
        }
    }

    pub fn set_tls_verify_server(&mut self, value: bool) {
        self.udp_cfg.tls_setting.verify_server = if value {
            PJ_TRUE as pj_bool_t
        } else {
            PJ_FALSE as pj_bool_t
        }
    }

    pub fn set_tls_verify_client(&mut self, value: bool) {
        let val = if value {
            PJ_TRUE as pj_bool_t
        } else {
            PJ_FALSE as pj_bool_t
        };

        self.udp_cfg.tls_setting.verify_client = val;
        self.udp_cfg.tls_setting.require_client_cert = val;
    }
}




#[derive(Copy, Clone)]
pub struct SIPCall {
    timer: pj_timer_entry,
    ringback_on: bool,
    ring_on: bool
}

impl SIPCall {
    pub fn new() -> SIPCall {
        SIPCall {
            timer: pj_timer_entry::new(),
            ringback_on: false,
            ring_on: false
        }
    }
}

pub struct SIPCalls {
    id_list: [pjsua_call_id; 32],
    call_data: [SIPCall; 32],
    call_opt: pjsua_call_setting,
    ringback_on: bool,
    ring_on: bool,
}

impl SIPCalls {

    pub fn new() -> SIPCalls {
        SIPCalls {
            id_list: [0;32],
            call_data: [SIPCall::new(); 32],
            call_opt: pjsua_call_setting::new(),
            ringback_on: false,
            ring_on: false,
        }
    }

    pub fn set_audio_count(&mut self, value: u32) {
        self.call_opt.aud_cnt = value;
    }
}


impl PjTimerEntry for SIPCalls {

    unsafe extern "C" fn pj_timer_heap_callback(
        timer_heap: *mut pj_timer_heap_t,
        entry: *mut pj_timer_entry,
    ) {
        let call_id: pjsua_call_id = (*entry).id;
        let mut msg_data_: pjsua_msg_data = pjsua_msg_data::new();
        let mut warn: pjsip_generic_string_hdr = pjsip_generic_string_hdr::new();
        let mut hname = pj_str(CString::new("Warning").expect("Error").into_raw());
        let mut hvalue = pj_str(
            CString::new("339 \" Call duration exceeded \"")
                .expect("Error")
                .into_raw(),
        );

        if call_id == PJSUA_INVALID_ID {
            println!("Invalid call ED intimer callback");
        }

        pjsua_msg_data_init(&mut msg_data_ as *mut _);
        pjsip_generic_string_hdr_init2(
            &mut warn as *mut _,
            &mut hname as *mut _,
            &mut hvalue as *mut _,
        );
        pj_list_insert_before(
            (&mut msg_data_.hdr_list as *mut _) as *mut _,
            (&mut warn as *mut _) as *mut _,
        );

        println!(
            "Duration (seconds) has been exceeded for call {}, disconnectiong the call.",
            call_id
        );

        (*entry).id = PJSUA_INVALID_ID;
        pjsua_call_hangup(call_id, 200, ptr::null(), &mut msg_data_ as *mut _);
    }
}

static mut SIP_CORE: Option<SIPCore> = None;
static mut CURRENT_CALL: Option<pjsua_call_id> = None;

pub struct SIPCore {
    pool: *mut pj_pool_t,
    app_config: pjsua_config,
    log_config: pjsua_logging_config,
    media_config: SIPMedia,
    no_udp: bool,
    no_tcp: bool,
    use_ipv6: bool,
    transports: SIPTransports,
    accounts: SIPAccounts,
    presence: SIPPresence,
    calls: SIPCalls,
    tones: Vec<SIPTones>,
    ringback: SIPRingback,
    ringtone: SIPRingtone,
    wav_player: Option<SIPWavPlayer>,
    wav_recorder: Option<SIPWavRecorder>,
    default_handler: pjsip_module,
    redir_op: pjsip_redirect_op,
    // input_level: f32,
    // output_level: f32,
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    auto_play_hangup: bool,
    duration: u32,
    current_call: i32,
    aud_cnt: u32,
    auto_answer: u32,
}

impl SIPCore {

    pub fn new() -> SIPCore {
        // create default data
        unsafe {
            pjsua_create();
        }

        let sip_core = SIPCore {
            pool: ptr::null_mut(),
            app_config: pjsua_config::new(),
            log_config: pjsua_logging_config::new(),
            media_config: SIPMedia::new(),
            no_udp: false,
            no_tcp: false,
            use_ipv6: false,
            transports: SIPTransports::new(),
            accounts: SIPAccounts::new(),
            presence: SIPPresence::new(),
            calls: SIPCalls::new(),
            tones: Vec::new(),
            ringback: SIPRingback::new(),
            ringtone: SIPRingtone::new(),
            wav_player: None,
            wav_recorder: None,
            default_handler: pjsip_module::new(),
            redir_op: PJSIP_REDIRECT_ACCEPT_REPLACE,
            // input_level: 1.0,
            // output_level: 1.0,
            input_dev: PJSUA_INVALID_ID,
            output_dev: PJSUA_INVALID_ID,
            input_latency: 100,
            output_latency: 140,
            auto_play_hangup: false,
            duration: 0,
            current_call: -1,
            aud_cnt: 1,
            auto_answer: 0,
        };

        sip_core
    }

    pub fn start(&mut self) {
        unsafe {
            self.pool = pjsua_pool_create(
                CString::new("ipcodec").expect("error").into_raw(),
                1000,
                1000,
            );

            assert_ne!(self.pool.is_null(), true);

            self.app_config.cb.on_call_state = Some(SIPCore::on_call_state);
            self.app_config.cb.on_stream_destroyed = Some(SIPCore::on_stream_destroyed);
            self.app_config.cb.on_call_media_state = Some(SIPCore::on_call_media_state);
            self.app_config.cb.on_incoming_call = Some(SIPCore::on_incoming_call);
            self.app_config.cb.on_dtmf_digit2 = Some(SIPCore::on_dtmf_digit2);
            self.app_config.cb.on_call_redirected = Some(SIPCore::on_call_redirected);
            self.app_config.cb.on_reg_state = Some(SIPCore::on_reg_state);
            self.app_config.cb.on_incoming_subscribe = Some(SIPCore::on_incoming_subscribe);
            self.app_config.cb.on_buddy_state = Some(SIPCore::on_buddy_state);
            self.app_config.cb.on_buddy_evsub_state = Some(SIPCore::on_buddy_evsub_state);
            self.app_config.cb.on_pager = Some(SIPCore::on_pager);
            self.app_config.cb.on_typing = Some(SIPCore::on_typing);
            self.app_config.cb.on_call_transfer_status = Some(SIPCore::on_call_transfer_status);
            self.app_config.cb.on_call_replaced = Some(SIPCore::on_call_replaced);
            self.app_config.cb.on_nat_detect = Some(SIPCore::on_nat_detect);
            self.app_config.cb.on_mwi_info = Some(SIPCore::on_mwi_info);
            self.app_config.cb.on_transport_state = Some(SIPCore::on_transport_state);
            self.app_config.cb.on_ice_transport_error = Some(SIPCore::on_ice_transport_error);
            self.app_config.cb.on_snd_dev_operation = Some(SIPCore::on_snd_dev_operation);
            self.app_config.cb.on_call_media_event = Some(SIPCore::on_call_media_event);
            self.app_config.cb.on_ip_change_progress = Some(SIPCore::on_ip_change_progress);

            // init pjsua
            pjsua_init(
                &mut self.app_config as *mut _,
                &mut self.log_config as *mut _,
                &mut self.media_config.ctx as *mut _,
            );

            // pjsip endpoint for unhadled error
            self.default_handler.on_rx_request = Some(SIPCore::on_rx_request);
            let status = pjsip_endpt_register_module(
                pjsua_get_pjsip_endpt(),
                &mut self.default_handler as *mut _,
            );

            if status != 0 {
                panic!("cant register module");
            }

            // add optional tones
            for _ in 0..32 {
                let mut tones = SIPTones::new();
                tones.init(self.pool, 440, 480);
                self.tones.push(tones);
            }

            // init ringback
            self.ringback.init(self.pool, self.media_config.ctx);

            // init ringtone
            self.ringtone.init(self.pool, self.media_config.ctx);

            // Initialize UDP Transport
            if !self.no_udp {
                self.transports
                    .add(pjsip_transport_type_e_PJSIP_TRANSPORT_UDP);
                if self.use_ipv6 == true {
                    self.transports
                        .add(pjsip_transport_type_e_PJSIP_TRANSPORT_UDP6);
                }
            }

            // initialize TCP transport
            if !self.no_tcp {
                self.transports
                    .add(pjsip_transport_type_e_PJSIP_TRANSPORT_TCP);
                if self.use_ipv6 {
                    self.transports
                        .add(pjsip_transport_type_e_PJSIP_TRANSPORT_TCP6);
                }
            }

            self.media_config.init();
            self.calls.set_audio_count(self.aud_cnt);

            let status = pjsua_start();
            if status != PJ_SUCCESS as pj_status_t {
                pjsua_perror(CString::new("cant start").expect("error").into_raw() as *const _,
                 CString::new("cant start").expect("error").into_raw() as *const _, status);
            }

        }
    }

    pub fn deinit(&mut self) {
        unsafe {
            pj_pool_safe_release(&mut self.pool as *mut _);
            pjsua_destroy();
        }
    }

    pub fn call(&mut self, call_addr: &str) {
        unsafe{

            let mut msg_data = pjsua_msg_data::new();
            pjsua_msg_data_init(&mut msg_data as *mut _);

            let default_acc = pjsua_acc_get_default();
            println!("default accid : {}", default_acc);
            let mut call_addr = pj_str(CString::new(call_addr).expect("error").into_raw() as *mut _);
            let status = pjsua_call_make_call(
                        default_acc,
                        &mut call_addr as *const _,
                        &mut self.calls.call_opt as *mut _,
                        // ptr::null_mut(),
                        // &mut msg_data as *mut _,
                        // self.current_call as *mut _
                        ptr::null_mut(),
                        ptr::null_mut(),
                        ptr::null_mut()
            );
        }
    }

    pub fn call_account(&self) {

    }

    pub fn get_input_level(&self) -> i32 {
        self.media_config.get_input_level()
    }

    pub fn get_output_level(&self) -> i32 {
        self.media_config.get_output_level()
    }

    pub fn set_input_level(&mut self, value: i32) {
        self.media_config.set_input_level(value);
    }

    pub fn set_output_level(&mut self, value: i32) {
        self.media_config.set_output_level(value);
    }

    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        self.media_config.get_signal_level()
    }

    pub fn ringback_start(&self, call_id: pjsua_call_id) {}

    pub fn ring_stop(&self, call_id: &pjsua_call_id) {}

    pub fn ring_start(&self, call_id: pjsua_call_id) {}

    pub fn find_next_call(&self) {}

    pub fn on_call_generic_media_state(&self, ci: &pjsua_call_info, mi: u32, has_error: &mut bool) {
        // println!("OnCallGenericMediaState");
        let status_name: [&str; 5] = [
            "None",
            "Active",
            "Local hold",
            "Remote hold",
            "Error"
        ];

        unsafe {
            let media_type = CStr::from_ptr(
                pjmedia_type_name(ci.media[mi as usize].type_)
            ).to_str().expect("Error string media type");

            println!("sipua.rs Call {} media {} [type={}], status is {}",
                ci.id,
                mi,
                media_type,
                status_name[ci.media[mi as usize].status as usize]
            );
        }

    }

    pub fn on_call_audio_state(&mut self, ci: &pjsua_call_info, mi: u32, has_error: &mut bool) {

        let media = ci.media[mi as usize];

        if media.status == PJSUA_CALL_MEDIA_ACTIVE ||
        media.status == PJSUA_CALL_MEDIA_REMOTE_HOLD {
            let call_conf_slot: pjsua_conf_port_id;

            unsafe {

                call_conf_slot = media.stream.aud.conf_slot;

                let mut call_ids: [pjsua_call_id; 32] = [-1; 32];
                let mut call_cnt = 32u32;

                pjsua_enum_calls(call_ids.as_mut_ptr(),
                    &mut call_cnt as *mut _);

                for idx in 0..call_cnt as usize {
                    if call_ids[idx] == ci.id { continue; }
                    if  pjsua_call_has_media(call_ids[idx].clone()) == PJ_FALSE as pj_bool_t { continue; }

                    // connect rx
                    pjsua_conf_connect(call_conf_slot,
                        pjsua_call_get_conf_port(call_ids[idx])
                    );

                    // connect tx
                    pjsua_conf_connect(pjsua_call_get_conf_port(call_ids[idx]),
                        call_conf_slot
                    );
                }

                pjsua_conf_connect(call_conf_slot, 0);
                pjsua_conf_connect(0, call_conf_slot);
            }
        }
    }

    pub fn callback_on_call_state(&mut self, call_id: pjsua_call_id, e: *mut pjsip_event) {
        println!("OnCallState");
        unsafe {
            let mut call_info: pjsua_call_info = pjsua_call_info::new();

            let status = pjsua_call_get_info(call_id, &mut call_info as *mut _);
            if status != PJ_SUCCESS as pj_status_t {
                panic!("OnCallState : pjsua_call_get_info fail.");
            }

            let state_message: &str = match call_info.state {
                PJSIP_INV_STATE_NULL => {
                    "Null"
                },
                PJSIP_INV_STATE_CALLING => {

                    "Calling"
                },
                PJSIP_INV_STATE_INCOMING => {
                    "Incoming"
                },
                PJSIP_INV_STATE_EARLY => {
                    "Early"
                },
                PJSIP_INV_STATE_CONNECTING => {
                    "Connecting"
                },
                PJSIP_INV_STATE_CONFIRMED => {
                    "Confirmed"
                },
                PJSIP_INV_STATE_DISCONNECTED => {
                    "Disconnected"
                },
                _ => {
                    "Unknown Error"
                }
            };


            println!("SIP Invite State : {}", state_message);

            // if call_info.state == PJSIP_INV_STATE_DISCONNECTED {
            //     // todo
            //     self.ring_stop(&call_id);
            //     //let cd: *mut SIPCall = &mut self.call_data[call_id as usize] as *mut _;
            //     let endpt = pjsua_get_pjsip_endpt();

            //     // (*cd).timer.id = PJSUA_INVALID_ID;
            //     // pjsip_endpt_cancel_timer(endpt, &mut (*cd).timer as *mut _);

            //     if call_id == self.current_call {
            //         self.find_next_call();
            //     }

            //     println!("Call disconnected.");
            // } else {
            //     if self.duration == 0x7FFFFFFF
            //         && call_info.state == PJSIP_INV_STATE_CONFIRMED
            //     {
            //         // let cd: *mut SIPCall = &mut self.call_data[call_id as usize] as *mut _;
            //         // let endpt = pjsua_get_pjsip_endpt();
            //         // let mut delay: pj_time_val = pj_time_val::new();

            //         // (*cd).timer.id = call_id;
            //         // delay.sec = self.duration as i64;
            //         // delay.msec = 0;
            //         // pjsip_endpt_schedule_timer_dbg(
            //         //     endpt,
            //         //     &mut (*cd).timer as *mut _,
            //         //     &delay as *const _,
            //         //     &mut mem::zeroed() as *mut _,
            //         //     0,
            //         // );
            //     }

            //     if call_info.state == PJSIP_INV_STATE_EARLY {
            //         println!("Call state changed.");
            //     }

            //     if self.current_call == PJSUA_INVALID_ID {
            //         self.current_call = call_id;
            //     }
            // }
        }
    }

    pub fn callback_on_call_media_state(&mut self, call_id: pjsua_call_id) {
        println!("OnCallMediaState");
        unsafe {

            let mut call_info: pjsua_call_info = pjsua_call_info::new();
            let mut has_error = false;
            let mut media_type: &str = "Unknown";

            pjsua_call_get_info(call_id, &mut call_info as *mut _);

            for mi in 0..call_info.media_cnt {

                self.on_call_generic_media_state(&call_info, mi, &mut has_error);

                media_type = match call_info.media[mi as usize].type_ {

                    PJMEDIA_TYPE_NONE => {
                        has_error = true;
                        "None"
                    },
                    PJMEDIA_TYPE_AUDIO => {
                        self.on_call_audio_state(&call_info, mi, &mut has_error);
                        "Audio"
                    },

                    PJMEDIA_TYPE_VIDEO => {
                        has_error = true;
                        "Video"
                    },

                    PJMEDIA_TYPE_APPLICATION => {
                        has_error = true;
                        "Application"
                    },

                    PJMEDIA_TYPE_UNKNOWN => {
                        has_error = true;
                        "Unknown"
                    }
                    _ => {
                        has_error = true;
                        "Unsuported"
                    }
                };
            }

            if has_error {
                let reason = CString::new(format!("Media failed : {}", media_type)).expect("cant create str");
                pjsua_call_hangup(
                    call_id,
                    500,
                    &pj_str(reason.as_ptr() as *mut _) as *const _,
                    ptr::null(),
                );
            }
        }
    }

    pub fn callback_on_incomming_call(
        &mut self,
        acc_id: pjsua_acc_id,
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
    ) {
        unsafe {
            let mut call_info: pjsua_call_info = pjsua_call_info::new();

            pjsua_call_get_info(call_id, &mut call_info as *mut _);

            self.current_call = call_id;

            self.ring_start(call_id);

            let mut opt: pjsua_call_setting = pjsua_call_setting::new();
            pjsua_call_setting_default(&mut opt as *mut _);
            opt.aud_cnt = self.aud_cnt;

            pjsua_call_answer2(
                call_id,
                &opt as *const _,
                self.auto_answer,
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }

    pub fn callback_on_dtmf_digit2(&self, call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        unsafe {
            let mut dtmf: &str = "None";

            match (*info).method {
                pjsua_dtmf_method_PJSUA_DTMF_METHOD_RFC2833 => {
                    dtmf = "RFC2833";
                }
                pjsua_dtmf_method_PJSUA_DTMF_METHOD_SIP_INFO => {
                    dtmf = "SIP INFO";
                }
                _ => println!("Unknown dtmf method"),
            }

            println!("Incomming DTMF on call using method {}", dtmf);
        }
    }

    pub fn callback_on_call_redirected(
        &self,
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        println!("Call {} is being redirected", call_id);
        self.redir_op
    }

    pub fn callback_on_reg_state(&self, acc_id: pjsua_acc_id) {
        println!("reg_state {}", acc_id);
    }

    pub fn callback_on_incoming_subscribe(
        &self,
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data)
    {
        // Todo
    }

    pub fn callback_on_buddy_state(&self, buddy_id: pjsua_buddy_id) {
        unsafe {
            let mut info: pjsua_buddy_info = pjsua_buddy_info::new();
            pjsua_buddy_get_info(buddy_id, &mut info as *mut _);
        }
    }

    pub fn callback_on_buddy_evsub_state(
        &self,
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
        unsafe {
            let rdata = (*event).body.tsx_state.src.rdata;
            // let astr = pjsip_rx_data_get_info(rdata);
            println!("Buddy subscription state");
        }
    }

    pub fn callback_on_pager(
        &self,
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
    ) {
        println!("OnPager");
    }

    pub fn callback_on_typing(
        &self,
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
        println!("IM indication.");
    }

    pub fn callback_on_call_transfer_status(
        &self,
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
        unsafe {
            println!("Call {} transfer status={}", call_id, st_code);
            if st_code / 100 == 2 {
                pjsua_call_hangup(
                    call_id,
                    pjsip_status_code_PJSIP_SC_GONE,
                    ptr::null() as *const _,
                    ptr::null() as *const _,
                );
                *p_cont = PJ_FALSE as pj_bool_t;
            }
        }
    }

    pub fn callback_on_call_replaced(
        &self,
        old_call_id: pjsua_call_id,
        new_call_id: pjsua_call_id,
    ) {
        unsafe {
            let mut old_ci: pjsua_call_info = pjsua_call_info::new();
            let mut new_ci: pjsua_call_info = pjsua_call_info::new();

            pjsua_call_get_info(old_call_id, &mut old_ci as *mut _);
            pjsua_call_get_info(new_call_id, &mut new_ci as *mut _);

            println!(
                "Call {} is being replaced by call {}",
                old_call_id, new_call_id
            );
        }
    }

    pub fn callback_on_nat_detect(&self, res: *const pj_stun_nat_detect_result) {
        unsafe {
            if (*res).status != PJ_SUCCESS as pj_status_t {
                println!("NAT detection failed.");
            } else {
                println!("NAT detected");
            }
        }
    }

    pub fn callback_on_mwi_info(&self, acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {
        unsafe {
            println!("Received MWI for acc_id {}", acc_id);
            let ctype = (*(*mwi_info).rdata).msg_info.ctype;

            if !ctype.is_null() {
                println!(
                    "Content-Type: {} {}/ {} {}",
                    (*ctype).media.type_.slen,
                    CString::from_raw((*ctype).media.type_.ptr)
                        .into_string()
                        .expect("error"),
                    (*ctype).media.subtype.slen,
                    CString::from_raw((*ctype).media.subtype.ptr)
                        .into_string()
                        .expect("error")
                );
            }
        }
    }

    pub fn callback_on_transport_state(
        &self,
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
        println!("OnTransportState");
        // unsafe {
        //     match state {
        //         pjsip_transport_state_PJSIP_TP_STATE_CONNECTED => {
        //             println!(
        //                 "SIP {} transport is connected to {}:{}",
        //                 CString::from_raw((*tp).type_name)
        //                     .into_string()
        //                     .expect("error"),
        //                 CString::from_raw((*tp).remote_name.host.ptr)
        //                     .into_string()
        //                     .expect("0.0.0.0"),
        //                 (*tp).remote_name.port
        //             );
        //         }
        //         pjsip_transport_state_PJSIP_TP_STATE_DISCONNECTED => {
        //             println!(
        //                 "SIP {} transport is disconnected form {}:{}",
        //                 CString::from_raw((*tp).type_name)
        //                     .into_string()
        //                     .expect("error"),
        //                 CString::from_raw((*tp).remote_name.host.ptr)
        //                     .into_string()
        //                     .expect("0.0.0.0"),
        //                 (*tp).remote_name.port
        //             );
        //         }
        //         _ => println!("check c code"),
        //     }
        // }
    }

    pub fn callback_on_ice_transport_error(
        &self,
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
        println!("ICE keep alive failure for transport {}", index);
    }

    pub fn callback_on_snd_dev_operation(&self, operation: c_int) -> pj_status_t {
        println!("OnSndDevOperation");
        unsafe {
            let mut cap_dev = -1;
            let mut play_dev = -1;
            let op: String;

            if operation > 0 {
                op = String::from("ON");
            } else {
                op = String::from("OFF");
            }

            pjsua_get_snd_dev(&mut cap_dev as *mut _, &mut play_dev as *mut _);
            println!(
                "Turning sound device input {} output {} : {}",
                cap_dev, play_dev, op
            );
        }
        PJ_SUCCESS as pj_status_t
    }

    pub fn callback_on_call_media_event(
        &self,
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event)
    {
        // unsafe {
        // let mut event_name: [c_char; 5] = [0; 5];

        // let fourcc_name = pjmedia_fourcc_name((*event).type_, &mut event_name as *mut _);

        println!("Event {}", "skip");
        // }
    }

    pub fn callback_on_ip_change_progress(
        &self,
        op: pjsua_ip_change_op,
        status: pj_status_t,
        info: *const pjsua_ip_change_op_info,
    ) {
        println!("OnIpChangeProgress");
        unsafe {
            let mut acc_info: pjsua_acc_info = pjsua_acc_info::new();
            let mut tp_info: pjsua_transport_info = pjsua_transport_info::new();

            if status == PJ_SUCCESS as pj_status_t {
                match op {
                    PJSUA_IP_CHANGE_OP_RESTART_LIS => {
                        pjsua_transport_get_info(
                            (*info).lis_restart.transport_id,
                            &mut tp_info as *mut _,
                        );
                        println!("restart transport.");
                    }
                    PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("transport shutdown for account.");
                    }
                    PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("update contact for account.");
                    }
                    PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("hangup call for account.");
                    }
                    PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("reinvite call for account.");
                    }
                    PJSUA_IP_CHANGE_OP_COMPLETED => {
                        println!("done");
                    }

                    _ => println!("warn validate c code."),
                }
            } else {
                println!("IP change progress fail.");
            }
        }
    }
}

impl PjsuaCallback for SIPCore {
    // Call status event
    unsafe extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {
        // call info data
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_state(call_id, e);
            }
            _ => panic!("Panic OnCallState"),
        }
    }

    // Stream Destroyed;
    unsafe extern "C" fn on_stream_destroyed(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
    ) {
        println!("Call stream destroyed");
    }

    // Call media satate
    unsafe extern "C" fn on_call_media_state(call_id: pjsua_call_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_media_state(call_id);
            }
            _ => panic!("Panic OnCallMediaState"),
        }
    }

    // DTMF Digit2
    unsafe extern "C" fn on_dtmf_digit2(call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        // todo here
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_dtmf_digit2(call_id, info);
            }
            _ => panic!("Panic OnDtmfDigit2"),
        }
    }

    // Call Redirected
    unsafe extern "C" fn on_call_redirected(
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        let result: pjsip_redirect_op;
        match SIP_CORE {
            Some(ref mut sipcore) => sipcore.callback_on_call_redirected(call_id, target, e),
            _ => panic!("Panic OnCallRedirected"),
        }
    }

    // REG state
    unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_reg_state(acc_id);
            }
            _ => panic!("Panic OnRegState"),
        }
    }

    // Incomming Subscribe
    unsafe extern "C" fn on_incoming_subscribe(
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_incoming_subscribe(
                    acc_id, srv_pres, buddy_id, from, rdata, code, reason, msg_data,
                );
            }
            _ => panic!("Panic OnIncomingSubscribe"),
        }
    }

    // Buddy State
    unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_buddy_state(buddy_id);
            }
            _ => panic!("Panic OnBuddyState"),
        }
    }

    // Buddy evsub state
    unsafe extern "C" fn on_buddy_evsub_state(
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_buddy_evsub_state(buddy_id, sub, event);
            }
            _ => panic!("Panic OnBuddyEvsubState"),
        }
    }

    // Pager
    unsafe extern "C" fn on_pager(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_pager(call_id, from, to, contact, mime_type, body);
            }
            _ => panic!("Panic OnPager"),
        }
    }

    // Typing event
    unsafe extern "C" fn on_typing(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_typing(call_id, from, to, contact, is_typing);
            }
            _ => panic!("Panic OnTyping"),
        }
    }

    // Call transfer status
    unsafe extern "C" fn on_call_transfer_status(
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_transfer_status(call_id, st_code, st_text, final_, p_cont);
            }
            _ => panic!("Panic OnCallTransferStatus"),
        }
    }

    // Call replaced
    unsafe extern "C" fn on_call_replaced(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_replaced(old_call_id, new_call_id);
            }
            _ => panic!("Panic OnCallReplaced"),
        }
    }

    // NAT detect
    unsafe extern "C" fn on_nat_detect(res: *const pj_stun_nat_detect_result) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_nat_detect(res);
            }
            _ => panic!("Panic OnNatDetect"),
        }
    }

    // MWI info
    unsafe extern "C" fn on_mwi_info(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_mwi_info(acc_id, mwi_info);
            }
            _ => panic!("Panic OnMwiInfo"),
        }
    }

    // Transport state
    unsafe extern "C" fn on_transport_state(
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_transport_state(tp, state, info);
            }
            _ => panic!("Panic OnTransportState"),
        }
    }

    // ICE transport error
    unsafe extern "C" fn on_ice_transport_error(
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_ice_transport_error(index, op, status, param);
            }
            _ => panic!("Panic OnTransportError"),
        }
    }

    // Sound device operation
    unsafe extern "C" fn on_snd_dev_operation(operation: c_int) -> pj_status_t {
        match SIP_CORE {
            Some(ref mut sipcore) => sipcore.callback_on_snd_dev_operation(operation),
            _ => panic!("Panic OnSndDevOperation"),
        }
    }

    // Call media event
    unsafe extern "C" fn on_call_media_event(
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_media_event(call_id, med_idx, event);
            }
            _ => panic!("Panic OnCallMediaEvent"),
        }
    }

    // IP change progress
    unsafe extern "C" fn on_ip_change_progress(
        op: pjsua_ip_change_op,
        status: pj_status_t,
        info: *const pjsua_ip_change_op_info,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_ip_change_progress(op, status, info);
            }
            _ => panic!("Panic OnIpChangeProgress"),
        }
    }
}


// SIPUserAgent
#[derive(Clone)]
pub struct SIPUserAgent {}

//type SIPConfig = pjsua_config;

pub const PJSUA_INVALID_ID: i32 = -1;

trait SIPUserAgentInternal {
    fn get_config(&mut self) -> &mut pjsua_config;
    fn get_log_config(&mut self) -> &mut pjsua_logging_config;
    fn get_media_config(&mut self) -> &mut pjsua_media_config;
}

impl SIPUserAgent {
    // create sip user sip user agent with default ivalue
    pub fn new() -> SIPUserAgent {
        unsafe {
            SIP_CORE = Some(SIPCore::new());
            CURRENT_CALL = Some(PJSUA_INVALID_ID);
        }

        SIPUserAgent {}
    }

    pub fn start(&self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => sipcore.start(),
                _ => panic!(""),
            }
        }
    }

    pub fn get_output_device_list(&self) -> Vec<String> {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => {
                    sipcore.media_config.get_output_device_list()
                },
                _ => panic!("")
            }
        }
    }

    pub fn get_input_device_list(&self) -> Vec<String> {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore ) => {
                    sipcore.media_config.get_input_device_list()
                },
                _ => panic!("")
            }
        }
    }

    pub fn call(&self, call_addr: &str){
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.call(call_addr);
                },
                _ => panic!("")
            }
        }
    }

    pub fn get_input_level(&self) -> i32 {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.get_input_level()
                },
                _ => { 0 }
            }
        }
    }

    pub fn get_output_level(&self) -> i32 {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.get_output_level()
                },
                _ => { 0 }
            }
        }
    }

    pub fn set_input_level(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_input_level(value);
                },
                _ => panic!("")
            }
        }
    }

    pub fn set_output_level(&self, value: i32) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipua) => {
                    sipua.set_output_level(value);
                },
                _ => panic!("")
            }
        }
    }

    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        unsafe {
            match SIP_CORE {
                Some(ref sipua) => {
                    sipua.get_signal_level()
                },
                _ => (0,0,0,0)
            }
        }
    }

}

impl Drop for SIPUserAgent {
    fn drop(&mut self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => {
                    sipcore.deinit();
                }
                _ => (),
            }
        }
    }
}

fn simple_registrar(rdata: *mut pjsip_rx_data) {
    unsafe {
        let tdata: *const pjsip_tx_data = ptr::null();
        let str_null: *const pj_str_t = ptr::null();
        let status: pj_status_t;
        let mut cnt: c_uint = 0;

        status = pjsip_endpt_create_response(
            pjsua_get_pjsip_endpt(),
            rdata as *const _,
            200,
            str_null as *const _,
            tdata as *mut _,
        );
        if status != PJ_SUCCESS as i32 {
            return;
        }

        let exp: *const pjsip_expires_hdr = pjsip_msg_find_hdr(
            (*rdata).msg_info.msg,
            pjsip_hdr_e_PJSIP_H_EXPIRES,
            ptr::null_mut(),
        ) as *const _;

        let llist: pjsip_hdr = (*(*rdata).msg_info.msg).hdr;
        let mut h: *mut pjsip_hdr = (*(*rdata).msg_info.msg).hdr.next;

        while h != llist.next {
            if (*h as pjsip_hdr).type_ == (pjsip_hdr_e_PJSIP_H_CONTACT as pjsip_hdr_e) {
                let c: *const pjsip_contact_hdr = h as *const pjsip_contact_hdr;
                let mut e: c_uint = (*c).expires;

                if e != 0xffffffff {
                    if !exp.is_null() {
                        e = (*exp).ivalue;
                    } else {
                        e = 3600;
                    }
                }

                if e > 0 {
                    let nc: *mut pjsip_contact_hdr =
                        pjsip_hdr_clone((*tdata).pool, h as *const _) as *mut pjsip_contact_hdr;

                    (*nc).expires = e;
                    pj_list_insert_before((*tdata).msg as *mut _, nc as *mut _);
                    cnt = cnt + 1;
                }
                h = (*h).next;
            }
        }

        // todo review c code for this. it's c clasic problem
        let srv: *mut pjsip_generic_string_hdr =
            pjsip_generic_string_hdr_create((*tdata).pool, str_null, str_null);
        // create server name
        let tmp: CString = CString::new("Server").expect("cant create Server string");
        (*srv).name = pj_str(tmp.as_ptr() as *mut c_char);
        // create add description
        let tmp: CString =
            CString::new("IpCodec simple registrar").expect("cant create simple registrar");
        (*srv).hvalue = pj_str(tmp.as_ptr() as *mut c_char);

        pj_list_insert_before((*tdata).msg as *mut _, srv as *mut _);
        let cb: pjsip_send_callback = None;
        pjsip_endpt_send_response2(
            pjsua_get_pjsip_endpt(),
            rdata,
            tdata as *mut _,
            ptr::null_mut(),
            None,
        );
    }
}

// handle for callback PjsipModule
impl PjsipModuleCallback for SIPCore {

    unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_status_t {
        println!("OnRxRequest");
        // base rx request handle undefined state.
        let tdata: *const pjsip_tx_data = ptr::null();
        let status_code: pjsip_status_code;
        let status: pj_status_t;

        let mut rdata = *rdata;
        let msg = *rdata.msg_info.msg;
        let mut method = msg.line.req.method;
        // let msg_info = method.msg_info;
        if pjsip_method_cmp(&mut method as *const _, &pjsip_ack_method as *const _) == 0 {
            return PJ_TRUE as pj_status_t;
        }

        if pjsip_method_cmp(&mut method as *const _, &pjsip_register_method as *const _) == 0 {
            // call simple registrar pjsip_tx_data
            simple_registrar(&mut rdata as *mut _);
            return PJ_TRUE as pj_status_t;
        }

        if pjsip_method_cmp(&mut method as *const _, &pjsip_notify_method as *const _) == 0 {
            status_code = pjsip_status_code_PJSIP_SC_BAD_REQUEST as pjsip_status_code;
        } else {
            status_code = pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED;
        }

        status = pjsip_endpt_create_response(
            pjsua_get_pjsip_endpt(),
            &mut rdata as *const _,
            status_code as c_int,
            ptr::null() as *const _,
            tdata as *mut *mut _,
        );

        if status != (PJ_SUCCESS as pj_status_t) {
            return PJ_TRUE as pj_status_t;
        }

        if status_code == pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED {
            #[allow(unused_assignments)]
            let mut cap_hdr: *const pjsip_hdr = ptr::null();

            cap_hdr = pjsip_endpt_get_capability(
                pjsua_get_pjsip_endpt(),
                pjsip_hdr_e_PJSIP_H_ALLOW as i32,
                ptr::null() as *const _,
            );

            if !cap_hdr.is_null() {
                //pjsip_msg_add_hdr(msg, pjsip_hdr_clone(tdata.pool, cap_hdr));
                pj_list_insert_before(
                    (*tdata).msg as *mut _,
                    pjsip_hdr_clone((*tdata).pool as *mut _, cap_hdr as *const _),
                );
            }
        }

        // add user-agent header
        #[allow(unused_assignments)]
        let mut h: *const pjsip_hdr = ptr::null();

        let ua_str = CString::new("User-Agent").expect("cant create str User-Agent.");
        let mut ua: pj_str_t = pj_str_t {
            ptr: ua_str.as_ptr() as *mut _,
            slen: 10,
        };
        let agent_str = CString::new("AudioIP 0.1").expect("cant create str AudioIP 0.1");
        let mut agent = pj_str_t {
            ptr: agent_str.as_ptr() as *mut _,
            slen: 11,
        };

        h = pjsip_generic_string_hdr_create(
            (*tdata).pool as *mut _,
            &mut ua as _,
            &mut agent as *mut _,
        ) as *mut _;

        pj_list_insert_before((*tdata).msg as *mut _, h as *mut _);

        PJ_TRUE as pj_status_t
    }
}

