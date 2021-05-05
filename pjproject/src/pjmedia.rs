#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use pj_sys::*;
use pjmedia_sys::*;
use pjmedia_codec_sys::*;
use pjmedia_audiodev_sys::*;

use super::prelude::*;
use super::utils;

use num_enum::*;


use std::ffi::CStr;
use std::ffi::CString;
pub mod auto;


/// pub type pjmedia_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaType {
    None = pjmedia_sys::PJMEDIA_TYPE_NONE,
    Audio = pjmedia_sys::PJMEDIA_TYPE_AUDIO,
    Video = pjmedia_sys::PJMEDIA_TYPE_VIDEO,
    Application = pjmedia_sys::PJMEDIA_TYPE_APPLICATION,
    Unknown = pjmedia_sys::PJMEDIA_TYPE_UNKNOWN,
}

/// pub type pjmedia_tp_proto = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaTpProto {
    None = pjmedia_sys::PJMEDIA_TP_PROTO_NONE,
    Unknown = pjmedia_sys::PJMEDIA_TP_PROTO_UNKNOWN,
    Udp = pjmedia_sys::PJMEDIA_TP_PROTO_UDP,
    Rtp = pjmedia_sys::PJMEDIA_TP_PROTO_RTP,
    Dtls = pjmedia_sys::PJMEDIA_TP_PROTO_DTLS,
    ProfileRtcpFb = pjmedia_sys::PJMEDIA_TP_PROFILE_RTCP_FB,
    ProfileSrtp = pjmedia_sys::PJMEDIA_TP_PROFILE_SRTP,
    ProfileAvp = pjmedia_sys::PJMEDIA_TP_PROFILE_AVP,
    RtpAvp = pjmedia_sys::PJMEDIA_TP_PROTO_RTP_AVP,
    RtpSavp = pjmedia_sys::PJMEDIA_TP_PROTO_RTP_SAVP,
    DtlsSrtp = pjmedia_sys::PJMEDIA_TP_PROTO_DTLS_SRTP,
    RtpAvpf = pjmedia_sys::PJMEDIA_TP_PROTO_RTP_AVPF,
    RtpSavpf = pjmedia_sys::PJMEDIA_TP_PROTO_RTP_SAVPF,
    DtlsSrtpf = pjmedia_sys::PJMEDIA_TP_PROTO_DTLS_SRTPF,
}

/// pub type pjmedia_dir = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaDir {
    None = pjmedia_sys::PJMEDIA_DIR_NONE,
    Encoding = pjmedia_sys::PJMEDIA_DIR_ENCODING,
    // Capture = pjmedia_sys::PJMEDIA_DIR_CAPTURE,
    // Decoding = pjmedia_sys::PJMEDIA_DIR_DECODING,
    // Palyback = pjmedia_sys::PJMEDIA_DIR_PLAYBACK,
    // Render = pjmedia_sys::PJMEDIA_DIR_RENDER,
    EncodingDecoding = pjmedia_sys::PJMEDIA_DIR_ENCODING_DECODING,
    // CapturePlayback = pjmedia_sys::PJMEDIA_DIR_CAPTURE_PLAYBACK,
    // CaptureRender = pjmedia_sys::PJMEDIA_DIR_CAPTURE_RENDER,
}

/// pub type pjmedia_coord_base = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaCoordBase {
    LeftTop = pjmedia_sys::PJMEDIA_COORD_BASE_LEFT_TOP,
    LeftBottom = pjmedia_sys::PJMEDIA_COORD_BASE_LEFT_BOTTOM,
}

/// pub type pjmedia_orient = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaOrient {
    Unknown = pjmedia_sys::PJMEDIA_ORIENT_UNKNOWN,
    Natural = pjmedia_sys::PJMEDIA_ORIENT_NATURAL,
    Rotate80Deg = pjmedia_sys::PJMEDIA_ORIENT_ROTATE_90DEG,
    Rotate180Deg = pjmedia_sys::PJMEDIA_ORIENT_ROTATE_180DEG,
    Rotate270Deg = pjmedia_sys::PJMEDIA_ORIENT_ROTATE_270DEG,
}

/// pub type pjmedia_clock_options = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaClockOptions {
    NoAsync = pjmedia_sys::PJMEDIA_CLOCK_NO_ASYNC,
    NoHighestPtio = pjmedia_sys::PJMEDIA_CLOCK_NO_HIGHEST_PRIO,
}

