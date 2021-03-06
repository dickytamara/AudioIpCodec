
use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;

use std::ptr;
use std::ffi::CString;

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
