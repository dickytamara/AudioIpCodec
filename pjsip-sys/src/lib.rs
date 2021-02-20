#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate pj_sys;
extern crate pjmedia_sys;
use pj_sys::*;
use pjmedia_sys::*;

use std::os::raw::{c_int, c_char, c_uint, c_void, c_long};


pub const PJSIP_MAX_TSX_COUNT: u32 = 1023;
pub const PJSIP_MAX_DIALOG_COUNT: u32 = 511;
pub const PJSIP_MAX_TRANSPORTS: u32 = 64;
pub const PJSIP_TPMGR_HTABLE_SIZE: u32 = 31;
pub const PJSIP_MAX_URL_SIZE: u32 = 256;
pub const PJSIP_MAX_MODULE: u32 = 32;
pub const PJSIP_MAX_PKT_LEN: u32 = 4000;
pub const PJSIP_DONT_SWITCH_TO_TCP: u32 = 0;
pub const PJSIP_DONT_SWITCH_TO_TLS: u32 = 0;
pub const PJSIP_HANDLE_EVENTS_HAS_SLEEP_ON_ERR: u32 = 1;
pub const PJSIP_ACCEPT_REPLACE_IN_EARLY_STATE: u32 = 0;
pub const PJSIP_UDP_SIZE_THRESHOLD: u32 = 1300;
pub const PJSIP_ENCODE_SHORT_HNAME: u32 = 0;
pub const PJSIP_INCLUDE_ALLOW_HDR_IN_DLG: u32 = 1;
pub const PJSIP_SAFE_MODULE: u32 = 1;
pub const PJSIP_CHECK_VIA_SENT_BY: u32 = 0;
pub const PJSIP_UNESCAPE_IN_PLACE: u32 = 0;
pub const PJSIP_ALLOW_PORT_IN_FROMTO_HDR: u32 = 0;
pub const PJSIP_MAX_NET_EVENTS: u32 = 1;
pub const PJSIP_MAX_TIMED_OUT_ENTRIES: u32 = 10;
pub const PJSIP_TRANSPORT_IDLE_TIME: u32 = 33;
pub const PJSIP_TRANSPORT_SERVER_IDLE_TIME: u32 = 600;
pub const PJSIP_TCP_TRANSPORT_BACKLOG: u32 = 5;
pub const PJSIP_TCP_TRANSPORT_REUSEADDR: u32 = 1;
pub const PJSIP_TCP_TRANSPORT_DONT_CREATE_LISTENER: u32 = 0;
pub const PJSIP_TLS_TRANSPORT_DONT_CREATE_LISTENER: u32 = 0;
pub const PJSIP_TCP_KEEP_ALIVE_INTERVAL: u32 = 90;
pub const PJSIP_TCP_INITIAL_TIMEOUT: u32 = 0;
pub const PJSIP_TLS_KEEP_ALIVE_INTERVAL: u32 = 90;
pub const PJSIP_HAS_RESOLVER: u32 = 1;
pub const PJSIP_MAX_RESOLVED_ADDRESSES: u32 = 16;
pub const PJSIP_HAS_TLS_TRANSPORT: u32 = 0;
pub const PJSIP_TLS_TRANSPORT_BACKLOG: u32 = 5;
pub const PJSIP_TLS_TRANSPORT_REUSEADDR: u32 = 1;
pub const PJSIP_POOL_LEN_ENDPT: u32 = 4000;
pub const PJSIP_POOL_INC_ENDPT: u32 = 4000;
pub const PJSIP_POOL_RDATA_LEN: u32 = 4000;
pub const PJSIP_POOL_RDATA_INC: u32 = 4000;
pub const PJSIP_POOL_LEN_TRANSPORT: u32 = 512;
pub const PJSIP_POOL_INC_TRANSPORT: u32 = 512;
pub const PJSIP_POOL_LEN_TDATA: u32 = 4000;
pub const PJSIP_POOL_INC_TDATA: u32 = 4000;
pub const PJSIP_POOL_LEN_UA: u32 = 512;
pub const PJSIP_POOL_INC_UA: u32 = 512;
pub const PJSIP_POOL_EVSUB_LEN: u32 = 512;
pub const PJSIP_POOL_EVSUB_INC: u32 = 512;
pub const PJSIP_MAX_FORWARDS_VALUE: u32 = 70;
pub const PJSIP_RFC3261_BRANCH_ID: &'static [u8; 8usize] = b"z9hG4bK\0";
pub const PJSIP_RFC3261_BRANCH_LEN: u32 = 7;
pub const PJSIP_POOL_TSX_LAYER_LEN: u32 = 512;
pub const PJSIP_POOL_TSX_LAYER_INC: u32 = 512;
pub const PJSIP_POOL_TSX_LEN: u32 = 1536;
pub const PJSIP_POOL_TSX_INC: u32 = 256;
pub const PJSIP_TSX_1XX_RETRANS_DELAY: u32 = 60;
pub const PJSIP_MAX_TSX_KEY_LEN: u32 = 512;
pub const PJSIP_POOL_LEN_USER_AGENT: u32 = 1024;
pub const PJSIP_POOL_INC_USER_AGENT: u32 = 1024;
pub const PJSIP_MAX_HNAME_LEN: u32 = 64;
pub const PJSIP_POOL_LEN_DIALOG: u32 = 1200;
pub const PJSIP_POOL_INC_DIALOG: u32 = 512;
pub const PJSIP_MAX_HEADER_TYPES: u32 = 72;
pub const PJSIP_MAX_URI_TYPES: u32 = 4;
pub const PJSIP_T1_TIMEOUT: u32 = 500;
pub const PJSIP_T2_TIMEOUT: u32 = 4000;
pub const PJSIP_T4_TIMEOUT: u32 = 5000;
pub const PJSIP_TD_TIMEOUT: u32 = 32000;
pub const PJSIP_AUTH_HEADER_CACHING: u32 = 0;
pub const PJSIP_AUTH_AUTO_SEND_NEXT: u32 = 0;
pub const PJSIP_AUTH_QOP_SUPPORT: u32 = 1;
pub const PJSIP_MAX_STALE_COUNT: u32 = 3;
pub const PJSIP_HAS_DIGEST_AKA_AUTH: u32 = 0;
pub const PJSIP_REGISTER_CLIENT_DELAY_BEFORE_REFRESH: u32 = 5;
pub const PJSIP_REGISTER_CLIENT_CHECK_CONTACT: u32 = 1;
pub const PJSIP_REGISTER_CLIENT_ADD_XUID_PARAM: u32 = 0;
pub const PJSIP_AUTH_CACHED_POOL_MAX_SIZE: u32 = 20480;
pub const PJSIP_AUTH_CNONCE_USE_DIGITS_ONLY: u32 = 1;
pub const PJSIP_EVSUB_TIME_UAC_REFRESH: u32 = 5;
pub const PJSIP_PUBLISHC_DELAY_BEFORE_REFRESH: u32 = 5;
pub const PJSIP_EVSUB_TIME_UAC_TERMINATE: u32 = 5;
pub const PJSIP_EVSUB_TIME_UAC_WAIT_NOTIFY: u32 = 5;
pub const PJSIP_PRES_DEFAULT_EXPIRES: u32 = 600;
pub const PJSIP_PRES_BAD_CONTENT_RESPONSE: u32 = 488;
pub const PJSIP_PRES_PIDF_ADD_TIMESTAMP: u32 = 1;
pub const PJSIP_SESS_TIMER_DEF_SE: u32 = 1800;
pub const PJSIP_SESS_TIMER_RETRY_DELAY: u32 = 10;
pub const PJSIP_PUBLISHC_QUEUE_REQUEST: u32 = 1;
pub const PJSIP_MWI_DEFAULT_EXPIRES: u32 = 3600;
pub const PJSIP_HAS_TX_DATA_LIST: u32 = 0;
pub const PJSIP_ERRNO_START: u32 = 170000;
pub const PJSIP_ERRNO_START_PJSIP: u32 = 171000;
pub const PJSIP_EBUSY: u32 = 171001;
pub const PJSIP_ETYPEEXISTS: u32 = 171002;
pub const PJSIP_ESHUTDOWN: u32 = 171003;
pub const PJSIP_ENOTINITIALIZED: u32 = 171004;
pub const PJSIP_ENOROUTESET: u32 = 171005;
pub const PJSIP_EINVALIDMSG: u32 = 171020;
pub const PJSIP_ENOTREQUESTMSG: u32 = 171021;
pub const PJSIP_ENOTRESPONSEMSG: u32 = 171022;
pub const PJSIP_EMSGTOOLONG: u32 = 171023;
pub const PJSIP_EPARTIALMSG: u32 = 171024;
pub const PJSIP_EINVALIDSTATUS: u32 = 171030;
pub const PJSIP_EINVALIDURI: u32 = 171039;
pub const PJSIP_EINVALIDSCHEME: u32 = 171040;
pub const PJSIP_EMISSINGREQURI: u32 = 171041;
pub const PJSIP_EINVALIDREQURI: u32 = 171042;
pub const PJSIP_EURITOOLONG: u32 = 171043;
pub const PJSIP_EMISSINGHDR: u32 = 171050;
pub const PJSIP_EINVALIDHDR: u32 = 171051;
pub const PJSIP_EINVALIDVIA: u32 = 171052;
pub const PJSIP_EMULTIPLEVIA: u32 = 171053;
pub const PJSIP_EMISSINGBODY: u32 = 171054;
pub const PJSIP_EINVALIDMETHOD: u32 = 171055;
pub const PJSIP_EUNSUPTRANSPORT: u32 = 171060;
pub const PJSIP_EPENDINGTX: u32 = 171061;
pub const PJSIP_ERXOVERFLOW: u32 = 171062;
pub const PJSIP_EBUFDESTROYED: u32 = 171063;
pub const PJSIP_ETPNOTSUITABLE: u32 = 171064;
pub const PJSIP_ETPNOTAVAIL: u32 = 171065;
pub const PJSIP_ETSXDESTROYED: u32 = 171070;
pub const PJSIP_ENOTSX: u32 = 171071;
pub const PJSIP_ECMPSCHEME: u32 = 171080;
pub const PJSIP_ECMPUSER: u32 = 171081;
pub const PJSIP_ECMPPASSWD: u32 = 171082;
pub const PJSIP_ECMPHOST: u32 = 171083;
pub const PJSIP_ECMPPORT: u32 = 171084;
pub const PJSIP_ECMPTRANSPORTPRM: u32 = 171085;
pub const PJSIP_ECMPTTLPARAM: u32 = 171086;
pub const PJSIP_ECMPUSERPARAM: u32 = 171087;
pub const PJSIP_ECMPMETHODPARAM: u32 = 171088;
pub const PJSIP_ECMPMADDRPARAM: u32 = 171089;
pub const PJSIP_ECMPOTHERPARAM: u32 = 171090;
pub const PJSIP_ECMPHEADERPARAM: u32 = 171091;
pub const PJSIP_EFAILEDCREDENTIAL: u32 = 171100;
pub const PJSIP_ENOCREDENTIAL: u32 = 171101;
pub const PJSIP_EINVALIDALGORITHM: u32 = 171102;
pub const PJSIP_EINVALIDQOP: u32 = 171103;
pub const PJSIP_EINVALIDAUTHSCHEME: u32 = 171104;
pub const PJSIP_EAUTHNOPREVCHAL: u32 = 171105;
pub const PJSIP_EAUTHNOAUTH: u32 = 171106;
pub const PJSIP_EAUTHACCNOTFOUND: u32 = 171107;
pub const PJSIP_EAUTHACCDISABLED: u32 = 171108;
pub const PJSIP_EAUTHINVALIDREALM: u32 = 171109;
pub const PJSIP_EAUTHINVALIDDIGEST: u32 = 171110;
pub const PJSIP_EAUTHSTALECOUNT: u32 = 171111;
pub const PJSIP_EAUTHINNONCE: u32 = 171112;
pub const PJSIP_EAUTHINAKACRED: u32 = 171113;
pub const PJSIP_EAUTHNOCHAL: u32 = 171114;
pub const PJSIP_EMISSINGTAG: u32 = 171120;
pub const PJSIP_ENOTREFER: u32 = 171121;
pub const PJSIP_ENOREFERSESSION: u32 = 171122;
pub const PJSIP_ESESSIONTERMINATED: u32 = 171140;
pub const PJSIP_ESESSIONSTATE: u32 = 171141;
pub const PJSIP_ESESSIONINSECURE: u32 = 171142;
pub const PJSIP_TLS_EUNKNOWN: u32 = 171160;
pub const PJSIP_TLS_EINVMETHOD: u32 = 171161;
pub const PJSIP_TLS_ECACERT: u32 = 171162;
pub const PJSIP_TLS_ECERTFILE: u32 = 171163;
pub const PJSIP_TLS_EKEYFILE: u32 = 171164;
pub const PJSIP_TLS_ECIPHER: u32 = 171165;
pub const PJSIP_TLS_ECTX: u32 = 171166;
pub const PJSIP_TLS_ESSLCONN: u32 = 171167;
pub const PJSIP_TLS_ECONNECT: u32 = 171168;
pub const PJSIP_TLS_EACCEPT: u32 = 171169;
pub const PJSIP_TLS_ESEND: u32 = 171170;
pub const PJSIP_TLS_EREAD: u32 = 171171;
pub const PJSIP_TLS_ETIMEDOUT: u32 = 171172;
pub const PJSIP_TLS_ECERTVERIF: u32 = 171173;
pub const PJSIP_GENERIC_ARRAY_MAX_COUNT: u32 = 32;
pub const PJSIP_MAX_ACCEPT_COUNT: u32 = 32;
pub const PJSIP_MIN_CONTENT_LENGTH: u32 = 0;
pub const PJSIP_MIN_PORT: u32 = 0;
pub const PJSIP_MIN_TTL: u32 = 0;
pub const PJSIP_MIN_STATUS_CODE: u32 = 100;
pub const PJSIP_MAX_STATUS_CODE: u32 = 999;
pub const PJSIP_MIN_Q1000: u32 = 0;
pub const PJSIP_MIN_EXPIRES: u32 = 0;
pub const PJSIP_MIN_CSEQ: u32 = 0;
pub const PJSIP_MIN_RETRY_AFTER: u32 = 0;
pub const PJSIP_MD5STRLEN: u32 = 32;
pub const PJSIP_AUTH_SRV_IS_PROXY: u32 = 1;
pub const PJSIP_AKA_AKLEN: u32 = 6;
pub const PJSIP_AKA_AMFLEN: u32 = 2;
pub const PJSIP_AKA_AUTNLEN: u32 = 16;
pub const PJSIP_AKA_CKLEN: u32 = 16;
pub const PJSIP_AKA_IKLEN: u32 = 16;
pub const PJSIP_AKA_KLEN: u32 = 16;
pub const PJSIP_AKA_MACLEN: u32 = 8;
pub const PJSIP_AKA_OPLEN: u32 = 16;
pub const PJSIP_AKA_RANDLEN: u32 = 16;
pub const PJSIP_AKA_RESLEN: u32 = 8;
pub const PJSIP_AKA_SQNLEN: u32 = 6;
pub const PJSIP_REGC_MAX_CONTACT: u32 = 10;
pub const PJSIP_REGC_CONTACT_BUF_SIZE: u32 = 512;
pub const PJSIP_EVSUB_POOL_LEN: u32 = 4000;
pub const PJSIP_EVSUB_POOL_INC: u32 = 4000;
pub const PJSIP_MAX_ALLOW_EVENTS: u32 = 16;
pub const PJSIP_PRES_STATUS_MAX_INFO: u32 = 8;


pub const pjsip_hdr_e_PJSIP_H_ACCEPT: pjsip_hdr_e = 0;
pub const pjsip_hdr_e_PJSIP_H_ACCEPT_ENCODING_UNIMP: pjsip_hdr_e = 1;
pub const pjsip_hdr_e_PJSIP_H_ACCEPT_LANGUAGE_UNIMP: pjsip_hdr_e = 2;
pub const pjsip_hdr_e_PJSIP_H_ALERT_INFO_UNIMP: pjsip_hdr_e = 3;
pub const pjsip_hdr_e_PJSIP_H_ALLOW: pjsip_hdr_e = 4;
pub const pjsip_hdr_e_PJSIP_H_AUTHENTICATION_INFO_UNIMP: pjsip_hdr_e = 5;
pub const pjsip_hdr_e_PJSIP_H_AUTHORIZATION: pjsip_hdr_e = 6;
pub const pjsip_hdr_e_PJSIP_H_CALL_ID: pjsip_hdr_e = 7;
pub const pjsip_hdr_e_PJSIP_H_CALL_INFO_UNIMP: pjsip_hdr_e = 8;
pub const pjsip_hdr_e_PJSIP_H_CONTACT: pjsip_hdr_e = 9;
pub const pjsip_hdr_e_PJSIP_H_CONTENT_DISPOSITION_UNIMP: pjsip_hdr_e = 10;
pub const pjsip_hdr_e_PJSIP_H_CONTENT_ENCODING_UNIMP: pjsip_hdr_e = 11;
pub const pjsip_hdr_e_PJSIP_H_CONTENT_LANGUAGE_UNIMP: pjsip_hdr_e = 12;
pub const pjsip_hdr_e_PJSIP_H_CONTENT_LENGTH: pjsip_hdr_e = 13;
pub const pjsip_hdr_e_PJSIP_H_CONTENT_TYPE: pjsip_hdr_e = 14;
pub const pjsip_hdr_e_PJSIP_H_CSEQ: pjsip_hdr_e = 15;
pub const pjsip_hdr_e_PJSIP_H_DATE_UNIMP: pjsip_hdr_e = 16;
pub const pjsip_hdr_e_PJSIP_H_ERROR_INFO_UNIMP: pjsip_hdr_e = 17;
pub const pjsip_hdr_e_PJSIP_H_EXPIRES: pjsip_hdr_e = 18;
pub const pjsip_hdr_e_PJSIP_H_FROM: pjsip_hdr_e = 19;
pub const pjsip_hdr_e_PJSIP_H_IN_REPLY_TO_UNIMP: pjsip_hdr_e = 20;
pub const pjsip_hdr_e_PJSIP_H_MAX_FORWARDS: pjsip_hdr_e = 21;
pub const pjsip_hdr_e_PJSIP_H_MIME_VERSION_UNIMP: pjsip_hdr_e = 22;
pub const pjsip_hdr_e_PJSIP_H_MIN_EXPIRES: pjsip_hdr_e = 23;
pub const pjsip_hdr_e_PJSIP_H_ORGANIZATION_UNIMP: pjsip_hdr_e = 24;
pub const pjsip_hdr_e_PJSIP_H_PRIORITY_UNIMP: pjsip_hdr_e = 25;
pub const pjsip_hdr_e_PJSIP_H_PROXY_AUTHENTICATE: pjsip_hdr_e = 26;
pub const pjsip_hdr_e_PJSIP_H_PROXY_AUTHORIZATION: pjsip_hdr_e = 27;
pub const pjsip_hdr_e_PJSIP_H_PROXY_REQUIRE_UNIMP: pjsip_hdr_e = 28;
pub const pjsip_hdr_e_PJSIP_H_RECORD_ROUTE: pjsip_hdr_e = 29;
pub const pjsip_hdr_e_PJSIP_H_REPLY_TO_UNIMP: pjsip_hdr_e = 30;
pub const pjsip_hdr_e_PJSIP_H_REQUIRE: pjsip_hdr_e = 31;
pub const pjsip_hdr_e_PJSIP_H_RETRY_AFTER: pjsip_hdr_e = 32;
pub const pjsip_hdr_e_PJSIP_H_ROUTE: pjsip_hdr_e = 33;
pub const pjsip_hdr_e_PJSIP_H_SERVER_UNIMP: pjsip_hdr_e = 34;
pub const pjsip_hdr_e_PJSIP_H_SUBJECT_UNIMP: pjsip_hdr_e = 35;
pub const pjsip_hdr_e_PJSIP_H_SUPPORTED: pjsip_hdr_e = 36;
pub const pjsip_hdr_e_PJSIP_H_TIMESTAMP_UNIMP: pjsip_hdr_e = 37;
pub const pjsip_hdr_e_PJSIP_H_TO: pjsip_hdr_e = 38;
pub const pjsip_hdr_e_PJSIP_H_UNSUPPORTED: pjsip_hdr_e = 39;
pub const pjsip_hdr_e_PJSIP_H_USER_AGENT_UNIMP: pjsip_hdr_e = 40;
pub const pjsip_hdr_e_PJSIP_H_VIA: pjsip_hdr_e = 41;
pub const pjsip_hdr_e_PJSIP_H_WARNING_UNIMP: pjsip_hdr_e = 42;
pub const pjsip_hdr_e_PJSIP_H_WWW_AUTHENTICATE: pjsip_hdr_e = 43;
pub const pjsip_hdr_e_PJSIP_H_OTHER: pjsip_hdr_e = 44;
pub type pjsip_hdr_e = c_uint;

