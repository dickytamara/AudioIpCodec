#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use super::pjdefault::{AutoCreate, FromString, boolean_to_pjbool, check_boolean, check_status};
use pj_sys::*;
use pjmedia_sys::*;

use super::pjlib::*;
use super::pjsua;

use std::{mem::MaybeUninit, ptr};
use std::ffi::CStr;
use std::ffi::CString;

impl AutoCreate<pjmedia_srtp_crypto> for pjmedia_srtp_crypto {
    fn new() -> pjmedia_srtp_crypto {
        pjmedia_srtp_crypto {
            key: pj_str_t::new(),
            name: pj_str_t::new(),
            flags: 0,
        }
    }
}

impl AutoCreate<pjmedia_vid_stream_rc_config> for pjmedia_vid_stream_rc_config {
    fn new() -> pjmedia_vid_stream_rc_config {
        pjmedia_vid_stream_rc_config {
            method: 0,
            bandwidth: 0,
        }
    }
}

impl AutoCreate<pjmedia_vid_stream_sk_config> for pjmedia_vid_stream_sk_config {
    fn new() -> pjmedia_vid_stream_sk_config {
        pjmedia_vid_stream_sk_config {
            count: 0,
            interval: 0,
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_cap> for pjmedia_rtcp_fb_cap {
    fn new() -> pjmedia_rtcp_fb_cap {
        pjmedia_rtcp_fb_cap {
            codec_id: pj_str_t::new(),
            type_: 0,
            type_name: pj_str_t::new(),
            param: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_setting> for pjmedia_rtcp_fb_setting {
    fn new() -> pjmedia_rtcp_fb_setting {
        pjmedia_rtcp_fb_setting {
            dont_use_avpf: PJ_FALSE as pj_bool_t,
            cap_count: 0,
            caps: [pjmedia_rtcp_fb_cap::new(); 16],
        }
    }
}

impl AutoCreate<pjmedia_audio_format_detail> for pjmedia_audio_format_detail {
    fn new() -> pjmedia_audio_format_detail {
        pjmedia_audio_format_detail {
            clock_rate: 0,
            channel_count: 0,
            frame_time_usec: 0,
            bits_per_sample: 0,
            avg_bps: 0,
            max_bps: 0,
        }
    }
}

impl AutoCreate<pjmedia_rect_size> for pjmedia_rect_size {
    fn new() -> pjmedia_rect_size {
        pjmedia_rect_size { w: 0, h: 0 }
    }
}

impl AutoCreate<pjmedia_ratio> for pjmedia_ratio {
    fn new() -> pjmedia_ratio {
        pjmedia_ratio { num: 0, denum: 0 }
    }
}

impl AutoCreate<pjmedia_video_format_detail> for pjmedia_video_format_detail {
    fn new() -> pjmedia_video_format_detail {
        pjmedia_video_format_detail {
            size: pjmedia_rect_size::new(),
            fps: pjmedia_ratio::new(),
            avg_bps: 0,
            max_bps: 0,
        }
    }
}

impl AutoCreate<pjmedia_format__bindgen_ty_1> for pjmedia_format__bindgen_ty_1 {
    fn new() -> pjmedia_format__bindgen_ty_1 {
        pjmedia_format__bindgen_ty_1 {
            aud: pjmedia_audio_format_detail::new(),
        }
    }
}

impl AutoCreate<pjmedia_format> for pjmedia_format {
    fn new() -> pjmedia_format {
        pjmedia_format {
            id: 0,
            type_: 0,
            detail_type: 0,
            det: pjmedia_format__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjmedia_port_info> for pjmedia_port_info {
    fn new() -> pjmedia_port_info {
        pjmedia_port_info {
            name: pj_str_t::new(),
            signature: 0,
            dir: 0,
            fmt: pjmedia_format::new(),
        }
    }
}

impl AutoCreate<pjmedia_port_port_data> for pjmedia_port_port_data {
    fn new() -> pjmedia_port_port_data {
        pjmedia_port_port_data {
            pdata: ptr::null_mut(),
            ldata: 0,
        }
    }
}

impl AutoCreate<pjmedia_port> for pjmedia_port {
    fn new() -> pjmedia_port {
        pjmedia_port {
            info: pjmedia_port_info::new(),
            port_data: pjmedia_port_port_data::new(),
            get_clock_src: None,
            put_frame: None,
            get_frame: None,
            on_destroy: None,
        }
    }
}

impl AutoCreate<pjmedia_transport_op> for pjmedia_transport_op {
    fn new() -> pjmedia_transport_op {
        pjmedia_transport_op {
            get_info: None,
            attach: None,
            detach: None,
            send_rtp: None,
            send_rtcp: None,
            send_rtcp2: None,
            media_create: None,
            encode_sdp: None,
            media_start: None,
            media_stop: None,
            simulate_lost: None,
            destroy: None,
            attach2: None,
        }
    }
}

impl AutoCreate<pjmedia_transport> for pjmedia_transport {
    fn new() -> pjmedia_transport {
        pjmedia_transport {
            name: [0; 32],
            type_: 0,
            op: &mut pjmedia_transport_op::new() as *mut _,
            user_data: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjmedia_tone_desc> for pjmedia_tone_desc {
    fn new () -> pjmedia_tone_desc {
        pjmedia_tone_desc {
            freq1: 0,
            freq2: 0,
            on_msec: 0,
            off_msec: 0,
            volume: 0,
            flags: 0
        }
    }
}

impl AutoCreate<pjmedia_aud_dev_info> for pjmedia_aud_dev_info {
    fn new () -> pjmedia_aud_dev_info {
        pjmedia_aud_dev_info {
            name: [0; 64],
            input_count: 0,
            output_count: 0,
            default_samples_per_sec: 0,
            driver: [0; 32],
            caps: 0,
            routes: 0,
            ext_fmt_cnt: 0,
            ext_fmt: [pjmedia_format::new(); 8]
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_info> for pjmedia_rtcp_fb_info {

    fn new () -> pjmedia_rtcp_fb_info {
        pjmedia_rtcp_fb_info {
            cap_count: 0,
            caps: [pjmedia_rtcp_fb_cap::new(); 16usize],
        }
    }

}

impl AutoCreate<pjmedia_codec_info> for pjmedia_codec_info {

    fn new () -> pjmedia_codec_info {
            pjmedia_codec_info {
            type_: 0,
            pt: 0,
            encoding_name: pj_str_t::new(),
            clock_rate: 0,
            channel_cnt: 0,
        }
    }

}

impl AutoCreate<pjmedia_stream_info> for pjmedia_stream_info {
    fn new () -> pjmedia_stream_info {
        pjmedia_stream_info {
            type_: 0,
            proto: 0,
            dir: 0,
            rem_addr: pj_sockaddr::new(),
            rem_rtcp: pj_sockaddr::new(),
            rtcp_mux: PJ_FALSE as pj_bool_t,
            loc_rtcp_fb: pjmedia_rtcp_fb_info::new(),
            rem_rtcp_fb: pjmedia_rtcp_fb_info::new(),
            fmt: pjmedia_codec_info::new(),
            param: ptr::null_mut(),
            tx_pt: 0,
            rx_pt: 0,
            tx_maxptime: 0,
            tx_event_pt: -1,
            rx_event_pt: -1,
            ssrc: 0,
            cname: pj_str_t::new(),
            has_rem_ssrc: PJ_FALSE as pj_bool_t,
            rem_ssrc: 0,
            rem_cname: pj_str_t::new(),
            rtp_ts: 0,
            rtp_seq: 0,
            rtp_seq_ts_set: 0,
            jb_init: -1,
            jb_min_pre: -1,
            jb_max_pre: -1,
            jb_max: -1,
            jb_discard_algo: 0,
            rtcp_sdes_bye_disabled: PJ_TRUE as pj_bool_t,
        }
    }
}

impl AutoCreate<__BindgenBitfieldUnit<[u8;1usize]>> for __BindgenBitfieldUnit<[u8;1usize]> {
    fn new () -> __BindgenBitfieldUnit<[u8;1usize]> {
        __BindgenBitfieldUnit {
            storage: [0; 1usize]
        }
    }
}

impl AutoCreate<pjmedia_rtcp_stream_stat__bindgen_ty_1> for pjmedia_rtcp_stream_stat__bindgen_ty_1 {
    fn new () -> pjmedia_rtcp_stream_stat__bindgen_ty_1 {
        pjmedia_rtcp_stream_stat__bindgen_ty_1 {
            _bitfield_align_1: [0; 0],
            _bitfield_1: __BindgenBitfieldUnit::new([0; 1usize]),
            __bindgen_padding_0: [0; 3usize],
        }
    }
}

impl AutoCreate<pjmedia_rtcp_sdes> for pjmedia_rtcp_sdes {
    fn new () -> pjmedia_rtcp_sdes {
        pjmedia_rtcp_sdes {
            cname: pj_str_t::new(),
            name: pj_str_t::new(),
            email: pj_str_t::new(),
            phone: pj_str_t::new(),
            loc: pj_str_t::new(),
            tool: pj_str_t::new(),
            note: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjmedia_rtcp_stream_stat> for pjmedia_rtcp_stream_stat {
    fn new () -> pjmedia_rtcp_stream_stat {
        pjmedia_rtcp_stream_stat {
            update: pj_time_val::new(),
            update_cnt: 0,
            pkt: 0,
            bytes: 0,
            discard: 0,
            loss: 0,
            reorder: 0,
            dup: 0,
            loss_period: pj_math_stat::new(),
            loss_type: pjmedia_rtcp_stream_stat__bindgen_ty_1::new(),
            jitter: pj_math_stat::new(),
        }
    }
}

impl AutoCreate<pjmedia_rtcp_stat> for pjmedia_rtcp_stat {
    fn new () -> pjmedia_rtcp_stat {
        pjmedia_rtcp_stat {
            start: pj_time_val::new(),
            tx: pjmedia_rtcp_stream_stat::new(),
            rx: pjmedia_rtcp_stream_stat::new(),
            rtt: pj_math_stat::new(),
            rtp_tx_last_ts: 0,
            rtp_tx_last_seq: 0,
            peer_sdes: pjmedia_rtcp_sdes::new(),
            peer_sdes_buf_: [0; 64usize]
        }
    }
}

impl AutoCreate<pjmedia_jb_state> for pjmedia_jb_state {
    fn new () -> pjmedia_jb_state {
        pjmedia_jb_state {
            frame_size: 0,
            min_prefetch: 0,
            max_prefetch: 0,
            max_count: 0,
            burst: 0,
            prefetch: 0,
            size: 0,
            avg_delay: 0,
            min_delay: 0,
            max_delay: 0,
            dev_delay: 0,
            avg_burst: 0,
            lost: 0,
            discard: 0,
            empty: 0,
        }
    }
}

impl AutoCreate<pjmedia_sock_info> for pjmedia_sock_info {
    fn new () -> pjmedia_sock_info {
        pjmedia_sock_info {
            rtp_sock: 0,
            rtp_addr_name: pj_sockaddr::new(),
            rtcp_sock: 0,
            rtcp_addr_name: pj_sockaddr::new(),
        }
    }
}

impl AutoCreate<pjmedia_transport_specific_info> for pjmedia_transport_specific_info {
    fn new () -> pjmedia_transport_specific_info {
        pjmedia_transport_specific_info {
            type_: 0,
            cbsize: -1,
            buffer: [0; 288usize],
        }
    }
}

impl AutoCreate<pjmedia_transport_info> for pjmedia_transport_info {
    fn new () -> pjmedia_transport_info {
        pjmedia_transport_info {
            sock_info: pjmedia_sock_info::new(),
            src_rtp_name: pj_sockaddr::new(),
            src_rtcp_name: pj_sockaddr::new(),
            specific_info_cnt: 0,
            spc_info: [pjmedia_transport_specific_info::new(); 4usize],
        }
    }
}

impl AutoCreate<pjmedia_codec_param__bindgen_ty_1> for pjmedia_codec_param__bindgen_ty_1 {
    fn new () -> pjmedia_codec_param__bindgen_ty_1 {
        pjmedia_codec_param__bindgen_ty_1 {
            clock_rate: 0,
            channel_cnt: 0,
            avg_bps: 0,
            max_bps: 0,
            max_rx_frame_size: 0,
            frm_ptime: 0,
            enc_ptime: 0,
            pcm_bits_per_sample: 0,
            pt: 0,
            fmt_id: 0,
        }
    }
}


impl AutoCreate<pjmedia_codec_fmtp_param> for pjmedia_codec_fmtp_param {
    fn new () -> pjmedia_codec_fmtp_param {
        pjmedia_codec_fmtp_param {
            name: pj_str_t::from_string(String::new()),
            val: pj_str_t::from_string(String::new())
        }
    }
}


impl AutoCreate<pjmedia_codec_fmtp> for pjmedia_codec_fmtp {
    fn new() -> pjmedia_codec_fmtp {
        pjmedia_codec_fmtp {
            cnt: 0,
            param: [pjmedia_codec_fmtp_param::new(); 16usize],
        }
    }
}


impl AutoCreate<pjmedia_codec_param__bindgen_ty_2> for pjmedia_codec_param__bindgen_ty_2 {
    fn new() -> pjmedia_codec_param__bindgen_ty_2 {
        pjmedia_codec_param__bindgen_ty_2 {
            frm_per_pkt: 0,
            _bitfield_align_1: [0; 0],
            _bitfield_1: __BindgenBitfieldUnit::new([0; 1usize]),
            enc_fmtp: pjmedia_codec_fmtp::new(),
            dec_fmtp: pjmedia_codec_fmtp::new(),
        }
    }
}

impl AutoCreate<pjmedia_codec_param> for pjmedia_codec_param {
    fn new () -> pjmedia_codec_param {
        pjmedia_codec_param {
            info: pjmedia_codec_param__bindgen_ty_1::new(),
            setting: pjmedia_codec_param__bindgen_ty_2::new()
        }
    }
}


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
            boolean_to_pjbool(refresh)
        );

        check_status(status)
    }
}

pub fn aud_driver_deinit(drv_idx: u32) {
    unsafe { pjmedia_aud_driver_deinit(drv_idx) }
}

// const char * 	pjmedia_aud_dev_cap_name (pjmedia_aud_dev_cap cap, const char **p_desc)
// pj_status_t 	pjmedia_aud_param_set_cap (pjmedia_aud_param *param, pjmedia_aud_dev_cap cap, const void *pval)
// pj_status_t 	pjmedia_aud_param_get_cap (const pjmedia_aud_param *param, pjmedia_aud_dev_cap cap, void *pval)

pub fn aud_dev_refresh() -> Result<(), pj_status_t> {
    unsafe {
        check_status(pjmedia_aud_dev_refresh())
    }
}

pub fn aud_dev_count() -> u32 {
    unsafe { pjmedia_aud_dev_count() }
}

pub fn aud_dev_get_info(id: pjmedia_aud_dev_index, info: &mut pjmedia_aud_dev_info) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_dev_get_info(id, info as *mut _);
        check_status(status)
    }
}

pub fn aud_dev_lookup (drv_name: String, dev_name: String, id: &mut pjmedia_aud_dev_index) -> Result<(), pj_status_t> {
    let drv_name = CString::new(drv_name.as_str()).expect("error drv_name").into_raw();
    let dev_name = CString::new(dev_name.as_str()).expect("error dev_name").into_raw();

    unsafe {
        let status = pjmedia_aud_dev_lookup(
            drv_name as *const _,
            dev_name as *const _,
            id as *mut _
        );

        check_status(status)
    }
}

pub fn aud_dev_default_param(id: pjmedia_aud_dev_index, param: &mut pjmedia_aud_param) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_dev_default_param(
            id,
            param as *mut _
        );

        check_status(status)
    }
}

// pj_status_t 	pjmedia_aud_stream_create (const pjmedia_aud_param *param, pjmedia_aud_rec_cb rec_cb, pjmedia_aud_play_cb play_cb, void *user_data, pjmedia_aud_stream **p_strm)

pub fn aud_stream_get_param (strm: &mut pjmedia_aud_stream, param: &mut pjmedia_aud_param) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_stream_get_param(
            strm as *mut _,
            param as *mut _
        );

        check_status(status)
    }
}

// pj_status_t 	pjmedia_aud_stream_get_cap (pjmedia_aud_stream *strm, pjmedia_aud_dev_cap cap, void *value)
// pj_status_t 	pjmedia_aud_stream_set_cap (pjmedia_aud_stream *strm, pjmedia_aud_dev_cap cap, const void *value)

pub fn aud_stream_start(strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_stream_start(strm as *mut _);
        check_status(status)
    }
}

pub fn aud_stream_stop(strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_stream_stop(strm as *mut _);
        check_status(status)
    }
}

pub fn aud_stream_destroy (strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_stream_destroy(strm as *mut _);
        check_status(status)
    }
}

pub fn aud_subsys_init(pf: *mut pj_pool_factory) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_subsys_init(pf);
        check_status(status)
    }
}

pub fn aud_subsys_get_pool_factory() -> *mut pj_pool_factory {
    unsafe { pjmedia_aud_subsys_get_pool_factory() }
}

pub fn aud_subsys_shutdown() -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_aud_subsys_shutdown();
        check_status(status)
    }
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

        check_status(result)
    }
}

pub fn tonegen_is_busy(tonegen: &mut pjmedia_port) -> bool {
    unsafe {
        let result = pjmedia_tonegen_is_busy(tonegen  as *mut _);
        check_boolean(result)
    }
}

pub fn tonegen_stop(tonegen: *mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_tonegen_stop(tonegen);
        check_status(status)
    }
}

