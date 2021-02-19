#![allow(dead_code)]

use super::pjdefault::AutoCreate;
use super::pjsua_sys::*;
use std::ptr;

pub const PJMEDIA_POOL_LEN_ENDPT: u32 = 512;
pub const PJMEDIA_POOL_INC_ENDPT: u32 = 512;
pub const PJMEDIA_POOL_LEN_EVTMGR: u32 = 500;
pub const PJMEDIA_POOL_INC_EVTMGR: u32 = 500;
pub const PJMEDIA_CONF_USE_SWITCH_BOARD: u32 = 0;
pub const PJMEDIA_CONF_USE_AGC: u32 = 1;
pub const PJMEDIA_HAS_LEGACY_SOUND_API: u32 = 1;
pub const PJMEDIA_SND_DEFAULT_REC_LATENCY: u32 = 100;
pub const PJMEDIA_SND_DEFAULT_PLAY_LATENCY: u32 = 140;
pub const PJMEDIA_WSOLA_IMP_NULL: u32 = 0;
pub const PJMEDIA_WSOLA_IMP_WSOLA: u32 = 1;
pub const PJMEDIA_WSOLA_IMP_WSOLA_LITE: u32 = 2;
pub const PJMEDIA_WSOLA_IMP: u32 = 1;
pub const PJMEDIA_WSOLA_MAX_EXPAND_MSEC: u32 = 80;
pub const PJMEDIA_WSOLA_TEMPLATE_LENGTH_MSEC: u32 = 5;
pub const PJMEDIA_WSOLA_DELAY_MSEC: u32 = 5;
pub const PJMEDIA_WSOLA_PLC_NO_FADING: u32 = 0;
pub const PJMEDIA_MAX_PLC_DURATION_MSEC: u32 = 240;
pub const PJMEDIA_SOUND_BUFFER_COUNT: u32 = 8;
pub const PJMEDIA_HAS_ALAW_ULAW_TABLE: u32 = 1;
pub const PJMEDIA_HAS_G711_CODEC: u32 = 1;
pub const PJMEDIA_RESAMPLE_NONE: u32 = 1;
pub const PJMEDIA_RESAMPLE_LIBRESAMPLE: u32 = 2;
pub const PJMEDIA_RESAMPLE_SPEEX: u32 = 3;
pub const PJMEDIA_RESAMPLE_LIBSAMPLERATE: u32 = 4;
pub const PJMEDIA_RESAMPLE_IMP: u32 = 2;
pub const PJMEDIA_FILE_PORT_BUFSIZE: u32 = 4000;
pub const PJMEDIA_MAX_FRAME_DURATION_MS: u32 = 200;
pub const PJMEDIA_MAX_MTU: u32 = 1500;
pub const PJMEDIA_MAX_MRU: u32 = 2000;
pub const PJMEDIA_DTMF_DURATION: u32 = 1600;
pub const PJMEDIA_DTMF_DURATION_MSEC: u32 = 0;
pub const PJMEDIA_RTP_NAT_PROBATION_CNT: u32 = 10;
pub const PJMEDIA_RTCP_NAT_PROBATION_CNT: u32 = 3;
pub const PJMEDIA_ADVERTISE_RTCP: u32 = 1;
pub const PJMEDIA_RTCP_INTERVAL: u32 = 5000;
pub const PJMEDIA_RTCP_FB_INTERVAL: u32 = 50;
pub const PJMEDIA_RTCP_IGNORE_FIRST_PACKETS: u32 = 25;
pub const PJMEDIA_RTCP_STAT_HAS_RAW_JITTER: u32 = 0;
pub const PJMEDIA_RTCP_NORMALIZE_FACTOR: u32 = 3;
pub const PJMEDIA_RTCP_STAT_HAS_IPDV: u32 = 0;
pub const PJMEDIA_HAS_RTCP_XR: u32 = 0;
pub const PJMEDIA_STREAM_ENABLE_XR: u32 = 0;
pub const PJMEDIA_RTCP_RX_SDES_BUF_LEN: u32 = 64;
pub const PJMEDIA_RTCP_FB_MAX_CAP: u32 = 16;
pub const PJMEDIA_STREAM_VAD_SUSPEND_MSEC: u32 = 600;
pub const PJMEDIA_STREAM_CHECK_RTP_PT: u32 = 1;
pub const PJMEDIA_STREAM_RESV_PAYLOAD_LEN: u32 = 20;
pub const PJMEDIA_CODEC_MAX_SILENCE_PERIOD: u32 = 5000;
pub const PJMEDIA_SILENCE_DET_THRESHOLD: u32 = 4;
pub const PJMEDIA_SILENCE_DET_MAX_THRESHOLD: u32 = 65536;
pub const PJMEDIA_HAS_SPEEX_AEC: u32 = 1;
pub const PJMEDIA_SPEEX_AEC_USE_AGC: u32 = 1;
pub const PJMEDIA_SPEEX_AEC_USE_DENOISE: u32 = 1;
pub const PJMEDIA_HAS_WEBRTC_AEC: u32 = 0;
pub const PJMEDIA_WEBRTC_AEC_USE_MOBILE: u32 = 0;
pub const PJMEDIA_CODEC_MAX_FMTP_CNT: u32 = 16;
pub const PJMEDIA_SDP_NEG_PREFER_REMOTE_CODEC_ORDER: u32 = 1;
pub const PJMEDIA_SDP_NEG_ANSWER_MULTIPLE_CODECS: u32 = 0;
pub const PJMEDIA_SDP_NEG_MAX_CUSTOM_FMT_NEG_CB: u32 = 8;
pub const PJMEDIA_SDP_NEG_ANSWER_SYMMETRIC_PT: u32 = 1;
pub const PJMEDIA_SDP_NEG_COMPARE_BEFORE_INC_VERSION: u32 = 0;
pub const PJMEDIA_HAS_RTCP_IN_SDP: u32 = 1;
pub const PJMEDIA_ADD_BANDWIDTH_TIAS_IN_SDP: u32 = 1;
pub const PJMEDIA_ADD_RTPMAP_FOR_STATIC_PT: u32 = 1;
pub const PJMEDIA_RTP_PT_TELEPHONE_EVENTS: u32 = 120;
pub const PJMEDIA_TELEPHONE_EVENT_ALL_CLOCKRATES: u32 = 1;
pub const PJMEDIA_TONEGEN_MAX_DIGITS: u32 = 32;
pub const PJMEDIA_TONEGEN_SINE: u32 = 1;
pub const PJMEDIA_TONEGEN_FLOATING_POINT: u32 = 2;
pub const PJMEDIA_TONEGEN_FIXED_POINT_CORDIC: u32 = 3;
pub const PJMEDIA_TONEGEN_FAST_FIXED_POINT: u32 = 4;
pub const PJMEDIA_TONEGEN_ALG: u32 = 2;
pub const PJMEDIA_TONEGEN_FIXED_POINT_CORDIC_LOOP: u32 = 10;
pub const PJMEDIA_TONEGEN_FADE_IN_TIME: u32 = 1;
pub const PJMEDIA_TONEGEN_FADE_OUT_TIME: u32 = 2;
pub const PJMEDIA_TONEGEN_VOLUME: u32 = 12288;
pub const PJMEDIA_HAS_SRTP: u32 = 1;
pub const PJMEDIA_SRTP_HAS_SDES: u32 = 1;
pub const PJMEDIA_SRTP_HAS_DTLS: u32 = 0;
pub const PJMEDIA_SRTP_DTLS_OSSL_CIPHERS: &'static [u8; 8usize] = b"DEFAULT\0";
pub const PJMEDIA_SRTP_MAX_CRYPTOS: u32 = 16;
pub const PJMEDIA_SRTP_HAS_AES_CM_256: u32 = 1;
pub const PJMEDIA_SRTP_HAS_AES_CM_192: u32 = 0;
pub const PJMEDIA_SRTP_HAS_AES_CM_128: u32 = 1;
pub const PJMEDIA_SRTP_HAS_AES_GCM_256: u32 = 0;
pub const PJMEDIA_SRTP_HAS_AES_GCM_128: u32 = 0;
pub const PJMEDIA_LIBSRTP_AUTO_INIT_DEINIT: u32 = 1;
pub const PJMEDIA_HANDLE_G722_MPEG_BUG: u32 = 1;
pub const PJMEDIA_TRANSPORT_SWITCH_REMOTE_ADDR: u32 = 1;
pub const PJMEDIA_TRANSPORT_SPECIFIC_INFO_MAXCNT: u32 = 4;
pub const PJMEDIA_STREAM_KA_EMPTY_RTP: u32 = 1;
pub const PJMEDIA_STREAM_KA_USER: u32 = 2;
pub const PJMEDIA_STREAM_ENABLE_KA: u32 = 0;
pub const PJMEDIA_STREAM_KA_INTERVAL: u32 = 5;
pub const PJMEDIA_STREAM_START_KA_CNT: u32 = 2;
pub const PJMEDIA_STREAM_START_KA_INTERVAL_MSEC: u32 = 1000;
pub const PJMEDIA_IGNORE_RECV_ERR_CNT: u32 = 20;
pub const PJMEDIA_HAS_VIDEO: u32 = 0;
pub const PJMEDIA_HAS_FFMPEG: u32 = 0;
pub const PJMEDIA_HAS_LIBAVFORMAT: u32 = 0;
pub const PJMEDIA_HAS_LIBAVCODEC: u32 = 0;
pub const PJMEDIA_HAS_LIBAVUTIL: u32 = 0;
pub const PJMEDIA_HAS_LIBSWSCALE: u32 = 0;
pub const PJMEDIA_HAS_LIBAVDEVICE: u32 = 0;
pub const PJMEDIA_MAX_VIDEO_PLANES: u32 = 4;
pub const PJMEDIA_MAX_VIDEO_FORMATS: u32 = 32;
pub const PJMEDIA_CLOCK_SYNC_MAX_SYNC_MSEC: u32 = 20000;
pub const PJMEDIA_MAX_VIDEO_ENC_FRAME_SIZE: u32 = 131072;
pub const PJMEDIA_CLOCK_SYNC_MAX_RESYNC_DURATION: u32 = 2000;
pub const PJMEDIA_JBUF_DISC_MIN_GAP: u32 = 200;
pub const PJMEDIA_JBUF_PRO_DISC_MIN_BURST: u32 = 1;
pub const PJMEDIA_JBUF_PRO_DISC_MAX_BURST: u32 = 100;
pub const PJMEDIA_JBUF_PRO_DISC_T1: u32 = 2000;
pub const PJMEDIA_JBUF_PRO_DISC_T2: u32 = 10000;
pub const PJMEDIA_STREAM_SOFT_START: u32 = 1;
pub const PJMEDIA_VID_STREAM_SKIP_PACKETS_TO_REDUCE_LATENCY: u32 = 0;
pub const PJMEDIA_MAX_VID_PAYLOAD_SIZE: u32 = 1336;
pub const PJMEDIA_TRANSPORT_SO_RCVBUF_SIZE: u32 = 0;
pub const PJMEDIA_TRANSPORT_SO_SNDBUF_SIZE: u32 = 0;
pub const PJMEDIA_HAS_LIBYUV: u32 = 0;
pub const PJMEDIA_HAS_DTMF_FLASH: u32 = 1;
pub const PJMEDIA_VID_STREAM_START_KEYFRAME_CNT: u32 = 5;
pub const PJMEDIA_VID_STREAM_START_KEYFRAME_INTERVAL_MSEC: u32 = 1000;
pub const PJMEDIA_VID_STREAM_MIN_KEYFRAME_INTERVAL_MSEC: u32 = 1000;
pub const PJMEDIA_VID_STREAM_DECODE_MIN_DELAY_MSEC: u32 = 100;
pub const PJMEDIA_AUD_DEV_INFO_NAME_LEN: u32 = 64;
pub const PJMEDIA_AUDIO_DEV_HAS_PORTAUDIO: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_OPENSL: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_BB10: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_ALSA: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_NULL_AUDIO: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_COREAUDIO: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_WMME: u32 = 1;
pub const PJMEDIA_AUDIO_DEV_HAS_WASAPI: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_BDIMAD: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_HAS_SYMB_APS: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_SYMB_APS_DETECTS_CODEC: u32 = 1;
pub const PJMEDIA_AUDIO_DEV_HAS_SYMB_VAS: u32 = 0;
pub const PJMEDIA_AUDIO_DEV_SYMB_VAS_VERSION: u32 = 1;
pub const PJMEDIA_AUDIO_DEV_MDA_USE_SYNC_START: u32 = 1;
pub const PJMEDIA_AUDIO_DEV_HAS_LEGACY_DEVICE: u32 = 0;
pub const PJMEDIA_AUDIODEV_ERRNO_START: u32 = 420000;
pub const PJMEDIA_AUDIODEV_ERRNO_END: u32 = 469999;
pub const PJMEDIA_AUDIODEV_PORTAUDIO_ERRNO_START: u32 = 459999;
pub const PJMEDIA_AUDIODEV_PORTAUDIO_ERRNO_END: u32 = 469998;
pub const PJMEDIA_AUDIODEV_WMME_IN_ERROR_START: u32 = 450000;
pub const PJMEDIA_AUDIODEV_WMME_IN_ERROR_END: u32 = 450999;
pub const PJMEDIA_AUDIODEV_WMME_OUT_ERROR_START: u32 = 451999;
pub const PJMEDIA_AUDIODEV_WMME_OUT_ERROR_END: u32 = 452999;
pub const PJMEDIA_AUDIODEV_COREAUDIO_ERRNO_START: u32 = 440000;
pub const PJMEDIA_AUDIODEV_COREAUDIO_ERRNO_END: u32 = 459999;
pub const PJMEDIA_AUDIODEV_BDIMAD_ERROR_START: u32 = 460000;
pub const PJMEDIA_AUDIODEV_BDIMAD_ERROR_END: u32 = 461999;
pub const PJMEDIA_EAUD_ERR: u32 = 420001;
pub const PJMEDIA_EAUD_SYSERR: u32 = 420002;
pub const PJMEDIA_EAUD_INIT: u32 = 420003;
pub const PJMEDIA_EAUD_INVDEV: u32 = 420004;
pub const PJMEDIA_EAUD_NODEV: u32 = 420005;
pub const PJMEDIA_EAUD_NODEFDEV: u32 = 420006;
pub const PJMEDIA_EAUD_NOTREADY: u32 = 420007;
pub const PJMEDIA_EAUD_INVCAP: u32 = 420008;
pub const PJMEDIA_EAUD_INVOP: u32 = 420009;
pub const PJMEDIA_EAUD_BADFORMAT: u32 = 420010;
pub const PJMEDIA_EAUD_SAMPFORMAT: u32 = 420011;
pub const PJMEDIA_EAUD_BADLATENCY: u32 = 420012;
pub const PJMEDIA_EAUD_WASAPI_ERROR: u32 = 420013;
pub const PJMEDIA_FORMAT_DETAIL_USER_SIZE: u32 = 1;
pub const PJMEDIA_AUD_DEFAULT_CAPTURE_DEV: i32 = -1;
pub const PJMEDIA_AUD_DEFAULT_PLAYBACK_DEV: i32 = -2;
pub const PJMEDIA_AUD_INVALID_DEV: i32 = -3;
pub const PJMEDIA_AUD_MAX_DRIVERS: u32 = 16;
pub const PJMEDIA_AUD_MAX_DEVS: u32 = 64;
pub const PJMEDIA_RTP_DTMF_EVENT_END_MASK: u32 = 128;
pub const PJMEDIA_RTP_DTMF_EVENT_VOLUME_MASK: u32 = 63;
pub const PJMEDIA_MAX_SDP_FMT: u32 = 32;
pub const PJMEDIA_MAX_SDP_BANDW: u32 = 4;
pub const PJMEDIA_MAX_SDP_ATTR: u32 = 68;
pub const PJMEDIA_MAX_SDP_MEDIA: u32 = 16;
pub const PJMEDIA_VID_DEV_INFO_FMT_CNT: u32 = 64;
pub const PJMEDIA_VID_DEV_MAX_DRIVERS: u32 = 8;
pub const PJMEDIA_VID_DEV_MAX_DEVS: u32 = 16;
pub const PJMEDIA_VIDEODEV_ERRNO_START: u32 = 520000;
pub const PJMEDIA_VIDEODEV_ERRNO_END: u32 = 569999;
pub const PJMEDIA_EVID_ERR: u32 = 520001;
pub const PJMEDIA_EVID_SYSERR: u32 = 520002;
pub const PJMEDIA_EVID_INIT: u32 = 520003;
pub const PJMEDIA_EVID_INVDEV: u32 = 520004;
pub const PJMEDIA_EVID_NODEV: u32 = 520005;
pub const PJMEDIA_EVID_NODEFDEV: u32 = 520006;
pub const PJMEDIA_EVID_NOTREADY: u32 = 520007;
pub const PJMEDIA_EVID_INVCAP: u32 = 520008;
pub const PJMEDIA_EVID_INVOP: u32 = 520009;
pub const PJMEDIA_EVID_BADFORMAT: u32 = 520010;
pub const PJMEDIA_EVID_SAMPFORMAT: u32 = 520011;
pub const PJMEDIA_EVID_BADLATENCY: u32 = 520012;
pub const PJMEDIA_EVID_BADSIZE: u32 = 520013;
pub const PJMEDIA_CODEC_MGR_MAX_CODECS: u32 = 32;
pub const PJMEDIA_ECHO_STAT_NOT_SPECIFIED: u32 = 999999;
pub const PJMEDIA_ERRNO_START: u32 = 220000;
pub const PJMEDIA_ERRNO_END: u32 = 269999;
pub const PJMEDIA_PORTAUDIO_ERRNO_START: u32 = 259999;
pub const PJMEDIA_PORTAUDIO_ERRNO_END: u32 = 269998;
pub const PJMEDIA_LIBSRTP_ERRNO_START: u32 = 259799;
pub const PJMEDIA_LIBSRTP_ERRNO_END: u32 = 259998;
pub const PJMEDIA_ERROR: u32 = 220001;
pub const PJMEDIA_SDP_EINSDP: u32 = 220020;
pub const PJMEDIA_SDP_EINVER: u32 = 220021;
pub const PJMEDIA_SDP_EINORIGIN: u32 = 220022;
pub const PJMEDIA_SDP_EINTIME: u32 = 220023;
pub const PJMEDIA_SDP_EINNAME: u32 = 220024;
pub const PJMEDIA_SDP_EINCONN: u32 = 220025;
pub const PJMEDIA_SDP_EMISSINGCONN: u32 = 220026;
pub const PJMEDIA_SDP_EINATTR: u32 = 220027;
pub const PJMEDIA_SDP_EINRTPMAP: u32 = 220028;
pub const PJMEDIA_SDP_ERTPMAPTOOLONG: u32 = 220029;
pub const PJMEDIA_SDP_EMISSINGRTPMAP: u32 = 220030;
pub const PJMEDIA_SDP_EINMEDIA: u32 = 220031;
pub const PJMEDIA_SDP_ENOFMT: u32 = 220032;
pub const PJMEDIA_SDP_EINPT: u32 = 220033;
pub const PJMEDIA_SDP_EINFMTP: u32 = 220034;
pub const PJMEDIA_SDP_EINRTCP: u32 = 220035;
pub const PJMEDIA_SDP_EINPROTO: u32 = 220036;
pub const PJMEDIA_SDP_EINBANDW: u32 = 220037;
pub const PJMEDIA_SDP_EINSSRC: u32 = 220038;
pub const PJMEDIA_SDPNEG_EINSTATE: u32 = 220040;
pub const PJMEDIA_SDPNEG_ENOINITIAL: u32 = 220041;
pub const PJMEDIA_SDPNEG_ENOACTIVE: u32 = 220042;
pub const PJMEDIA_SDPNEG_ENONEG: u32 = 220043;
pub const PJMEDIA_SDPNEG_EMISMEDIA: u32 = 220044;
pub const PJMEDIA_SDPNEG_EINVANSMEDIA: u32 = 220045;
pub const PJMEDIA_SDPNEG_EINVANSTP: u32 = 220046;
pub const PJMEDIA_SDPNEG_EANSNOMEDIA: u32 = 220047;
pub const PJMEDIA_SDPNEG_ENOMEDIA: u32 = 220048;
pub const PJMEDIA_SDPNEG_NOANSCODEC: u32 = 220049;
pub const PJMEDIA_SDPNEG_NOANSTELEVENT: u32 = 220050;
pub const PJMEDIA_SDPNEG_NOANSUNKNOWN: u32 = 220051;
pub const PJMEDIA_SDP_EMEDIANOTEQUAL: u32 = 220060;
pub const PJMEDIA_SDP_EPORTNOTEQUAL: u32 = 220061;
pub const PJMEDIA_SDP_ETPORTNOTEQUAL: u32 = 220062;
pub const PJMEDIA_SDP_EFORMATNOTEQUAL: u32 = 220063;
pub const PJMEDIA_SDP_ECONNNOTEQUAL: u32 = 220064;
pub const PJMEDIA_SDP_EATTRNOTEQUAL: u32 = 220065;
pub const PJMEDIA_SDP_EDIRNOTEQUAL: u32 = 220066;
pub const PJMEDIA_SDP_EFMTPNOTEQUAL: u32 = 220067;
pub const PJMEDIA_SDP_ERTPMAPNOTEQUAL: u32 = 220068;
pub const PJMEDIA_SDP_ESESSNOTEQUAL: u32 = 220069;
pub const PJMEDIA_SDP_EORIGINNOTEQUAL: u32 = 220070;
pub const PJMEDIA_SDP_ENAMENOTEQUAL: u32 = 220071;
pub const PJMEDIA_SDP_ETIMENOTEQUAL: u32 = 220072;
pub const PJMEDIA_CODEC_EUNSUP: u32 = 220080;
pub const PJMEDIA_CODEC_EFAILED: u32 = 220081;
pub const PJMEDIA_CODEC_EFRMTOOSHORT: u32 = 220082;
pub const PJMEDIA_CODEC_EPCMTOOSHORT: u32 = 220083;
pub const PJMEDIA_CODEC_EFRMINLEN: u32 = 220084;
pub const PJMEDIA_CODEC_EPCMFRMINLEN: u32 = 220085;
pub const PJMEDIA_CODEC_EINMODE: u32 = 220086;
pub const PJMEDIA_CODEC_EBADBITSTREAM: u32 = 220087;
pub const PJMEDIA_EINVALIDIP: u32 = 220100;
pub const PJMEDIA_EASYMCODEC: u32 = 220101;
pub const PJMEDIA_EINVALIDPT: u32 = 220102;
pub const PJMEDIA_EMISSINGRTPMAP: u32 = 220103;
pub const PJMEDIA_EINVALIMEDIATYPE: u32 = 220104;
pub const PJMEDIA_EREMOTENODTMF: u32 = 220105;
pub const PJMEDIA_RTP_EINDTMF: u32 = 220106;
pub const PJMEDIA_RTP_EREMNORFC2833: u32 = 220107;
pub const PJMEDIA_EBADFMT: u32 = 220108;
pub const PJMEDIA_EUNSUPMEDIATYPE: u32 = 220109;
pub const PJMEDIA_RTP_EINPKT: u32 = 220120;
pub const PJMEDIA_RTP_EINPACK: u32 = 220121;
pub const PJMEDIA_RTP_EINVER: u32 = 220122;
pub const PJMEDIA_RTP_EINSSRC: u32 = 220123;
pub const PJMEDIA_RTP_EINPT: u32 = 220124;
pub const PJMEDIA_RTP_EINLEN: u32 = 220125;
pub const PJMEDIA_RTP_ESESSRESTART: u32 = 220130;
pub const PJMEDIA_RTP_ESESSPROBATION: u32 = 220131;
pub const PJMEDIA_RTP_EBADSEQ: u32 = 220132;
pub const PJMEDIA_RTP_EBADDEST: u32 = 220133;
pub const PJMEDIA_RTP_ENOCONFIG: u32 = 220134;
pub const PJMEDIA_ENOTCOMPATIBLE: u32 = 220160;
pub const PJMEDIA_ENCCLOCKRATE: u32 = 220161;
pub const PJMEDIA_ENCSAMPLESPFRAME: u32 = 220162;
pub const PJMEDIA_ENCTYPE: u32 = 220163;
pub const PJMEDIA_ENCBITS: u32 = 220164;
pub const PJMEDIA_ENCBYTES: u32 = 220165;
pub const PJMEDIA_ENCCHANNEL: u32 = 220166;
pub const PJMEDIA_ENOTVALIDWAVE: u32 = 220180;
pub const PJMEDIA_EWAVEUNSUPP: u32 = 220181;
pub const PJMEDIA_EWAVETOOSHORT: u32 = 220182;
pub const PJMEDIA_EFRMFILETOOBIG: u32 = 220183;
pub const PJMEDIA_EAVIUNSUPP: u32 = 220191;
pub const PJMEDIA_ENOSNDREC: u32 = 220200;
pub const PJMEDIA_ENOSNDPLAY: u32 = 220201;
pub const PJMEDIA_ESNDINDEVID: u32 = 220202;
pub const PJMEDIA_ESNDINSAMPLEFMT: u32 = 220203;
pub const PJMEDIA_SRTP_ECRYPTONOTMATCH: u32 = 220220;
pub const PJMEDIA_SRTP_EINKEYLEN: u32 = 220221;
pub const PJMEDIA_SRTP_ENOTSUPCRYPTO: u32 = 220222;
pub const PJMEDIA_SRTP_ESDPAMBIGUEANS: u32 = 220223;
pub const PJMEDIA_SRTP_ESDPDUPCRYPTOTAG: u32 = 220224;
pub const PJMEDIA_SRTP_ESDPINCRYPTO: u32 = 220225;
pub const PJMEDIA_SRTP_ESDPINCRYPTOTAG: u32 = 220226;
pub const PJMEDIA_SRTP_ESDPINTRANSPORT: u32 = 220227;
pub const PJMEDIA_SRTP_ESDPREQCRYPTO: u32 = 220228;
pub const PJMEDIA_SRTP_ESDPREQSECTP: u32 = 220229;
pub const PJMEDIA_SRTP_EKEYNOTREADY: u32 = 220230;
pub const PJMEDIA_SRTP_DTLS_ENOCRYPTO: u32 = 220240;
pub const PJMEDIA_SRTP_DTLS_EPEERNOCERT: u32 = 220241;
pub const PJMEDIA_SRTP_DTLS_EFPNOTMATCH: u32 = 220242;
pub const PJMEDIA_SRTP_DTLS_ENOFPRINT: u32 = 220243;
pub const PJMEDIA_SRTP_DTLS_ENOPROFILE: u32 = 220244;
pub const PJMEDIA_HAS_L16_CODEC: u32 = 1;
pub const PJMEDIA_CODEC_L16_HAS_8KHZ_MONO: u32 = 0;
pub const PJMEDIA_CODEC_L16_HAS_8KHZ_STEREO: u32 = 0;
pub const PJMEDIA_CODEC_L16_HAS_16KHZ_MONO: u32 = 0;
pub const PJMEDIA_CODEC_L16_HAS_16KHZ_STEREO: u32 = 0;
pub const PJMEDIA_CODEC_L16_HAS_48KHZ_MONO: u32 = 0;
pub const PJMEDIA_CODEC_L16_HAS_48KHZ_STEREO: u32 = 0;
pub const PJMEDIA_HAS_GSM_CODEC: u32 = 1;
pub const PJMEDIA_HAS_SPEEX_CODEC: u32 = 1;
pub const PJMEDIA_CODEC_SPEEX_DEFAULT_COMPLEXITY: u32 = 2;
pub const PJMEDIA_CODEC_SPEEX_DEFAULT_QUALITY: u32 = 8;
pub const PJMEDIA_HAS_ILBC_CODEC: u32 = 1;
pub const PJMEDIA_HAS_G722_CODEC: u32 = 1;
pub const PJMEDIA_POOL_LEN_G722_CODEC: u32 = 1000;
pub const PJMEDIA_POOL_INC_G722_CODEC: u32 = 1000;
pub const PJMEDIA_G722_DEFAULT_PCM_SHIFT: u32 = 2;
pub const PJMEDIA_G722_STOP_PCM_SHIFT_ON_CLIPPING: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP: u32 = 0;
pub const PJMEDIA_AUTO_LINK_IPP_LIBS: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_AMR: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_AMRWB: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_G729: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_G723_1: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_G726: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_G728: u32 = 1;
pub const PJMEDIA_HAS_INTEL_IPP_CODEC_G722_1: u32 = 1;
pub const PJMEDIA_HAS_PASSTHROUGH_CODECS: u32 = 0;
pub const PJMEDIA_HAS_PASSTHROUGH_CODEC_AMR: u32 = 1;
pub const PJMEDIA_HAS_PASSTHROUGH_CODEC_G729: u32 = 1;
pub const PJMEDIA_HAS_PASSTHROUGH_CODEC_ILBC: u32 = 1;
pub const PJMEDIA_HAS_PASSTHROUGH_CODEC_PCMU: u32 = 1;
pub const PJMEDIA_HAS_PASSTHROUGH_CODEC_PCMA: u32 = 1;
pub const PJMEDIA_HAS_G7221_CODEC: u32 = 0;
pub const PJMEDIA_HAS_OPENCORE_AMRNB_CODEC: u32 = 0;
pub const PJMEDIA_HAS_OPENCORE_AMRWB_CODEC: u32 = 0;
pub const PJMEDIA_AUTO_LINK_OPENCORE_AMR_LIBS: u32 = 1;
pub const PJMEDIA_OPENCORE_AMR_BUILT_WITH_GCC: u32 = 1;
pub const PJMEDIA_G7221_DEFAULT_PCM_SHIFT: u32 = 1;
pub const PJMEDIA_HAS_SILK_CODEC: u32 = 0;
pub const PJMEDIA_CODEC_SILK_DEFAULT_COMPLEXITY: u32 = 2;
pub const PJMEDIA_CODEC_SILK_DEFAULT_QUALITY: u32 = 10;
pub const PJMEDIA_HAS_OPUS_CODEC: u32 = 0;
pub const PJMEDIA_CODEC_OPUS_DEFAULT_SAMPLE_RATE: u32 = 48000;
pub const PJMEDIA_CODEC_OPUS_DEFAULT_BIT_RATE: u32 = 0;
pub const PJMEDIA_CODEC_OPUS_DEFAULT_COMPLEXITY: u32 = 5;
pub const PJMEDIA_HAS_BCG729: u32 = 0;
pub const PJMEDIA_HAS_FFMPEG_CODEC: u32 = 0;
pub const PJMEDIA_HAS_FFMPEG_VID_CODEC: u32 = 0;
pub const PJMEDIA_HAS_FFMPEG_CODEC_H263P: u32 = 0;
pub const PJMEDIA_HAS_FFMPEG_CODEC_H264: u32 = 0;
pub const PJMEDIA_HAS_VPX_CODEC_VP8: u32 = 1;
pub const PJMEDIA_HAS_VPX_CODEC_VP9: u32 = 0;
pub const PJMEDIA_JB_DEFAULT_INIT_DELAY: u32 = 15;
pub const PJMEDIA_VID_CODEC_MAX_DEC_FMT_CNT: u32 = 8;
pub const PJMEDIA_VID_CODEC_MAX_FPS_CNT: u32 = 16;
pub const PJMEDIA_VID_CODEC_MGR_MAX_CODECS: u32 = 32;