pub const pjsip_uri_context_e_PJSIP_URI_IN_REQ_URI: pjsip_uri_context_e = 0;
pub const pjsip_uri_context_e_PJSIP_URI_IN_FROMTO_HDR: pjsip_uri_context_e = 1;
pub const pjsip_uri_context_e_PJSIP_URI_IN_CONTACT_HDR: pjsip_uri_context_e = 2;
pub const pjsip_uri_context_e_PJSIP_URI_IN_ROUTING_HDR: pjsip_uri_context_e = 3;
pub const pjsip_uri_context_e_PJSIP_URI_IN_OTHER: pjsip_uri_context_e = 4;
pub type pjsip_uri_context_e = c_uint;

pub const pjsip_method_e_PJSIP_INVITE_METHOD: pjsip_method_e = 0;
pub const pjsip_method_e_PJSIP_CANCEL_METHOD: pjsip_method_e = 1;
pub const pjsip_method_e_PJSIP_ACK_METHOD: pjsip_method_e = 2;
pub const pjsip_method_e_PJSIP_BYE_METHOD: pjsip_method_e = 3;
pub const pjsip_method_e_PJSIP_REGISTER_METHOD: pjsip_method_e = 4;
pub const pjsip_method_e_PJSIP_OPTIONS_METHOD: pjsip_method_e = 5;
pub const pjsip_method_e_PJSIP_OTHER_METHOD: pjsip_method_e = 6;
pub type pjsip_method_e = c_uint;

pub const pjsip_transport_type_e_PJSIP_TRANSPORT_UNSPECIFIED: pjsip_transport_type_e = 0;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_UDP: pjsip_transport_type_e = 1;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_TCP: pjsip_transport_type_e = 2;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_TLS: pjsip_transport_type_e = 3;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_DTLS: pjsip_transport_type_e = 4;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_SCTP: pjsip_transport_type_e = 5;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_LOOP: pjsip_transport_type_e = 6;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_LOOP_DGRAM: pjsip_transport_type_e = 7;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_START_OTHER: pjsip_transport_type_e = 8;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_IPV6: pjsip_transport_type_e = 128;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_UDP6: pjsip_transport_type_e = 129;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_TCP6: pjsip_transport_type_e = 130;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_TLS6: pjsip_transport_type_e = 131;
pub const pjsip_transport_type_e_PJSIP_TRANSPORT_DTLS6: pjsip_transport_type_e = 132;
pub type pjsip_transport_type_e = c_uint;

pub const pjsip_status_code_PJSIP_SC_NULL: pjsip_status_code = 0;
pub const pjsip_status_code_PJSIP_SC_TRYING: pjsip_status_code = 100;
pub const pjsip_status_code_PJSIP_SC_RINGING: pjsip_status_code = 180;
pub const pjsip_status_code_PJSIP_SC_CALL_BEING_FORWARDED: pjsip_status_code = 181;
pub const pjsip_status_code_PJSIP_SC_QUEUED: pjsip_status_code = 182;
pub const pjsip_status_code_PJSIP_SC_PROGRESS: pjsip_status_code = 183;
pub const pjsip_status_code_PJSIP_SC_EARLY_DIALOG_TERMINATED: pjsip_status_code = 199;
pub const pjsip_status_code_PJSIP_SC_OK: pjsip_status_code = 200;
pub const pjsip_status_code_PJSIP_SC_ACCEPTED: pjsip_status_code = 202;
pub const pjsip_status_code_PJSIP_SC_NO_NOTIFICATION: pjsip_status_code = 204;
pub const pjsip_status_code_PJSIP_SC_MULTIPLE_CHOICES: pjsip_status_code = 300;
pub const pjsip_status_code_PJSIP_SC_MOVED_PERMANENTLY: pjsip_status_code = 301;
pub const pjsip_status_code_PJSIP_SC_MOVED_TEMPORARILY: pjsip_status_code = 302;
pub const pjsip_status_code_PJSIP_SC_USE_PROXY: pjsip_status_code = 305;
pub const pjsip_status_code_PJSIP_SC_ALTERNATIVE_SERVICE: pjsip_status_code = 380;
pub const pjsip_status_code_PJSIP_SC_BAD_REQUEST: pjsip_status_code = 400;
pub const pjsip_status_code_PJSIP_SC_UNAUTHORIZED: pjsip_status_code = 401;
pub const pjsip_status_code_PJSIP_SC_PAYMENT_REQUIRED: pjsip_status_code = 402;
pub const pjsip_status_code_PJSIP_SC_FORBIDDEN: pjsip_status_code = 403;
pub const pjsip_status_code_PJSIP_SC_NOT_FOUND: pjsip_status_code = 404;
pub const pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED: pjsip_status_code = 405;
pub const pjsip_status_code_PJSIP_SC_NOT_ACCEPTABLE: pjsip_status_code = 406;
pub const pjsip_status_code_PJSIP_SC_PROXY_AUTHENTICATION_REQUIRED: pjsip_status_code = 407;
pub const pjsip_status_code_PJSIP_SC_REQUEST_TIMEOUT: pjsip_status_code = 408;
pub const pjsip_status_code_PJSIP_SC_CONFLICT: pjsip_status_code = 409;
pub const pjsip_status_code_PJSIP_SC_GONE: pjsip_status_code = 410;
pub const pjsip_status_code_PJSIP_SC_LENGTH_REQUIRED: pjsip_status_code = 411;
pub const pjsip_status_code_PJSIP_SC_CONDITIONAL_REQUEST_FAILED: pjsip_status_code = 412;
pub const pjsip_status_code_PJSIP_SC_REQUEST_ENTITY_TOO_LARGE: pjsip_status_code = 413;
pub const pjsip_status_code_PJSIP_SC_REQUEST_URI_TOO_LONG: pjsip_status_code = 414;
pub const pjsip_status_code_PJSIP_SC_UNSUPPORTED_MEDIA_TYPE: pjsip_status_code = 415;
pub const pjsip_status_code_PJSIP_SC_UNSUPPORTED_URI_SCHEME: pjsip_status_code = 416;
pub const pjsip_status_code_PJSIP_SC_UNKNOWN_RESOURCE_PRIORITY: pjsip_status_code = 417;
pub const pjsip_status_code_PJSIP_SC_BAD_EXTENSION: pjsip_status_code = 420;
pub const pjsip_status_code_PJSIP_SC_EXTENSION_REQUIRED: pjsip_status_code = 421;
pub const pjsip_status_code_PJSIP_SC_SESSION_TIMER_TOO_SMALL: pjsip_status_code = 422;
pub const pjsip_status_code_PJSIP_SC_INTERVAL_TOO_BRIEF: pjsip_status_code = 423;
pub const pjsip_status_code_PJSIP_SC_BAD_LOCATION_INFORMATION: pjsip_status_code = 424;
pub const pjsip_status_code_PJSIP_SC_USE_IDENTITY_HEADER: pjsip_status_code = 428;
pub const pjsip_status_code_PJSIP_SC_PROVIDE_REFERRER_HEADER: pjsip_status_code = 429;
pub const pjsip_status_code_PJSIP_SC_FLOW_FAILED: pjsip_status_code = 430;
pub const pjsip_status_code_PJSIP_SC_ANONIMITY_DISALLOWED: pjsip_status_code = 433;
pub const pjsip_status_code_PJSIP_SC_BAD_IDENTITY_INFO: pjsip_status_code = 436;
pub const pjsip_status_code_PJSIP_SC_UNSUPPORTED_CERTIFICATE: pjsip_status_code = 437;
pub const pjsip_status_code_PJSIP_SC_INVALID_IDENTITY_HEADER: pjsip_status_code = 438;
pub const pjsip_status_code_PJSIP_SC_FIRST_HOP_LACKS_OUTBOUND_SUPPORT: pjsip_status_code = 439;
pub const pjsip_status_code_PJSIP_SC_MAX_BREADTH_EXCEEDED: pjsip_status_code = 440;
pub const pjsip_status_code_PJSIP_SC_BAD_INFO_PACKAGE: pjsip_status_code = 469;
pub const pjsip_status_code_PJSIP_SC_CONSENT_NEEDED: pjsip_status_code = 470;
pub const pjsip_status_code_PJSIP_SC_TEMPORARILY_UNAVAILABLE: pjsip_status_code = 480;
pub const pjsip_status_code_PJSIP_SC_CALL_TSX_DOES_NOT_EXIST: pjsip_status_code = 481;
pub const pjsip_status_code_PJSIP_SC_LOOP_DETECTED: pjsip_status_code = 482;
pub const pjsip_status_code_PJSIP_SC_TOO_MANY_HOPS: pjsip_status_code = 483;
pub const pjsip_status_code_PJSIP_SC_ADDRESS_INCOMPLETE: pjsip_status_code = 484;
pub const pjsip_status_code_PJSIP_AC_AMBIGUOUS: pjsip_status_code = 485;
pub const pjsip_status_code_PJSIP_SC_BUSY_HERE: pjsip_status_code = 486;
pub const pjsip_status_code_PJSIP_SC_REQUEST_TERMINATED: pjsip_status_code = 487;
pub const pjsip_status_code_PJSIP_SC_NOT_ACCEPTABLE_HERE: pjsip_status_code = 488;
pub const pjsip_status_code_PJSIP_SC_BAD_EVENT: pjsip_status_code = 489;
pub const pjsip_status_code_PJSIP_SC_REQUEST_UPDATED: pjsip_status_code = 490;
pub const pjsip_status_code_PJSIP_SC_REQUEST_PENDING: pjsip_status_code = 491;
pub const pjsip_status_code_PJSIP_SC_UNDECIPHERABLE: pjsip_status_code = 493;
pub const pjsip_status_code_PJSIP_SC_SECURITY_AGREEMENT_NEEDED: pjsip_status_code = 494;
pub const pjsip_status_code_PJSIP_SC_INTERNAL_SERVER_ERROR: pjsip_status_code = 500;
pub const pjsip_status_code_PJSIP_SC_NOT_IMPLEMENTED: pjsip_status_code = 501;
pub const pjsip_status_code_PJSIP_SC_BAD_GATEWAY: pjsip_status_code = 502;
pub const pjsip_status_code_PJSIP_SC_SERVICE_UNAVAILABLE: pjsip_status_code = 503;
pub const pjsip_status_code_PJSIP_SC_SERVER_TIMEOUT: pjsip_status_code = 504;
pub const pjsip_status_code_PJSIP_SC_VERSION_NOT_SUPPORTED: pjsip_status_code = 505;
pub const pjsip_status_code_PJSIP_SC_MESSAGE_TOO_LARGE: pjsip_status_code = 513;
pub const pjsip_status_code_PJSIP_SC_PUSH_NOTIFICATION_SERVICE_NOT_SUPPORTED: pjsip_status_code = 555;
pub const pjsip_status_code_PJSIP_SC_PRECONDITION_FAILURE: pjsip_status_code = 580;
pub const pjsip_status_code_PJSIP_SC_BUSY_EVERYWHERE: pjsip_status_code = 600;
pub const pjsip_status_code_PJSIP_SC_DECLINE: pjsip_status_code = 603;
pub const pjsip_status_code_PJSIP_SC_DOES_NOT_EXIST_ANYWHERE: pjsip_status_code = 604;
pub const pjsip_status_code_PJSIP_SC_NOT_ACCEPTABLE_ANYWHERE: pjsip_status_code = 606;
pub const pjsip_status_code_PJSIP_SC_UNWANTED: pjsip_status_code = 607;
pub const pjsip_status_code_PJSIP_SC_REJECTED: pjsip_status_code = 608;
pub const pjsip_status_code_PJSIP_SC_TSX_TIMEOUT: pjsip_status_code = 408;
pub const pjsip_status_code_PJSIP_SC_TSX_TRANSPORT_ERROR: pjsip_status_code = 503;
pub const pjsip_status_code_PJSIP_SC__force_32bit: pjsip_status_code = 2147483647;
pub type pjsip_status_code = c_uint;

pub type pjsip_user_agent = pjsip_module;
pub const pjsip_role_e_PJSIP_ROLE_UAC: pjsip_role_e = 0;
pub const pjsip_role_e_PJSIP_ROLE_UAS: pjsip_role_e = 1;
pub const pjsip_role_e_PJSIP_UAC_ROLE: pjsip_role_e = 0;
pub const pjsip_role_e_PJSIP_UAS_ROLE: pjsip_role_e = 1;
pub type pjsip_role_e = c_uint;

pub type pjsip_accept_hdr = pjsip_generic_array_hdr;
pub type pjsip_allow_hdr = pjsip_generic_array_hdr;

pub type pjsip_warning_hdr = pjsip_generic_string_hdr;
pub type pjsip_accept_encoding_hdr = pjsip_generic_string_hdr;
pub type pjsip_accept_lang_hdr = pjsip_generic_string_hdr;
pub type pjsip_alert_info_hdr = pjsip_generic_string_hdr;
pub type pjsip_auth_info_hdr = pjsip_generic_string_hdr;
pub type pjsip_call_info_hdr = pjsip_generic_string_hdr;
pub type pjsip_content_disposition_hdr = pjsip_generic_string_hdr;
pub type pjsip_content_encoding_hdr = pjsip_generic_string_hdr;
pub type pjsip_content_lang_hdr = pjsip_generic_string_hdr;
pub type pjsip_date_hdr = pjsip_generic_string_hdr;
pub type pjsip_err_info_hdr = pjsip_generic_string_hdr;
pub type pjsip_in_reply_to_hdr = pjsip_generic_string_hdr;
pub type pjsip_mime_version_hdr = pjsip_generic_string_hdr;
pub type pjsip_organization_hdr = pjsip_generic_string_hdr;
pub type pjsip_priority_hdr = pjsip_generic_string_hdr;
pub type pjsip_proxy_require_hdr = pjsip_generic_string_hdr;
pub type pjsip_reply_to_hdr = pjsip_generic_string_hdr;
pub type pjsip_server_hdr = pjsip_generic_string_hdr;
pub type pjsip_subject_hdr = pjsip_generic_string_hdr;
pub type pjsip_timestamp_hdr = pjsip_generic_string_hdr;
pub type pjsip_user_agent_hdr = pjsip_generic_string_hdr;
pub type pjsip_expires_hdr = pjsip_generic_int_hdr;
pub type pjsip_from_hdr = pjsip_fromto_hdr;
pub type pjsip_to_hdr = pjsip_fromto_hdr;
pub type pjsip_max_fwd_hdr = pjsip_generic_int_hdr;
pub type pjsip_min_expires_hdr = pjsip_generic_int_hdr;
pub type pjsip_rr_hdr = pjsip_routing_hdr;
pub type pjsip_route_hdr = pjsip_routing_hdr;
pub type pjsip_require_hdr = pjsip_generic_array_hdr;

pub type pjsip_supported_hdr = pjsip_generic_array_hdr;
pub type pjsip_unsupported_hdr = pjsip_generic_array_hdr;
pub const PJSIP_PARSE_URI_AS_NAMEADDR: c_uint = 1;
pub const PJSIP_PARSE_URI_IN_FROM_TO_HDR: c_uint = 2;
pub const PJSIP_PARSE_REMOVE_QUOTE: c_uint = 1;

pub type pjsip_parse_hdr_func = Option<unsafe extern "C" fn(context: *mut pjsip_parse_ctx) -> *mut pjsip_hdr>;
pub type pjsip_parse_uri_func = Option<
unsafe extern "C" fn(
        scanner: *mut pj_scanner,
        pool: *mut pj_pool_t,
        parse_params: pj_bool_t,
    ) -> *mut c_void,
>;

pub const pjsip_event_id_e_PJSIP_EVENT_UNKNOWN: pjsip_event_id_e = 0;
pub const pjsip_event_id_e_PJSIP_EVENT_TIMER: pjsip_event_id_e = 1;
pub const pjsip_event_id_e_PJSIP_EVENT_TX_MSG: pjsip_event_id_e = 2;
pub const pjsip_event_id_e_PJSIP_EVENT_RX_MSG: pjsip_event_id_e = 3;
pub const pjsip_event_id_e_PJSIP_EVENT_TRANSPORT_ERROR: pjsip_event_id_e = 4;
pub const pjsip_event_id_e_PJSIP_EVENT_TSX_STATE: pjsip_event_id_e = 5;
pub const pjsip_event_id_e_PJSIP_EVENT_USER: pjsip_event_id_e = 6;
pub type pjsip_event_id_e = c_uint;

pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_TRANSPORT_LAYER: pjsip_module_priority = 8;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_TSX_LAYER: pjsip_module_priority = 16;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_UA_PROXY_LAYER: pjsip_module_priority = 32;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_DIALOG_USAGE: pjsip_module_priority = 48;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_APPLICATION: pjsip_module_priority = 64;
pub type pjsip_module_priority = c_uint;

