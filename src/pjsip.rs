#![allow(non_camel_case_types, unused_variables)]
#![allow(dead_code)]
use super::pjdefault::AutoCreate;
use super::pjsua_sys::*;
use std::ptr;


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
pub type pjsip_hdr_e = ::std::os::raw::c_uint;

pub const pjsip_uri_context_e_PJSIP_URI_IN_REQ_URI: pjsip_uri_context_e = 0;
pub const pjsip_uri_context_e_PJSIP_URI_IN_FROMTO_HDR: pjsip_uri_context_e = 1;
pub const pjsip_uri_context_e_PJSIP_URI_IN_CONTACT_HDR: pjsip_uri_context_e = 2;
pub const pjsip_uri_context_e_PJSIP_URI_IN_ROUTING_HDR: pjsip_uri_context_e = 3;
pub const pjsip_uri_context_e_PJSIP_URI_IN_OTHER: pjsip_uri_context_e = 4;
pub type pjsip_uri_context_e = ::std::os::raw::c_uint;

pub const pjsip_method_e_PJSIP_INVITE_METHOD: pjsip_method_e = 0;
pub const pjsip_method_e_PJSIP_CANCEL_METHOD: pjsip_method_e = 1;
pub const pjsip_method_e_PJSIP_ACK_METHOD: pjsip_method_e = 2;
pub const pjsip_method_e_PJSIP_BYE_METHOD: pjsip_method_e = 3;
pub const pjsip_method_e_PJSIP_REGISTER_METHOD: pjsip_method_e = 4;
pub const pjsip_method_e_PJSIP_OPTIONS_METHOD: pjsip_method_e = 5;
pub const pjsip_method_e_PJSIP_OTHER_METHOD: pjsip_method_e = 6;
pub type pjsip_method_e = ::std::os::raw::c_uint;

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
pub type pjsip_transport_type_e = ::std::os::raw::c_uint;

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
pub type pjsip_status_code = ::std::os::raw::c_uint;

pub type pjsip_user_agent = pjsip_module;
pub const pjsip_role_e_PJSIP_ROLE_UAC: pjsip_role_e = 0;
pub const pjsip_role_e_PJSIP_ROLE_UAS: pjsip_role_e = 1;
pub const pjsip_role_e_PJSIP_UAC_ROLE: pjsip_role_e = 0;
pub const pjsip_role_e_PJSIP_UAS_ROLE: pjsip_role_e = 1;
pub type pjsip_role_e = ::std::os::raw::c_uint;

pub const pjsip_msg_type_e_PJSIP_REQUEST_MSG: pjsip_msg_type_e = 0;
pub const pjsip_msg_type_e_PJSIP_RESPONSE_MSG: pjsip_msg_type_e = 1;
pub type pjsip_msg_type_e = ::std::os::raw::c_uint;

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
pub const PJSIP_PARSE_URI_AS_NAMEADDR: ::std::os::raw::c_uint = 1;
pub const PJSIP_PARSE_URI_IN_FROM_TO_HDR: ::std::os::raw::c_uint = 2;
pub const PJSIP_PARSE_REMOVE_QUOTE: ::std::os::raw::c_uint = 1;

pub type pjsip_parse_hdr_func = ::std::option::Option<unsafe extern "C" fn(context: *mut pjsip_parse_ctx) -> *mut pjsip_hdr>;
pub type pjsip_parse_uri_func = ::std::option::Option<
unsafe extern "C" fn(
        scanner: *mut pj_scanner,
        pool: *mut pj_pool_t,
        parse_params: pj_bool_t,
    ) -> *mut ::std::os::raw::c_void,
>;

