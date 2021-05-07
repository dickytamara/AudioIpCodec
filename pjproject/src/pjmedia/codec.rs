use std::convert::TryFrom;
// use crate::pjmedia::*;
use crate::utils::check_boolean;

use super::*;


pub trait MediaStreamInfoExt {

    /// Media type (audio, video)
    fn get_type_ (&self) -> MediaType;

    /// Transport protocol (RTP/AVP, etc.)
    fn get_proto (&self) -> MediaTpProto;

    /// Media direction.
    fn get_dir (&self) -> MediaDir;

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

    fn get_jb_discard_algo (&self) -> MediaJbDiscardAlgo;

    fn get_rtcp_sdes_bye_disabled (&self) -> bool;
}


impl MediaStreamInfoExt for MediaStreamInfo {

    fn get_type_ (&self) -> MediaType {
        MediaType::try_from(self.type_)
        .expect("Error MediaStreamInfo get type_")
    }

    fn get_proto (&self) -> MediaTpProto {
        MediaTpProto::try_from(self.proto)
        .expect("Error MediaStreamInfo get proto")
    }

    fn get_dir (&self) -> MediaDir {
        MediaDir::try_from(self.dir)
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

    fn get_jb_discard_algo (&self) -> MediaJbDiscardAlgo {
        MediaJbDiscardAlgo::try_from(self.jb_discard_algo)
        .expect("Error MediaStreamInfo get jb_discard_algo")
    }

    fn get_rtcp_sdes_bye_disabled (&self) -> bool {
        check_boolean(self.rtcp_sdes_bye_disabled)
    }
}


// pub trait RtcpFbSettingExt {

//     /// Specify whether transport protocol in SDP media description uses RTP/AVP instead
//     /// of RTP/AVPF. Note that RFC4585 mandates to signal RTP/AVPF profile, but it may
//     /// cause SDP negotiation failure when negotiating with endpoints that does not
//     /// support RTP/AVPF (including older version of PJSIP), furthermore, there is RFC8643
//     /// that promotes interoperability over the strictness of RTP profile specifications.
//     ///
//     /// # default
//     /// true
//     fn set_dont_use_avpf (&mut self, value: bool);
//     fn get_dont_use_avpf (&self) -> bool;

//     /// Number of RTCP Feedback capabilities.
//     fn set_cap_count (&mut self, value: u32);
//     fn get_cap_count (&self) -> u32;

//     /// The RTCP Feedback capabilities.
//     fn set_caps (&mut self, value: [RtcpFbCapability; 16usize]);
//     fn get_caps (&self) -> &[RtcpFbCapability; 16usize];
// }


// pub trait RtcpFbCapabilityExt {

//     /// Specify the codecs to which the capability is applicable. Codec ID is using the same
//     /// format as in pjmedia_codec_mgr_find_codecs_by_id() and
//     /// pjmedia_vid_codec_mgr_find_codecs_by_id(), e.g: "L16/8000/1", "PCMU", "H264".
//     /// This can also be an asterisk ("*") to represent all codecs.
//     fn set_codec_id (&mut self, value: String);
//     fn get_codec_id (&self) -> String ;

//     /// Specify the RTCP Feedback type.
//     fn set_type_ (&mut self, value: MediaRtcpFbType);
//     fn get_type_ (&self) -> MediaRtcpFbType;

//     /// Specify the type name if RTCP Feedback type is PJMEDIA_RTCP_FB_OTHER.
//     fn set_type_name (&mut self, value: String);
//     fn get_type_name (&self) -> String;

//     /// Specify the RTCP Feedback parameters. Feedback subtypes should be specified in this field, e.g:
//     ///
//     /// - 'pli' for Picture Loss Indication feedback,
//     /// - 'sli' for Slice Loss Indication feedback,
//     /// - 'rpsi' for Reference Picture Selection Indication feedback,
//     /// - 'app' for specific/proprietary application layer feedback.
//     fn set_param (&mut self, value: String);
//     fn get_param (&self) -> String;
// }


// impl RtcpFbSettingExt for RtcpFbSetting {

//     fn set_dont_use_avpf (&mut self, value: bool) {
//         self.dont_use_avpf = boolean_to_pjbool(value);
//     }

//     fn get_dont_use_avpf (&self) -> bool {
//         check_boolean(self.dont_use_avpf)
//     }

//     fn set_cap_count (&mut self, value: u32) {
//         self.cap_count = value;
//     }

//     fn get_cap_count (&self) -> u32 {
//         self.cap_count
//     }

//     fn set_caps (&mut self, value: [RtcpFbCapability; 16usize]) {
//         self.caps = value;
//     }

//     fn get_caps (&self) -> &[RtcpFbCapability; 16usize] {
//         &self.caps
//     }
// }


// impl RtcpFbCapabilityExt for RtcpFbCapability {

//     fn set_codec_id (&mut self, value: String) {
//         self.codec_id = pj_str_t::from_string(value);
//     }

//     fn get_codec_id (&self) -> String  {
//         self.codec_id.to_string()
//     }

//     fn set_type_ (&mut self, value: MediaRtcpFbType) {
//         self.type_ = value.into();
//     }

//     fn get_type_ (&self) -> MediaRtcpFbType {
//         MediaRtcpFbType::try_from(self.type_)
//         .expect("Error RtcpFbCapability get type_")
//     }

//     fn set_type_name (&mut self, value: String) {
//         self.type_name = pj_str_t::from_string(value);
//     }

//     fn get_type_name (&self) -> String {
//         self.type_name.to_string()
//     }

//     fn set_param (&mut self, value: String) {
//         self.param = pj_str_t::from_string(value);
//     }

//     fn get_param (&self) -> String {
//         self.param.to_string()
//     }
// }