/// pub type pjmedia_format_id = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaFormatId {
    L16orPCM = pjmedia_sys::PJMEDIA_FORMAT_L16,
    // x = pjmedia_sys::PJMEDIA_FORMAT_PCM,
    PcmaOrAlaw = pjmedia_sys::PJMEDIA_FORMAT_PCMA,
    // Alaw = pjmedia_sys::PJMEDIA_FORMAT_ALAW,
    PcmuOrUlaw = pjmedia_sys::PJMEDIA_FORMAT_PCMU,
    // Ulaw = pjmedia_sys::PJMEDIA_FORMAT_ULAW,
    Amr = pjmedia_sys::PJMEDIA_FORMAT_AMR,
    G279 = pjmedia_sys::PJMEDIA_FORMAT_G729,
    Ilbc = pjmedia_sys::PJMEDIA_FORMAT_ILBC,
    Rgb24 = pjmedia_sys::PJMEDIA_FORMAT_RGB24,
    RgbaOrRgb32 = pjmedia_sys::PJMEDIA_FORMAT_RGBA,
    Bgra = pjmedia_sys::PJMEDIA_FORMAT_BGRA,
    // Rgb32 = pjmedia_sys::PJMEDIA_FORMAT_RGB32,
    Dib = pjmedia_sys::PJMEDIA_FORMAT_DIB,
    Dbrp = pjmedia_sys::PJMEDIA_FORMAT_GBRP,
    Ayuv = pjmedia_sys::PJMEDIA_FORMAT_AYUV,
    Yuy2 = pjmedia_sys::PJMEDIA_FORMAT_YUY2,
    Uyvy = pjmedia_sys::PJMEDIA_FORMAT_UYVY,
    Yvyu = pjmedia_sys::PJMEDIA_FORMAT_YVYU,
    I240OrIyuv = pjmedia_sys::PJMEDIA_FORMAT_I420,
    // Iyuv = pjmedia_sys::PJMEDIA_FORMAT_IYUV,
    Yv12 = pjmedia_sys::PJMEDIA_FORMAT_YV12,
    Nv12 = pjmedia_sys::PJMEDIA_FORMAT_NV12,
    Nv21 = pjmedia_sys::PJMEDIA_FORMAT_NV21,
    I422 = pjmedia_sys::PJMEDIA_FORMAT_I422,
    I420Jpeg = pjmedia_sys::PJMEDIA_FORMAT_I420JPEG,
    I422Jpeg = pjmedia_sys::PJMEDIA_FORMAT_I422JPEG,
    H261 = pjmedia_sys::PJMEDIA_FORMAT_H261,
    H263 = pjmedia_sys::PJMEDIA_FORMAT_H263,
    H263P = pjmedia_sys::PJMEDIA_FORMAT_H263P,
    H264 = pjmedia_sys::PJMEDIA_FORMAT_H264,
    Vp8 = pjmedia_sys::PJMEDIA_FORMAT_VP8,
    Vp9 = pjmedia_sys::PJMEDIA_FORMAT_VP9,
    Mjpeg = pjmedia_sys::PJMEDIA_FORMAT_MJPEG,
    Mpeg1Video = pjmedia_sys::PJMEDIA_FORMAT_MPEG1VIDEO,
    Mpeg2Video = pjmedia_sys::PJMEDIA_FORMAT_MPEG2VIDEO,
    Mpeg4 = pjmedia_sys::PJMEDIA_FORMAT_MPEG4,
    Invalid = pjmedia_sys::PJMEDIA_FORMAT_INVALID,

}

/// pub type pjmedia_format_detail_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaFormatDetailType {
    None = pjmedia_sys::PJMEDIA_FORMAT_DETAIL_NONE,
    Audio = pjmedia_sys::PJMEDIA_FORMAT_DETAIL_AUDIO,
    Video = pjmedia_sys::PJMEDIA_FORMAT_DETAIL_VIDEO,
    Max = pjmedia_sys::PJMEDIA_FORMAT_DETAIL_MAX,
}

/// pub type pjmedia_color_model = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaColorModel {
    None = pjmedia_sys::PJMEDIA_COLOR_MODEL_NONE,
    Rgb = pjmedia_sys::PJMEDIA_COLOR_MODEL_RGB,
    Yuv = pjmedia_sys::PJMEDIA_COLOR_MODEL_YUV,
}

/// pub type pjmedia_frame_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaFrameType {
    None = pjmedia_sys::PJMEDIA_FRAME_TYPE_NONE,
    Audio = pjmedia_sys::PJMEDIA_FRAME_TYPE_AUDIO,
    Extended = pjmedia_sys::PJMEDIA_FRAME_TYPE_EXTENDED,
    Video = pjmedia_sys::PJMEDIA_FRAME_TYPE_VIDEO,
}

/// pub type pjmedia_aud_dev_cap = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaAudDevCap {
    ExtFormat = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_EXT_FORMAT,
    InputLatency = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_INPUT_LATENCY,
    OutputLatency = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_OUTPUT_LATENCY,
    InputVolumeSetting = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_INPUT_VOLUME_SETTING,
    OutputVolumeSetting = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_OUTPUT_VOLUME_SETTING,
    InputSignalMeter = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_INPUT_SIGNAL_METER,
    OutputSignalMeter = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_OUTPUT_SIGNAL_METER,
    InputRouteOrSource = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_INPUT_ROUTE,
    // InputSouce = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_INPUT_SOURCE,
    OutputRoute = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_OUTPUT_ROUTE,
    Ec = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_EC,
    EcTail = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_EC_TAIL,
    Vad = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_VAD,
    Cng = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_CNG,
    Plc = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_PLC,
    Max = pjmedia_sys::PJMEDIA_AUD_DEV_CAP_MAX,
}

/// pub type pjmedia_aud_dev_route = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MeidaaudDevRoute {
    Default = pjmedia_sys::PJMEDIA_AUD_DEV_ROUTE_DEFAULT,
    LoudSpeaker = pjmedia_sys::PJMEDIA_AUD_DEV_ROUTE_LOUDSPEAKER,
    EarPiece = pjmedia_sys::PJMEDIA_AUD_DEV_ROUTE_EARPIECE,
    Bluetooth = pjmedia_sys::PJMEDIA_AUD_DEV_ROUTE_BLUETOOTH,
    Custom = pjmedia_sys::PJMEDIA_AUD_DEV_ROUTE_CUSTOM,
}