pub fn tonegen_rewind(tonegen: &mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_tonegen_rewind(tonegen as *mut _);
        check_status(status)
    }
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
        let status = pjmedia_tonegen_play(
            tonegen,
            count,
            tones.as_ptr(),
            options
        );

        check_status(status)
    }
}

// pj_status_t 	pjmedia_tonegen_play_digits (pjmedia_port *tonegen, unsigned count, const pjmedia_tone_digit digits[], unsigned options)
// pj_status_t 	pjmedia_tonegen_get_digit_map (pjmedia_port *tonegen, const pjmedia_tone_digit_map **m)
// pj_status_t 	pjmedia_tonegen_set_digit_map (pjmedia_port *tonegen, pjmedia_tone_digit_map *m)


// pj_status_t 	pjmedia_codec_opus_init (pjmedia_endpt *endpt)
pub fn codec_opus_init(endpt: &mut pjmedia_endpt) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_codec_opus_init(endpt as *mut _);
        check_status(status)
    }
}

// pj_status_t 	pjmedia_codec_opus_deinit (void)
pub fn codec_opus_deinit() -> Result<(), pj_status_t> {
    unsafe {
        check_status(pjmedia_codec_opus_deinit())
    }
}

// pj_status_t 	pjmedia_codec_opus_get_config (pjmedia_codec_opus_config *cfg)
pub fn codec_opus_get_config(cfg: &mut pjmedia_codec_opus_config) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_codec_opus_get_config(
            cfg as *mut _
        );
        check_status(status)
    }
}

// pj_status_t 	pjmedia_codec_opus_set_default_param (const pjmedia_codec_opus_config *cfg, pjmedia_codec_param *param)
pub fn codec_opus_set_default_param(cfg: &mut pjmedia_codec_opus_config, param: &mut pjmedia_codec_param) -> Result<(), pj_status_t> {
    unsafe {
        let status = pjmedia_codec_opus_set_default_param (
            cfg as *mut _,
            param as *mut _
        );
        check_status(status)
    }
}
