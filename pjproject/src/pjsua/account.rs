use std::convert::TryFrom;
use pjnath_sys::{pj_ice_sess_options, pj_stun_auth_cred};

use crate::{pjmedia::{MediaRtcpFbType, MediaSrtpUse, RtcpFbCapability, RtcpFbSetting}, pjnath::TurnTpType, pjsip::SIPStatusCode, utils::{boolean_to_pjbool, check_boolean}};
use super::*;

pub trait UAIceConfigExt {

    /// Enable ICE.
    fn set_enable_ice(&mut self, value: bool);
    fn get_enable_ice(&self) -> bool;

    /// Set the maximum number of host candidates.
    ///
    /// # default:
    /// -1 (maximum not set)
    ///
    fn set_ice_max_host_cands(&mut self, value: i32);
    fn get_ice_max_host_cands(&self) -> i32;

    /// ICE session options.
    fn set_ice_opt(&mut self, value: pj_ice_sess_options);
    fn get_ice_opt(&self) -> &pj_ice_sess_options;

    ///
    /// Disable RTCP component.
    ///
    /// # default
    /// no
    fn set_ice_no_rtcp(&mut self, value: bool);
    fn get_ice_no_rtcp(&self) -> bool;

    /// Send re-INVITE/UPDATE every after ICE connectivity check regardless the default ICE transport
    /// address is changed or not. When this is set to PJ_FALSE, re-INVITE/UPDATE will be sent only
    /// when the default ICE transport address is changed.
    ///
    /// # default:
    /// true
    ///
    fn set_ice_always_update(&mut self, value: bool);
    fn get_ice_always_update(&self) -> bool;
}

pub trait UATurnConfigExt {

    /// Enable TURN candidate in ICE.
    fn set_enable_turn(&mut self, value: bool);
    fn get_enable_turn(&self) -> bool;

    /// Specify TURN domain name or host name, in in "DOMAIN:PORT" or "HOST:PORT" format.
    fn set_turn_server(&mut self, value: String);
    fn get_turn_server(&self) -> String;

    ///
    /// Specify the connection type to be used to the TURN server. Valid values are
    /// PJ_TURN_TP_UDP, PJ_TURN_TP_TCP or PJ_TURN_TP_TLS.
    ///
    /// # default
    /// PJ_TURN_TP_UDP
    ///
    fn set_turn_conn_type(&mut self, value: TurnTpType);
    fn get_turn_conn_type(&self) -> TurnTpType;

    /// Specify the credential to authenticate with the TURN server.
    fn set_turn_auth_cred(&mut self, value: pj_stun_auth_cred);
    fn get_turn_auth_cred(&self) -> &pj_stun_auth_cred;

    /// This specifies TLS settings for TURN TLS. It is only be used when this TLS is
    /// used to connect to the TURN server.
    fn set_turn_tls_setting(&mut self, value: pj_turn_sock_tls_cfg);
    fn get_turn_tls_setting(&self) -> &pj_turn_sock_tls_cfg;
}

pub trait UAAccConfigExt {
    // no implementation for user_data fields
    // user_data: *mut c_void,

    /// Account priority, which is used to control the order of matching incoming/outgoing requests.
    /// The higher the number means the higher the priority is, and the account will be matched first.
    fn set_priority(&mut self, value: i32);
    fn get_priority(&self) -> i32;

    /// The full SIP URL for the account. The value can take name address or URL format,
    /// and will look something like "sip:account@serviceprovider" or "/"
    /// Display Name" <sip:account@provider>".
    ///
    /// This field is mandatory.
    fn set_id(&mut self, value: String);
    fn get_id(&self) -> String;

    /// This is the URL to be put in the request URI for the registration,
    /// and will look something like "sip:serviceprovider".
    ///
    /// This field should be specified if registration is desired.
    /// If the value is empty, no account registration will be performed.
    fn set_reg_uri(&mut self, value: String);
    fn get_reg_uri(&self) -> String;

    /// The optional custom SIP headers to be put in the registration request.
    // fn set_reg_hdr_list(&self, value: pjsip_hdr);
    // fn get_reg_hdr_list(&self) -> pjsip_hdr;
    /// Additional parameters that will be appended in the Contact header for this account.
    /// This will only affect REGISTER requests and will be appended after contact_params;
    ///
    /// The parameters should be preceeded by semicolon, and all strings must be properly escaped.
    /// Example: ";my-param=X;another-param=Hi%20there"
    fn set_reg_contact_params(&mut self, value: String);
    fn get_reg_contact_params(&self) -> String;

    // The optional custom SIP headers to be put in the presence subscription request.
    // fn set_sub_hdr_list(&self, value: pjsip_hdr);
    // fn get_sub_hdr_list(&self) -> pjsip_hdr;
    /// Subscribe to message waiting indication events (RFC 3842).
    ///
    /// See also enable_unsolicited_mwi field on pjsua_config.
    ///
    /// # Default
    /// no
    fn set_mwi_enabled(&mut self, value: bool);
    fn get_mwi_enabled(&self) -> bool;

    /// Specify the default expiration time for Message Waiting Indication (RFC 3842)
    /// event subscription. This must not be zero.
    ///
    /// # Default
    /// PJSIP_MWI_DEFAULT_EXPIRES
    fn set_mwi_expires(&mut self, value: u32);
    fn get_mwi_expires(&self) -> u32;

    /// If this flag is set, the presence information of this account will be
    /// PUBLISH-ed to the server where the account belongs.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_publish_enabled(&mut self, value: bool);
    fn get_publish_enabled(&self) -> bool;

    /// Event publication options.
    fn set_publish_opt(&mut self, value: bool);
    fn get_publish_opt(&self) -> bool;

    /// Maximum time to wait for unpublication transaction(s) to complete during shutdown process,
    /// before sending unregistration. The library tries to wait for the unpublication
    /// (un-PUBLISH) to complete before sending REGISTER request to unregister the account,
    /// during library shutdown process. If the value is set too short, it is possible that the
    /// unregistration is sent before unpublication completes, causing unpublication request to fail.
    ///
    /// # Default
    /// PJSUA_UNPUBLISH_MAX_WAIT_TIME_MSEC
    fn set_unpublish_max_wait_time_msec(&mut self, value: u32);
    fn get_unpublish_max_wait_time_msec(&self) -> u32;

    /// Authentication preference.
    fn set_auth_pref(&mut self, initial_auth: bool, algorithm: String);
    fn get_auth_pref(&self) -> (bool, String);

    /// Optional PIDF tuple ID for outgoing PUBLISH and NOTIFY. If this value is not specified,
    /// a random string will be used.
    fn set_pidf_tuple_id(&mut self, value: String);
    fn get_pidf_tuple_id(&self) -> String;