// pub type pjmedia_rtcp_xr_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaRtcpXrType {
    LosRle = pjmedia_sys::PJMEDIA_RTCP_XR_LOSS_RLE,
    DupRle = pjmedia_sys::PJMEDIA_RTCP_XR_DUP_RLE,
    RcptTimes = pjmedia_sys::PJMEDIA_RTCP_XR_RCPT_TIMES,
    RrTime = pjmedia_sys::PJMEDIA_RTCP_XR_RR_TIME,
    Dlrr = pjmedia_sys::PJMEDIA_RTCP_XR_DLRR,
    Stats = pjmedia_sys::PJMEDIA_RTCP_XR_STATS,
    VoipMetrics = pjmedia_sys::PJMEDIA_RTCP_XR_VOIP_METRICS,
}

/// pub type pjmedia_rtcp_xr_info = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaRtcpXrInfo {
    SignalLvl = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_SIGNAL_LVL,
    NoiseLvl = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_NOISE_LVL,
    Rerl = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_RERL,
    RFactor = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_R_FACTOR,
    MosLq = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_MOS_LQ,
    MosCq = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_MOS_CQ,
    Plc = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_CONF_PLC,
    Jba = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_CONF_JBA,
    Jbr = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_CONF_JBR,
    Nom = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_JB_NOM,
    Max = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_JB_MAX,
    AbsMax = pjmedia_sys::PJMEDIA_RTCP_XR_INFO_JB_ABS_MAX,
}

/// pub type pjmedia_rtcp_xr_plc_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaRtcpXrPlcType {
    Unk = PJMEDIA_RTCP_XR_PLC_UNK,
    Dis = PJMEDIA_RTCP_XR_PLC_DIS,
    Enh = PJMEDIA_RTCP_XR_PLC_ENH,
    Std = PJMEDIA_RTCP_XR_PLC_STD,
}

/// pub type pjmedia_rtcp_xr_jb_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaRtcpXrJbType {
    Unknown = pjmedia_sys::PJMEDIA_RTCP_XR_JB_UNKNOWN,
    Fixed = pjmedia_sys::PJMEDIA_RTCP_XR_JB_FIXED,
    Adaptive = pjmedia_sys::PJMEDIA_RTCP_XR_JB_ADAPTIVE,
}

/// pub type pjmedia_rtcp_fb_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaRtcpFbType {
    Ack = pjmedia_sys::PJMEDIA_RTCP_FB_ACK,
    Nack = pjmedia_sys::PJMEDIA_RTCP_FB_NACK,
    TrrInt = pjmedia_sys::PJMEDIA_RTCP_FB_TRR_INT,
    Other = pjmedia_sys::PJMEDIA_RTCP_FB_OTHER,
}

/// pub type pjmedia_vid_dev_hwnd_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidDevHwndType {
    None = pjmedia_sys::PJMEDIA_VID_DEV_HWND_TYPE_NONE,
    Windows = pjmedia_sys::PJMEDIA_VID_DEV_HWND_TYPE_WINDOWS,
    Ios = pjmedia_sys::PJMEDIA_VID_DEV_HWND_TYPE_IOS,
    Android = pjmedia_sys::PJMEDIA_VID_DEV_HWND_TYPE_ANDROID,
}

/// pub type pjmedia_vid_dev_wnd_flag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidDevWndFlag {
    Border = pjmedia_sys::PJMEDIA_VID_DEV_WND_BORDER,
    Resizable = pjmedia_sys::PJMEDIA_VID_DEV_WND_RESIZABLE,
}

/// pub type pjmedia_vid_dev_std_index = i32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum MediaVidDevStdIndex {
    DefaultCapture = pjmedia_sys::PJMEDIA_VID_DEFAULT_CAPTURE_DEV,
    DefaultRender = pjmedia_sys::PJMEDIA_VID_DEFAULT_RENDER_DEV,
    Invalid = pjmedia_sys::PJMEDIA_VID_INVALID_DEV,
}

/// pub type pjmedia_vid_dev_cap = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidDevCap {
    Format = pjmedia_sys::PJMEDIA_VID_DEV_CAP_FORMAT,
    InputScale = pjmedia_sys::PJMEDIA_VID_DEV_CAP_INPUT_SCALE,
    OutputWindow = pjmedia_sys::PJMEDIA_VID_DEV_CAP_OUTPUT_WINDOW,
    OutputResize = pjmedia_sys::PJMEDIA_VID_DEV_CAP_OUTPUT_RESIZE,
    OutputPosition = pjmedia_sys::PJMEDIA_VID_DEV_CAP_OUTPUT_POSITION,
    OutputHide = pjmedia_sys::PJMEDIA_VID_DEV_CAP_OUTPUT_HIDE,
    InputPreview = pjmedia_sys::PJMEDIA_VID_DEV_CAP_INPUT_PREVIEW,
    Orientation = pjmedia_sys::PJMEDIA_VID_DEV_CAP_ORIENTATION,
    Switch = pjmedia_sys::PJMEDIA_VID_DEV_CAP_SWITCH,
    WIndowFlags = pjmedia_sys::PJMEDIA_VID_DEV_CAP_OUTPUT_WINDOW_FLAGS,
    FullScreen = pjmedia_sys::PJMEDIA_VID_DEV_CAP_OUTPUT_FULLSCREEN,
    Max = pjmedia_sys::PJMEDIA_VID_DEV_CAP_MAX,
}

