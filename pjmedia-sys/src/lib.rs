#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate pj_sys;
use pj_sys::*;

use std::os::raw::{c_void, c_uint, c_char, c_int, c_long, c_short};

// pub const PJMEDIA_POOL_LEN_ENDPT: u32 = 512;
// pub const PJMEDIA_POOL_INC_ENDPT: u32 = 512;
// pub const PJMEDIA_POOL_LEN_EVTMGR: u32 = 500;
// pub const PJMEDIA_POOL_INC_EVTMGR: u32 = 500;
// pub const PJMEDIA_CONF_USE_SWITCH_BOARD: u32 = 0;
// pub const PJMEDIA_CONF_USE_AGC: u32 = 1;

// pub const PJMEDIA_SND_DEFAULT_REC_LATENCY: u32 = 100;
// pub const PJMEDIA_SND_DEFAULT_PLAY_LATENCY: u32 = 140;
// pub const PJMEDIA_SOUND_BUFFER_COUNT: u32 = 8;

// pub const PJMEDIA_WSOLA_IMP_NULL: u32 = 0;
// pub const PJMEDIA_WSOLA_IMP_WSOLA: u32 = 1;
// pub const PJMEDIA_WSOLA_IMP_WSOLA_LITE: u32 = 2;
// pub const PJMEDIA_WSOLA_IMP: u32 = 1;
// pub const PJMEDIA_WSOLA_MAX_EXPAND_MSEC: u32 = 80;
// pub const PJMEDIA_WSOLA_TEMPLATE_LENGTH_MSEC: u32 = 5;
// pub const PJMEDIA_WSOLA_DELAY_MSEC: u32 = 5;
// pub const PJMEDIA_WSOLA_PLC_NO_FADING: u32 = 0;
// pub const PJMEDIA_MAX_PLC_DURATION_MSEC: u32 = 240;

// pub const PJMEDIA_RESAMPLE_NONE: u32 = 1;
// pub const PJMEDIA_RESAMPLE_LIBRESAMPLE: u32 = 2;
// pub const PJMEDIA_RESAMPLE_SPEEX: u32 = 3;
// pub const PJMEDIA_RESAMPLE_LIBSAMPLERATE: u32 = 4;
// pub const PJMEDIA_RESAMPLE_IMP: u32 = 2;
// pub const PJMEDIA_FILE_PORT_BUFSIZE: u32 = 4000;
// pub const PJMEDIA_MAX_FRAME_DURATION_MS: u32 = 200;
// pub const PJMEDIA_MAX_MTU: u32 = 1500;
// pub const PJMEDIA_MAX_MRU: u32 = 2000;
// pub const PJMEDIA_DTMF_DURATION: u32 = 1600;
// pub const PJMEDIA_DTMF_DURATION_MSEC: u32 = 0;
// pub const PJMEDIA_RTP_NAT_PROBATION_CNT: u32 = 10;
// pub const PJMEDIA_RTCP_NAT_PROBATION_CNT: u32 = 3;
// pub const PJMEDIA_ADVERTISE_RTCP: u32 = 1;

// pub const PJMEDIA_RTCP_INTERVAL: u32 = 5000;
// pub const PJMEDIA_RTCP_FB_INTERVAL: u32 = 50;
// pub const PJMEDIA_RTCP_IGNORE_FIRST_PACKETS: u32 = 25;
// pub const PJMEDIA_RTCP_STAT_HAS_RAW_JITTER: u32 = 0;
// pub const PJMEDIA_RTCP_NORMALIZE_FACTOR: u32 = 3;
// pub const PJMEDIA_RTCP_STAT_HAS_IPDV: u32 = 0;

// pub const PJMEDIA_STREAM_ENABLE_XR: u32 = 0;
// pub const PJMEDIA_RTCP_RX_SDES_BUF_LEN: u32 = 64;
// pub const PJMEDIA_RTCP_FB_MAX_CAP: u32 = 16;
// pub const PJMEDIA_STREAM_VAD_SUSPEND_MSEC: u32 = 600;
// pub const PJMEDIA_STREAM_CHECK_RTP_PT: u32 = 1;
// pub const PJMEDIA_STREAM_RESV_PAYLOAD_LEN: u32 = 20;
// pub const PJMEDIA_CODEC_MAX_SILENCE_PERIOD: u32 = 5000;
// pub const PJMEDIA_SILENCE_DET_THRESHOLD: u32 = 4;
// pub const PJMEDIA_SILENCE_DET_MAX_THRESHOLD: u32 = 65536;

// pub const PJMEDIA_SPEEX_AEC_USE_AGC: u32 = 1;
// pub const PJMEDIA_SPEEX_AEC_USE_DENOISE: u32 = 1;

// pub const PJMEDIA_WEBRTC_AEC_USE_MOBILE: u32 = 0;
// pub const PJMEDIA_CODEC_MAX_FMTP_CNT: u32 = 16;
// pub const PJMEDIA_SDP_NEG_PREFER_REMOTE_CODEC_ORDER: u32 = 1;
// pub const PJMEDIA_SDP_NEG_ANSWER_MULTIPLE_CODECS: u32 = 0;
// pub const PJMEDIA_SDP_NEG_MAX_CUSTOM_FMT_NEG_CB: u32 = 8;
// pub const PJMEDIA_SDP_NEG_ANSWER_SYMMETRIC_PT: u32 = 1;
// pub const PJMEDIA_SDP_NEG_COMPARE_BEFORE_INC_VERSION: u32 = 0;

// pub const PJMEDIA_ADD_BANDWIDTH_TIAS_IN_SDP: u32 = 1;
// pub const PJMEDIA_ADD_RTPMAP_FOR_STATIC_PT: u32 = 1;
// pub const PJMEDIA_RTP_PT_TELEPHONE_EVENTS: u32 = 120;
// pub const PJMEDIA_TELEPHONE_EVENT_ALL_CLOCKRATES: u32 = 1;
// pub const PJMEDIA_TONEGEN_MAX_DIGITS: u32 = 32;
// pub const PJMEDIA_TONEGEN_SINE: u32 = 1;
// pub const PJMEDIA_TONEGEN_FLOATING_POINT: u32 = 2;
// pub const PJMEDIA_TONEGEN_FIXED_POINT_CORDIC: u32 = 3;
// pub const PJMEDIA_TONEGEN_FAST_FIXED_POINT: u32 = 4;
// pub const PJMEDIA_TONEGEN_ALG: u32 = 2;
// pub const PJMEDIA_TONEGEN_FIXED_POINT_CORDIC_LOOP: u32 = 10;
// pub const PJMEDIA_TONEGEN_FADE_IN_TIME: u32 = 1;
// pub const PJMEDIA_TONEGEN_FADE_OUT_TIME: u32 = 2;
// pub const PJMEDIA_TONEGEN_VOLUME: u32 = 12288;

// pub const PJMEDIA_SRTP_HAS_SDES: u32 = 1;
// pub const PJMEDIA_SRTP_HAS_DTLS: u32 = 0;
// pub const PJMEDIA_SRTP_DTLS_OSSL_CIPHERS: &'static [u8; 8usize] = b"DEFAULT\0";
// pub const PJMEDIA_SRTP_MAX_CRYPTOS: u32 = 16;
// pub const PJMEDIA_SRTP_HAS_AES_CM_256: u32 = 1;
// pub const PJMEDIA_SRTP_HAS_AES_CM_192: u32 = 0;
// pub const PJMEDIA_SRTP_HAS_AES_CM_128: u32 = 1;
// pub const PJMEDIA_SRTP_HAS_AES_GCM_256: u32 = 0;
// pub const PJMEDIA_SRTP_HAS_AES_GCM_128: u32 = 0;
// pub const PJMEDIA_LIBSRTP_AUTO_INIT_DEINIT: u32 = 1;
// pub const PJMEDIA_HANDLE_G722_MPEG_BUG: u32 = 1;
// pub const PJMEDIA_TRANSPORT_SWITCH_REMOTE_ADDR: u32 = 1;
// pub const PJMEDIA_TRANSPORT_SPECIFIC_INFO_MAXCNT: u32 = 4;
// pub const PJMEDIA_STREAM_KA_EMPTY_RTP: u32 = 1;
// pub const PJMEDIA_STREAM_KA_USER: u32 = 2;
// pub const PJMEDIA_STREAM_ENABLE_KA: u32 = 0;
// pub const PJMEDIA_STREAM_KA_INTERVAL: u32 = 5;
// pub const PJMEDIA_STREAM_START_KA_CNT: u32 = 2;
// pub const PJMEDIA_STREAM_START_KA_INTERVAL_MSEC: u32 = 1000;
// pub const PJMEDIA_IGNORE_RECV_ERR_CNT: u32 = 20;

// pub const PJMEDIA_MAX_VIDEO_PLANES: u32 = 4;
// pub const PJMEDIA_MAX_VIDEO_FORMATS: u32 = 32;
// pub const PJMEDIA_CLOCK_SYNC_MAX_SYNC_MSEC: u32 = 20000;
// pub const PJMEDIA_MAX_VIDEO_ENC_FRAME_SIZE: u32 = 131072;
// pub const PJMEDIA_CLOCK_SYNC_MAX_RESYNC_DURATION: u32 = 2000;
// pub const PJMEDIA_JBUF_DISC_MIN_GAP: u32 = 200;
// pub const PJMEDIA_JBUF_PRO_DISC_MIN_BURST: u32 = 1;
// pub const PJMEDIA_JBUF_PRO_DISC_MAX_BURST: u32 = 100;
// pub const PJMEDIA_JBUF_PRO_DISC_T1: u32 = 2000;
// pub const PJMEDIA_JBUF_PRO_DISC_T2: u32 = 10000;
// pub const PJMEDIA_STREAM_SOFT_START: u32 = 1;
// pub const PJMEDIA_VID_STREAM_SKIP_PACKETS_TO_REDUCE_LATENCY: u32 = 0;
// pub const PJMEDIA_MAX_VID_PAYLOAD_SIZE: u32 = 1336;
// pub const PJMEDIA_TRANSPORT_SO_RCVBUF_SIZE: u32 = 0;
// pub const PJMEDIA_TRANSPORT_SO_SNDBUF_SIZE: u32 = 0;

// pub const PJMEDIA_VID_STREAM_START_KEYFRAME_CNT: u32 = 5;
// pub const PJMEDIA_VID_STREAM_START_KEYFRAME_INTERVAL_MSEC: u32 = 1000;
// pub const PJMEDIA_VID_STREAM_MIN_KEYFRAME_INTERVAL_MSEC: u32 = 1000;
// pub const PJMEDIA_VID_STREAM_DECODE_MIN_DELAY_MSEC: u32 = 100;
// pub const PJMEDIA_AUD_DEV_INFO_NAME_LEN: u32 = 64;

// pub const PJMEDIA_AUDIO_DEV_HAS_PORTAUDIO: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_OPENSL: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_BB10: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_ALSA: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_NULL_AUDIO: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_COREAUDIO: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_WMME: u32 = 1;
// pub const PJMEDIA_AUDIO_DEV_HAS_WASAPI: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_BDIMAD: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_HAS_SYMB_APS: u32 = 0;

// pub const PJMEDIA_AUDIO_DEV_SYMB_APS_DETECTS_CODEC: u32 = 1;
// pub const PJMEDIA_AUDIO_DEV_HAS_SYMB_VAS: u32 = 0;
// pub const PJMEDIA_AUDIO_DEV_SYMB_VAS_VERSION: u32 = 1;
// pub const PJMEDIA_AUDIO_DEV_MDA_USE_SYNC_START: u32 = 1;
// pub const PJMEDIA_AUDIO_DEV_HAS_LEGACY_DEVICE: u32 = 0;
// pub const PJMEDIA_AUDIODEV_ERRNO_START: u32 = 420000;
// pub const PJMEDIA_AUDIODEV_ERRNO_END: u32 = 469999;
// pub const PJMEDIA_AUDIODEV_PORTAUDIO_ERRNO_START: u32 = 459999;
// pub const PJMEDIA_AUDIODEV_PORTAUDIO_ERRNO_END: u32 = 469998;
// pub const PJMEDIA_AUDIODEV_WMME_IN_ERROR_START: u32 = 450000;
// pub const PJMEDIA_AUDIODEV_WMME_IN_ERROR_END: u32 = 450999;
// pub const PJMEDIA_AUDIODEV_WMME_OUT_ERROR_START: u32 = 451999;
// pub const PJMEDIA_AUDIODEV_WMME_OUT_ERROR_END: u32 = 452999;
// pub const PJMEDIA_AUDIODEV_COREAUDIO_ERRNO_START: u32 = 440000;
// pub const PJMEDIA_AUDIODEV_COREAUDIO_ERRNO_END: u32 = 459999;
// pub const PJMEDIA_AUDIODEV_BDIMAD_ERROR_START: u32 = 460000;
// pub const PJMEDIA_AUDIODEV_BDIMAD_ERROR_END: u32 = 461999;

// pub const PJMEDIA_EAUD_ERR: u32 = 420001;
// pub const PJMEDIA_EAUD_SYSERR: u32 = 420002;
// pub const PJMEDIA_EAUD_INIT: u32 = 420003;
// pub const PJMEDIA_EAUD_INVDEV: u32 = 420004;
// pub const PJMEDIA_EAUD_NODEV: u32 = 420005;
// pub const PJMEDIA_EAUD_NODEFDEV: u32 = 420006;
// pub const PJMEDIA_EAUD_NOTREADY: u32 = 420007;
// pub const PJMEDIA_EAUD_INVCAP: u32 = 420008;
// pub const PJMEDIA_EAUD_INVOP: u32 = 420009;
// pub const PJMEDIA_EAUD_BADFORMAT: u32 = 420010;
// pub const PJMEDIA_EAUD_SAMPFORMAT: u32 = 420011;
// pub const PJMEDIA_EAUD_BADLATENCY: u32 = 420012;
// pub const PJMEDIA_EAUD_WASAPI_ERROR: u32 = 420013;
// pub const PJMEDIA_FORMAT_DETAIL_USER_SIZE: u32 = 1;
pub const PJMEDIA_AUD_DEFAULT_CAPTURE_DEV: i32 = -1;
pub const PJMEDIA_AUD_DEFAULT_PLAYBACK_DEV: i32 = -2;
// pub const PJMEDIA_AUD_INVALID_DEV: i32 = -3;
// pub const PJMEDIA_AUD_MAX_DRIVERS: u32 = 16;
// pub const PJMEDIA_AUD_MAX_DEVS: u32 = 64;
// pub const PJMEDIA_RTP_DTMF_EVENT_END_MASK: u32 = 128;
// pub const PJMEDIA_RTP_DTMF_EVENT_VOLUME_MASK: u32 = 63;
// pub const PJMEDIA_MAX_SDP_FMT: u32 = 32;
// pub const PJMEDIA_MAX_SDP_BANDW: u32 = 4;
// pub const PJMEDIA_MAX_SDP_ATTR: u32 = 68;
// pub const PJMEDIA_MAX_SDP_MEDIA: u32 = 16;
// pub const PJMEDIA_VID_DEV_INFO_FMT_CNT: u32 = 64;
// pub const PJMEDIA_VID_DEV_MAX_DRIVERS: u32 = 8;
// pub const PJMEDIA_VID_DEV_MAX_DEVS: u32 = 16;
// pub const PJMEDIA_VIDEODEV_ERRNO_START: u32 = 520000;
// pub const PJMEDIA_VIDEODEV_ERRNO_END: u32 = 569999;
// pub const PJMEDIA_EVID_ERR: u32 = 520001;
// pub const PJMEDIA_EVID_SYSERR: u32 = 520002;
// pub const PJMEDIA_EVID_INIT: u32 = 520003;
// pub const PJMEDIA_EVID_INVDEV: u32 = 520004;
// pub const PJMEDIA_EVID_NODEV: u32 = 520005;
// pub const PJMEDIA_EVID_NODEFDEV: u32 = 520006;
// pub const PJMEDIA_EVID_NOTREADY: u32 = 520007;
// pub const PJMEDIA_EVID_INVCAP: u32 = 520008;
// pub const PJMEDIA_EVID_INVOP: u32 = 520009;
// pub const PJMEDIA_EVID_BADFORMAT: u32 = 520010;
// pub const PJMEDIA_EVID_SAMPFORMAT: u32 = 520011;
// pub const PJMEDIA_EVID_BADLATENCY: u32 = 520012;
// pub const PJMEDIA_EVID_BADSIZE: u32 = 520013;
// pub const PJMEDIA_CODEC_MGR_MAX_CODECS: u32 = 32;
// pub const PJMEDIA_ECHO_STAT_NOT_SPECIFIED: u32 = 999999;
// pub const PJMEDIA_ERRNO_START: u32 = 220000;
// pub const PJMEDIA_ERRNO_END: u32 = 269999;
// pub const PJMEDIA_PORTAUDIO_ERRNO_START: u32 = 259999;
// pub const PJMEDIA_PORTAUDIO_ERRNO_END: u32 = 269998;
// pub const PJMEDIA_LIBSRTP_ERRNO_START: u32 = 259799;
// pub const PJMEDIA_LIBSRTP_ERRNO_END: u32 = 259998;
// pub const PJMEDIA_ERROR: u32 = 220001;
// pub const PJMEDIA_SDP_EINSDP: u32 = 220020;
// pub const PJMEDIA_SDP_EINVER: u32 = 220021;
// pub const PJMEDIA_SDP_EINORIGIN: u32 = 220022;
// pub const PJMEDIA_SDP_EINTIME: u32 = 220023;
// pub const PJMEDIA_SDP_EINNAME: u32 = 220024;
// pub const PJMEDIA_SDP_EINCONN: u32 = 220025;
// pub const PJMEDIA_SDP_EMISSINGCONN: u32 = 220026;
// pub const PJMEDIA_SDP_EINATTR: u32 = 220027;
// pub const PJMEDIA_SDP_EINRTPMAP: u32 = 220028;
// pub const PJMEDIA_SDP_ERTPMAPTOOLONG: u32 = 220029;
// pub const PJMEDIA_SDP_EMISSINGRTPMAP: u32 = 220030;
// pub const PJMEDIA_SDP_EINMEDIA: u32 = 220031;
// pub const PJMEDIA_SDP_ENOFMT: u32 = 220032;
// pub const PJMEDIA_SDP_EINPT: u32 = 220033;
// pub const PJMEDIA_SDP_EINFMTP: u32 = 220034;
// pub const PJMEDIA_SDP_EINRTCP: u32 = 220035;
// pub const PJMEDIA_SDP_EINPROTO: u32 = 220036;
// pub const PJMEDIA_SDP_EINBANDW: u32 = 220037;
// pub const PJMEDIA_SDP_EINSSRC: u32 = 220038;
// pub const PJMEDIA_SDPNEG_EINSTATE: u32 = 220040;
// pub const PJMEDIA_SDPNEG_ENOINITIAL: u32 = 220041;
// pub const PJMEDIA_SDPNEG_ENOACTIVE: u32 = 220042;
// pub const PJMEDIA_SDPNEG_ENONEG: u32 = 220043;
// pub const PJMEDIA_SDPNEG_EMISMEDIA: u32 = 220044;
// pub const PJMEDIA_SDPNEG_EINVANSMEDIA: u32 = 220045;
// pub const PJMEDIA_SDPNEG_EINVANSTP: u32 = 220046;
// pub const PJMEDIA_SDPNEG_EANSNOMEDIA: u32 = 220047;
// pub const PJMEDIA_SDPNEG_ENOMEDIA: u32 = 220048;
// pub const PJMEDIA_SDPNEG_NOANSCODEC: u32 = 220049;
// pub const PJMEDIA_SDPNEG_NOANSTELEVENT: u32 = 220050;
// pub const PJMEDIA_SDPNEG_NOANSUNKNOWN: u32 = 220051;
// pub const PJMEDIA_SDP_EMEDIANOTEQUAL: u32 = 220060;
// pub const PJMEDIA_SDP_EPORTNOTEQUAL: u32 = 220061;
// pub const PJMEDIA_SDP_ETPORTNOTEQUAL: u32 = 220062;
// pub const PJMEDIA_SDP_EFORMATNOTEQUAL: u32 = 220063;
// pub const PJMEDIA_SDP_ECONNNOTEQUAL: u32 = 220064;
// pub const PJMEDIA_SDP_EATTRNOTEQUAL: u32 = 220065;
// pub const PJMEDIA_SDP_EDIRNOTEQUAL: u32 = 220066;
// pub const PJMEDIA_SDP_EFMTPNOTEQUAL: u32 = 220067;
// pub const PJMEDIA_SDP_ERTPMAPNOTEQUAL: u32 = 220068;
// pub const PJMEDIA_SDP_ESESSNOTEQUAL: u32 = 220069;
// pub const PJMEDIA_SDP_EORIGINNOTEQUAL: u32 = 220070;
// pub const PJMEDIA_SDP_ENAMENOTEQUAL: u32 = 220071;
// pub const PJMEDIA_SDP_ETIMENOTEQUAL: u32 = 220072;
// pub const PJMEDIA_CODEC_EUNSUP: u32 = 220080;
// pub const PJMEDIA_CODEC_EFAILED: u32 = 220081;
// pub const PJMEDIA_CODEC_EFRMTOOSHORT: u32 = 220082;
// pub const PJMEDIA_CODEC_EPCMTOOSHORT: u32 = 220083;
// pub const PJMEDIA_CODEC_EFRMINLEN: u32 = 220084;
// pub const PJMEDIA_CODEC_EPCMFRMINLEN: u32 = 220085;
// pub const PJMEDIA_CODEC_EINMODE: u32 = 220086;
// pub const PJMEDIA_CODEC_EBADBITSTREAM: u32 = 220087;
// pub const PJMEDIA_EINVALIDIP: u32 = 220100;
// pub const PJMEDIA_EASYMCODEC: u32 = 220101;
// pub const PJMEDIA_EINVALIDPT: u32 = 220102;
// pub const PJMEDIA_EMISSINGRTPMAP: u32 = 220103;
// pub const PJMEDIA_EINVALIMEDIATYPE: u32 = 220104;
// pub const PJMEDIA_EREMOTENODTMF: u32 = 220105;
// pub const PJMEDIA_RTP_EINDTMF: u32 = 220106;
// pub const PJMEDIA_RTP_EREMNORFC2833: u32 = 220107;
// pub const PJMEDIA_EBADFMT: u32 = 220108;
// pub const PJMEDIA_EUNSUPMEDIATYPE: u32 = 220109;
// pub const PJMEDIA_RTP_EINPKT: u32 = 220120;
// pub const PJMEDIA_RTP_EINPACK: u32 = 220121;
// pub const PJMEDIA_RTP_EINVER: u32 = 220122;
// pub const PJMEDIA_RTP_EINSSRC: u32 = 220123;
// pub const PJMEDIA_RTP_EINPT: u32 = 220124;
// pub const PJMEDIA_RTP_EINLEN: u32 = 220125;
// pub const PJMEDIA_RTP_ESESSRESTART: u32 = 220130;
// pub const PJMEDIA_RTP_ESESSPROBATION: u32 = 220131;
// pub const PJMEDIA_RTP_EBADSEQ: u32 = 220132;
// pub const PJMEDIA_RTP_EBADDEST: u32 = 220133;
// pub const PJMEDIA_RTP_ENOCONFIG: u32 = 220134;
// pub const PJMEDIA_ENOTCOMPATIBLE: u32 = 220160;
// pub const PJMEDIA_ENCCLOCKRATE: u32 = 220161;
// pub const PJMEDIA_ENCSAMPLESPFRAME: u32 = 220162;
// pub const PJMEDIA_ENCTYPE: u32 = 220163;
// pub const PJMEDIA_ENCBITS: u32 = 220164;
// pub const PJMEDIA_ENCBYTES: u32 = 220165;
// pub const PJMEDIA_ENCCHANNEL: u32 = 220166;
// pub const PJMEDIA_ENOTVALIDWAVE: u32 = 220180;
// pub const PJMEDIA_EWAVEUNSUPP: u32 = 220181;
// pub const PJMEDIA_EWAVETOOSHORT: u32 = 220182;
// pub const PJMEDIA_EFRMFILETOOBIG: u32 = 220183;
// pub const PJMEDIA_EAVIUNSUPP: u32 = 220191;
// pub const PJMEDIA_ENOSNDREC: u32 = 220200;
// pub const PJMEDIA_ENOSNDPLAY: u32 = 220201;
// pub const PJMEDIA_ESNDINDEVID: u32 = 220202;
// pub const PJMEDIA_ESNDINSAMPLEFMT: u32 = 220203;

// pub const PJMEDIA_SRTP_ECRYPTONOTMATCH: u32 = 220220;
// pub const PJMEDIA_SRTP_EINKEYLEN: u32 = 220221;
// pub const PJMEDIA_SRTP_ENOTSUPCRYPTO: u32 = 220222;
// pub const PJMEDIA_SRTP_ESDPAMBIGUEANS: u32 = 220223;
// pub const PJMEDIA_SRTP_ESDPDUPCRYPTOTAG: u32 = 220224;
// pub const PJMEDIA_SRTP_ESDPINCRYPTO: u32 = 220225;
// pub const PJMEDIA_SRTP_ESDPINCRYPTOTAG: u32 = 220226;
// pub const PJMEDIA_SRTP_ESDPINTRANSPORT: u32 = 220227;
// pub const PJMEDIA_SRTP_ESDPREQCRYPTO: u32 = 220228;
// pub const PJMEDIA_SRTP_ESDPREQSECTP: u32 = 220229;
// pub const PJMEDIA_SRTP_EKEYNOTREADY: u32 = 220230;
// pub const PJMEDIA_SRTP_DTLS_ENOCRYPTO: u32 = 220240;
// pub const PJMEDIA_SRTP_DTLS_EPEERNOCERT: u32 = 220241;
// pub const PJMEDIA_SRTP_DTLS_EFPNOTMATCH: u32 = 220242;
// pub const PJMEDIA_SRTP_DTLS_ENOFPRINT: u32 = 220243;
// pub const PJMEDIA_SRTP_DTLS_ENOPROFILE: u32 = 220244;