pub const pjmedia_type_PJMEDIA_TYPE_NONE: pjmedia_type = 0;
pub const pjmedia_type_PJMEDIA_TYPE_AUDIO: pjmedia_type = 1;
pub const pjmedia_type_PJMEDIA_TYPE_VIDEO: pjmedia_type = 2;
pub const pjmedia_type_PJMEDIA_TYPE_APPLICATION: pjmedia_type = 3;
pub const pjmedia_type_PJMEDIA_TYPE_UNKNOWN: pjmedia_type = 4;
pub type pjmedia_type = ::std::os::raw::c_uint;
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
pub type pjmedia_tp_proto = ::std::os::raw::c_uint;
pub const pjmedia_dir_PJMEDIA_DIR_NONE: pjmedia_dir = 0;
pub const pjmedia_dir_PJMEDIA_DIR_ENCODING: pjmedia_dir = 1;
pub const pjmedia_dir_PJMEDIA_DIR_CAPTURE: pjmedia_dir = 1;
pub const pjmedia_dir_PJMEDIA_DIR_DECODING: pjmedia_dir = 2;
pub const pjmedia_dir_PJMEDIA_DIR_PLAYBACK: pjmedia_dir = 2;
pub const pjmedia_dir_PJMEDIA_DIR_RENDER: pjmedia_dir = 2;
pub const pjmedia_dir_PJMEDIA_DIR_ENCODING_DECODING: pjmedia_dir = 3;
pub const pjmedia_dir_PJMEDIA_DIR_CAPTURE_PLAYBACK: pjmedia_dir = 3;
pub const pjmedia_dir_PJMEDIA_DIR_CAPTURE_RENDER: pjmedia_dir = 3;
pub type pjmedia_dir = ::std::os::raw::c_uint;



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