pub const pjsip_event_id_e_PJSIP_EVENT_UNKNOWN: pjsip_event_id_e = 0;
pub const pjsip_event_id_e_PJSIP_EVENT_TIMER: pjsip_event_id_e = 1;
pub const pjsip_event_id_e_PJSIP_EVENT_TX_MSG: pjsip_event_id_e = 2;
pub const pjsip_event_id_e_PJSIP_EVENT_RX_MSG: pjsip_event_id_e = 3;
pub const pjsip_event_id_e_PJSIP_EVENT_TRANSPORT_ERROR: pjsip_event_id_e = 4;
pub const pjsip_event_id_e_PJSIP_EVENT_TSX_STATE: pjsip_event_id_e = 5;
pub const pjsip_event_id_e_PJSIP_EVENT_USER: pjsip_event_id_e = 6;
pub type pjsip_event_id_e = ::std::os::raw::c_uint;




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
pub struct pjsip_uri_vptr {
    pub p_get_scheme: ::std::option::Option<
        unsafe extern "C" fn(uri: *const ::std::os::raw::c_void) -> *const pj_str_t,
    >,
    pub p_get_uri: ::std::option::Option<
        unsafe extern "C" fn(uri: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
    >,
    pub p_print: ::std::option::Option<
        unsafe extern "C" fn(
            context: pjsip_uri_context_e,
            uri: *const ::std::os::raw::c_void,
            buf: *mut ::std::os::raw::c_char,
            size: pj_size_t,
        ) -> pj_ssize_t,
    >,
    pub p_compare: ::std::option::Option<
        unsafe extern "C" fn(
            context: pjsip_uri_context_e,
            uri1: *const ::std::os::raw::c_void,
            uri2: *const ::std::os::raw::c_void,
        ) -> pj_status_t,
    >,
    pub p_clone: ::std::option::Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            uri: *const ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
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
    pub port: ::std::os::raw::c_int,
    pub user_param: pj_str_t,
    pub method_param: pj_str_t,
    pub transport_param: pj_str_t,
    pub ttl_param: ::std::os::raw::c_int,
    pub lr_param: ::std::os::raw::c_int,
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
    pub max_count: ::std::os::raw::c_uint,
    pub t1: ::std::os::raw::c_uint,
    pub t2: ::std::os::raw::c_uint,
    pub t4: ::std::os::raw::c_uint,
    pub td: ::std::os::raw::c_uint,
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
    pub keep_alive_interval: ::std::os::raw::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_cfg_t__bindgen_ty_5 {
    pub keep_alive_interval: ::std::os::raw::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_tpmgr {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_endpoint {
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
    pub start: *mut ::std::os::raw::c_char,
    pub cur: *mut ::std::os::raw::c_char,
    pub end: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_host_port {
    pub host: pj_str_t,
    pub port: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_host_info {
    pub flag: ::std::os::raw::c_uint,
    pub type_: pjsip_transport_type_e,
    pub addr: pjsip_host_port,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_hdr_vptr {
    pub clone: ::std::option::Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            hdr: *const ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub shallow_clone: ::std::option::Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            hdr: *const ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub print_on: ::std::option::Option<
        unsafe extern "C" fn(
            hdr: *mut ::std::os::raw::c_void,
            buf: *mut ::std::os::raw::c_char,
            len: pj_size_t,
        ) -> ::std::os::raw::c_int,
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
    pub code: ::std::os::raw::c_int,
    pub reason: pj_str_t,
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
    pub data: *mut ::std::os::raw::c_void,
    pub len: ::std::os::raw::c_uint,
    pub print_body: ::std::option::Option<
        unsafe extern "C" fn(
            msg_body: *mut pjsip_msg_body,
            buf: *mut ::std::os::raw::c_char,
            size: pj_size_t,
        ) -> ::std::os::raw::c_int,
        >,
    pub clone_data: ::std::option::Option<
        unsafe extern "C" fn(
            pool: *mut pj_pool_t,
            data: *const ::std::os::raw::c_void,
            len: ::std::os::raw::c_uint,
        ) -> *mut ::std::os::raw::c_void,
        >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pjsip_msg {
    pub type_: pjsip_msg_type_e,
    pub line: pjsip_msg__bindgen_ty_1,
    pub hdr: pjsip_hdr,
    pub body: *mut pjsip_msg_body,
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
    pub count: ::std::os::raw::c_uint,
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
    pub len: ::std::os::raw::c_int,
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
    pub star: ::std::os::raw::c_int,
    pub uri: *mut pjsip_uri,
    pub q1000: ::std::os::raw::c_int,
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
    pub ttl_param: ::std::os::raw::c_int,
    pub rport_param: ::std::os::raw::c_int,
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
    pub except_code: ::std::os::raw::c_int,
    pub line: ::std::os::raw::c_int,
    pub col: ::std::os::raw::c_int,
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
    pub prev_state: ::std::os::raw::c_int,
    pub type_: pjsip_event_id_e,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pjsip_event__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    pub rdata: *mut pjsip_rx_data,
    pub tdata: *mut pjsip_tx_data,
    pub timer: *mut pj_timer_entry,
    pub status: pj_status_t,
    pub data: *mut ::std::os::raw::c_void,
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
    pub user1: *mut ::std::os::raw::c_void,
    pub user2: *mut ::std::os::raw::c_void,
    pub user3: *mut ::std::os::raw::c_void,
    pub user4: *mut ::std::os::raw::c_void,
}

extern "C" {
    pub fn pjsip_event_str(e: pjsip_event_id_e) -> *const ::std::os::raw::c_char;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pjsip_module {
    pub prev: *mut pjsip_module,
    pub next: *mut pjsip_module,
    pub name: pj_str_t,
    pub id: ::std::os::raw::c_int,
    pub priority: ::std::os::raw::c_int,
    pub load:
        ::std::option::Option<unsafe extern "C" fn(endpt: *mut pjsip_endpoint) -> pj_status_t>,
    pub start: ::std::option::Option<unsafe extern "C" fn() -> pj_status_t>,
    pub stop: ::std::option::Option<unsafe extern "C" fn() -> pj_status_t>,
    pub unload: ::std::option::Option<unsafe extern "C" fn() -> pj_status_t>,
    pub on_rx_request:
        ::std::option::Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>,
    pub on_rx_response:
        ::std::option::Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>,
    pub on_tx_request:
        ::std::option::Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>,
    pub on_tx_response:
        ::std::option::Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>,
    pub on_tsx_state: ::std::option::Option<
        unsafe extern "C" fn(tsx: *mut pjsip_transaction, event: *mut pjsip_event),
    >,
}

pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_TRANSPORT_LAYER: pjsip_module_priority = 8;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_TSX_LAYER: pjsip_module_priority = 16;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_UA_PROXY_LAYER: pjsip_module_priority = 32;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_DIALOG_USAGE: pjsip_module_priority = 48;
pub const pjsip_module_priority_PJSIP_MOD_PRIORITY_APPLICATION: pjsip_module_priority = 64;
pub type pjsip_module_priority = ::std::os::raw::c_uint;

extern "C" {
    pub fn pjsip_param_find( param_list: *const pjsip_param, name: *const pj_str_t, ) -> *mut pjsip_param;
    pub fn pjsip_param_cmp( param_list1: *const pjsip_param, param_list2: *const pjsip_param, ig_nf: pj_bool_t, ) -> ::std::os::raw::c_int;
    pub fn pjsip_param_clone( pool: *mut pj_pool_t, dst_list: *mut pjsip_param, src_list: *const pjsip_param, );
    pub fn pjsip_param_shallow_clone( pool: *mut pj_pool_t, dst_list: *mut pjsip_param, src_list: *const pjsip_param,);
    pub fn pjsip_param_print_on( param_list: *const pjsip_param, buf: *mut ::std::os::raw::c_char, size: pj_size_t, pname_unres: *const pj_cis_t, pvalue_unres: *const pj_cis_t, sep: ::std::os::raw::c_int, ) -> pj_ssize_t;
    pub fn pjsip_sip_uri_create(pool: *mut pj_pool_t, secure: pj_bool_t) -> *mut pjsip_sip_uri;
    pub fn pjsip_sip_uri_set_secure(uri: *mut pjsip_sip_uri, secure: pj_bool_t);
    pub fn pjsip_sip_uri_init(url: *mut pjsip_sip_uri, secure: pj_bool_t);
    pub fn pjsip_sip_uri_assign( pool: *mut pj_pool_t, url: *mut pjsip_sip_uri, rhs: *const pjsip_sip_uri, );
    pub fn pjsip_name_addr_create(pool: *mut pj_pool_t) -> *mut pjsip_name_addr;
    pub fn pjsip_name_addr_init(name_addr: *mut pjsip_name_addr);
    pub fn pjsip_name_addr_assign( pool: *mut pj_pool_t, addr: *mut pjsip_name_addr, rhs: *const pjsip_name_addr, );
    pub fn pjsip_other_uri_create(pool: *mut pj_pool_t) -> *mut pjsip_other_uri;
    pub fn pjsip_tel_uri_create(pool: *mut pj_pool_t) -> *mut pjsip_tel_uri;
    pub fn pjsip_tel_nb_cmp(nb1: *const pj_str_t, nb2: *const pj_str_t) -> ::std::os::raw::c_int;
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
    pub fn pjsip_method_cmp( m1: *const pjsip_method, m2: *const pjsip_method, ) -> ::std::os::raw::c_int;
    pub static mut pjsip_sip_cfg_var: pjsip_cfg_t;
    pub fn pjsip_exception_to_status(exception_id: ::std::os::raw::c_int) -> pj_status_t;
    pub fn pjsip_hdr_clone( pool: *mut pj_pool_t, hdr: *const ::std::os::raw::c_void, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_hdr_shallow_clone( pool: *mut pj_pool_t, hdr: *const ::std::os::raw::c_void, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_hdr_print_on( hdr: *mut ::std::os::raw::c_void, buf: *mut ::std::os::raw::c_char, len: pj_size_t, ) -> ::std::os::raw::c_int;
    pub fn pjsip_get_status_text(status_code: ::std::os::raw::c_int) -> *const pj_str_t;
    pub fn pjsip_print_text_body( msg_body: *mut pjsip_msg_body, buf: *mut ::std::os::raw::c_char, size: pj_size_t, ) -> ::std::os::raw::c_int;
    pub fn pjsip_clone_text_data( pool: *mut pj_pool_t, data: *const ::std::os::raw::c_void, len: ::std::os::raw::c_uint, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_msg_body_copy( pool: *mut pj_pool_t, dst_body: *mut pjsip_msg_body, src_body: *const pjsip_msg_body, ) -> pj_status_t;
    pub fn pjsip_msg_body_clone( pool: *mut pj_pool_t, body: *const pjsip_msg_body, ) -> *mut pjsip_msg_body;
    pub fn pjsip_msg_body_create( pool: *mut pj_pool_t, type_: *const pj_str_t, subtype: *const pj_str_t, text: *const pj_str_t, ) -> *mut pjsip_msg_body;
    pub fn pjsip_media_type_init( mt: *mut pjsip_media_type, type_: *mut pj_str_t, subtype: *mut pj_str_t, );
    pub fn pjsip_media_type_init2( mt: *mut pjsip_media_type, type_: *mut ::std::os::raw::c_char, subtype: *mut ::std::os::raw::c_char, );
    pub fn pjsip_media_type_cmp( mt1: *const pjsip_media_type, mt2: *const pjsip_media_type, cmp_param: ::std::os::raw::c_int, ) -> ::std::os::raw::c_int;
    pub fn pjsip_media_type_cp( pool: *mut pj_pool_t, dst: *mut pjsip_media_type, src: *const pjsip_media_type, );
    pub fn pjsip_media_type_print( buf: *mut ::std::os::raw::c_char, len: ::std::os::raw::c_uint, mt: *const pjsip_media_type, ) -> ::std::os::raw::c_int;
    pub fn pjsip_msg_create(pool: *mut pj_pool_t, type_: pjsip_msg_type_e) -> *mut pjsip_msg;
    pub fn pjsip_msg_clone(pool: *mut pj_pool_t, msg: *const pjsip_msg) -> *mut pjsip_msg;
    pub fn pjsip_msg_find_hdr( msg: *const pjsip_msg, type_: pjsip_hdr_e, start: *const ::std::os::raw::c_void, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_msg_find_hdr_by_name( msg: *const pjsip_msg, name: *const pj_str_t, start: *const ::std::os::raw::c_void, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_msg_find_hdr_by_names( msg: *const pjsip_msg, name: *const pj_str_t, sname: *const pj_str_t, start: *const ::std::os::raw::c_void, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_msg_find_remove_hdr( msg: *mut pjsip_msg, hdr: pjsip_hdr_e, start: *mut ::std::os::raw::c_void, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_msg_print( msg: *const pjsip_msg, buf: *mut ::std::os::raw::c_char, size: pj_size_t,) -> pj_ssize_t;
    pub fn pjsip_generic_string_hdr_create( pool: *mut pj_pool_t, hname: *const pj_str_t, hvalue: *const pj_str_t, ) -> *mut pjsip_generic_string_hdr;
    pub fn pjsip_generic_string_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, hname: *const pj_str_t, hvalue: *const pj_str_t, ) -> *mut pjsip_generic_string_hdr;
    pub fn pjsip_generic_string_hdr_init2( h: *mut pjsip_generic_string_hdr, hname: *mut pj_str_t, hvalue: *mut pj_str_t,);
    pub fn pjsip_generic_int_hdr_create( pool: *mut pj_pool_t, hname: *const pj_str_t, hvalue: ::std::os::raw::c_uint, ) -> *mut pjsip_generic_int_hdr;
    pub fn pjsip_generic_int_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, hname: *const pj_str_t, value: ::std::os::raw::c_uint, ) -> *mut pjsip_generic_int_hdr;
    pub fn pjsip_generic_array_hdr_create( pool: *mut pj_pool_t, hname: *const pj_str_t, ) -> *mut pjsip_generic_array_hdr;
    pub fn pjsip_generic_array_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, hname: *const pj_str_t, ) -> *mut pjsip_generic_array_hdr;
    pub fn pjsip_accept_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_accept_hdr;
    pub fn pjsip_accept_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_accept_hdr;
    pub fn pjsip_allow_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_allow_hdr;
    pub fn pjsip_allow_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_allow_hdr;
    pub fn pjsip_cid_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_cid_hdr;
    pub fn pjsip_cid_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_cid_hdr;
    pub fn pjsip_clen_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_clen_hdr;
    pub fn pjsip_clen_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_clen_hdr;
    pub fn pjsip_cseq_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_cseq_hdr;
    pub fn pjsip_cseq_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_cseq_hdr;
    pub fn pjsip_contact_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_contact_hdr;
    pub fn pjsip_contact_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void,) -> *mut pjsip_contact_hdr;
    pub fn pjsip_ctype_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_ctype_hdr;
    pub fn pjsip_ctype_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_ctype_hdr;
    pub fn pjsip_expires_hdr_create( pool: *mut pj_pool_t, value: ::std::os::raw::c_uint, ) -> *mut pjsip_expires_hdr;
    pub fn pjsip_expires_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, value: ::std::os::raw::c_uint, ) -> *mut pjsip_expires_hdr;
    pub fn pjsip_from_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_from_hdr;
    pub fn pjsip_from_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_from_hdr;
    pub fn pjsip_to_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_to_hdr;
    pub fn pjsip_to_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_to_hdr;
    pub fn pjsip_fromto_hdr_set_from(hdr: *mut pjsip_fromto_hdr) -> *mut pjsip_from_hdr;
    pub fn pjsip_fromto_hdr_set_to(hdr: *mut pjsip_fromto_hdr) -> *mut pjsip_to_hdr;
    pub fn pjsip_max_fwd_hdr_create( pool: *mut pj_pool_t, value: ::std::os::raw::c_uint, ) -> *mut pjsip_max_fwd_hdr;
    pub fn pjsip_max_fwd_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, value: ::std::os::raw::c_uint, ) -> *mut pjsip_max_fwd_hdr;
    pub fn pjsip_min_expires_hdr_create( pool: *mut pj_pool_t, value: ::std::os::raw::c_uint, ) -> *mut pjsip_min_expires_hdr;
    pub fn pjsip_min_expires_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, value: ::std::os::raw::c_uint, ) -> *mut pjsip_min_expires_hdr;
    pub fn pjsip_rr_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_rr_hdr;
    pub fn pjsip_rr_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_rr_hdr;
    pub fn pjsip_route_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_route_hdr;
    pub fn pjsip_route_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_route_hdr;
    pub fn pjsip_routing_hdr_set_rr(r: *mut pjsip_routing_hdr) -> *mut pjsip_rr_hdr;
    pub fn pjsip_routing_hdr_set_route(r: *mut pjsip_routing_hdr) -> *mut pjsip_route_hdr;
    pub fn pjsip_require_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_require_hdr;
    pub fn pjsip_require_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_require_hdr;
    pub fn pjsip_retry_after_hdr_create( pool: *mut pj_pool_t, value: ::std::os::raw::c_int, ) -> *mut pjsip_retry_after_hdr;
    pub fn pjsip_retry_after_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, value: ::std::os::raw::c_int, ) -> *mut pjsip_retry_after_hdr;
    pub fn pjsip_supported_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_supported_hdr;
    pub fn pjsip_supported_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_supported_hdr;
    pub fn pjsip_unsupported_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_unsupported_hdr;
    pub fn pjsip_unsupported_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_unsupported_hdr;
    pub fn pjsip_via_hdr_create(pool: *mut pj_pool_t) -> *mut pjsip_via_hdr;
    pub fn pjsip_via_hdr_init( pool: *mut pj_pool_t, mem: *mut ::std::os::raw::c_void, ) -> *mut pjsip_via_hdr;
    pub fn pjsip_warning_hdr_create( pool: *mut pj_pool_t, code: ::std::os::raw::c_int, host: *const pj_str_t, text: *const pj_str_t, ) -> *mut pjsip_warning_hdr;
    pub fn pjsip_warning_hdr_create_from_status( pool: *mut pj_pool_t, host: *const pj_str_t, status: pj_status_t, ) -> *mut pjsip_warning_hdr;
    pub fn pjsip_multipart_create( pool: *mut pj_pool_t, ctype: *const pjsip_media_type, boundary: *const pj_str_t, ) -> *mut pjsip_msg_body;
    pub fn pjsip_multipart_create_part(pool: *mut pj_pool_t) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_clone_part( pool: *mut pj_pool_t, part: *const pjsip_multipart_part, ) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_add_part( pool: *mut pj_pool_t, mp: *mut pjsip_msg_body, part: *mut pjsip_multipart_part, ) -> pj_status_t;
    pub fn pjsip_multipart_get_first_part(mp: *const pjsip_msg_body) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_get_next_part( mp: *const pjsip_msg_body, part: *mut pjsip_multipart_part, ) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_find_part( mp: *const pjsip_msg_body, content_type: *const pjsip_media_type, start: *const pjsip_multipart_part, ) -> *mut pjsip_multipart_part;
    pub fn pjsip_multipart_parse( pool: *mut pj_pool_t, buf: *mut ::std::os::raw::c_char, len: pj_size_t, ctype: *const pjsip_media_type, options: ::std::os::raw::c_uint, ) -> *mut pjsip_msg_body;
    pub fn pjsip_multipart_get_raw( mp: *mut pjsip_msg_body, boundary: *mut pj_str_t, raw_data: *mut pj_str_t, ) -> pj_status_t;
    pub static mut PJSIP_SYN_ERR_EXCEPTION: ::std::os::raw::c_int;
    pub static mut PJSIP_EINVAL_ERR_EXCEPTION: ::std::os::raw::c_int;
    pub fn pjsip_register_hdr_parser( hname: *const ::std::os::raw::c_char, hshortname: *const ::std::os::raw::c_char, fptr: pjsip_parse_hdr_func, ) -> pj_status_t;
    pub fn pjsip_unregister_hdr_parser( hname: *const ::std::os::raw::c_char, hshortname: *const ::std::os::raw::c_char, fptr: pjsip_parse_hdr_func, ) -> pj_status_t;
    pub fn pjsip_register_uri_parser( scheme: *mut ::std::os::raw::c_char, func: pjsip_parse_uri_func, ) -> pj_status_t;
    pub fn pjsip_unregister_uri_parser( scheme: *const ::std::os::raw::c_char, func: pjsip_parse_uri_func, ) -> pj_status_t;
    pub fn pjsip_parse_uri( pool: *mut pj_pool_t, buf: *mut ::std::os::raw::c_char, size: pj_size_t, options: ::std::os::raw::c_uint, ) -> *mut pjsip_uri;
    pub fn pjsip_parse_status_line( buf: *mut ::std::os::raw::c_char, size: pj_size_t, status_line: *mut pjsip_status_line, ) -> pj_status_t;
    pub fn pjsip_parse_msg( pool: *mut pj_pool_t, buf: *mut ::std::os::raw::c_char, size: pj_size_t, err_list: *mut pjsip_parser_err_report, ) -> *mut pjsip_msg;
    pub fn pjsip_parse_rdata( buf: *mut ::std::os::raw::c_char, size: pj_size_t, rdata: *mut pjsip_rx_data, ) -> *mut pjsip_msg;
    pub fn pjsip_find_msg( buf: *const ::std::os::raw::c_char, size: pj_size_t, is_datagram: pj_bool_t, msg_size: *mut pj_size_t, ) -> pj_status_t;
    pub fn pjsip_parse_hdr( pool: *mut pj_pool_t, hname: *const pj_str_t, line: *mut ::std::os::raw::c_char, size: pj_size_t, parsed_len: *mut ::std::os::raw::c_int, ) -> *mut ::std::os::raw::c_void;
    pub fn pjsip_parse_headers( pool: *mut pj_pool_t, input: *mut ::std::os::raw::c_char, size: pj_size_t, hlist: *mut pjsip_hdr, options: ::std::os::raw::c_uint,) -> pj_status_t;
    pub fn pjsip_parser_const() -> *const pjsip_parser_const_t;
    pub fn pjsip_parse_param_imp( scanner: *mut pj_scanner, pool: *mut pj_pool_t, pname: *mut pj_str_t, pvalue: *mut pj_str_t, opt: ::std::os::raw::c_uint, );
    pub fn pjsip_parse_uri_param_imp( scanner: *mut pj_scanner, pool: *mut pj_pool_t, pname: *mut pj_str_t, pvalue: *mut pj_str_t, opt: ::std::os::raw::c_uint, );
    pub fn pjsip_concat_param_imp( param: *mut pj_str_t, pool: *mut pj_pool_t, pname: *const pj_str_t, pvalue: *const pj_str_t, sepchar: ::std::os::raw::c_int, );
    pub fn pjsip_parse_end_hdr_imp(scanner: *mut pj_scanner);
    pub fn pjsip_parse_generic_array_hdr_imp( hdr: *mut pjsip_generic_array_hdr, scanner: *mut pj_scanner, );
}


















impl AutoCreate<pjsip_cred_info__bindgen_ty_1__bindgen_ty_1>
    for pjsip_cred_info__bindgen_ty_1__bindgen_ty_1
{
    fn new() -> pjsip_cred_info__bindgen_ty_1__bindgen_ty_1 {
        pjsip_cred_info__bindgen_ty_1__bindgen_ty_1 {
            k: pj_str_t::new(),
            op: pj_str_t::new(),
            amf: pj_str_t::new(),
            cb: None,
        }
    }
}

impl AutoCreate<pjsip_cred_info__bindgen_ty_1> for pjsip_cred_info__bindgen_ty_1 {
    fn new() -> pjsip_cred_info__bindgen_ty_1 {
        pjsip_cred_info__bindgen_ty_1 {
            aka: pjsip_cred_info__bindgen_ty_1__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsip_cred_info> for pjsip_cred_info {
    fn new() -> pjsip_cred_info {
        pjsip_cred_info {
            realm: pj_str_t::new(),
            scheme: pj_str_t::new(),
            username: pj_str_t::new(),
            data_type: 0,
            data: pj_str_t::new(),
            ext: pjsip_cred_info__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsip_hdr_vptr> for pjsip_hdr_vptr {
    fn new() -> pjsip_hdr_vptr {
        pjsip_hdr_vptr {
            clone: None,
            shallow_clone: None,
            print_on: None,
        }
    }
}

impl AutoCreate<pjsip_hdr> for pjsip_hdr {
    fn new() -> pjsip_hdr {
        pjsip_hdr {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            type_: 0,
            name: pj_str_t::new(),
            sname: pj_str_t::new(),
            vptr: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_tls_setting> for pjsip_tls_setting {
    fn new() -> pjsip_tls_setting {
        pjsip_tls_setting {
            ca_list_file: pj_str_t::new(),
            ca_list_path: pj_str_t::new(),
            cert_file: pj_str_t::new(),
            privkey_file: pj_str_t::new(),
            ca_buf: pj_str_t::new() as pj_ssl_cert_buffer,
            cert_buf: pj_str_t::new() as pj_ssl_cert_buffer,
            privkey_buf: pj_str_t::new() as pj_ssl_cert_buffer,
            password: pj_str_t::new(),
            method: 0,
            proto: 0,
            ciphers_num: 0,
            ciphers: ptr::null_mut(),
            curves_num: 0,
            curves: ptr::null_mut(),
            sigalgs: pj_str_t::new(),
            entropy_type: 0,
            entropy_path: pj_str_t::new(),
            verify_server: pj_constants__PJ_FALSE as pj_bool_t,
            verify_client: pj_constants__PJ_FALSE as pj_bool_t,
            require_client_cert: pj_constants__PJ_FALSE as pj_bool_t,
            timeout: pj_time_val::new(),
            reuse_addr: pj_constants__PJ_FALSE as pj_bool_t,
            qos_type: 0,
            qos_params: pj_qos_params::new(),
            qos_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
            sockopt_params: pj_sockopt_params::new(),
            sockopt_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
            on_accept_fail_cb: None,
        }
    }
}

pub trait PjsipModuleCallback {
    unsafe extern "C" fn start() -> pj_status_t {
        0
    }
    unsafe extern "C" fn stop() -> pj_status_t {
        0
    }
    unsafe extern "C" fn unload() -> pj_status_t {
        0
    }
    unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_bool_t {
        0
    }
    unsafe extern "C" fn on_rx_response(rdata: *mut pjsip_rx_data) -> pj_bool_t {
        0
    }
    unsafe extern "C" fn on_tx_request(tdata: *mut pjsip_tx_data) -> pj_status_t {
        0
    }
    unsafe extern "C" fn on_tx_response(tdata: *mut pjsip_tx_data) -> pj_status_t {
        0
    }
    unsafe extern "C" fn on_tsx_state(tsx: *mut pjsip_transaction, event: *mut pjsip_event) {}
}

impl AutoCreate<pjsip_module> for pjsip_module {
    fn new() -> pjsip_module {
        pjsip_module {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            name: pj_str_t::new(),
            id: 0,
            priority: 0,
            load: None,
            start: None,
            stop: None,
            unload: None,
            on_rx_request: None,
            on_rx_response: None,
            on_tx_request: None,
            on_tx_response: None,
            on_tsx_state: None,
        }
    }
}

impl AutoCreate<pjsip_tx_data_op_key> for pjsip_tx_data_op_key {
    fn new() -> pjsip_tx_data_op_key {
        pjsip_tx_data_op_key {
            key: pj_ioqueue_op_key_t::new(),
            tdata: ptr::null_mut(),
            token: ptr::null_mut(),
            callback: None,
        }
    }
}

impl AutoCreate<pjsip_buffer> for pjsip_buffer {
    fn new() -> pjsip_buffer {
        pjsip_buffer {
            start: ptr::null_mut(),
            cur: ptr::null_mut(),
            end: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_server_addresses__bindgen_ty_1> for pjsip_server_addresses__bindgen_ty_1 {
    fn new() -> pjsip_server_addresses__bindgen_ty_1 {
        pjsip_server_addresses__bindgen_ty_1 {
            type_: 0,
            priority: 0,
            weight: 0,
            addr: pj_sockaddr::new(),
            addr_len: 0,
        }
    }
}

impl AutoCreate<pjsip_server_addresses> for pjsip_server_addresses {
    fn new() -> pjsip_server_addresses {
        pjsip_server_addresses {
            count: 0,
            entry: [pjsip_server_addresses__bindgen_ty_1::new(); 16],
        }
    }
}

impl AutoCreate<pjsip_tx_data__bindgen_ty_1> for pjsip_tx_data__bindgen_ty_1 {
    fn new() -> pjsip_tx_data__bindgen_ty_1 {
        pjsip_tx_data__bindgen_ty_1 {
            name: pj_str_t::new(),
            addr: pjsip_server_addresses::new(),
            cur_addr: 0x00000000,
        }
    }
}

impl AutoCreate<pjsip_tx_data__bindgen_ty_2> for pjsip_tx_data__bindgen_ty_2 {
    fn new() -> pjsip_tx_data__bindgen_ty_2 {
        pjsip_tx_data__bindgen_ty_2 {
            transport: ptr::null_mut(),
            dst_addr: pj_sockaddr::new(),
            dst_addr_len: 0,
            dst_name: [0x0; 46],
            dst_port: 0,
        }
    }
}

impl AutoCreate<pjsip_tpselector__bindgen_ty_1> for pjsip_tpselector__bindgen_ty_1 {
    fn new() -> pjsip_tpselector__bindgen_ty_1 {
        pjsip_tpselector__bindgen_ty_1 {
            ptr: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_tpselector> for pjsip_tpselector {
    fn new() -> pjsip_tpselector {
        pjsip_tpselector {
            type_: 0,
            disable_connection_reuse: pj_constants__PJ_FALSE as pj_bool_t,
            u: pjsip_tpselector__bindgen_ty_1::new(),
        }
    }
}

impl AutoCreate<pjsip_host_port> for pjsip_host_port {
    fn new() -> pjsip_host_port {
        pjsip_host_port {
            host: pj_str_t::new(),
            port: 0,
        }
    }
}

impl AutoCreate<pjsip_tx_data> for pjsip_tx_data {
    fn new() -> pjsip_tx_data {
        pjsip_tx_data {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            pool: ptr::null_mut(),
            obj_name: [0x00; 32],
            info: ptr::null_mut(),
            rx_timestamp: pj_time_val::new(),
            mgr: ptr::null_mut(),
            op_key: pjsip_tx_data_op_key::new(),
            lock: ptr::null_mut(),
            msg: ptr::null_mut(),
            saved_strict_route: ptr::null_mut(),
            buf: pjsip_buffer::new(),
            ref_cnt: ptr::null_mut(),
            is_pending: 0,
            token: ptr::null_mut(),
            cb: None,
            dest_info: pjsip_tx_data__bindgen_ty_1::new(),
            tp_info: pjsip_tx_data__bindgen_ty_2::new(),
            tp_sel: pjsip_tpselector::new(),
            auth_retry: pj_constants__PJ_FALSE as pj_bool_t,
            mod_data: [ptr::null_mut(); 32],
            via_addr: pjsip_host_port::new(),
            via_tp: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_generic_string_hdr> for pjsip_generic_string_hdr {
    fn new() -> pjsip_generic_string_hdr {
        pjsip_generic_string_hdr {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            type_: 0,
            name: pj_str_t::new(),
            sname: pj_str_t::new(),
            vptr: ptr::null_mut(),
            hvalue: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjsip_pres_status__bindgen_ty_1> for pjsip_pres_status__bindgen_ty_1 {
    fn new() -> pjsip_pres_status__bindgen_ty_1 {
        pjsip_pres_status__bindgen_ty_1 {
            basic_open: pj_constants__PJ_FALSE as pj_bool_t,
            rpid: pjrpid_element::new(),
            id: pj_str_t::new(),
            contact: pj_str_t::new(),
            tuple_node: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_pres_status> for pjsip_pres_status {
    fn new() -> pjsip_pres_status {
        pjsip_pres_status {
            info_cnt: 0,
            info: [pjsip_pres_status__bindgen_ty_1::new(); 8],
            _is_valid: pj_constants__PJ_FALSE as pj_bool_t,
        }
    }
}

impl AutoCreate<pjsip_param> for pjsip_param {
    fn new() -> pjsip_param {
        pjsip_param {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            name: pj_str_t::new(),
            value: pj_str_t::new(),
        }
    }
}

impl AutoCreate<pjsip_multipart_part> for pjsip_multipart_part {
    fn new() -> pjsip_multipart_part {
        pjsip_multipart_part {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            hdr: pjsip_hdr::new(),
            body: ptr::null_mut(),
        }
    }
}

impl AutoCreate<pjsip_media_type> for pjsip_media_type {
    fn new() -> pjsip_media_type {
        pjsip_media_type {
            type_: pj_str_t::new(),
            subtype: pj_str_t::new(),
            param: pjsip_param::new(),
        }
    }
}
