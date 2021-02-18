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




