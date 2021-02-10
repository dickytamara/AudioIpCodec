

use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;

impl AutoCreate<pjmedia_srtp_crypto>  for pjmedia_srtp_crypto {
    fn new() -> pjmedia_srtp_crypto {
        pjmedia_srtp_crypto {
            key: pj_str_t::new(),
            name: pj_str_t::new(),
            flags: 0
        }
    }
}


impl AutoCreate<pjmedia_vid_stream_rc_config> for pjmedia_vid_stream_rc_config {
    fn new () -> pjmedia_vid_stream_rc_config {
        pjmedia_vid_stream_rc_config {
            method: 0,
            bandwidth: 0
        }
    }
}

impl AutoCreate<pjmedia_vid_stream_sk_config> for pjmedia_vid_stream_sk_config {
    fn new () -> pjmedia_vid_stream_sk_config {
        pjmedia_vid_stream_sk_config {
            count: 0,
            interval: 0
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_cap> for pjmedia_rtcp_fb_cap {
    fn new () -> pjmedia_rtcp_fb_cap {
        pjmedia_rtcp_fb_cap {
            codec_id: pj_str_t::new(),
            type_: 0,
            type_name: pj_str_t::new(),
            param: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_setting> for pjmedia_rtcp_fb_setting {
    fn new () -> pjmedia_rtcp_fb_setting {
        pjmedia_rtcp_fb_setting {
            dont_use_avpf: pj_constants__PJ_FALSE,
            cap_count: 0,
            caps: [ pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                    pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                    pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                    pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new()
            ]
        }
    }
}
