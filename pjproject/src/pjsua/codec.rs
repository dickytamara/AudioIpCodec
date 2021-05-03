// use pj_sys::pj_sockaddr;
use super::*;

pub trait CodecInfoExt {
    fn get_codec_id (&self) -> String;
    fn get_priority (&self) -> u8;
    fn get_desc (&self) -> String;
    // fn get_buf_ (&self) -> [::std::os::raw::c_char; 64usize],
}

pub trait StreamInfoExt {
    fn get_type_ (&self) -> MediaType;
    fn get_proto (&self) -> MediaTransportProtocol;
    fn get_dir (&self) -> MediaDirection;
    // fn get_rem_addr (&self) -> pj_sockaddr;
    // fn get_rem_rtcp (&self) -> pj_sockaddr;
    fn get_rtcp_mux (&self) -> bool;
    fn get_loc_rtcp_fb (&self) -> RtcpFbInfo;
    fn get_rem_rtcp_fb (&self) -> RtcpFbInfo;
    fn get_fmt (&self) -> CodecInfo;
    fn get_param (&self) -> *mut pjmedia_codec_param;
    fn get_tx_pt (&self) -> u32;
    fn get_rx_pt (&self) -> u32;
    fn get_tx_maxptime (&self) -> u32;
    fn get_tx_event_pt (&self) -> i32;
    fn get_rx_event_pt (&self) -> i32;
    fn get_ssrc (&self) -> u32;
    fn get_cname (&self) -> String;
    fn get_has_rem_ssrc (&self) -> bool;
    fn get_rem_ssrc (&self) -> u32;
    fn get_rem_cname (&self) -> String;
    fn get_rtp_ts (&self) -> u32;
    fn get_rtp_seq (&self) -> u16;
    fn get_rtp_seq_ts_set (&self) -> u8;
    fn get_jb_init (&self) -> i32;
    fn get_jb_min_pre (&self) -> i32;
    fn get_jb_max_pre (&self) -> i32;
    fn get_jb_max (&self) -> i32;
    fn get_jb_discard_algo (&self) -> JbDiscardAlgo;
    fn get_rtcp_sdes_bye_disabled (&self) -> bool;
}

impl CodecInfoExt for CodecInfo {
    fn get_codec_id (&self) -> String {
        todo!()
    }

    fn get_priority (&self) -> u8 {
        todo!()
    }

    fn get_desc (&self) -> String {
        todo!()
    }
}

impl StreamInfoExt for StreamInfo {
    fn get_type_ (&self) -> MediaType {
        todo!()
    }

    fn get_proto (&self) -> MediaTransportProtocol {
        todo!()
    }

    fn get_dir (&self) -> MediaDirection {
        todo!()
    }

    // fn get_rem_addr (&self) -> pj_sockaddr {
    //     todo!()
    // }

    // fn get_rem_rtcp (&self) -> pj_sockaddr {
    //     todo!()
    // }

    fn get_rtcp_mux (&self) -> bool {
        todo!()
    }

    fn get_loc_rtcp_fb (&self) -> RtcpFbInfo {
        todo!()
    }

    fn get_rem_rtcp_fb (&self) -> RtcpFbInfo {
        todo!()
    }

    fn get_fmt (&self) -> CodecInfo {
        todo!()
    }

    fn get_param (&self) -> *mut pjmedia_codec_param {
        todo!()
    }

    fn get_tx_pt (&self) -> u32 {
        todo!()
    }

    fn get_rx_pt (&self) -> u32 {
        todo!()
    }

    fn get_tx_maxptime (&self) -> u32 {
        todo!()
    }

    fn get_tx_event_pt (&self) -> i32 {
        todo!()
    }

    fn get_rx_event_pt (&self) -> i32 {
        todo!()
    }

    fn get_ssrc (&self) -> u32 {
        todo!()
    }

    fn get_cname (&self) -> String {
        todo!()
    }

    fn get_has_rem_ssrc (&self) -> bool {
        todo!()
    }

    fn get_rem_ssrc (&self) -> u32 {
        todo!()
    }

    fn get_rem_cname (&self) -> String {
        todo!()
    }

    fn get_rtp_ts (&self) -> u32 {
        todo!()
    }

    fn get_rtp_seq (&self) -> u16 {
        todo!()
    }

    fn get_rtp_seq_ts_set (&self) -> u8 {
        todo!()
    }

    fn get_jb_init (&self) -> i32 {
        todo!()
    }

    fn get_jb_min_pre (&self) -> i32 {
        todo!()
    }

    fn get_jb_max_pre (&self) -> i32 {
        todo!()
    }

    fn get_jb_max (&self) -> i32 {
        todo!()
    }

    fn get_jb_discard_algo (&self) -> JbDiscardAlgo {
        todo!()
    }

    fn get_rtcp_sdes_bye_disabled (&self) -> bool {
        todo!()
    }
}