    /// Optional URI to be put as Contact for this account. It is recommended that this field
    /// is left empty, so that the value will be calculated automatically based on the transport
    /// address.
    fn set_force_contact(&mut self, value: String);
    fn get_force_contact(&self) -> String;

    /// Additional parameters that will be appended in the Contact header for this account.
    /// This will affect the Contact header in all SIP messages sent on behalf of this account,
    /// including but not limited to REGISTER, INVITE, and SUBCRIBE requests or responses.
    ///
    /// The parameters should be preceeded by semicolon, and all strings must be properly escaped.
    /// Example: ";my-param=X;another-param=Hi%20there"
    fn set_contact_params(&mut self, value: String);
    fn get_contact_params(&self) -> String;

    /// Additional URI parameters that will be appended in the Contact URI for this account.
    /// This will affect the Contact URI in all SIP messages sent on behalf of this account,
    /// including but not limited to REGISTER, INVITE, and SUBCRIBE requests or responses.
    ///
    /// The parameters should be preceeded by semicolon, and all strings must be properly escaped.
    /// Example: ";my-param=X;another-param=Hi%20there"
    fn set_contact_uri_params(&mut self, value: String);
    fn get_contact_uri_params(&self) -> String;

    /// Specify how support for reliable provisional response (100rel/ PRACK) should be used for
    /// all sessions in this account. See the documentation of pjsua_100rel_use enumeration for more info.
    ///
    /// # Default
    /// The default value is taken from the value of require_100rel in pjsua_config.
    fn set_require_100rel(&mut self, value: UAConfig100relUse);
    fn get_require_100rel(&self) -> UAConfig100relUse;

    /// Specify the usage of Session Timers for all sessions. See the pjsua_sip_timer_use for possible values.
    ///
    /// # Default
    /// PJSUA_SIP_TIMER_OPTIONAL
    fn set_use_timer(&mut self, value: UAConfigSipTimerUse);
    fn get_use_timer(&self) -> UAConfigSipTimerUse;

    /// Specify Session Timer settings, see pjsip_timer_setting.
    fn set_timer_setting(&mut self, min_se: u32, sess_expires: u32);
    fn get_timer_setting(&self) -> (u32, u32);

    /// Number of proxies in the proxy array below.
    fn set_proxy_cnt(&mut self, value: u32);
    fn get_proxy_cnt(&self) -> u32;

    /// Optional URI of the proxies to be visited for all outgoing requests that are using this account
    /// (REGISTER, INVITE, etc). Application need to specify these proxies if the service provider
    /// requires that requests destined towards its network should go through certain proxies first
    /// (for example, border controllers).
    ///
    /// These proxies will be put in the route set for this account, with maintaining the orders
    /// (the first proxy in the array will be visited first). If global outbound proxies are configured
    /// in pjsua_config, then these account proxies will be placed after the global outbound
    /// proxies in the routeset.
    fn set_proxy(&mut self, proxy1: Option<String>, proxy2: Option<String>, proxy3: Option<String>, proxy4: Option<String>);
    fn get_proxy(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>);

    /// If remote sends SDP answer containing more than one format or codec in the media line,
    /// send re-INVITE or UPDATE with just one codec to lock which codec to use.
    ///
    /// # Default
    /// 1 (Yes). Set to zero to disable.
    fn set_lock_codec(&mut self, value: u32);
    fn get_lock_codec(&self) -> u32;

    /// Optional interval for registration, in seconds. If the value is zero,
    /// default interval will be used (PJSUA_REG_INTERVAL, 300 seconds).
    fn set_reg_timeout(&mut self, value: u32);
    fn get_reg_timeout(&self) -> u32;

    /// Specify the number of seconds to refresh the client registration before the
    /// registration expires.
    ///
    /// # Default
    /// PJSIP_REGISTER_CLIENT_DELAY_BEFORE_REFRESH, 5 seconds
    fn set_reg_delay_before_refresh(&mut self, value: u32);
    fn get_reg_delay_before_refresh(&self) -> u32;

    /// Specify the maximum time to wait for unregistration requests to complete
    /// during library shutdown sequence.
    ///
    /// # Default
    /// PJSUA_UNREG_TIMEOUT
    fn set_unreg_timeout(&mut self, value: u32);
    fn get_unreg_timeout(&self) -> u32;

    /// Number of credentials in the credential array.
    fn set_cred_count(&mut self, value: u32);
    fn get_cred_count(&self) -> u32;

    /// Array of credentials. If registration is desired, normally there should be at least
    /// one credential specified, to successfully authenticate against the service provider.
    /// More credentials can be specified, for example when the requests are expected to be
    /// challenged by the proxies in the route set.
    fn set_cred_info(&mut self, value: [CredentialInfo; 8usize]);
    fn get_cred_info(&self) -> [CredentialInfo; 8usize];

    /// Optionally bind this account to specific transport. This normally is not a good idea,
    /// as account should be able to send requests using any available transports according
    /// to the destination. But some application may want to have explicit control over the
    /// transport to use, so in that case it can set this field.
    ///
    /// # Default
    /// -1 (PJSUA_INVALID_ID)
    ///
    /// # See also
    ///     pjsua_acc_set_transport()
    fn set_transport_id(&mut self, value: i32);
    fn get_transport_id(&self) -> i32;

    /// This option is used to update the transport address and the Contact header of REGISTER
    /// request. When this option is enabled, the library will keep track of the public IP address
    /// from the response of REGISTER request. Once it detects that the address has changed, it
    /// will unregister current Contact, update the Contact with transport address learned from Via
    /// header, and register a new Contact to the registrar. This will also update the public name
    /// of UDP transport if STUN is configured.
    ///
    /// See also contact_rewrite_method field.
    ///
    /// # Default
    /// 1 (yes)
    fn set_allow_contact_rewrite(&mut self, value: bool);
    fn get_allow_contact_rewrite(&self) -> bool;

    /// Specify how Contact update will be done with the registration, if allow_contact_rewrite is enabled.
    /// The value is bitmask combination of pjsua_contact_rewrite_method. See also pjsua_contact_rewrite_method.
    ///
    /// Value PJSUA_CONTACT_REWRITE_UNREGISTER(1) is the legacy behavior.
    ///
    /// # Default
    /// value: PJSUA_CONTACT_REWRITE_METHOD (PJSUA_CONTACT_REWRITE_NO_UNREG | PJSUA_CONTACT_REWRITE_ALWAYS_UPDATE)
    fn set_contact_rewrite_method(&mut self, value: i32);
    fn get_contact_rewrite_method(&self) -> i32;