/// pub type pjmedia_event_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaEventType {
    None = pjmedia_sys::PJMEDIA_EVENT_NONE,
    FmtChanged = pjmedia_sys::PJMEDIA_EVENT_FMT_CHANGED,
    WndClosing = pjmedia_sys::PJMEDIA_EVENT_WND_CLOSING,
    WndClosed = pjmedia_sys::PJMEDIA_EVENT_WND_CLOSED,
    WndResized = pjmedia_sys::PJMEDIA_EVENT_WND_RESIZED,
    MouseBtnDown = pjmedia_sys::PJMEDIA_EVENT_MOUSE_BTN_DOWN,
    KeyframwFound = pjmedia_sys::PJMEDIA_EVENT_KEYFRAME_FOUND,
    KeyframeMissing = pjmedia_sys::PJMEDIA_EVENT_KEYFRAME_MISSING,
    OrientChanged = pjmedia_sys::PJMEDIA_EVENT_ORIENT_CHANGED,
    RxRtcpFb = pjmedia_sys::PJMEDIA_EVENT_RX_RTCP_FB,
    AudDevError = pjmedia_sys::PJMEDIA_EVENT_AUD_DEV_ERROR,
    VidDevError = pjmedia_sys::PJMEDIA_EVENT_VID_DEV_ERROR,
    TpErr = pjmedia_sys::PJMEDIA_EVENT_MEDIA_TP_ERR,
    Callback = pjmedia_sys::PJMEDIA_EVENT_CALLBACK,
}

/// pub type pjmedia_port_op = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaPortOpt {
    NoChange = pjmedia_sys::PJMEDIA_PORT_NO_CHANGE,
    Disable = pjmedia_sys::PJMEDIA_PORT_DISABLE,
    Mute = pjmedia_sys::PJMEDIA_PORT_MUTE,
    Enable = pjmedia_sys::PJMEDIA_PORT_ENABLE,
}

/// pub type pjmedia_rtp_pt = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaRtpPt {
    Pcmu = pjmedia_sys::PJMEDIA_RTP_PT_PCMU,
    G721 = pjmedia_sys::PJMEDIA_RTP_PT_G721,
    Gsm = pjmedia_sys::PJMEDIA_RTP_PT_GSM,
    G723 = pjmedia_sys::PJMEDIA_RTP_PT_G723,
    Dvi4_8K = pjmedia_sys::PJMEDIA_RTP_PT_DVI4_8K,
    Dvi4_16K = pjmedia_sys::PJMEDIA_RTP_PT_DVI4_16K,
    Lpc = pjmedia_sys::PJMEDIA_RTP_PT_LPC,
    Pcma = pjmedia_sys::PJMEDIA_RTP_PT_PCMA,
    G722 = pjmedia_sys::PJMEDIA_RTP_PT_G722,
    L16_2 = pjmedia_sys::PJMEDIA_RTP_PT_L16_2,
    L16_1 = pjmedia_sys::PJMEDIA_RTP_PT_L16_1,
    QCelp = pjmedia_sys::PJMEDIA_RTP_PT_QCELP,
    Cn = pjmedia_sys::PJMEDIA_RTP_PT_CN,
    Mpa = pjmedia_sys::PJMEDIA_RTP_PT_MPA,
    G728 = pjmedia_sys::PJMEDIA_RTP_PT_G728,
    Dvi4_11K = pjmedia_sys::PJMEDIA_RTP_PT_DVI4_11K,
    Dvi4_22K = pjmedia_sys::PJMEDIA_RTP_PT_DVI4_22K,
    G729 = pjmedia_sys::PJMEDIA_RTP_PT_G729,
    Celb = pjmedia_sys::PJMEDIA_RTP_PT_CELB,
    Jpeg = pjmedia_sys::PJMEDIA_RTP_PT_JPEG,
    Nv = pjmedia_sys::PJMEDIA_RTP_PT_NV,
    H261 = pjmedia_sys::PJMEDIA_RTP_PT_H261,
    Mpv = pjmedia_sys::PJMEDIA_RTP_PT_MPV,
    Mp2T = pjmedia_sys::PJMEDIA_RTP_PT_MP2T,
    H263 = pjmedia_sys::PJMEDIA_RTP_PT_H263,
    Dynamic = pjmedia_sys::PJMEDIA_RTP_PT_DYNAMIC,
}

/// pub type pjmedia_codec_priority = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaCodecPriority {
    Highest = pjmedia_sys::PJMEDIA_CODEC_PRIO_HIGHEST,
    nextHigher = pjmedia_sys::PJMEDIA_CODEC_PRIO_NEXT_HIGHER,
    Normal = pjmedia_sys::PJMEDIA_CODEC_PRIO_NORMAL,
    Lowest = pjmedia_sys::PJMEDIA_CODEC_PRIO_LOWEST,
    Disbled = pjmedia_sys::PJMEDIA_CODEC_PRIO_DISABLED,
}

