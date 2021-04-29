

// use pjmedia_sys::*;
// use pjsua_sys::*;

use crate::pjproject::prelude::*;
use crate::pjproject::pjsua;



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

    pub fn get_param(&self) -> Option<pjmedia_codec_param> {

        let mut param = pjmedia_codec_param::new();

        if let Err(e) = pjsua::codec_get_param(
            self.codec_info.codec_id.to_string(),
            &mut param
        ) {
            return None;
        }

        Some(param)
    }

    pub fn set_param(&self, param: &mut pjmedia_codec_param) {
        pjsua::codec_set_param(self.codec_info.codec_id.to_string(),
            param
        ).expect("Can't set codec parameter");
    }
}


pub struct SIPCodecs { }

impl SIPCodecs {

    pub fn new() -> Self {
        SIPCodecs { }
    }

    pub fn enum_codecs(&self) -> Vec<pjsua_codec_info> {

        let ret: Vec<pjsua_codec_info> = Vec::new();

        // let mut count: u32 = 0;
        // let mut codecs = [pjsua_codec_info::new(); 32];

        // pjsua::enum_codecs(&mut codecs, &mut count).unwrap();

        // for i in 0..count as usize {
        //     ret.push(codecs[i]);
        // }

        ret
    }

    pub fn get_codec_param(&self, codec_id: String) -> Option<pjmedia_codec_param> {

        let param = pjmedia_codec_param::new();

        // param
        // TODO


        Some(param)
    }

}