    /// Specify if source TCP port should be used as the initial Contact address if TCP/TLS transport is used.
    /// Note that this feature will be automatically turned off when nameserver is configured because it
    /// may yield different destination address due to DNS SRV resolution. Also some platforms are unable to
    /// report the local address of the TCP socket when it is still connecting. In these cases, this feature
    /// will also be turned off.
    ///
    /// # Default
    /// PJ_TRUE (yes).
    fn set_contact_use_src_port(&mut self, value: bool);
    fn get_contact_use_src_port(&self) -> bool;

    /// This option is used to overwrite the "sent-by" field of the Via header for outgoing messages with
    /// the same interface address as the one in the REGISTER request, as long as the request uses the same
    /// transport instance as the previous REGISTER request.
    ///
    /// # Default
    /// 1 (yes)
    fn set_allow_via_rewrite(&mut self, value: bool);
    fn get_allow_via_rewrite(&self) -> bool;

    /// This option controls whether the IP address in SDP should be replaced with the IP address found
    /// in Via header of the REGISTER response, ONLY when STUN and ICE are not used. If the value is FALSE
    /// (the original behavior), then the local IP address will be used. If TRUE, and when STUN and ICE
    /// are disabled, then the IP address found in registration response will be used.
    ///
    /// # Default
    /// PJ_FALSE (no)
    fn set_allow_sdp_nat_rewrite(&mut self, value: bool);
    fn get_allow_sdp_nat_rewrite(&self) -> bool;

    /// Control the use of SIP outbound feature. SIP outbound is described in RFC 5626 to enable
    /// proxies or registrar to send inbound requests back to UA using the same connection initiated
    /// by the UA for its registration. This feature is highly useful in NAT-ed deployemtns,
    /// hence it is enabled by default.
    ///
    /// Note: currently SIP outbound can only be used with TCP and TLS transports. If UDP is
    /// used for the registration, the SIP outbound feature will be silently ignored for the account.
    ///
    /// # Default
    /// PJ_TRUE
    fn set_use_rfc5626(&mut self, value: u32);
    fn get_use_rfc5626(&self) -> u32;

    /// Specify SIP outbound (RFC 5626) instance ID to be used by this application. If empty,
    /// an instance ID will be generated based on the hostname of this agent. If application
    /// specifies this parameter, the value will
    /// look like "<urn:uuid:00000000-0000-1000-8000-AABBCCDDEEFF>" without the doublequote.
    ///
    /// # Default
    /// empty
    fn set_rfc5626_instance_id(&mut self, value: String);
    fn get_rfc5626_instance_id(&self) -> String;
    // Specify SIP outbound (RFC 5626) registration ID. The default value is empty,
    // which would cause the library to automatically generate a suitable value.
    //
    // # Default
    // empty
    fn set_rfc5626_reg_id(&mut self, value: String);
    fn get_rfc5626_reg_id(&self) -> String;

    /// Set the interval for periodic keep-alive transmission for this account. If this value is zero,
    /// keep-alive will be disabled for this account. The keep-alive transmission will be sent to the
    /// registrar's address, after successful registration.
    ///
    /// # Default
    /// 15 (seconds)
    fn set_ka_interval(&mut self, value: u32);
    fn get_ka_interval(&self) -> u32;

    /// Specify the data to be transmitted as keep-alive packets.
    ///
    /// # Default
    /// CR-LF
    fn set_ka_data(&mut self, value: String);
    fn get_ka_data(&self) -> String;

    // skiped AudioIpCodecs not support video
    // vid_in_auto_show: pj_bool_t,
    // vid_out_auto_transmit: pj_bool_t,
    // vid_wnd_flags: c_uint,
    // vid_cap_dev: pjmedia_vid_dev_index,
    // vid_rend_dev: pjmedia_vid_dev_index,
    // vid_stream_rc_cfg: pjmedia_vid_stream_rc_config,
    // vid_stream_sk_cfg: pjmedia_vid_stream_sk_config,

    /// Media transport config.
    fn set_rtp_cfg(&mut self, value: UATransportConfig);
    fn get_rtp_cfg(&self) -> &UATransportConfig;

    /// Specify NAT64 options.
    ///
    /// # Default
    /// PJSUA_NAT64_DISABLED
    fn set_nat64_opt(&mut self, value: bool);
    fn get_nat64_opt(&self) -> bool;

    /// Specify whether IPv6 should be used on media.
    fn set_ipv6_media_use(&mut self, value: bool);
    fn get_ipv6_media_use(&self) -> bool;

    /// Control the use of STUN for the SIP signaling.
    ///
    /// # Default
    /// PJSUA_STUN_USE_DEFAULT
    fn set_sip_stun_use(&mut self, value: AccountConfigStunUse);
    fn get_sip_stun_use(&self) -> AccountConfigStunUse;

    /// Control the use of STUN for the media transports.
    ///
    /// # Default
    /// PJSUA_STUN_RETRY_ON_FAILURE
    fn set_media_stun_use(&mut self, value: AccountConfigStunUse);
    fn get_media_stun_use(&self) -> AccountConfigStunUse;

    /// Use loopback media transport. This may be useful if application doesn't want PJSIP
    /// to create real media transports/sockets, such as when using third party media.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_use_loop_med_tp(&mut self, value: bool);
    fn get_use_loop_med_tp(&self) -> bool;

    /// Enable local loopback when loop_med_tp_use is set to PJ_TRUE. If enabled, packets sent
    /// to the transport will be sent back to the streams attached to the transport.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_enable_loopback(&mut self, value: bool);
    fn get_enable_loopback(&self) -> bool;

    /// Control the use of ICE in the account. By default, the settings in the
    /// pjsua_media_config will be used.
    ///
    /// # Default
    /// PJSUA_ICE_CONFIG_USE_DEFAULT
    fn set_ice_cfg_use(&mut self, value: AccountConfigIceUse);
    fn get_ice_cfg_use(&self) -> AccountConfigIceUse;

    /// The custom ICE setting for this account. This setting will only be used if ice_cfg_use
    /// is set to PJSUA_ICE_CONFIG_USE_CUSTOM
    fn set_ice_cfg(&mut self, value: UAIceConfig);
    fn get_ice_cfg(&self) -> &UAIceConfig;

    /// Control the use of TURN in the account. By default, the settings in the
    /// pjsua_media_config will be used
    ///
    /// # Default
    /// PJSUA_TURN_CONFIG_USE_DEFAULT
    fn set_turn_cfg_use(&mut self, value: AccountConfigStunUse);
    fn get_turn_cfg_use(&self) -> AccountConfigStunUse;

    /// The custom TURN setting for this account. This setting will only be used if turn_cfg_use
    /// is set to PJSUA_TURN_CONFIG_USE_CUSTOM
    fn set_turn_cfg(&mut self, value: UATurnConfig);
    fn get_turn_cfg(&self) -> &UATurnConfig;

