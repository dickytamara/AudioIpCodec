/* automatically generated by rust-bindgen 0.58.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate pj_sys;
extern crate pjmedia_sys;
use pj_sys::*;
use pjmedia_sys::*;

extern "C" {
    pub fn pjmedia_video_format_mgr_create(
        pool: *mut pj_pool_t,
        max_fmt: ::std::os::raw::c_uint,
        options: ::std::os::raw::c_uint,
        p_mgr: *mut *mut pjmedia_video_format_mgr,
    ) -> pj_status_t;
}
extern "C" {
    pub fn pjmedia_video_format_mgr_instance() -> *mut pjmedia_video_format_mgr;
}
extern "C" {
    pub fn pjmedia_video_format_mgr_set_instance(mgr: *mut pjmedia_video_format_mgr);
}
extern "C" {
    pub fn pjmedia_video_format_mgr_destroy(mgr: *mut pjmedia_video_format_mgr);
}