pub const pjsip_transport_flags_e_PJSIP_TRANSPORT_RELIABLE: pjsip_transport_flags_e = 1;
pub const pjsip_transport_flags_e_PJSIP_TRANSPORT_SECURE: pjsip_transport_flags_e = 2;
pub const pjsip_transport_flags_e_PJSIP_TRANSPORT_DATAGRAM: pjsip_transport_flags_e = 4;
pub type pjsip_transport_flags_e = c_uint;

pub const pjsip_tpselector_type_PJSIP_TPSELECTOR_NONE: pjsip_tpselector_type = 0;
pub const pjsip_tpselector_type_PJSIP_TPSELECTOR_TRANSPORT: pjsip_tpselector_type = 1;
pub const pjsip_tpselector_type_PJSIP_TPSELECTOR_LISTENER: pjsip_tpselector_type = 2;
pub type pjsip_tpselector_type = c_uint;

pub const pjsip_transport_dir_PJSIP_TP_DIR_NONE: pjsip_transport_dir = 0;
pub const pjsip_transport_dir_PJSIP_TP_DIR_OUTGOING: pjsip_transport_dir = 1;
pub const pjsip_transport_dir_PJSIP_TP_DIR_INCOMING: pjsip_transport_dir = 2;
pub type pjsip_transport_dir = c_uint;

pub const pjsip_transport_state_PJSIP_TP_STATE_CONNECTED: pjsip_transport_state = 0;
pub const pjsip_transport_state_PJSIP_TP_STATE_DISCONNECTED: pjsip_transport_state = 1;
pub const pjsip_transport_state_PJSIP_TP_STATE_SHUTDOWN: pjsip_transport_state = 2;
pub const pjsip_transport_state_PJSIP_TP_STATE_DESTROY: pjsip_transport_state = 3;
pub type pjsip_transport_state = c_uint;
pub type pjsip_tp_state_listener_key = c_void;

pub const pjsip_redirect_op_PJSIP_REDIRECT_REJECT: pjsip_redirect_op = 0;
pub const pjsip_redirect_op_PJSIP_REDIRECT_ACCEPT: pjsip_redirect_op = 1;
pub const pjsip_redirect_op_PJSIP_REDIRECT_ACCEPT_REPLACE: pjsip_redirect_op = 2;
pub const pjsip_redirect_op_PJSIP_REDIRECT_PENDING: pjsip_redirect_op = 3;
pub const pjsip_redirect_op_PJSIP_REDIRECT_STOP: pjsip_redirect_op = 4;
pub type pjsip_redirect_op = c_uint;

pub const PJSIP_UDP_TRANSPORT_KEEP_SOCKET: c_uint = 1;
pub const PJSIP_UDP_TRANSPORT_DESTROY_SOCKET: c_uint = 2;

pub type pjsip_proxy_authenticate_hdr = pjsip_www_authenticate_hdr;
pub const pjsip_cred_data_type_PJSIP_CRED_DATA_PLAIN_PASSWD: pjsip_cred_data_type = 0;
pub const pjsip_cred_data_type_PJSIP_CRED_DATA_DIGEST: pjsip_cred_data_type = 1;
pub const pjsip_cred_data_type_PJSIP_CRED_DATA_EXT_AKA: pjsip_cred_data_type = 16;
pub type pjsip_cred_data_type = c_uint;

pub const pjsip_auth_qop_type_PJSIP_AUTH_QOP_NONE: pjsip_auth_qop_type = 0;
pub const pjsip_auth_qop_type_PJSIP_AUTH_QOP_AUTH: pjsip_auth_qop_type = 1;
pub const pjsip_auth_qop_type_PJSIP_AUTH_QOP_AUTH_INT: pjsip_auth_qop_type = 2;
pub const pjsip_auth_qop_type_PJSIP_AUTH_QOP_UNKNOWN: pjsip_auth_qop_type = 3;
pub type pjsip_auth_qop_type = c_uint;

pub type pjsip_cred_cb = Option<
    unsafe extern "C" fn(
        pool: *mut pj_pool_t,
        chal: *const pjsip_digest_challenge,
        cred: *const pjsip_cred_info,
        method: *const pj_str_t,
        auth: *mut pjsip_digest_credential,
    ) -> pj_status_t,
>;

pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_NULL: pjsip_tsx_state_e = 0;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_CALLING: pjsip_tsx_state_e = 1;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_TRYING: pjsip_tsx_state_e = 2;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_PROCEEDING: pjsip_tsx_state_e = 3;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_COMPLETED: pjsip_tsx_state_e = 4;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_CONFIRMED: pjsip_tsx_state_e = 5;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_TERMINATED: pjsip_tsx_state_e = 6;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_DESTROYED: pjsip_tsx_state_e = 7;
pub const pjsip_tsx_state_e_PJSIP_TSX_STATE_MAX: pjsip_tsx_state_e = 8;
pub type pjsip_tsx_state_e = c_uint;

pub const pjsip_dialog_state_PJSIP_DIALOG_STATE_NULL: pjsip_dialog_state = 0;
pub const pjsip_dialog_state_PJSIP_DIALOG_STATE_ESTABLISHED: pjsip_dialog_state = 1;
pub type pjsip_dialog_state = c_uint;
pub const pjsip_dialog_cap_status_PJSIP_DIALOG_CAP_UNSUPPORTED: pjsip_dialog_cap_status = 0;
pub const pjsip_dialog_cap_status_PJSIP_DIALOG_CAP_SUPPORTED: pjsip_dialog_cap_status = 1;
pub const pjsip_dialog_cap_status_PJSIP_DIALOG_CAP_UNKNOWN: pjsip_dialog_cap_status = 2;
pub type pjsip_dialog_cap_status = c_uint;