    /// Specify whether secure media transport should be used for this account. Valid values are
    /// PJMEDIA_SRTP_DISABLED, PJMEDIA_SRTP_OPTIONAL, and PJMEDIA_SRTP_MANDATORY.
    ///
    /// # Default
    /// PJSUA_DEFAULT_USE_SRTP
    fn set_use_srtp(&mut self, value: MediaSrtpUse);
    fn get_use_srtp(&self) -> MediaSrtpUse;

    /// Specify whether SRTP requires secure signaling to be used. This option is only used when
    /// use_srtp option above is non-zero.
    ///
    /// # Valid values are:
    /// - 0: SRTP does not require secure signaling
    /// - 1: SRTP requires secure transport such as TLS
    /// - 2: SRTP requires secure end-to-end transport (SIPS)
    ///
    /// # Default
    /// PJSUA_DEFAULT_SRTP_SECURE_SIGNALING
    fn set_srtp_secure_signaling(&mut self, value: i32);
    fn get_srtp_secure_signaling(&self) -> i32;

    /// This setting has been deprecated and will be ignored.
    fn set_srtp_optional_dup_offer(&mut self, value: bool);
    fn get_srtp_optional_dup_offer(&self) -> bool;

    /// Specify SRTP transport setting. Application can initialize it with default values using
    /// pjsua_srtp_opt_default().
    fn set_srtp_opt(&mut self, value: UASrtpOpt);
    fn get_srtp_opt(&self) -> &UASrtpOpt;

    /// Specify interval of auto registration retry upon registration failure, in seconds.
    /// Set to 0 to disable auto re-registration. Note that registration will only be automatically
    /// retried for temporal failures considered to be recoverable in relatively short term,
    /// # such as:
    /// - 408 (Request Timeout),
    /// - 480 (Temporarily Unavailable),
    /// - 500 (Internal Server Error),
    /// - 502 (Bad Gateway),
    /// - 503 (Service Unavailable),
    /// - 504 (Server Timeout),
    /// - 6xx (global failure),
    ///
    /// and failure caused by transport problem. For registration retry caused by transport failure,
    /// the first retry will be done after reg_first_retry_interval seconds instead. Note that the
    /// interval will be randomized slightly by some seconds (specified in reg_retry_random_interval)
    /// to avoid all clients re-registering at the same time.
    ///
    /// # See also
    /// reg_first_retry_interval setting.
    ///
    /// # Default
    /// PJSUA_REG_RETRY_INTERVAL
    fn set_reg_retry_interval(&mut self, value: u32);
    fn get_reg_retry_interval(&self) -> u32;

    /// This specifies the interval for the first registration retry. The registration retry is explained
    /// in reg_retry_interval. Note that the value here will also be randomized by some seconds
    /// (specified in reg_retry_random_interval) to avoid all clients re-registering at the same time.
    ///
    /// # Default
    /// 0
    fn set_reg_first_retry_interval(&mut self, value: u32);
    fn get_reg_first_retry_interval(&self) -> u32;

    /// This specifies maximum randomized value to be added/substracted to/from the registration
    /// retry interval specified in reg_retry_interval and reg_first_retry_interval, in second.
    /// This is useful to avoid all clients re-registering at the same time. For example, if the
    /// registration retry interval is set to 100 seconds and this is set to 10 seconds, the actual
    /// registration retry interval will be in the range of 90 to 110 seconds.
    ///
    /// # Default
    /// 10
    fn set_reg_retry_random_interval(&mut self, value: u32);
    fn get_reg_retry_random_interval(&self) -> u32;

    /// Specify whether calls of the configured account should be dropped after registration failure and
    /// an attempt of re-registration has also failed.
    ///
    /// # Default
    /// PJ_FALSE (disabled)
    fn set_drop_calls_on_reg_fail(&mut self, value: bool);
    fn get_drop_calls_on_reg_fail(&self) -> bool;

    /// Specify how the registration uses the outbound and account proxy settings. This controls if
    /// and what Route headers will appear in the REGISTER request of this account. The value is bitmask
    /// combination of PJSUA_REG_USE_OUTBOUND_PROXY and PJSUA_REG_USE_ACC_PROXY bits. If the value is set to 0,
    /// the REGISTER request will not use any proxy (i.e. it will not have any Route headers).
    ///
    /// # Default
    /// 3 (PJSUA_REG_USE_OUTBOUND_PROXY | PJSUA_REG_USE_ACC_PROXY)
    fn set_reg_use_proxy(&mut self, value: u32);
    fn get_reg_use_proxy(&mut self) -> u32;

    /// Specify how to offer call hold to remote peer. Please see the documentation on
    /// pjsua_call_hold_type for more info.
    ///
    /// # Default
    /// PJSUA_CALL_HOLD_TYPE_DEFAULT
    fn set_call_hold_type(&mut self, value: CallHoldType);
    fn get_call_hold_type(&self) -> CallHoldType;

    /// Specify whether the account should register as soon as it is added to the UA. Application can
    /// set this to PJ_FALSE and control the registration manually with pjsua_acc_set_registration().
    ///
    /// # Default
    /// PJ_TRUE
    fn set_register_on_acc_add(&mut self, value: bool);
    fn get_register_on_acc_add(&self) -> bool;

    /// Specify account configuration specific to IP address change used when calling
    /// pjsua_handle_ip_change().
    fn set_ip_change_cfg(&mut self, shutdown_tp: Option<bool>, hangup_calls: Option<bool>, reinvite_flags: Option<CallFlags>);
    fn get_ip_change_cfg(&self) -> (bool, bool, CallFlags);

    /// Enable RTP and RTCP multiplexing.
    fn set_enable_rtcp_mux(&mut self, value: bool);
    fn get_enable_rtcp_mux(&self) -> bool;

    /// RTCP Feedback configuration.
    fn set_rtcp_fb_cfg(&mut self, value: RtcpFbSetting);
    fn get_rtcp_fb_cfg(&self) -> &RtcpFbSetting;
}

pub trait UAAccInfoExt {
    fn get_id (&self) -> pjsua_acc_id;
    fn get_is_default (&self) -> bool;
    fn get_acc_uri (&self) -> String;
    fn get_has_registration (&self) -> bool;
    fn get_expires (&self) -> u32;
    fn get_status (&self) -> SIPStatusCode;
    fn get_reg_last_err (&self) -> i32;
    fn get_status_text (&self) -> String;
    fn get_online_status (&self) -> bool;
    fn get_online_status_text (&self) -> String;
    // fn get_rpid (&self) -> pjrpid_element;
    // fn get_buf_ (&self) [::std::os::raw::c_char; 80usize],
}


impl UAIceConfigExt for UAIceConfig {

    fn set_enable_ice(&mut self, value: bool) {
        self.enable_ice = boolean_to_pjbool(value)
    }

    fn get_enable_ice(&self) -> bool {
        check_boolean(self.enable_ice)
    }

