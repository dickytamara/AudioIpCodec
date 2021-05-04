#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use pj_sys::*;
use pjmedia_sys::*;
use pjmedia_codec_sys::*;
use pjmedia_audiodev_sys::*;

use super::prelude::*;
use super::utils;

// use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;


pub mod auto;


pub fn type_name(media_type: pjmedia_type) -> String {
    unsafe {
        String::from(CStr::from_ptr(
            pjmedia_type_name(media_type)
        ).to_str()
        .expect("Error string media type"))
    }
}

// function helper to reduce unsafe block


// Audio device API

// pjmedia_aud_subsys * 	pjmedia_get_aud_subsys (void)
pub fn get_aud_subsys() -> *mut pjmedia_aud_subsys {
    unsafe { pjmedia_get_aud_subsys() }
}

pub fn aud_driver_init(drv_idx: u32, refresh: bool) -> Result<(), pj_status_t> {

    unsafe {
        let status = pjmedia_aud_driver_init(
            drv_idx,
            utils::boolean_to_pjbool(refresh)
        );

        utils::check_status(status)
    }
}

pub fn aud_driver_deinit(drv_idx: u32) {
    unsafe { pjmedia_aud_driver_deinit(drv_idx) }
}

// const char * 	pjmedia_aud_dev_cap_name (pjmedia_aud_dev_cap cap, const char **p_desc)
// pj_status_t 	pjmedia_aud_param_set_cap (pjmedia_aud_param *param, pjmedia_aud_dev_cap cap, const void *pval)
// pj_status_t 	pjmedia_aud_param_get_cap (const pjmedia_aud_param *param, pjmedia_aud_dev_cap cap, void *pval)

pub fn aud_dev_refresh() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_dev_refresh()) }
}

pub fn aud_dev_count() -> u32 {
    unsafe { pjmedia_aud_dev_count() }
}

pub fn aud_dev_get_info(id: pjmedia_aud_dev_index, info: &mut pjmedia_aud_dev_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_dev_get_info(id, info as *mut _))
    }
}

pub fn aud_dev_lookup (drv_name: String, dev_name: String, id: &mut pjmedia_aud_dev_index) -> Result<(), pj_status_t> {
    let drv_name: *const i8 = CString::new(drv_name.as_str()).expect("error drv_name").into_raw();
    let dev_name: *const i8 = CString::new(dev_name.as_str()).expect("error dev_name").into_raw();

    unsafe {
        utils::check_status(pjmedia_aud_dev_lookup( drv_name, dev_name, id as *mut _))
    }
}

pub fn aud_dev_default_param(id: pjmedia_aud_dev_index, param: &mut pjmedia_aud_param) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_dev_default_param( id, param as *mut _)) }
}

// pj_status_t 	pjmedia_aud_stream_create (const pjmedia_aud_param *param, pjmedia_aud_rec_cb rec_cb, pjmedia_aud_play_cb play_cb, void *user_data, pjmedia_aud_stream **p_strm)

pub fn aud_stream_get_param (strm: &mut pjmedia_aud_stream, param: &mut pjmedia_aud_param) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_get_param(strm as *mut _, param as *mut _))
    }
}

// pj_status_t 	pjmedia_aud_stream_get_cap (pjmedia_aud_stream *strm, pjmedia_aud_dev_cap cap, void *value)
// pj_status_t 	pjmedia_aud_stream_set_cap (pjmedia_aud_stream *strm, pjmedia_aud_dev_cap cap, const void *value)

pub fn aud_stream_start(strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_start(strm as *mut _))
    }
}

pub fn aud_stream_stop(strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_stop(strm as *mut _))
    }
}

pub fn aud_stream_destroy (strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_destroy(strm as *mut _))
    }
}

pub fn aud_subsys_init(pf: *mut pj_pool_factory) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_subsys_init(pf)) }
}

pub fn aud_subsys_get_pool_factory() -> *mut pj_pool_factory {
    unsafe { pjmedia_aud_subsys_get_pool_factory() }
}

pub fn aud_subsys_shutdown() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_subsys_shutdown()) }
}

// pj_status_t 	pjmedia_aud_register_factory (pjmedia_aud_dev_factory_create_func_ptr adf)
// pj_status_t 	pjmedia_aud_unregister_factory (pjmedia_aud_dev_factory_create_func_ptr adf)


// pj_status_t 	pjmedia_tonegen_create (pj_pool_t *pool, unsigned clock_rate, unsigned channel_count, unsigned samples_per_frame, unsigned bits_per_sample, unsigned options, pjmedia_port **p_port)

pub fn tonegen_create2(
    pool: *mut pj_pool_t,
    name: String,
    clock_rate: u32,
    channel_count: u32,
    samples_per_frame: u32,
    bits_per_sample: u32,
    options: u32,
    p_port: &mut Box<*mut pjmedia_port>
) -> Result<(), pj_status_t> {
    unsafe {

        let mut name = pj_str_t::from_string(name);

        let result = pjmedia_tonegen_create2(
            pool,
            &mut name as *const _,
            clock_rate,
            channel_count,
            samples_per_frame,
            bits_per_sample,
            options,
            p_port.as_mut() as *mut _
        );

        utils::check_status(result)
    }
}

pub fn tonegen_is_busy(tonegen: &mut pjmedia_port) -> bool {
    unsafe {
        utils::check_boolean(pjmedia_tonegen_is_busy(tonegen  as *mut _))
    }
}

pub fn tonegen_stop(tonegen: *mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_tonegen_stop(tonegen)) }
}

pub fn tonegen_rewind(tonegen: &mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_tonegen_rewind(tonegen as *mut _) )}
}

// pj_status_t 	pjmedia_tonegen_play (pjmedia_port *tonegen, unsigned count, const pjmedia_tone_desc tones[], unsigned options)
// PJMEDIA_TONEGEN_MAX_DIGITS 32
pub fn tonegen_play(
    tonegen: *mut pjmedia_port,
    count: u32,
    tones: &mut [pjmedia_tone_desc; 32usize],
    options: u32
) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_tonegen_play( tonegen, count, tones.as_ptr(), options ))
    }
}

// pj_status_t 	pjmedia_tonegen_play_digits (pjmedia_port *tonegen, unsigned count, const pjmedia_tone_digit digits[], unsigned options)
// pj_status_t 	pjmedia_tonegen_get_digit_map (pjmedia_port *tonegen, const pjmedia_tone_digit_map **m)
// pj_status_t 	pjmedia_tonegen_set_digit_map (pjmedia_port *tonegen, pjmedia_tone_digit_map *m)


// pj_status_t 	pjmedia_codec_opus_init (pjmedia_endpt *endpt)
pub fn codec_opus_init(endpt: &mut pjmedia_endpt) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_codec_opus_init(endpt as *mut _)) }
}

// pj_status_t 	pjmedia_codec_opus_deinit (void)
pub fn codec_opus_deinit() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_codec_opus_deinit()) }
}

// pj_status_t 	pjmedia_codec_opus_get_config (pjmedia_codec_opus_config *cfg)
pub fn codec_opus_get_config(cfg: &mut pjmedia_codec_opus_config) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_codec_opus_get_config( cfg as *mut _ )) }
}

// pj_status_t 	pjmedia_codec_opus_set_default_param (const pjmedia_codec_opus_config *cfg, pjmedia_codec_param *param)
pub fn codec_opus_set_default_param(cfg: &mut pjmedia_codec_opus_config, param: &mut pjmedia_codec_param) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_codec_opus_set_default_param ( cfg as *mut _, param as *mut _ ))
    }
}