// pub type pjmedia_conf_option = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaConfOption {
    NoMic = pjmedia_sys::PJMEDIA_CONF_NO_MIC,
    NoDevice = pjmedia_sys::PJMEDIA_CONF_NO_DEVICE,
    SmallFilter = pjmedia_sys::PJMEDIA_CONF_SMALL_FILTER,
    UseLinear = pjmedia_sys::PJMEDIA_CONF_USE_LINEAR,
}

/// pub type pjmedia_converter_priority_guide = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaConverterPriorityGuide {
    Lowest = pjmedia_sys::PJMEDIA_CONVERTER_PRIORITY_LOWEST,
    Normal = pjmedia_sys::PJMEDIA_CONVERTER_PRIORITY_NORMAL,
    Highest = pjmedia_sys::PJMEDIA_CONVERTER_PRIORITY_HIGHEST,
}

/// pub type pjmedia_echo_flag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaEchoFlag {
    Default = pjmedia_sys::PJMEDIA_ECHO_DEFAULT,
    Speex = pjmedia_sys::PJMEDIA_ECHO_SPEEX,
    Simple = pjmedia_sys::PJMEDIA_ECHO_SIMPLE,
    Webrtc = pjmedia_sys::PJMEDIA_ECHO_WEBRTC,
    AlogMask = pjmedia_sys::PJMEDIA_ECHO_ALGO_MASK,
    NoLock = pjmedia_sys::PJMEDIA_ECHO_NO_LOCK,
    SimpleFifo = pjmedia_sys::PJMEDIA_ECHO_USE_SIMPLE_FIFO,
    SwEcho = pjmedia_sys::PJMEDIA_ECHO_USE_SW_ECHO,
    Supresor = pjmedia_sys::PJMEDIA_ECHO_USE_NOISE_SUPPRESSOR,
    // AggressivenessDefault = pjmedia_sys::PJMEDIA_ECHO_AGGRESSIVENESS_DEFAULT,
    // AggressivenessConservative = pjmedia_sys::PJMEDIA_ECHO_AGGRESSIVENESS_CONSERVATIVE,
    // AggressivenessModerate = pjmedia_sys::PJMEDIA_ECHO_AGGRESSIVENESS_MODERATE,
    // AggressivenessAgressive = pjmedia_sys::PJMEDIA_ECHO_AGGRESSIVENESS_AGGRESSIVE,
    // AggressivenessMask = pjmedia_sys::PJMEDIA_ECHO_AGGRESSIVENESS_MASK,
}

/// pub type pjmedia_tranport_media_option = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaTranportMediaOption {
    NoTransportChecking = pjmedia_sys::PJMEDIA_TPMED_NO_TRANSPORT_CHECKING,
    RtcpMux = pjmedia_sys::PJMEDIA_TPMED_RTCP_MUX,
}

/// pub type pjmedia_transport_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaTransportType {
    Udp = pjmedia_sys::PJMEDIA_TRANSPORT_TYPE_UDP,
    Ice = pjmedia_sys::PJMEDIA_TRANSPORT_TYPE_ICE,
    Srtp = pjmedia_sys::PJMEDIA_TRANSPORT_TYPE_SRTP,
    Loop = pjmedia_sys::PJMEDIA_TRANSPORT_TYPE_LOOP,
    User = pjmedia_sys::PJMEDIA_TRANSPORT_TYPE_USER,
}

/// pub type pjmedia_audio_pt = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaAudioPt {
    Start = pjmedia_sys::PJMEDIA_RTP_PT_START,
    SpeexNb = pjmedia_sys::PJMEDIA_RTP_PT_SPEEX_NB,
    SpeexWb = pjmedia_sys::PJMEDIA_RTP_PT_SPEEX_WB,
    SpeexUwb = pjmedia_sys::PJMEDIA_RTP_PT_SPEEX_UWB,
    SilkNb = pjmedia_sys::PJMEDIA_RTP_PT_SILK_NB,
    SilkMb = pjmedia_sys::PJMEDIA_RTP_PT_SILK_MB,
    SilkWb = pjmedia_sys::PJMEDIA_RTP_PT_SILK_WB,
    SilkSwb = pjmedia_sys::PJMEDIA_RTP_PT_SILK_SWB,
    Ilbc = pjmedia_sys::PJMEDIA_RTP_PT_ILBC,
    Amr = pjmedia_sys::PJMEDIA_RTP_PT_AMR,
    AmrWb = pjmedia_sys::PJMEDIA_RTP_PT_AMRWB,
    AmrWbe = pjmedia_sys::PJMEDIA_RTP_PT_AMRWBE,
    G726_16 = pjmedia_sys::PJMEDIA_RTP_PT_G726_16,
    G726_24 = pjmedia_sys::PJMEDIA_RTP_PT_G726_24,
    G726_32 = pjmedia_sys::PJMEDIA_RTP_PT_G726_32,
    G726_40 = pjmedia_sys::PJMEDIA_RTP_PT_G726_40,
    G722_1_16 = pjmedia_sys::PJMEDIA_RTP_PT_G722_1_16,
    G722_1_24 = pjmedia_sys::PJMEDIA_RTP_PT_G722_1_24,
    G722_1_32 = pjmedia_sys::PJMEDIA_RTP_PT_G722_1_32,
    G7221c24 = pjmedia_sys::PJMEDIA_RTP_PT_G7221C_24,
    G7221c32 = pjmedia_sys::PJMEDIA_RTP_PT_G7221C_32,
    G7221c48 = pjmedia_sys::PJMEDIA_RTP_PT_G7221C_48,
    G7221Rsv1 = pjmedia_sys::PJMEDIA_RTP_PT_G7221_RSV1,
    G7221Rsv2 = pjmedia_sys::PJMEDIA_RTP_PT_G7221_RSV2,
    Opus = pjmedia_sys::PJMEDIA_RTP_PT_OPUS,
}