    fn set_ice_max_host_cands(&mut self, value: i32) {
        self.ice_max_host_cands = value;
    }

    fn get_ice_max_host_cands(&self) -> i32 {
        self.ice_max_host_cands
    }

    fn set_ice_opt(&mut self, value: pj_ice_sess_options) {
        self.ice_opt = value;
    }

    fn get_ice_opt(&self) -> &pj_ice_sess_options {
        &self.ice_opt
    }

    fn set_ice_no_rtcp(&mut self, value: bool) {
        self.ice_no_rtcp = boolean_to_pjbool(value);
    }

    fn get_ice_no_rtcp(&self) -> bool {
        check_boolean(self.ice_no_rtcp)
    }

    fn set_ice_always_update(&mut self, value: bool)  {
        self.ice_always_update = boolean_to_pjbool(value);
    }

    fn get_ice_always_update(&self) -> bool {
        check_boolean(self.ice_always_update)
    }

}


impl UATurnConfigExt for UATurnConfig {

    fn set_enable_turn(&mut self, value: bool) {
        self.enable_turn = boolean_to_pjbool(value);
    }

    fn get_enable_turn(&self) -> bool {
        check_boolean(self.enable_turn)
    }

    fn set_turn_server(&mut self, value: String) {
        self.turn_server = pj_str_t::from_string(value);
    }

    fn get_turn_server(&self) -> String {
        self.turn_server.to_string()
    }

    fn set_turn_conn_type(&mut self, value: TurnTpType) {
        self.turn_conn_type = value.into();
    }

    fn get_turn_conn_type(&self) -> TurnTpType {
        TurnTpType::try_from(self.turn_conn_type)
        .expect("Error UATurnConfig get turn_conn_type")
    }

    fn set_turn_auth_cred(&mut self, value: pj_stun_auth_cred) {
        self.turn_auth_cred = value;
    }

    fn get_turn_auth_cred(&self) -> &pj_stun_auth_cred {
        &self.turn_auth_cred
    }

    fn set_turn_tls_setting(&mut self, value: pj_turn_sock_tls_cfg) {
        self.turn_tls_setting = value;
    }

    fn get_turn_tls_setting(&self) -> &pj_turn_sock_tls_cfg {
        &self.turn_tls_setting
    }

}


impl UAAccConfigExt for UAAccConfig {

    fn set_priority(&mut self, value: i32) {
        self.priority = value;
    }

    fn get_priority(&self) -> i32 {
        self.priority
    }

    fn set_id(&mut self, value: String) {
        self.id = pj_str_t::from_string(value);
    }

    fn get_id(&self) -> String {
        self.id.to_string()
    }

    fn set_reg_uri(&mut self, value: String) {
        self.reg_uri = pj_str_t::from_string(value);
    }

    fn get_reg_uri(&self) -> String {
        self.reg_uri.to_string()
    }

    fn set_reg_contact_params(&mut self, value: String) {
        self.reg_contact_params = pj_str_t::from_string(value);
    }

    fn get_reg_contact_params(&self) -> String {
        self.reg_contact_params.to_string()
    }

    fn set_mwi_enabled(&mut self, value: bool) {
        self.mwi_enabled = boolean_to_pjbool(value);
    }

    fn get_mwi_enabled(&self) -> bool {
        check_boolean(self.mwi_enabled)
    }

    fn set_mwi_expires(&mut self, value: u32) {
        self.mwi_expires = value;
    }

    fn get_mwi_expires(&self) -> u32 {
        self.mwi_expires
    }

    fn set_publish_enabled(&mut self, value: bool) {
        self.publish_enabled = boolean_to_pjbool(value);
    }

    fn get_publish_enabled(&self) -> bool {
        check_boolean(self.publish_enabled)
    }

    fn set_publish_opt(&mut self, value: bool) {
        self.publish_opt.queue_request = boolean_to_pjbool(value);
    }

    fn get_publish_opt(&self) -> bool {
        check_boolean(self.publish_opt.queue_request)
    }

    fn set_unpublish_max_wait_time_msec(&mut self, value: u32) {
        self.unpublish_max_wait_time_msec = value;
    }

    fn get_unpublish_max_wait_time_msec(&self) -> u32 {
        self.unpublish_max_wait_time_msec
    }

    fn set_auth_pref(&mut self, initial_auth: bool, algorithm: String) {
        self.auth_pref.initial_auth = boolean_to_pjbool(initial_auth);
        self.auth_pref.algorithm = pj_str_t::from_string(algorithm);
    }

    fn get_auth_pref(&self) -> (bool, String) {
        (
            check_boolean(self.auth_pref.initial_auth),
            self.auth_pref.algorithm.to_string()
        )
    }

    fn set_pidf_tuple_id(&mut self, value: String) {
        self.pidf_tuple_id = pj_str_t::from_string(value);
    }

    fn get_pidf_tuple_id(&self) -> String {
        self.pidf_tuple_id.to_string()
    }

    fn set_force_contact(&mut self, value: String) {
        self.force_contact = pj_str_t::from_string(value);
    }

    fn get_force_contact(&self) -> String {
        self.force_contact.to_string()
    }

    fn set_contact_params(&mut self, value: String) {
        self.contact_params = pj_str_t::from_string(value);
    }

    fn get_contact_params(&self) -> String {
        self.contact_params.to_string()
    }

    fn set_contact_uri_params(&mut self, value: String) {
        self.contact_uri_params = pj_str_t::from_string(value);
    }

    fn get_contact_uri_params(&self) -> String {
        self.contact_uri_params.to_string()
    }

    fn set_require_100rel(&mut self, value: UAConfig100relUse) {
        self.require_100rel = value.into();
    }

    fn get_require_100rel(&self) -> UAConfig100relUse {
        UAConfig100relUse::try_from(self.require_100rel)
        .expect("Error AccountConfig get require_100rel")
    }

    fn set_use_timer(&mut self, value: UAConfigSipTimerUse) {
        self.use_timer = value.into();
    }

    fn get_use_timer(&self) -> UAConfigSipTimerUse {
        UAConfigSipTimerUse::try_from(self.use_timer)
        .expect("Error AccountConfig get use_timer")
    }

    fn set_timer_setting(&mut self, min_se: u32, sess_expires: u32) {
        self.timer_setting.min_se = min_se;
        self.timer_setting.sess_expires = sess_expires;
    }

    fn get_timer_setting(&self) -> (u32, u32) {
        (
            self.timer_setting.min_se,
            self.timer_setting.sess_expires
        )
    }

    fn set_proxy_cnt(&mut self, value: u32) {
        self.proxy_cnt = value;
    }

    fn get_proxy_cnt(&self) -> u32 {
        self.proxy_cnt
    }

