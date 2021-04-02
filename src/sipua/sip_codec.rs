use super::pj_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjsua::*;
use super::pjdefault::*;

use super::pjmedia;
use super::pjsua;

use std::os::raw::c_uint;
use std::ffi::CStr;


pub struct SIPCodec {
    codec_info: pjsua_codec_info
}


impl SIPCodec {

    pub fn from_info(info: pjsua_codec_info) -> Self {
        SIPCodec {
            codec_info : info
        }
    }

    pub fn set_priority(&self, priority: u8) {
        pjsua::codec_set_priority(
            self.codec_info.codec_id.to_string(),
            priority
        ).expect("Can't set codec priority.");
    }

    // pub fn get_param(&self) -> Option<pjmedia_codec_param> {

    //     let mut param = pjmedia_codec_param::new();

    //     if let Err(e) = pjsua::codec_get_param(
    //         self.codec_info.codec_id.to_string(),
    //         &mut param
    //     ) {
    //         return None;
    //     }

    //     Some(param)
    // }

    pub fn set_param(&self, param: &mut pjmedia_codec_param) {

    }
}


pub struct SIPCodecs { }

impl SIPCodecs {

    // pj_status_t 	pjsua_enum_codecs (pjsua_codec_info id[], unsigned *count)
    // pj_status_t 	pjsua_codec_set_priority (const pj_str_t *codec_id, pj_uint8_t priority)
    // pj_status_t 	pjsua_codec_get_param (const pj_str_t *codec_id, pjmedia_codec_param *param)
    // pj_status_t 	pjsua_codec_set_param (const pj_str_t *codec_id, const pjmedia_codec_param *param)

    pub fn new() -> Self {
        SIPCodecs { }
    }


    pub fn enum_codecs(&self) -> Vec<pjsua_codec_info> {

        let mut ret: Vec<pjsua_codec_info> = Vec::new();

        let mut count: u32 = 0;
        let mut codecs = [pjsua_codec_info::new(); 32];

        pjsua::enum_codecs(&mut codecs, &mut count).unwrap();

        for i in 0..count as usize {
            ret.push(codecs[i]);
        }

        ret
    }

}