/// pub type pjmedia_video_pt = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVideoPt {
    VidStart = pjmedia_sys::PJMEDIA_RTP_PT_VID_START,
    H263p = pjmedia_sys::PJMEDIA_RTP_PT_H263P,
    H264 = pjmedia_sys::PJMEDIA_RTP_PT_H264,
    H264Rsv1 = pjmedia_sys::PJMEDIA_RTP_PT_H264_RSV1,
    H264Rsv2 = pjmedia_sys::PJMEDIA_RTP_PT_H264_RSV2,
    H264Rsv3 = pjmedia_sys::PJMEDIA_RTP_PT_H264_RSV3,
    H264Rsv4 = pjmedia_sys::PJMEDIA_RTP_PT_H264_RSV4,
    Vp8 = pjmedia_sys::PJMEDIA_RTP_PT_VP8,
    Vp9 = pjmedia_sys::PJMEDIA_RTP_PT_VP9,
}

/// pub type pjmedia_jb_frame_type = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaJbFrameType {
    MissingFrame = pjmedia_sys::PJMEDIA_JB_MISSING_FRAME,
    NormalFrame = pjmedia_sys::PJMEDIA_JB_NORMAL_FRAME, 
    PrefetchFrame = pjmedia_sys::PJMEDIA_JB_ZERO_PREFETCH_FRAME, 
    EmptyFrame = pjmedia_sys::PJMEDIA_JB_ZERO_EMPTY_FRAME, 
}

/// pub type pjmedia_jb_discard_algo = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaJbDiscardAlgo {
    None = pjmedia_sys::PJMEDIA_JB_DISCARD_NONE,
    Static = pjmedia_sys::PJMEDIA_JB_DISCARD_STATIC,
    Progressive = pjmedia_sys::PJMEDIA_JB_DISCARD_PROGRESSIVE,
}

/// pub type pjmedia_resample_port_options = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaResamplePortOptions {
    Linear = pjmedia_sys::PJMEDIA_RESAMPLE_USE_LINEAR,
    SmallFilter = pjmedia_sys::PJMEDIA_RESAMPLE_USE_SMALL_FILTER,
    DontDestroyDn = pjmedia_sys::PJMEDIA_RESAMPLE_DONT_DESTROY_DN,
}

/// pub type pjmedia_sdp_neg_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaSdpNegState {
    Null = pjmedia_sys::PJMEDIA_SDP_NEG_STATE_NULL,
    LocalOffer = pjmedia_sys::PJMEDIA_SDP_NEG_STATE_LOCAL_OFFER,
    RemoteOffer = pjmedia_sys::PJMEDIA_SDP_NEG_STATE_REMOTE_OFFER,
    WaitNego = pjmedia_sys::PJMEDIA_SDP_NEG_STATE_WAIT_NEGO,
    Done = pjmedia_sys::PJMEDIA_SDP_NEG_STATE_DONE,
}

/// pub type pjmedia_vid_packing = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidPacking {
    Unknown = pjmedia_sys::PJMEDIA_VID_PACKING_UNKNOWN,
    Packets = pjmedia_sys::PJMEDIA_VID_PACKING_PACKETS,
    Whole = pjmedia_sys::PJMEDIA_VID_PACKING_WHOLE,
}

/// pub type pjmedia_vid_frm_bit_info = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidFrmBitInfo {
    Keyframe = pjmedia_sys::PJMEDIA_VID_FRM_KEYFRAME
}

/// pub type pjmedia_stream_dtmf_event_flags = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaStreamDtmfEventFlags {
    Update = pjmedia_sys::PJMEDIA_STREAM_DTMF_IS_UPDATE,
    End = pjmedia_sys::PJMEDIA_STREAM_DTMF_IS_END,
}

/// pub type pjmedia_transport_ice_options = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaTransportIceOptions {
    NoSrcAddrChecking = pjmedia_sys::PJMEDIA_ICE_NO_SRC_ADDR_CHECKING,
    DisableIceMismatch = pjmedia_sys::PJMEDIA_ICE_DISABLE_ICE_MISMATCH,
}

/// pub type pjmedia_srtp_crypto_option = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaSrtpCryptoOption {
    NoEncryption = pjmedia_sys::PJMEDIA_SRTP_NO_ENCRYPTION,
    NoAuthentication = pjmedia_sys::PJMEDIA_SRTP_NO_AUTHENTICATION,
}

