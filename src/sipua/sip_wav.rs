
use pj_sys::*;
use pjmedia_sys::*;
use pjsua_sys::*;

use crate::pjproject::utils::{self, AutoCreate};
use crate::pjproject::pjsua;

use std::ptr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::mem::MaybeUninit;


pub struct SIPWavPlayer {
    id: pjsua_player_id,
    files: String,
    play_opt: u32,
    play_hangup: bool,
    play_timer: pj_timer_entry,
    port: Box<pjmedia_port>,
}

trait SIPWavPlayerDone {
    unsafe extern "C" fn eof(port: *mut pjmedia_port, usr_data: *mut c_void);
}

// wav player
impl SIPWavPlayer {

    pub fn new(file_path: String, play_options: u32, auto_play_hangup: bool) -> Self {

        let mut player = SIPWavPlayer {
            id: -1,
            files: file_path,
            play_opt: play_options,
            play_hangup: auto_play_hangup,
            play_timer: pj_timer_entry::new(),
            port: Box::new(pjmedia_port::new())
        };

        player.play_opt |= player.play_opt;
        // unsafe {

            pjsua::player_create(
                player.files.clone(),
                player.play_opt,
                &mut player.id
            ).unwrap();

            // let conf_port = pjsua_player_get_conf_port(player.id);
            let conf_port = pjsua::player_get_conf_port(player.id);


            // pjsua_player_get_port(conf_port, player.port as *mut _);

            let status = pjsua::player_get_port(conf_port, player.port.as_mut());

            if player.play_hangup {
                // let status = pjmedia_wav_player_set_eof_cb2(
                //     player.port,
                //     ptr::null_mut(),
                //     Some(SIPWavPlayer::eof),
                // );

                // if status != PJ_SUCCESS as i32 {
                //     panic!("panic set pjmedia_wav_player_set_eof_cb2");
                // }

                // pj_timer_entry_init(
                //     player.port as *mut _,
                //     0,
                //     ptr::null_mut(),
                //     Some(SIPWavPlayer::pj_timer_heap_callback),
                // );
            // }
        }

        player
    }

    pub fn get_conf_port(&self) -> i32 {
        pjsua::player_get_conf_port(self.id)
    }
}

impl SIPWavPlayerDone for SIPWavPlayer {
    unsafe extern "C" fn eof(port: *mut pjmedia_port, usr_data: *mut c_void) {
        println!("");
    }
}


impl Drop for SIPWavPlayer {
    fn drop(&mut self) {
        // destroy player
        // TODO event
        pjsua::player_destroy(self.id).expect("SIPWavPlayer");
    }
}

// wav recorder for log
pub struct SIPWavRecorder {
    id: i32,
    files: String,
    port: *mut pjmedia_port,
}

impl SIPWavRecorder {
    pub fn new() -> Self {
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