    fn set_proxy(&mut self, proxy1: Option<String>, proxy2: Option<String>, proxy3: Option<String>, proxy4: Option<String>) {
        // reset proxy
        self.proxy_cnt = 0;

        if proxy1.is_some() {
            self.proxy[self.proxy_cnt as usize] = pj_str_t::from_string(proxy1.unwrap());
            self.proxy_cnt+= 1;
        }

        if proxy2.is_some() {
            self.proxy[self.proxy_cnt as usize] = pj_str_t::from_string(proxy2.unwrap());
            self.proxy_cnt+= 1;
        }

        if proxy3.is_some() {
            self.proxy[self.proxy_cnt as usize] = pj_str_t::from_string(proxy3.unwrap());
            self.proxy_cnt+= 1;
        }

        if proxy4.is_some() {
            self.proxy[self.proxy_cnt as usize] = pj_str_t::from_string(proxy4.unwrap());
            self.proxy_cnt+= 1;
        }
    }

    fn get_proxy(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
       let mut proxy = (None, None, None, None);

        if self.proxy_cnt > 0 {
            proxy.0 = Some(self.proxy[0].to_string());
        }

        if self.proxy_cnt > 1 {
            proxy.1 = Some(self.proxy[1].to_string());
        }

        if self.proxy_cnt > 2 {
            proxy.2 = Some(self.proxy[2].to_string());
        }

        if self.proxy_cnt > 3 {
            proxy.3 = Some(self.proxy[3].to_string());
        }

       proxy
    }

    fn set_lock_codec(&mut self, value: u32) {
        self.lock_codec = value;
    }

    fn get_lock_codec(&self) -> u32 {
        self.lock_codec
    }

    fn set_reg_timeout(&mut self, value: u32) {
        self.reg_timeout = value;
    }

    fn get_reg_timeout(&self) -> u32 {
        self.reg_timeout
    }

    fn set_reg_delay_before_refresh(&mut self, value: u32) {
        self.reg_delay_before_refresh = value;
    }

    fn get_reg_delay_before_refresh(&self) -> u32 {
        self.reg_delay_before_refresh
    }

    fn set_unreg_timeout(&mut self, value: u32) {
        self.unreg_timeout = value;
    }

    fn get_unreg_timeout(&self) -> u32 {
        self.unreg_timeout
    }

    fn set_cred_count(&mut self, value: u32) {
        self.cred_count = value;
    }

    fn get_cred_count(&self) -> u32 {
        self.cred_count
    }

    fn set_cred_info(&mut self, value: [CredentialInfo; 8usize]) {
        todo!()
    }

    fn get_cred_info(&self) -> [CredentialInfo; 8usize] {
        todo!()
    }

    fn set_transport_id(&mut self, value: i32) {
        self.transport_id = value;
    }

    fn get_transport_id(&self) -> i32 {
        self.transport_id
    }

    fn set_allow_contact_rewrite(&mut self, value: bool) {
        self.allow_contact_rewrite = boolean_to_pjbool(value);
    }

    fn get_allow_contact_rewrite(&self) -> bool {
        check_boolean(self.allow_contact_rewrite)
    }

    fn set_contact_rewrite_method(&mut self, value: i32) {
        self.contact_rewrite_method = value;
    }

    fn get_contact_rewrite_method(&self) -> i32 {
        self.contact_rewrite_method
    }

    fn set_contact_use_src_port(&mut self, value: bool) {
        self.contact_use_src_port = boolean_to_pjbool(value);
    }

    fn get_contact_use_src_port(&self) -> bool {
        check_boolean(self.contact_use_src_port)
    }

    fn set_allow_via_rewrite(&mut self, value: bool) {
        self.allow_via_rewrite = boolean_to_pjbool(value);
    }

    fn get_allow_via_rewrite(&self) -> bool {
        check_boolean(self.allow_via_rewrite)
    }

    fn set_allow_sdp_nat_rewrite(&mut self, value: bool) {
        self.allow_sdp_nat_rewrite = boolean_to_pjbool(value);
    }

    fn get_allow_sdp_nat_rewrite(&self) -> bool {
        check_boolean(self.allow_sdp_nat_rewrite)
    }

    fn set_use_rfc5626(&mut self, value: u32) {
        self.use_rfc5626 = value;
    }

    fn get_use_rfc5626(&self) -> u32 {
        self.use_rfc5626
    }

    fn set_rfc5626_instance_id(&mut self, value: String) {
        self.rfc5626_instance_id = pj_str_t::from_string(value);
    }

    fn get_rfc5626_instance_id(&self) -> String {
        self.rfc5626_instance_id.to_string()
    }

    fn set_rfc5626_reg_id(&mut self, value: String) {
        self.rfc5626_reg_id = pj_str_t::from_string(value);
    }

    fn get_rfc5626_reg_id(&self) -> String {
        self.rfc5626_reg_id.to_string()
    }

    fn set_ka_interval(&mut self, value: u32) {
        self.ka_interval = value;
    }

    fn get_ka_interval(&self) -> u32 {
        self.ka_interval
    }

    fn set_ka_data(&mut self, value: String) {
        self.ka_data = pj_str_t::from_string(value);
    }

    fn get_ka_data(&self) -> String {
        self.ka_data.to_string()
    }

    fn set_rtp_cfg(&mut self, value: UATransportConfig) {
        self.rtp_cfg = value;
    }

    fn get_rtp_cfg(&self) -> &UATransportConfig {
        &self.rtp_cfg
    }

    fn set_nat64_opt(&mut self, value: bool) {
        self.nat64_opt = boolean_to_pjbool(value) as u32;
    }

    fn get_nat64_opt(&self) -> bool {
        check_boolean(self.nat64_opt as i32)
    }

    fn set_ipv6_media_use(&mut self, value: bool) {
        self.ipv6_media_use = boolean_to_pjbool(value) as u32;
    }

    fn get_ipv6_media_use(&self) -> bool {
        check_boolean(self.ipv6_media_use as i32)
    }

    fn set_sip_stun_use(&mut self, value: AccountConfigStunUse) {
        self.sip_stun_use = value.into();
    }

    fn get_sip_stun_use(&self) -> AccountConfigStunUse {
        AccountConfigStunUse::try_from(self.sip_stun_use)
        .expect("Error AccountConfig get sip_stun_use")
    }

    fn set_media_stun_use(&mut self, value: AccountConfigStunUse) {
        self.media_stun_use = value.into();
    }

    fn get_media_stun_use(&self) -> AccountConfigStunUse {
        AccountConfigStunUse::try_from(self.media_stun_use)
        .expect("Error AccountConfig get media_stun_use")
    }

    fn set_use_loop_med_tp(&mut self, value: bool) {
        self.use_loop_med_tp = boolean_to_pjbool(value);
    }

    fn get_use_loop_med_tp(&self) -> bool {
        check_boolean(self.use_loop_med_tp)
    }