/// pub type pjmedia_srtp_use = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaSrtpUse {
    DisabledOrUnknown = pjmedia_sys::PJMEDIA_SRTP_DISABLED,
    // Unknown = pjmedia_sys::PJMEDIA_SRTP_UNKNOWN,
    Optional = pjmedia_sys::PJMEDIA_SRTP_OPTIONAL,
    Mandatory = pjmedia_sys::PJMEDIA_SRTP_MANDATORY,
}

/// pub type pjmedia_srtp_keying_method = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaSrtpKeyingMethod {
    Sdes = pjmedia_sys::PJMEDIA_SRTP_KEYING_SDES,
    DtlsSrtp = pjmedia_sys::PJMEDIA_SRTP_KEYING_DTLS_SRTP,
    Count = pjmedia_sys::PJMEDIA_SRTP_KEYINGS_COUNT,
}

/// pub type pjmedia_vid_conf_layout = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidConfLayout {
    Default = pjmedia_sys::PJMEDIA_VID_CONF_LAYOUT_DEFAULT,
    SelectiveFocus = pjmedia_sys::PJMEDIA_VID_CONF_LAYOUT_SELECTIVE_FOCUS,
    IntervalFocus = pjmedia_sys::PJMEDIA_VID_CONF_LAYOUT_INTERVAL_FOCUS,
    Custom = pjmedia_sys::PJMEDIA_VID_CONF_LAYOUT_CUSTOM,
}

/// pub type pjmedia_vid_stream_rc_method = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaVidStreamRcMethod {
    None = pjmedia_sys::PJMEDIA_VID_STREAM_RC_NONE,
    SimpleBlocking = pjmedia_sys::PJMEDIA_VID_STREAM_RC_SIMPLE_BLOCKING,
}

/// pub type pjmedia_file_writer_option = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaFileWriterOption {
    Pcm = pjmedia_sys::PJMEDIA_FILE_WRITE_PCM,
    Alaw = pjmedia_sys::PJMEDIA_FILE_WRITE_ALAW,
    Ulaw = pjmedia_sys::PJMEDIA_FILE_WRITE_ULAW,
}

/// pub type pjmedia_wave_fmt_tag = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaWaveFmtTag {
    Pcm = pjmedia_sys::PJMEDIA_WAVE_FMT_TAG_PCM,
    Alaw = pjmedia_sys::PJMEDIA_WAVE_FMT_TAG_ALAW,
    Ulaw = pjmedia_sys::PJMEDIA_WAVE_FMT_TAG_ULAW,
}

/// pub type pjmedia_wsola_option = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MediaWsolaOption {
    NoHanning = pjmedia_sys::PJMEDIA_WSOLA_NO_HANNING,
    Plc = pjmedia_sys::PJMEDIA_WSOLA_NO_PLC,
    NoDiscard = pjmedia_sys::PJMEDIA_WSOLA_NO_DISCARD,
    NoFading = pjmedia_sys::PJMEDIA_WSOLA_NO_FADING,
}

pub fn type_name(media_type: pjmedia_type) -> String {
    unsafe {
        String::from(CStr::from_ptr(
            pjmedia_type_name(media_type)
        ).to_str()
        .expect("Error string media type"))
    }
}

// function helper to reduce unsafe block


// Audio device API

// pjmedia_aud_subsys * 	pjmedia_get_aud_subsys (void)
pub fn get_aud_subsys() -> *mut pjmedia_aud_subsys {
    unsafe { pjmedia_get_aud_subsys() }
}

pub fn aud_driver_init(drv_idx: u32, refresh: bool) -> Result<(), pj_status_t> {

    unsafe {
        let status = pjmedia_aud_driver_init(
            drv_idx,
            utils::boolean_to_pjbool(refresh)
        );

        utils::check_status(status)
    }
}

pub fn aud_driver_deinit(drv_idx: u32) {
    unsafe { pjmedia_aud_driver_deinit(drv_idx) }
}

// const char * 	pjmedia_aud_dev_cap_name (pjmedia_aud_dev_cap cap, const char **p_desc)
// pj_status_t 	pjmedia_aud_param_set_cap (pjmedia_aud_param *param, pjmedia_aud_dev_cap cap, const void *pval)
// pj_status_t 	pjmedia_aud_param_get_cap (const pjmedia_aud_param *param, pjmedia_aud_dev_cap cap, void *pval)

pub fn aud_dev_refresh() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_dev_refresh()) }
}

pub fn aud_dev_count() -> u32 {
    unsafe { pjmedia_aud_dev_count() }
}

pub fn aud_dev_get_info(id: pjmedia_aud_dev_index, info: &mut pjmedia_aud_dev_info) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_dev_get_info(id, info as *mut _))
    }
}

pub fn aud_dev_lookup (drv_name: String, dev_name: String, id: &mut pjmedia_aud_dev_index) -> Result<(), pj_status_t> {
    let drv_name: *const i8 = CString::new(drv_name.as_str()).expect("error drv_name").into_raw();
    let dev_name: *const i8 = CString::new(dev_name.as_str()).expect("error dev_name").into_raw();

    unsafe {
        utils::check_status(pjmedia_aud_dev_lookup( drv_name, dev_name, id as *mut _))
    }
}

pub fn aud_dev_default_param(id: pjmedia_aud_dev_index, param: &mut pjmedia_aud_param) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_dev_default_param( id, param as *mut _)) }
}