pub const pjsip_msg_type_e_PJSIP_REQUEST_MSG: pjsip_msg_type_e = 0;
pub const pjsip_msg_type_e_PJSIP_RESPONSE_MSG: pjsip_msg_type_e = 1;
pub type pjsip_msg_type_e = c_uint;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_param {
    pub prev: *mut pjsip_param,
    pub next: *mut pjsip_param,
    pub name: pj_str_t,
    pub value: pj_str_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_endpoint {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_media_type {
    pub type_: pj_str_t,
    pub subtype: pj_str_t,
    pub param: pjsip_param,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_msg_body {
    pub content_type: pjsip_media_type,
    pub data: *mut c_void,
    pub len: c_uint,
    pub print_body: Option<
        unsafe extern "C" fn(
            msg_body: *mut pjsip_msg_body,
            buf: *mut c_char,
            size: pj_size_t,
        ) -> c_int,
        >,
    pub clone_data: Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            data: *const c_void,
            len: c_uint,
        ) -> *mut c_void,
        >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_uri_vptr {
    pub p_get_scheme: Option<
        unsafe extern "C" fn(uri: *const c_void) -> *const pj_str_t,
    >,
    pub p_get_uri: Option<
        unsafe extern "C" fn(uri: *mut c_void) -> *mut c_void,
    >,
    pub p_print: Option<
        unsafe extern "C" fn(
            context: pjsip_uri_context_e,
            uri: *const c_void,
            buf: *mut c_char,
            size: pj_size_t,
        ) -> pj_ssize_t,
    >,
    pub p_compare: Option<
        unsafe extern "C" fn(
            context: pjsip_uri_context_e,
            uri1: *const c_void,
            uri2: *const c_void,
        ) -> pj_status_t,
    >,
    pub p_clone: Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            uri: *const c_void,
        ) -> *mut c_void,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_uri {
    pub vptr: *mut pjsip_uri_vptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_sip_uri {
    pub vptr: *mut pjsip_uri_vptr,
    pub user: pj_str_t,
    pub passwd: pj_str_t,
    pub host: pj_str_t,
    pub port: c_int,
    pub user_param: pj_str_t,
    pub method_param: pj_str_t,
    pub transport_param: pj_str_t,
    pub ttl_param: c_int,
    pub lr_param: c_int,
    pub maddr_param: pj_str_t,
    pub other_param: pjsip_param,
    pub header_param: pjsip_param,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_name_addr {
    pub vptr: *mut pjsip_uri_vptr,
    pub display: pj_str_t,
    pub uri: *mut pjsip_uri,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_other_uri {
    pub vptr: *mut pjsip_uri_vptr,
    pub scheme: pj_str_t,
    pub content: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tel_uri {
    pub vptr: *mut pjsip_uri_vptr,
    pub number: pj_str_t,
    pub context: pj_str_t,
    pub ext_param: pj_str_t,
    pub isub_param: pj_str_t,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_method {
    pub id: pjsip_method_e,
    pub name: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t {
    pub endpt: pjsip_cfg_t__bindgen_ty_1,
    pub tsx: pjsip_cfg_t__bindgen_ty_2,
    pub regc: pjsip_cfg_t__bindgen_ty_3,
    pub tcp: pjsip_cfg_t__bindgen_ty_4,
    pub tls: pjsip_cfg_t__bindgen_ty_5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t__bindgen_ty_1 {
    pub allow_port_in_fromto_hdr: pj_bool_t,
    pub accept_replace_in_early_state: pj_bool_t,
    pub allow_tx_hash_in_uri: pj_bool_t,
    pub disable_rport: pj_bool_t,
    pub disable_tcp_switch: pj_bool_t,
    pub disable_tls_switch: pj_bool_t,
    pub follow_early_media_fork: pj_bool_t,
    pub req_has_via_alias: pj_bool_t,
    pub resolve_hostname_to_get_interface: pj_bool_t,
    pub disable_secure_dlg_check: pj_bool_t,
    pub use_compact_form: pj_bool_t,
    pub accept_multiple_sdp_answers: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t__bindgen_ty_2 {
    pub max_count: c_uint,
    pub t1: c_uint,
    pub t2: c_uint,
    pub t4: c_uint,
    pub td: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t__bindgen_ty_3 {
    pub check_contact: pj_bool_t,
    pub add_xuid_param: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t__bindgen_ty_4 {
    pub keep_alive_interval: c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t__bindgen_ty_5 {
    pub keep_alive_interval: c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tpmgr {
    _unused: [u8; 0],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_resolver_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_buffer {
    pub start: *mut c_char,
    pub cur: *mut c_char,
    pub end: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_host_port {
    pub host: pj_str_t,
    pub port: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_host_info {
    pub flag: c_uint,
    pub type_: pjsip_transport_type_e,
    pub addr: pjsip_host_port,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_hdr_vptr {
    pub clone: Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            hdr: *const c_void,
        ) -> *mut c_void,
    >,
    pub shallow_clone: Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            hdr: *const c_void,
        ) -> *mut c_void,
    >,
    pub print_on: Option<
        unsafe extern "C" fn(
            hdr: *mut c_void,
            buf: *mut c_char,
            len: pj_size_t,
        ) -> c_int,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_hdr {
    pub prev: *mut pjsip_hdr,
    pub next: *mut pjsip_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_request_line {
    pub method: pjsip_method,
    pub uri: *mut pjsip_uri,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_status_line {
    pub code: c_int,
    pub reason: pj_str_t,
}



#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_msg__bindgen_ty_1 {
    pub req: pjsip_request_line,
    pub status: pjsip_status_line,
    _bindgen_union_align: [u64; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_generic_string_hdr {
    pub prev: *mut pjsip_generic_string_hdr,
    pub next: *mut pjsip_generic_string_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub hvalue: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_generic_int_hdr {
    pub prev: *mut pjsip_generic_int_hdr,
    pub next: *mut pjsip_generic_int_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub ivalue: pj_uint32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_generic_array_hdr {
    pub prev: *mut pjsip_generic_array_hdr,
    pub next: *mut pjsip_generic_array_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub count: c_uint,
    pub values: [pj_str_t; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cid_hdr {
    pub prev: *mut pjsip_cid_hdr,
    pub next: *mut pjsip_cid_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub id: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_clen_hdr {
    pub prev: *mut pjsip_clen_hdr,
    pub next: *mut pjsip_clen_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub len: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cseq_hdr {
    pub prev: *mut pjsip_cseq_hdr,
    pub next: *mut pjsip_cseq_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub cseq: pj_int32_t,
    pub method: pjsip_method,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_contact_hdr {
    pub prev: *mut pjsip_contact_hdr,
    pub next: *mut pjsip_contact_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub star: c_int,
    pub uri: *mut pjsip_uri,
    pub q1000: c_int,
    pub expires: pj_uint32_t,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_ctype_hdr {
    pub prev: *mut pjsip_ctype_hdr,
    pub next: *mut pjsip_ctype_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub media: pjsip_media_type,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_fromto_hdr {
    pub prev: *mut pjsip_fromto_hdr,
    pub next: *mut pjsip_fromto_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub uri: *mut pjsip_uri,
    pub tag: pj_str_t,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_routing_hdr {
    pub prev: *mut pjsip_routing_hdr,
    pub next: *mut pjsip_routing_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub name_addr: pjsip_name_addr,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_retry_after_hdr {
    pub prev: *mut pjsip_retry_after_hdr,
    pub next: *mut pjsip_retry_after_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub ivalue: pj_int32_t,
    pub param: pjsip_param,
    pub comment: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_via_hdr {
    pub prev: *mut pjsip_via_hdr,
    pub next: *mut pjsip_via_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub transport: pj_str_t,
    pub sent_by: pjsip_host_port,
    pub ttl_param: c_int,
    pub rport_param: c_int,
    pub maddr_param: pj_str_t,
    pub recvd_param: pj_str_t,
    pub branch_param: pj_str_t,
    pub other_param: pjsip_param,
    pub comment: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_multipart_part {
    pub prev: *mut pjsip_multipart_part,
    pub next: *mut pjsip_multipart_part,
    pub hdr: pjsip_hdr,
    pub body: *mut pjsip_msg_body,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_parser_err_report {
    pub prev: *mut pjsip_parser_err_report,
    pub next: *mut pjsip_parser_err_report,
    pub except_code: c_int,
    pub line: c_int,
    pub col: c_int,
    pub hname: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_parse_ctx {
    pub scanner: *mut pj_scanner,
    pub pool: *mut pj_pool_t,
    pub rdata: *mut pjsip_rx_data,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_parser_const_t {
    pub pjsip_USER_STR: pj_str_t,
    pub pjsip_METHOD_STR: pj_str_t,
    pub pjsip_TRANSPORT_STR: pj_str_t,
    pub pjsip_MADDR_STR: pj_str_t,
    pub pjsip_LR_STR: pj_str_t,
    pub pjsip_SIP_STR: pj_str_t,
    pub pjsip_SIPS_STR: pj_str_t,
    pub pjsip_TEL_STR: pj_str_t,
    pub pjsip_BRANCH_STR: pj_str_t,
    pub pjsip_TTL_STR: pj_str_t,
    pub pjsip_RECEIVED_STR: pj_str_t,
    pub pjsip_Q_STR: pj_str_t,
    pub pjsip_EXPIRES_STR: pj_str_t,
    pub pjsip_TAG_STR: pj_str_t,
    pub pjsip_RPORT_STR: pj_str_t,
    pub pjsip_HOST_SPEC: pj_cis_t,
    pub pjsip_DIGIT_SPEC: pj_cis_t,
    pub pjsip_ALPHA_SPEC: pj_cis_t,
    pub pjsip_ALNUM_SPEC: pj_cis_t,
    pub pjsip_TOKEN_SPEC: pj_cis_t,
    pub pjsip_TOKEN_SPEC_ESC: pj_cis_t,
    pub pjsip_VIA_PARAM_SPEC: pj_cis_t,
    pub pjsip_VIA_PARAM_SPEC_ESC: pj_cis_t,
    pub pjsip_HEX_SPEC: pj_cis_t,
    pub pjsip_PARAM_CHAR_SPEC: pj_cis_t,
    pub pjsip_PARAM_CHAR_SPEC_ESC: pj_cis_t,
    pub pjsip_HDR_CHAR_SPEC: pj_cis_t,
    pub pjsip_HDR_CHAR_SPEC_ESC: pj_cis_t,
    pub pjsip_PROBE_USER_HOST_SPEC: pj_cis_t,
    pub pjsip_PASSWD_SPEC: pj_cis_t,
    pub pjsip_PASSWD_SPEC_ESC: pj_cis_t,
    pub pjsip_USER_SPEC: pj_cis_t,
    pub pjsip_USER_SPEC_ESC: pj_cis_t,
    pub pjsip_USER_SPEC_LENIENT: pj_cis_t,
    pub pjsip_USER_SPEC_LENIENT_ESC: pj_cis_t,
    pub pjsip_NOT_NEWLINE: pj_cis_t,
    pub pjsip_NOT_COMMA_OR_NEWLINE: pj_cis_t,
    pub pjsip_DISPLAY_SPEC: pj_cis_t,
    pub pjsip_OTHER_URI_CONTENT: pj_cis_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_event {
    pub prev: *mut pjsip_event,
    pub next: *mut pjsip_event,
    pub type_: pjsip_event_id_e,
    pub body: pjsip_event__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_event__bindgen_ty_1 {
    pub timer: pjsip_event__bindgen_ty_1__bindgen_ty_1,
    pub tsx_state: pjsip_event__bindgen_ty_1__bindgen_ty_2,
    pub tx_msg: pjsip_event__bindgen_ty_1__bindgen_ty_3,
    pub tx_error: pjsip_event__bindgen_ty_1__bindgen_ty_4,
    pub rx_msg: pjsip_event__bindgen_ty_1__bindgen_ty_5,
    pub user: pjsip_event__bindgen_ty_1__bindgen_ty_6,
    _bindgen_union_align: [u64; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_event__bindgen_ty_1__bindgen_ty_1 {
    pub entry: *mut pj_timer_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_event__bindgen_ty_1__bindgen_ty_2 {
    pub src: pjsip_event__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1,
    pub tsx: *mut pjsip_transaction,
    pub prev_state: c_int,
    pub type_: pjsip_event_id_e,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_event__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    pub rdata: *mut pjsip_rx_data,
    pub tdata: *mut pjsip_tx_data,
    pub timer: *mut pj_timer_entry,
    pub status: pj_status_t,
    pub data: *mut c_void,
    _bindgen_union_align: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_event__bindgen_ty_1__bindgen_ty_3 {
    pub tdata: *mut pjsip_tx_data,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_event__bindgen_ty_1__bindgen_ty_4 {
    pub tdata: *mut pjsip_tx_data,
    pub tsx: *mut pjsip_transaction,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_event__bindgen_ty_1__bindgen_ty_5 {
    pub rdata: *mut pjsip_rx_data,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_event__bindgen_ty_1__bindgen_ty_6 {
    pub user1: *mut c_void,
    pub user2: *mut c_void,
    pub user3: *mut c_void,
    pub user4: *mut c_void,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_module {
    pub prev: *mut pjsip_module,
    pub next: *mut pjsip_module,
    pub name: pj_str_t,
    pub id: c_int,
    pub priority: c_int,
    pub load:
        Option<unsafe extern "C" fn(endpt: *mut pjsip_endpoint) -> pj_status_t>,
    pub start: Option<unsafe extern "C" fn() -> pj_status_t>,
    pub stop: Option<unsafe extern "C" fn() -> pj_status_t>,
    pub unload: Option<unsafe extern "C" fn() -> pj_status_t>,
    pub on_rx_request:
        Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>,
    pub on_rx_response:
        Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>,
    pub on_tx_request:
        Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>,
    pub on_tx_response:
        Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>,
    pub on_tsx_state: Option<
        unsafe extern "C" fn(tsx: *mut pjsip_transaction, event: *mut pjsip_event),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_server_addresses {
    pub count: c_uint,
    pub entry: [pjsip_server_addresses__bindgen_ty_1; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_server_addresses__bindgen_ty_1 {
    pub type_: pjsip_transport_type_e,
    pub priority: c_uint,
    pub weight: c_uint,
    pub addr: pj_sockaddr,
    pub addr_len: c_int,
}
pub type pjsip_resolver_callback = Option<
    unsafe extern "C" fn(
        status: pj_status_t,
        token: *mut c_void,
        addr: *const pjsip_server_addresses,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_ext_resolver {
    pub resolve: Option<
        unsafe extern "C" fn(
            resolver: *mut pjsip_resolver_t,
            pool: *mut pj_pool_t,
            target: *const pjsip_host_info,
            token: *mut c_void,
            cb: pjsip_resolver_callback,
        ),
    >,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_tpselector {
    pub type_: pjsip_tpselector_type,
    pub disable_connection_reuse: pj_bool_t,
    pub u: pjsip_tpselector__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_tpselector__bindgen_ty_1 {
    pub transport: *mut pjsip_transport,
    pub listener: *mut pjsip_tpfactory,
    pub ptr: *mut c_void,
    _bindgen_union_align: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_rx_data_op_key {
    pub op_key: pj_ioqueue_op_key_t,
    pub rdata: *mut pjsip_rx_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_rx_data {
    pub tp_info: pjsip_rx_data__bindgen_ty_1,
    pub pkt_info: pjsip_rx_data__bindgen_ty_2,
    pub msg_info: pjsip_rx_data__bindgen_ty_3,
    pub endpt_info: pjsip_rx_data__bindgen_ty_4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_rx_data__bindgen_ty_1 {
    pub pool: *mut pj_pool_t,
    pub transport: *mut pjsip_transport,
    pub tp_data: *mut c_void,
    pub op_key: pjsip_rx_data_op_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_rx_data__bindgen_ty_2 {
    pub timestamp: pj_time_val,
    pub packet: [c_char; 4000usize],
    pub zero: pj_uint32_t,
    pub len: pj_ssize_t,
    pub src_addr: pj_sockaddr,
    pub src_addr_len: c_int,
    pub src_name: [c_char; 46usize],
    pub src_port: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_rx_data__bindgen_ty_3 {
    pub msg_buf: *mut c_char,
    pub len: c_int,
    pub msg: *mut pjsip_msg,
    pub info: *mut c_char,
    pub cid: *mut pjsip_cid_hdr,
    pub from: *mut pjsip_from_hdr,
    pub to: *mut pjsip_to_hdr,
    pub via: *mut pjsip_via_hdr,
    pub cseq: *mut pjsip_cseq_hdr,
    pub max_fwd: *mut pjsip_max_fwd_hdr,
    pub route: *mut pjsip_route_hdr,
    pub record_route: *mut pjsip_rr_hdr,
    pub ctype: *mut pjsip_ctype_hdr,
    pub clen: *mut pjsip_clen_hdr,
    pub require: *mut pjsip_require_hdr,
    pub supported: *mut pjsip_supported_hdr,
    pub parse_err: pjsip_parser_err_report,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_rx_data__bindgen_ty_4 {
    pub mod_data: [*mut c_void; 32usize],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tx_data_op_key {
    pub key: pj_ioqueue_op_key_t,
    pub tdata: *mut pjsip_tx_data,
    pub token: *mut c_void,
    pub callback: Option<
        unsafe extern "C" fn(
            arg1: *mut pjsip_transport,
            arg2: *mut c_void,
            arg3: pj_ssize_t,
        ),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_tx_data {
    pub prev: *mut pjsip_tx_data,
    pub next: *mut pjsip_tx_data,
    pub pool: *mut pj_pool_t,
    pub obj_name: [c_char; 32usize],
    pub info: *mut c_char,
    pub rx_timestamp: pj_time_val,
    pub mgr: *mut pjsip_tpmgr,
    pub op_key: pjsip_tx_data_op_key,
    pub lock: *mut pj_lock_t,
    pub msg: *mut pjsip_msg,
    pub saved_strict_route: *mut pjsip_route_hdr,
    pub buf: pjsip_buffer,
    pub ref_cnt: *mut pj_atomic_t,
    pub is_pending: c_int,
    pub token: *mut c_void,
    pub cb: Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *mut pjsip_tx_data,
            arg3: pj_ssize_t,
        ),
    >,
    pub dest_info: pjsip_tx_data__bindgen_ty_1,
    pub tp_info: pjsip_tx_data__bindgen_ty_2,
    pub tp_sel: pjsip_tpselector,
    pub auth_retry: pj_bool_t,
    pub mod_data: [*mut c_void; 32usize],
    pub via_addr: pjsip_host_port,
    pub via_tp: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_tx_data__bindgen_ty_1 {
    pub name: pj_str_t,
    pub addr: pjsip_server_addresses,
    pub cur_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_tx_data__bindgen_ty_2 {
    pub transport: *mut pjsip_transport,
    pub dst_addr: pj_sockaddr,
    pub dst_addr_len: c_int,
    pub dst_name: [c_char; 46usize],
    pub dst_port: c_int,
}

pub type pjsip_transport_callback = Option<
    unsafe extern "C" fn(
        tp: *mut pjsip_transport,
        token: *mut c_void,
        sent_bytes: pj_ssize_t,
    ),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_transport_key {
    pub type_: c_long,
    pub rem_addr: pj_sockaddr,
}



#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_transport {
    pub obj_name: [c_char; 32usize],
    pub pool: *mut pj_pool_t,
    pub ref_cnt: *mut pj_atomic_t,
    pub lock: *mut pj_lock_t,
    pub grp_lock: *mut pj_grp_lock_t,
    pub tracing: pj_bool_t,
    pub is_shutdown: pj_bool_t,
    pub is_destroying: pj_bool_t,
    pub key: pjsip_transport_key,
    pub type_name: *mut c_char,
    pub flag: c_uint,
    pub info: *mut c_char,
    pub addr_len: c_int,
    pub local_addr: pj_sockaddr,
    pub local_name: pjsip_host_port,
    pub remote_name: pjsip_host_port,
    pub dir: pjsip_transport_dir,
    pub endpt: *mut pjsip_endpoint,
    pub tpmgr: *mut pjsip_tpmgr,
    pub factory: *mut pjsip_tpfactory,
    pub idle_timer: pj_timer_entry,
    pub last_recv_ts: pj_timestamp,
    pub last_recv_len: pj_size_t,
    pub data: *mut c_void,
    pub send_msg: Option<
        unsafe extern "C" fn(
            transport: *mut pjsip_transport,
            tdata: *mut pjsip_tx_data,
            rem_addr: *const pj_sockaddr_t,
            addr_len: c_int,
            token: *mut c_void,
            callback: pjsip_transport_callback,
        ) -> pj_status_t,
    >,
    pub do_shutdown:
        Option<unsafe extern "C" fn(transport: *mut pjsip_transport) -> pj_status_t>,
    pub destroy:
        Option<unsafe extern "C" fn(transport: *mut pjsip_transport) -> pj_status_t>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_tpfactory {
    pub prev: *mut pjsip_tpfactory,
    pub next: *mut pjsip_tpfactory,
    pub obj_name: [c_char; 32usize],
    pub pool: *mut pj_pool_t,
    pub lock: *mut pj_lock_t,
    pub type_: pjsip_transport_type_e,
    pub type_name: *mut c_char,
    pub flag: c_uint,
    pub info: *mut c_char,
    pub local_addr: pj_sockaddr,
    pub addr_name: pjsip_host_port,
    pub create_transport: Option<
        unsafe extern "C" fn(
            factory: *mut pjsip_tpfactory,
            mgr: *mut pjsip_tpmgr,
            endpt: *mut pjsip_endpoint,
            rem_addr: *const pj_sockaddr,
            addr_len: c_int,
            transport: *mut *mut pjsip_transport,
        ) -> pj_status_t,
    >,
    pub create_transport2: Option<
        unsafe extern "C" fn(
            factory: *mut pjsip_tpfactory,
            mgr: *mut pjsip_tpmgr,
            endpt: *mut pjsip_endpoint,
            rem_addr: *const pj_sockaddr,
            addr_len: c_int,
            tdata: *mut pjsip_tx_data,
            transport: *mut *mut pjsip_transport,
        ) -> pj_status_t,
    >,
    pub destroy:
        Option<unsafe extern "C" fn(factory: *mut pjsip_tpfactory) -> pj_status_t>,
}

pub type pjsip_rx_callback = Option<
unsafe extern "C" fn(ep: *mut pjsip_endpoint, status: pj_status_t, rd: *mut pjsip_rx_data),
>;
pub type pjsip_tx_callback = Option<
unsafe extern "C" fn(ep: *mut pjsip_endpoint, td: *mut pjsip_tx_data) -> pj_status_t,
>;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tpmgr_fla2_param {
    pub tp_type: pjsip_transport_type_e,
    pub tp_sel: *const pjsip_tpselector,
    pub dst_host: pj_str_t,
    pub local_if: pj_bool_t,
    pub ret_addr: pj_str_t,
    pub ret_port: pj_uint16_t,
    pub ret_tp: *const c_void,
}



pub type pjsip_tp_send_callback = Option<
    unsafe extern "C" fn(
        token: *mut c_void,
        tdata: *mut pjsip_tx_data,
        bytes_sent: pj_ssize_t,
    ),
>;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_transport_state_info {
    pub status: pj_status_t,
    pub ext_info: *mut c_void,
    pub user_data: *mut c_void,
}
pub type pjsip_tp_state_callback = Option<
    unsafe extern "C" fn(
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tp_dropped_data {
    pub tp: *mut pjsip_transport,
    pub data: *mut c_void,
    pub len: pj_size_t,
    pub status: pj_status_t,
}

pub type pjsip_tp_on_rx_dropped_cb =
    Option<unsafe extern "C" fn(data: *mut pjsip_tp_dropped_data)>;

    extern "C" {
    pub fn pjsip_tpmgr_set_drop_data_cb(
        mgr: *mut pjsip_tpmgr,
        cb: pjsip_tp_on_rx_dropped_cb,
    ) -> pj_status_t;
}

pub type pjsip_endpt_exit_callback =
    Option<unsafe extern "C" fn(endpt: *mut pjsip_endpoint)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_process_rdata_param {
    pub start_prio: c_uint,
    pub start_mod: *mut c_void,
    pub idx_after_start: c_uint,
    pub silent: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_target {
    pub prev: *mut pjsip_target,
    pub next: *mut pjsip_target,
    pub uri: *mut pjsip_uri,
    pub q1000: c_int,
    pub code: pjsip_status_code,
    pub reason: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_target_set {
    pub head: pjsip_target,
    pub current: *mut pjsip_target,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_send_state {
    pub token: *mut c_void,
    pub endpt: *mut pjsip_endpoint,
    pub tdata: *mut pjsip_tx_data,
    pub cur_transport: *mut pjsip_transport,
    pub app_cb: Option<
        unsafe extern "C" fn(arg1: *mut pjsip_send_state, sent: pj_ssize_t, cont: *mut pj_bool_t),
    >,
}
pub type pjsip_send_callback = Option<
    unsafe extern "C" fn(st: *mut pjsip_send_state, sent: pj_ssize_t, cont: *mut pj_bool_t),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_response_addr {
    pub transport: *mut pjsip_transport,
    pub addr: pj_sockaddr,
    pub addr_len: c_int,
    pub dst_host: pjsip_host_info,
}

pub type pjsip_endpt_send_callback = Option<
    unsafe extern "C" fn(token: *mut c_void, e: *mut pjsip_event),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_udp_transport_cfg {
    pub af: c_int,
    pub bind_addr: pj_sockaddr,
    pub addr_name: pjsip_host_port,
    pub async_cnt: c_uint,
    pub qos_type: pj_qos_type,
    pub qos_params: pj_qos_params,
    pub sockopt_params: pj_sockopt_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_tcp_transport_cfg {
    pub af: c_int,
    pub bind_addr: pj_sockaddr,
    pub reuse_addr: pj_bool_t,
    pub addr_name: pjsip_host_port,
    pub async_cnt: c_uint,
    pub qos_type: pj_qos_type,
    pub qos_params: pj_qos_params,
    pub sockopt_params: pj_sockopt_params,
    pub initial_timeout: c_uint,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tls_state_info {
    pub ssl_sock_info: *mut pj_ssl_sock_info,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_common_credential {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_digest_credential {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
    pub username: pj_str_t,
    pub nonce: pj_str_t,
    pub uri: pj_str_t,
    pub response: pj_str_t,
    pub algorithm: pj_str_t,
    pub cnonce: pj_str_t,
    pub opaque: pj_str_t,
    pub qop: pj_str_t,
    pub nc: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_pgp_credential {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
    pub version: pj_str_t,
    pub signature: pj_str_t,
    pub signed_by: pj_str_t,
    pub nonce: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_oauth_credential {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
    pub username: pj_str_t,
    pub token: pj_str_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_authorization_hdr {
    pub prev: *mut pjsip_authorization_hdr,
    pub next: *mut pjsip_authorization_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub scheme: pj_str_t,
    pub credential: pjsip_authorization_hdr__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_authorization_hdr__bindgen_ty_1 {
    pub common: pjsip_common_credential,
    pub digest: pjsip_digest_credential,
    pub pgp: pjsip_pgp_credential,
    pub oauth: pjsip_oauth_credential,
    _bindgen_union_align: [u64; 26usize],
}

pub type pjsip_proxy_authorization_hdr = pjsip_authorization_hdr;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_common_challenge {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_digest_challenge {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
    pub domain: pj_str_t,
    pub nonce: pj_str_t,
    pub opaque: pj_str_t,
    pub stale: c_int,
    pub algorithm: pj_str_t,
    pub qop: pj_str_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_pgp_challenge {
    pub realm: pj_str_t,
    pub other_param: pjsip_param,
    pub version: pj_str_t,
    pub micalgorithm: pj_str_t,
    pub pubalgorithm: pj_str_t,
    pub nonce: pj_str_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_www_authenticate_hdr {
    pub prev: *mut pjsip_www_authenticate_hdr,
    pub next: *mut pjsip_www_authenticate_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub scheme: pj_str_t,
    pub challenge: pjsip_www_authenticate_hdr__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_www_authenticate_hdr__bindgen_ty_1 {
    pub common: pjsip_common_challenge,
    pub digest: pjsip_digest_challenge,
    pub pgp: pjsip_pgp_challenge,
    _bindgen_union_align: [u64; 19usize],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_cred_info {
    pub realm: pj_str_t,
    pub scheme: pj_str_t,
    pub username: pj_str_t,
    pub data_type: c_int,
    pub data: pj_str_t,
    pub ext: pjsip_cred_info__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_cred_info__bindgen_ty_1 {
    pub aka: pjsip_cred_info__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: [u64; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cred_info__bindgen_ty_1__bindgen_ty_1 {
    pub k: pj_str_t,
    pub op: pj_str_t,
    pub amf: pj_str_t,
    pub cb: pjsip_cred_cb,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cached_auth_hdr {
    pub prev: *mut pjsip_cached_auth_hdr,
    pub next: *mut pjsip_cached_auth_hdr,
    pub method: pjsip_method,
    pub hdr: *mut pjsip_authorization_hdr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cached_auth {
    pub prev: *mut pjsip_cached_auth,
    pub next: *mut pjsip_cached_auth,
    pub pool: *mut pj_pool_t,
    pub realm: pj_str_t,
    pub is_proxy: pj_bool_t,
    pub qop_value: pjsip_auth_qop_type,
    pub stale_cnt: c_uint,
    pub nc: pj_uint32_t,
    pub cnonce: pj_str_t,
    pub last_chal: *mut pjsip_www_authenticate_hdr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_auth_clt_pref {
    pub initial_auth: pj_bool_t,
    pub algorithm: pj_str_t,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_auth_clt_sess {
    pub pool: *mut pj_pool_t,
    pub endpt: *mut pjsip_endpoint,
    pub pref: pjsip_auth_clt_pref,
    pub cred_cnt: c_uint,
    pub cred_info: *mut pjsip_cred_info,
    pub cached_auth: pjsip_cached_auth,
}


pub type pjsip_auth_lookup_cred = Option<
    unsafe extern "C" fn(
        pool: *mut pj_pool_t,
        realm: *const pj_str_t,
        acc_name: *const pj_str_t,
        cred_info: *mut pjsip_cred_info,
    ) -> pj_status_t,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_auth_lookup_cred_param {
    pub realm: pj_str_t,
    pub acc_name: pj_str_t,
    pub rdata: *mut pjsip_rx_data,
}

pub type pjsip_auth_lookup_cred2 = Option<
    unsafe extern "C" fn(
        pool: *mut pj_pool_t,
        param: *const pjsip_auth_lookup_cred_param,
        cred_info: *mut pjsip_cred_info,
    ) -> pj_status_t,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_auth_srv {
    pub realm: pj_str_t,
    pub is_proxy: pj_bool_t,
    pub lookup: pjsip_auth_lookup_cred,
    pub lookup2: pjsip_auth_lookup_cred2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_auth_srv_init_param {
    pub realm: *const pj_str_t,
    pub lookup2: pjsip_auth_lookup_cred2,
    pub options: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_transaction {
    pub pool: *mut pj_pool_t,
    pub tsx_user: *mut pjsip_module,
    pub endpt: *mut pjsip_endpoint,
    pub terminating: pj_bool_t,
    pub grp_lock: *mut pj_grp_lock_t,
    pub mutex_b: *mut pj_mutex_t,
    pub obj_name: [c_char; 32usize],
    pub role: pjsip_role_e,
    pub method: pjsip_method,
    pub cseq: pj_int32_t,
    pub transaction_key: pj_str_t,
    pub hashed_key: pj_uint32_t,
    pub branch: pj_str_t,
    pub status_code: c_int,
    pub status_text: pj_str_t,
    pub state: pjsip_tsx_state_e,
    pub handle_200resp: c_int,
    pub tracing: c_int,
    pub state_handler: Option<
        unsafe extern "C" fn(arg1: *mut pjsip_transaction, arg2: *mut pjsip_event) -> pj_status_t,
    >,
    pub transport: *mut pjsip_transport,
    pub is_reliable: pj_bool_t,
    pub addr: pj_sockaddr,
    pub addr_len: c_int,
    pub res_addr: pjsip_response_addr,
    pub transport_flag: c_uint,
    pub transport_err: pj_status_t,
    pub tp_sel: pjsip_tpselector,
    pub pending_tx: *mut pjsip_tx_data,
    pub tp_st_key: *mut pjsip_tp_state_listener_key,
    pub last_tx: *mut pjsip_tx_data,
    pub retransmit_count: c_int,
    pub retransmit_timer: pj_timer_entry,
    pub timeout_timer: pj_timer_entry,
    pub mod_data: [*mut c_void; 32usize],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_ua_init_param {
    pub on_dlg_forked: Option<
        unsafe extern "C" fn(
            first_set: *mut pjsip_dialog,
            res: *mut pjsip_rx_data,
        ) -> *mut pjsip_dialog,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_dlg_party {
    pub info: *mut pjsip_fromto_hdr,
    pub info_str: pj_str_t,
    pub tag_hval: pj_uint32_t,
    pub contact: *mut pjsip_contact_hdr,
    pub first_cseq: pj_int32_t,
    pub cseq: pj_int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_dialog {
    pub prev: *mut pjsip_dialog,
    pub next: *mut pjsip_dialog,
    pub obj_name: [c_char; 32usize],
    pub pool: *mut pj_pool_t,
    pub ua: *mut pjsip_user_agent,
    pub endpt: *mut pjsip_endpoint,
    pub grp_lock_: *mut pj_grp_lock_t,
    pub dlg_set: *mut c_void,
    pub state: pjsip_dialog_state,
    pub target: *mut pjsip_uri,
    pub target_set: pjsip_target_set,
    pub inv_hdr: pjsip_hdr,
    pub local: pjsip_dlg_party,
    pub remote: pjsip_dlg_party,
    pub rem_cap_hdr: pjsip_hdr,
    pub role: pjsip_role_e,
    pub uac_has_2xx: pj_bool_t,
    pub secure: pj_bool_t,
    pub add_allow: pj_bool_t,
    pub call_id: *mut pjsip_cid_hdr,
    pub route_set: pjsip_route_hdr,
    pub route_set_frozen: pj_bool_t,
    pub auth_sess: pjsip_auth_clt_sess,
    pub sess_count: c_int,
    pub tsx_count: c_int,
    pub tp_sel: pjsip_tpselector,
    pub usage_cnt: c_uint,
    pub usage: [*mut pjsip_module; 32usize],
    pub mod_data: [*mut c_void; 32usize],
    pub via_addr: pjsip_host_port,
    pub via_tp: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_dlg_create_uac_param {
    pub ua: *mut pjsip_user_agent,
    pub local_uri: pj_str_t,
    pub local_contact: pj_str_t,
    pub remote_uri: pj_str_t,
    pub target: pj_str_t,
    pub grp_lock: *mut pj_grp_lock_t,
}




pub const pjsip_inv_state_PJSIP_INV_STATE_NULL: pjsip_inv_state = 0;
pub const pjsip_inv_state_PJSIP_INV_STATE_CALLING: pjsip_inv_state = 1;
pub const pjsip_inv_state_PJSIP_INV_STATE_INCOMING: pjsip_inv_state = 2;
pub const pjsip_inv_state_PJSIP_INV_STATE_EARLY: pjsip_inv_state = 3;
pub const pjsip_inv_state_PJSIP_INV_STATE_CONNECTING: pjsip_inv_state = 4;
pub const pjsip_inv_state_PJSIP_INV_STATE_CONFIRMED: pjsip_inv_state = 5;
pub const pjsip_inv_state_PJSIP_INV_STATE_DISCONNECTED: pjsip_inv_state = 6;
pub type pjsip_inv_state = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_inv_on_rx_offer_cb_param {
    pub offer: *const pjmedia_sdp_session,
    pub rdata: *const pjsip_rx_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_inv_callback {
    pub on_state_changed: Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, e: *mut pjsip_event),
    >,
    pub on_new_session: Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, e: *mut pjsip_event),
    >,
    pub on_tsx_state_changed: Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            tsx: *mut pjsip_transaction,
            e: *mut pjsip_event,
        ),
    >,
    pub on_rx_offer: Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, offer: *const pjmedia_sdp_session),
    >,
    pub on_rx_offer2: Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            param: *mut pjsip_inv_on_rx_offer_cb_param,
        ),
    >,
    pub on_rx_reinvite: Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            offer: *const pjmedia_sdp_session,
            rdata: *mut pjsip_rx_data,
        ) -> pj_status_t,
    >,
    pub on_create_offer: Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, p_offer: *mut *mut pjmedia_sdp_session),
    >,
    pub on_media_update: Option<
        unsafe extern "C" fn(inv_ses: *mut pjsip_inv_session, status: pj_status_t),
    >,
    pub on_send_ack: Option<
        unsafe extern "C" fn(inv: *mut pjsip_inv_session, rdata: *mut pjsip_rx_data),
    >,
    pub on_redirected: Option<
        unsafe extern "C" fn(
            inv: *mut pjsip_inv_session,
            target: *const pjsip_uri,
            e: *const pjsip_event,
        ) -> pjsip_redirect_op,
    >,
}

pub const pjsip_inv_option_PJSIP_INV_SUPPORT_100REL: pjsip_inv_option = 1;
pub const pjsip_inv_option_PJSIP_INV_SUPPORT_TIMER: pjsip_inv_option = 2;
pub const pjsip_inv_option_PJSIP_INV_SUPPORT_UPDATE: pjsip_inv_option = 4;
pub const pjsip_inv_option_PJSIP_INV_SUPPORT_ICE: pjsip_inv_option = 8;
pub const pjsip_inv_option_PJSIP_INV_REQUIRE_ICE: pjsip_inv_option = 16;
pub const pjsip_inv_option_PJSIP_INV_REQUIRE_100REL: pjsip_inv_option = 32;
pub const pjsip_inv_option_PJSIP_INV_REQUIRE_TIMER: pjsip_inv_option = 64;
pub const pjsip_inv_option_PJSIP_INV_ALWAYS_USE_TIMER: pjsip_inv_option = 128;
pub const pjsip_inv_option_PJSIP_INV_SUPPORT_TRICKLE_ICE: pjsip_inv_option = 256;
pub const pjsip_inv_option_PJSIP_INV_REQUIRE_TRICKLE_ICE: pjsip_inv_option = 512;
pub type pjsip_inv_option = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_timer {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_inv_session {
    pub obj_name: [c_char; 32usize],
    pub pool: *mut pj_pool_t,
    pub pool_prov: *mut pj_pool_t,
    pub pool_active: *mut pj_pool_t,
    pub state: pjsip_inv_state,
    pub cancelling: pj_bool_t,
    pub pending_cancel: pj_bool_t,
    pub pending_bye: *mut pjsip_tx_data,
    pub cause: pjsip_status_code,
    pub cause_text: pj_str_t,
    pub notify: pj_bool_t,
    pub cb_called: c_uint,
    pub dlg: *mut pjsip_dialog,
    pub role: pjsip_role_e,
    pub options: c_uint,
    pub neg: *mut pjmedia_sdp_neg,
    pub sdp_neg_flags: c_uint,
    pub invite_tsx: *mut pjsip_transaction,
    pub invite_req: *mut pjsip_tx_data,
    pub last_answer: *mut pjsip_tx_data,
    pub last_ack: *mut pjsip_tx_data,
    pub last_ack_cseq: pj_int32_t,
    pub mod_data: [*mut c_void; 32usize],
    pub timer: *mut pjsip_timer,
    pub following_fork: pj_bool_t,
    pub ref_cnt: *mut pj_atomic_t,
    pub updated_sdp_answer: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_rdata_sdp_info {
    pub body: pj_str_t,
    pub sdp_err: pj_status_t,
    pub sdp: *mut pjmedia_sdp_session,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_regc {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_regc_cbparam {
    pub regc: *mut pjsip_regc,
    pub token: *mut c_void,
    pub status: pj_status_t,
    pub code: c_int,
    pub reason: pj_str_t,
    pub rdata: *mut pjsip_rx_data,
    pub expiration: c_uint,
    pub contact_cnt: c_int,
    pub contact: [*mut pjsip_contact_hdr; 10usize],
    pub is_unreg: pj_bool_t,
}

pub type pjsip_regc_cb = Option<unsafe extern "C" fn(param: *mut pjsip_regc_cbparam)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_regc_tsx_cb_param {
    pub cbparam: pjsip_regc_cbparam,
    pub contact_cnt: c_int,
    pub contact: [pj_str_t; 10usize],
}

pub type pjsip_regc_tsx_cb = Option<unsafe extern "C" fn(param: *mut pjsip_regc_tsx_cb_param)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_regc_info {
    pub server_uri: pj_str_t,
    pub client_uri: pj_str_t,
    pub is_busy: pj_bool_t,
    pub auto_reg: pj_bool_t,
    pub interval: c_uint,
    pub next_reg: c_uint,
    pub transport: *mut pjsip_transport,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_replaces_hdr {
    pub prev: *mut pjsip_replaces_hdr,
    pub next: *mut pjsip_replaces_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub call_id: pj_str_t,
    pub to_tag: pj_str_t,
    pub from_tag: pj_str_t,
    pub early_only: pj_bool_t,
    pub other_param: pjsip_param,
}

pub type _bindgen_ty_18 = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_timer_setting {
    pub min_se: c_uint,
    pub sess_expires: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_sess_expires_hdr {
    pub prev: *mut pjsip_sess_expires_hdr,
    pub next: *mut pjsip_sess_expires_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub sess_expires: c_uint,
    pub refresher: pj_str_t,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_min_se_hdr {
    pub prev: *mut pjsip_min_se_hdr,
    pub next: *mut pjsip_min_se_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub min_se: c_uint,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_event_hdr {
    pub prev: *mut pjsip_event_hdr,
    pub next: *mut pjsip_event_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub event_type: pj_str_t,
    pub id_param: pj_str_t,
    pub other_param: pjsip_param,
}


pub type pjsip_allow_events_hdr = pjsip_generic_array_hdr;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_sub_state_hdr {
    pub prev: *mut pjsip_sub_state_hdr,
    pub next: *mut pjsip_sub_state_hdr,
    pub type_: pjsip_hdr_e,
    pub name: pj_str_t,
    pub sname: pj_str_t,
    pub vptr: *mut pjsip_hdr_vptr,
    pub sub_state: pj_str_t,
    pub reason_param: pj_str_t,
    pub expires_param: c_uint,
    pub retry_after: c_int,
    pub other_param: pjsip_param,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_pres_status {
    pub info_cnt: c_uint,
    pub info: [pjsip_pres_status__bindgen_ty_1; 8usize],
    pub _is_valid: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_pres_status__bindgen_ty_1 {
    pub basic_open: pj_bool_t,
    pub rpid: pjrpid_element,
    pub id: pj_str_t,
    pub contact: pj_str_t,
    pub tuple_node: *mut pj_xml_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_publishc {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_publishc_opt {
    pub queue_request: pj_bool_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_publishc_cbparam {
    pub pubc: *mut pjsip_publishc,
    pub token: *mut c_void,
    pub status: pj_status_t,
    pub code: c_int,
    pub reason: pj_str_t,
    pub rdata: *mut pjsip_rx_data,
    pub expiration: c_uint,
}

pub type pjsip_publishc_cb = Option<unsafe extern "C" fn(param: *mut pjsip_publishc_cbparam)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_msg {
    pub type_: pjsip_msg_type_e,
    pub line: pjsip_msg__bindgen_ty_1,
    pub hdr: pjsip_hdr,
    pub body: *mut pjsip_msg_body,
}

#[link(name="pjsip")]
extern "C" {
    pub fn pjsip_event_str(e: pjsip_event_id_e) -> *const c_char;
    pub fn pjsip_param_find( param_list: *const pjsip_param, name: *const pj_str_t, ) -> *mut pjsip_param;
    pub fn pjsip_param_cmp( param_list1: *const pjsip_param, param_list2: *const pjsip_param, ig_nf: pj_bool_t, ) -> c_int;
    pub fn pjsip_param_clone( pool: *mut pj_pool_t, dst_list: *mut pjsip_param, src_list: *const pjsip_param, );
    pub fn pjsip_param_shallow_clone( pool: *mut pj_pool_t, dst_list: *mut pjsip_param, src_list: *const pjsip_param,);
    pub fn pjsip_param_print_on( param_list: *const pjsip_param, buf: *mut c_char, size: pj_size_t, pname_unres: *const pj_cis_t, pvalue_unres: *const pj_cis_t, sep: c_int, ) -> pj_ssize_t;
    pub fn pjsip_sip_uri_create(pool: *mut pj_pool_t, secure: pj_bool_t) -> *mut pjsip_sip_uri;
    pub fn pjsip_sip_uri_set_secure(uri: *mut pjsip_sip_uri, secure: pj_bool_t);
    pub fn pjsip_sip_uri_init(url: *mut pjsip_sip_uri, secure: pj_bool_t);
    pub fn pjsip_sip_uri_assign( pool: *mut pj_pool_t, url: *mut pjsip_sip_uri, rhs: *const pjsip_sip_uri, );
    pub fn pjsip_name_addr_create(pool: *mut pj_pool_t) -> *mut pjsip_name_addr;
    pub fn pjsip_name_addr_init(name_addr: *mut pjsip_name_addr);
    pub fn pjsip_name_addr_assign( pool: *mut pj_pool_t, addr: *mut pjsip_name_addr, rhs: *const pjsip_name_addr, );
    pub fn pjsip_other_uri_create(pool: *mut pj_pool_t) -> *mut pjsip_other_uri;
    pub fn pjsip_tel_uri_create(pool: *mut pj_pool_t) -> *mut pjsip_tel_uri;
    pub fn pjsip_tel_nb_cmp(nb1: *const pj_str_t, nb2: *const pj_str_t) -> c_int;
    pub static pjsip_invite_method: pjsip_method;
    pub static pjsip_cancel_method: pjsip_method;
    pub static pjsip_ack_method: pjsip_method;
    pub static pjsip_bye_method: pjsip_method;
    pub static pjsip_register_method: pjsip_method;
    pub static pjsip_options_method: pjsip_method;
    pub fn pjsip_get_invite_method() -> *const pjsip_method;
    pub fn pjsip_get_cancel_method() -> *const pjsip_method;
    pub fn pjsip_get_ack_method() -> *const pjsip_method;
    pub fn pjsip_get_bye_method() -> *const pjsip_method;
    pub fn pjsip_get_register_method() -> *const pjsip_method;
    pub fn pjsip_get_options_method() -> *const pjsip_method;
    pub fn pjsip_method_init(m: *mut pjsip_method, pool: *mut pj_pool_t, str_: *const pj_str_t);
    pub fn pjsip_method_init_np(m: *mut pjsip_method, str_: *mut pj_str_t);
    pub fn pjsip_method_set(m: *mut pjsip_method, id: pjsip_method_e);
    pub fn pjsip_method_copy( pool: *mut pj_pool_t, method: *mut pjsip_method, rhs: *const pjsip_method, );
    pub fn pjsip_method_cmp( m1: *const pjsip_method, m2: *const pjsip_method, ) -> c_int;
    pub static mut pjsip_sip_cfg_var: pjsip_cfg_t;
    pub fn pjsip_exception_to_status(exception_id: c_int) -> pj_status_t;
    pub fn pjsip_hdr_clone( pool: *mut pj_pool_t, hdr: *const c_void, ) -> *mut c_void;
    pub fn pjsip_hdr_shallow_clone( pool: *mut pj_pool_t, hdr: *const c_void, ) -> *mut c_void;
    pub fn pjsip_hdr_print_on( hdr: *mut c_void, buf: *mut c_char, len: pj_size_t, ) -> c_int;
    pub fn pjsip_get_status_text(status_code: c_int) -> *const pj_str_t;
    pub fn pjsip_print_text_body( msg_body: *mut pjsip_msg_body, buf: *mut c_char, size: pj_size_t, ) -> c_int;
    pub fn pjsip_clone_text_data( pool: *mut pj_pool_t, data: *const c_void, len: c_uint, ) -> *mut c_void;
    pub fn pjsip_media_type_init( mt: *mut pjsip_media_type, type_: *mut pj_str_t, subtype: *mut pj_str_t, );
    pub fn pjsip_media_type_init2( mt: *mut pjsip_media_type, type_: *mut c_char, subtype: *mut c_char, );
    pub fn pjsip_media_type_cmp( mt1: *const pjsip_media_type, mt2: *const pjsip_media_type, cmp_param: c_int, ) -> c_int;
    pub fn pjsip_media_type_cp( pool: *mut pj_pool_t, dst: *mut pjsip_media_type, src: *const pjsip_media_type, );
    pub fn pjsip_media_type_print( buf: *mut c_char, len: c_uint, mt: *const pjsip_media_type, ) -> c_int;
    pub fn pjsip_generic_string_hdr_create( pool: *mut pj_pool_t, hname: *const pj_str_t, hvalue: *const pj_str_t, ) -> *mut pjsip_generic_string_hdr;
    pub fn pjsip_generic_string_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, hname: *const pj_str_t, hvalue: *const pj_str_t, ) -> *mut pjsip_generic_string_hdr;
    pub fn pjsip_generic_string_hdr_init2( h: *mut pjsip_generic_string_hdr, hname: *mut pj_str_t, hvalue: *mut pj_str_t,);
    pub fn pjsip_generic_int_hdr_create( pool: *mut pj_pool_t, hname: *const pj_str_t, hvalue: c_uint, ) -> *mut pjsip_generic_int_hdr;
    pub fn pjsip_generic_int_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, hname: *const pj_str_t, value: c_uint, ) -> *mut pjsip_generic_int_hdr;
    pub fn pjsip_generic_array_hdr_create( pool: *mut pj_pool_t, hname: *const pj_str_t, ) -> *mut pjsip_generic_array_hdr;
    pub fn pjsip_generic_array_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, hname: *const pj_str_t, ) -> *mut pjsip_generic_array_hdr;
    pub fn pjsip_accept_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_accept_hdr;
    pub fn pjsip_accept_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_accept_hdr;
    pub fn pjsip_allow_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_allow_hdr;
    pub fn pjsip_allow_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_allow_hdr;
    pub fn pjsip_cid_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_cid_hdr;
    pub fn pjsip_cid_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_cid_hdr;
    pub fn pjsip_clen_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_clen_hdr;
    pub fn pjsip_clen_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_clen_hdr;
    pub fn pjsip_cseq_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_cseq_hdr;
    pub fn pjsip_cseq_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_cseq_hdr;
    pub fn pjsip_contact_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_contact_hdr;
    pub fn pjsip_contact_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void,) -> *mut pjsip_contact_hdr;
    pub fn pjsip_ctype_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_ctype_hdr;
    pub fn pjsip_ctype_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_ctype_hdr;
    pub fn pjsip_expires_hdr_create( pool: *mut pj_pool_t, value: c_uint, ) -> *mut pjsip_expires_hdr;
    pub fn pjsip_expires_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, value: c_uint, ) -> *mut pjsip_expires_hdr;
    pub fn pjsip_from_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_from_hdr;
    pub fn pjsip_from_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_from_hdr;
    pub fn pjsip_to_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_to_hdr;
    pub fn pjsip_to_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_to_hdr;
    pub fn pjsip_fromto_hdr_set_from(hdr: *mut pjsip_fromto_hdr) -> *mut pjsip_from_hdr;
    pub fn pjsip_fromto_hdr_set_to(hdr: *mut pjsip_fromto_hdr) -> *mut pjsip_to_hdr;
    pub fn pjsip_max_fwd_hdr_create( pool: *mut pj_pool_t, value: c_uint, ) -> *mut pjsip_max_fwd_hdr;
    pub fn pjsip_max_fwd_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, value: c_uint, ) -> *mut pjsip_max_fwd_hdr;
    pub fn pjsip_min_expires_hdr_create( pool: *mut pj_pool_t, value: c_uint, ) -> *mut pjsip_min_expires_hdr;
    pub fn pjsip_min_expires_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, value: c_uint, ) -> *mut pjsip_min_expires_hdr;
    pub fn pjsip_rr_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_rr_hdr;
    pub fn pjsip_rr_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_rr_hdr;
    pub fn pjsip_route_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_route_hdr;
    pub fn pjsip_route_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_route_hdr;
    pub fn pjsip_routing_hdr_set_rr(r: *mut pjsip_routing_hdr) -> *mut pjsip_rr_hdr;
    pub fn pjsip_routing_hdr_set_route(r: *mut pjsip_routing_hdr) -> *mut pjsip_route_hdr;
    pub fn pjsip_require_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_require_hdr;
    pub fn pjsip_require_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_require_hdr;
    pub fn pjsip_retry_after_hdr_create( pool: *mut pj_pool_t, value: c_int, ) -> *mut pjsip_retry_after_hdr;
    pub fn pjsip_retry_after_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, value: c_int, ) -> *mut pjsip_retry_after_hdr;
    pub fn pjsip_supported_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_supported_hdr;
    pub fn pjsip_supported_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_supported_hdr;
    pub fn pjsip_unsupported_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_unsupported_hdr;
    pub fn pjsip_unsupported_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_unsupported_hdr;
    pub fn pjsip_via_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_via_hdr;
    pub fn pjsip_via_hdr_init( pool: *mut pj_pool_t, mem: *mut c_void, ) -> *mut pjsip_via_hdr;
    pub fn pjsip_warning_hdr_create( pool: *mut pj_pool_t, code: c_int, host: *const pj_str_t, text: *const pj_str_t, ) -> *mut pjsip_warning_hdr;
    pub fn pjsip_warning_hdr_create_from_status( pool: *mut pj_pool_t, host: *const pj_str_t, status: pj_status_t, ) -> *mut pjsip_warning_hdr;
    pub fn pjsip_multipart_create( pool: *mut pj_pool_t, ctype: *const pjsip_media_type, boundary: *const pj_str_t, ) -> *mut pjsip_msg_body;
    pub fn pjsip_multipart_create_part(pool: *mut pj_pool_t) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_clone_part( pool: *mut pj_pool_t, part: *const pjsip_multipart_part, ) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_add_part( pool: *mut pj_pool_t, mp: *mut pjsip_msg_body, part: *mut pjsip_multipart_part, ) -> pj_status_t;
    pub fn pjsip_multipart_get_first_part(mp: *const pjsip_msg_body) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_get_next_part( mp: *const pjsip_msg_body, part: *mut pjsip_multipart_part, ) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_find_part( mp: *const pjsip_msg_body, content_type: *const pjsip_media_type, start: *const pjsip_multipart_part, ) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_parse( pool: *mut pj_pool_t, buf: *mut c_char, len: pj_size_t, ctype: *const pjsip_media_type, options: c_uint, ) -> *mut pjsip_msg_body;
    pub fn pjsip_multipart_get_raw( mp: *mut pjsip_msg_body, boundary: *mut pj_str_t, raw_data: *mut pj_str_t, ) -> pj_status_t;
    pub static mut PJSIP_SYN_ERR_EXCEPTION: c_int;
    pub static mut PJSIP_EINVAL_ERR_EXCEPTION: c_int;
    pub fn pjsip_register_hdr_parser( hname: *const c_char, hshortname: *const c_char, fptr: pjsip_parse_hdr_func, ) -> pj_status_t;
    pub fn pjsip_unregister_hdr_parser( hname: *const c_char, hshortname: *const c_char, fptr: pjsip_parse_hdr_func, ) -> pj_status_t;
    pub fn pjsip_register_uri_parser( scheme: *mut c_char, func: pjsip_parse_uri_func, ) -> pj_status_t;
    pub fn pjsip_unregister_uri_parser( scheme: *const c_char, func: pjsip_parse_uri_func, ) -> pj_status_t;
    pub fn pjsip_parse_uri( pool: *mut pj_pool_t, buf: *mut c_char, size: pj_size_t, options: c_uint, ) -> *mut pjsip_uri;
    pub fn pjsip_parse_status_line( buf: *mut c_char, size: pj_size_t, status_line: *mut pjsip_status_line, ) -> pj_status_t;
    pub fn pjsip_parse_msg( pool: *mut pj_pool_t, buf: *mut c_char, size: pj_size_t, err_list: *mut pjsip_parser_err_report, ) -> *mut pjsip_msg;
    pub fn pjsip_parse_rdata( buf: *mut c_char, size: pj_size_t, rdata: *mut pjsip_rx_data, ) -> *mut pjsip_msg;
    pub fn pjsip_find_msg( buf: *const c_char, size: pj_size_t, is_datagram: pj_bool_t, msg_size: *mut pj_size_t, ) -> pj_status_t;
    pub fn pjsip_parse_hdr( pool: *mut pj_pool_t, hname: *const pj_str_t, line: *mut c_char, size: pj_size_t, parsed_len: *mut c_int, ) -> *mut c_void;
    pub fn pjsip_parse_headers( pool: *mut pj_pool_t, input: *mut c_char, size: pj_size_t, hlist: *mut pjsip_hdr, options: c_uint,) -> pj_status_t;
    pub fn pjsip_parser_const() -> *const pjsip_parser_const_t;
    pub fn pjsip_parse_param_imp( scanner: *mut pj_scanner, pool: *mut pj_pool_t, pname: *mut pj_str_t, pvalue: *mut pj_str_t, opt: c_uint, );
    pub fn pjsip_parse_uri_param_imp( scanner: *mut pj_scanner, pool: *mut pj_pool_t, pname: *mut pj_str_t, pvalue: *mut pj_str_t, opt: c_uint, );
    pub fn pjsip_concat_param_imp( param: *mut pj_str_t, pool: *mut pj_pool_t, pname: *const pj_str_t, pvalue: *const pj_str_t, sepchar: c_int, );
    pub fn pjsip_parse_end_hdr_imp(scanner: *mut pj_scanner);
    pub fn pjsip_parse_generic_array_hdr_imp( hdr: *mut pjsip_generic_array_hdr, scanner: *mut pj_scanner, );
    pub fn pjsip_resolver_create( pool: *mut pj_pool_t, p_res: *mut *mut pjsip_resolver_t, ) -> pj_status_t;
    pub fn pjsip_resolver_set_resolver( res: *mut pjsip_resolver_t, dns_res: *mut pj_dns_resolver,) -> pj_status_t;
    pub fn pjsip_resolver_set_ext_resolver(res: *mut pjsip_resolver_t,ext_res: *mut pjsip_ext_resolver, ) -> pj_status_t;
    pub fn pjsip_resolver_get_resolver(res: *mut pjsip_resolver_t) -> *mut pj_dns_resolver;
    pub fn pjsip_resolver_destroy(resolver: *mut pjsip_resolver_t);
    pub fn pjsip_resolve( resolver: *mut pjsip_resolver_t, pool: *mut pj_pool_t, target: *const pjsip_host_info, token: *mut c_void, cb: pjsip_resolver_callback,);
    pub fn pjsip_transport_register_type( tp_flag: c_uint, tp_name: *const c_char, def_port: c_int, p_tp_type: *mut c_int, ) -> pj_status_t;
    pub fn pjsip_transport_get_type_from_name(name: *const pj_str_t) -> pjsip_transport_type_e;
    pub fn pjsip_transport_get_type_from_flag( flag: c_uint, ) -> pjsip_transport_type_e;
    pub fn pjsip_transport_type_get_af(type_: pjsip_transport_type_e) -> c_int;
    pub fn pjsip_transport_get_flag_from_type( type_: pjsip_transport_type_e, ) -> c_uint;
    pub fn pjsip_transport_get_default_port_for_type( type_: pjsip_transport_type_e, ) -> c_int;
    pub fn pjsip_transport_get_type_name( t: pjsip_transport_type_e, ) -> *const c_char;
    pub fn pjsip_transport_get_type_desc(t: pjsip_transport_type_e, ) -> *const c_char;
    pub fn pjsip_tpselector_add_ref(sel: *mut pjsip_tpselector);
    pub fn pjsip_tpselector_dec_ref(sel: *mut pjsip_tpselector);
    pub fn pjsip_rx_data_get_info(rdata: *mut pjsip_rx_data) -> *mut c_char;
    pub fn pjsip_rx_data_clone( src: *const pjsip_rx_data, flags: c_uint, p_rdata: *mut *mut pjsip_rx_data, ) -> pj_status_t;
    pub fn pjsip_rx_data_free_cloned(rdata: *mut pjsip_rx_data) -> pj_status_t;
    pub fn pjsip_tx_data_create( mgr: *mut pjsip_tpmgr, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_tx_data_add_ref(tdata: *mut pjsip_tx_data);
    pub fn pjsip_tx_data_dec_ref(tdata: *mut pjsip_tx_data) -> pj_status_t;
    pub fn pjsip_tx_data_encode(tdata: *mut pjsip_tx_data) -> pj_status_t;
    pub fn pjsip_tx_data_is_valid(tdata: *mut pjsip_tx_data) -> pj_bool_t;
    pub fn pjsip_tx_data_invalidate_msg(tdata: *mut pjsip_tx_data);
    pub fn pjsip_tx_data_get_info(tdata: *mut pjsip_tx_data) -> *mut c_char;
    pub fn pjsip_tx_data_set_transport( tdata: *mut pjsip_tx_data, sel: *const pjsip_tpselector, ) -> pj_status_t;
    pub fn pjsip_tx_data_clone( src: *const pjsip_tx_data, flags: c_uint, p_rdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_transport_register(mgr: *mut pjsip_tpmgr, tp: *mut pjsip_transport) -> pj_status_t;
    pub fn pjsip_transport_shutdown(tp: *mut pjsip_transport) -> pj_status_t;
    pub fn pjsip_transport_shutdown2(tp: *mut pjsip_transport, force: pj_bool_t) -> pj_status_t;
    pub fn pjsip_transport_destroy(tp: *mut pjsip_transport) -> pj_status_t;
    pub fn pjsip_transport_add_ref(tp: *mut pjsip_transport) -> pj_status_t;
    pub fn pjsip_transport_dec_ref(tp: *mut pjsip_transport) -> pj_status_t;
    pub fn pjsip_tpmgr_receive_packet( mgr: *mut pjsip_tpmgr, rdata: *mut pjsip_rx_data, ) -> pj_ssize_t;
    pub fn pjsip_tpmgr_register_tpfactory( mgr: *mut pjsip_tpmgr, tpf: *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tpmgr_unregister_tpfactory( mgr: *mut pjsip_tpmgr, tpf: *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tpmgr_create( pool: *mut pj_pool_t, endpt: *mut pjsip_endpoint, rx_cb: pjsip_rx_callback, tx_cb: pjsip_tx_callback, p_mgr: *mut *mut pjsip_tpmgr, ) -> pj_status_t;
    pub fn pjsip_tpmgr_find_local_addr( tpmgr: *mut pjsip_tpmgr, pool: *mut pj_pool_t, type_: pjsip_transport_type_e, sel: *const pjsip_tpselector, ip_addr: *mut pj_str_t, port: *mut c_int, ) -> pj_status_t;
    pub fn pjsip_tpmgr_fla2_param_default(prm: *mut pjsip_tpmgr_fla2_param);
    pub fn pjsip_tpmgr_find_local_addr2( tpmgr: *mut pjsip_tpmgr, pool: *mut pj_pool_t, prm: *mut pjsip_tpmgr_fla2_param, ) -> pj_status_t;
    pub fn pjsip_tpmgr_get_transport_count(mgr: *mut pjsip_tpmgr) -> c_uint;
    pub fn pjsip_tpmgr_destroy(mgr: *mut pjsip_tpmgr) -> pj_status_t;
    pub fn pjsip_tpmgr_dump_transports(mgr: *mut pjsip_tpmgr);
    pub fn pjsip_tpmgr_acquire_transport( mgr: *mut pjsip_tpmgr, type_: pjsip_transport_type_e, remote: *const pj_sockaddr_t, addr_len: c_int, sel: *const pjsip_tpselector, tp: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_tpmgr_acquire_transport2( mgr: *mut pjsip_tpmgr, type_: pjsip_transport_type_e, remote: *const pj_sockaddr_t, addr_len: c_int, sel: *const pjsip_tpselector, tdata: *mut pjsip_tx_data, tp: *mut *mut pjsip_transport,) -> pj_status_t;
    pub fn pjsip_transport_send( tr: *mut pjsip_transport, tdata: *mut pjsip_tx_data, addr: *const pj_sockaddr_t, addr_len: c_int, token: *mut c_void, cb: pjsip_tp_send_callback, ) -> pj_status_t;
    pub fn pjsip_tpmgr_send_raw( mgr: *mut pjsip_tpmgr, tp_type: pjsip_transport_type_e, sel: *const pjsip_tpselector, tdata: *mut pjsip_tx_data, raw_data: *const c_void, data_len: pj_size_t, addr: *const pj_sockaddr_t, addr_len: c_int, token: *mut c_void, cb: pjsip_tp_send_callback, ) -> pj_status_t;
    pub fn pjsip_tpmgr_set_state_cb( mgr: *mut pjsip_tpmgr, cb: pjsip_tp_state_callback, ) -> pj_status_t;
    pub fn pjsip_tpmgr_get_state_cb(mgr: *const pjsip_tpmgr) -> pjsip_tp_state_callback;
    pub fn pjsip_transport_add_state_listener( tp: *mut pjsip_transport, cb: pjsip_tp_state_callback, user_data: *mut c_void, key: *mut *mut pjsip_tp_state_listener_key, ) -> pj_status_t;
    pub fn pjsip_transport_remove_state_listener( tp: *mut pjsip_transport, key: *mut pjsip_tp_state_listener_key, user_data: *const c_void, ) -> pj_status_t;
    pub fn pjsip_endpt_create( pf: *mut pj_pool_factory, name: *const c_char, endpt: *mut *mut pjsip_endpoint, ) -> pj_status_t;
    pub fn pjsip_endpt_destroy(endpt: *mut pjsip_endpoint);
    pub fn pjsip_endpt_name(endpt: *const pjsip_endpoint) -> *const pj_str_t;
    pub fn pjsip_endpt_handle_events( endpt: *mut pjsip_endpoint, max_timeout: *const pj_time_val, ) -> pj_status_t;
    pub fn pjsip_endpt_handle_events2( endpt: *mut pjsip_endpoint, max_timeout: *const pj_time_val, count: *mut c_uint, ) -> pj_status_t;
    pub fn pjsip_endpt_schedule_timer_dbg( endpt: *mut pjsip_endpoint, entry: *mut pj_timer_entry, delay: *const pj_time_val, src_file: *const c_char, src_line: c_int, ) -> pj_status_t;
    pub fn pjsip_endpt_schedule_timer_w_grp_lock_dbg( endpt: *mut pjsip_endpoint, entry: *mut pj_timer_entry, delay: *const pj_time_val, id_val: c_int, grp_lock: *mut pj_grp_lock_t, src_file: *const c_char, src_line: c_int, ) -> pj_status_t;
    pub fn pjsip_endpt_cancel_timer(endpt: *mut pjsip_endpoint, entry: *mut pj_timer_entry);
    pub fn pjsip_endpt_get_timer_heap(endpt: *mut pjsip_endpoint) -> *mut pj_timer_heap_t;
    pub fn pjsip_endpt_register_module( endpt: *mut pjsip_endpoint, module: *mut pjsip_module,) -> pj_status_t;
    pub fn pjsip_endpt_unregister_module( endpt: *mut pjsip_endpoint, module: *mut pjsip_module,) -> pj_status_t;
    pub fn pjsip_process_rdata_param_default(p: *mut pjsip_process_rdata_param);
    pub fn pjsip_endpt_process_rx_data( endpt: *mut pjsip_endpoint, rdata: *mut pjsip_rx_data, p: *mut pjsip_process_rdata_param, p_handled: *mut pj_bool_t, ) -> pj_status_t;
    pub fn pjsip_endpt_create_pool( endpt: *mut pjsip_endpoint, pool_name: *const c_char, initial: pj_size_t, increment: pj_size_t, ) -> *mut pj_pool_t;
    pub fn pjsip_endpt_release_pool(endpt: *mut pjsip_endpoint, pool: *mut pj_pool_t);
    pub fn pjsip_endpt_find_tsx( endpt: *mut pjsip_endpoint, key: *const pj_str_t,) -> *mut pjsip_transaction;
    pub fn pjsip_endpt_register_tsx(endpt: *mut pjsip_endpoint, tsx: *mut pjsip_transaction);
    pub fn pjsip_endpt_destroy_tsx(endpt: *mut pjsip_endpoint, tsx: *mut pjsip_transaction);
    pub fn pjsip_endpt_create_tdata( endpt: *mut pjsip_endpoint, p_tdata: *mut *mut pjsip_tx_data,) -> pj_status_t;
    pub fn pjsip_endpt_create_resolver( endpt: *mut pjsip_endpoint, p_resv: *mut *mut pj_dns_resolver, ) -> pj_status_t;
    pub fn pjsip_endpt_set_resolver( endpt: *mut pjsip_endpoint, resv: *mut pj_dns_resolver, ) -> pj_status_t;
    pub fn pjsip_endpt_set_ext_resolver( endpt: *mut pjsip_endpoint, ext_res: *mut pjsip_ext_resolver, ) -> pj_status_t;
    pub fn pjsip_endpt_get_resolver(endpt: *mut pjsip_endpoint) -> *mut pj_dns_resolver;
    pub fn pjsip_endpt_resolve( endpt: *mut pjsip_endpoint, pool: *mut pj_pool_t, target: *mut pjsip_host_info, token: *mut c_void, cb: pjsip_resolver_callback, );
    pub fn pjsip_endpt_get_tpmgr(endpt: *mut pjsip_endpoint) -> *mut pjsip_tpmgr;
    pub fn pjsip_endpt_get_ioqueue(endpt: *mut pjsip_endpoint) -> *mut pj_ioqueue_t;
    pub fn pjsip_endpt_acquire_transport( endpt: *mut pjsip_endpoint, type_: pjsip_transport_type_e, remote: *const pj_sockaddr_t, addr_len: c_int, sel: *const pjsip_tpselector, p_tp: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_endpt_acquire_transport2( endpt: *mut pjsip_endpoint, type_: pjsip_transport_type_e, remote: *const pj_sockaddr_t, addr_len: c_int, sel: *const pjsip_tpselector, tdata: *mut pjsip_tx_data, p_tp: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_endpt_get_capability( endpt: *mut pjsip_endpoint, htype: c_int, hname: *const pj_str_t, ) -> *const pjsip_hdr;
    pub fn pjsip_endpt_has_capability( endpt: *mut pjsip_endpoint, htype: c_int, hname: *const pj_str_t, token: *const pj_str_t, ) -> pj_bool_t;
    pub fn pjsip_endpt_add_capability( endpt: *mut pjsip_endpoint, mod_: *mut pjsip_module, htype: c_int, hname: *const pj_str_t, count: c_uint, tags: *const pj_str_t,) -> pj_status_t;
    pub fn pjsip_endpt_get_request_headers(e: *mut pjsip_endpoint) -> *const pjsip_hdr;
    pub fn pjsip_endpt_dump(endpt: *mut pjsip_endpoint, detail: pj_bool_t);
    pub fn pjsip_endpt_atexit( endpt: *mut pjsip_endpoint, func: pjsip_endpt_exit_callback, ) -> pj_status_t;
    pub fn pjsip_endpt_log_error( endpt: *mut pjsip_endpoint, sender: *const c_char, error_code: pj_status_t, format: *const c_char, ...);
    pub fn pjsip_endpt_send_tsx_event(endpt: *mut pjsip_endpoint, evt: *mut pjsip_event);
    pub fn pjsip_target_set_add_uri( tset: *mut pjsip_target_set, pool: *mut pj_pool_t, uri: *const pjsip_uri, q1000: c_int, ) -> pj_status_t;
    pub fn pjsip_target_set_add_from_msg( tset: *mut pjsip_target_set, pool: *mut pj_pool_t, msg: *const pjsip_msg,) -> pj_status_t;
    pub fn pjsip_target_set_get_next(tset: *const pjsip_target_set) -> *mut pjsip_target;
    pub fn pjsip_target_set_set_current( tset: *mut pjsip_target_set, target: *mut pjsip_target, ) -> pj_status_t;
    pub fn pjsip_target_assign_status( target: *mut pjsip_target, pool: *mut pj_pool_t, status_code: c_int, reason: *const pj_str_t,) -> pj_status_t;
    pub fn pjsip_endpt_create_request( endpt: *mut pjsip_endpoint, method: *const pjsip_method, target: *const pj_str_t, from: *const pj_str_t, to: *const pj_str_t, contact: *const pj_str_t, call_id: *const pj_str_t, cseq: c_int, text: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_endpt_create_request_from_hdr( endpt: *mut pjsip_endpoint, method: *const pjsip_method, target: *const pjsip_uri, from: *const pjsip_from_hdr, to: *const pjsip_to_hdr, contact: *const pjsip_contact_hdr, call_id: *const pjsip_cid_hdr, cseq: c_int, text: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_endpt_create_response( endpt: *mut pjsip_endpoint, rdata: *const pjsip_rx_data, st_code: c_int, st_text: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_endpt_create_ack( endpt: *mut pjsip_endpoint, tdata: *const pjsip_tx_data, rdata: *const pjsip_rx_data, ack: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_endpt_create_cancel( endpt: *mut pjsip_endpoint, tdata: *const pjsip_tx_data, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_get_dest_info( target_uri: *const pjsip_uri, request_uri: *const pjsip_uri, pool: *mut pj_pool_t, dest_info: *mut pjsip_host_info, ) -> pj_status_t;
    pub fn pjsip_get_request_dest( tdata: *const pjsip_tx_data, dest_info: *mut pjsip_host_info, ) -> pj_status_t;
    pub fn pjsip_process_route_set( tdata: *mut pjsip_tx_data, dest_info: *mut pjsip_host_info, ) -> pj_status_t;
    pub fn pjsip_restore_strict_route_set(tdata: *mut pjsip_tx_data);
    pub fn pjsip_endpt_send_request_stateless( endpt: *mut pjsip_endpoint, tdata: *mut pjsip_tx_data, token: *mut c_void, cb: pjsip_send_callback, ) -> pj_status_t;
    pub fn pjsip_endpt_send_raw( endpt: *mut pjsip_endpoint, tp_type: pjsip_transport_type_e, sel: *const pjsip_tpselector, raw_data: *const c_void, data_len: pj_size_t, addr: *const pj_sockaddr_t, addr_len: c_int, token: *mut c_void, cb: pjsip_tp_send_callback, ) -> pj_status_t;
    pub fn pjsip_endpt_send_raw_to_uri( endpt: *mut pjsip_endpoint, dst_uri: *const pj_str_t, sel: *const pjsip_tpselector, raw_data: *const c_void, data_len: pj_size_t, token: *mut c_void, cb: pjsip_tp_send_callback, ) -> pj_status_t;
    pub fn pjsip_get_response_addr( pool: *mut pj_pool_t, rdata: *mut pjsip_rx_data, res_addr: *mut pjsip_response_addr, ) -> pj_status_t;
    pub fn pjsip_endpt_send_response( endpt: *mut pjsip_endpoint, res_addr: *mut pjsip_response_addr, tdata: *mut pjsip_tx_data, token: *mut c_void, cb: pjsip_send_callback, ) -> pj_status_t;
    pub fn pjsip_endpt_send_response2( endpt: *mut pjsip_endpoint, rdata: *mut pjsip_rx_data, tdata: *mut pjsip_tx_data, token: *mut c_void, cb: pjsip_send_callback, ) -> pj_status_t;
    pub fn pjsip_endpt_respond_stateless( endpt: *mut pjsip_endpoint, rdata: *mut pjsip_rx_data, st_code: c_int, st_text: *const pj_str_t, hdr_list: *const pjsip_hdr, body: *const pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_endpt_respond( endpt: *mut pjsip_endpoint, tsx_user: *mut pjsip_module, rdata: *mut pjsip_rx_data, st_code: c_int, st_text: *const pj_str_t, hdr_list: *const pjsip_hdr, body: *const pjsip_msg_body, p_tsx: *mut *mut pjsip_transaction, ) -> pj_status_t;
    pub fn pjsip_endpt_send_request( endpt: *mut pjsip_endpoint, tdata: *mut pjsip_tx_data, timeout: pj_int32_t, token: *mut c_void, cb: pjsip_endpt_send_callback, ) -> pj_status_t;
    pub fn pjsip_endpt_create_request_fwd( endpt: *mut pjsip_endpoint, rdata: *mut pjsip_rx_data, uri: *const pjsip_uri, branch: *const pj_str_t, options: c_uint, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_endpt_create_response_fwd( endpt: *mut pjsip_endpoint, rdata: *mut pjsip_rx_data, options: c_uint, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_calculate_branch_id(rdata: *mut pjsip_rx_data) -> pj_str_t;
    pub fn pjsip_udp_transport_cfg_default( cfg: *mut pjsip_udp_transport_cfg, af: c_int, );
    pub fn pjsip_udp_transport_start2( endpt: *mut pjsip_endpoint, cfg: *const pjsip_udp_transport_cfg, p_transport: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_udp_transport_start( endpt: *mut pjsip_endpoint, local: *const pj_sockaddr_in, a_name: *const pjsip_host_port, async_cnt: c_uint, p_transport: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_udp_transport_start6( endpt: *mut pjsip_endpoint, local: *const pj_sockaddr_in6, a_name: *const pjsip_host_port, async_cnt: c_uint, p_transport: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_udp_transport_attach( endpt: *mut pjsip_endpoint, sock: pj_sock_t, a_name: *const pjsip_host_port, async_cnt: c_uint, p_transport: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_udp_transport_attach2( endpt: *mut pjsip_endpoint, type_: pjsip_transport_type_e, sock: pj_sock_t, a_name: *const pjsip_host_port, async_cnt: c_uint, p_transport: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_udp_transport_get_socket(transport: *mut pjsip_transport) -> pj_sock_t;
    pub fn pjsip_udp_transport_pause( transport: *mut pjsip_transport, option: c_uint, ) -> pj_status_t;
    pub fn pjsip_udp_transport_restart( transport: *mut pjsip_transport, option: c_uint, sock: pj_sock_t, local: *const pj_sockaddr_in, a_name: *const pjsip_host_port, ) -> pj_status_t;
    pub fn pjsip_udp_transport_restart2( transport: *mut pjsip_transport, option: c_uint, sock: pj_sock_t, local: *const pj_sockaddr, a_name: *const pjsip_host_port, ) -> pj_status_t;
    pub fn pjsip_loop_start( endpt: *mut pjsip_endpoint, transport: *mut *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_loop_set_discard( tp: *mut pjsip_transport, discard: pj_bool_t, prev_value: *mut pj_bool_t, ) -> pj_status_t;
    pub fn pjsip_loop_set_failure( tp: *mut pjsip_transport, fail_flag: c_int, prev_value: *mut c_int, ) -> pj_status_t;
    pub fn pjsip_loop_set_recv_delay( tp: *mut pjsip_transport, delay: c_uint, prev_value: *mut c_uint, ) -> pj_status_t;
    pub fn pjsip_loop_set_send_callback_delay( tp: *mut pjsip_transport, delay: c_uint, prev_value: *mut c_uint, ) -> pj_status_t;
    pub fn pjsip_loop_set_delay( tp: *mut pjsip_transport, delay: c_uint, ) -> pj_status_t;
    pub fn pjsip_tcp_transport_cfg_default( cfg: *mut pjsip_tcp_transport_cfg, af: c_int, );
    pub fn pjsip_tcp_transport_start( endpt: *mut pjsip_endpoint, local: *const pj_sockaddr_in, async_cnt: c_uint, p_factory: *mut *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tcp_transport_start2( endpt: *mut pjsip_endpoint, local: *const pj_sockaddr_in, a_name: *const pjsip_host_port, async_cnt: c_uint, p_factory: *mut *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tcp_transport_start3( endpt: *mut pjsip_endpoint, cfg: *const pjsip_tcp_transport_cfg, p_factory: *mut *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tcp_transport_get_socket(transport: *mut pjsip_transport) -> pj_sock_t;
    pub fn pjsip_tcp_transport_lis_start( factory: *mut pjsip_tpfactory, local: *const pj_sockaddr, a_name: *const pjsip_host_port, ) -> pj_status_t;
    pub fn pjsip_tcp_transport_restart( factory: *mut pjsip_tpfactory, local: *const pj_sockaddr, a_name: *const pjsip_host_port, ) -> pj_status_t;
    pub fn pjsip_tls_setting_wipe_keys(opt: *mut pjsip_tls_setting);
    pub fn pjsip_tls_transport_start( endpt: *mut pjsip_endpoint, opt: *const pjsip_tls_setting, local: *const pj_sockaddr_in, a_name: *const pjsip_host_port, async_cnt: c_uint, p_factory: *mut *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tls_transport_start2( endpt: *mut pjsip_endpoint, opt: *const pjsip_tls_setting, local: *const pj_sockaddr, a_name: *const pjsip_host_port, async_cnt: c_uint, p_factory: *mut *mut pjsip_tpfactory, ) -> pj_status_t;
    pub fn pjsip_tls_transport_lis_start( factory: *mut pjsip_tpfactory, local: *const pj_sockaddr, a_name: *const pjsip_host_port, ) -> pj_status_t;
    pub fn pjsip_tls_transport_restart( factory: *mut pjsip_tpfactory, local: *const pj_sockaddr, a_name: *const pjsip_host_port, ) -> pj_status_t;
    pub fn pjsip_authorization_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_authorization_hdr;
    pub fn pjsip_proxy_authorization_hdr_create( pool: *mut pj_pool_t, ) -> *mut pjsip_proxy_authorization_hdr;
    pub fn pjsip_www_authenticate_hdr_create( pool: *mut pj_pool_t, ) -> *mut pjsip_www_authenticate_hdr;
    pub fn pjsip_proxy_authenticate_hdr_create( pool: *mut pj_pool_t, ) -> *mut pjsip_proxy_authenticate_hdr;
    pub fn pjsip_auth_clt_pref_dup( pool: *mut pj_pool_t, dst: *mut pjsip_auth_clt_pref, src: *const pjsip_auth_clt_pref, );
    pub fn pjsip_cred_info_dup( pool: *mut pj_pool_t, dst: *mut pjsip_cred_info, src: *const pjsip_cred_info, );
    pub fn pjsip_cred_info_cmp( cred1: *const pjsip_cred_info, cred2: *const pjsip_cred_info, ) -> c_int;
    pub fn pjsip_auth_clt_init( sess: *mut pjsip_auth_clt_sess, endpt: *mut pjsip_endpoint, pool: *mut pj_pool_t, options: c_uint, ) -> pj_status_t;
    pub fn pjsip_auth_clt_deinit(sess: *mut pjsip_auth_clt_sess) -> pj_status_t;
    pub fn pjsip_auth_clt_clone( pool: *mut pj_pool_t, sess: *mut pjsip_auth_clt_sess, rhs: *const pjsip_auth_clt_sess, ) -> pj_status_t;
    pub fn pjsip_auth_clt_set_credentials( sess: *mut pjsip_auth_clt_sess, cred_cnt: c_int, c: *const pjsip_cred_info, ) -> pj_status_t;
    pub fn pjsip_auth_clt_set_prefs( sess: *mut pjsip_auth_clt_sess, p: *const pjsip_auth_clt_pref, ) -> pj_status_t;
    pub fn pjsip_auth_clt_get_prefs( sess: *mut pjsip_auth_clt_sess, p: *mut pjsip_auth_clt_pref, ) -> pj_status_t;
    pub fn pjsip_auth_clt_init_req( sess: *mut pjsip_auth_clt_sess, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_auth_clt_reinit_req( sess: *mut pjsip_auth_clt_sess, rdata: *const pjsip_rx_data, old_request: *mut pjsip_tx_data, new_request: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_auth_srv_init( pool: *mut pj_pool_t, auth_srv: *mut pjsip_auth_srv, realm: *const pj_str_t, lookup: pjsip_auth_lookup_cred, options: c_uint, ) -> pj_status_t;
    pub fn pjsip_auth_srv_init2( pool: *mut pj_pool_t, auth_srv: *mut pjsip_auth_srv, param: *const pjsip_auth_srv_init_param, ) -> pj_status_t;
    pub fn pjsip_auth_srv_verify( auth_srv: *mut pjsip_auth_srv, rdata: *mut pjsip_rx_data, status_code: *mut c_int, ) -> pj_status_t;
    pub fn pjsip_auth_srv_challenge( auth_srv: *mut pjsip_auth_srv, qop: *const pj_str_t, nonce: *const pj_str_t, opaque: *const pj_str_t, stale: pj_bool_t, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_auth_create_digest( result: *mut pj_str_t, nonce: *const pj_str_t, nc: *const pj_str_t, cnonce: *const pj_str_t, qop: *const pj_str_t, uri: *const pj_str_t, realm: *const pj_str_t, cred_info: *const pjsip_cred_info, method: *const pj_str_t, );
    pub fn pjsip_auth_create_aka_response( pool: *mut pj_pool_t, chal: *const pjsip_digest_challenge, cred: *const pjsip_cred_info, method: *const pj_str_t, auth: *mut pjsip_digest_credential, ) -> pj_status_t;
    pub fn pjsip_tsx_layer_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_tsx_layer_instance() -> *mut pjsip_module;
    pub fn pjsip_tsx_layer_destroy() -> pj_status_t;
    pub fn pjsip_tsx_layer_get_tsx_count() -> c_uint;
    pub fn pjsip_tsx_layer_find_tsx( key: *const pj_str_t, lock: pj_bool_t, ) -> *mut pjsip_transaction;
    pub fn pjsip_tsx_layer_find_tsx2( key: *const pj_str_t, add_ref: pj_bool_t, ) -> *mut pjsip_transaction;
    pub fn pjsip_tsx_create_uac( tsx_user: *mut pjsip_module, tdata: *mut pjsip_tx_data, p_tsx: *mut *mut pjsip_transaction, ) -> pj_status_t;
    pub fn pjsip_tsx_create_uac2( tsx_user: *mut pjsip_module, tdata: *mut pjsip_tx_data, grp_lock: *mut pj_grp_lock_t, p_tsx: *mut *mut pjsip_transaction, ) -> pj_status_t;
    pub fn pjsip_tsx_create_uas( tsx_user: *mut pjsip_module, rdata: *mut pjsip_rx_data, p_tsx: *mut *mut pjsip_transaction, ) -> pj_status_t;
    pub fn pjsip_tsx_create_uas2( tsx_user: *mut pjsip_module, rdata: *mut pjsip_rx_data, grp_lock: *mut pj_grp_lock_t, p_tsx: *mut *mut pjsip_transaction, ) -> pj_status_t;
    pub fn pjsip_tsx_set_transport( tsx: *mut pjsip_transaction, sel: *const pjsip_tpselector, ) -> pj_status_t;
    pub fn pjsip_tsx_recv_msg(tsx: *mut pjsip_transaction, rdata: *mut pjsip_rx_data);
    pub fn pjsip_tsx_send_msg( tsx: *mut pjsip_transaction, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_tsx_retransmit_no_state( tsx: *mut pjsip_transaction, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_tsx_create_key( pool: *mut pj_pool_t, key: *mut pj_str_t, role: pjsip_role_e, method: *const pjsip_method, rdata: *const pjsip_rx_data, ) -> pj_status_t;
    pub fn pjsip_tsx_terminate( tsx: *mut pjsip_transaction, code: c_int, ) -> pj_status_t;
    pub fn pjsip_tsx_stop_retransmit(tsx: *mut pjsip_transaction) -> pj_status_t;
    pub fn pjsip_tsx_set_timeout( tsx: *mut pjsip_transaction, millisec: c_uint, ) -> pj_status_t;
    pub fn pjsip_rdata_get_tsx(rdata: *mut pjsip_rx_data) -> *mut pjsip_transaction;
    pub fn pjsip_tsx_layer_dump(detail: pj_bool_t);
    pub fn pjsip_tsx_state_str(state: pjsip_tsx_state_e) -> *const c_char;
    pub fn pjsip_role_name(role: pjsip_role_e) -> *const c_char;
    pub fn pjsip_ua_init_module( endpt: *mut pjsip_endpoint, prm: *const pjsip_ua_init_param, ) -> pj_status_t;
    pub fn pjsip_ua_instance() -> *mut pjsip_user_agent;
    pub fn pjsip_ua_get_dlg_set_count() -> pj_uint32_t;
    pub fn pjsip_ua_find_dialog( call_id: *const pj_str_t, local_tag: *const pj_str_t, remote_tag: *const pj_str_t, lock_dialog: pj_bool_t, ) -> *mut pjsip_dialog;
    pub fn pjsip_ua_destroy() -> pj_status_t;
    pub fn pjsip_ua_dump(detail: pj_bool_t);
    pub fn pjsip_ua_get_endpt(ua: *mut pjsip_user_agent) -> *mut pjsip_endpoint;
    pub fn pjsip_ua_register_dlg(ua: *mut pjsip_user_agent, dlg: *mut pjsip_dialog) -> pj_status_t;
    pub fn pjsip_ua_unregister_dlg( ua: *mut pjsip_user_agent, dlg: *mut pjsip_dialog, ) -> pj_status_t;
    pub fn pjsip_method_creates_dialog(m: *const pjsip_method) -> pj_bool_t;
    pub fn pjsip_dlg_create_uac( ua: *mut pjsip_user_agent, local_uri: *const pj_str_t, local_contact: *const pj_str_t, remote_uri: *const pj_str_t, target: *const pj_str_t, p_dlg: *mut *mut pjsip_dialog, ) -> pj_status_t;
    pub fn pjsip_dlg_create_uac2( create_param: *const pjsip_dlg_create_uac_param, p_dlg: *mut *mut pjsip_dialog, ) -> pj_status_t;
    pub fn pjsip_dlg_create_uas_and_inc_lock( ua: *mut pjsip_user_agent, rdata: *mut pjsip_rx_data, contact: *const pj_str_t, p_dlg: *mut *mut pjsip_dialog, ) -> pj_status_t;
    pub fn pjsip_dlg_set_transport( dlg: *mut pjsip_dialog, sel: *const pjsip_tpselector, ) -> pj_status_t;
    pub fn pjsip_dlg_set_via_sent_by( dlg: *mut pjsip_dialog, via_addr: *mut pjsip_host_port, via_tp: *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_dlg_fork( original_dlg: *const pjsip_dialog, rdata: *const pjsip_rx_data, new_dlg: *mut *mut pjsip_dialog, ) -> pj_status_t;
    pub fn pjsip_dlg_terminate(dlg: *mut pjsip_dialog) -> pj_status_t;
    pub fn pjsip_dlg_set_route_set( dlg: *mut pjsip_dialog, route_set: *const pjsip_route_hdr, ) -> pj_status_t;
    pub fn pjsip_dlg_inc_session(dlg: *mut pjsip_dialog, mod_: *mut pjsip_module) -> pj_status_t;
    pub fn pjsip_dlg_dec_session(dlg: *mut pjsip_dialog, mod_: *mut pjsip_module) -> pj_status_t;
    pub fn pjsip_dlg_add_usage( dlg: *mut pjsip_dialog, module: *mut pjsip_module, mod_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsip_dlg_has_usage(dlg: *mut pjsip_dialog, module: *mut pjsip_module) -> pj_bool_t;
    pub fn pjsip_dlg_set_mod_data( dlg: *mut pjsip_dialog, mod_id: c_int, data: *mut c_void, ) -> pj_status_t;
    pub fn pjsip_dlg_get_mod_data( dlg: *mut pjsip_dialog, mod_id: c_int, ) -> *mut c_void;
    pub fn pjsip_dlg_inc_lock(dlg: *mut pjsip_dialog);
    pub fn pjsip_dlg_try_inc_lock(dlg: *mut pjsip_dialog) -> pj_status_t;
    pub fn pjsip_dlg_dec_lock(dlg: *mut pjsip_dialog);
    pub fn pjsip_dlg_get_lock(dlg: *mut pjsip_dialog) -> *mut pj_grp_lock_t;
    pub fn pjsip_rdata_get_dlg(rdata: *mut pjsip_rx_data) -> *mut pjsip_dialog;
    pub fn pjsip_tdata_get_dlg(tdata: *mut pjsip_tx_data) -> *mut pjsip_dialog;
    pub fn pjsip_tsx_get_dlg(tsx: *mut pjsip_transaction) -> *mut pjsip_dialog;
    pub fn pjsip_dlg_create_request( dlg: *mut pjsip_dialog, method: *const pjsip_method, cseq: c_int, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_dlg_send_request( dlg: *mut pjsip_dialog, tdata: *mut pjsip_tx_data, mod_data_id: c_int, mod_data: *mut c_void, ) -> pj_status_t;
    pub fn pjsip_dlg_create_response( dlg: *mut pjsip_dialog, rdata: *mut pjsip_rx_data, st_code: c_int, st_text: *const pj_str_t, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_dlg_modify_response( dlg: *mut pjsip_dialog, tdata: *mut pjsip_tx_data, st_code: c_int, st_text: *const pj_str_t, ) -> pj_status_t;
    pub fn pjsip_dlg_send_response( dlg: *mut pjsip_dialog, tsx: *mut pjsip_transaction, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_dlg_respond( dlg: *mut pjsip_dialog, rdata: *mut pjsip_rx_data, st_code: c_int, st_text: *const pj_str_t, hdr_list: *const pjsip_hdr, body: *const pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_dlg_remote_has_cap( dlg: *mut pjsip_dialog, htype: c_int, hname: *const pj_str_t, token: *const pj_str_t, ) -> pjsip_dialog_cap_status;
    pub fn pjsip_dlg_get_remote_cap_hdr( dlg: *mut pjsip_dialog, htype: c_int, hname: *const pj_str_t, ) -> *const pjsip_hdr;
    pub fn pjsip_dlg_set_remote_cap_hdr( dlg: *mut pjsip_dialog, cap_hdr: *const pjsip_generic_array_hdr, ) -> pj_status_t;
    pub fn pjsip_dlg_remove_remote_cap_hdr( dlg: *mut pjsip_dialog, htype: c_int, hname: *const pj_str_t, ) -> pj_status_t;
    pub fn pjsip_dlg_update_remote_cap( dlg: *mut pjsip_dialog, msg: *const pjsip_msg, strict: pj_bool_t, ) -> pj_status_t;
    pub fn pjsip_dlg_on_tsx_state( dlg: *mut pjsip_dialog, tsx: *mut pjsip_transaction, e: *mut pjsip_event, );
    pub fn pjsip_dlg_on_rx_request(dlg: *mut pjsip_dialog, rdata: *mut pjsip_rx_data);
    pub fn pjsip_dlg_on_rx_response(dlg: *mut pjsip_dialog, rdata: *mut pjsip_rx_data);


    pub fn pjsip_inv_usage_init( endpt: *mut pjsip_endpoint, cb: *const pjsip_inv_callback, ) -> pj_status_t;
    pub fn pjsip_inv_usage_instance() -> *mut pjsip_module;
    pub fn pjsip_inv_usage_dump();
    pub fn pjsip_inv_create_uac( dlg: *mut pjsip_dialog, local_sdp: *const pjmedia_sdp_session, options: c_uint, p_inv: *mut *mut pjsip_inv_session, ) -> pj_status_t;
    pub fn pjsip_inv_verify_request( rdata: *mut pjsip_rx_data, options: *mut c_uint, sdp: *const pjmedia_sdp_session, dlg: *mut pjsip_dialog, endpt: *mut pjsip_endpoint, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_verify_request2( rdata: *mut pjsip_rx_data, options: *mut c_uint, offer: *const pjmedia_sdp_session, answer: *const pjmedia_sdp_session, dlg: *mut pjsip_dialog, endpt: *mut pjsip_endpoint, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_verify_request3( rdata: *mut pjsip_rx_data, tmp_pool: *mut pj_pool_t, options: *mut c_uint, offer: *const pjmedia_sdp_session, answer: *const pjmedia_sdp_session, dlg: *mut pjsip_dialog, endpt: *mut pjsip_endpoint, tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_create_uas( dlg: *mut pjsip_dialog, rdata: *mut pjsip_rx_data, local_sdp: *const pjmedia_sdp_session, options: c_uint, p_inv: *mut *mut pjsip_inv_session, ) -> pj_status_t;
    pub fn pjsip_inv_add_ref(inv: *mut pjsip_inv_session) -> pj_status_t;
    pub fn pjsip_inv_dec_ref(inv: *mut pjsip_inv_session) -> pj_status_t;
    pub fn pjsip_inv_terminate( inv: *mut pjsip_inv_session, st_code: c_int, notify: pj_bool_t, ) -> pj_status_t;
    pub fn pjsip_inv_uac_restart(inv: *mut pjsip_inv_session, new_offer: pj_bool_t) -> pj_status_t;
    pub fn pjsip_inv_process_redirect( inv: *mut pjsip_inv_session, cmd: pjsip_redirect_op, e: *mut pjsip_event, ) -> pj_status_t;
    pub fn pjsip_inv_invite( inv: *mut pjsip_inv_session, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_initial_answer( inv: *mut pjsip_inv_session, rdata: *mut pjsip_rx_data, st_code: c_int, st_text: *const pj_str_t, sdp: *const pjmedia_sdp_session, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_answer( inv: *mut pjsip_inv_session, st_code: c_int, st_text: *const pj_str_t, local_sdp: *const pjmedia_sdp_session, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_set_local_sdp( inv: *mut pjsip_inv_session, sdp: *const pjmedia_sdp_session, ) -> pj_status_t;
    pub fn pjsip_inv_set_sdp_answer( inv: *mut pjsip_inv_session, sdp: *const pjmedia_sdp_session, ) -> pj_status_t;
    pub fn pjsip_inv_end_session( inv: *mut pjsip_inv_session, st_code: c_int, st_text: *const pj_str_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_cancel_reinvite( inv: *mut pjsip_inv_session, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_reinvite( inv: *mut pjsip_inv_session, new_contact: *const pj_str_t, new_offer: *const pjmedia_sdp_session, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_update( inv: *mut pjsip_inv_session, new_contact: *const pj_str_t, offer: *const pjmedia_sdp_session, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_create_ack( inv: *mut pjsip_inv_session, cseq: c_int, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_inv_send_msg( inv: *mut pjsip_inv_session, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_dlg_get_inv_session(dlg: *mut pjsip_dialog) -> *mut pjsip_inv_session;
    pub fn pjsip_tsx_get_inv_session(tsx: *mut pjsip_transaction) -> *mut pjsip_inv_session;
    pub fn pjsip_inv_state_name(state: pjsip_inv_state) -> *const c_char;
    pub fn pjsip_create_sdp_body( pool: *mut pj_pool_t, sdp: *mut pjmedia_sdp_session, p_body: *mut *mut pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_rdata_get_sdp_info(rdata: *mut pjsip_rx_data) -> *mut pjsip_rdata_sdp_info;
    pub fn pjsip_rdata_get_sdp_info2( rdata: *mut pjsip_rx_data, med_type: *const pjsip_media_type, ) -> *mut pjsip_rdata_sdp_info;

    pub fn pjsip_regc_get_module() -> *mut pjsip_module;
    pub fn pjsip_regc_create( endpt: *mut pjsip_endpoint, token: *mut c_void, cb: pjsip_regc_cb, p_regc: *mut *mut pjsip_regc, ) -> pj_status_t;
    pub fn pjsip_regc_destroy(regc: *mut pjsip_regc) -> pj_status_t;
    pub fn pjsip_regc_get_info(regc: *mut pjsip_regc, info: *mut pjsip_regc_info) -> pj_status_t;
    pub fn pjsip_regc_get_pool(regc: *mut pjsip_regc) -> *mut pj_pool_t;
    pub fn pjsip_regc_init( regc: *mut pjsip_regc, srv_url: *const pj_str_t, from_url: *const pj_str_t, to_url: *const pj_str_t, ccnt: c_int, contact: *const pj_str_t, expires: pj_uint32_t, ) -> pj_status_t;
    pub fn pjsip_regc_add_ref(regc: *mut pjsip_regc);
    pub fn pjsip_regc_dec_ref(regc: *mut pjsip_regc) -> pj_status_t;
    pub fn pjsip_regc_set_reg_tsx_cb( regc: *mut pjsip_regc, tsx_cb: pjsip_regc_tsx_cb, ) -> pj_status_t;
    pub fn pjsip_regc_set_via_sent_by( regc: *mut pjsip_regc, via_addr: *mut pjsip_host_port, via_tp: *mut pjsip_transport, ) -> pj_status_t;
    pub fn pjsip_regc_set_delay_before_refresh( regc: *mut pjsip_regc, delay: pj_uint32_t, ) -> pj_status_t;
    pub fn pjsip_regc_set_credentials( regc: *mut pjsip_regc, count: c_int, cred: *const pjsip_cred_info, ) -> pj_status_t;
    pub fn pjsip_regc_set_prefs( regc: *mut pjsip_regc, pref: *const pjsip_auth_clt_pref, ) -> pj_status_t;
    pub fn pjsip_regc_set_route_set( regc: *mut pjsip_regc, route_set: *const pjsip_route_hdr, ) -> pj_status_t;
    pub fn pjsip_regc_set_transport( regc: *mut pjsip_regc, sel: *const pjsip_tpselector, ) -> pj_status_t;
    pub fn pjsip_regc_release_transport(regc: *mut pjsip_regc) -> pj_status_t;
    pub fn pjsip_regc_add_headers(regc: *mut pjsip_regc, hdr_list: *const pjsip_hdr) -> pj_status_t;
    pub fn pjsip_regc_register( regc: *mut pjsip_regc, autoreg: pj_bool_t, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_regc_unregister( regc: *mut pjsip_regc, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_regc_unregister_all( regc: *mut pjsip_regc, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_regc_update_contact( regc: *mut pjsip_regc, ccnt: c_int, contact: *const pj_str_t, ) -> pj_status_t;
    pub fn pjsip_regc_update_expires(regc: *mut pjsip_regc, expires: pj_uint32_t) -> pj_status_t;
    pub fn pjsip_regc_send(regc: *mut pjsip_regc, tdata: *mut pjsip_tx_data) -> pj_status_t;

    pub static pjsip_subscribe_method: pjsip_method;
    pub static pjsip_notify_method: pjsip_method;
    pub fn pjsip_get_subscribe_method() -> *const pjsip_method;
    pub fn pjsip_get_notify_method() -> *const pjsip_method;
    pub static pjsip_refer_method: pjsip_method;
    pub fn pjsip_get_refer_method() -> *const pjsip_method;
    pub static pjsip_prack_method: pjsip_method;
    pub fn pjsip_get_prack_method() -> *const pjsip_method;
    pub fn pjsip_100rel_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_100rel_attach(inv: *mut pjsip_inv_session) -> pj_status_t;
    pub fn pjsip_100rel_is_reliable(rdata: *mut pjsip_rx_data) -> pj_bool_t;
    pub fn pjsip_100rel_create_prack( inv: *mut pjsip_inv_session, rdata: *mut pjsip_rx_data, p_tdata: *mut *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_100rel_send_prack( inv: *mut pjsip_inv_session, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_100rel_on_rx_prack( inv: *mut pjsip_inv_session, rdata: *mut pjsip_rx_data, ) -> pj_status_t;
    pub fn pjsip_100rel_tx_response( inv: *mut pjsip_inv_session, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_100rel_end_session(inv: *mut pjsip_inv_session) -> pj_status_t;
    pub fn pjsip_timer_init_module(endpt: *mut pjsip_endpoint) -> pj_status_t;
    pub fn pjsip_timer_setting_default(setting: *mut pjsip_timer_setting) -> pj_status_t;
    pub fn pjsip_timer_init_session( inv: *mut pjsip_inv_session, setting: *const pjsip_timer_setting, ) -> pj_status_t;
    pub fn pjsip_sess_expires_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_sess_expires_hdr;
    pub fn pjsip_min_se_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_min_se_hdr;
    pub fn pjsip_timer_update_req( inv: *mut pjsip_inv_session, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_timer_process_resp( inv: *mut pjsip_inv_session, rdata: *const pjsip_rx_data, st_code: *mut pjsip_status_code, ) -> pj_status_t;
    pub fn pjsip_timer_handle_refresh_error( inv: *mut pjsip_inv_session, event: *mut pjsip_event, ) -> pj_status_t;
    pub fn pjsip_timer_process_req( inv: *mut pjsip_inv_session, rdata: *const pjsip_rx_data, st_code: *mut pjsip_status_code, ) -> pj_status_t;
    pub fn pjsip_timer_update_resp( inv: *mut pjsip_inv_session, tdata: *mut pjsip_tx_data, ) -> pj_status_t;
    pub fn pjsip_timer_end_session(inv: *mut pjsip_inv_session) -> pj_status_t;
    pub fn pjsip_event_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_event_hdr;
    pub fn pjsip_allow_events_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_allow_events_hdr;
    pub fn pjsip_sub_state_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_sub_state_hdr;

    pub fn pjsip_msg_body_copy( pool: *mut pj_pool_t, dst_body: *mut pjsip_msg_body, src_body: *const pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_msg_body_clone( pool: *mut pj_pool_t, body: *const pjsip_msg_body, ) -> *mut pjsip_msg_body;
    pub fn pjsip_msg_body_create( pool: *mut pj_pool_t, type_: *const pj_str_t, subtype: *const pj_str_t, text: *const pj_str_t, ) -> *mut pjsip_msg_body;

    pub fn pjsip_msg_create(pool: *mut pj_pool_t, type_: pjsip_msg_type_e) -> *mut pjsip_msg;
    pub fn pjsip_msg_clone(pool: *mut pj_pool_t, msg: *const pjsip_msg) -> *mut pjsip_msg;
    pub fn pjsip_msg_find_hdr( msg: *const pjsip_msg, type_: pjsip_hdr_e, start: *const c_void, ) -> *mut c_void;
    pub fn pjsip_msg_find_hdr_by_name( msg: *const pjsip_msg, name: *const pj_str_t, start: *const c_void, ) -> *mut c_void;
    pub fn pjsip_msg_find_hdr_by_names( msg: *const pjsip_msg, name: *const pj_str_t, sname: *const pj_str_t, start: *const c_void, ) -> *mut c_void;
    pub fn pjsip_msg_find_remove_hdr( msg: *mut pjsip_msg, hdr: pjsip_hdr_e, start: *mut c_void, ) -> *mut c_void;
    pub fn pjsip_msg_print( msg: *const pjsip_msg, buf: *mut c_char, size: pj_size_t,) -> pj_ssize_t;

}