    fn set_enable_loopback(&mut self, value: bool) {
        self.enable_loopback = boolean_to_pjbool(value);
    }

    fn get_enable_loopback(&self) -> bool {
        check_boolean(self.enable_loopback)
    }

    fn set_ice_cfg_use(&mut self, value: AccountConfigIceUse) {
        self.ice_cfg_use = value.into();
    }

    fn get_ice_cfg_use(&self) -> AccountConfigIceUse {
        AccountConfigIceUse::try_from(self.ice_cfg_use)
        .expect("Error AccountConfig get ice_cfg_use")
    }

    fn set_ice_cfg(&mut self, value: UAIceConfig) {
        self.ice_cfg = value;
    }

    fn get_ice_cfg(&self) -> &UAIceConfig {
        &self.ice_cfg
    }

    fn set_turn_cfg_use(&mut self, value: AccountConfigStunUse) {
        self.turn_cfg_use = value.into();
    }

    fn get_turn_cfg_use(&self) -> AccountConfigStunUse {
        AccountConfigStunUse::try_from(self.turn_cfg_use)
        .expect("Error AccountConfig get turn_cfg_use")
    }

    fn set_turn_cfg(&mut self, value: UATurnConfig) {
        self.turn_cfg = value;
    }

    fn get_turn_cfg(&self) -> &UATurnConfig {
        &self.turn_cfg
    }

    fn set_use_srtp(&mut self, value: MediaSrtpUse) {
        self.use_srtp = value.into();
    }

    fn get_use_srtp(&self) -> MediaSrtpUse {
        MediaSrtpUse::try_from(self.use_srtp)
        .expect("Error AccountConfig get use_srtp")
    }

    fn set_srtp_secure_signaling(&mut self, value: i32) {
        self.srtp_secure_signaling = value;
    }

    fn get_srtp_secure_signaling(&self) -> i32 {
        self.srtp_secure_signaling
    }

    fn set_srtp_optional_dup_offer(&mut self, value: bool) {
        self.srtp_optional_dup_offer = boolean_to_pjbool(value);
    }

    fn get_srtp_optional_dup_offer(&self) -> bool {
        check_boolean(self.srtp_optional_dup_offer)
    }

    fn set_srtp_opt(&mut self, value: UASrtpOpt) {
        self.srtp_opt = value;
    }

    fn get_srtp_opt(&self) -> &UASrtpOpt {
        &self.srtp_opt
    }

    fn set_reg_retry_interval(&mut self, value: u32) {
        self.reg_retry_interval = value;
    }

    fn get_reg_retry_interval(&self) -> u32 {
        self.reg_retry_interval
    }

    fn set_reg_first_retry_interval(&mut self, value: u32) {
        self.reg_first_retry_interval = value;
    }

    fn get_reg_first_retry_interval(&self) -> u32 {
        self.reg_first_retry_interval
    }

    fn set_reg_retry_random_interval(&mut self, value: u32) {
        self.reg_retry_random_interval = value;
    }

    fn get_reg_retry_random_interval(&self) -> u32 {
        self.reg_retry_random_interval
    }

    fn set_drop_calls_on_reg_fail(&mut self, value: bool) {
        self.drop_calls_on_reg_fail = boolean_to_pjbool(value);
    }

    fn get_drop_calls_on_reg_fail(&self) -> bool {
        check_boolean(self.drop_calls_on_reg_fail)
    }

    fn set_reg_use_proxy(&mut self, value: u32) {
        self.reg_use_proxy = value;
    }

    fn get_reg_use_proxy(&mut self) -> u32 {
        self.reg_use_proxy
    }

    fn set_call_hold_type(&mut self, value: CallHoldType) {
        self.call_hold_type = value.into();
    }

    fn get_call_hold_type(&self) -> CallHoldType {
        CallHoldType::try_from(self.call_hold_type)
        .expect("Error AccountConfig get call_hold_type")
    }

    fn set_register_on_acc_add(&mut self, value: bool) {
        self.register_on_acc_add = boolean_to_pjbool(value);
    }

    fn get_register_on_acc_add(&self) -> bool {
        check_boolean(self.register_on_acc_add)
    }

    fn set_ip_change_cfg(&mut self,
        shutdown_tp: Option<bool>,
        hangup_calls: Option<bool>,
        reinvite_flags: Option<CallFlags>
    ) {

        if shutdown_tp.is_some() {
            self.ip_change_cfg.shutdown_tp = boolean_to_pjbool(shutdown_tp.unwrap());
        }

        if hangup_calls.is_some() {
            self.ip_change_cfg.hangup_calls = boolean_to_pjbool(hangup_calls.unwrap());
        }

        if reinvite_flags.is_some() {
            self.ip_change_cfg.reinvite_flags = hangup_calls.unwrap().into();
        }
    }

    fn get_ip_change_cfg(&self) -> (bool, bool, CallFlags) {
        (
            check_boolean(self.ip_change_cfg.shutdown_tp),
            check_boolean(self.ip_change_cfg.hangup_calls),
            CallFlags::try_from(self.ip_change_cfg.reinvite_flags)
            .expect("Error AccountConfig get ip_change_cfg.reinvite_flags")
        )
    }

    fn set_enable_rtcp_mux(&mut self, value: bool) {
        self.enable_rtcp_mux = boolean_to_pjbool(value);
    }

    fn get_enable_rtcp_mux(&self) -> bool {
        check_boolean(self.enable_rtcp_mux)
    }

    // this settings for video support only
    // an audio implementation don't need
    // to touch this.
    fn set_rtcp_fb_cfg(&mut self, value: RtcpFbSetting) {
        self.rtcp_fb_cfg = value;
    }

    fn get_rtcp_fb_cfg(&self) -> &RtcpFbSetting {
        &self.rtcp_fb_cfg
    }
}


impl UAAccInfoExt for UAAccInfo {

    fn get_id (&self) -> pjsua_acc_id {
        self.id
    }

    fn get_is_default (&self) -> bool {
        check_boolean(self.is_default)
    }

    fn get_acc_uri (&self) -> String {
        self.acc_uri.to_string()
    }

    fn get_has_registration (&self) -> bool {
        check_boolean(self.has_registration)
    }

    fn get_expires (&self) -> u32 {
        self.expires
    }

    fn get_status (&self) -> SIPStatusCode {
        SIPStatusCode::try_from(self.status)
        .expect("Error AccountInfo get status")
    }

    fn get_reg_last_err (&self) -> i32 {
        self.reg_last_err
    }

    fn get_status_text (&self) -> String {
        self.status_text.to_string()
    }

    fn get_online_status (&self) -> bool {
        check_boolean(self.online_status)
    }

    fn get_online_status_text (&self) -> String {
        self.online_status_text.to_string()
    }
}


// pjsua account helper

