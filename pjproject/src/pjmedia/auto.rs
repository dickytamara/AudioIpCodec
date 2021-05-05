#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]


use super::utils::AutoCreate;
use super::*;

use std::ptr;

impl AutoCreate<pjmedia_srtp_crypto> for pjmedia_srtp_crypto {
    fn new() -> Self {
        Self {
            key: pj_str_t::new(),
            name: pj_str_t::new(),
            flags: 0,
        }
    }
}

impl AutoCreate<pjmedia_vid_stream_rc_config> for pjmedia_vid_stream_rc_config {
    fn new() -> Self {
        Self {
            method: 0,
            bandwidth: 0,
        }
    }
}

impl AutoCreate<pjmedia_vid_stream_sk_config> for pjmedia_vid_stream_sk_config {
    fn new() -> Self {
        Self {
            count: 0,
            interval: 0,
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_cap> for pjmedia_rtcp_fb_cap {
    fn new() -> Self {
        Self {
            codec_id: pj_str_t::new(),
            type_: 0,
            type_name: pj_str_t::new(),
            param: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_setting> for pjmedia_rtcp_fb_setting {
    fn new() -> Self {
        Self {
            dont_use_avpf: PJ_FALSE as pj_bool_t,
            cap_count: 0,
            caps: [
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
            ],
        }
    }
}

impl AutoCreate<pjmedia_audio_format_detail> for pjmedia_audio_format_detail {
    fn new() -> Self {
        Self {
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
    fn new() -> Self {
        Self { w: 0, h: 0 }
    }
}

impl AutoCreate<pjmedia_ratio> for pjmedia_ratio {
    fn new() -> Self {
        Self { num: 0, denum: 0 }
    }
}

impl AutoCreate<pjmedia_video_format_detail> for pjmedia_video_format_detail {
    fn new() -> Self {
        Self {
            size: pjmedia_rect_size::new(),
            fps: pjmedia_ratio::new(),
            avg_bps: 0,
            max_bps: 0,
        }
    }
}

impl AutoCreate<pjmedia_format__bindgen_ty_1> for pjmedia_format__bindgen_ty_1 {
    fn new() -> Self {
        let mut result = Self {
            aud: pjmedia_sys::__BindgenUnionField::<pjmedia_audio_format_detail>::default(),
            vid: pjmedia_sys::__BindgenUnionField::<pjmedia_video_format_detail>::default(),
            user: pjmedia_sys::__BindgenUnionField::<[::std::os::raw::c_char; 1usize]>::default(),
            bindgen_union_field: [0; 6]
        };

        unsafe {
            *result.aud.as_mut() = pjmedia_audio_format_detail::new();
            *result.vid.as_mut() = pjmedia_video_format_detail::new();
            *result.user.as_mut() = [0;1];
        }

        result
    }
}

impl AutoCreate<pjmedia_format> for pjmedia_format {
    fn new() -> Self {
        Self {
            id: 0,
            type_: 0,
            detail_type: 0,
            det: pjmedia_format__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjmedia_port_info> for pjmedia_port_info {
    fn new() -> Self {
        Self {
            name: pj_str_t::new(),
            signature: 0,
            dir: 0,
            fmt: pjmedia_format::new(),
        }
    }
}

impl AutoCreate<pjmedia_port_port_data> for pjmedia_port_port_data {
    fn new() -> Self {
        Self {
            pdata: ptr::null_mut(),
            ldata: 0,
        }
    }
}

impl AutoCreate<pjmedia_port> for pjmedia_port {
    fn new() -> Self {
        Self {
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
    fn new() -> Self {
        Self {
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
    fn new() -> Self {
        Self {
            name: [0; 32],
            type_: 0,
            op: &mut pjmedia_transport_op::new() as *mut _,
            user_data: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjmedia_tone_desc> for pjmedia_tone_desc {
    fn new () -> Self {
        Self {
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
    fn new () -> Self {
        Self {
            name: [0; 64],
            input_count: 0,
            output_count: 0,
            default_samples_per_sec: 0,
            driver: [0; 32],
            caps: 0,
            routes: 0,
            ext_fmt_cnt: 0,
            ext_fmt: [
                pjmedia_format::new(), pjmedia_format::new(),
                pjmedia_format::new(), pjmedia_format::new(),
                pjmedia_format::new(), pjmedia_format::new(),
                pjmedia_format::new(), pjmedia_format::new(),
            ]
        }
    }
}

impl AutoCreate<pjmedia_rtcp_fb_info> for pjmedia_rtcp_fb_info {
    fn new () -> Self {
        Self {
            cap_count: 0,
            caps: [
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
                pjmedia_rtcp_fb_cap::new(), pjmedia_rtcp_fb_cap::new(),
            ],
        }
    }
}

impl AutoCreate<pjmedia_codec_info> for pjmedia_codec_info {
    fn new () -> Self {
            Self {
            type_: 0,
            pt: 0,
            encoding_name: pj_str_t::new(),
            clock_rate: 0,
            channel_cnt: 0,
        }
    }
}

impl AutoCreate<pjmedia_stream_info> for pjmedia_stream_info {
    fn new () -> Self {
        Self {
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

impl AutoCreate<pjmedia_rtcp_stream_stat__bindgen_ty_1> for pjmedia_rtcp_stream_stat__bindgen_ty_1 {
    fn new () -> Self {
        Self {
            _bitfield_align_1: [0; 0],
            _bitfield_1: pjmedia_sys::__BindgenBitfieldUnit::<[u8; 1usize]>::default(),
            __bindgen_padding_0: [0; 3usize],
        }
    }
}

impl AutoCreate<pjmedia_rtcp_sdes> for pjmedia_rtcp_sdes {
    fn new () -> Self {
        Self {
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
    fn new () -> Self {
        Self {
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
    fn new () -> Self {
        Self {
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
    fn new () -> Self {
        Self {
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
    fn new () -> Self {
        Self {
            rtp_sock: 0,
            rtp_addr_name: pj_sockaddr::new(),
            rtcp_sock: 0,
            rtcp_addr_name: pj_sockaddr::new(),
        }
    }
}

impl AutoCreate<pjmedia_transport_specific_info> for pjmedia_transport_specific_info {
    fn new () -> Self {
        Self {
            type_: 0,
            cbsize: -1,
            buffer: [0; 288usize],
        }
    }
}

impl AutoCreate<pjmedia_transport_info> for pjmedia_transport_info {
    fn new () -> Self {
        Self {
            sock_info: pjmedia_sock_info::new(),
            src_rtp_name: pj_sockaddr::new(),
            src_rtcp_name: pj_sockaddr::new(),
            specific_info_cnt: 0,
            spc_info: [pjmedia_transport_specific_info::new(); 4usize],
        }
    }
}

impl AutoCreate<pjmedia_codec_param__bindgen_ty_1> for pjmedia_codec_param__bindgen_ty_1 {
    fn new () -> Self {
        Self {
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
    fn new () -> Self {
        Self {
            name: pj_str_t::new(),
            val: pj_str_t::new()
        }
    }
}

impl AutoCreate<pjmedia_codec_fmtp> for pjmedia_codec_fmtp {
    fn new() -> Self {
        Self {
            cnt: 0,
            param: [
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
                pjmedia_codec_fmtp_param::new(), pjmedia_codec_fmtp_param::new(),
            ],
        }
    }
}

impl AutoCreate<pjmedia_codec_param__bindgen_ty_2> for pjmedia_codec_param__bindgen_ty_2 {
    fn new() -> Self {
        Self {
            frm_per_pkt: 0,
            _bitfield_align_1: [0; 0],
            _bitfield_1: __BindgenBitfieldUnit::new([0; 1usize]),
            enc_fmtp: pjmedia_codec_fmtp::new(),
            dec_fmtp: pjmedia_codec_fmtp::new(),
        }
    }
}

impl AutoCreate<pjmedia_codec_param> for pjmedia_codec_param {
    fn new () -> Self {
        Self {
            info: pjmedia_codec_param__bindgen_ty_1::new(),
            setting: pjmedia_codec_param__bindgen_ty_2::new()
        }
    }
}

impl AutoCreate<pjmedia_vid_codec_info> for pjmedia_vid_codec_info {
    fn new() -> Self {
        Self {
            fmt_id: 0,
            pt: 0,
            encoding_name: pj_str_t::new(),
            encoding_desc: pj_str_t::new(),
            clock_rate: 0,
            dir: 0,
            dec_fmt_id_cnt: 0,
            dec_fmt_id: [0; 8usize],
            packings: 0,
            fps_cnt: 0,
            fps: [pjmedia_ratio::new(); 16usize],
        }
    }
}

impl AutoCreate<pjmedia_vid_stream_info> for pjmedia_vid_stream_info {
    fn new() -> Self {
        Self {
            type_: 0,
            proto: 0,
            dir: 0,
            rem_addr: pj_sockaddr::new(),
            rem_rtcp: pj_sockaddr::new(),
            rtcp_mux: 0,
            loc_rtcp_fb: pjmedia_rtcp_fb_info::new(),
            rem_rtcp_fb: pjmedia_rtcp_fb_info::new(),
            tx_pt: 0,
            rx_pt: 0,
            ssrc: 0,
            cname: pj_str_t::new(),
            has_rem_ssrc: 0,
            rem_ssrc: 0,
            rem_cname: pj_str_t::new(),
            rtp_ts: 0,
            rtp_seq: 0,
            rtp_seq_ts_set: 0,
            jb_init: 0,
            jb_min_pre: 0,
            jb_max_pre: 0,
            jb_max: 0,
            codec_info: pjmedia_vid_codec_info::new(),
            codec_param: ptr::null_mut(),
            rtcp_sdes_bye_disabled: 0,
            rc_cfg: pjmedia_vid_stream_rc_config::new(),
            sk_cfg: pjmedia_vid_stream_sk_config::new(),
        }
    }
}