// pub const PJMEDIA_CODEC_L16_HAS_8KHZ_MONO: u32 = 0;
// pub const PJMEDIA_CODEC_L16_HAS_8KHZ_STEREO: u32 = 0;
// pub const PJMEDIA_CODEC_L16_HAS_16KHZ_MONO: u32 = 0;
// pub const PJMEDIA_CODEC_L16_HAS_16KHZ_STEREO: u32 = 0;
// pub const PJMEDIA_CODEC_L16_HAS_48KHZ_MONO: u32 = 0;
// pub const PJMEDIA_CODEC_L16_HAS_48KHZ_STEREO: u32 = 0;
// pub const PJMEDIA_CODEC_SPEEX_DEFAULT_COMPLEXITY: u32 = 2;
// pub const PJMEDIA_CODEC_SPEEX_DEFAULT_QUALITY: u32 = 8;
// pub const PJMEDIA_POOL_LEN_G722_CODEC: u32 = 1000;
// pub const PJMEDIA_POOL_INC_G722_CODEC: u32 = 1000;
// pub const PJMEDIA_G722_DEFAULT_PCM_SHIFT: u32 = 2;
// pub const PJMEDIA_G722_STOP_PCM_SHIFT_ON_CLIPPING: u32 = 1;
// pub const PJMEDIA_AUTO_LINK_IPP_LIBS: u32 = 1;

// pub const PJMEDIA_AUTO_LINK_OPENCORE_AMR_LIBS: u32 = 1;
// pub const PJMEDIA_OPENCORE_AMR_BUILT_WITH_GCC: u32 = 1;
// pub const PJMEDIA_G7221_DEFAULT_PCM_SHIFT: u32 = 1;
// pub const PJMEDIA_CODEC_SILK_DEFAULT_COMPLEXITY: u32 = 2;
// pub const PJMEDIA_CODEC_SILK_DEFAULT_QUALITY: u32 = 10;
// pub const PJMEDIA_CODEC_OPUS_DEFAULT_SAMPLE_RATE: u32 = 48000;
// pub const PJMEDIA_CODEC_OPUS_DEFAULT_BIT_RATE: u32 = 0;
// pub const PJMEDIA_CODEC_OPUS_DEFAULT_COMPLEXITY: u32 = 5;
// pub const PJMEDIA_JB_DEFAULT_INIT_DELAY: u32 = 15;
// pub const PJMEDIA_VID_CODEC_MAX_DEC_FMT_CNT: u32 = 8;
// pub const PJMEDIA_VID_CODEC_MAX_FPS_CNT: u32 = 16;
// pub const PJMEDIA_VID_CODEC_MGR_MAX_CODECS: u32 = 32;

pub const PJMEDIA_TYPE_NONE: pjmedia_type = 0;
pub const PJMEDIA_TYPE_AUDIO: pjmedia_type = 1;
pub const PJMEDIA_TYPE_VIDEO: pjmedia_type = 2;
pub const PJMEDIA_TYPE_APPLICATION: pjmedia_type = 3;
pub const PJMEDIA_TYPE_UNKNOWN: pjmedia_type = 4;
pub type pjmedia_type = c_uint;

pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_NONE: pjmedia_tp_proto = 0;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_UNKNOWN: pjmedia_tp_proto = 1;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_UDP: pjmedia_tp_proto = 2;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_RTP: pjmedia_tp_proto = 4;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_DTLS: pjmedia_tp_proto = 8;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROFILE_RTCP_FB: pjmedia_tp_proto = 8192;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROFILE_SRTP: pjmedia_tp_proto = 16384;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROFILE_AVP: pjmedia_tp_proto = 32768;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_RTP_AVP: pjmedia_tp_proto = 32772;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_RTP_SAVP: pjmedia_tp_proto = 49156;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_DTLS_SRTP: pjmedia_tp_proto = 49164;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_RTP_AVPF: pjmedia_tp_proto = 40964;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_RTP_SAVPF: pjmedia_tp_proto = 57348;
pub const pjmedia_tp_proto_PJMEDIA_TP_PROTO_DTLS_SRTPF: pjmedia_tp_proto = 57356;
pub type pjmedia_tp_proto = c_uint;

pub const pjmedia_dir_PJMEDIA_DIR_NONE: pjmedia_dir = 0;
pub const pjmedia_dir_PJMEDIA_DIR_ENCODING: pjmedia_dir = 1;
pub const pjmedia_dir_PJMEDIA_DIR_CAPTURE: pjmedia_dir = 1;
pub const pjmedia_dir_PJMEDIA_DIR_DECODING: pjmedia_dir = 2;
pub const pjmedia_dir_PJMEDIA_DIR_PLAYBACK: pjmedia_dir = 2;
pub const pjmedia_dir_PJMEDIA_DIR_RENDER: pjmedia_dir = 2;
pub const pjmedia_dir_PJMEDIA_DIR_ENCODING_DECODING: pjmedia_dir = 3;
pub const pjmedia_dir_PJMEDIA_DIR_CAPTURE_PLAYBACK: pjmedia_dir = 3;
pub const pjmedia_dir_PJMEDIA_DIR_CAPTURE_RENDER: pjmedia_dir = 3;
pub type pjmedia_dir = c_uint;

pub const pjmedia_port_op_PJMEDIA_PORT_NO_CHANGE: pjmedia_port_op = 0;
pub const pjmedia_port_op_PJMEDIA_PORT_DISABLE: pjmedia_port_op = 1;
pub const pjmedia_port_op_PJMEDIA_PORT_MUTE: pjmedia_port_op = 2;
pub const pjmedia_port_op_PJMEDIA_PORT_ENABLE: pjmedia_port_op = 3;
pub type pjmedia_port_op = c_uint;

pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_PCMU: pjmedia_rtp_pt = 0;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_G721: pjmedia_rtp_pt = 2;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_GSM: pjmedia_rtp_pt = 3;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_G723: pjmedia_rtp_pt = 4;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_DVI4_8K: pjmedia_rtp_pt = 5;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_DVI4_16K: pjmedia_rtp_pt = 6;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_LPC: pjmedia_rtp_pt = 7;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_PCMA: pjmedia_rtp_pt = 8;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_G722: pjmedia_rtp_pt = 9;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_L16_2: pjmedia_rtp_pt = 10;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_L16_1: pjmedia_rtp_pt = 11;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_QCELP: pjmedia_rtp_pt = 12;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_CN: pjmedia_rtp_pt = 13;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_MPA: pjmedia_rtp_pt = 14;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_G728: pjmedia_rtp_pt = 15;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_DVI4_11K: pjmedia_rtp_pt = 16;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_DVI4_22K: pjmedia_rtp_pt = 17;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_G729: pjmedia_rtp_pt = 18;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_CELB: pjmedia_rtp_pt = 25;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_JPEG: pjmedia_rtp_pt = 26;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_NV: pjmedia_rtp_pt = 28;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_H261: pjmedia_rtp_pt = 31;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_MPV: pjmedia_rtp_pt = 32;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_MP2T: pjmedia_rtp_pt = 33;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_H263: pjmedia_rtp_pt = 34;
pub const pjmedia_rtp_pt_PJMEDIA_RTP_PT_DYNAMIC: pjmedia_rtp_pt = 96;
pub type pjmedia_rtp_pt = c_uint;

pub const pjmedia_converter_priority_guide_PJMEDIA_CONVERTER_PRIORITY_LOWEST: pjmedia_converter_priority_guide = 0;
pub const pjmedia_converter_priority_guide_PJMEDIA_CONVERTER_PRIORITY_NORMAL: pjmedia_converter_priority_guide = 15000;
pub const pjmedia_converter_priority_guide_PJMEDIA_CONVERTER_PRIORITY_HIGHEST: pjmedia_converter_priority_guide = 32000;
pub type pjmedia_converter_priority_guide = c_uint;

pub type pjmedia_converter_convert_setting = c_void;

pub const pjmedia_codec_priority_PJMEDIA_CODEC_PRIO_HIGHEST: pjmedia_codec_priority = 255;
pub const pjmedia_codec_priority_PJMEDIA_CODEC_PRIO_NEXT_HIGHER: pjmedia_codec_priority = 254;
pub const pjmedia_codec_priority_PJMEDIA_CODEC_PRIO_NORMAL: pjmedia_codec_priority = 128;
pub const pjmedia_codec_priority_PJMEDIA_CODEC_PRIO_LOWEST: pjmedia_codec_priority = 1;
pub const pjmedia_codec_priority_PJMEDIA_CODEC_PRIO_DISABLED: pjmedia_codec_priority = 0;
pub type pjmedia_codec_priority = c_uint;
pub type pjmedia_codec_id = [c_char; 32usize];

pub const pjmedia_conf_option_PJMEDIA_CONF_NO_MIC: pjmedia_conf_option = 1;
pub const pjmedia_conf_option_PJMEDIA_CONF_NO_DEVICE: pjmedia_conf_option = 2;
pub const pjmedia_conf_option_PJMEDIA_CONF_SMALL_FILTER: pjmedia_conf_option = 4;
pub const pjmedia_conf_option_PJMEDIA_CONF_USE_LINEAR: pjmedia_conf_option = 8;
pub type pjmedia_conf_option = c_uint;

pub const pjmedia_delay_buf_flag_PJMEDIA_DELAY_BUF_SIMPLE_FIFO: pjmedia_delay_buf_flag = 1;
pub type pjmedia_delay_buf_flag = c_uint;

pub const PJMEDIA_ECHO_DEFAULT: pjmedia_echo_flag = 0;
pub const PJMEDIA_ECHO_SPEEX: pjmedia_echo_flag = 1;
pub const PJMEDIA_ECHO_SIMPLE: pjmedia_echo_flag = 2;
pub const PJMEDIA_ECHO_WEBRTC: pjmedia_echo_flag = 3;
pub const PJMEDIA_ECHO_ALGO_MASK: pjmedia_echo_flag = 15;
pub const PJMEDIA_ECHO_NO_LOCK: pjmedia_echo_flag = 16;
pub const PJMEDIA_ECHO_USE_SIMPLE_FIFO: pjmedia_echo_flag = 32;
pub const PJMEDIA_ECHO_USE_SW_ECHO: pjmedia_echo_flag = 64;
pub const PJMEDIA_ECHO_USE_NOISE_SUPPRESSOR: pjmedia_echo_flag = 128;
pub const PJMEDIA_ECHO_AGGRESSIVENESS_DEFAULT: pjmedia_echo_flag = 0;
pub const PJMEDIA_ECHO_AGGRESSIVENESS_CONSERVATIVE: pjmedia_echo_flag = 256;
pub const PJMEDIA_ECHO_AGGRESSIVENESS_MODERATE: pjmedia_echo_flag = 512;
pub const PJMEDIA_ECHO_AGGRESSIVENESS_AGGRESSIVE: pjmedia_echo_flag = 768;
pub const PJMEDIA_ECHO_AGGRESSIVENESS_MASK: pjmedia_echo_flag = 3840;
pub type pjmedia_echo_flag = c_uint;

pub const pjmedia_tranport_media_option_PJMEDIA_TPMED_NO_TRANSPORT_CHECKING: pjmedia_tranport_media_option = 1;
pub const pjmedia_tranport_media_option_PJMEDIA_TPMED_RTCP_MUX: pjmedia_tranport_media_option = 2;
pub type pjmedia_tranport_media_option = c_uint;

pub const pjmedia_transport_type_PJMEDIA_TRANSPORT_TYPE_UDP: pjmedia_transport_type = 0;
pub const pjmedia_transport_type_PJMEDIA_TRANSPORT_TYPE_ICE: pjmedia_transport_type = 1;
pub const pjmedia_transport_type_PJMEDIA_TRANSPORT_TYPE_SRTP: pjmedia_transport_type = 2;
pub const pjmedia_transport_type_PJMEDIA_TRANSPORT_TYPE_LOOP: pjmedia_transport_type = 3;
pub const pjmedia_transport_type_PJMEDIA_TRANSPORT_TYPE_USER: pjmedia_transport_type = 4;
pub type pjmedia_transport_type = c_uint;

pub const pjmedia_endpt_flag_PJMEDIA_ENDPT_HAS_TELEPHONE_EVENT_FLAG: pjmedia_endpt_flag = 0;
pub type pjmedia_endpt_flag = c_uint;

pub type pjmedia_endpt_exit_callback = Option<unsafe extern "C" fn(endpt: *mut pjmedia_endpt)>;

pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_START: pjmedia_audio_pt = 95;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SPEEX_NB: pjmedia_audio_pt = 96;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SPEEX_WB: pjmedia_audio_pt = 97;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SPEEX_UWB: pjmedia_audio_pt = 98;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SILK_NB: pjmedia_audio_pt = 99;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SILK_MB: pjmedia_audio_pt = 100;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SILK_WB: pjmedia_audio_pt = 101;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_SILK_SWB: pjmedia_audio_pt = 102;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_ILBC: pjmedia_audio_pt = 103;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_AMR: pjmedia_audio_pt = 104;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_AMRWB: pjmedia_audio_pt = 105;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_AMRWBE: pjmedia_audio_pt = 106;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G726_16: pjmedia_audio_pt = 107;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G726_24: pjmedia_audio_pt = 108;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G726_32: pjmedia_audio_pt = 109;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G726_40: pjmedia_audio_pt = 110;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G722_1_16: pjmedia_audio_pt = 111;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G722_1_24: pjmedia_audio_pt = 112;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G722_1_32: pjmedia_audio_pt = 113;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G7221C_24: pjmedia_audio_pt = 114;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G7221C_32: pjmedia_audio_pt = 115;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G7221C_48: pjmedia_audio_pt = 116;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G7221_RSV1: pjmedia_audio_pt = 117;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_G7221_RSV2: pjmedia_audio_pt = 118;
pub const pjmedia_audio_pt_PJMEDIA_RTP_PT_OPUS: pjmedia_audio_pt = 119;
pub type pjmedia_audio_pt = c_uint;

pub const pjmedia_video_pt_PJMEDIA_RTP_PT_VID_START: pjmedia_video_pt = 95;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_H263P: pjmedia_video_pt = 96;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_H264: pjmedia_video_pt = 97;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_H264_RSV1: pjmedia_video_pt = 98;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_H264_RSV2: pjmedia_video_pt = 99;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_H264_RSV3: pjmedia_video_pt = 100;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_H264_RSV4: pjmedia_video_pt = 101;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_VP8: pjmedia_video_pt = 102;
pub const pjmedia_video_pt_PJMEDIA_RTP_PT_VP9: pjmedia_video_pt = 103;
pub type pjmedia_video_pt = c_uint;

pub const pjmedia_jb_frame_type_PJMEDIA_JB_MISSING_FRAME: pjmedia_jb_frame_type = 0;
pub const pjmedia_jb_frame_type_PJMEDIA_JB_NORMAL_FRAME: pjmedia_jb_frame_type = 1;
pub const pjmedia_jb_frame_type_PJMEDIA_JB_ZERO_PREFETCH_FRAME: pjmedia_jb_frame_type = 2;
pub const pjmedia_jb_frame_type_PJMEDIA_JB_ZERO_EMPTY_FRAME: pjmedia_jb_frame_type = 3;
pub type pjmedia_jb_frame_type = c_uint;

pub const pjmedia_jb_discard_algo_PJMEDIA_JB_DISCARD_NONE: pjmedia_jb_discard_algo = 0;
pub const pjmedia_jb_discard_algo_PJMEDIA_JB_DISCARD_STATIC: pjmedia_jb_discard_algo = 1;
pub const pjmedia_jb_discard_algo_PJMEDIA_JB_DISCARD_PROGRESSIVE: pjmedia_jb_discard_algo = 2;
pub type pjmedia_jb_discard_algo = c_uint;

pub const pjmedia_mem_player_option_PJMEDIA_MEM_NO_LOOP: pjmedia_mem_player_option = 1;
pub type pjmedia_mem_player_option = c_uint;

pub const pjmedia_resample_port_options_PJMEDIA_RESAMPLE_USE_LINEAR: pjmedia_resample_port_options = 1;
pub const pjmedia_resample_port_options_PJMEDIA_RESAMPLE_USE_SMALL_FILTER: pjmedia_resample_port_options = 2;
pub const pjmedia_resample_port_options_PJMEDIA_RESAMPLE_DONT_DESTROY_DN: pjmedia_resample_port_options = 4;
pub type pjmedia_resample_port_options = c_uint;

pub const pjmedia_sdp_neg_state_PJMEDIA_SDP_NEG_STATE_NULL: pjmedia_sdp_neg_state = 0;
pub const pjmedia_sdp_neg_state_PJMEDIA_SDP_NEG_STATE_LOCAL_OFFER: pjmedia_sdp_neg_state = 1;
pub const pjmedia_sdp_neg_state_PJMEDIA_SDP_NEG_STATE_REMOTE_OFFER: pjmedia_sdp_neg_state = 2;
pub const pjmedia_sdp_neg_state_PJMEDIA_SDP_NEG_STATE_WAIT_NEGO: pjmedia_sdp_neg_state = 3;
pub const pjmedia_sdp_neg_state_PJMEDIA_SDP_NEG_STATE_DONE: pjmedia_sdp_neg_state = 4;
pub type pjmedia_sdp_neg_state = c_uint;

pub const pjmedia_mod_offer_flag_PJMEDIA_SDP_NEG_ALLOW_MEDIA_CHANGE: pjmedia_mod_offer_flag = 1;
pub type pjmedia_mod_offer_flag = c_uint;

pub const pjmedia_sdp_neg_fmt_match_flag_PJMEDIA_SDP_NEG_FMT_MATCH_ALLOW_MODIFY_ANSWER: pjmedia_sdp_neg_fmt_match_flag = 1;
pub type pjmedia_sdp_neg_fmt_match_flag = c_uint;
pub type pjmedia_sdp_neg_fmt_match_cb = Option<
    unsafe extern "C" fn(
        pool: *mut pj_pool_t,
        offer: *mut pjmedia_sdp_media,
        o_fmt_idx: c_uint,
        answer: *mut pjmedia_sdp_media,
        a_fmt_idx: c_uint,
        option: c_uint,
    ) -> pj_status_t,
>;

pub const pjmedia_snd_port_option_PJMEDIA_SND_PORT_NO_AUTO_START: pjmedia_snd_port_option = 1;
pub type pjmedia_snd_port_option = c_uint;

pub const pjmedia_stereo_port_options_PJMEDIA_STEREO_DONT_DESTROY_DN: pjmedia_stereo_port_options = 4;
pub type pjmedia_stereo_port_options = c_uint;

pub const pjmedia_vid_packing_PJMEDIA_VID_PACKING_UNKNOWN: pjmedia_vid_packing = 0;
pub const pjmedia_vid_packing_PJMEDIA_VID_PACKING_PACKETS: pjmedia_vid_packing = 1;
pub const pjmedia_vid_packing_PJMEDIA_VID_PACKING_WHOLE: pjmedia_vid_packing = 2;
pub type pjmedia_vid_packing = c_uint;

pub const pjmedia_vid_frm_bit_info_PJMEDIA_VID_FRM_KEYFRAME: pjmedia_vid_frm_bit_info = 1;
pub type pjmedia_vid_frm_bit_info = c_uint;

pub const pjmedia_stream_dtmf_event_flags_PJMEDIA_STREAM_DTMF_IS_UPDATE: pjmedia_stream_dtmf_event_flags = 1;
pub const pjmedia_stream_dtmf_event_flags_PJMEDIA_STREAM_DTMF_IS_END: pjmedia_stream_dtmf_event_flags = 2;
pub type pjmedia_stream_dtmf_event_flags = c_uint;

pub const PJMEDIA_TONEGEN_LOOP: c_uint = 1;
pub const PJMEDIA_TONEGEN_NO_LOCK: c_uint = 2;
pub type _bindgen_ty_17 = c_uint;

pub const pjmedia_orient_PJMEDIA_ORIENT_UNKNOWN: pjmedia_orient = 0;
pub const pjmedia_orient_PJMEDIA_ORIENT_NATURAL: pjmedia_orient = 1;
pub const pjmedia_orient_PJMEDIA_ORIENT_ROTATE_90DEG: pjmedia_orient = 2;
pub const pjmedia_orient_PJMEDIA_ORIENT_ROTATE_180DEG: pjmedia_orient = 3;
pub const pjmedia_orient_PJMEDIA_ORIENT_ROTATE_270DEG: pjmedia_orient = 4;
pub type pjmedia_orient = c_uint;

pub const pjmedia_clock_options_PJMEDIA_CLOCK_NO_ASYNC: pjmedia_clock_options = 1;
pub const pjmedia_clock_options_PJMEDIA_CLOCK_NO_HIGHEST_PRIO: pjmedia_clock_options = 2;
pub type pjmedia_clock_options = c_uint;

pub const pjmedia_coord_base_PJMEDIA_COORD_BASE_LEFT_TOP: pjmedia_coord_base = 0;
pub const pjmedia_coord_base_PJMEDIA_COORD_BASE_LEFT_BOTTOM: pjmedia_coord_base = 1;
pub type pjmedia_coord_base = c_uint;

pub const pjmedia_format_id_PJMEDIA_FORMAT_L16: pjmedia_format_id = 0;
pub const pjmedia_format_id_PJMEDIA_FORMAT_PCM: pjmedia_format_id = 0;
pub const pjmedia_format_id_PJMEDIA_FORMAT_PCMA: pjmedia_format_id = 1463897153;
pub const pjmedia_format_id_PJMEDIA_FORMAT_ALAW: pjmedia_format_id = 1463897153;
pub const pjmedia_format_id_PJMEDIA_FORMAT_PCMU: pjmedia_format_id = 1463897205;
pub const pjmedia_format_id_PJMEDIA_FORMAT_ULAW: pjmedia_format_id = 1463897205;
pub const pjmedia_format_id_PJMEDIA_FORMAT_AMR: pjmedia_format_id = 1380794656;
pub const pjmedia_format_id_PJMEDIA_FORMAT_G729: pjmedia_format_id = 959592263;
pub const pjmedia_format_id_PJMEDIA_FORMAT_ILBC: pjmedia_format_id = 1128418377;
pub const pjmedia_format_id_PJMEDIA_FORMAT_RGB24: pjmedia_format_id = 859981650;
pub const pjmedia_format_id_PJMEDIA_FORMAT_RGBA: pjmedia_format_id = 1094862674;
pub const pjmedia_format_id_PJMEDIA_FORMAT_BGRA: pjmedia_format_id = 1095911234;
pub const pjmedia_format_id_PJMEDIA_FORMAT_RGB32: pjmedia_format_id = 1094862674;
pub const pjmedia_format_id_PJMEDIA_FORMAT_DIB: pjmedia_format_id = 541215044;
pub const pjmedia_format_id_PJMEDIA_FORMAT_GBRP: pjmedia_format_id = 1347568199;
pub const pjmedia_format_id_PJMEDIA_FORMAT_AYUV: pjmedia_format_id = 1448433985;
pub const pjmedia_format_id_PJMEDIA_FORMAT_YUY2: pjmedia_format_id = 844715353;
pub const pjmedia_format_id_PJMEDIA_FORMAT_UYVY: pjmedia_format_id = 1498831189;
pub const pjmedia_format_id_PJMEDIA_FORMAT_YVYU: pjmedia_format_id = 1431918169;
pub const pjmedia_format_id_PJMEDIA_FORMAT_I420: pjmedia_format_id = 808596553;
pub const pjmedia_format_id_PJMEDIA_FORMAT_IYUV: pjmedia_format_id = 808596553;
pub const pjmedia_format_id_PJMEDIA_FORMAT_YV12: pjmedia_format_id = 842094169;
pub const pjmedia_format_id_PJMEDIA_FORMAT_NV12: pjmedia_format_id = 842094158;
pub const pjmedia_format_id_PJMEDIA_FORMAT_NV21: pjmedia_format_id = 825382478;
pub const pjmedia_format_id_PJMEDIA_FORMAT_I422: pjmedia_format_id = 842150985;
pub const pjmedia_format_id_PJMEDIA_FORMAT_I420JPEG: pjmedia_format_id = 808596554;
pub const pjmedia_format_id_PJMEDIA_FORMAT_I422JPEG: pjmedia_format_id = 842150986;
pub const pjmedia_format_id_PJMEDIA_FORMAT_H261: pjmedia_format_id = 825635400;
pub const pjmedia_format_id_PJMEDIA_FORMAT_H263: pjmedia_format_id = 859189832;
pub const pjmedia_format_id_PJMEDIA_FORMAT_H263P: pjmedia_format_id = 859189840;
pub const pjmedia_format_id_PJMEDIA_FORMAT_H264: pjmedia_format_id = 875967048;
pub const pjmedia_format_id_PJMEDIA_FORMAT_VP8: pjmedia_format_id = 808996950;
pub const pjmedia_format_id_PJMEDIA_FORMAT_VP9: pjmedia_format_id = 809062486;
pub const pjmedia_format_id_PJMEDIA_FORMAT_MJPEG: pjmedia_format_id = 1196444237;
pub const pjmedia_format_id_PJMEDIA_FORMAT_MPEG1VIDEO: pjmedia_format_id = 1446072397;
pub const pjmedia_format_id_PJMEDIA_FORMAT_MPEG2VIDEO: pjmedia_format_id = 1446137933;
pub const pjmedia_format_id_PJMEDIA_FORMAT_MPEG4: pjmedia_format_id = 877088845;
pub const pjmedia_format_id_PJMEDIA_FORMAT_INVALID: pjmedia_format_id = 4294967295;
pub type pjmedia_format_id = c_uint;

pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_NONE: pjmedia_format_detail_type = 0;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_AUDIO: pjmedia_format_detail_type = 1;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_VIDEO: pjmedia_format_detail_type = 2;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_MAX: pjmedia_format_detail_type = 3;
pub type pjmedia_format_detail_type = c_uint;

pub const pjmedia_color_model_PJMEDIA_COLOR_MODEL_NONE: pjmedia_color_model = 0;
pub const pjmedia_color_model_PJMEDIA_COLOR_MODEL_RGB: pjmedia_color_model = 1;
pub const pjmedia_color_model_PJMEDIA_COLOR_MODEL_YUV: pjmedia_color_model = 2;
pub type pjmedia_color_model = c_uint;

pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_NONE: pjmedia_frame_type = 0;
pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_AUDIO: pjmedia_frame_type = 1;
pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_EXTENDED: pjmedia_frame_type = 2;
pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_VIDEO: pjmedia_frame_type = 3;
pub type pjmedia_frame_type = c_uint;

pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_EXT_FORMAT: pjmedia_aud_dev_cap = 1;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_INPUT_LATENCY: pjmedia_aud_dev_cap = 2;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_OUTPUT_LATENCY: pjmedia_aud_dev_cap = 4;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_INPUT_VOLUME_SETTING: pjmedia_aud_dev_cap = 8;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_OUTPUT_VOLUME_SETTING: pjmedia_aud_dev_cap = 16;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_INPUT_SIGNAL_METER: pjmedia_aud_dev_cap = 32;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_OUTPUT_SIGNAL_METER: pjmedia_aud_dev_cap = 64;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_INPUT_ROUTE: pjmedia_aud_dev_cap = 128;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_INPUT_SOURCE: pjmedia_aud_dev_cap = 128;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_OUTPUT_ROUTE: pjmedia_aud_dev_cap = 256;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_EC: pjmedia_aud_dev_cap = 512;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_EC_TAIL: pjmedia_aud_dev_cap = 1024;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_VAD: pjmedia_aud_dev_cap = 2048;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_CNG: pjmedia_aud_dev_cap = 4096;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_PLC: pjmedia_aud_dev_cap = 8192;
pub const pjmedia_aud_dev_cap_PJMEDIA_AUD_DEV_CAP_MAX: pjmedia_aud_dev_cap = 16384;
pub type pjmedia_aud_dev_cap = c_uint;

pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_DEFAULT: pjmedia_aud_dev_route = 0;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_LOUDSPEAKER: pjmedia_aud_dev_route = 1;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_EARPIECE: pjmedia_aud_dev_route = 2;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_BLUETOOTH: pjmedia_aud_dev_route = 4;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_CUSTOM: pjmedia_aud_dev_route = 128;
pub type pjmedia_aud_dev_route = c_uint;

pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_LOSS_RLE: pjmedia_rtcp_xr_type = 1;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_DUP_RLE: pjmedia_rtcp_xr_type = 2;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_RCPT_TIMES: pjmedia_rtcp_xr_type = 4;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_RR_TIME: pjmedia_rtcp_xr_type = 8;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_DLRR: pjmedia_rtcp_xr_type = 16;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_STATS: pjmedia_rtcp_xr_type = 32;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_VOIP_METRICS: pjmedia_rtcp_xr_type = 64;
pub type pjmedia_rtcp_xr_type = c_uint;

pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_SIGNAL_LVL: pjmedia_rtcp_xr_info = 1;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_NOISE_LVL: pjmedia_rtcp_xr_info = 2;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_RERL: pjmedia_rtcp_xr_info = 3;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_R_FACTOR: pjmedia_rtcp_xr_info = 4;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_MOS_LQ: pjmedia_rtcp_xr_info = 5;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_MOS_CQ: pjmedia_rtcp_xr_info = 6;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_CONF_PLC: pjmedia_rtcp_xr_info = 7;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_CONF_JBA: pjmedia_rtcp_xr_info = 8;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_CONF_JBR: pjmedia_rtcp_xr_info = 9;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_JB_NOM: pjmedia_rtcp_xr_info = 10;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_JB_MAX: pjmedia_rtcp_xr_info = 11;
pub const pjmedia_rtcp_xr_info_PJMEDIA_RTCP_XR_INFO_JB_ABS_MAX: pjmedia_rtcp_xr_info = 12;
pub type pjmedia_rtcp_xr_info = c_uint;

pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_UNK: pjmedia_rtcp_xr_plc_type = 0;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_DIS: pjmedia_rtcp_xr_plc_type = 1;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_ENH: pjmedia_rtcp_xr_plc_type = 2;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_STD: pjmedia_rtcp_xr_plc_type = 3;
pub type pjmedia_rtcp_xr_plc_type = c_uint;
pub const pjmedia_rtcp_xr_jb_type_PJMEDIA_RTCP_XR_JB_UNKNOWN: pjmedia_rtcp_xr_jb_type = 0;
pub const pjmedia_rtcp_xr_jb_type_PJMEDIA_RTCP_XR_JB_FIXED: pjmedia_rtcp_xr_jb_type = 2;
pub const pjmedia_rtcp_xr_jb_type_PJMEDIA_RTCP_XR_JB_ADAPTIVE: pjmedia_rtcp_xr_jb_type = 3;
pub type pjmedia_rtcp_xr_jb_type = c_uint;

pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_ACK: pjmedia_rtcp_fb_type = 0;
pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_NACK: pjmedia_rtcp_fb_type = 1;
pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_TRR_INT: pjmedia_rtcp_fb_type = 2;
pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_OTHER: pjmedia_rtcp_fb_type = 3;
pub type pjmedia_rtcp_fb_type = c_uint;

pub type pjmedia_obj_sig = pj_uint32_t;

pub type pjmedia_vid_dev_index = pj_int32_t;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_NONE: pjmedia_vid_dev_hwnd_type = 0;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_WINDOWS: pjmedia_vid_dev_hwnd_type = 1;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_IOS: pjmedia_vid_dev_hwnd_type = 2;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_ANDROID: pjmedia_vid_dev_hwnd_type = 3;
pub type pjmedia_vid_dev_hwnd_type = c_uint;

pub const pjmedia_vid_dev_wnd_flag_PJMEDIA_VID_DEV_WND_BORDER: pjmedia_vid_dev_wnd_flag = 1;
pub const pjmedia_vid_dev_wnd_flag_PJMEDIA_VID_DEV_WND_RESIZABLE: pjmedia_vid_dev_wnd_flag = 2;
pub type pjmedia_vid_dev_wnd_flag = c_uint;
pub const pjmedia_vid_dev_std_index_PJMEDIA_VID_DEFAULT_CAPTURE_DEV: pjmedia_vid_dev_std_index = -1;
pub const pjmedia_vid_dev_std_index_PJMEDIA_VID_DEFAULT_RENDER_DEV: pjmedia_vid_dev_std_index = -2;
pub const pjmedia_vid_dev_std_index_PJMEDIA_VID_INVALID_DEV: pjmedia_vid_dev_std_index = -3;
pub type pjmedia_vid_dev_std_index = c_int;

pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_FORMAT: pjmedia_vid_dev_cap = 1;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_INPUT_SCALE: pjmedia_vid_dev_cap = 2;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_OUTPUT_WINDOW: pjmedia_vid_dev_cap = 4;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_OUTPUT_RESIZE: pjmedia_vid_dev_cap = 8;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_OUTPUT_POSITION: pjmedia_vid_dev_cap = 16;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_OUTPUT_HIDE: pjmedia_vid_dev_cap = 32;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_INPUT_PREVIEW: pjmedia_vid_dev_cap = 64;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_ORIENTATION: pjmedia_vid_dev_cap = 128;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_SWITCH: pjmedia_vid_dev_cap = 256;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_OUTPUT_WINDOW_FLAGS: pjmedia_vid_dev_cap = 512;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_OUTPUT_FULLSCREEN: pjmedia_vid_dev_cap = 1024;
pub const pjmedia_vid_dev_cap_PJMEDIA_VID_DEV_CAP_MAX: pjmedia_vid_dev_cap = 16384;
pub type pjmedia_vid_dev_cap = c_uint;

pub const pjmedia_event_type_PJMEDIA_EVENT_NONE: pjmedia_event_type = 0;
pub const pjmedia_event_type_PJMEDIA_EVENT_FMT_CHANGED: pjmedia_event_type = 1212370246;
pub const pjmedia_event_type_PJMEDIA_EVENT_WND_CLOSING: pjmedia_event_type = 1279479383;
pub const pjmedia_event_type_PJMEDIA_EVENT_WND_CLOSED: pjmedia_event_type = 1329811031;
pub const pjmedia_event_type_PJMEDIA_EVENT_WND_RESIZED: pjmedia_event_type = 1515343447;
pub const pjmedia_event_type_PJMEDIA_EVENT_MOUSE_BTN_DOWN: pjmedia_event_type = 1313100621;
pub const pjmedia_event_type_PJMEDIA_EVENT_KEYFRAME_FOUND: pjmedia_event_type = 1179797065;
pub const pjmedia_event_type_PJMEDIA_EVENT_KEYFRAME_MISSING: pjmedia_event_type = 1297237577;
pub const pjmedia_event_type_PJMEDIA_EVENT_ORIENT_CHANGED: pjmedia_event_type = 1414419023;
pub const pjmedia_event_type_PJMEDIA_EVENT_RX_RTCP_FB: pjmedia_event_type = 1111905362;
pub const pjmedia_event_type_PJMEDIA_EVENT_AUD_DEV_ERROR: pjmedia_event_type = 1381123393;
pub const pjmedia_event_type_PJMEDIA_EVENT_VID_DEV_ERROR: pjmedia_event_type = 1381123414;
pub const pjmedia_event_type_PJMEDIA_EVENT_MEDIA_TP_ERR: pjmedia_event_type = 1381123412;
pub const pjmedia_event_type_PJMEDIA_EVENT_CALLBACK: pjmedia_event_type = 538985027;
pub type pjmedia_event_type = c_uint;

pub const pjmedia_event_publish_flag_PJMEDIA_EVENT_PUBLISH_DEFAULT: pjmedia_event_publish_flag = 0;
pub const pjmedia_event_publish_flag_PJMEDIA_EVENT_PUBLISH_POST_EVENT: pjmedia_event_publish_flag = 1;
pub type pjmedia_event_publish_flag = c_uint;

pub const pjmedia_event_mgr_flag_PJMEDIA_EVENT_MGR_NO_THREAD: pjmedia_event_mgr_flag = 1;
pub type pjmedia_event_mgr_flag = c_uint;

pub const pjmedia_transport_ice_options_PJMEDIA_ICE_NO_SRC_ADDR_CHECKING: pjmedia_transport_ice_options = 1;
pub const pjmedia_transport_ice_options_PJMEDIA_ICE_DISABLE_ICE_MISMATCH: pjmedia_transport_ice_options = 2;
pub type pjmedia_transport_ice_options = c_uint;

pub const pjmedia_srtp_crypto_option_PJMEDIA_SRTP_NO_ENCRYPTION: pjmedia_srtp_crypto_option = 1;
pub const pjmedia_srtp_crypto_option_PJMEDIA_SRTP_NO_AUTHENTICATION: pjmedia_srtp_crypto_option = 2;
pub type pjmedia_srtp_crypto_option = c_uint;

pub const pjmedia_srtp_use_PJMEDIA_SRTP_DISABLED: pjmedia_srtp_use = 0;
pub const pjmedia_srtp_use_PJMEDIA_SRTP_UNKNOWN: pjmedia_srtp_use = 0;
pub const pjmedia_srtp_use_PJMEDIA_SRTP_OPTIONAL: pjmedia_srtp_use = 1;
pub const pjmedia_srtp_use_PJMEDIA_SRTP_MANDATORY: pjmedia_srtp_use = 2;
pub type pjmedia_srtp_use = c_uint;

pub const pjmedia_srtp_keying_method_PJMEDIA_SRTP_KEYING_SDES: pjmedia_srtp_keying_method = 0;
pub const pjmedia_srtp_keying_method_PJMEDIA_SRTP_KEYING_DTLS_SRTP: pjmedia_srtp_keying_method = 1;
pub const pjmedia_srtp_keying_method_PJMEDIA_SRTP_KEYINGS_COUNT: pjmedia_srtp_keying_method = 2;
pub type pjmedia_srtp_keying_method = c_uint;

pub const pjmedia_transport_udp_options_PJMEDIA_UDP_NO_SRC_ADDR_CHECKING: pjmedia_transport_udp_options = 1;
pub type pjmedia_transport_udp_options = c_uint;

pub const pjmedia_vid_stream_rc_method_PJMEDIA_VID_STREAM_RC_NONE: pjmedia_vid_stream_rc_method = 0;
pub const pjmedia_vid_stream_rc_method_PJMEDIA_VID_STREAM_RC_SIMPLE_BLOCKING: pjmedia_vid_stream_rc_method = 1;
pub type pjmedia_vid_stream_rc_method = c_uint;

pub const pjmedia_vid_conf_layout_PJMEDIA_VID_CONF_LAYOUT_DEFAULT: pjmedia_vid_conf_layout = 0;
pub const pjmedia_vid_conf_layout_PJMEDIA_VID_CONF_LAYOUT_SELECTIVE_FOCUS: pjmedia_vid_conf_layout = 1;
pub const pjmedia_vid_conf_layout_PJMEDIA_VID_CONF_LAYOUT_INTERVAL_FOCUS: pjmedia_vid_conf_layout = 2;
pub const pjmedia_vid_conf_layout_PJMEDIA_VID_CONF_LAYOUT_CUSTOM: pjmedia_vid_conf_layout = 3;
pub type pjmedia_vid_conf_layout = c_uint;

pub const pjmedia_file_player_option_PJMEDIA_FILE_NO_LOOP: pjmedia_file_player_option = 1;
pub type pjmedia_file_player_option = c_uint;

pub const pjmedia_file_writer_option_PJMEDIA_FILE_WRITE_PCM: pjmedia_file_writer_option = 0;
pub const pjmedia_file_writer_option_PJMEDIA_FILE_WRITE_ALAW: pjmedia_file_writer_option = 1;
pub const pjmedia_file_writer_option_PJMEDIA_FILE_WRITE_ULAW: pjmedia_file_writer_option = 2;
pub type pjmedia_file_writer_option = c_uint;

pub const pjmedia_wave_fmt_tag_PJMEDIA_WAVE_FMT_TAG_PCM: pjmedia_wave_fmt_tag = 1;
pub const pjmedia_wave_fmt_tag_PJMEDIA_WAVE_FMT_TAG_ALAW: pjmedia_wave_fmt_tag = 6;
pub const pjmedia_wave_fmt_tag_PJMEDIA_WAVE_FMT_TAG_ULAW: pjmedia_wave_fmt_tag = 7;
pub type pjmedia_wave_fmt_tag = c_uint;

pub const pjmedia_wsola_option_PJMEDIA_WSOLA_NO_HANNING: pjmedia_wsola_option = 1;
pub const pjmedia_wsola_option_PJMEDIA_WSOLA_NO_PLC: pjmedia_wsola_option = 2;
pub const pjmedia_wsola_option_PJMEDIA_WSOLA_NO_DISCARD: pjmedia_wsola_option = 4;
pub const pjmedia_wsola_option_PJMEDIA_WSOLA_NO_FADING: pjmedia_wsola_option = 8;
pub type pjmedia_wsola_option = c_uint;

pub const pjmedia_amr_options_PJMEDIA_AMR_NO_NB: pjmedia_amr_options = 1;
pub const pjmedia_amr_options_PJMEDIA_AMR_NO_WB: pjmedia_amr_options = 2;
pub type pjmedia_amr_options = c_uint;

pub type pjmedia_codec_amrnb_config = pjmedia_codec_amr_config;
pub type pjmedia_codec_amrwb_config = pjmedia_codec_amr_config;

