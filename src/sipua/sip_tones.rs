
use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;

use super::pjsua;
use super::pjmedia;
use std::ptr;
use std::ffi::CString;

// Optional
#[derive(Clone)]
pub struct SIPTones {
    slot: i32,
    tones: [pjmedia_tone_desc; 32usize],
    port: Box::<*mut pjmedia_port>,
}

impl SIPTones {
    pub fn new() -> SIPTones {
        SIPTones {
            slot: -1,
            tones: [pjmedia_tone_desc::new(); 32usize],
            port: Box::new(ptr::null_mut())

        }
    }

    pub fn init(&mut self, pool: *mut pj_pool_t, freq1: u16, freq2: u16) {

        let mut status = pjmedia::tonegen_create2(
            pool,
            format!("tone-{}-{}", freq1, freq2),
            8000,
            1,
            160,
            16,
            PJMEDIA_TONEGEN_LOOP,
            &mut self.port
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPTones");
        }

        status = pjsua::conf_add_port(
            &mut self.port,
            Some(&mut self.slot)
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPTones");
        }

        status = pjmedia::tonegen_play(
            &mut self.port,
            1,
            &mut self.tones,
            0
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPTones");
        }

        assert_ne!(self.slot, -1);

        // println!(
        //     "SIPTones init slot {}, freq {} and {}",
        //     self.slot, freq1, freq2
        // );
    }
}

//  Ringback tone
pub struct SIPRingback {
    tones: [pjmedia_tone_desc; 32usize],
    slot: i32,
    port: Box<*mut pjmedia_port>,
}

impl SIPRingback {
    pub fn new() -> SIPRingback {
        SIPRingback {
            tones: [pjmedia_tone_desc::new(); 32usize],
            slot: -1,
            port: Box::new(ptr::null_mut()),
        }
    }

    pub fn init(&mut self, pool: *mut pj_pool_t, media_config: pjsua_media_config) {

        let samples_per_frame = media_config.audio_frame_ptime
            * media_config.clock_rate
            * media_config.channel_count
            / 1000;

        self.tones[0].freq1 = 440;
        self.tones[0].freq2 = 480;
        self.tones[0].on_msec = 2000;
        self.tones[0].off_msec = 4000;

        let mut status = pjmedia::tonegen_create2(
            pool,
            String::from("ringback"),
            media_config.clock_rate,
            media_config.channel_count,
            samples_per_frame,
            16,
            PJMEDIA_TONEGEN_LOOP,
            &mut self.port
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPRingback");
        }

        status = pjsua::conf_add_port(
            &mut self.port,
            Some(&mut self.slot)
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPRingback");
        }

        status = pjmedia::tonegen_play(
            &mut self.port,
            1,
            &mut self.tones,
            PJMEDIA_TONEGEN_LOOP
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPRingback");
        }

        assert_ne!(self.port.is_null(), true);
        assert_ne!(self.slot, -1);
        // println!("SIPRingback init with slot {}", self.slot);
    }
}

impl Drop for SIPRingback {
    fn drop(&mut self) {
        pjmedia::tonegen_stop(&mut self.port);
    }
}

// this tone gen will alert on incoming call
pub struct SIPRingtone {
    tones: [pjmedia_tone_desc; 32usize],
    slot: i32,
    port: Box<*mut pjmedia_port>,
}

impl SIPRingtone {
    pub fn new() -> SIPRingtone {
        SIPRingtone {
            tones: [pjmedia_tone_desc::new(); 32usize],
            slot: -1,
            port: Box::new(ptr::null_mut()),
        }
    }

    pub fn init(&mut self, pool: *mut pj_pool_t, media_config: pjsua_media_config) {

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

        let mut status = pjmedia::tonegen_create2(
            pool,
            String::from("ringtone"),
            media_config.clock_rate,
            media_config.channel_count,
            samples_per_frame,
            16,
            PJMEDIA_TONEGEN_LOOP,
            &mut self.port
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPRingtone");
        }

        status = pjsua::conf_add_port(
            &mut self.port,
            Some(&mut self.slot));

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPRingtone");
        }

        status = pjmedia::tonegen_play(
            &mut self.port,
            3,
            &mut self.tones,
            PJMEDIA_TONEGEN_LOOP
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init SIPRingtone");
        }

        assert_ne!(self.port.is_null(), true);
        assert_ne!(self.slot, -1);
        println!("SIPRingtone init with slot {}", self.slot);
    }
}

impl Drop for SIPRingtone {
    fn drop(&mut self) {
        pjmedia::tonegen_stop(&mut self.port);
    }
}
