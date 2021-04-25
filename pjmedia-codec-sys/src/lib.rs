/* automatically generated by rust-bindgen 0.58.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate pj_sys;
extern crate pjmedia_sys;
use pj_sys::*;
use pjmedia_sys::*;

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1 {
    pub win: __BindgenUnionField<pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_1>,
    pub x11: __BindgenUnionField<pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_2>,
    pub cocoa: __BindgenUnionField<pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_3>,
    pub ios: __BindgenUnionField<pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_4>,
    pub android: __BindgenUnionField<pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_5>,
    pub window: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_passthrough_setting {
    pub fmt_cnt: ::std::os::raw::c_uint,
    pub fmts: *mut pjmedia_format,
    pub ilbc_mode: ::std::os::raw::c_uint,
}
extern "C" {
    pub fn pjmedia_codec_passthrough_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_passthrough_init2(
        endpt: *mut pjmedia_endpt,
        setting: *const pjmedia_codec_passthrough_setting,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_passthrough_deinit() -> pj_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_codec_config {
    pub speex: pjmedia_audio_codec_config__bindgen_ty_1,
    pub ilbc: pjmedia_audio_codec_config__bindgen_ty_2,
    pub passthrough: pjmedia_audio_codec_config__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_codec_config__bindgen_ty_1 {
    pub option: ::std::os::raw::c_uint,
    pub quality: ::std::os::raw::c_int,
    pub complexity: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_codec_config__bindgen_ty_2 {
    pub mode: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_codec_config__bindgen_ty_3 {
    pub setting: pjmedia_codec_passthrough_setting,
}
extern "C" {
    pub fn pjmedia_audio_codec_config_default(cfg: *mut pjmedia_audio_codec_config);
}
extern "C" {
    pub fn pjmedia_codec_register_audio_codecs(
        endpt: *mut pjmedia_endpt,
        c: *const pjmedia_audio_codec_config,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_bcg729_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_bcg729_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_ffmpeg_vid_init(
        mgr: *mut pjmedia_vid_codec_mgr,
        pf: *mut pj_pool_factory,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_ffmpeg_vid_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g722_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g722_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g722_set_pcm_shift(val: ::std::os::raw::c_uint) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g7221_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g7221_set_mode(
        sample_rate: ::std::os::raw::c_uint,
        bitrate: ::std::os::raw::c_uint,
        enabled: pj_bool_t,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g7221_set_pcm_shift(val: ::std::os::raw::c_int) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_g7221_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_gsm_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_gsm_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_ilbc_init(
        endpt: *mut pjmedia_endpt,
        mode: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_ilbc_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_ipp_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_ipp_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_l16_init(
        endpt: *mut pjmedia_endpt,
        options: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_l16_deinit() -> pj_status_t;
}
pub const PJMEDIA_AMR_NO_NB: pjmedia_amr_options = 1;
pub const PJMEDIA_AMR_NO_WB: pjmedia_amr_options = 2;
pub type pjmedia_amr_options = u32;
#[repr(C)]
pub struct pjmedia_codec_amr_config {
    pub octet_align: pj_bool_t,
    pub bitrate: ::std::os::raw::c_uint,
}
pub type pjmedia_codec_amrnb_config = pjmedia_codec_amr_config;
pub type pjmedia_codec_amrwb_config = pjmedia_codec_amr_config;
extern "C" {
    pub fn pjmedia_codec_opencore_amr_init(
        endpt: *mut pjmedia_endpt,
        options: ::std::os::raw::c_uint,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opencore_amr_init_default(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opencore_amr_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opencore_amrnb_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opencore_amrnb_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opencore_amrnb_set_config(
        cfg: *const pjmedia_codec_amrnb_config,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opencore_amrwb_set_config(
        cfg: *const pjmedia_codec_amrwb_config,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_openh264_vid_init(
        mgr: *mut pjmedia_vid_codec_mgr,
        pf: *mut pj_pool_factory,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_openh264_vid_deinit() -> pj_status_t;
}
#[repr(C)]
pub struct pjmedia_codec_opus_config {
    pub sample_rate: ::std::os::raw::c_uint,
    pub channel_cnt: ::std::os::raw::c_uint,
    pub frm_ptime: ::std::os::raw::c_uint,
    pub bit_rate: ::std::os::raw::c_uint,
    pub packet_loss: ::std::os::raw::c_uint,
    pub complexity: ::std::os::raw::c_uint,
    pub cbr: pj_bool_t,
}
extern "C" {
    pub fn pjmedia_codec_opus_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opus_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opus_get_config(cfg: *mut pjmedia_codec_opus_config) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_opus_set_default_param(
        cfg: *const pjmedia_codec_opus_config,
        param: *mut pjmedia_codec_param,
    ) -> pj_status_t;
}
#[repr(C)]
pub struct pjmedia_codec_silk_setting {
    pub enabled: pj_bool_t,
    pub quality: ::std::os::raw::c_int,
    pub complexity: ::std::os::raw::c_int,
}
extern "C" {
    pub fn pjmedia_codec_silk_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_silk_set_config(
        clock_rate: ::std::os::raw::c_uint,
        opt: *const pjmedia_codec_silk_setting,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_silk_deinit() -> pj_status_t;
}
pub const PJMEDIA_SPEEX_NO_NB: pjmedia_speex_options = 1;
pub const PJMEDIA_SPEEX_NO_WB: pjmedia_speex_options = 2;
pub const PJMEDIA_SPEEX_NO_UWB: pjmedia_speex_options = 4;
pub type pjmedia_speex_options = u32;
extern "C" {
    pub fn pjmedia_codec_speex_init(
        endpt: *mut pjmedia_endpt,
        options: ::std::os::raw::c_uint,
        quality: ::std::os::raw::c_int,
        complexity: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_speex_init_default(endpt: *mut pjmedia_endpt) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_speex_set_param(
        clock_rate: ::std::os::raw::c_uint,
        quality: ::std::os::raw::c_int,
        complexity: ::std::os::raw::c_int,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_speex_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_vid_toolbox_init(
        mgr: *mut pjmedia_vid_codec_mgr,
        pf: *mut pj_pool_factory,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_vid_toolbox_deinit() -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_vpx_vid_init(
        mgr: *mut pjmedia_vid_codec_mgr,
        pf: *mut pj_pool_factory,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_codec_vpx_vid_deinit() -> pj_status_t;
}
