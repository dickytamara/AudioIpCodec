
use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjlib::PjTimerEntry;
use super::pjdefault::*;

use std::ptr;
use std::ffi::CString;
use std::os::raw::c_void;


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