pub fn ice_config_from_media_config(dst: &mut UAIceConfig, src: &mut UAMediaConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_ice_config_from_media_config(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn ice_config_dup(dst: &mut UAIceConfig, src: &mut UAIceConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_ice_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn turn_config_from_media_config(dst: &mut UATurnConfig, src: &mut UAMediaConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_turn_config_from_media_config(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn turn_config_dup(dst: &mut UATurnConfig, src: &mut UATurnConfig) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_turn_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        )
    }

    pool_release(pool);
}

pub fn srtp_opt_default(cfg: &mut UASrtpOpt) {
    unsafe {
        pjsua_sys::pjsua_srtp_opt_default(
            cfg as *mut _
        )
    }
}

pub fn srtp_opt_dup(dst: &mut UASrtpOpt, src: &mut UASrtpOpt, check_str: bool) {

    let pool = pool_create("tmp-pool");

    unsafe {
        pjsua_sys::pjsua_srtp_opt_dup(
            pool,
            dst as *mut _,
            src as *const _,
            utils::boolean_to_pjbool(check_str)
        )
    }

    pool_release(pool);
}

pub fn acc_config_default (cfg: &mut UAAccConfig) {
    unsafe { pjsua_sys::pjsua_acc_config_default(cfg as *mut _); }
}

pub fn acc_config_dup (dst: &mut UAAccConfig, src: &mut UAAccConfig) {
    unsafe {
        let pool = pool_create("tmp-pool");

        pjsua_sys::pjsua_acc_config_dup(
            pool,
            dst as *mut _,
            src as *const _
        );

        pool_release(pool);
    }
}

pub fn acc_get_count() -> u32 {
    unsafe { pjsua_sys::pjsua_acc_get_count() }
}

pub fn acc_is_valid(acc_id: i32) -> bool {
    unsafe { utils::check_boolean(pjsua_sys::pjsua_acc_is_valid(acc_id)) }
}

pub fn acc_set_default(acc_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_acc_set_default(acc_id)) }
}

pub fn acc_get_default() -> i32 {
    unsafe { pjsua_sys::pjsua_acc_get_default() }
}

pub fn acc_add(acc_cfg: &mut UAAccConfig, is_default: bool, p_acc_id: &mut i32) -> Result<(), i32> {
    unsafe {

        let status = pjsua_sys::pjsua_acc_add(
            acc_cfg as *const _,
            utils::boolean_to_pjbool(is_default),
            p_acc_id as *mut _
        );

        utils::check_status(status)
    }
}

pub fn acc_add_local(tid: i32, is_default: bool, p_acc_id: &mut i32) -> Result<(), i32> {
    unsafe {

        let status = pjsua_sys::pjsua_acc_add_local(
            tid,
            utils::boolean_to_pjbool(is_default),
            p_acc_id as *mut _
        );

        utils::check_status(status)
    }
}

// i32 	pjsua_acc_set_user_data (i32 acc_id, void *user_data)
// void * 	pjsua_acc_get_user_data (i32 acc_id)

pub fn acc_del(acc_id: i32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_del(acc_id))
    }
}

pub fn acc_get_config (acc_id: i32, acc_cfg: &mut UAAccConfig) -> Result<(), i32> {
    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_sys::pjsua_acc_get_config(acc_id, pool, acc_cfg as *mut _);

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_modify(acc_id: i32, acc_cfg: &mut UAAccConfig) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_modify( acc_id, acc_cfg as *const _ ))
    }
}

pub fn acc_set_online_status(acc_id: i32, is_online: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_set_online_status( acc_id, utils::boolean_to_pjbool(is_online)))
    }
}

pub fn acc_set_online_status2(acc_id: i32, is_online: bool, pr: &mut  pjrpid_element) -> Result<(), i32> {

    unsafe {
        let status = pjsua_sys::pjsua_acc_set_online_status2(
            acc_id,
            utils::boolean_to_pjbool(is_online),
            pr as *const _
        );
        utils::check_status(status)
    }
}

pub fn acc_set_registration(acc_id: i32, renew: bool) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_set_registration( acc_id, utils::boolean_to_pjbool(renew)))
    }
}

pub fn acc_get_info (acc_id: i32, info: &mut UAAccInfo) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_get_info(acc_id, info as *mut _))
    }
}

pub fn enum_accs(ids: &mut [i32; pjsua_sys::PJSUA_MAX_ACC as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_enum_accs( ids.as_mut_ptr(), count as *mut _))
    }
}

pub fn acc_enum_info(info: &mut [UAAccInfo; pjsua_sys::PJSUA_MAX_ACC as usize], count: &mut u32) -> Result<(), i32> {
    unsafe {
        utils::check_status(pjsua_sys::pjsua_acc_enum_info( info.as_mut_ptr(), count as *mut _ ))
    }
}

pub fn acc_find_for_outgoing(url: String) -> i32 {

    let mut url = pj_str_t::from_string(url);

    unsafe {
        pjsua_sys::pjsua_acc_find_for_outgoing(
            &mut url as *const _
        )
    }

}

pub fn acc_find_for_incoming(rdata: &mut pjsip_rx_data) -> i32 {

    unsafe {
        pjsua_sys::pjsua_acc_find_for_incoming(
            rdata as *mut _
        )
    }
}

pub fn acc_create_request(acc_id: i32, method: &mut pjsip_method, target: String, p_tdata: &mut pjsip_tx_data) -> Result<(), i32> {

    let mut target = pj_str_t::from_string(target);

    unsafe {
        let status = pjsua_sys::pjsua_acc_create_request(
            acc_id,
            method as *const _,
            &mut target as *const _,
            (p_tdata as *mut _) as *mut _
        );

        utils::check_status(status)
    }
}

pub fn acc_create_uac_contact(contact: String, acc_id: i32, uri: String) -> Result<(), i32> {

    let mut contact = pj_str_t::from_string(contact);
    let mut uri = pj_str_t::from_string(uri);

    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_sys::pjsua_acc_create_uac_contact(
            pool,
            &mut contact as *mut _,
            acc_id,
            &mut uri as *mut _
        );

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_create_uas_contact(contact: String, acc_id: i32, rdata: &mut pjsip_rx_data) -> Result<(), i32> {

    let mut contact = pj_str_t::from_string(contact);

    unsafe {
        let pool = pool_create("tmp-pool");

        let status = pjsua_sys::pjsua_acc_create_uas_contact(
            pool,
            &mut contact as *mut _,
            acc_id,
            rdata as *mut _
        );

        pool_release(pool);

        utils::check_status(status)
    }
}

pub fn acc_set_transport(acc_id: i32, tp_id: i32) -> Result<(), i32> {
    unsafe { utils::check_status(pjsua_sys::pjsua_acc_set_transport( acc_id, tp_id )) }
}