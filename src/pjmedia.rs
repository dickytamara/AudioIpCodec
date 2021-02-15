use super::pjdefault::AutoCreate;
use super::pjsua_sys::*;
use std::ptr;

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
            dont_use_avpf: pj_constants__PJ_FALSE as pj_bool_t,
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