pub const pjmedia_speex_options_PJMEDIA_SPEEX_NO_NB: pjmedia_speex_options = 1;
pub const pjmedia_speex_options_PJMEDIA_SPEEX_NO_WB: pjmedia_speex_options = 2;
pub const pjmedia_speex_options_PJMEDIA_SPEEX_NO_UWB: pjmedia_speex_options = 4;
pub type pjmedia_speex_options = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_endpt {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_stream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_ratio {
    pub num: c_int,
    pub denum: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_coord {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rect_size {
    pub w: c_uint,
    pub h: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rect {
    pub coord: pjmedia_coord,
    pub size: pjmedia_rect_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_clock_src {
    pub media_type: pjmedia_type,
    pub clock_rate: c_uint,
    pub ptime_usec: c_uint,
    pub timestamp: pj_timestamp,
    pub last_update: pj_timestamp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_clock {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_clock_param {
    pub usec_interval: c_uint,
    pub clock_rate: c_uint,
}
pub type pjmedia_clock_callback = Option<
    unsafe extern "C" fn(ts: *const pj_timestamp, user_data: *mut c_void),
>;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_format_detail {
    pub clock_rate: c_uint,
    pub channel_count: c_uint,
    pub frame_time_usec: c_uint,
    pub bits_per_sample: c_uint,
    pub avg_bps: pj_uint32_t,
    pub max_bps: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_video_format_detail {
    pub size: pjmedia_rect_size,
    pub fps: pjmedia_ratio,
    pub avg_bps: pj_uint32_t,
    pub max_bps: pj_uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_format {
    pub id: pj_uint32_t,
    pub type_: pjmedia_type,
    pub detail_type: pjmedia_format_detail_type,
    pub det: pjmedia_format__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjmedia_format__bindgen_ty_1 {
    pub aud: pjmedia_audio_format_detail,
    pub vid: pjmedia_video_format_detail,
    pub user: [c_char; 1usize],
    _bindgen_union_align: [u32; 6usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_video_apply_fmt_param {
    pub size: pjmedia_rect_size,
    pub buffer: *mut pj_uint8_t,
    pub framebytes: pj_size_t,
    pub strides: [c_int; 4usize],
    pub planes: [*mut pj_uint8_t; 4usize],
    pub plane_bytes: [pj_size_t; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_video_format_info {
    pub id: pj_uint32_t,
    pub name: [c_char; 8usize],
    pub color_model: pjmedia_color_model,
    pub bpp: pj_uint8_t,
    pub plane_cnt: pj_uint8_t,
    pub apply_fmt: Option<
        unsafe extern "C" fn(
            vfi: *const pjmedia_video_format_info,
            vafp: *mut pjmedia_video_apply_fmt_param,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_video_format_mgr {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_frame {
    pub type_: pjmedia_frame_type,
    pub buf: *mut c_void,
    pub size: pj_size_t,
    pub timestamp: pj_timestamp,
    pub bit_info: pj_uint32_t,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pjmedia_frame_ext {
    pub base: pjmedia_frame,
    pub samples_cnt: pj_uint16_t,
    pub subframe_cnt: pj_uint16_t,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_frame_ext_subframe {
    pub bitlen: pj_uint16_t,
    pub data: [pj_uint8_t; 1usize],
}

pub type pjmedia_aud_dev_index = pj_int32_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_aud_stream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_aud_dev_factory {
    _unused: [u8; 0],
}
pub type pjmedia_aud_dev_factory_create_func_ptr = Option<
    unsafe extern "C" fn(arg1: *mut pj_pool_factory) -> *mut pjmedia_aud_dev_factory,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_aud_driver {
    pub create: pjmedia_aud_dev_factory_create_func_ptr,
    pub f: *mut pjmedia_aud_dev_factory,
    pub name: [c_char; 32usize],
    pub dev_cnt: c_uint,
    pub start_idx: c_uint,
    pub rec_dev_idx: c_int,
    pub play_dev_idx: c_int,
    pub dev_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_aud_subsys {
    pub init_count: c_uint,
    pub pf: *mut pj_pool_factory,
    pub drv_cnt: c_uint,
    pub drv: [pjmedia_aud_driver; 16usize],
    pub dev_cnt: c_uint,
    pub dev_list: [pj_uint32_t; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_aud_dev_info {
    pub name: [c_char; 64usize],
    pub input_count: c_uint,
    pub output_count: c_uint,
    pub default_samples_per_sec: c_uint,
    pub driver: [c_char; 32usize],
    pub caps: c_uint,
    pub routes: c_uint,
    pub ext_fmt_cnt: c_uint,
    pub ext_fmt: [pjmedia_format; 8usize],
}
pub type pjmedia_aud_play_cb = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        frame: *mut pjmedia_frame,
    ) -> pj_status_t,
>;
pub type pjmedia_aud_rec_cb = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        frame: *mut pjmedia_frame,
    ) -> pj_status_t,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_aud_param {
    pub dir: pjmedia_dir,
    pub rec_id: pjmedia_aud_dev_index,
    pub play_id: pjmedia_aud_dev_index,
    pub clock_rate: c_uint,
    pub channel_count: c_uint,
    pub samples_per_frame: c_uint,
    pub bits_per_sample: c_uint,
    pub flags: c_uint,
    pub ext_fmt: pjmedia_format,
    pub input_latency_ms: c_uint,
    pub output_latency_ms: c_uint,
    pub input_vol: c_uint,
    pub output_vol: c_uint,
    pub input_route: pjmedia_aud_dev_route,
    pub output_route: pjmedia_aud_dev_route,
    pub ec_enabled: pj_bool_t,
    pub ec_tail_ms: c_uint,
    pub plc_enabled: pj_bool_t,
    pub cng_enabled: pj_bool_t,
    pub vad_enabled: pj_bool_t,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_rb_header {
    pub bt: pj_uint8_t,
    pub specific: pj_uint8_t,
    pub length: pj_uint16_t,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_rb_rr_time {
    pub header: pjmedia_rtcp_xr_rb_header,
    pub ntp_sec: pj_uint32_t,
    pub ntp_frac: pj_uint32_t,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_rb_dlrr_item {
    pub ssrc: pj_uint32_t,
    pub lrr: pj_uint32_t,
    pub dlrr: pj_uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_rb_dlrr {
    pub header: pjmedia_rtcp_xr_rb_header,
    pub item: pjmedia_rtcp_xr_rb_dlrr_item,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_rb_stats {
    pub header: pjmedia_rtcp_xr_rb_header,
    pub ssrc: pj_uint32_t,
    pub begin_seq: pj_uint16_t,
    pub end_seq: pj_uint16_t,
    pub lost: pj_uint32_t,
    pub dup: pj_uint32_t,
    pub jitter_min: pj_uint32_t,
    pub jitter_max: pj_uint32_t,
    pub jitter_mean: pj_uint32_t,
    pub jitter_dev: pj_uint32_t,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}

impl pjmedia_rtcp_xr_rb_stats {
    #[inline]
    pub fn toh_min(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_toh_min(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn toh_max(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_toh_max(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn toh_mean(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_toh_mean(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn toh_dev(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_toh_dev(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        toh_min: pj_uint32_t,
        toh_max: pj_uint32_t,
        toh_mean: pj_uint32_t,
        toh_dev: pj_uint32_t,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let toh_min: u32 = unsafe { ::std::mem::transmute(toh_min) };
            toh_min as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let toh_max: u32 = unsafe { ::std::mem::transmute(toh_max) };
            toh_max as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let toh_mean: u32 = unsafe { ::std::mem::transmute(toh_mean) };
            toh_mean as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let toh_dev: u32 = unsafe { ::std::mem::transmute(toh_dev) };
            toh_dev as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_rb_voip_mtc {
    pub header: pjmedia_rtcp_xr_rb_header,
    pub ssrc: pj_uint32_t,
    pub loss_rate: pj_uint8_t,
    pub discard_rate: pj_uint8_t,
    pub burst_den: pj_uint8_t,
    pub gap_den: pj_uint8_t,
    pub burst_dur: pj_uint16_t,
    pub gap_dur: pj_uint16_t,
    pub rnd_trip_delay: pj_uint16_t,
    pub end_sys_delay: pj_uint16_t,
    pub signal_lvl: pj_uint8_t,
    pub noise_lvl: pj_uint8_t,
    pub rerl: pj_uint8_t,
    pub gmin: pj_uint8_t,
    pub r_factor: pj_uint8_t,
    pub ext_r_factor: pj_uint8_t,
    pub mos_lq: pj_uint8_t,
    pub mos_cq: pj_uint8_t,
    pub rx_config: pj_uint8_t,
    pub reserved2: pj_uint8_t,
    pub jb_nom: pj_uint16_t,
    pub jb_max: pj_uint16_t,
    pub jb_abs_max: pj_uint16_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_rtcp_xr_pkt {
    pub common: pjmedia_rtcp_xr_pkt__bindgen_ty_1,
    pub buf: [pj_int8_t; 104usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_pkt__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub ssrc: pj_uint32_t,
}
impl pjmedia_rtcp_xr_pkt__bindgen_ty_1 {
    #[inline]
    pub fn count(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_count(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn p(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_p(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn version(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_version(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn pt(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_pt(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn length(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_length(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        count: c_uint,
        p: c_uint,
        version: c_uint,
        pt: c_uint,
        length: c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 5u8, {
            let count: u32 = unsafe { ::std::mem::transmute(count) };
            count as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let p: u32 = unsafe { ::std::mem::transmute(p) };
            p as u64
        });
        __bindgen_bitfield_unit.set(6usize, 2u8, {
            let version: u32 = unsafe { ::std::mem::transmute(version) };
            version as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let pt: u32 = unsafe { ::std::mem::transmute(pt) };
            pt as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let length: u32 = unsafe { ::std::mem::transmute(length) };
            length as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_stream_stat {
    pub stat_sum: pjmedia_rtcp_xr_stream_stat__bindgen_ty_1,
    pub voip_mtc: pjmedia_rtcp_xr_stream_stat__bindgen_ty_2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_stream_stat__bindgen_ty_1 {
    pub update: pj_time_val,
    pub begin_seq: pj_uint32_t,
    pub end_seq: pj_uint32_t,
    pub count: c_uint,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub lost: c_uint,
    pub dup: c_uint,
    pub jitter: pj_math_stat,
    pub toh: pj_math_stat,
}

impl pjmedia_rtcp_xr_stream_stat__bindgen_ty_1 {
    #[inline]
    pub fn l(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_l(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn d(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_d(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn j(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_j(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn t(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_t(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        l: c_uint,
        d: c_uint,
        j: c_uint,
        t: c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let l: u32 = unsafe { ::std::mem::transmute(l) };
            l as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let d: u32 = unsafe { ::std::mem::transmute(d) };
            d as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let j: u32 = unsafe { ::std::mem::transmute(j) };
            j as u64
        });
        __bindgen_bitfield_unit.set(3usize, 2u8, {
            let t: u32 = unsafe { ::std::mem::transmute(t) };
            t as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_stream_stat__bindgen_ty_2 {
    pub update: pj_time_val,
    pub loss_rate: pj_uint8_t,
    pub discard_rate: pj_uint8_t,
    pub burst_den: pj_uint8_t,
    pub gap_den: pj_uint8_t,
    pub burst_dur: pj_uint16_t,
    pub gap_dur: pj_uint16_t,
    pub rnd_trip_delay: pj_uint16_t,
    pub end_sys_delay: pj_uint16_t,
    pub signal_lvl: pj_int8_t,
    pub noise_lvl: pj_int8_t,
    pub rerl: pj_uint8_t,
    pub gmin: pj_uint8_t,
    pub r_factor: pj_uint8_t,
    pub ext_r_factor: pj_uint8_t,
    pub mos_lq: pj_uint8_t,
    pub mos_cq: pj_uint8_t,
    pub rx_config: pj_uint8_t,
    pub jb_nom: pj_uint16_t,
    pub jb_max: pj_uint16_t,
    pub jb_abs_max: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_stat {
    pub rx: pjmedia_rtcp_xr_stream_stat,
    pub tx: pjmedia_rtcp_xr_stream_stat,
    pub rtt: pj_math_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_rtcp_xr_session {
    pub name: *mut c_char,
    pub pkt: pjmedia_rtcp_xr_pkt,
    pub rx_lrr: pj_uint32_t,
    pub rx_lrr_time: pj_timestamp,
    pub rx_last_rr: pj_uint32_t,
    pub stat: pjmedia_rtcp_xr_stat,
    pub src_ref_seq: pj_uint32_t,
    pub uninitialized_src_ref_seq: pj_bool_t,
    pub voip_mtc_stat: pjmedia_rtcp_xr_session__bindgen_ty_1,
    pub ptime: c_uint,
    pub frames_per_packet: c_uint,
    pub rtcp_session: *mut pjmedia_rtcp_session,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_xr_session__bindgen_ty_1 {
    pub pkt: pj_uint32_t,
    pub lost: pj_uint32_t,
    pub loss_count: pj_uint32_t,
    pub discard_count: pj_uint32_t,
    pub c11: pj_uint32_t,
    pub c13: pj_uint32_t,
    pub c14: pj_uint32_t,
    pub c22: pj_uint32_t,
    pub c23: pj_uint32_t,
    pub c33: pj_uint32_t,
}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_hdr {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub seq: pj_uint16_t,
    pub ts: pj_uint32_t,
    pub ssrc: pj_uint32_t,
}

impl pjmedia_rtp_hdr {
    #[inline]
    pub fn cc(&self) -> pj_uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_cc(&mut self, val: pj_uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn x(&self) -> pj_uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_x(&mut self, val: pj_uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn p(&self) -> pj_uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_p(&mut self, val: pj_uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn v(&self) -> pj_uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_v(&mut self, val: pj_uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn pt(&self) -> pj_uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 7u8) as u16) }
    }
    #[inline]
    pub fn set_pt(&mut self, val: pj_uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn m(&self) -> pj_uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_m(&mut self, val: pj_uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        cc: pj_uint16_t,
        x: pj_uint16_t,
        p: pj_uint16_t,
        v: pj_uint16_t,
        pt: pj_uint16_t,
        m: pj_uint16_t,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let cc: u16 = unsafe { ::std::mem::transmute(cc) };
            cc as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let x: u16 = unsafe { ::std::mem::transmute(x) };
            x as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let p: u16 = unsafe { ::std::mem::transmute(p) };
            p as u64
        });
        __bindgen_bitfield_unit.set(6usize, 2u8, {
            let v: u16 = unsafe { ::std::mem::transmute(v) };
            v as u64
        });
        __bindgen_bitfield_unit.set(8usize, 7u8, {
            let pt: u16 = unsafe { ::std::mem::transmute(pt) };
            pt as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let m: u16 = unsafe { ::std::mem::transmute(m) };
            m as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_ext_hdr {
    pub profile_data: pj_uint16_t,
    pub length: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_dec_hdr {
    pub ext_hdr: *mut pjmedia_rtp_ext_hdr,
    pub ext: *mut pj_uint32_t,
    pub ext_len: c_uint,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_dtmf_event {
    pub event: pj_uint8_t,
    pub e_vol: pj_uint8_t,
    pub duration: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_seq_session {
    pub max_seq: pj_uint16_t,
    pub cycles: pj_uint32_t,
    pub base_seq: pj_uint32_t,
    pub bad_seq: pj_uint32_t,
    pub probation: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_session {
    pub out_hdr: pjmedia_rtp_hdr,
    pub seq_ctrl: pjmedia_rtp_seq_session,
    pub out_pt: pj_uint16_t,
    pub out_extseq: pj_uint32_t,
    pub has_peer_ssrc: pj_bool_t,
    pub peer_ssrc: pj_uint32_t,
    pub received: pj_uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_rtp_status {
    pub status: pjmedia_rtp_status__bindgen_ty_1,
    pub diff: pj_uint16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjmedia_rtp_status__bindgen_ty_1 {
    pub flag: pjmedia_rtp_status__bindgen_ty_1_flag,
    pub value: pj_uint16_t,
    _bindgen_union_align: u32,
}

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_status__bindgen_ty_1_flag {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}

impl pjmedia_rtp_status__bindgen_ty_1_flag {
    #[inline]
    pub fn bad(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bad(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn badpt(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_badpt(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn badssrc(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_badssrc(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn dup(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_dup(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outorder(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outorder(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn probation(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_probation(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn restart(&self) -> c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_restart(&mut self, val: c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        bad: c_int,
        badpt: c_int,
        badssrc: c_int,
        dup: c_int,
        outorder: c_int,
        probation: c_int,
        restart: c_int,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let bad: u32 = unsafe { ::std::mem::transmute(bad) };
            bad as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let badpt: u32 = unsafe { ::std::mem::transmute(badpt) };
            badpt as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let badssrc: u32 = unsafe { ::std::mem::transmute(badssrc) };
            badssrc as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let dup: u32 = unsafe { ::std::mem::transmute(dup) };
            dup as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let outorder: u32 = unsafe { ::std::mem::transmute(outorder) };
            outorder as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let probation: u32 = unsafe { ::std::mem::transmute(probation) };
            probation as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let restart: u32 = unsafe { ::std::mem::transmute(restart) };
            restart as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtp_session_setting {
    pub flags: pj_uint8_t,
    pub default_pt: c_int,
    pub sender_ssrc: pj_uint32_t,
    pub peer_ssrc: pj_uint32_t,
    pub seq: pj_uint16_t,
    pub ts: pj_uint32_t,
}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_sr {
    pub ntp_sec: pj_uint32_t,
    pub ntp_frac: pj_uint32_t,
    pub rtp_ts: pj_uint32_t,
    pub sender_pcount: pj_uint32_t,
    pub sender_bcount: pj_uint32_t,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_rr {
    pub ssrc: pj_uint32_t,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub last_seq: pj_uint32_t,
    pub jitter: pj_uint32_t,
    pub lsr: pj_uint32_t,
    pub dlsr: pj_uint32_t,
}
impl pjmedia_rtcp_rr {
    #[inline]
    pub fn fract_lost(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_fract_lost(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn total_lost_2(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_total_lost_2(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn total_lost_1(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_total_lost_1(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn total_lost_0(&self) -> pj_uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_total_lost_0(&mut self, val: pj_uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        fract_lost: pj_uint32_t,
        total_lost_2: pj_uint32_t,
        total_lost_1: pj_uint32_t,
        total_lost_0: pj_uint32_t,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let fract_lost: u32 = unsafe { ::std::mem::transmute(fract_lost) };
            fract_lost as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let total_lost_2: u32 = unsafe { ::std::mem::transmute(total_lost_2) };
            total_lost_2 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let total_lost_1: u32 = unsafe { ::std::mem::transmute(total_lost_1) };
            total_lost_1 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let total_lost_0: u32 = unsafe { ::std::mem::transmute(total_lost_0) };
            total_lost_0 as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_common {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub ssrc: pj_uint32_t,
}
impl pjmedia_rtcp_common {
    #[inline]
    pub fn count(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_count(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn p(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_p(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn version(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_version(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn pt(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_pt(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn length(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_length(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        count: c_uint,
        p: c_uint,
        version: c_uint,
        pt: c_uint,
        length: c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 5u8, {
            let count: u32 = unsafe { ::std::mem::transmute(count) };
            count as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let p: u32 = unsafe { ::std::mem::transmute(p) };
            p as u64
        });
        __bindgen_bitfield_unit.set(6usize, 2u8, {
            let version: u32 = unsafe { ::std::mem::transmute(version) };
            version as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let pt: u32 = unsafe { ::std::mem::transmute(pt) };
            pt as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let length: u32 = unsafe { ::std::mem::transmute(length) };
            length as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_sr_pkt {
    pub common: pjmedia_rtcp_common,
    pub sr: pjmedia_rtcp_sr,
    pub rr: pjmedia_rtcp_rr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_rr_pkt {
    pub common: pjmedia_rtcp_common,
    pub rr: pjmedia_rtcp_rr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_sdes {
    pub cname: pj_str_t,
    pub name: pj_str_t,
    pub email: pj_str_t,
    pub phone: pj_str_t,
    pub loc: pj_str_t,
    pub tool: pj_str_t,
    pub note: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_ntp_rec {
    pub hi: pj_uint32_t,
    pub lo: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_stream_stat {
    pub update: pj_time_val,
    pub update_cnt: c_uint,
    pub pkt: pj_uint32_t,
    pub bytes: pj_uint32_t,
    pub discard: c_uint,
    pub loss: c_uint,
    pub reorder: c_uint,
    pub dup: c_uint,
    pub loss_period: pj_math_stat,
    pub loss_type: pjmedia_rtcp_stream_stat__bindgen_ty_1,
    pub jitter: pj_math_stat,
}

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_stream_stat__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
impl pjmedia_rtcp_stream_stat__bindgen_ty_1 {
    #[inline]
    pub fn burst(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_burst(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn random(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_random(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        burst: c_uint,
        random: c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let burst: u32 = unsafe { ::std::mem::transmute(burst) };
            burst as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let random: u32 = unsafe { ::std::mem::transmute(random) };
            random as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_rtcp_stat {
    pub start: pj_time_val,
    pub tx: pjmedia_rtcp_stream_stat,
    pub rx: pjmedia_rtcp_stream_stat,
    pub rtt: pj_math_stat,
    pub rtp_tx_last_ts: pj_uint32_t,
    pub rtp_tx_last_seq: pj_uint16_t,
    pub peer_sdes: pjmedia_rtcp_sdes,
    pub peer_sdes_buf_: [c_char; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_rtcp_session {
    pub name: *mut c_char,
    pub rtcp_sr_pkt: pjmedia_rtcp_sr_pkt,
    pub rtcp_rr_pkt: pjmedia_rtcp_rr_pkt,
    pub seq_ctrl: pjmedia_rtp_seq_session,
    pub rtp_last_ts: c_uint,
    pub clock_rate: c_uint,
    pub pkt_size: c_uint,
    pub received: pj_uint32_t,
    pub exp_prior: pj_uint32_t,
    pub rx_prior: pj_uint32_t,
    pub transit: pj_int32_t,
    pub jitter: pj_uint32_t,
    pub tv_base: pj_time_val,
    pub ts_base: pj_timestamp,
    pub ts_freq: pj_timestamp,
    pub rtp_ts_base: pj_uint32_t,
    pub rx_lsr: pj_uint32_t,
    pub rx_lsr_time: pj_timestamp,
    pub peer_ssrc: pj_uint32_t,
    pub stat: pjmedia_rtcp_stat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_session_setting {
    pub name: *mut c_char,
    pub clock_rate: c_uint,
    pub samples_per_frame: c_uint,
    pub ssrc: pj_uint32_t,
    pub rtp_ts_base: pj_uint32_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_attr {
    pub name: pj_str_t,
    pub value: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_rtpmap {
    pub pt: pj_str_t,
    pub enc_name: pj_str_t,
    pub clock_rate: c_uint,
    pub param: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_fmtp {
    pub fmt: pj_str_t,
    pub fmt_param: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_rtcp_attr {
    pub port: c_uint,
    pub net_type: pj_str_t,
    pub addr_type: pj_str_t,
    pub addr: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_ssrc_attr {
    pub ssrc: pj_uint32_t,
    pub cname: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_conn {
    pub net_type: pj_str_t,
    pub addr_type: pj_str_t,
    pub addr: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_bandw {
    pub modifier: pj_str_t,
    pub value: pj_uint32_t,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_sdp_media {
    pub desc: pjmedia_sdp_media__bindgen_ty_1,
    pub conn: *mut pjmedia_sdp_conn,
    pub bandw_count: c_uint,
    pub bandw: [*mut pjmedia_sdp_bandw; 4usize],
    pub attr_count: c_uint,
    pub attr: [*mut pjmedia_sdp_attr; 68usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_media__bindgen_ty_1 {
    pub media: pj_str_t,
    pub port: pj_uint16_t,
    pub port_count: c_uint,
    pub transport: pj_str_t,
    pub fmt_count: c_uint,
    pub fmt: [pj_str_t; 32usize],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_sdp_session {
    pub origin: pjmedia_sdp_session__bindgen_ty_1,
    pub name: pj_str_t,
    pub conn: *mut pjmedia_sdp_conn,
    pub bandw_count: c_uint,
    pub bandw: [*mut pjmedia_sdp_bandw; 4usize],
    pub time: pjmedia_sdp_session__bindgen_ty_2,
    pub attr_count: c_uint,
    pub attr: [*mut pjmedia_sdp_attr; 68usize],
    pub media_count: c_uint,
    pub media: [*mut pjmedia_sdp_media; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_session__bindgen_ty_1 {
    pub user: pj_str_t,
    pub id: pj_uint32_t,
    pub version: pj_uint32_t,
    pub net_type: pj_str_t,
    pub addr_type: pj_str_t,
    pub addr: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_session__bindgen_ty_2 {
    pub start: pj_uint32_t,
    pub stop: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_cap {
    pub codec_id: pj_str_t,
    pub type_: pjmedia_rtcp_fb_type,
    pub type_name: pj_str_t,
    pub param: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_info {
    pub cap_count: c_uint,
    pub caps: [pjmedia_rtcp_fb_cap; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_setting {
    pub dont_use_avpf: pj_bool_t,
    pub cap_count: c_uint,
    pub caps: [pjmedia_rtcp_fb_cap; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_nack {
    pub pid: pj_int32_t,
    pub blp: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_sli {
    pub first: pj_uint16_t,
    pub number: pj_uint16_t,
    pub pict_id: pj_uint8_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_rpsi {
    pub pt: pj_uint8_t,
    pub rpsi: pj_str_t,
    pub rpsi_bit_len: pj_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_event_rx_rtcp_fb_data {
    pub cap: pjmedia_rtcp_fb_cap,
    pub msg: pjmedia_event_rx_rtcp_fb_data__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjmedia_event_rx_rtcp_fb_data__bindgen_ty_1 {
    pub nack: pjmedia_rtcp_fb_nack,
    pub sli: pjmedia_rtcp_fb_sli,
    pub rpsi: pjmedia_rtcp_fb_rpsi,
    _bindgen_union_align: [u64; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd {
    pub type_: pjmedia_vid_dev_hwnd_type,
    pub info: pjmedia_vid_dev_hwnd__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjmedia_vid_dev_hwnd__bindgen_ty_1 {
    pub win: pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_1,
    pub x11: pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_2,
    pub cocoa: pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_3,
    pub ios: pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_4,
    pub android: pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_5,
    pub window: *mut c_void,
    _bindgen_union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_1 {
    pub hwnd: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_2 {
    pub window: *mut c_void,
    pub display: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_3 {
    pub window: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_4 {
    pub window: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_5 {
    pub window: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_switch_param {
    pub target_id: pjmedia_vid_dev_index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_vid_dev_info {
    pub id: pjmedia_vid_dev_index,
    pub name: [c_char; 64usize],
    pub driver: [c_char; 32usize],
    pub dir: pjmedia_dir,
    pub has_callback: pj_bool_t,
    pub caps: c_uint,
    pub fmt_cnt: c_uint,
    pub fmt: [pjmedia_format; 64usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_cb {
    pub capture_cb: Option<
        unsafe extern "C" fn(
            stream: *mut pjmedia_vid_dev_stream,
            user_data: *mut c_void,
            frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub render_cb: Option<
        unsafe extern "C" fn(
            stream: *mut pjmedia_vid_dev_stream,
            user_data: *mut c_void,
            frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_vid_dev_param {
    pub dir: pjmedia_dir,
    pub cap_id: pjmedia_vid_dev_index,
    pub rend_id: pjmedia_vid_dev_index,
    pub clock_rate: c_uint,
    pub flags: c_uint,
    pub fmt: pjmedia_format,
    pub window: pjmedia_vid_dev_hwnd,
    pub disp_size: pjmedia_rect_size,
    pub window_pos: pjmedia_coord,
    pub window_hide: pj_bool_t,
    pub native_preview: pj_bool_t,
    pub orient: pjmedia_orient,
    pub window_flags: c_uint,
    pub window_fullscreen: pj_bool_t,
}

pub type pjmedia_vid_dev_factory_create_func_ptr = Option<
    unsafe extern "C" fn(arg1: *mut pj_pool_factory) -> *mut pjmedia_vid_dev_factory,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_driver {
    pub create: pjmedia_vid_dev_factory_create_func_ptr,
    pub f: *mut pjmedia_vid_dev_factory,
    pub name: [c_char; 32usize],
    pub dev_cnt: c_uint,
    pub start_idx: c_uint,
    pub cap_dev_idx: c_int,
    pub rend_dev_idx: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_subsys {
    pub init_count: c_uint,
    pub pf: *mut pj_pool_factory,
    pub drv_cnt: c_uint,
    pub drv: [pjmedia_vid_driver; 8usize],
    pub dev_cnt: c_uint,
    pub dev_list: [pj_uint32_t; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_event_fmt_changed_data {
    pub dir: pjmedia_dir,
    pub new_fmt: pjmedia_format,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_dummy_data {
    pub dummy: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_wnd_resized_data {
    pub new_size: pjmedia_rect_size,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_wnd_closing_data {
    pub cancel: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_aud_dev_err_data {
    pub dir: pjmedia_dir,
    pub id: pjmedia_aud_dev_index,
    pub status: pj_status_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_vid_dev_err_data {
    pub dir: pjmedia_dir,
    pub id: pjmedia_vid_dev_index,
    pub status: pj_status_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_media_tp_err_data {
    pub type_: pjmedia_type,
    pub is_rtp: pj_bool_t,
    pub dir: pjmedia_dir,
    pub status: pj_status_t,
}

pub type pjmedia_event_wnd_closed_data = pjmedia_event_dummy_data;
pub type pjmedia_event_mouse_btn_down_data = pjmedia_event_dummy_data;
pub type pjmedia_event_keyframe_found_data = pjmedia_event_dummy_data;
pub type pjmedia_event_keyframe_missing_data = pjmedia_event_dummy_data;
pub type pjmedia_event_user_data = [c_char; 40usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_event {
    pub type_: pjmedia_event_type,
    pub timestamp: pj_timestamp,
    pub src: *const c_void,
    pub epub: *const c_void,
    pub data: pjmedia_event__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjmedia_event__bindgen_ty_1 {
    pub fmt_changed: pjmedia_event_fmt_changed_data,
    pub wnd_resized: pjmedia_event_wnd_resized_data,
    pub wnd_closing: pjmedia_event_wnd_closing_data,
    pub wnd_closed: pjmedia_event_wnd_closed_data,
    pub mouse_btn_down: pjmedia_event_mouse_btn_down_data,
    pub keyframe_found: pjmedia_event_keyframe_found_data,
    pub keyframe_missing: pjmedia_event_keyframe_missing_data,
    pub aud_dev_err: pjmedia_event_aud_dev_err_data,
    pub vid_dev_err: pjmedia_event_vid_dev_err_data,
    pub user: pjmedia_event_user_data,
    pub med_tp_err: pjmedia_event_media_tp_err_data,
    pub rx_rtcp_fb: pjmedia_event_rx_rtcp_fb_data,
    pub ptr: *mut c_void,
    _bindgen_union_align: [u64; 11usize],
}
pub type pjmedia_event_cb = Option<
    unsafe extern "C" fn(
        event: *mut pjmedia_event,
        user_data: *mut c_void,
    ) -> pj_status_t,
>;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_mgr {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_port_info {
    pub name: pj_str_t,
    pub signature: pj_uint32_t,
    pub dir: pjmedia_dir,
    pub fmt: pjmedia_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_port {
    pub info: pjmedia_port_info,
    pub port_data: pjmedia_port_port_data,
    pub get_clock_src: Option<
        unsafe extern "C" fn(
            this_port: *mut pjmedia_port,
            dir: pjmedia_dir,
        ) -> *mut pjmedia_clock_src,
    >,
    pub put_frame: Option<
        unsafe extern "C" fn(
            this_port: *mut pjmedia_port,
            frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub get_frame: Option<
        unsafe extern "C" fn(
            this_port: *mut pjmedia_port,
            frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub on_destroy:
        Option<unsafe extern "C" fn(this_port: *mut pjmedia_port) -> pj_status_t>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_port_port_data {
    pub pdata: *mut c_void,
    pub ldata: c_long,
}


// pub const pjmedia_avi_file_player_option_PJMEDIA_AVI_FILE_NO_LOOP: pjmedia_avi_file_player_option = 1;
// pub type pjmedia_avi_file_player_option = c_uint;
// pub type pjmedia_avi_stream = pjmedia_port;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct pjmedia_avi_streams {
//     _unused: [u8; 0],
// }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_circ_buf {
    pub buf: *mut pj_int16_t,
    pub capacity: c_uint,
    pub start: *mut pj_int16_t,
    pub len: c_uint,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_info {
    pub type_: pjmedia_type,
    pub pt: c_uint,
    pub encoding_name: pj_str_t,
    pub clock_rate: c_uint,
    pub channel_cnt: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_fmtp {
    pub cnt: pj_uint8_t,
    pub param: [pjmedia_codec_fmtp_param; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_fmtp_param {
    pub name: pj_str_t,
    pub val: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_param {
    pub info: pjmedia_codec_param__bindgen_ty_1,
    pub setting: pjmedia_codec_param__bindgen_ty_2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_param__bindgen_ty_1 {
    pub clock_rate: c_uint,
    pub channel_cnt: c_uint,
    pub avg_bps: pj_uint32_t,
    pub max_bps: pj_uint32_t,
    pub max_rx_frame_size: c_uint,
    pub frm_ptime: pj_uint16_t,
    pub enc_ptime: pj_uint16_t,
    pub pcm_bits_per_sample: pj_uint8_t,
    pub pt: pj_uint8_t,
    pub fmt_id: pjmedia_format_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_param__bindgen_ty_2 {
    pub frm_per_pkt: pj_uint8_t,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub enc_fmtp: pjmedia_codec_fmtp,
    pub dec_fmtp: pjmedia_codec_fmtp,
}

impl pjmedia_codec_param__bindgen_ty_2 {
    #[inline]
    pub fn vad(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_vad(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn cng(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_cng(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn penh(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_penh(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn plc(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_plc(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        vad: c_uint,
        cng: c_uint,
        penh: c_uint,
        plc: c_uint,
        reserved: c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let vad: u32 = unsafe { ::std::mem::transmute(vad) };
            vad as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let cng: u32 = unsafe { ::std::mem::transmute(cng) };
            cng as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let penh: u32 = unsafe { ::std::mem::transmute(penh) };
            penh as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let plc: u32 = unsafe { ::std::mem::transmute(plc) };
            plc as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let reserved: u32 = unsafe { ::std::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_op {
    pub init: Option<
        unsafe extern "C" fn(codec: *mut pjmedia_codec, pool: *mut pj_pool_t) -> pj_status_t,
    >,
    pub open: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_codec,
            param: *mut pjmedia_codec_param,
        ) -> pj_status_t,
    >,
    pub close:
        Option<unsafe extern "C" fn(codec: *mut pjmedia_codec) -> pj_status_t>,
    pub modify: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_codec,
            param: *const pjmedia_codec_param,
        ) -> pj_status_t,
    >,
    pub parse: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_codec,
            pkt: *mut c_void,
            pkt_size: pj_size_t,
            timestamp: *const pj_timestamp,
            frame_cnt: *mut c_uint,
            frames: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub encode: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_codec,
            input: *const pjmedia_frame,
            out_size: c_uint,
            output: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub decode: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_codec,
            input: *const pjmedia_frame,
            out_size: c_uint,
            output: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub recover: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_codec,
            out_size: c_uint,
            output: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec {
    pub prev: *mut pjmedia_codec,
    pub next: *mut pjmedia_codec,
    pub codec_data: *mut c_void,
    pub factory: *mut pjmedia_codec_factory,
    pub op: *mut pjmedia_codec_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_factory_op {
    pub test_alloc: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_codec_factory,
            info: *const pjmedia_codec_info,
        ) -> pj_status_t,
    >,
    pub default_attr: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_codec_factory,
            info: *const pjmedia_codec_info,
            attr: *mut pjmedia_codec_param,
        ) -> pj_status_t,
    >,
    pub enum_info: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_codec_factory,
            count: *mut c_uint,
            codecs: *mut pjmedia_codec_info,
        ) -> pj_status_t,
    >,
    pub alloc_codec: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_codec_factory,
            info: *const pjmedia_codec_info,
            p_codec: *mut *mut pjmedia_codec,
        ) -> pj_status_t,
    >,
    pub dealloc_codec: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_codec_factory,
            codec: *mut pjmedia_codec,
        ) -> pj_status_t,
    >,
    pub destroy: Option<unsafe extern "C" fn() -> pj_status_t>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_factory {
    pub prev: *mut pjmedia_codec_factory,
    pub next: *mut pjmedia_codec_factory,
    pub factory_data: *mut c_void,
    pub op: *mut pjmedia_codec_factory_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_default_param {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_desc {
    pub info: pjmedia_codec_info,
    pub id: pjmedia_codec_id,
    pub prio: pjmedia_codec_priority,
    pub factory: *mut pjmedia_codec_factory,
    pub param: *mut pjmedia_codec_default_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_mgr {
    pub pf: *mut pj_pool_factory,
    pub pool: *mut pj_pool_t,
    pub mutex: *mut pj_mutex_t,
    pub factory_list: pjmedia_codec_factory,
    pub codec_cnt: c_uint,
    pub codec_desc: [pjmedia_codec_desc; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_conf {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_conf_port_info {
    pub slot: c_uint,
    pub name: pj_str_t,
    pub format: pjmedia_format,
    pub tx_setting: pjmedia_port_op,
    pub rx_setting: pjmedia_port_op,
    pub listener_cnt: c_uint,
    pub listener_slots: *mut c_uint,
    pub listener_adj_level: *mut c_uint,
    pub transmitter_cnt: c_uint,
    pub clock_rate: c_uint,
    pub channel_count: c_uint,
    pub samples_per_frame: c_uint,
    pub bits_per_sample: c_uint,
    pub tx_adj_level: c_int,
    pub rx_adj_level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_conversion_param {
    pub src: pjmedia_format,
    pub dst: pjmedia_format,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_converter_factory {
    pub prev: *mut pjmedia_converter_factory,
    pub next: *mut pjmedia_converter_factory,
    pub name: *const c_char,
    pub priority: c_int,
    pub op: *mut pjmedia_converter_factory_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_converter {
    pub op: *mut pjmedia_converter_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_converter_factory_op {
    pub create_converter: Option<
        unsafe extern "C" fn(
            cf: *mut pjmedia_converter_factory,
            pool: *mut pj_pool_t,
            prm: *const pjmedia_conversion_param,
            p_cv: *mut *mut pjmedia_converter,
        ) -> pj_status_t,
    >,
    pub destroy_factory:
        Option<unsafe extern "C" fn(cf: *mut pjmedia_converter_factory)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_converter_op {
    pub convert: Option<
        unsafe extern "C" fn(
            cv: *mut pjmedia_converter,
            src_frame: *mut pjmedia_frame,
            dst_frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub destroy: Option<unsafe extern "C" fn(cv: *mut pjmedia_converter)>,
    pub convert2: Option<
        unsafe extern "C" fn(
            cv: *mut pjmedia_converter,
            src_frame: *mut pjmedia_frame,
            src_frame_size: *const pjmedia_rect_size,
            src_pos: *const pjmedia_coord,
            dst_frame: *mut pjmedia_frame,
            dst_frame_size: *const pjmedia_rect_size,
            dst_pos: *const pjmedia_coord,
            param: *mut pjmedia_converter_convert_setting,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_converter_mgr {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_delay_buf {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_echo_state {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_echo_stat {
    pub name: *const c_char,
    pub median: c_int,
    pub std: c_int,
    pub frac_delay: f32,
    pub learning: c_uint,
    pub duration: c_uint,
    pub tail: c_uint,
    pub min_factor: c_int,
    pub avg_factor: c_int,
    pub stat_info: pj_str_t,
    pub buf_: [c_char; 128usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_sock_info {
    pub rtp_sock: pj_sock_t,
    pub rtp_addr_name: pj_sockaddr,
    pub rtcp_sock: pj_sock_t,
    pub rtcp_addr_name: pj_sockaddr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_transport_op {
    pub get_info: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            info: *mut pjmedia_transport_info,
        ) -> pj_status_t,
    >,
    pub attach: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            user_data: *mut c_void,
            rem_addr: *const pj_sockaddr_t,
            rem_rtcp: *const pj_sockaddr_t,
            addr_len: c_uint,
            rtp_cb: Option<
                unsafe extern "C" fn(
                    user_data: *mut c_void,
                    pkt: *mut c_void,
                    size: pj_ssize_t,
                ),
            >,
            rtcp_cb: Option<
                unsafe extern "C" fn(
                    user_data: *mut c_void,
                    pkt: *mut c_void,
                    size: pj_ssize_t,
                ),
            >,
        ) -> pj_status_t,
    >,
    pub detach: Option<
        unsafe extern "C" fn(tp: *mut pjmedia_transport, user_data: *mut c_void),
    >,
    pub send_rtp: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            pkt: *const c_void,
            size: pj_size_t,
        ) -> pj_status_t,
    >,
    pub send_rtcp: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            pkt: *const c_void,
            size: pj_size_t,
        ) -> pj_status_t,
    >,
    pub send_rtcp2: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            addr: *const pj_sockaddr_t,
            addr_len: c_uint,
            pkt: *const c_void,
            size: pj_size_t,
        ) -> pj_status_t,
    >,
    pub media_create: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            sdp_pool: *mut pj_pool_t,
            options: c_uint,
            remote_sdp: *const pjmedia_sdp_session,
            media_index: c_uint,
        ) -> pj_status_t,
    >,
    pub encode_sdp: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            sdp_pool: *mut pj_pool_t,
            sdp_local: *mut pjmedia_sdp_session,
            rem_sdp: *const pjmedia_sdp_session,
            media_index: c_uint,
        ) -> pj_status_t,
    >,
    pub media_start: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            tmp_pool: *mut pj_pool_t,
            sdp_local: *const pjmedia_sdp_session,
            sdp_remote: *const pjmedia_sdp_session,
            media_index: c_uint,
        ) -> pj_status_t,
    >,
    pub media_stop:
        Option<unsafe extern "C" fn(tp: *mut pjmedia_transport) -> pj_status_t>,
    pub simulate_lost: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            dir: pjmedia_dir,
            pct_lost: c_uint,
        ) -> pj_status_t,
    >,
    pub destroy:
        Option<unsafe extern "C" fn(tp: *mut pjmedia_transport) -> pj_status_t>,
    pub attach2: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            att_param: *mut pjmedia_transport_attach_param,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_transport {
    pub name: [c_char; 32usize],
    pub type_: pjmedia_transport_type,
    pub op: *mut pjmedia_transport_op,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_transport_specific_info {
    pub type_: pjmedia_transport_type,
    pub cbsize: c_int,
    pub buffer: [c_char; 288usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_transport_info {
    pub sock_info: pjmedia_sock_info,
    pub src_rtp_name: pj_sockaddr,
    pub src_rtcp_name: pj_sockaddr,
    pub specific_info_cnt: c_uint,
    pub spc_info: [pjmedia_transport_specific_info; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_tp_cb_param {
    pub user_data: *mut c_void,
    pub pkt: *mut c_void,
    pub size: pj_ssize_t,
    pub src_addr: *mut pj_sockaddr,
    pub rem_switch: pj_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_transport_attach_param {
    pub stream: *mut c_void,
    pub media_type: pjmedia_type,
    pub rem_addr: pj_sockaddr,
    pub rem_rtcp: pj_sockaddr,
    pub addr_len: c_uint,
    pub user_data: *mut c_void,
    pub rtp_cb: Option<
        unsafe extern "C" fn(
            user_data: *mut c_void,
            pkt: *mut c_void,
            arg1: pj_ssize_t,
        ),
    >,
    pub rtcp_cb: Option<
        unsafe extern "C" fn(
            user_data: *mut c_void,
            pkt: *mut c_void,
            arg1: pj_ssize_t,
        ),
    >,
    pub rtp_cb2: Option<unsafe extern "C" fn(param: *mut pjmedia_tp_cb_param)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_jb_state {
    pub frame_size: c_uint,
    pub min_prefetch: c_uint,
    pub max_prefetch: c_uint,
    pub max_count: c_uint,
    pub burst: c_uint,
    pub prefetch: c_uint,
    pub size: c_uint,
    pub avg_delay: c_uint,
    pub min_delay: c_uint,
    pub max_delay: c_uint,
    pub dev_delay: c_uint,
    pub avg_burst: c_uint,
    pub lost: c_uint,
    pub discard: c_uint,
    pub empty: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_jbuf {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_master_port {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_plc {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_resample {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_neg {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_silence_det {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_snd_stream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_snd_dev_info {
    pub name: [c_char; 64usize],
    pub input_count: c_uint,
    pub output_count: c_uint,
    pub default_samples_per_sec: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_snd_stream_info {
    pub dir: pjmedia_dir,
    pub play_id: c_int,
    pub rec_id: c_int,
    pub clock_rate: c_uint,
    pub channel_count: c_uint,
    pub samples_per_frame: c_uint,
    pub bits_per_sample: c_uint,
    pub rec_latency: c_uint,
    pub play_latency: c_uint,
}
pub type pjmedia_snd_play_cb = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        timestamp: pj_uint32_t,
        output: *mut c_void,
        size: c_uint,
    ) -> pj_status_t,
>;
pub type pjmedia_snd_rec_cb = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        timestamp: pj_uint32_t,
        input: *mut c_void,
        size: c_uint,
    ) -> pj_status_t,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_snd_port_param {
    pub base: pjmedia_aud_param,
    pub options: c_uint,
    pub ec_options: c_uint,
    pub user_data: *mut c_void,
    pub on_play_frame: pjmedia_aud_play_cb,
    pub on_rec_frame: pjmedia_aud_rec_cb,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_snd_port {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_encode_opt {
    pub force_keyframe: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_codec_info {
    pub fmt_id: pjmedia_format_id,
    pub pt: c_uint,
    pub encoding_name: pj_str_t,
    pub encoding_desc: pj_str_t,
    pub clock_rate: c_uint,
    pub dir: pjmedia_dir,
    pub dec_fmt_id_cnt: c_uint,
    pub dec_fmt_id: [pjmedia_format_id; 8usize],
    pub packings: c_uint,
    pub fps_cnt: c_uint,
    pub fps: [pjmedia_ratio; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_vid_codec_param {
    pub dir: pjmedia_dir,
    pub packing: pjmedia_vid_packing,
    pub enc_fmt: pjmedia_format,
    pub enc_fmtp: pjmedia_codec_fmtp,
    pub enc_mtu: c_uint,
    pub dec_fmt: pjmedia_format,
    pub dec_fmtp: pjmedia_codec_fmtp,
    pub ignore_fmtp: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_codec_op {
    pub init: Option<
        unsafe extern "C" fn(codec: *mut pjmedia_vid_codec, pool: *mut pj_pool_t) -> pj_status_t,
    >,
    pub open: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            param: *mut pjmedia_vid_codec_param,
        ) -> pj_status_t,
    >,
    pub close:
        Option<unsafe extern "C" fn(codec: *mut pjmedia_vid_codec) -> pj_status_t>,
    pub modify: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            param: *const pjmedia_vid_codec_param,
        ) -> pj_status_t,
    >,
    pub get_param: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            param: *mut pjmedia_vid_codec_param,
        ) -> pj_status_t,
    >,
    pub encode_begin: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            opt: *const pjmedia_vid_encode_opt,
            input: *const pjmedia_frame,
            out_size: c_uint,
            output: *mut pjmedia_frame,
            has_more: *mut pj_bool_t,
        ) -> pj_status_t,
    >,
    pub encode_more: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            out_size: c_uint,
            output: *mut pjmedia_frame,
            has_more: *mut pj_bool_t,
        ) -> pj_status_t,
    >,
    pub decode: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            count: pj_size_t,
            packets: *mut pjmedia_frame,
            out_size: c_uint,
            output: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub recover: Option<
        unsafe extern "C" fn(
            codec: *mut pjmedia_vid_codec,
            out_size: c_uint,
            output: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_codec {
    pub prev: *mut pjmedia_vid_codec,
    pub next: *mut pjmedia_vid_codec,
    pub codec_data: *mut c_void,
    pub factory: *mut pjmedia_vid_codec_factory,
    pub op: *mut pjmedia_vid_codec_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_codec_factory_op {
    pub test_alloc: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_vid_codec_factory,
            info: *const pjmedia_vid_codec_info,
        ) -> pj_status_t,
    >,
    pub default_attr: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_vid_codec_factory,
            info: *const pjmedia_vid_codec_info,
            attr: *mut pjmedia_vid_codec_param,
        ) -> pj_status_t,
    >,
    pub enum_info: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_vid_codec_factory,
            count: *mut c_uint,
            codecs: *mut pjmedia_vid_codec_info,
        ) -> pj_status_t,
    >,
    pub alloc_codec: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_vid_codec_factory,
            info: *const pjmedia_vid_codec_info,
            p_codec: *mut *mut pjmedia_vid_codec,
        ) -> pj_status_t,
    >,
    pub dealloc_codec: Option<
        unsafe extern "C" fn(
            factory: *mut pjmedia_vid_codec_factory,
            codec: *mut pjmedia_vid_codec,
        ) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_codec_factory {
    pub prev: *mut pjmedia_vid_codec_factory,
    pub next: *mut pjmedia_vid_codec_factory,
    pub factory_data: *mut c_void,
    pub op: *mut pjmedia_vid_codec_factory_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_codec_mgr {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_stream_rtp_sess_info {
    pub rx_rtp: *const pjmedia_rtp_session,
    pub tx_rtp: *const pjmedia_rtp_session,
    pub rtcp: *const pjmedia_rtcp_session,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_channel {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_stream_info {
    pub type_: pjmedia_type,
    pub proto: pjmedia_tp_proto,
    pub dir: pjmedia_dir,
    pub rem_addr: pj_sockaddr,
    pub rem_rtcp: pj_sockaddr,
    pub rtcp_mux: pj_bool_t,
    pub loc_rtcp_fb: pjmedia_rtcp_fb_info,
    pub rem_rtcp_fb: pjmedia_rtcp_fb_info,
    pub fmt: pjmedia_codec_info,
    pub param: *mut pjmedia_codec_param,
    pub tx_pt: c_uint,
    pub rx_pt: c_uint,
    pub tx_maxptime: c_uint,
    pub tx_event_pt: c_int,
    pub rx_event_pt: c_int,
    pub ssrc: pj_uint32_t,
    pub cname: pj_str_t,
    pub has_rem_ssrc: pj_bool_t,
    pub rem_ssrc: pj_uint32_t,
    pub rem_cname: pj_str_t,
    pub rtp_ts: pj_uint32_t,
    pub rtp_seq: pj_uint16_t,
    pub rtp_seq_ts_set: pj_uint8_t,
    pub jb_init: c_int,
    pub jb_min_pre: c_int,
    pub jb_max_pre: c_int,
    pub jb_max: c_int,
    pub jb_discard_algo: pjmedia_jb_discard_algo,
    pub rtcp_sdes_bye_disabled: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_stream_dtmf_event {
    pub digit: c_int,
    pub timestamp: pj_uint32_t,
    pub duration: pj_uint16_t,
    pub flags: pj_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_tone_desc {
    pub freq1: c_short,
    pub freq2: c_short,
    pub on_msec: c_short,
    pub off_msec: c_short,
    pub volume: c_short,
    pub flags: c_short,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_tone_digit {
    pub digit: c_char,
    pub on_msec: c_short,
    pub off_msec: c_short,
    pub volume: c_short,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_tone_digit_map {
    pub count: c_uint,
    pub digits: [pjmedia_tone_digit_map__bindgen_ty_1; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_tone_digit_map__bindgen_ty_1 {
    pub digit: c_char,
    pub freq1: c_short,
    pub freq2: c_short,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_ice_cb {
    pub on_ice_complete: Option<
        unsafe extern "C" fn(tp: *mut pjmedia_transport, op: pj_ice_strans_op, status: pj_status_t),
    >,
    pub on_ice_complete2: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            op: pj_ice_strans_op,
            status: pj_status_t,
            user_data: *mut c_void,
        ),
    >,
    pub on_new_candidate: Option<
        unsafe extern "C" fn(
            tp: *mut pjmedia_transport,
            cand: *const pj_ice_sess_cand,
            last: pj_bool_t,
        ),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_ice_transport_info {
    pub active: pj_bool_t,
    pub sess_state: pj_ice_strans_state,
    pub role: pj_ice_sess_role,
    pub comp_cnt: c_uint,
    pub comp: [pjmedia_ice_transport_info__bindgen_ty_1; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_ice_transport_info__bindgen_ty_1 {
    pub lcand_type: pj_ice_cand_type,
    pub lcand_addr: pj_sockaddr,
    pub rcand_type: pj_ice_cand_type,
    pub rcand_addr: pj_sockaddr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_loop_tp_setting {
    pub af: c_int,
    pub addr: pj_str_t,
    pub port: c_int,
    pub disable_rx: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_srtp_crypto {
    pub key: pj_str_t,
    pub name: pj_str_t,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_srtp_cb {
    pub on_srtp_nego_complete: Option<
        unsafe extern "C" fn(tp: *mut pjmedia_transport, status: pj_status_t),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_srtp_setting {
    pub use_: pjmedia_srtp_use,
    pub close_member_tp: pj_bool_t,
    pub crypto_count: c_uint,
    pub crypto: [pjmedia_srtp_crypto; 16usize],
    pub keying_count: c_uint,
    pub keying: [pjmedia_srtp_keying_method; 2usize],
    pub cb: pjmedia_srtp_cb,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_srtp_info {
    pub active: pj_bool_t,
    pub rx_policy: pjmedia_srtp_crypto,
    pub tx_policy: pjmedia_srtp_crypto,
    pub use_: pjmedia_srtp_use,
    pub peer_use: pjmedia_srtp_use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_srtp_dtls_nego_param {
    pub rem_fingerprint: pj_str_t,
    pub rem_addr: pj_sockaddr,
    pub rem_rtcp: pj_sockaddr,
    pub is_role_active: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_conf {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_conf_setting {
    pub max_slot_cnt: c_uint,
    pub frame_rate: c_uint,
    pub layout: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_vid_conf_port_info {
    pub slot: c_uint,
    pub name: pj_str_t,
    pub format: pjmedia_format,
    pub listener_cnt: c_uint,
    pub listener_slots: *mut c_uint,
    pub transmitter_cnt: c_uint,
    pub transmitter_slots: *mut c_uint,
}

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct pjmedia_vid_port_param {
//     pub vidparam: pjmedia_vid_dev_param,
//     pub active: pj_bool_t,
// }

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct pjmedia_vid_port {
//     _unused: [u8; 0],
// }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_stream_rc_config {
    pub method: pjmedia_vid_stream_rc_method,
    pub bandwidth: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_stream_sk_config {
    pub count: c_uint,
    pub interval: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_vid_stream_info {
    pub type_: pjmedia_type,
    pub proto: pjmedia_tp_proto,
    pub dir: pjmedia_dir,
    pub rem_addr: pj_sockaddr,
    pub rem_rtcp: pj_sockaddr,
    pub rtcp_mux: pj_bool_t,
    pub loc_rtcp_fb: pjmedia_rtcp_fb_info,
    pub rem_rtcp_fb: pjmedia_rtcp_fb_info,
    pub tx_pt: c_uint,
    pub rx_pt: c_uint,
    pub ssrc: pj_uint32_t,
    pub cname: pj_str_t,
    pub has_rem_ssrc: pj_bool_t,
    pub rem_ssrc: pj_uint32_t,
    pub rem_cname: pj_str_t,
    pub rtp_ts: pj_uint32_t,
    pub rtp_seq: pj_uint16_t,
    pub rtp_seq_ts_set: pj_uint8_t,
    pub jb_init: c_int,
    pub jb_min_pre: c_int,
    pub jb_max_pre: c_int,
    pub jb_max: c_int,
    pub codec_info: pjmedia_vid_codec_info,
    pub codec_param: *mut pjmedia_vid_codec_param,
    pub rtcp_sdes_bye_disabled: pj_bool_t,
    pub rc_cfg: pjmedia_vid_stream_rc_config,
    pub sk_cfg: pjmedia_vid_stream_sk_config,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_stream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wav_player_info {
    pub fmt_id: pjmedia_format_id,
    pub payload_bits_per_sample: c_uint,
    pub size_bytes: pj_uint32_t,
    pub size_samples: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wave_hdr {
    pub riff_hdr: pjmedia_wave_hdr__bindgen_ty_1,
    pub fmt_hdr: pjmedia_wave_hdr__bindgen_ty_2,
    pub data_hdr: pjmedia_wave_hdr__bindgen_ty_3,
}

#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wave_hdr__bindgen_ty_1 {
    pub riff: pj_uint32_t,
    pub file_len: pj_uint32_t,
    pub wave: pj_uint32_t,
}

#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wave_hdr__bindgen_ty_2 {
    pub fmt: pj_uint32_t,
    pub len: pj_uint32_t,
    pub fmt_tag: pj_uint16_t,
    pub nchan: pj_uint16_t,
    pub sample_rate: pj_uint32_t,
    pub bytes_per_sec: pj_uint32_t,
    pub block_align: pj_uint16_t,
    pub bits_per_sample: pj_uint16_t,
}

#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wave_hdr__bindgen_ty_3 {
    pub data: pj_uint32_t,
    pub len: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wave_subchunk {
    pub id: pj_uint32_t,
    pub len: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_wsola {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_passthrough_setting {
    pub fmt_cnt: c_uint,
    pub fmts: *mut pjmedia_format,
    pub ilbc_mode: c_uint,
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
    pub option: c_uint,
    pub quality: c_int,
    pub complexity: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_codec_config__bindgen_ty_2 {
    pub mode: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_codec_config__bindgen_ty_3 {
    pub setting: pjmedia_codec_passthrough_setting,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_amr_config {
    pub octet_align: pj_bool_t,
    pub bitrate: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_opus_config {
    pub sample_rate: c_uint,
    pub channel_cnt: c_uint,
    pub frm_ptime: c_uint,
    pub bit_rate: c_uint,
    pub packet_loss: c_uint,
    pub complexity: c_uint,
    pub cbr: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_codec_silk_setting {
    pub enabled: pj_bool_t,
    pub quality: c_int,
    pub complexity: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_factory_op {
    pub init:
        Option<unsafe extern "C" fn(f: *mut pjmedia_vid_dev_factory) -> pj_status_t>,
    pub destroy:
        Option<unsafe extern "C" fn(f: *mut pjmedia_vid_dev_factory) -> pj_status_t>,
    pub get_dev_count: Option<
        unsafe extern "C" fn(f: *mut pjmedia_vid_dev_factory) -> c_uint,
    >,
    pub get_dev_info: Option<
        unsafe extern "C" fn(
            f: *mut pjmedia_vid_dev_factory,
            index: c_uint,
            info: *mut pjmedia_vid_dev_info,
        ) -> pj_status_t,
    >,
    pub default_param: Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            f: *mut pjmedia_vid_dev_factory,
            index: c_uint,
            param: *mut pjmedia_vid_dev_param,
        ) -> pj_status_t,
    >,
    pub create_stream: Option<
        unsafe extern "C" fn(
            f: *mut pjmedia_vid_dev_factory,
            param: *mut pjmedia_vid_dev_param,
            cb: *const pjmedia_vid_dev_cb,
            user_data: *mut c_void,
            p_vid_strm: *mut *mut pjmedia_vid_dev_stream,
        ) -> pj_status_t,
    >,
    pub refresh:
        Option<unsafe extern "C" fn(f: *mut pjmedia_vid_dev_factory) -> pj_status_t>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_factory {
    pub sys: pjmedia_vid_dev_factory__bindgen_ty_1,
    pub op: *mut pjmedia_vid_dev_factory_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_factory__bindgen_ty_1 {
    pub drv_idx: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_stream_op {
    pub get_param: Option<
        unsafe extern "C" fn(
            strm: *mut pjmedia_vid_dev_stream,
            param: *mut pjmedia_vid_dev_param,
        ) -> pj_status_t,
    >,
    pub get_cap: Option<
        unsafe extern "C" fn(
            strm: *mut pjmedia_vid_dev_stream,
            cap: pjmedia_vid_dev_cap,
            value: *mut c_void,
        ) -> pj_status_t,
    >,
    pub set_cap: Option<
        unsafe extern "C" fn(
            strm: *mut pjmedia_vid_dev_stream,
            cap: pjmedia_vid_dev_cap,
            value: *const c_void,
        ) -> pj_status_t,
    >,
    pub start: Option<
        unsafe extern "C" fn(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t,
    >,
    pub get_frame: Option<
        unsafe extern "C" fn(
            strm: *mut pjmedia_vid_dev_stream,
            frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub put_frame: Option<
        unsafe extern "C" fn(
            strm: *mut pjmedia_vid_dev_stream,
            frame: *const pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub stop: Option<
        unsafe extern "C" fn(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t,
    >,
    pub destroy: Option<
        unsafe extern "C" fn(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_stream {
    pub sys: pjmedia_vid_dev_stream__bindgen_ty_1,
    pub op: *mut pjmedia_vid_dev_stream_op,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_stream__bindgen_ty_1 {
    pub drv_idx: c_uint,
    pub is_running: pj_bool_t,
}

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct pjmedia_avi_dev_param {
//     pub path: pj_str_t,
//     pub title: pj_str_t,
//     pub avi_streams: *mut pjmedia_avi_streams,
// }

#[link(name="pjmedia")]
extern "C" {
    pub fn pjmedia_type_name(t: pjmedia_type) -> *const c_char;
    pub fn pjmedia_get_type(name: *const pj_str_t) -> pjmedia_type;
    pub static pjmedia_linear2ulaw_tab: [pj_uint8_t; 16384usize];
    pub static pjmedia_linear2alaw_tab: [pj_uint8_t; 16384usize];
    pub static pjmedia_ulaw2linear_tab: [pj_int16_t; 256usize];
    pub static pjmedia_alaw2linear_tab: [pj_int16_t; 256usize];

    // AVI File Player
    // pub fn pjmedia_avi_player_create_streams( pool: *mut pj_pool_t, filename: *const c_char, flags: c_uint, p_streams: *mut *mut pjmedia_avi_streams, ) -> pj_status_t;
    // pub fn pjmedia_avi_streams_get_num_streams( streams: *mut pjmedia_avi_streams, ) -> c_uint;
    // pub fn pjmedia_avi_streams_get_stream( streams: *mut pjmedia_avi_streams, idx: c_uint, ) -> *mut pjmedia_avi_stream;
    // pub fn pjmedia_avi_streams_get_stream_by_media( streams: *mut pjmedia_avi_streams, start_idx: c_uint, media_type: pjmedia_type, ) -> *mut pjmedia_avi_stream;
    // pub fn pjmedia_avi_stream_get_len(stream: *mut pjmedia_avi_stream) -> pj_ssize_t;
    // pub fn pjmedia_avi_stream_set_eof_cb( stream: *mut pjmedia_avi_stream, user_data: *mut c_void, cb: Option< unsafe extern "C" fn( stream: *mut pjmedia_avi_stream, usr_data: *mut c_void, ) -> pj_status_t, >, ) -> pj_status_t;
    // pub fn pjmedia_avi_stream_set_eof_cb2( stream: *mut pjmedia_avi_stream, user_data: *mut c_void, cb: Option< unsafe extern "C" fn( stream: *mut pjmedia_avi_stream, usr_data: *mut c_void, ), >, ) -> pj_status_t;

    pub fn pjmedia_wav_player_port_create( pool: *mut pj_pool_t, filename: *const c_char, ptime: c_uint, flags: c_uint, buff_size: pj_ssize_t, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_wav_player_get_info( port: *mut pjmedia_port, i: *mut pjmedia_wav_player_info, ) -> pj_status_t;
    pub fn pjmedia_wav_player_get_len(port: *mut pjmedia_port) -> pj_ssize_t;
    pub fn pjmedia_wav_player_port_set_pos( port: *mut pjmedia_port, offset: pj_uint32_t, ) -> pj_status_t;
    pub fn pjmedia_wav_player_port_get_pos(port: *mut pjmedia_port) -> pj_ssize_t;
    pub fn pjmedia_wav_player_set_eof_cb( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option<unsafe extern "C" fn( port: *mut pjmedia_port, usr_data: *mut c_void, ) -> pj_status_t, >, ) -> pj_status_t;
    pub fn pjmedia_wav_player_set_eof_cb2( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option< unsafe extern "C" fn(port: *mut pjmedia_port, usr_data: *mut c_void), >, ) -> pj_status_t;

    // Bidirectional port
    pub fn pjmedia_bidirectional_port_create( pool: *mut pj_pool_t, get_port: *mut pjmedia_port, put_port: *mut pjmedia_port, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;

    // Conference Bridge
    pub fn pjmedia_conf_create( pool: *mut pj_pool_t, max_slots: c_uint, sampling_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_conf: *mut *mut pjmedia_conf, ) -> pj_status_t;
    pub fn pjmedia_conf_destroy(conf: *mut pjmedia_conf) -> pj_status_t;
    pub fn pjmedia_conf_get_master_port(conf: *mut pjmedia_conf) -> *mut pjmedia_port;
    pub fn pjmedia_conf_set_port0_name( conf: *mut pjmedia_conf, name: *const pj_str_t, ) -> pj_status_t;
    pub fn pjmedia_conf_add_port( conf: *mut pjmedia_conf, pool: *mut pj_pool_t, strm_port: *mut pjmedia_port, name: *const pj_str_t, p_slot: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_configure_port( conf: *mut pjmedia_conf, slot: c_uint, tx: pjmedia_port_op, rx: pjmedia_port_op, ) -> pj_status_t;
    pub fn pjmedia_conf_connect_port( conf: *mut pjmedia_conf, src_slot: c_uint, sink_slot: c_uint, adj_level: c_int, ) -> pj_status_t;
    pub fn pjmedia_conf_disconnect_port( conf: *mut pjmedia_conf, src_slot: c_uint, sink_slot: c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_disconnect_port_from_sources( conf: *mut pjmedia_conf, sink_slot: c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_disconnect_port_from_sinks( conf: *mut pjmedia_conf, src_slot: c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_get_port_count(conf: *mut pjmedia_conf) -> c_uint;
    pub fn pjmedia_conf_get_connect_count(conf: *mut pjmedia_conf) -> c_uint;
    pub fn pjmedia_conf_remove_port( conf: *mut pjmedia_conf, slot: c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_enum_ports( conf: *mut pjmedia_conf, ports: *mut c_uint, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_get_port_info( conf: *mut pjmedia_conf, slot: c_uint, info: *mut pjmedia_conf_port_info, ) -> pj_status_t;
    pub fn pjmedia_conf_get_ports_info( conf: *mut pjmedia_conf, size: *mut c_uint, info: *mut pjmedia_conf_port_info, ) -> pj_status_t;
    pub fn pjmedia_conf_get_signal_level( conf: *mut pjmedia_conf, slot: c_uint, tx_level: *mut c_uint, rx_level: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_conf_adjust_rx_level( conf: *mut pjmedia_conf, slot: c_uint, adj_level: c_int, ) -> pj_status_t;
    pub fn pjmedia_conf_adjust_tx_level( conf: *mut pjmedia_conf, slot: c_uint, adj_level: c_int, ) -> pj_status_t;
    pub fn pjmedia_conf_adjust_conn_level( conf: *mut pjmedia_conf, src_slot: c_uint, sink_slot: c_uint, adj_level: c_int, ) -> pj_status_t;

    // Echo Cancellation
    pub fn pjmedia_echo_port_create( pool: *mut pj_pool_t, dn_port: *mut pjmedia_port, tail_ms: c_uint, latency_ms: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    // pub fn pjmedia_echo_stat_default(stat: *mut pjmedia_echo_stat);
    // pub fn pjmedia_echo_create( pool: *mut pj_pool_t, clock_rate: c_uint, samples_per_frame: c_uint, tail_ms: c_uint, latency_ms: c_uint, options: c_uint, p_echo: *mut *mut pjmedia_echo_state, ) -> pj_status_t;
    // pub fn pjmedia_echo_create2( pool: *mut pj_pool_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, tail_ms: c_uint, latency_ms: c_uint, options: c_uint, p_echo: *mut *mut pjmedia_echo_state, ) -> pj_status_t;
    // pub fn pjmedia_echo_destroy(echo: *mut pjmedia_echo_state) -> pj_status_t;
    // pub fn pjmedia_echo_reset(echo: *mut pjmedia_echo_state) -> pj_status_t;
    // pub fn pjmedia_echo_get_stat( echo: *mut pjmedia_echo_state, p_stat: *mut pjmedia_echo_stat, ) -> pj_status_t;
    // pub fn pjmedia_echo_playback( echo: *mut pjmedia_echo_state, play_frm: *mut pj_int16_t, ) -> pj_status_t;
    // pub fn pjmedia_echo_capture( echo: *mut pjmedia_echo_state, rec_frm: *mut pj_int16_t, options: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_echo_cancel( echo: *mut pjmedia_echo_state, rec_frm: *mut pj_int16_t, play_frm: *const pj_int16_t, options: c_uint, reserved: *mut c_void, ) -> pj_status_t;

    // Memory/Buffer-based Playback Port
    pub fn pjmedia_mem_player_create( pool: *mut pj_pool_t, buffer: *const c_void, size: pj_size_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_mem_player_set_eof_cb( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option< unsafe extern "C" fn( port: *mut pjmedia_port, usr_data: *mut c_void, ) -> pj_status_t, >, ) -> pj_status_t;
    pub fn pjmedia_mem_player_set_eof_cb2( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option< unsafe extern "C" fn(port: *mut pjmedia_port, usr_data: *mut c_void), >, ) -> pj_status_t;

    // Memory/Buffer-based Capture Port
    pub fn pjmedia_mem_capture_create( pool: *mut pj_pool_t, buffer: *mut c_void, size: pj_size_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_mem_capture_set_eof_cb( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option< unsafe extern "C" fn( port: *mut pjmedia_port, usr_data: *mut c_void, ) -> pj_status_t, >, ) -> pj_status_t;
    pub fn pjmedia_mem_capture_set_eof_cb2( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option< unsafe extern "C" fn(port: *mut pjmedia_port, usr_data: *mut c_void), >, ) -> pj_status_t;
    pub fn pjmedia_mem_capture_get_size(port: *mut pjmedia_port) -> pj_size_t;

    // Null Port
    pub fn pjmedia_null_port_create( pool: *mut pj_pool_t, sampling_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;

    // Resample Port
    pub fn pjmedia_resample_port_create( pool: *mut pj_pool_t, dn_port: *mut pjmedia_port, clock_rate: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    // pub fn pjmedia_resample_create( pool: *mut pj_pool_t, high_quality: pj_bool_t, large_filter: pj_bool_t, channel_count: c_uint, rate_in: c_uint, rate_out: c_uint, samples_per_frame: c_uint, p_resample: *mut *mut pjmedia_resample, ) -> pj_status_t;
    // pub fn pjmedia_resample_run( resample: *mut pjmedia_resample, input: *const pj_int16_t, output: *mut pj_int16_t, );
    // pub fn pjmedia_resample_get_input_size( resample: *mut pjmedia_resample, ) -> c_uint;
    // pub fn pjmedia_resample_destroy(resample: *mut pjmedia_resample);

    // Streams
    // pub fn pjmedia_stream_info_parse_fmtp( pool: *mut pj_pool_t, m: *const pjmedia_sdp_media, pt: c_uint, fmtp: *mut pjmedia_codec_fmtp, ) -> pj_status_t;
    pub fn pjmedia_stream_info_from_sdp( si: *mut pjmedia_stream_info, pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, local: *const pjmedia_sdp_session, remote: *const pjmedia_sdp_session, stream_idx: c_uint, ) -> pj_status_t;
    pub fn pjmedia_stream_create( endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, info: *const pjmedia_stream_info, tp: *mut pjmedia_transport, user_data: *mut c_void, p_stream: *mut *mut pjmedia_stream, ) -> pj_status_t;
    pub fn pjmedia_stream_destroy(stream: *mut pjmedia_stream) -> pj_status_t;
    pub fn pjmedia_stream_get_last_jb_frame_type( stream: *mut pjmedia_stream, ) -> c_char;
    pub fn pjmedia_stream_get_port( stream: *mut pjmedia_stream, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_stream_get_transport(st: *mut pjmedia_stream) -> *mut pjmedia_transport;
    pub fn pjmedia_stream_start(stream: *mut pjmedia_stream) -> pj_status_t;
    pub fn pjmedia_stream_get_info( stream: *const pjmedia_stream, info: *mut pjmedia_stream_info, ) -> pj_status_t;
    pub fn pjmedia_stream_get_stat( stream: *const pjmedia_stream, stat: *mut pjmedia_rtcp_stat, ) -> pj_status_t;
    pub fn pjmedia_stream_reset_stat(stream: *mut pjmedia_stream) -> pj_status_t;
    pub fn pjmedia_stream_get_stat_jbuf( stream: *const pjmedia_stream, state: *mut pjmedia_jb_state, ) -> pj_status_t;
    pub fn pjmedia_stream_pause(stream: *mut pjmedia_stream, dir: pjmedia_dir) -> pj_status_t;
    pub fn pjmedia_stream_resume(stream: *mut pjmedia_stream, dir: pjmedia_dir) -> pj_status_t;
    pub fn pjmedia_stream_dial_dtmf( stream: *mut pjmedia_stream, ascii_digit: *const pj_str_t, ) -> pj_status_t;
    pub fn pjmedia_stream_check_dtmf(stream: *mut pjmedia_stream) -> pj_bool_t;
    pub fn pjmedia_stream_get_dtmf( stream: *mut pjmedia_stream, ascii_digits: *mut c_char, size: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_stream_set_dtmf_callback( stream: *mut pjmedia_stream, cb: Option< unsafe extern "C" fn( arg1: *mut pjmedia_stream, user_data: *mut c_void, digit: c_int, ), >, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_stream_set_dtmf_event_callback( stream: *mut pjmedia_stream, cb: Option< unsafe extern "C" fn( arg1: *mut pjmedia_stream, user_data: *mut c_void, event: *const pjmedia_stream_dtmf_event, ), >, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_stream_send_rtcp_sdes(stream: *mut pjmedia_stream) -> pj_status_t;
    pub fn pjmedia_stream_send_rtcp_bye(stream: *mut pjmedia_stream) -> pj_status_t;
    pub fn pjmedia_stream_get_rtp_session_info( stream: *mut pjmedia_stream, session_info: *mut pjmedia_stream_rtp_sess_info, ) -> pj_status_t;

    // Multi-frequency tone generator
    pub fn pjmedia_tonegen_create( pool: *mut pj_pool_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_tonegen_create2( pool: *mut pj_pool_t, name: *const pj_str_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_tonegen_is_busy(tonegen: *mut pjmedia_port) -> pj_bool_t;
    pub fn pjmedia_tonegen_stop(tonegen: *mut pjmedia_port) -> pj_status_t;
    pub fn pjmedia_tonegen_stop_loop(tonegen: *mut pjmedia_port) -> pj_status_t;
    pub fn pjmedia_tonegen_rewind(tonegen: *mut pjmedia_port) -> pj_status_t;
    pub fn pjmedia_tonegen_play( tonegen: *mut pjmedia_port, count: c_uint, tones: *const pjmedia_tone_desc, options: c_uint, ) -> pj_status_t;
    pub fn pjmedia_tonegen_play_digits( tonegen: *mut pjmedia_port, count: c_uint, digits: *const pjmedia_tone_digit, options: c_uint, ) -> pj_status_t;
    pub fn pjmedia_tonegen_get_digit_map( tonegen: *mut pjmedia_port, m: *mut *const pjmedia_tone_digit_map, ) -> pj_status_t;
    pub fn pjmedia_tonegen_set_digit_map( tonegen: *mut pjmedia_port, m: *mut pjmedia_tone_digit_map, ) -> pj_status_t;

    // Video Stream
    // pub fn pjmedia_vid_stream_info_from_sdp( si: *mut pjmedia_vid_stream_info, pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, local: *const pjmedia_sdp_session, remote: *const pjmedia_sdp_session, stream_idx: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_rc_config_default(cfg: *mut pjmedia_vid_stream_rc_config);
    // pub fn pjmedia_vid_stream_sk_config_default(cfg: *mut pjmedia_vid_stream_sk_config);
    // pub fn pjmedia_vid_stream_create( endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, info: *mut pjmedia_vid_stream_info, tp: *mut pjmedia_transport, user_data: *mut c_void, p_stream: *mut *mut pjmedia_vid_stream, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_destroy(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_get_port( stream: *mut pjmedia_vid_stream, dir: pjmedia_dir, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_get_transport(st: *mut pjmedia_vid_stream) -> *mut pjmedia_transport;
    // pub fn pjmedia_vid_stream_get_stat( stream: *const pjmedia_vid_stream, stat: *mut pjmedia_rtcp_stat, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_reset_stat(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_get_stat_jbuf( stream: *const pjmedia_vid_stream, state: *mut pjmedia_jb_state, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_get_info( stream: *const pjmedia_vid_stream, info: *mut pjmedia_vid_stream_info, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_start(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_is_running( stream: *mut pjmedia_vid_stream, dir: pjmedia_dir, ) -> pj_bool_t;
    // pub fn pjmedia_vid_stream_pause( stream: *mut pjmedia_vid_stream, dir: pjmedia_dir, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_resume( stream: *mut pjmedia_vid_stream, dir: pjmedia_dir, ) -> pj_status_t;
    // pub fn pjmedia_vid_stream_send_keyframe(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_send_rtcp_sdes(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_send_rtcp_bye(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_send_rtcp_pli(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    // pub fn pjmedia_vid_stream_get_rtp_session_info( stream: *mut pjmedia_vid_stream, session_info: *mut pjmedia_stream_rtp_sess_info, ) -> pj_status_t;

    // WAV File Play List
    pub fn pjmedia_wav_playlist_create( pool: *mut pj_pool_t, port_label: *const pj_str_t, file_list: *const pj_str_t, file_count: c_int, ptime: c_uint, options: c_uint, buff_size: pj_ssize_t, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_wav_playlist_set_eof_cb( port: *mut pjmedia_port, user_data: *mut c_void, cb: Option< unsafe extern "C" fn( port: *mut pjmedia_port, usr_data: *mut c_void, ) -> pj_status_t, >, ) -> pj_status_t;
    pub fn pjmedia_wav_playlist_set_eof_cb2(port: *mut pjmedia_port,user_data: *mut c_void,cb: Option< unsafe extern "C" fn(port: *mut pjmedia_port, usr_data: *mut c_void), >, ) -> pj_status_t;

    // File Writer
    pub fn pjmedia_wav_writer_port_create( pool: *mut pj_pool_t, filename: *const c_char, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, flags: c_uint, buff_size: pj_ssize_t, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_wav_writer_port_get_pos(port: *mut pjmedia_port) -> pj_ssize_t;
    pub fn pjmedia_wav_writer_port_set_cb( port: *mut pjmedia_port, pos: pj_size_t, user_data: *mut c_void, cb: Option< unsafe extern "C" fn( port: *mut pjmedia_port, usr_data: *mut c_void, ) -> pj_status_t, >, ) -> pj_status_t;
    pub fn pjmedia_wav_writer_port_set_cb2( port: *mut pjmedia_port, pos: pj_size_t, user_data: *mut c_void, cb: Option< unsafe extern "C" fn(port: *mut pjmedia_port, usr_data: *mut c_void), >, ) -> pj_status_t;

    // Media Channel spliter/combiner
    pub fn pjmedia_splitcomb_create( pool: *mut pj_pool_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_splitcomb: *mut *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_splitcomb_set_channel( splitcomb: *mut pjmedia_port, ch_num: c_uint, options: c_uint, port: *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_splitcomb_create_rev_channel( pool: *mut pj_pool_t, splitcomb: *mut pjmedia_port, ch_num: c_uint, options: c_uint, p_chport: *mut *mut pjmedia_port, ) -> pj_status_t;

    // Video Confrence Bridge
    // pub fn pjmedia_vid_conf_setting_default(opt: *mut pjmedia_vid_conf_setting);
    // pub fn pjmedia_vid_conf_create( pool: *mut pj_pool_t, opt: *const pjmedia_vid_conf_setting, p_vid_conf: *mut *mut pjmedia_vid_conf, ) -> pj_status_t;
    // pub fn pjmedia_vid_conf_destroy(vid_conf: *mut pjmedia_vid_conf) -> pj_status_t;
    // pub fn pjmedia_vid_conf_add_port( vid_conf: *mut pjmedia_vid_conf, pool: *mut pj_pool_t, port: *mut pjmedia_port, name: *const pj_str_t, opt: *mut c_void, p_slot: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_conf_remove_port( vid_conf: *mut pjmedia_vid_conf, slot: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_conf_get_port_count( vid_conf: *mut pjmedia_vid_conf, ) -> c_uint;
    // pub fn pjmedia_vid_conf_enum_ports( vid_conf: *mut pjmedia_vid_conf, slots: *mut c_uint, count: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_conf_get_port_info( vid_conf: *mut pjmedia_vid_conf, slot: c_uint, info: *mut pjmedia_vid_conf_port_info, ) -> pj_status_t;
    // pub fn pjmedia_vid_conf_connect_port( vid_conf: *mut pjmedia_vid_conf, src_slot: c_uint, sink_slot: c_uint, opt: *mut c_void, ) -> pj_status_t;
    // pub fn pjmedia_vid_conf_disconnect_port( vid_conf: *mut pjmedia_vid_conf, src_slot: c_uint, sink_slot: c_uint, ) -> pj_status_t;

    // Video Source duplicator
    // unimported

    // Media Ports Framework
    pub fn pjmedia_port_info_init( info: *mut pjmedia_port_info, name: *const pj_str_t, signature: c_uint, clock_rate: c_uint, channel_count: c_uint, bits_per_sample: c_uint, samples_per_frame: c_uint, ) -> pj_status_t;
    pub fn pjmedia_port_info_init2( info: *mut pjmedia_port_info, name: *const pj_str_t, signature: c_uint, dir: pjmedia_dir, fmt: *const pjmedia_format, ) -> pj_status_t;
    pub fn pjmedia_port_get_clock_src( port: *mut pjmedia_port, dir: pjmedia_dir, ) -> *mut pjmedia_clock_src;
    pub fn pjmedia_port_get_frame( port: *mut pjmedia_port, frame: *mut pjmedia_frame, ) -> pj_status_t;
    pub fn pjmedia_port_put_frame( port: *mut pjmedia_port, frame: *mut pjmedia_frame, ) -> pj_status_t;
    pub fn pjmedia_port_destroy(port: *mut pjmedia_port) -> pj_status_t;

    // Clock/Timing
    // Master Port
    pub fn pjmedia_master_port_create( pool: *mut pj_pool_t, u_port: *mut pjmedia_port, d_port: *mut pjmedia_port, options: c_uint, p_m: *mut *mut pjmedia_master_port, ) -> pj_status_t;
    pub fn pjmedia_master_port_start(m: *mut pjmedia_master_port) -> pj_status_t;
    pub fn pjmedia_master_port_stop(m: *mut pjmedia_master_port) -> pj_status_t;
    pub fn pjmedia_master_port_wait( m: *mut pjmedia_master_port, wait: pj_bool_t, ts: *mut pj_timestamp, ) -> pj_bool_t;
    pub fn pjmedia_master_port_set_uport( m: *mut pjmedia_master_port, port: *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_master_port_get_uport(m: *mut pjmedia_master_port) -> *mut pjmedia_port;
    pub fn pjmedia_master_port_set_dport( m: *mut pjmedia_master_port, port: *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_master_port_get_dport(m: *mut pjmedia_master_port) -> *mut pjmedia_port;
    pub fn pjmedia_master_port_destroy( m: *mut pjmedia_master_port, destroy_ports: pj_bool_t, ) -> pj_status_t;

    // Sound Device Port
    pub fn pjmedia_snd_port_param_default(prm: *mut pjmedia_snd_port_param);
    pub fn pjmedia_snd_port_create( pool: *mut pj_pool_t, rec_id: c_int, play_id: c_int, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_snd_port, ) -> pj_status_t;
    pub fn pjmedia_snd_port_create_rec( pool: *mut pj_pool_t, index: c_int, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_snd_port, ) -> pj_status_t;
    pub fn pjmedia_snd_port_create_player( pool: *mut pj_pool_t, index: c_int, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, options: c_uint, p_port: *mut *mut pjmedia_snd_port, ) -> pj_status_t;
    pub fn pjmedia_snd_port_create2( pool: *mut pj_pool_t, prm: *const pjmedia_snd_port_param, p_port: *mut *mut pjmedia_snd_port, ) -> pj_status_t;
    pub fn pjmedia_snd_port_destroy(snd_port: *mut pjmedia_snd_port) -> pj_status_t;
    pub fn pjmedia_snd_port_get_snd_stream( snd_port: *mut pjmedia_snd_port, ) -> *mut pjmedia_aud_stream;
    pub fn pjmedia_snd_port_set_ec( snd_port: *mut pjmedia_snd_port, pool: *mut pj_pool_t, tail_ms: c_uint, options: c_uint, ) -> pj_status_t;
    pub fn pjmedia_snd_port_get_ec_tail( snd_port: *mut pjmedia_snd_port, p_length: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_snd_port_get_ec_stat( snd_port: *mut pjmedia_snd_port, p_stat: *mut pjmedia_echo_stat, ) -> pj_status_t;
    pub fn pjmedia_snd_port_get_clock_src( snd_port: *mut pjmedia_snd_port, dir: pjmedia_dir, ) -> *mut pjmedia_clock_src;
    pub fn pjmedia_snd_port_connect( snd_port: *mut pjmedia_snd_port, port: *mut pjmedia_port, ) -> pj_status_t;
    pub fn pjmedia_snd_port_get_port(snd_port: *mut pjmedia_snd_port) -> *mut pjmedia_port;
    pub fn pjmedia_snd_port_disconnect(snd_port: *mut pjmedia_snd_port) -> pj_status_t;

    // Video Media Port
    // pub fn pjmedia_vid_port_param_default(prm: *mut pjmedia_vid_port_param);
    // pub fn pjmedia_vid_port_create( pool: *mut pj_pool_t, prm: *const pjmedia_vid_port_param, p_vp: *mut *mut pjmedia_vid_port, ) -> pj_status_t;
    // pub fn pjmedia_vid_port_set_cb( vid_port: *mut pjmedia_vid_port, cb: *const pjmedia_vid_dev_cb, user_data: *mut c_void, );
    // pub fn pjmedia_vid_port_get_stream( vid_port: *mut pjmedia_vid_port, ) -> *mut pjmedia_vid_dev_stream;
    // pub fn pjmedia_vid_port_get_passive_port(vid_port: *mut pjmedia_vid_port) -> *mut pjmedia_port;
    // pub fn pjmedia_vid_port_get_clock_src( vid_port: *mut pjmedia_vid_port, ) -> *mut pjmedia_clock_src;
    // pub fn pjmedia_vid_port_set_clock_src( vid_port: *mut pjmedia_vid_port, clocksrc: *mut pjmedia_clock_src, ) -> pj_status_t;
    // pub fn pjmedia_vid_port_subscribe_event( vid_port: *mut pjmedia_vid_port, port: *mut pjmedia_port, ) -> pj_status_t;
    // pub fn pjmedia_vid_port_connect( vid_port: *mut pjmedia_vid_port, port: *mut pjmedia_port, destroy: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_vid_port_disconnect(vid_port: *mut pjmedia_vid_port) -> pj_status_t;
    // pub fn pjmedia_vid_port_get_connected_port( vid_port: *mut pjmedia_vid_port, ) -> *mut pjmedia_port;
    // pub fn pjmedia_vid_port_start(vid_port: *mut pjmedia_vid_port) -> pj_status_t;
    // pub fn pjmedia_vid_port_is_running(vid_port: *mut pjmedia_vid_port) -> pj_bool_t;
    // pub fn pjmedia_vid_port_stop(vid_port: *mut pjmedia_vid_port) -> pj_status_t;
    // pub fn pjmedia_vid_port_destroy(vid_port: *mut pjmedia_vid_port);

    // Clock Generator
    pub fn pjmedia_clock_src_init( clocksrc: *mut pjmedia_clock_src, media_type: pjmedia_type, clock_rate: c_uint, ptime_usec: c_uint, ) -> pj_status_t;
    pub fn pjmedia_clock_src_update( clocksrc: *mut pjmedia_clock_src, timestamp: *const pj_timestamp, ) -> pj_status_t;
    pub fn pjmedia_clock_src_get_current_timestamp( clocksrc: *const pjmedia_clock_src, timestamp: *mut pj_timestamp, ) -> pj_status_t;
    pub fn pjmedia_clock_src_get_time_msec(clocksrc: *const pjmedia_clock_src) -> pj_uint32_t;
    pub fn pjmedia_clock_create( pool: *mut pj_pool_t, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, options: c_uint, cb: pjmedia_clock_callback, user_data: *mut c_void, p_clock: *mut *mut pjmedia_clock, ) -> pj_status_t;
    pub fn pjmedia_clock_create2( pool: *mut pj_pool_t, param: *const pjmedia_clock_param, options: c_uint, cb: pjmedia_clock_callback, user_data: *mut c_void, p_clock: *mut *mut pjmedia_clock, ) -> pj_status_t;
    pub fn pjmedia_clock_start(clock: *mut pjmedia_clock) -> pj_status_t;
    pub fn pjmedia_clock_stop(clock: *mut pjmedia_clock) -> pj_status_t;
    pub fn pjmedia_clock_modify( clock: *mut pjmedia_clock, param: *const pjmedia_clock_param, ) -> pj_status_t;
    pub fn pjmedia_clock_wait( clock: *mut pjmedia_clock, wait: pj_bool_t, ts: *mut pj_timestamp, ) -> pj_bool_t;
    pub fn pjmedia_clock_destroy(clock: *mut pjmedia_clock) -> pj_status_t;


    // Media Transport

    // Sample Transport Adapter
    pub fn pjmedia_tp_adapter_create( endpt: *mut pjmedia_endpt, name: *const c_char, base_tp: *mut pjmedia_transport, del_base: pj_bool_t, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;

    // ICE Media Transport
    pub fn pjmedia_ice_create( endpt: *mut pjmedia_endpt, name: *const c_char, comp_cnt: c_uint, cfg: *const pj_ice_strans_cfg, cb: *const pjmedia_ice_cb, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_ice_create2( endpt: *mut pjmedia_endpt, name: *const c_char, comp_cnt: c_uint, cfg: *const pj_ice_strans_cfg, cb: *const pjmedia_ice_cb, options: c_uint, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_ice_create3( endpt: *mut pjmedia_endpt, name: *const c_char, comp_cnt: c_uint, cfg: *const pj_ice_strans_cfg, cb: *const pjmedia_ice_cb, options: c_uint, user_data: *mut c_void, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_ice_get_grp_lock(tp: *mut pjmedia_transport) -> *mut pj_grp_lock_t;
    pub fn pjmedia_ice_add_ice_cb( tp: *mut pjmedia_transport, cb: *const pjmedia_ice_cb, user_data: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_ice_remove_ice_cb( tp: *mut pjmedia_transport, cb: *const pjmedia_ice_cb, user_data: *mut c_void, ) -> pj_status_t;

    // pub fn pjmedia_ice_sdp_has_trickle( sdp: *const pjmedia_sdp_session, med_idx: c_uint, ) -> pj_bool_t;
    // pub fn pjmedia_ice_trickle_update( tp: *mut pjmedia_transport, rem_ufrag: *const pj_str_t, rem_passwd: *const pj_str_t, rcand_cnt: c_uint, rcand: *const pj_ice_sess_cand, rcand_end: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_ice_trickle_decode_sdp( sdp: *const pjmedia_sdp_session, media_index: c_uint, mid: *mut pj_str_t, ufrag: *mut pj_str_t, passwd: *mut pj_str_t, cand_cnt: *mut c_uint, cand: *mut pj_ice_sess_cand, end_of_cand: *mut pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_ice_trickle_encode_sdp( sdp_pool: *mut pj_pool_t, sdp: *mut pjmedia_sdp_session, mid: *const pj_str_t, ufrag: *const pj_str_t, passwd: *const pj_str_t, cand_cnt: c_uint, cand: *const pj_ice_sess_cand, end_of_cand: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_ice_trickle_has_new_cand(tp: *mut pjmedia_transport) -> pj_bool_t;
    // pub fn pjmedia_ice_trickle_send_local_cand( tp: *mut pjmedia_transport, sdp_pool: *mut pj_pool_t, sdp: *mut pjmedia_sdp_session, p_end_of_cand: *mut pj_bool_t, ) -> pj_status_t;

    // Loopback Media Transport
    pub fn pjmedia_loop_tp_setting_default(opt: *mut pjmedia_loop_tp_setting);
    pub fn pjmedia_transport_loop_create( endpt: *mut pjmedia_endpt, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_transport_loop_create2( endpt: *mut pjmedia_endpt, opt: *const pjmedia_loop_tp_setting, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_transport_loop_disable_rx( tp: *mut pjmedia_transport, user: *mut c_void, disabled: pj_bool_t, ) -> pj_status_t;

    // Secure RTP (SRTP) Media Transport
    pub fn pjmedia_srtp_init_lib(endpt: *mut pjmedia_endpt) -> pj_status_t;
    pub fn pjmedia_srtp_setting_default(opt: *mut pjmedia_srtp_setting);
    pub fn pjmedia_srtp_enum_crypto( count: *mut c_uint, crypto: *mut pjmedia_srtp_crypto, ) -> pj_status_t;
    pub fn pjmedia_srtp_enum_keying( count: *mut c_uint, keying: *mut pjmedia_srtp_keying_method, ) -> pj_status_t;
    pub fn pjmedia_transport_srtp_create( endpt: *mut pjmedia_endpt, tp: *mut pjmedia_transport, opt: *const pjmedia_srtp_setting, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_transport_srtp_dtls_get_fingerprint( srtp: *mut pjmedia_transport, hash: *const c_char, buf: *mut c_char, len: *mut pj_size_t, ) -> pj_status_t;
    pub fn pjmedia_transport_srtp_dtls_start_nego( srtp: *mut pjmedia_transport, param: *const pjmedia_srtp_dtls_nego_param, ) -> pj_status_t;
    pub fn pjmedia_transport_srtp_start( srtp: *mut pjmedia_transport, tx: *const pjmedia_srtp_crypto, rx: *const pjmedia_srtp_crypto, ) -> pj_status_t;
    pub fn pjmedia_transport_srtp_stop(srtp: *mut pjmedia_transport) -> pj_status_t;
    pub fn pjmedia_transport_srtp_decrypt_pkt( tp: *mut pjmedia_transport, is_rtp: pj_bool_t, pkt: *mut c_void, pkt_len: *mut c_int, ) -> pj_status_t;
    pub fn pjmedia_transport_srtp_get_member( srtp: *mut pjmedia_transport, ) -> *mut pjmedia_transport;

    // UDP Media Transport
    pub fn pjmedia_transport_udp_create( endpt: *mut pjmedia_endpt, name: *const c_char, port: c_int, options: c_uint, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_transport_udp_create2( endpt: *mut pjmedia_endpt, name: *const c_char, addr: *const pj_str_t, port: c_int, options: c_uint, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_transport_udp_create3( endpt: *mut pjmedia_endpt, af: c_int, name: *const c_char, addr: *const pj_str_t, port: c_int, options: c_uint, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    pub fn pjmedia_transport_udp_attach( endpt: *mut pjmedia_endpt, name: *const c_char, si: *const pjmedia_sock_info, options: c_uint, p_tp: *mut *mut pjmedia_transport, ) -> pj_status_t;
    

    // Audio device module
    pub fn pjmedia_aud_driver_init( drv_idx: c_uint, refresh: pj_bool_t, ) -> pj_status_t;
    pub fn pjmedia_aud_driver_deinit(drv_idx: c_uint);
    pub fn pjmedia_aud_dev_cap_name( cap: pjmedia_aud_dev_cap, p_desc: *mut *const c_char, ) -> *const c_char;
    pub fn pjmedia_aud_param_set_cap( param: *mut pjmedia_aud_param, cap: pjmedia_aud_dev_cap, pval: *const c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_param_get_cap( param: *const pjmedia_aud_param, cap: pjmedia_aud_dev_cap, pval: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_dev_refresh() -> pj_status_t;
    pub fn pjmedia_aud_dev_count() -> c_uint;
    pub fn pjmedia_aud_dev_get_info( id: pjmedia_aud_dev_index, info: *mut pjmedia_aud_dev_info, ) -> pj_status_t;
    pub fn pjmedia_aud_dev_lookup( drv_name: *const c_char, dev_name: *const c_char, id: *mut pjmedia_aud_dev_index, ) -> pj_status_t;
    pub fn pjmedia_aud_dev_default_param( id: pjmedia_aud_dev_index, param: *mut pjmedia_aud_param, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_create( param: *const pjmedia_aud_param, rec_cb: pjmedia_aud_rec_cb, play_cb: pjmedia_aud_play_cb, user_data: *mut c_void, p_strm: *mut *mut pjmedia_aud_stream, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_get_param( strm: *mut pjmedia_aud_stream, param: *mut pjmedia_aud_param, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_get_cap(strm: *mut pjmedia_aud_stream,cap: pjmedia_aud_dev_cap,value: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_set_cap( strm: *mut pjmedia_aud_stream, cap: pjmedia_aud_dev_cap, value: *const c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_start(strm: *mut pjmedia_aud_stream) -> pj_status_t;
    pub fn pjmedia_aud_stream_stop(strm: *mut pjmedia_aud_stream) -> pj_status_t;
    pub fn pjmedia_aud_stream_destroy(strm: *mut pjmedia_aud_stream) -> pj_status_t;
    pub fn pjmedia_audiodev_strerror( status: pj_status_t, buffer: *mut c_char, bufsize: pj_size_t, ) -> pj_str_t;
    pub fn pjmedia_get_aud_subsys() -> *mut pjmedia_aud_subsys;


    // pub fn pjmedia_format_init_video( fmt: *mut pjmedia_format, fmt_id: pj_uint32_t, width: c_uint, height: c_uint, fps_num: c_uint, fps_denum: c_uint, );
    // pub fn pjmedia_format_copy( dst: *mut pjmedia_format, src: *const pjmedia_format, ) -> *mut pjmedia_format;
    // pub fn pjmedia_format_get_audio_format_detail( fmt: *const pjmedia_format, assert_valid: pj_bool_t, ) -> *mut pjmedia_audio_format_detail;
    // pub fn pjmedia_format_get_video_format_detail( fmt: *const pjmedia_format, assert_valid: pj_bool_t, ) -> *mut pjmedia_video_format_detail;

    // pub fn pjmedia_video_format_mgr_create( pool: *mut pj_pool_t, max_fmt: c_uint, options: c_uint, p_mgr: *mut *mut pjmedia_video_format_mgr,) -> pj_status_t;
    // pub fn pjmedia_video_format_mgr_instance() -> *mut pjmedia_video_format_mgr;
    // pub fn pjmedia_video_format_mgr_set_instance(mgr: *mut pjmedia_video_format_mgr);
    // pub fn pjmedia_get_video_format_info( mgr: *mut pjmedia_video_format_mgr, id: pj_uint32_t, ) -> *const pjmedia_video_format_info;
    // pub fn pjmedia_register_video_format_info( mgr: *mut pjmedia_video_format_mgr, vfi: *mut pjmedia_video_format_info, ) -> pj_status_t;
    // pub fn pjmedia_video_format_mgr_destroy(mgr: *mut pjmedia_video_format_mgr);

    // pub fn pjmedia_rtp_session_init( ses: *mut pjmedia_rtp_session, default_pt: c_int, sender_ssrc: pj_uint32_t, ) -> pj_status_t;
    // pub fn pjmedia_rtp_session_init2( ses: *mut pjmedia_rtp_session, settings: pjmedia_rtp_session_setting, ) -> pj_status_t;
    // pub fn pjmedia_rtp_encode_rtp( ses: *mut pjmedia_rtp_session, pt: c_int, m: c_int, payload_len: c_int, ts_len: c_int, rtphdr: *mut *const c_void, hdrlen: *mut c_int, ) -> pj_status_t;
    // pub fn pjmedia_rtp_decode_rtp( ses: *mut pjmedia_rtp_session, pkt: *const c_void, pkt_len: c_int, hdr: *mut *const pjmedia_rtp_hdr, payload: *mut *const c_void, payloadlen: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_rtp_decode_rtp2( ses: *mut pjmedia_rtp_session, pkt: *const c_void, pkt_len: c_int, hdr: *mut *const pjmedia_rtp_hdr, dec_hdr: *mut pjmedia_rtp_dec_hdr, payload: *mut *const c_void, payloadlen: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_rtp_session_update( ses: *mut pjmedia_rtp_session, hdr: *const pjmedia_rtp_hdr, seq_st: *mut pjmedia_rtp_status, );
    // pub fn pjmedia_rtp_session_update2( ses: *mut pjmedia_rtp_session, hdr: *const pjmedia_rtp_hdr, seq_st: *mut pjmedia_rtp_status, check_pt: pj_bool_t, );
    // pub fn pjmedia_rtp_seq_init(seq_ctrl: *mut pjmedia_rtp_seq_session, seq: pj_uint16_t);
    // pub fn pjmedia_rtp_seq_update( seq_ctrl: *mut pjmedia_rtp_seq_session, seq: pj_uint16_t, seq_status: *mut pjmedia_rtp_status, );

    // pub fn pjmedia_rtcp_session_setting_default(settings: *mut pjmedia_rtcp_session_setting);
    // pub fn pjmedia_rtcp_init_stat(stat: *mut pjmedia_rtcp_stat);
    // pub fn pjmedia_rtcp_init( session: *mut pjmedia_rtcp_session, name: *mut c_char, clock_rate: c_uint, samples_per_frame: c_uint, ssrc: pj_uint32_t, );
    // pub fn pjmedia_rtcp_init2( session: *mut pjmedia_rtcp_session, settings: *const pjmedia_rtcp_session_setting, );
    // pub fn pjmedia_rtcp_get_ntp_time( sess: *const pjmedia_rtcp_session, ntp: *mut pjmedia_rtcp_ntp_rec, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fini(session: *mut pjmedia_rtcp_session);
    // pub fn pjmedia_rtcp_rx_rtp( session: *mut pjmedia_rtcp_session, seq: c_uint, ts: c_uint, payload: c_uint, );
    // pub fn pjmedia_rtcp_rx_rtp2( session: *mut pjmedia_rtcp_session, seq: c_uint, ts: c_uint, payload: c_uint, discarded: pj_bool_t, );
    // pub fn pjmedia_rtcp_tx_rtp(session: *mut pjmedia_rtcp_session, ptsize: c_uint);
    // pub fn pjmedia_rtcp_rx_rtcp( session: *mut pjmedia_rtcp_session, rtcp_pkt: *const c_void, size: pj_size_t, );
    // pub fn pjmedia_rtcp_build_rtcp( session: *mut pjmedia_rtcp_session, rtcp_pkt: *mut *mut c_void, len: *mut c_int, );
    // pub fn pjmedia_rtcp_build_rtcp_sdes( session: *mut pjmedia_rtcp_session, buf: *mut c_void, length: *mut pj_size_t, sdes: *const pjmedia_rtcp_sdes, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_build_rtcp_bye( session: *mut pjmedia_rtcp_session, buf: *mut c_void, length: *mut pj_size_t, reason: *const pj_str_t, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_enable_xr( session: *mut pjmedia_rtcp_session, enable: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_build_rtcp_xr( session: *mut pjmedia_rtcp_xr_session, rpt_types: c_uint, rtcp_pkt: *mut *mut c_void, len: *mut c_int, );
    // pub fn pjmedia_rtcp_xr_update_info( session: *mut pjmedia_rtcp_xr_session, info: c_uint, val: pj_int32_t, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_xr_init( session: *mut pjmedia_rtcp_xr_session, r_session: *mut pjmedia_rtcp_session, gmin: pj_uint8_t, frames_per_packet: c_uint, );
    // pub fn pjmedia_rtcp_xr_fini(session: *mut pjmedia_rtcp_xr_session);
    // pub fn pjmedia_rtcp_xr_rx_rtcp_xr( session: *mut pjmedia_rtcp_xr_session, rtcp_pkt: *const c_void, size: pj_size_t, );
    // pub fn pjmedia_rtcp_xr_rx_rtp( session: *mut pjmedia_rtcp_xr_session, seq: c_uint, lost: c_int, dup: c_int, discarded: c_int, jitter: c_int, toh: c_int, toh_ipv4: pj_bool_t, );
    // pub fn pjmedia_rtcp_xr_tx_rtp( session: *mut pjmedia_rtcp_xr_session, ptsize: c_uint, );

    // SDP Parsing and Data Structure
    // pub fn pjmedia_sdp_attr_create( pool: *mut pj_pool_t, name: *const c_char, value: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_attr_clone( pool: *mut pj_pool_t, attr: *const pjmedia_sdp_attr, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_attr_find( count: c_uint, attr_array: *const *mut pjmedia_sdp_attr, name: *const pj_str_t, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_attr_find2( count: c_uint, attr_array: *const *mut pjmedia_sdp_attr, name: *const c_char, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_attr_add( count: *mut c_uint, attr_array: *mut *mut pjmedia_sdp_attr, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_remove_all( count: *mut c_uint, attr_array: *mut *mut pjmedia_sdp_attr, name: *const c_char, ) -> c_uint;
    // pub fn pjmedia_sdp_attr_remove( count: *mut c_uint, attr_array: *mut *mut pjmedia_sdp_attr, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_to_rtpmap( pool: *mut pj_pool_t, attr: *const pjmedia_sdp_attr, p_rtpmap: *mut *mut pjmedia_sdp_rtpmap, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_get_rtpmap( attr: *const pjmedia_sdp_attr, rtpmap: *mut pjmedia_sdp_rtpmap, ) -> pj_status_t;
    // pub fn pjmedia_sdp_rtpmap_to_attr( pool: *mut pj_pool_t, rtpmap: *const pjmedia_sdp_rtpmap, p_attr: *mut *mut pjmedia_sdp_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_get_fmtp( attr: *const pjmedia_sdp_attr, fmtp: *mut pjmedia_sdp_fmtp, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_get_rtcp( attr: *const pjmedia_sdp_attr, rtcp: *mut pjmedia_sdp_rtcp_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_create_rtcp( pool: *mut pj_pool_t, a: *const pj_sockaddr, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_attr_get_ssrc( attr: *const pjmedia_sdp_attr, rtcp: *mut pjmedia_sdp_ssrc_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_attr_create_ssrc( pool: *mut pj_pool_t, ssrc: pj_uint32_t, cname: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_conn_clone( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_conn, ) -> *mut pjmedia_sdp_conn;
    // pub fn pjmedia_sdp_conn_cmp( conn1: *const pjmedia_sdp_conn, conn2: *const pjmedia_sdp_conn, option: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_sdp_bandw_clone( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_bandw, ) -> *mut pjmedia_sdp_bandw;
    // pub fn pjmedia_sdp_media_clone( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_media, ) -> *mut pjmedia_sdp_media;
    // pub fn pjmedia_sdp_media_find_attr( m: *const pjmedia_sdp_media, name: *const pj_str_t, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_media_find_attr2( m: *const pjmedia_sdp_media, name: *const c_char, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    // pub fn pjmedia_sdp_media_add_attr( m: *mut pjmedia_sdp_media, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_media_remove_all_attr( m: *mut pjmedia_sdp_media, name: *const c_char, ) -> c_uint;
    // pub fn pjmedia_sdp_media_remove_attr( m: *mut pjmedia_sdp_media, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    // pub fn pjmedia_sdp_media_cmp( sd1: *const pjmedia_sdp_media, sd2: *const pjmedia_sdp_media, option: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_sdp_transport_cmp(t1: *const pj_str_t, t2: *const pj_str_t) -> pj_status_t;
    // pub fn pjmedia_sdp_transport_get_proto(tp: *const pj_str_t) -> pj_uint32_t;
    // pub fn pjmedia_sdp_media_deactivate( pool: *mut pj_pool_t, m: *mut pjmedia_sdp_media, ) -> pj_status_t;
    // pub fn pjmedia_sdp_media_clone_deactivate( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_media, ) -> *mut pjmedia_sdp_media;
    // pub fn pjmedia_sdp_parse( pool: *mut pj_pool_t, buf: *mut c_char, len: pj_size_t, p_sdp: *mut *mut pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_print( sdp: *const pjmedia_sdp_session, buf: *mut c_char, size: pj_size_t, ) -> c_int;
    // pub fn pjmedia_sdp_validate(sdp: *const pjmedia_sdp_session) -> pj_status_t;
    // pub fn pjmedia_sdp_validate2(sdp: *const pjmedia_sdp_session, strict: pj_bool_t) -> pj_status_t;
    // pub fn pjmedia_sdp_session_clone( pool: *mut pj_pool_t, sdp: *const pjmedia_sdp_session, ) -> *mut pjmedia_sdp_session;
    // pub fn pjmedia_sdp_session_cmp( sd1: *const pjmedia_sdp_session, sd2: *const pjmedia_sdp_session, option: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_sdp_session_add_attr( s: *mut pjmedia_sdp_session, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;

    // SDP Negotiation State Machine (Offer/Answer Model, RFC 3264)
    // pub fn pjmedia_sdp_neg_state_str(state: pjmedia_sdp_neg_state) -> *const c_char;
    // pub fn pjmedia_sdp_neg_create_w_local_offer( pool: *mut pj_pool_t, local: *const pjmedia_sdp_session, p_neg: *mut *mut pjmedia_sdp_neg, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_create_w_remote_offer( pool: *mut pj_pool_t, initial: *const pjmedia_sdp_session, remote: *const pjmedia_sdp_session, p_neg: *mut *mut pjmedia_sdp_neg, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_set_prefer_remote_codec_order( neg: *mut pjmedia_sdp_neg, prefer_remote: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_set_answer_multiple_codecs( neg: *mut pjmedia_sdp_neg, answer_multiple: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_get_state(neg: *mut pjmedia_sdp_neg) -> pjmedia_sdp_neg_state;
    // pub fn pjmedia_sdp_neg_get_active_local( neg: *mut pjmedia_sdp_neg, local: *mut *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_get_active_remote( neg: *mut pjmedia_sdp_neg, remote: *mut *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_was_answer_remote(neg: *mut pjmedia_sdp_neg) -> pj_bool_t;
    // pub fn pjmedia_sdp_neg_get_neg_remote( neg: *mut pjmedia_sdp_neg, remote: *mut *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_get_neg_local( neg: *mut pjmedia_sdp_neg, local: *mut *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_modify_local_offer( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, local: *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_modify_local_offer2( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, flags: c_uint, local: *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_send_local_offer( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, offer: *mut *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_set_remote_answer( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, remote: *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_set_remote_offer( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, remote: *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_set_local_answer( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, local: *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_has_local_answer(neg: *mut pjmedia_sdp_neg) -> pj_bool_t;
    // pub fn pjmedia_sdp_neg_cancel_offer(neg: *mut pjmedia_sdp_neg) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_negotiate( pool: *mut pj_pool_t, neg: *mut pjmedia_sdp_neg, allow_asym: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_register_fmt_match_cb( fmt_name: *const pj_str_t, cb: pjmedia_sdp_neg_fmt_match_cb, ) -> pj_status_t;
    // pub fn pjmedia_sdp_neg_fmt_match( pool: *mut pj_pool_t, offer: *mut pjmedia_sdp_media, o_fmt_idx: c_uint, answer: *mut pjmedia_sdp_media, a_fmt_idx: c_uint, option: c_uint, ) -> pj_status_t;


    // pub fn pjmedia_rtcp_fb_setting_default(opt: *mut pjmedia_rtcp_fb_setting) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_setting_dup( pool: *mut pj_pool_t, dst: *mut pjmedia_rtcp_fb_setting, src: *const pjmedia_rtcp_fb_setting, );
    // pub fn pjmedia_rtcp_fb_info_dup( pool: *mut pj_pool_t, dst: *mut pjmedia_rtcp_fb_info, src: *const pjmedia_rtcp_fb_info, );
    // pub fn pjmedia_rtcp_fb_encode_sdp( pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, opt: *const pjmedia_rtcp_fb_setting, sdp_local: *mut pjmedia_sdp_session, med_idx: c_uint, sdp_remote: *const pjmedia_sdp_session, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_decode_sdp( pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, opt: *const c_void, sdp: *const pjmedia_sdp_session, med_idx: c_uint, info: *mut pjmedia_rtcp_fb_info, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_decode_sdp2( pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, opt: *const c_void, sdp: *const pjmedia_sdp_session, med_idx: c_uint, pt: c_int, info: *mut pjmedia_rtcp_fb_info, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_build_nack( session: *mut pjmedia_rtcp_session, buf: *mut c_void, length: *mut pj_size_t, nack_cnt: c_uint, nack: *const pjmedia_rtcp_fb_nack, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_build_pli( session: *mut pjmedia_rtcp_session, buf: *mut c_void, length: *mut pj_size_t, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_build_sli( session: *mut pjmedia_rtcp_session, buf: *mut c_void, length: *mut pj_size_t, sli_cnt: c_uint, sli: *const pjmedia_rtcp_fb_sli, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_build_rpsi( session: *mut pjmedia_rtcp_session, buf: *mut c_void, length: *mut pj_size_t, rpsi: *const pjmedia_rtcp_fb_rpsi, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_parse_nack( buf: *const c_void, length: pj_size_t, nack_cnt: *mut c_uint, nack: *mut pjmedia_rtcp_fb_nack, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_parse_pli( buf: *const c_void, length: pj_size_t, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_parse_sli( buf: *const c_void, length: pj_size_t, sli_cnt: *mut c_uint, sli: *mut pjmedia_rtcp_fb_sli, ) -> pj_status_t;
    // pub fn pjmedia_rtcp_fb_parse_rpsi( buf: *const c_void, length: pj_size_t, rpsi: *mut pjmedia_rtcp_fb_rpsi, ) -> pj_status_t;

    // pub fn pjmedia_videodev_strerror( status: pj_status_t, buffer: *mut c_char, bufsize: pj_size_t, ) -> pj_str_t;
    // pub fn pjmedia_get_vid_subsys() -> *mut pjmedia_vid_subsys;
    // pub fn pjmedia_vid_driver_init( drv_idx: c_uint, refresh: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_vid_driver_deinit(drv_idx: c_uint);
    // pub fn pjmedia_vid_dev_cap_name( cap: pjmedia_vid_dev_cap, p_desc: *mut *const c_char, ) -> *const c_char;
    // pub fn pjmedia_vid_dev_param_set_cap( param: *mut pjmedia_vid_dev_param, cap: pjmedia_vid_dev_cap, pval: *const c_void, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_param_get_cap( param: *const pjmedia_vid_dev_param, cap: pjmedia_vid_dev_cap, pval: *mut c_void, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_refresh() -> pj_status_t;
    // pub fn pjmedia_vid_dev_count() -> c_uint;
    // pub fn pjmedia_vid_dev_get_info( id: pjmedia_vid_dev_index, info: *mut pjmedia_vid_dev_info, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_lookup( drv_name: *const c_char, dev_name: *const c_char, id: *mut pjmedia_vid_dev_index, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_default_param( pool: *mut pj_pool_t, id: pjmedia_vid_dev_index, param: *mut pjmedia_vid_dev_param, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_create( param: *mut pjmedia_vid_dev_param, cb: *const pjmedia_vid_dev_cb, user_data: *mut c_void, p_strm: *mut *mut pjmedia_vid_dev_stream, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_get_param( strm: *mut pjmedia_vid_dev_stream, param: *mut pjmedia_vid_dev_param, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_get_cap( strm: *mut pjmedia_vid_dev_stream, cap: pjmedia_vid_dev_cap, value: *mut c_void, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_set_cap( strm: *mut pjmedia_vid_dev_stream, cap: pjmedia_vid_dev_cap, value: *const c_void, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_start(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_is_running(strm: *mut pjmedia_vid_dev_stream) -> pj_bool_t;
    // pub fn pjmedia_vid_dev_stream_get_frame( strm: *mut pjmedia_vid_dev_stream, frame: *mut pjmedia_frame, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_put_frame( strm: *mut pjmedia_vid_dev_stream, frame: *const pjmedia_frame, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_stop(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    // pub fn pjmedia_vid_dev_stream_destroy(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;

    pub fn pjmedia_event_mgr_create( pool: *mut pj_pool_t, options: c_uint, mgr: *mut *mut pjmedia_event_mgr, ) -> pj_status_t;
    pub fn pjmedia_event_mgr_instance() -> *mut pjmedia_event_mgr;
    pub fn pjmedia_event_mgr_set_instance(mgr: *mut pjmedia_event_mgr);
    pub fn pjmedia_event_mgr_destroy(mgr: *mut pjmedia_event_mgr);
    pub fn pjmedia_event_init( event: *mut pjmedia_event, type_: pjmedia_event_type, ts: *const pj_timestamp, src: *const c_void, );
    pub fn pjmedia_event_subscribe( mgr: *mut pjmedia_event_mgr, cb: pjmedia_event_cb, user_data: *mut c_void, epub: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_event_unsubscribe( mgr: *mut pjmedia_event_mgr, cb: pjmedia_event_cb, user_data: *mut c_void, epub: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_event_publish( mgr: *mut pjmedia_event_mgr, epub: *mut c_void, event: *mut pjmedia_event, flag: pjmedia_event_publish_flag, ) -> pj_status_t;

    pub fn pjmedia_codec_param_clone( pool: *mut pj_pool_t, src: *const pjmedia_codec_param, ) -> *mut pjmedia_codec_param;
    pub fn pjmedia_codec_mgr_init( mgr: *mut pjmedia_codec_mgr, pf: *mut pj_pool_factory, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_destroy(mgr: *mut pjmedia_codec_mgr) -> pj_status_t;
    pub fn pjmedia_codec_mgr_register_factory( mgr: *mut pjmedia_codec_mgr, factory: *mut pjmedia_codec_factory, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_unregister_factory( mgr: *mut pjmedia_codec_mgr, factory: *mut pjmedia_codec_factory, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_enum_codecs( mgr: *mut pjmedia_codec_mgr, count: *mut c_uint, info: *mut pjmedia_codec_info, prio: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_get_codec_info( mgr: *mut pjmedia_codec_mgr, pt: c_uint, inf: *mut *const pjmedia_codec_info, ) -> pj_status_t;
    pub fn pjmedia_codec_info_to_id( info: *const pjmedia_codec_info, id: *mut c_char, max_len: c_uint, ) -> *mut c_char;
    pub fn pjmedia_codec_mgr_find_codecs_by_id( mgr: *mut pjmedia_codec_mgr, codec_id: *const pj_str_t, count: *mut c_uint, p_info: *mut *const pjmedia_codec_info, prio: *mut c_uint, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_set_codec_priority( mgr: *mut pjmedia_codec_mgr, codec_id: *const pj_str_t, prio: pj_uint8_t, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_get_default_param( mgr: *mut pjmedia_codec_mgr, info: *const pjmedia_codec_info, param: *mut pjmedia_codec_param, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_set_default_param( mgr: *mut pjmedia_codec_mgr, info: *const pjmedia_codec_info, param: *const pjmedia_codec_param, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_alloc_codec( mgr: *mut pjmedia_codec_mgr, info: *const pjmedia_codec_info, p_codec: *mut *mut pjmedia_codec, ) -> pj_status_t;
    pub fn pjmedia_codec_mgr_dealloc_codec( mgr: *mut pjmedia_codec_mgr, codec: *mut pjmedia_codec, ) -> pj_status_t;

    pub fn pjmedia_converter_mgr_create( pool: *mut pj_pool_t, mgr: *mut *mut pjmedia_converter_mgr, ) -> pj_status_t;
    pub fn pjmedia_converter_mgr_instance() -> *mut pjmedia_converter_mgr;
    pub fn pjmedia_converter_mgr_set_instance(mgr: *mut pjmedia_converter_mgr);
    pub fn pjmedia_converter_mgr_destroy(mgr: *mut pjmedia_converter_mgr);
    pub fn pjmedia_converter_mgr_register_factory( mgr: *mut pjmedia_converter_mgr, f: *mut pjmedia_converter_factory, ) -> pj_status_t;
    pub fn pjmedia_converter_mgr_unregister_factory( mgr: *mut pjmedia_converter_mgr, f: *mut pjmedia_converter_factory, call_destroy: pj_bool_t, ) -> pj_status_t;
    pub fn pjmedia_converter_create( mgr: *mut pjmedia_converter_mgr, pool: *mut pj_pool_t, param: *mut pjmedia_conversion_param, p_cv: *mut *mut pjmedia_converter, ) -> pj_status_t;
    pub fn pjmedia_converter_convert( cv: *mut pjmedia_converter, src_frame: *mut pjmedia_frame, dst_frame: *mut pjmedia_frame, ) -> pj_status_t;
    pub fn pjmedia_converter_convert2( cv: *mut pjmedia_converter, src_frame: *mut pjmedia_frame, src_frame_size: *const pjmedia_rect_size, src_pos: *const pjmedia_coord, dst_frame: *mut pjmedia_frame, dst_frame_size: *const pjmedia_rect_size, dst_pos: *const pjmedia_coord, param: *mut pjmedia_converter_convert_setting, ) -> pj_status_t;
    pub fn pjmedia_converter_destroy(cv: *mut pjmedia_converter);

    pub fn pjmedia_delay_buf_create( pool: *mut pj_pool_t, name: *const c_char, clock_rate: c_uint, samples_per_frame: c_uint, channel_count: c_uint, max_delay: c_uint, options: c_uint, p_b: *mut *mut pjmedia_delay_buf, ) -> pj_status_t;
    pub fn pjmedia_delay_buf_put(b: *mut pjmedia_delay_buf, frame: *mut pj_int16_t) -> pj_status_t;
    pub fn pjmedia_delay_buf_get(b: *mut pjmedia_delay_buf, frame: *mut pj_int16_t) -> pj_status_t;
    pub fn pjmedia_delay_buf_reset(b: *mut pjmedia_delay_buf) -> pj_status_t;
    pub fn pjmedia_delay_buf_destroy(b: *mut pjmedia_delay_buf) -> pj_status_t;

    pub fn pjmedia_strerror( status: pj_status_t, buffer: *mut c_char, bufsize: pj_size_t, ) -> pj_str_t;

    pub fn pjmedia_aud_subsys_init(pf: *mut pj_pool_factory) -> pj_status_t;
    pub fn pjmedia_aud_subsys_get_pool_factory() -> *mut pj_pool_factory;
    pub fn pjmedia_aud_subsys_shutdown() -> pj_status_t;
    pub fn pjmedia_aud_register_factory( adf: pjmedia_aud_dev_factory_create_func_ptr, ) -> pj_status_t;
    pub fn pjmedia_aud_unregister_factory( adf: pjmedia_aud_dev_factory_create_func_ptr, ) -> pj_status_t;

    pub fn pjmedia_endpt_create2( pf: *mut pj_pool_factory, ioqueue: *mut pj_ioqueue_t, worker_cnt: c_uint, p_endpt: *mut *mut pjmedia_endpt, ) -> pj_status_t;
    pub fn pjmedia_endpt_destroy2(endpt: *mut pjmedia_endpt) -> pj_status_t;
    pub fn pjmedia_endpt_set_flag( endpt: *mut pjmedia_endpt, flag: pjmedia_endpt_flag, value: *const c_void, ) -> pj_status_t;
    pub fn pjmedia_endpt_get_flag( endpt: *mut pjmedia_endpt, flag: pjmedia_endpt_flag, value: *mut c_void, ) -> pj_status_t;
    pub fn pjmedia_endpt_get_ioqueue(endpt: *mut pjmedia_endpt) -> *mut pj_ioqueue_t;
    pub fn pjmedia_endpt_get_thread_count(endpt: *mut pjmedia_endpt) -> c_uint;
    pub fn pjmedia_endpt_get_thread( endpt: *mut pjmedia_endpt, index: c_uint, ) -> *mut pj_thread_t;
    pub fn pjmedia_endpt_stop_threads(endpt: *mut pjmedia_endpt) -> pj_status_t;
    pub fn pjmedia_endpt_create_pool( endpt: *mut pjmedia_endpt, name: *const c_char, initial: pj_size_t, increment: pj_size_t, ) -> *mut pj_pool_t;
    pub fn pjmedia_endpt_get_codec_mgr(endpt: *mut pjmedia_endpt) -> *mut pjmedia_codec_mgr;
    pub fn pjmedia_endpt_create_sdp( endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, stream_cnt: c_uint, sock_info: *const pjmedia_sock_info, p_sdp: *mut *mut pjmedia_sdp_session, ) -> pj_status_t;
    pub fn pjmedia_endpt_create_base_sdp( endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, sess_name: *const pj_str_t, origin: *const pj_sockaddr, p_sdp: *mut *mut pjmedia_sdp_session, ) -> pj_status_t;
    pub fn pjmedia_endpt_create_audio_sdp( endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, si: *const pjmedia_sock_info, options: c_uint, p_m: *mut *mut pjmedia_sdp_media, ) -> pj_status_t;
    pub fn pjmedia_endpt_create_video_sdp( endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, si: *const pjmedia_sock_info, options: c_uint, p_m: *mut *mut pjmedia_sdp_media, ) -> pj_status_t;
    pub fn pjmedia_endpt_dump(endpt: *mut pjmedia_endpt) -> pj_status_t;
    pub fn pjmedia_endpt_atexit( endpt: *mut pjmedia_endpt, func: pjmedia_endpt_exit_callback, ) -> pj_status_t;


    // Jitter buffer Module
    // pub fn pjmedia_jbuf_create( pool: *mut pj_pool_t, name: *const pj_str_t, frame_size: c_uint, ptime: c_uint, max_count: c_uint, p_jb: *mut *mut pjmedia_jbuf, ) -> pj_status_t;
    // pub fn pjmedia_jbuf_set_ptime( jb: *mut pjmedia_jbuf, ptime: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_jbuf_set_fixed( jb: *mut pjmedia_jbuf, prefetch: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_jbuf_set_adaptive( jb: *mut pjmedia_jbuf, prefetch: c_uint, min_prefetch: c_uint, max_prefetch: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_jbuf_set_discard( jb: *mut pjmedia_jbuf, algo: pjmedia_jb_discard_algo, ) -> pj_status_t;
    // pub fn pjmedia_jbuf_destroy(jb: *mut pjmedia_jbuf) -> pj_status_t;
    // pub fn pjmedia_jbuf_reset(jb: *mut pjmedia_jbuf) -> pj_status_t;
    // pub fn pjmedia_jbuf_put_frame( jb: *mut pjmedia_jbuf, frame: *const c_void, size: pj_size_t, frame_seq: c_int,);
    // pub fn pjmedia_jbuf_put_frame2( jb: *mut pjmedia_jbuf, frame: *const c_void, size: pj_size_t, bit_info: pj_uint32_t, frame_seq: c_int, discarded: *mut pj_bool_t, );
    // pub fn pjmedia_jbuf_put_frame3( jb: *mut pjmedia_jbuf, frame: *const c_void, size: pj_size_t, bit_info: pj_uint32_t, frame_seq: c_int, frame_ts: pj_uint32_t, discarded: *mut pj_bool_t, );
    // pub fn pjmedia_jbuf_get_frame( jb: *mut pjmedia_jbuf, frame: *mut c_void, p_frm_type: *mut c_char,);
    // pub fn pjmedia_jbuf_get_frame2( jb: *mut pjmedia_jbuf, frame: *mut c_void, size: *mut pj_size_t, p_frm_type: *mut c_char, bit_info: *mut pj_uint32_t, );
    // pub fn pjmedia_jbuf_get_frame3( jb: *mut pjmedia_jbuf, frame: *mut c_void, size: *mut pj_size_t, p_frm_type: *mut c_char, bit_info: *mut pj_uint32_t, ts: *mut pj_uint32_t, seq: *mut c_int,);
    // pub fn pjmedia_jbuf_peek_frame( jb: *mut pjmedia_jbuf, offset: c_uint, frame: *mut *const c_void, size: *mut pj_size_t, p_frm_type: *mut c_char, bit_info: *mut pj_uint32_t, ts: *mut pj_uint32_t, seq: *mut c_int, );
    // pub fn pjmedia_jbuf_remove_frame( jb: *mut pjmedia_jbuf, frame_cnt: c_uint, ) -> c_uint;
    // pub fn pjmedia_jbuf_is_full(jb: *const pjmedia_jbuf) -> pj_bool_t;
    // pub fn pjmedia_jbuf_get_state( jb: *const pjmedia_jbuf, state: *mut pjmedia_jb_state, ) -> pj_status_t;

    // Packet lost concalement
    pub fn pjmedia_plc_create( pool: *mut pj_pool_t, clock_rate: c_uint, samples_per_frame: c_uint, options: c_uint, p_plc: *mut *mut pjmedia_plc, ) -> pj_status_t;
    pub fn pjmedia_plc_save(plc: *mut pjmedia_plc, frame: *mut pj_int16_t) -> pj_status_t;
    pub fn pjmedia_plc_generate(plc: *mut pjmedia_plc, frame: *mut pj_int16_t) -> pj_status_t;


    // Silence Detection Module

    // pub fn pjmedia_silence_det_create( pool: *mut pj_pool_t, clock_rate: c_uint, samples_per_frame: c_uint, p_sd: *mut *mut pjmedia_silence_det, ) -> pj_status_t;
    // pub fn pjmedia_silence_det_set_name( sd: *mut pjmedia_silence_det, name: *const c_char, ) -> pj_status_t;
    // pub fn pjmedia_silence_det_set_fixed( sd: *mut pjmedia_silence_det, threshold: c_int, ) -> pj_status_t;
    // pub fn pjmedia_silence_det_set_adaptive( sd: *mut pjmedia_silence_det, threshold: c_int, ) -> pj_status_t;
    // pub fn pjmedia_silence_det_set_params( sd: *mut pjmedia_silence_det, before_silence: c_int, recalc_time1: c_int, recalc_time2: c_int, ) -> pj_status_t;
    // pub fn pjmedia_silence_det_disable(sd: *mut pjmedia_silence_det) -> pj_status_t;
    // pub fn pjmedia_silence_det_detect( sd: *mut pjmedia_silence_det, samples: *const pj_int16_t, count: pj_size_t, p_level: *mut pj_int32_t, ) -> pj_bool_t;
    // pub fn pjmedia_calc_avg_signal(samples: *const pj_int16_t, count: pj_size_t) -> pj_int32_t;
    // pub fn pjmedia_silence_det_apply(sd: *mut pjmedia_silence_det, level: pj_uint32_t) -> pj_bool_t;

    pub fn pjmedia_snd_get_dev_info(index: c_uint) -> *const pjmedia_snd_dev_info;
    pub fn pjmedia_snd_set_latency( input_latency: c_uint, output_latency: c_uint, ) -> pj_status_t;
    pub fn pjmedia_snd_open( rec_id: c_int, play_id: c_int, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, rec_cb: pjmedia_snd_rec_cb, play_cb: pjmedia_snd_play_cb, user_data: *mut c_void, p_snd_strm: *mut *mut pjmedia_snd_stream, ) -> pj_status_t;
    pub fn pjmedia_snd_open_rec( index: c_int, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, rec_cb: pjmedia_snd_rec_cb, user_data: *mut c_void, p_snd_strm: *mut *mut pjmedia_snd_stream, ) -> pj_status_t;
    pub fn pjmedia_snd_open_player( index: c_int, clock_rate: c_uint, channel_count: c_uint, samples_per_frame: c_uint, bits_per_sample: c_uint, play_cb: pjmedia_snd_play_cb, user_data: *mut c_void, p_snd_strm: *mut *mut pjmedia_snd_stream, ) -> pj_status_t;
    pub fn pjmedia_snd_stream_get_info( strm: *mut pjmedia_snd_stream, pi: *mut pjmedia_snd_stream_info, ) -> pj_status_t;
    pub fn pjmedia_snd_stream_start(stream: *mut pjmedia_snd_stream) -> pj_status_t;
    pub fn pjmedia_snd_stream_stop(stream: *mut pjmedia_snd_stream) -> pj_status_t;
    pub fn pjmedia_snd_stream_close(stream: *mut pjmedia_snd_stream) -> pj_status_t;



    pub fn pjmedia_stereo_port_create( pool: *mut pj_pool_t, dn_port: *mut pjmedia_port, channel_count: c_uint, options: c_uint, p_port: *mut *mut pjmedia_port, ) -> pj_status_t;

    // pub fn pjmedia_vid_codec_param_clone( pool: *mut pj_pool_t, src: *const pjmedia_vid_codec_param, ) -> *mut pjmedia_vid_codec_param;
    // pub fn pjmedia_vid_codec_mgr_create( pool: *mut pj_pool_t, mgr: *mut *mut pjmedia_vid_codec_mgr, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_destroy(mgr: *mut pjmedia_vid_codec_mgr) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_instance() -> *mut pjmedia_vid_codec_mgr;
    // pub fn pjmedia_vid_codec_mgr_set_instance(mgr: *mut pjmedia_vid_codec_mgr);
    // pub fn pjmedia_vid_codec_mgr_register_factory( mgr: *mut pjmedia_vid_codec_mgr, factory: *mut pjmedia_vid_codec_factory, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_unregister_factory( mgr: *mut pjmedia_vid_codec_mgr, factory: *mut pjmedia_vid_codec_factory, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_enum_codecs( mgr: *mut pjmedia_vid_codec_mgr, count: *mut c_uint, info: *mut pjmedia_vid_codec_info, prio: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_get_codec_info( mgr: *mut pjmedia_vid_codec_mgr, pt: c_uint, info: *mut *const pjmedia_vid_codec_info, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_get_codec_info2( mgr: *mut pjmedia_vid_codec_mgr, fmt_id: pjmedia_format_id, info: *mut *const pjmedia_vid_codec_info, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_info_to_id( info: *const pjmedia_vid_codec_info, id: *mut c_char, max_len: c_uint, ) -> *mut c_char;
    // pub fn pjmedia_vid_codec_mgr_find_codecs_by_id( mgr: *mut pjmedia_vid_codec_mgr, codec_id: *const pj_str_t, count: *mut c_uint, p_info: *mut *const pjmedia_vid_codec_info, prio: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_set_codec_priority( mgr: *mut pjmedia_vid_codec_mgr, codec_id: *const pj_str_t, prio: pj_uint8_t, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_get_default_param( mgr: *mut pjmedia_vid_codec_mgr, info: *const pjmedia_vid_codec_info, param: *mut pjmedia_vid_codec_param, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_set_default_param( mgr: *mut pjmedia_vid_codec_mgr, info: *const pjmedia_vid_codec_info, param: *const pjmedia_vid_codec_param, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_alloc_codec( mgr: *mut pjmedia_vid_codec_mgr, info: *const pjmedia_vid_codec_info, p_codec: *mut *mut pjmedia_vid_codec, ) -> pj_status_t;
    // pub fn pjmedia_vid_codec_mgr_dealloc_codec( mgr: *mut pjmedia_vid_codec_mgr, codec: *mut pjmedia_vid_codec, ) -> pj_status_t;

    // pub fn pjmedia_vid_dev_subsys_init(pf: *mut pj_pool_factory) -> pj_status_t;
    // pub fn pjmedia_vid_dev_subsys_get_pool_factory() -> *mut pj_pool_factory;
    // pub fn pjmedia_vid_dev_subsys_shutdown() -> pj_status_t;

    // pub fn pjmedia_vid_register_factory( vdf: pjmedia_vid_dev_factory_create_func_ptr, factory: *mut pjmedia_vid_dev_factory, ) -> pj_status_t;
    // pub fn pjmedia_vid_unregister_factory( vdf: pjmedia_vid_dev_factory_create_func_ptr, factory: *mut pjmedia_vid_dev_factory, ) -> pj_status_t;

    pub fn pjmedia_wave_hdr_file_to_host(hdr: *mut pjmedia_wave_hdr);
    pub fn pjmedia_wave_hdr_host_to_file(hdr: *mut pjmedia_wave_hdr);

    // pub fn pjmedia_wsola_create( pool: *mut pj_pool_t, clock_rate: c_uint, samples_per_frame: c_uint, channel_count: c_uint, options: c_uint, p_wsola: *mut *mut pjmedia_wsola, ) -> pj_status_t;
    // pub fn pjmedia_wsola_set_max_expand( wsola: *mut pjmedia_wsola, msec: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_wsola_destroy(wsola: *mut pjmedia_wsola) -> pj_status_t;
    // pub fn pjmedia_wsola_reset( wsola: *mut pjmedia_wsola, options: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_wsola_save( wsola: *mut pjmedia_wsola, frm: *mut pj_int16_t, prev_lost: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_wsola_generate(wsola: *mut pjmedia_wsola, frm: *mut pj_int16_t) -> pj_status_t;
    // pub fn pjmedia_wsola_discard( wsola: *mut pjmedia_wsola, buf1: *mut pj_int16_t, buf1_cnt: c_uint, buf2: *mut pj_int16_t, buf2_cnt: c_uint, erase_cnt: *mut c_uint, ) -> pj_status_t;

    // pub fn pjmedia_codec_g711_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_g711_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_passthrough_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_passthrough_init2( endpt: *mut pjmedia_endpt, setting: *const pjmedia_codec_passthrough_setting, ) -> pj_status_t;
    // pub fn pjmedia_codec_passthrough_deinit() -> pj_status_t;

    pub fn pjmedia_audio_codec_config_default(cfg: *mut pjmedia_audio_codec_config);
    pub fn pjmedia_codec_register_audio_codecs( endpt: *mut pjmedia_endpt, c: *const pjmedia_audio_codec_config, ) -> pj_status_t;

    // pub fn pjmedia_codec_bcg729_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_bcg729_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_ffmpeg_vid_init( mgr: *mut pjmedia_vid_codec_mgr, pf: *mut pj_pool_factory, ) -> pj_status_t;
    // pub fn pjmedia_codec_ffmpeg_vid_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_g722_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_g722_deinit() -> pj_status_t;
    // pub fn pjmedia_codec_g722_set_pcm_shift(val: c_uint) -> pj_status_t;

    // pub fn pjmedia_codec_g7221_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_g7221_set_mode( sample_rate: c_uint, bitrate: c_uint, enabled: pj_bool_t, ) -> pj_status_t;
    // pub fn pjmedia_codec_g7221_set_pcm_shift(val: c_int) -> pj_status_t;
    // pub fn pjmedia_codec_g7221_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_gsm_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_gsm_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_ilbc_init( endpt: *mut pjmedia_endpt, mode: c_int, ) -> pj_status_t;
    // pub fn pjmedia_codec_ilbc_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_ipp_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_ipp_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_l16_init( endpt: *mut pjmedia_endpt, options: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_codec_l16_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_opencore_amr_init( endpt: *mut pjmedia_endpt, options: c_uint, ) -> pj_status_t;
    // pub fn pjmedia_codec_opencore_amr_init_default(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_opencore_amr_deinit() -> pj_status_t;
    // pub fn pjmedia_codec_opencore_amrnb_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_opencore_amrnb_deinit() -> pj_status_t;
    // pub fn pjmedia_codec_opencore_amrnb_set_config( cfg: *const pjmedia_codec_amrnb_config, ) -> pj_status_t;
    // pub fn pjmedia_codec_opencore_amrwb_set_config( cfg: *const pjmedia_codec_amrwb_config, ) -> pj_status_t;

    // pub fn pjmedia_codec_openh264_vid_init( mgr: *mut pjmedia_vid_codec_mgr, pf: *mut pj_pool_factory, ) -> pj_status_t;
    // pub fn pjmedia_codec_openh264_vid_deinit() -> pj_status_t;

    pub fn pjmedia_codec_opus_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    pub fn pjmedia_codec_opus_deinit() -> pj_status_t;
    pub fn pjmedia_codec_opus_get_config(cfg: *mut pjmedia_codec_opus_config) -> pj_status_t;
    pub fn pjmedia_codec_opus_set_default_param( cfg: *const pjmedia_codec_opus_config, param: *mut pjmedia_codec_param, ) -> pj_status_t;

    // pub fn pjmedia_codec_silk_init(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_silk_set_config( clock_rate: c_uint, opt: *const pjmedia_codec_silk_setting, ) -> pj_status_t;
    // pub fn pjmedia_codec_silk_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_speex_init( endpt: *mut pjmedia_endpt, options: c_uint, quality: c_int, complexity: c_int, ) -> pj_status_t;
    // pub fn pjmedia_codec_speex_init_default(endpt: *mut pjmedia_endpt) -> pj_status_t;
    // pub fn pjmedia_codec_speex_set_param( clock_rate: c_uint, quality: c_int, complexity: c_int, ) -> pj_status_t;
    // pub fn pjmedia_codec_speex_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_vid_toolbox_init( mgr: *mut pjmedia_vid_codec_mgr, pf: *mut pj_pool_factory, ) -> pj_status_t;
    // pub fn pjmedia_codec_vid_toolbox_deinit() -> pj_status_t;

    // pub fn pjmedia_codec_vpx_vid_init( mgr: *mut pjmedia_vid_codec_mgr, pf: *mut pj_pool_factory, ) -> pj_status_t;
    // pub fn pjmedia_codec_vpx_vid_deinit() -> pj_status_t;

    // pub fn pjmedia_vid_dev_get_local_index( id: pjmedia_vid_dev_index, p_f: *mut *mut pjmedia_vid_dev_factory, p_local_index: *mut c_uint, ) -> pj_status_t;
    // pub fn pjmedia_vid_dev_get_global_index( f: *const pjmedia_vid_dev_factory, local_idx: c_uint, pid: *mut pjmedia_vid_dev_index, ) -> pj_status_t;

    // pub fn pjmedia_avi_dev_param_default(p: *mut pjmedia_avi_dev_param);
    // pub fn pjmedia_avi_dev_create_factory( pf: *mut pj_pool_factory, max_dev: c_uint, p_ret: *mut *mut pjmedia_vid_dev_factory, ) -> pj_status_t;
    // pub fn pjmedia_avi_dev_alloc( f: *mut pjmedia_vid_dev_factory, param: *mut pjmedia_avi_dev_param, p_id: *mut pjmedia_vid_dev_index, ) -> pj_status_t;
    // pub fn pjmedia_avi_dev_get_param( id: pjmedia_vid_dev_index, param: *mut pjmedia_avi_dev_param, ) -> pj_status_t;
    // pub fn pjmedia_avi_dev_free(id: pjmedia_vid_dev_index) -> pj_status_t;
}