// pj_status_t 	pjmedia_aud_stream_create (const pjmedia_aud_param *param, pjmedia_aud_rec_cb rec_cb, pjmedia_aud_play_cb play_cb, void *user_data, pjmedia_aud_stream **p_strm)

pub fn aud_stream_get_param (strm: &mut pjmedia_aud_stream, param: &mut pjmedia_aud_param) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_get_param(strm as *mut _, param as *mut _))
    }
}

// pj_status_t 	pjmedia_aud_stream_get_cap (pjmedia_aud_stream *strm, pjmedia_aud_dev_cap cap, void *value)
// pj_status_t 	pjmedia_aud_stream_set_cap (pjmedia_aud_stream *strm, pjmedia_aud_dev_cap cap, const void *value)

pub fn aud_stream_start(strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_start(strm as *mut _))
    }
}

pub fn aud_stream_stop(strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_stop(strm as *mut _))
    }
}

pub fn aud_stream_destroy (strm: &mut pjmedia_aud_stream) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_aud_stream_destroy(strm as *mut _))
    }
}

pub fn aud_subsys_init(pf: *mut pj_pool_factory) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_subsys_init(pf)) }
}

pub fn aud_subsys_get_pool_factory() -> *mut pj_pool_factory {
    unsafe { pjmedia_aud_subsys_get_pool_factory() }
}

pub fn aud_subsys_shutdown() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_aud_subsys_shutdown()) }
}

// pj_status_t 	pjmedia_aud_register_factory (pjmedia_aud_dev_factory_create_func_ptr adf)
// pj_status_t 	pjmedia_aud_unregister_factory (pjmedia_aud_dev_factory_create_func_ptr adf)


// pj_status_t 	pjmedia_tonegen_create (pj_pool_t *pool, unsigned clock_rate, unsigned channel_count, unsigned samples_per_frame, unsigned bits_per_sample, unsigned options, pjmedia_port **p_port)

pub fn tonegen_create2(
    pool: *mut pj_pool_t,
    name: String,
    clock_rate: u32,
    channel_count: u32,
    samples_per_frame: u32,
    bits_per_sample: u32,
    options: u32,
    p_port: &mut Box<*mut pjmedia_port>
) -> Result<(), pj_status_t> {
    unsafe {

        let mut name = pj_str_t::from_string(name);

        let result = pjmedia_tonegen_create2(
            pool,
            &mut name as *const _,
            clock_rate,
            channel_count,
            samples_per_frame,
            bits_per_sample,
            options,
            p_port.as_mut() as *mut _
        );

        utils::check_status(result)
    }
}

pub fn tonegen_is_busy(tonegen: &mut pjmedia_port) -> bool {
    unsafe {
        utils::check_boolean(pjmedia_tonegen_is_busy(tonegen  as *mut _))
    }
}

pub fn tonegen_stop(tonegen: *mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_tonegen_stop(tonegen)) }
}

pub fn tonegen_rewind(tonegen: &mut pjmedia_port) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_tonegen_rewind(tonegen as *mut _) )}
}

// pj_status_t 	pjmedia_tonegen_play (pjmedia_port *tonegen, unsigned count, const pjmedia_tone_desc tones[], unsigned options)
// PJMEDIA_TONEGEN_MAX_DIGITS 32
pub fn tonegen_play(
    tonegen: *mut pjmedia_port,
    count: u32,
    tones: &mut [pjmedia_tone_desc; 32usize],
    options: u32
) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_tonegen_play( tonegen, count, tones.as_ptr(), options ))
    }
}

// pj_status_t 	pjmedia_tonegen_play_digits (pjmedia_port *tonegen, unsigned count, const pjmedia_tone_digit digits[], unsigned options)
// pj_status_t 	pjmedia_tonegen_get_digit_map (pjmedia_port *tonegen, const pjmedia_tone_digit_map **m)
// pj_status_t 	pjmedia_tonegen_set_digit_map (pjmedia_port *tonegen, pjmedia_tone_digit_map *m)


// pj_status_t 	pjmedia_codec_opus_init (pjmedia_endpt *endpt)
pub fn codec_opus_init(endpt: &mut pjmedia_endpt) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_codec_opus_init(endpt as *mut _)) }
}

// pj_status_t 	pjmedia_codec_opus_deinit (void)
pub fn codec_opus_deinit() -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_codec_opus_deinit()) }
}

// pj_status_t 	pjmedia_codec_opus_get_config (pjmedia_codec_opus_config *cfg)
pub fn codec_opus_get_config(cfg: &mut pjmedia_codec_opus_config) -> Result<(), pj_status_t> {
    unsafe { utils::check_status(pjmedia_codec_opus_get_config( cfg as *mut _ )) }
}

// pj_status_t 	pjmedia_codec_opus_set_default_param (const pjmedia_codec_opus_config *cfg, pjmedia_codec_param *param)
pub fn codec_opus_set_default_param(cfg: &mut pjmedia_codec_opus_config, param: &mut pjmedia_codec_param) -> Result<(), pj_status_t> {
    unsafe {
        utils::check_status(pjmedia_codec_opus_set_default_param ( cfg as *mut _, param as *mut _ ))
    }
}