pub const pjmedia_coord_base_PJMEDIA_COORD_BASE_LEFT_TOP: pjmedia_coord_base = 0;
pub const pjmedia_coord_base_PJMEDIA_COORD_BASE_LEFT_BOTTOM: pjmedia_coord_base = 1;
pub type pjmedia_coord_base = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_ratio {
    pub num: ::std::os::raw::c_int,
    pub denum: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_coord {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rect_size {
    pub w: ::std::os::raw::c_uint,
    pub h: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rect {
    pub coord: pjmedia_coord,
    pub size: pjmedia_rect_size,
}

pub const pjmedia_orient_PJMEDIA_ORIENT_UNKNOWN: pjmedia_orient = 0;
pub const pjmedia_orient_PJMEDIA_ORIENT_NATURAL: pjmedia_orient = 1;
pub const pjmedia_orient_PJMEDIA_ORIENT_ROTATE_90DEG: pjmedia_orient = 2;
pub const pjmedia_orient_PJMEDIA_ORIENT_ROTATE_180DEG: pjmedia_orient = 3;
pub const pjmedia_orient_PJMEDIA_ORIENT_ROTATE_270DEG: pjmedia_orient = 4;
pub type pjmedia_orient = ::std::os::raw::c_uint;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_clock_src {
    pub media_type: pjmedia_type,
    pub clock_rate: ::std::os::raw::c_uint,
    pub ptime_usec: ::std::os::raw::c_uint,
    pub timestamp: pj_timestamp,
    pub last_update: pj_timestamp,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_clock {
    _unused: [u8; 0],
}

pub const pjmedia_clock_options_PJMEDIA_CLOCK_NO_ASYNC: pjmedia_clock_options = 1;
pub const pjmedia_clock_options_PJMEDIA_CLOCK_NO_HIGHEST_PRIO: pjmedia_clock_options = 2;
pub type pjmedia_clock_options = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_clock_param {
    pub usec_interval: ::std::os::raw::c_uint,
    pub clock_rate: ::std::os::raw::c_uint,
}
pub type pjmedia_clock_callback = ::std::option::Option<
    unsafe extern "C" fn(ts: *const pj_timestamp, user_data: *mut ::std::os::raw::c_void),
>;


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
pub type pjmedia_format_id = ::std::os::raw::c_uint;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_NONE: pjmedia_format_detail_type = 0;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_AUDIO: pjmedia_format_detail_type = 1;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_VIDEO: pjmedia_format_detail_type = 2;
pub const pjmedia_format_detail_type_PJMEDIA_FORMAT_DETAIL_MAX: pjmedia_format_detail_type = 3;
pub type pjmedia_format_detail_type = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_audio_format_detail {
    pub clock_rate: ::std::os::raw::c_uint,
    pub channel_count: ::std::os::raw::c_uint,
    pub frame_time_usec: ::std::os::raw::c_uint,
    pub bits_per_sample: ::std::os::raw::c_uint,
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
    pub user: [::std::os::raw::c_char; 1usize],
    _bindgen_union_align: [u32; 6usize],
}

pub const pjmedia_color_model_PJMEDIA_COLOR_MODEL_NONE: pjmedia_color_model = 0;
pub const pjmedia_color_model_PJMEDIA_COLOR_MODEL_RGB: pjmedia_color_model = 1;
pub const pjmedia_color_model_PJMEDIA_COLOR_MODEL_YUV: pjmedia_color_model = 2;
pub type pjmedia_color_model = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_video_apply_fmt_param {
    pub size: pjmedia_rect_size,
    pub buffer: *mut pj_uint8_t,
    pub framebytes: pj_size_t,
    pub strides: [::std::os::raw::c_int; 4usize],
    pub planes: [*mut pj_uint8_t; 4usize],
    pub plane_bytes: [pj_size_t; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_video_format_info {
    pub id: pj_uint32_t,
    pub name: [::std::os::raw::c_char; 8usize],
    pub color_model: pjmedia_color_model,
    pub bpp: pj_uint8_t,
    pub plane_cnt: pj_uint8_t,
    pub apply_fmt: ::std::option::Option<
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


pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_NONE: pjmedia_frame_type = 0;
pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_AUDIO: pjmedia_frame_type = 1;
pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_EXTENDED: pjmedia_frame_type = 2;
pub const pjmedia_frame_type_PJMEDIA_FRAME_TYPE_VIDEO: pjmedia_frame_type = 3;
pub type pjmedia_frame_type = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_frame {
    pub type_: pjmedia_frame_type,
    pub buf: *mut ::std::os::raw::c_void,
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
pub type pjmedia_aud_dev_factory_create_func_ptr = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut pj_pool_factory) -> *mut pjmedia_aud_dev_factory,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_aud_driver {
    pub create: pjmedia_aud_dev_factory_create_func_ptr,
    pub f: *mut pjmedia_aud_dev_factory,
    pub name: [::std::os::raw::c_char; 32usize],
    pub dev_cnt: ::std::os::raw::c_uint,
    pub start_idx: ::std::os::raw::c_uint,
    pub rec_dev_idx: ::std::os::raw::c_int,
    pub play_dev_idx: ::std::os::raw::c_int,
    pub dev_idx: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_aud_subsys {
    pub init_count: ::std::os::raw::c_uint,
    pub pf: *mut pj_pool_factory,
    pub drv_cnt: ::std::os::raw::c_uint,
    pub drv: [pjmedia_aud_driver; 16usize],
    pub dev_cnt: ::std::os::raw::c_uint,
    pub dev_list: [pj_uint32_t; 64usize],
}

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
pub type pjmedia_aud_dev_cap = ::std::os::raw::c_uint;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_DEFAULT: pjmedia_aud_dev_route = 0;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_LOUDSPEAKER: pjmedia_aud_dev_route = 1;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_EARPIECE: pjmedia_aud_dev_route = 2;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_BLUETOOTH: pjmedia_aud_dev_route = 4;
pub const pjmedia_aud_dev_route_PJMEDIA_AUD_DEV_ROUTE_CUSTOM: pjmedia_aud_dev_route = 128;
pub type pjmedia_aud_dev_route = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_aud_dev_info {
    pub name: [::std::os::raw::c_char; 64usize],
    pub input_count: ::std::os::raw::c_uint,
    pub output_count: ::std::os::raw::c_uint,
    pub default_samples_per_sec: ::std::os::raw::c_uint,
    pub driver: [::std::os::raw::c_char; 32usize],
    pub caps: ::std::os::raw::c_uint,
    pub routes: ::std::os::raw::c_uint,
    pub ext_fmt_cnt: ::std::os::raw::c_uint,
    pub ext_fmt: [pjmedia_format; 8usize],
}
pub type pjmedia_aud_play_cb = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        frame: *mut pjmedia_frame,
    ) -> pj_status_t,
>;
pub type pjmedia_aud_rec_cb = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        frame: *mut pjmedia_frame,
    ) -> pj_status_t,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_aud_param {
    pub dir: pjmedia_dir,
    pub rec_id: pjmedia_aud_dev_index,
    pub play_id: pjmedia_aud_dev_index,
    pub clock_rate: ::std::os::raw::c_uint,
    pub channel_count: ::std::os::raw::c_uint,
    pub samples_per_frame: ::std::os::raw::c_uint,
    pub bits_per_sample: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub ext_fmt: pjmedia_format,
    pub input_latency_ms: ::std::os::raw::c_uint,
    pub output_latency_ms: ::std::os::raw::c_uint,
    pub input_vol: ::std::os::raw::c_uint,
    pub output_vol: ::std::os::raw::c_uint,
    pub input_route: pjmedia_aud_dev_route,
    pub output_route: pjmedia_aud_dev_route,
    pub ec_enabled: pj_bool_t,
    pub ec_tail_ms: ::std::os::raw::c_uint,
    pub plc_enabled: pj_bool_t,
    pub cng_enabled: pj_bool_t,
    pub vad_enabled: pj_bool_t,
}



pub type pj_highprec_t = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pj_math_stat {
    pub n: ::std::os::raw::c_int,
    pub max: ::std::os::raw::c_int,
    pub min: ::std::os::raw::c_int,
    pub last: ::std::os::raw::c_int,
    pub mean: ::std::os::raw::c_int,
    pub fmean_: f32,
    pub m2_: pj_highprec_t,
}
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_LOSS_RLE: pjmedia_rtcp_xr_type = 1;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_DUP_RLE: pjmedia_rtcp_xr_type = 2;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_RCPT_TIMES: pjmedia_rtcp_xr_type = 4;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_RR_TIME: pjmedia_rtcp_xr_type = 8;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_DLRR: pjmedia_rtcp_xr_type = 16;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_STATS: pjmedia_rtcp_xr_type = 32;
pub const pjmedia_rtcp_xr_type_PJMEDIA_RTCP_XR_VOIP_METRICS: pjmedia_rtcp_xr_type = 64;
pub type pjmedia_rtcp_xr_type = ::std::os::raw::c_uint;
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
pub type pjmedia_rtcp_xr_info = ::std::os::raw::c_uint;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_UNK: pjmedia_rtcp_xr_plc_type = 0;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_DIS: pjmedia_rtcp_xr_plc_type = 1;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_ENH: pjmedia_rtcp_xr_plc_type = 2;
pub const pjmedia_rtcp_xr_plc_type_PJMEDIA_RTCP_XR_PLC_STD: pjmedia_rtcp_xr_plc_type = 3;
pub type pjmedia_rtcp_xr_plc_type = ::std::os::raw::c_uint;
pub const pjmedia_rtcp_xr_jb_type_PJMEDIA_RTCP_XR_JB_UNKNOWN: pjmedia_rtcp_xr_jb_type = 0;
pub const pjmedia_rtcp_xr_jb_type_PJMEDIA_RTCP_XR_JB_FIXED: pjmedia_rtcp_xr_jb_type = 2;
pub const pjmedia_rtcp_xr_jb_type_PJMEDIA_RTCP_XR_JB_ADAPTIVE: pjmedia_rtcp_xr_jb_type = 3;
pub type pjmedia_rtcp_xr_jb_type = ::std::os::raw::c_uint;

pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_ACK: pjmedia_rtcp_fb_type = 0;
pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_NACK: pjmedia_rtcp_fb_type = 1;
pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_TRR_INT: pjmedia_rtcp_fb_type = 2;
pub const pjmedia_rtcp_fb_type_PJMEDIA_RTCP_FB_OTHER: pjmedia_rtcp_fb_type = 3;
pub type pjmedia_rtcp_fb_type = ::std::os::raw::c_uint;

pub type pjmedia_obj_sig = pj_uint32_t;

pub type pjmedia_vid_dev_index = pj_int32_t;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_NONE: pjmedia_vid_dev_hwnd_type = 0;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_WINDOWS: pjmedia_vid_dev_hwnd_type = 1;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_IOS: pjmedia_vid_dev_hwnd_type = 2;
pub const pjmedia_vid_dev_hwnd_type_PJMEDIA_VID_DEV_HWND_TYPE_ANDROID: pjmedia_vid_dev_hwnd_type = 3;
pub type pjmedia_vid_dev_hwnd_type = ::std::os::raw::c_uint;

pub const pjmedia_vid_dev_wnd_flag_PJMEDIA_VID_DEV_WND_BORDER: pjmedia_vid_dev_wnd_flag = 1;
pub const pjmedia_vid_dev_wnd_flag_PJMEDIA_VID_DEV_WND_RESIZABLE: pjmedia_vid_dev_wnd_flag = 2;
pub type pjmedia_vid_dev_wnd_flag = ::std::os::raw::c_uint;
pub const pjmedia_vid_dev_std_index_PJMEDIA_VID_DEFAULT_CAPTURE_DEV: pjmedia_vid_dev_std_index = -1;
pub const pjmedia_vid_dev_std_index_PJMEDIA_VID_DEFAULT_RENDER_DEV: pjmedia_vid_dev_std_index = -2;
pub const pjmedia_vid_dev_std_index_PJMEDIA_VID_INVALID_DEV: pjmedia_vid_dev_std_index = -3;
pub type pjmedia_vid_dev_std_index = ::std::os::raw::c_int;
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
pub type pjmedia_vid_dev_cap = ::std::os::raw::c_uint;

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

pub type pjmedia_event_type = ::std::os::raw::c_uint;
pub const pjmedia_event_publish_flag_PJMEDIA_EVENT_PUBLISH_DEFAULT: pjmedia_event_publish_flag = 0;
pub const pjmedia_event_publish_flag_PJMEDIA_EVENT_PUBLISH_POST_EVENT: pjmedia_event_publish_flag = 1;
pub type pjmedia_event_publish_flag = ::std::os::raw::c_uint;
pub const pjmedia_event_mgr_flag_PJMEDIA_EVENT_MGR_NO_THREAD: pjmedia_event_mgr_flag = 1;
pub type pjmedia_event_mgr_flag = ::std::os::raw::c_uint;


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
    pub fn count(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_count(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn p(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_p(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn version(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_version(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn pt(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_pt(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn length(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_length(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        count: ::std::os::raw::c_uint,
        p: ::std::os::raw::c_uint,
        version: ::std::os::raw::c_uint,
        pt: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
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
    pub count: ::std::os::raw::c_uint,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub lost: ::std::os::raw::c_uint,
    pub dup: ::std::os::raw::c_uint,
    pub jitter: pj_math_stat,
    pub toh: pj_math_stat,
}

impl pjmedia_rtcp_xr_stream_stat__bindgen_ty_1 {
    #[inline]
    pub fn l(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_l(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn j(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_j(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn t(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_t(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        l: ::std::os::raw::c_uint,
        d: ::std::os::raw::c_uint,
        j: ::std::os::raw::c_uint,
        t: ::std::os::raw::c_uint,
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
    pub name: *mut ::std::os::raw::c_char,
    pub pkt: pjmedia_rtcp_xr_pkt,
    pub rx_lrr: pj_uint32_t,
    pub rx_lrr_time: pj_timestamp,
    pub rx_last_rr: pj_uint32_t,
    pub stat: pjmedia_rtcp_xr_stat,
    pub src_ref_seq: pj_uint32_t,
    pub uninitialized_src_ref_seq: pj_bool_t,
    pub voip_mtc_stat: pjmedia_rtcp_xr_session__bindgen_ty_1,
    pub ptime: ::std::os::raw::c_uint,
    pub frames_per_packet: ::std::os::raw::c_uint,
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
    pub ext_len: ::std::os::raw::c_uint,
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
    pub fn bad(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bad(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn badpt(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_badpt(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn badssrc(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_badssrc(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn dup(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_dup(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outorder(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outorder(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn probation(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_probation(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn restart(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_restart(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        bad: ::std::os::raw::c_int,
        badpt: ::std::os::raw::c_int,
        badssrc: ::std::os::raw::c_int,
        dup: ::std::os::raw::c_int,
        outorder: ::std::os::raw::c_int,
        probation: ::std::os::raw::c_int,
        restart: ::std::os::raw::c_int,
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
    pub default_pt: ::std::os::raw::c_int,
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
    pub fn count(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_count(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn p(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_p(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn version(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_version(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn pt(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_pt(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn length(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_length(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        count: ::std::os::raw::c_uint,
        p: ::std::os::raw::c_uint,
        version: ::std::os::raw::c_uint,
        pt: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
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
    pub update_cnt: ::std::os::raw::c_uint,
    pub pkt: pj_uint32_t,
    pub bytes: pj_uint32_t,
    pub discard: ::std::os::raw::c_uint,
    pub loss: ::std::os::raw::c_uint,
    pub reorder: ::std::os::raw::c_uint,
    pub dup: ::std::os::raw::c_uint,
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
    pub fn burst(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_burst(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn random(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_random(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        burst: ::std::os::raw::c_uint,
        random: ::std::os::raw::c_uint,
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
    pub peer_sdes_buf_: [::std::os::raw::c_char; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_rtcp_session {
    pub name: *mut ::std::os::raw::c_char,
    pub rtcp_sr_pkt: pjmedia_rtcp_sr_pkt,
    pub rtcp_rr_pkt: pjmedia_rtcp_rr_pkt,
    pub seq_ctrl: pjmedia_rtp_seq_session,
    pub rtp_last_ts: ::std::os::raw::c_uint,
    pub clock_rate: ::std::os::raw::c_uint,
    pub pkt_size: ::std::os::raw::c_uint,
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
    pub name: *mut ::std::os::raw::c_char,
    pub clock_rate: ::std::os::raw::c_uint,
    pub samples_per_frame: ::std::os::raw::c_uint,
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
    pub clock_rate: ::std::os::raw::c_uint,
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
    pub port: ::std::os::raw::c_uint,
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
    pub bandw_count: ::std::os::raw::c_uint,
    pub bandw: [*mut pjmedia_sdp_bandw; 4usize],
    pub attr_count: ::std::os::raw::c_uint,
    pub attr: [*mut pjmedia_sdp_attr; 68usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_sdp_media__bindgen_ty_1 {
    pub media: pj_str_t,
    pub port: pj_uint16_t,
    pub port_count: ::std::os::raw::c_uint,
    pub transport: pj_str_t,
    pub fmt_count: ::std::os::raw::c_uint,
    pub fmt: [pj_str_t; 32usize],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_sdp_session {
    pub origin: pjmedia_sdp_session__bindgen_ty_1,
    pub name: pj_str_t,
    pub conn: *mut pjmedia_sdp_conn,
    pub bandw_count: ::std::os::raw::c_uint,
    pub bandw: [*mut pjmedia_sdp_bandw; 4usize],
    pub time: pjmedia_sdp_session__bindgen_ty_2,
    pub attr_count: ::std::os::raw::c_uint,
    pub attr: [*mut pjmedia_sdp_attr; 68usize],
    pub media_count: ::std::os::raw::c_uint,
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
    pub cap_count: ::std::os::raw::c_uint,
    pub caps: [pjmedia_rtcp_fb_cap; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_rtcp_fb_setting {
    pub dont_use_avpf: pj_bool_t,
    pub cap_count: ::std::os::raw::c_uint,
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
    pub window: *mut ::std::os::raw::c_void,
    _bindgen_union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_1 {
    pub hwnd: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_2 {
    pub window: *mut ::std::os::raw::c_void,
    pub display: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_3 {
    pub window: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_4 {
    pub window: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_5 {
    pub window: *mut ::std::os::raw::c_void,
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
    pub name: [::std::os::raw::c_char; 64usize],
    pub driver: [::std::os::raw::c_char; 32usize],
    pub dir: pjmedia_dir,
    pub has_callback: pj_bool_t,
    pub caps: ::std::os::raw::c_uint,
    pub fmt_cnt: ::std::os::raw::c_uint,
    pub fmt: [pjmedia_format; 64usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_dev_cb {
    pub capture_cb: ::std::option::Option<
        unsafe extern "C" fn(
            stream: *mut pjmedia_vid_dev_stream,
            user_data: *mut ::std::os::raw::c_void,
            frame: *mut pjmedia_frame,
        ) -> pj_status_t,
    >,
    pub render_cb: ::std::option::Option<
        unsafe extern "C" fn(
            stream: *mut pjmedia_vid_dev_stream,
            user_data: *mut ::std::os::raw::c_void,
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
    pub clock_rate: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub fmt: pjmedia_format,
    pub window: pjmedia_vid_dev_hwnd,
    pub disp_size: pjmedia_rect_size,
    pub window_pos: pjmedia_coord,
    pub window_hide: pj_bool_t,
    pub native_preview: pj_bool_t,
    pub orient: pjmedia_orient,
    pub window_flags: ::std::os::raw::c_uint,
    pub window_fullscreen: pj_bool_t,
}
pub type pjmedia_vid_dev_factory_create_func_ptr = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut pj_pool_factory) -> *mut pjmedia_vid_dev_factory,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_driver {
    pub create: pjmedia_vid_dev_factory_create_func_ptr,
    pub f: *mut pjmedia_vid_dev_factory,
    pub name: [::std::os::raw::c_char; 32usize],
    pub dev_cnt: ::std::os::raw::c_uint,
    pub start_idx: ::std::os::raw::c_uint,
    pub cap_dev_idx: ::std::os::raw::c_int,
    pub rend_dev_idx: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_vid_subsys {
    pub init_count: ::std::os::raw::c_uint,
    pub pf: *mut pj_pool_factory,
    pub drv_cnt: ::std::os::raw::c_uint,
    pub drv: [pjmedia_vid_driver; 8usize],
    pub dev_cnt: ::std::os::raw::c_uint,
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
    pub dummy: ::std::os::raw::c_int,
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
pub type pjmedia_event_user_data = [::std::os::raw::c_char; 40usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjmedia_event {
    pub type_: pjmedia_event_type,
    pub timestamp: pj_timestamp,
    pub src: *const ::std::os::raw::c_void,
    pub epub: *const ::std::os::raw::c_void,
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
    pub ptr: *mut ::std::os::raw::c_void,
    _bindgen_union_align: [u64; 11usize],
}
pub type pjmedia_event_cb = ::std::option::Option<
    unsafe extern "C" fn(
        event: *mut pjmedia_event,
        user_data: *mut ::std::os::raw::c_void,
    ) -> pj_status_t,
>;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjmedia_event_mgr {
    _unused: [u8; 0],
}

extern "C" {
    pub fn pjmedia_type_name(t: pjmedia_type) -> *const ::std::os::raw::c_char;
    pub fn pjmedia_get_type(name: *const pj_str_t) -> pjmedia_type;
    pub static pjmedia_linear2ulaw_tab: [pj_uint8_t; 16384usize];
    pub static pjmedia_linear2alaw_tab: [pj_uint8_t; 16384usize];
    pub static pjmedia_ulaw2linear_tab: [pj_int16_t; 256usize];
    pub static pjmedia_alaw2linear_tab: [pj_int16_t; 256usize];
    pub fn pjmedia_clock_src_init( clocksrc: *mut pjmedia_clock_src, media_type: pjmedia_type, clock_rate: ::std::os::raw::c_uint, ptime_usec: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pjmedia_clock_src_update( clocksrc: *mut pjmedia_clock_src, timestamp: *const pj_timestamp, ) -> pj_status_t;
    pub fn pjmedia_clock_src_get_current_timestamp( clocksrc: *const pjmedia_clock_src, timestamp: *mut pj_timestamp, ) -> pj_status_t;
    pub fn pjmedia_clock_src_get_time_msec(clocksrc: *const pjmedia_clock_src) -> pj_uint32_t;
    pub fn pjmedia_clock_create( pool: *mut pj_pool_t, clock_rate: ::std::os::raw::c_uint, channel_count: ::std::os::raw::c_uint, samples_per_frame: ::std::os::raw::c_uint, options: ::std::os::raw::c_uint, cb: pjmedia_clock_callback, user_data: *mut ::std::os::raw::c_void, p_clock: *mut *mut pjmedia_clock, ) -> pj_status_t;
    pub fn pjmedia_clock_create2( pool: *mut pj_pool_t, param: *const pjmedia_clock_param, options: ::std::os::raw::c_uint, cb: pjmedia_clock_callback, user_data: *mut ::std::os::raw::c_void, p_clock: *mut *mut pjmedia_clock, ) -> pj_status_t;
    pub fn pjmedia_clock_start(clock: *mut pjmedia_clock) -> pj_status_t;
    pub fn pjmedia_clock_stop(clock: *mut pjmedia_clock) -> pj_status_t;
    pub fn pjmedia_clock_modify( clock: *mut pjmedia_clock, param: *const pjmedia_clock_param, ) -> pj_status_t;
    pub fn pjmedia_clock_wait( clock: *mut pjmedia_clock, wait: pj_bool_t, ts: *mut pj_timestamp, ) -> pj_bool_t;
    pub fn pjmedia_clock_destroy(clock: *mut pjmedia_clock) -> pj_status_t;
    pub fn pjmedia_audiodev_strerror( status: pj_status_t, buffer: *mut ::std::os::raw::c_char, bufsize: pj_size_t, ) -> pj_str_t;
    pub fn pjmedia_format_init_video( fmt: *mut pjmedia_format, fmt_id: pj_uint32_t, width: ::std::os::raw::c_uint, height: ::std::os::raw::c_uint, fps_num: ::std::os::raw::c_uint, fps_denum: ::std::os::raw::c_uint, );
    pub fn pjmedia_format_copy( dst: *mut pjmedia_format, src: *const pjmedia_format, ) -> *mut pjmedia_format;
    pub fn pjmedia_format_get_audio_format_detail( fmt: *const pjmedia_format, assert_valid: pj_bool_t, ) -> *mut pjmedia_audio_format_detail;
    pub fn pjmedia_format_get_video_format_detail( fmt: *const pjmedia_format, assert_valid: pj_bool_t, ) -> *mut pjmedia_video_format_detail;
    pub fn pjmedia_video_format_mgr_create( pool: *mut pj_pool_t, max_fmt: ::std::os::raw::c_uint, options: ::std::os::raw::c_uint, p_mgr: *mut *mut pjmedia_video_format_mgr,) -> pj_status_t;
    pub fn pjmedia_video_format_mgr_instance() -> *mut pjmedia_video_format_mgr;
    pub fn pjmedia_video_format_mgr_set_instance(mgr: *mut pjmedia_video_format_mgr);
    pub fn pjmedia_get_video_format_info( mgr: *mut pjmedia_video_format_mgr, id: pj_uint32_t, ) -> *const pjmedia_video_format_info;
    pub fn pjmedia_register_video_format_info( mgr: *mut pjmedia_video_format_mgr, vfi: *mut pjmedia_video_format_info, ) -> pj_status_t;
    pub fn pjmedia_video_format_mgr_destroy(mgr: *mut pjmedia_video_format_mgr);
    pub fn pjmedia_get_aud_subsys() -> *mut pjmedia_aud_subsys;
    pub fn pjmedia_aud_driver_init( drv_idx: ::std::os::raw::c_uint, refresh: pj_bool_t, ) -> pj_status_t;
    pub fn pjmedia_aud_driver_deinit(drv_idx: ::std::os::raw::c_uint);
    pub fn pjmedia_aud_dev_cap_name( cap: pjmedia_aud_dev_cap, p_desc: *mut *const ::std::os::raw::c_char, ) -> *const ::std::os::raw::c_char;
    pub fn pjmedia_aud_param_set_cap( param: *mut pjmedia_aud_param, cap: pjmedia_aud_dev_cap, pval: *const ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_param_get_cap( param: *const pjmedia_aud_param, cap: pjmedia_aud_dev_cap, pval: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_dev_refresh() -> pj_status_t;
    pub fn pjmedia_aud_dev_count() -> ::std::os::raw::c_uint;
    pub fn pjmedia_aud_dev_get_info( id: pjmedia_aud_dev_index, info: *mut pjmedia_aud_dev_info, ) -> pj_status_t;
    pub fn pjmedia_aud_dev_lookup( drv_name: *const ::std::os::raw::c_char, dev_name: *const ::std::os::raw::c_char, id: *mut pjmedia_aud_dev_index, ) -> pj_status_t;
    pub fn pjmedia_aud_dev_default_param( id: pjmedia_aud_dev_index, param: *mut pjmedia_aud_param, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_create( param: *const pjmedia_aud_param, rec_cb: pjmedia_aud_rec_cb, play_cb: pjmedia_aud_play_cb, user_data: *mut ::std::os::raw::c_void, p_strm: *mut *mut pjmedia_aud_stream, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_get_param( strm: *mut pjmedia_aud_stream, param: *mut pjmedia_aud_param, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_get_cap(strm: *mut pjmedia_aud_stream,cap: pjmedia_aud_dev_cap,value: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_set_cap( strm: *mut pjmedia_aud_stream, cap: pjmedia_aud_dev_cap, value: *const ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_aud_stream_start(strm: *mut pjmedia_aud_stream) -> pj_status_t;
    pub fn pjmedia_aud_stream_stop(strm: *mut pjmedia_aud_stream) -> pj_status_t;
    pub fn pjmedia_aud_stream_destroy(strm: *mut pjmedia_aud_stream) -> pj_status_t;
    pub fn pjmedia_rtcp_build_rtcp_xr( session: *mut pjmedia_rtcp_xr_session, rpt_types: ::std::os::raw::c_uint, rtcp_pkt: *mut *mut ::std::os::raw::c_void, len: *mut ::std::os::raw::c_int, );
    pub fn pjmedia_rtcp_xr_update_info( session: *mut pjmedia_rtcp_xr_session, info: ::std::os::raw::c_uint, val: pj_int32_t, ) -> pj_status_t;
    pub fn pjmedia_rtcp_xr_init( session: *mut pjmedia_rtcp_xr_session, r_session: *mut pjmedia_rtcp_session, gmin: pj_uint8_t, frames_per_packet: ::std::os::raw::c_uint, );
    pub fn pjmedia_rtcp_xr_fini(session: *mut pjmedia_rtcp_xr_session);
    pub fn pjmedia_rtcp_xr_rx_rtcp_xr( session: *mut pjmedia_rtcp_xr_session, rtcp_pkt: *const ::std::os::raw::c_void, size: pj_size_t, );
    pub fn pjmedia_rtcp_xr_rx_rtp( session: *mut pjmedia_rtcp_xr_session, seq: ::std::os::raw::c_uint, lost: ::std::os::raw::c_int, dup: ::std::os::raw::c_int, discarded: ::std::os::raw::c_int, jitter: ::std::os::raw::c_int, toh: ::std::os::raw::c_int, toh_ipv4: pj_bool_t, );
    pub fn pjmedia_rtcp_xr_tx_rtp( session: *mut pjmedia_rtcp_xr_session, ptsize: ::std::os::raw::c_uint, );
    pub fn pjmedia_rtp_session_init( ses: *mut pjmedia_rtp_session, default_pt: ::std::os::raw::c_int, sender_ssrc: pj_uint32_t, ) -> pj_status_t;
    pub fn pjmedia_rtp_session_init2( ses: *mut pjmedia_rtp_session, settings: pjmedia_rtp_session_setting, ) -> pj_status_t;
    pub fn pjmedia_rtp_encode_rtp( ses: *mut pjmedia_rtp_session, pt: ::std::os::raw::c_int, m: ::std::os::raw::c_int, payload_len: ::std::os::raw::c_int, ts_len: ::std::os::raw::c_int, rtphdr: *mut *const ::std::os::raw::c_void, hdrlen: *mut ::std::os::raw::c_int, ) -> pj_status_t;
    pub fn pjmedia_rtp_decode_rtp( ses: *mut pjmedia_rtp_session, pkt: *const ::std::os::raw::c_void, pkt_len: ::std::os::raw::c_int, hdr: *mut *const pjmedia_rtp_hdr, payload: *mut *const ::std::os::raw::c_void, payloadlen: *mut ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pjmedia_rtp_decode_rtp2( ses: *mut pjmedia_rtp_session, pkt: *const ::std::os::raw::c_void, pkt_len: ::std::os::raw::c_int, hdr: *mut *const pjmedia_rtp_hdr, dec_hdr: *mut pjmedia_rtp_dec_hdr, payload: *mut *const ::std::os::raw::c_void, payloadlen: *mut ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pjmedia_rtp_session_update( ses: *mut pjmedia_rtp_session, hdr: *const pjmedia_rtp_hdr, seq_st: *mut pjmedia_rtp_status, );
    pub fn pjmedia_rtp_session_update2( ses: *mut pjmedia_rtp_session, hdr: *const pjmedia_rtp_hdr, seq_st: *mut pjmedia_rtp_status, check_pt: pj_bool_t, );
    pub fn pjmedia_rtp_seq_init(seq_ctrl: *mut pjmedia_rtp_seq_session, seq: pj_uint16_t);
    pub fn pjmedia_rtp_seq_update( seq_ctrl: *mut pjmedia_rtp_seq_session, seq: pj_uint16_t, seq_status: *mut pjmedia_rtp_status, );
    pub fn pjmedia_rtcp_session_setting_default(settings: *mut pjmedia_rtcp_session_setting);
    pub fn pjmedia_rtcp_init_stat(stat: *mut pjmedia_rtcp_stat);
    pub fn pjmedia_rtcp_init( session: *mut pjmedia_rtcp_session, name: *mut ::std::os::raw::c_char, clock_rate: ::std::os::raw::c_uint, samples_per_frame: ::std::os::raw::c_uint, ssrc: pj_uint32_t, );
    pub fn pjmedia_rtcp_init2( session: *mut pjmedia_rtcp_session, settings: *const pjmedia_rtcp_session_setting, );
    pub fn pjmedia_rtcp_get_ntp_time( sess: *const pjmedia_rtcp_session, ntp: *mut pjmedia_rtcp_ntp_rec, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fini(session: *mut pjmedia_rtcp_session);
    pub fn pjmedia_rtcp_rx_rtp( session: *mut pjmedia_rtcp_session, seq: ::std::os::raw::c_uint, ts: ::std::os::raw::c_uint, payload: ::std::os::raw::c_uint, );
    pub fn pjmedia_rtcp_rx_rtp2( session: *mut pjmedia_rtcp_session, seq: ::std::os::raw::c_uint, ts: ::std::os::raw::c_uint, payload: ::std::os::raw::c_uint, discarded: pj_bool_t, );
    pub fn pjmedia_rtcp_tx_rtp(session: *mut pjmedia_rtcp_session, ptsize: ::std::os::raw::c_uint);
    pub fn pjmedia_rtcp_rx_rtcp( session: *mut pjmedia_rtcp_session, rtcp_pkt: *const ::std::os::raw::c_void, size: pj_size_t, );
    pub fn pjmedia_rtcp_build_rtcp( session: *mut pjmedia_rtcp_session, rtcp_pkt: *mut *mut ::std::os::raw::c_void, len: *mut ::std::os::raw::c_int, );
    pub fn pjmedia_rtcp_build_rtcp_sdes( session: *mut pjmedia_rtcp_session, buf: *mut ::std::os::raw::c_void, length: *mut pj_size_t, sdes: *const pjmedia_rtcp_sdes, ) -> pj_status_t;
    pub fn pjmedia_rtcp_build_rtcp_bye( session: *mut pjmedia_rtcp_session, buf: *mut ::std::os::raw::c_void, length: *mut pj_size_t, reason: *const pj_str_t, ) -> pj_status_t;
    pub fn pjmedia_rtcp_enable_xr( session: *mut pjmedia_rtcp_session, enable: pj_bool_t, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_create( pool: *mut pj_pool_t, name: *const ::std::os::raw::c_char, value: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_attr_clone( pool: *mut pj_pool_t, attr: *const pjmedia_sdp_attr, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_attr_find( count: ::std::os::raw::c_uint, attr_array: *const *mut pjmedia_sdp_attr, name: *const pj_str_t, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_attr_find2( count: ::std::os::raw::c_uint, attr_array: *const *mut pjmedia_sdp_attr, name: *const ::std::os::raw::c_char, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_attr_add( count: *mut ::std::os::raw::c_uint, attr_array: *mut *mut pjmedia_sdp_attr, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_remove_all( count: *mut ::std::os::raw::c_uint, attr_array: *mut *mut pjmedia_sdp_attr, name: *const ::std::os::raw::c_char, ) -> ::std::os::raw::c_uint;
    pub fn pjmedia_sdp_attr_remove( count: *mut ::std::os::raw::c_uint, attr_array: *mut *mut pjmedia_sdp_attr, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_to_rtpmap( pool: *mut pj_pool_t, attr: *const pjmedia_sdp_attr, p_rtpmap: *mut *mut pjmedia_sdp_rtpmap, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_get_rtpmap( attr: *const pjmedia_sdp_attr, rtpmap: *mut pjmedia_sdp_rtpmap, ) -> pj_status_t;
    pub fn pjmedia_sdp_rtpmap_to_attr( pool: *mut pj_pool_t, rtpmap: *const pjmedia_sdp_rtpmap, p_attr: *mut *mut pjmedia_sdp_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_get_fmtp( attr: *const pjmedia_sdp_attr, fmtp: *mut pjmedia_sdp_fmtp, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_get_rtcp( attr: *const pjmedia_sdp_attr, rtcp: *mut pjmedia_sdp_rtcp_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_create_rtcp( pool: *mut pj_pool_t, a: *const pj_sockaddr, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_attr_get_ssrc( attr: *const pjmedia_sdp_attr, rtcp: *mut pjmedia_sdp_ssrc_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_attr_create_ssrc( pool: *mut pj_pool_t, ssrc: pj_uint32_t, cname: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_conn_clone( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_conn, ) -> *mut pjmedia_sdp_conn;
    pub fn pjmedia_sdp_conn_cmp( conn1: *const pjmedia_sdp_conn, conn2: *const pjmedia_sdp_conn, option: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pjmedia_sdp_bandw_clone( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_bandw, ) -> *mut pjmedia_sdp_bandw;
    pub fn pjmedia_sdp_media_clone( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_media, ) -> *mut pjmedia_sdp_media;
    pub fn pjmedia_sdp_media_find_attr( m: *const pjmedia_sdp_media, name: *const pj_str_t, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_media_find_attr2( m: *const pjmedia_sdp_media, name: *const ::std::os::raw::c_char, fmt: *const pj_str_t, ) -> *mut pjmedia_sdp_attr;
    pub fn pjmedia_sdp_media_add_attr( m: *mut pjmedia_sdp_media, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_media_remove_all_attr( m: *mut pjmedia_sdp_media, name: *const ::std::os::raw::c_char, ) -> ::std::os::raw::c_uint;
    pub fn pjmedia_sdp_media_remove_attr( m: *mut pjmedia_sdp_media, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    pub fn pjmedia_sdp_media_cmp( sd1: *const pjmedia_sdp_media, sd2: *const pjmedia_sdp_media, option: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pjmedia_sdp_transport_cmp(t1: *const pj_str_t, t2: *const pj_str_t) -> pj_status_t;
    pub fn pjmedia_sdp_transport_get_proto(tp: *const pj_str_t) -> pj_uint32_t;
    pub fn pjmedia_sdp_media_deactivate( pool: *mut pj_pool_t, m: *mut pjmedia_sdp_media, ) -> pj_status_t;
    pub fn pjmedia_sdp_media_clone_deactivate( pool: *mut pj_pool_t, rhs: *const pjmedia_sdp_media, ) -> *mut pjmedia_sdp_media;
    pub fn pjmedia_sdp_parse( pool: *mut pj_pool_t, buf: *mut ::std::os::raw::c_char, len: pj_size_t, p_sdp: *mut *mut pjmedia_sdp_session, ) -> pj_status_t;
    pub fn pjmedia_sdp_print( sdp: *const pjmedia_sdp_session, buf: *mut ::std::os::raw::c_char, size: pj_size_t, ) -> ::std::os::raw::c_int;
    pub fn pjmedia_sdp_validate(sdp: *const pjmedia_sdp_session) -> pj_status_t;
    pub fn pjmedia_sdp_validate2(sdp: *const pjmedia_sdp_session, strict: pj_bool_t) -> pj_status_t;
    pub fn pjmedia_sdp_session_clone( pool: *mut pj_pool_t, sdp: *const pjmedia_sdp_session, ) -> *mut pjmedia_sdp_session;
    pub fn pjmedia_sdp_session_cmp( sd1: *const pjmedia_sdp_session, sd2: *const pjmedia_sdp_session, option: ::std::os::raw::c_uint, ) -> pj_status_t;
    pub fn pjmedia_sdp_session_add_attr( s: *mut pjmedia_sdp_session, attr: *mut pjmedia_sdp_attr, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_setting_default(opt: *mut pjmedia_rtcp_fb_setting) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_setting_dup( pool: *mut pj_pool_t, dst: *mut pjmedia_rtcp_fb_setting, src: *const pjmedia_rtcp_fb_setting, );
    pub fn pjmedia_rtcp_fb_info_dup( pool: *mut pj_pool_t, dst: *mut pjmedia_rtcp_fb_info, src: *const pjmedia_rtcp_fb_info, );
    pub fn pjmedia_rtcp_fb_encode_sdp( pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, opt: *const pjmedia_rtcp_fb_setting, sdp_local: *mut pjmedia_sdp_session, med_idx: ::std::os::raw::c_uint, sdp_remote: *const pjmedia_sdp_session, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_decode_sdp( pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, opt: *const ::std::os::raw::c_void, sdp: *const pjmedia_sdp_session, med_idx: ::std::os::raw::c_uint, info: *mut pjmedia_rtcp_fb_info, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_decode_sdp2( pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, opt: *const ::std::os::raw::c_void, sdp: *const pjmedia_sdp_session, med_idx: ::std::os::raw::c_uint, pt: ::std::os::raw::c_int, info: *mut pjmedia_rtcp_fb_info, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_build_nack( session: *mut pjmedia_rtcp_session, buf: *mut ::std::os::raw::c_void, length: *mut pj_size_t, nack_cnt: ::std::os::raw::c_uint, nack: *const pjmedia_rtcp_fb_nack, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_build_pli( session: *mut pjmedia_rtcp_session, buf: *mut ::std::os::raw::c_void, length: *mut pj_size_t, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_build_sli( session: *mut pjmedia_rtcp_session, buf: *mut ::std::os::raw::c_void, length: *mut pj_size_t, sli_cnt: ::std::os::raw::c_uint, sli: *const pjmedia_rtcp_fb_sli, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_build_rpsi( session: *mut pjmedia_rtcp_session, buf: *mut ::std::os::raw::c_void, length: *mut pj_size_t, rpsi: *const pjmedia_rtcp_fb_rpsi, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_parse_nack( buf: *const ::std::os::raw::c_void, length: pj_size_t, nack_cnt: *mut ::std::os::raw::c_uint, nack: *mut pjmedia_rtcp_fb_nack, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_parse_pli( buf: *const ::std::os::raw::c_void, length: pj_size_t, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_parse_sli( buf: *const ::std::os::raw::c_void, length: pj_size_t, sli_cnt: *mut ::std::os::raw::c_uint, sli: *mut pjmedia_rtcp_fb_sli, ) -> pj_status_t;
    pub fn pjmedia_rtcp_fb_parse_rpsi( buf: *const ::std::os::raw::c_void, length: pj_size_t, rpsi: *mut pjmedia_rtcp_fb_rpsi, ) -> pj_status_t;
    pub fn pjmedia_videodev_strerror( status: pj_status_t, buffer: *mut ::std::os::raw::c_char, bufsize: pj_size_t, ) -> pj_str_t;
    pub fn pjmedia_get_vid_subsys() -> *mut pjmedia_vid_subsys;
    pub fn pjmedia_vid_driver_init( drv_idx: ::std::os::raw::c_uint, refresh: pj_bool_t, ) -> pj_status_t;
    pub fn pjmedia_vid_driver_deinit(drv_idx: ::std::os::raw::c_uint);
    pub fn pjmedia_vid_dev_cap_name( cap: pjmedia_vid_dev_cap, p_desc: *mut *const ::std::os::raw::c_char, ) -> *const ::std::os::raw::c_char;
    pub fn pjmedia_vid_dev_param_set_cap( param: *mut pjmedia_vid_dev_param, cap: pjmedia_vid_dev_cap, pval: *const ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_param_get_cap( param: *const pjmedia_vid_dev_param, cap: pjmedia_vid_dev_cap, pval: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_refresh() -> pj_status_t;
    pub fn pjmedia_vid_dev_count() -> ::std::os::raw::c_uint;
    pub fn pjmedia_vid_dev_get_info( id: pjmedia_vid_dev_index, info: *mut pjmedia_vid_dev_info, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_lookup( drv_name: *const ::std::os::raw::c_char, dev_name: *const ::std::os::raw::c_char, id: *mut pjmedia_vid_dev_index, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_default_param( pool: *mut pj_pool_t, id: pjmedia_vid_dev_index, param: *mut pjmedia_vid_dev_param, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_create( param: *mut pjmedia_vid_dev_param, cb: *const pjmedia_vid_dev_cb, user_data: *mut ::std::os::raw::c_void, p_strm: *mut *mut pjmedia_vid_dev_stream, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_get_param( strm: *mut pjmedia_vid_dev_stream, param: *mut pjmedia_vid_dev_param, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_get_cap( strm: *mut pjmedia_vid_dev_stream, cap: pjmedia_vid_dev_cap, value: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_set_cap( strm: *mut pjmedia_vid_dev_stream, cap: pjmedia_vid_dev_cap, value: *const ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_start(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_is_running(strm: *mut pjmedia_vid_dev_stream) -> pj_bool_t;
    pub fn pjmedia_vid_dev_stream_get_frame( strm: *mut pjmedia_vid_dev_stream, frame: *mut pjmedia_frame, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_put_frame( strm: *mut pjmedia_vid_dev_stream, frame: *const pjmedia_frame, ) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_stop(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_destroy(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_event_mgr_create( pool: *mut pj_pool_t, options: ::std::os::raw::c_uint, mgr: *mut *mut pjmedia_event_mgr, ) -> pj_status_t;
    pub fn pjmedia_event_mgr_instance() -> *mut pjmedia_event_mgr;
    pub fn pjmedia_event_mgr_set_instance(mgr: *mut pjmedia_event_mgr);
    pub fn pjmedia_event_mgr_destroy(mgr: *mut pjmedia_event_mgr);
    pub fn pjmedia_event_init( event: *mut pjmedia_event, type_: pjmedia_event_type, ts: *const pj_timestamp, src: *const ::std::os::raw::c_void, );
    pub fn pjmedia_event_subscribe( mgr: *mut pjmedia_event_mgr, cb: pjmedia_event_cb, user_data: *mut ::std::os::raw::c_void, epub: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_event_unsubscribe( mgr: *mut pjmedia_event_mgr, cb: pjmedia_event_cb, user_data: *mut ::std::os::raw::c_void, epub: *mut ::std::os::raw::c_void, ) -> pj_status_t;
    pub fn pjmedia_event_publish( mgr: *mut pjmedia_event_mgr, epub: *mut ::std::os::raw::c_void, event: *mut pjmedia_event, flag: pjmedia_event_publish_flag, ) -> pj_status_t;
}


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




