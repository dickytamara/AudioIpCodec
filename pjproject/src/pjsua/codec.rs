
use std::convert::TryFrom;
use crate::utils::check_boolean;

use super::*;

// use pj_sys::pj_sockaddr;

pub trait CodecInfoExt {
    /// Codec unique identification.
    fn get_codec_id (&self) -> String;

    /// Codec priority (integer 0-255).
    fn get_priority (&self) -> u8;

    ///Codec description.
    fn get_desc (&self) -> String;

    // fn get_buf_ (&self) -> [::std::os::raw::c_char; 64usize],
}

pub trait MediaStreamInfoExt {

    /// Media type (audio, video)
    fn get_type_ (&self) -> MediaType;

    /// Transport protocol (RTP/AVP, etc.)
    fn get_proto (&self) -> MediaTransportProtocol;

    /// Media direction.
    fn get_dir (&self) -> MediaDirection;

    // fn get_rem_addr (&self) -> pj_sockaddr;
    // fn get_rem_rtcp (&self) -> pj_sockaddr;

    /// Use RTP and RTCP multiplexing.
    fn get_rtcp_mux (&self) -> bool;

    /// Local RTCP-FB info.
    fn get_loc_rtcp_fb (&self) -> &RtcpFbInfo;

    /// Remote RTCP-FB info.
    fn get_rem_rtcp_fb (&self) -> &RtcpFbInfo;

    /// Incoming codec format info.
    fn get_fmt (&self) -> &MediaCodecInfo;

    /// Optional codec param.
    fn get_param (&self) -> *mut MediaCodecParam;

    /// Outgoing codec paylaod type.
    fn get_tx_pt (&self) -> u32;

    /// Incoming codec paylaod type.
    fn get_rx_pt (&self) -> u32;

    /// Outgoing codec max ptime.
    fn get_tx_maxptime (&self) -> u32;

    /// Outgoing pt for telephone-events.
    fn get_tx_event_pt (&self) -> i32;

    /// Incoming pt for telephone-events.
    fn get_rx_event_pt (&self) -> i32;

    /// RTP SSRC.
    fn get_ssrc (&self) -> u32;

    /// Remote RTCP CNAME.
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
        self.codec_id.to_string()
    }

    fn get_priority (&self) -> u8 {
        self.priority
    }

    fn get_desc (&self) -> String {
        self.desc.to_string()
    }
}

impl MediaStreamInfoExt for MediaStreamInfo {

    fn get_type_ (&self) -> MediaType {
        MediaType::try_from(self.type_)
        .expect("Error MediaStreamInfo get type_")
    }

    fn get_proto (&self) -> MediaTransportProtocol {
        MediaTransportProtocol::try_from(self.proto)
        .expect("Error MediaStreamInfo get proto")
    }

    fn get_dir (&self) -> MediaDirection {
        MediaDirection::try_from(self.dir)
        .expect("Error MediaStreamInfo get dir")
    }

    // fn get_rem_addr (&self) -> pj_sockaddr {
    //     todo!()
    // }

    // fn get_rem_rtcp (&self) -> pj_sockaddr {
    //     todo!()
    // }

    fn get_rtcp_mux (&self) -> bool {
        check_boolean(self.rtcp_mux)
    }

    fn get_loc_rtcp_fb (&self) -> &RtcpFbInfo {
        &self.loc_rtcp_fb
    }

    fn get_rem_rtcp_fb (&self) -> &RtcpFbInfo {
        &self.rem_rtcp_fb
    }

    fn get_fmt (&self) -> &MediaCodecInfo {
        &self.fmt
    }

    fn get_param (&self) -> *mut MediaCodecParam {
        self.param
    }

    fn get_tx_pt (&self) -> u32 {
        self.tx_pt
    }

    fn get_rx_pt (&self) -> u32 {
        self.rx_pt
    }

    fn get_tx_maxptime (&self) -> u32 {
        self.tx_maxptime
    }

    fn get_tx_event_pt (&self) -> i32 {
        self.tx_event_pt
    }

    fn get_rx_event_pt (&self) -> i32 {
        self.rx_event_pt
    }

    fn get_ssrc (&self) -> u32 {
        self.ssrc
    }

    fn get_cname (&self) -> String {
        self.cname.to_string()
    }

    fn get_has_rem_ssrc (&self) -> bool {
        check_boolean(self.has_rem_ssrc)
    }

    fn get_rem_ssrc (&self) -> u32 {
        self.rem_ssrc
    }

    fn get_rem_cname (&self) -> String {
        self.rem_cname.to_string()
    }

    fn get_rtp_ts (&self) -> u32 {
        self.rtp_ts
    }

    fn get_rtp_seq (&self) -> u16 {
        self.rtp_seq
    }

    fn get_rtp_seq_ts_set (&self) -> u8 {
        self.rtp_seq_ts_set
    }

    fn get_jb_init (&self) -> i32 {
        self.jb_init
    }

    fn get_jb_min_pre (&self) -> i32 {
        self.jb_min_pre
    }

    fn get_jb_max_pre (&self) -> i32 {
        self.jb_max_pre
    }

    fn get_jb_max (&self) -> i32 {
        self.jb_max
    }

    fn get_jb_discard_algo (&self) -> JbDiscardAlgo {
        JbDiscardAlgo::try_from(self.jb_discard_algo)
        .expect("Error MediaStreamInfo get jb_discard_algo")
    }

    fn get_rtcp_sdes_bye_disabled (&self) -> bool {
        check_boolean(self.rtcp_sdes_bye_disabled)
    }
}