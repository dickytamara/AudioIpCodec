use pjmedia_sys::{pjmedia_rtcp_fb_setting, pjmedia_srtp_use};
use pjsip_simple_sys::pjsip_publishc_opt;
use pjsip_sys::{pjsip_auth_clt_pref, pjsip_cred_info};
use pjsip_ua_sys::pjsip_timer_setting;
use pjsua_sys::{pjsua_100rel_use, pjsua_sip_timer_use};

use super::*;


pub trait AccountConfigExt {
    // skiped
    // user_data: *mut c_void,

    /// Account priority, which is used to control the order of matching incoming/outgoing requests.
    /// The higher the number means the higher the priority is, and the account will be matched first.
    fn set_priority(&self, value: i32);
    fn get_priority(&self) -> i32;
    /// The full SIP URL for the account. The value can take name address or URL format,
    /// and will look something like "sip:account@serviceprovider" or "/"
    /// Display Name" <sip:account@provider>".
    ///
    /// This field is mandatory.
    fn set_id(&self, value: String);
    fn get_id(&self) -> String;
    /// This is the URL to be put in the request URI for the registration,
    /// and will look something like "sip:serviceprovider".
    ///
    /// This field should be specified if registration is desired.
    /// If the value is empty, no account registration will be performed.
    fn set_reg_uri(&self, value: String);
    fn get_reg_uri(&self) -> String;
    /// The optional custom SIP headers to be put in the registration request.
    // fn set_reg_hdr_list(&self, value: pjsip_hdr);
    // fn get_reg_hdr_list(&self) -> pjsip_hdr;
    /// Additional parameters that will be appended in the Contact header for this account.
    /// This will only affect REGISTER requests and will be appended after contact_params;
    ///
    /// The parameters should be preceeded by semicolon, and all strings must be properly escaped.
    /// Example: ";my-param=X;another-param=Hi%20there"
    fn set_reg_contact_params(&self, value: String);
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
    fn set_mwi_enabled(&self, value: bool);
    fn get_mwi_enabled(&self) -> bool;
    /// Specify the default expiration time for Message Waiting Indication (RFC 3842)
    /// event subscription. This must not be zero.
    ///
    /// # Default
    /// PJSIP_MWI_DEFAULT_EXPIRES
    fn set_mwi_expires(&self, value: u32);
    fn get_mwi_expires(&self) -> u32;
    /// If this flag is set, the presence information of this account will be
    /// PUBLISH-ed to the server where the account belongs.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_publish_enabled(&self, value: bool);
    fn get_publish_enabled(&self) -> bool;
    /// Event publication options.
    fn set_publish_opt(&self, value: pjsip_publishc_opt);
    // fn get_publish_opt(&self) -> pjsip_publishc_opt;
    /// Maximum time to wait for unpublication transaction(s) to complete during shutdown process,
    /// before sending unregistration. The library tries to wait for the unpublication
    /// (un-PUBLISH) to complete before sending REGISTER request to unregister the account,
    /// during library shutdown process. If the value is set too short, it is possible that the
    /// unregistration is sent before unpublication completes, causing unpublication request to fail.
    ///
    /// # Default
    /// PJSUA_UNPUBLISH_MAX_WAIT_TIME_MSEC
    fn set_unpublish_max_wait_time_msec(&self, value: u32);
    fn get_unpublish_max_wait_time_msec(&self) -> u32;
    /// Authentication preference.
    fn set_auth_pref(&self, value: pjsip_auth_clt_pref);
    // fn get_auth_pref(&self) -> pjsip_auth_clt_pref;
    /// Optional PIDF tuple ID for outgoing PUBLISH and NOTIFY. If this value is not specified,
    /// a random string will be used.
    fn set_pidf_tuple_id(&self, value: String);
    fn get_pidf_tuple_id(&self) -> String;
    /// Optional URI to be put as Contact for this account. It is recommended that this field
    /// is left empty, so that the value will be calculated automatically based on the transport
    /// address.
    fn set_force_contact(&self, value: String);
    fn get_force_contact(&self) -> String;
    /// Additional parameters that will be appended in the Contact header for this account.
    /// This will affect the Contact header in all SIP messages sent on behalf of this account,
    /// including but not limited to REGISTER, INVITE, and SUBCRIBE requests or responses.
    ///
    /// The parameters should be preceeded by semicolon, and all strings must be properly escaped.
    /// Example: ";my-param=X;another-param=Hi%20there"
    fn set_contact_params(&self, value: String);
    fn get_contact_params(&self) -> String;
    /// Additional URI parameters that will be appended in the Contact URI for this account.
    /// This will affect the Contact URI in all SIP messages sent on behalf of this account,
    /// including but not limited to REGISTER, INVITE, and SUBCRIBE requests or responses.
    ///
    /// The parameters should be preceeded by semicolon, and all strings must be properly escaped.
    /// Example: ";my-param=X;another-param=Hi%20there"
    fn set_contact_uri_params(&self, value: String);
    fn get_contact_uri_params(&self) -> String;
    /// Specify how support for reliable provisional response (100rel/ PRACK) should be used for
    /// all sessions in this account. See the documentation of pjsua_100rel_use enumeration for more info.
    ///
    /// # Default
    /// The default value is taken from the value of require_100rel in pjsua_config.
    fn set_require_100rel(&self, value: pjsua_100rel_use);
    fn get_require_100rel(&self) -> pjsua_100rel_use;
    /// Specify the usage of Session Timers for all sessions. See the pjsua_sip_timer_use for possible values.
    ///
    /// # Default
    /// PJSUA_SIP_TIMER_OPTIONAL
    fn set_use_timer(&self, value: pjsua_sip_timer_use);
    fn get_use_timer(&self) -> pjsua_sip_timer_use;
    /// Specify Session Timer settings, see pjsip_timer_setting.
    fn set_timer_setting(&self, value: pjsip_timer_setting);
    fn get_timer_setting(&self) -> pjsip_timer_setting;
    /// Number of proxies in the proxy array below.
    fn set_proxy_cnt(&self, value: u32);
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
    fn set_proxy(&self, value: [String; 8usize]);
    fn get_proxy(&self) -> [String; 8usize];
    /// If remote sends SDP answer containing more than one format or codec in the media line,
    /// send re-INVITE or UPDATE with just one codec to lock which codec to use.
    ///
    /// # Default
    /// 1 (Yes). Set to zero to disable.
    fn set_lock_codec(&self, value: u32);
    fn get_lock_codec(&self) -> u32;
    /// Optional interval for registration, in seconds. If the value is zero,
    /// default interval will be used (PJSUA_REG_INTERVAL, 300 seconds).
    fn set_reg_timeout(&self, value: u32);
    fn get_reg_timeout(&self) -> u32;
    /// Specify the number of seconds to refresh the client registration before the
    /// registration expires.
    ///
    /// # Default
    /// PJSIP_REGISTER_CLIENT_DELAY_BEFORE_REFRESH, 5 seconds
    fn set_reg_delay_before_refresh(&self, value: u32);
    fn get_reg_delay_before_refresh(&self) -> u32;
    /// Specify the maximum time to wait for unregistration requests to complete
    /// during library shutdown sequence.
    ///
    /// # Default
    /// PJSUA_UNREG_TIMEOUT
    fn set_unreg_timeout(&self, value: u32);
    fn get_unreg_timeout(&self) -> u32;
    /// Number of credentials in the credential array.
    fn set_cred_count(&self, value: u32);
    fn get_cred_count(&self) -> u32;
    /// Array of credentials. If registration is desired, normally there should be at least
    /// one credential specified, to successfully authenticate against the service provider.
    /// More credentials can be specified, for example when the requests are expected to be
    /// challenged by the proxies in the route set.
    fn set_cred_info(&self, value: [pjsip_cred_info; 8usize]);
    // fn get_cred_info(&self) -> [pjsip_cred_info; 8usize];
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
    fn set_transport_id(&self, value: i32);
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
    fn set_allow_contact_rewrite(&self, value: bool);
    fn get_allow_contact_rewrite(&self) -> bool;
    /// Specify how Contact update will be done with the registration, if allow_contact_rewrite is enabled.
    /// The value is bitmask combination of pjsua_contact_rewrite_method. See also pjsua_contact_rewrite_method.
    ///
    /// Value PJSUA_CONTACT_REWRITE_UNREGISTER(1) is the legacy behavior.
    ///
    /// # Default
    /// value: PJSUA_CONTACT_REWRITE_METHOD (PJSUA_CONTACT_REWRITE_NO_UNREG | PJSUA_CONTACT_REWRITE_ALWAYS_UPDATE)
    fn set_contact_rewrite_method(&self, value: i32);
    fn get_contact_rewrite_method(&self) -> i32;
    /// Specify if source TCP port should be used as the initial Contact address if TCP/TLS transport is used.
    /// Note that this feature will be automatically turned off when nameserver is configured because it
    /// may yield different destination address due to DNS SRV resolution. Also some platforms are unable to 
    /// report the local address of the TCP socket when it is still connecting. In these cases, this feature
    /// will also be turned off.
    ///
    /// # Default
    /// PJ_TRUE (yes).
    fn set_contact_use_src_port(&self, value: bool);
    fn get_contact_use_src_port(&self) -> bool;
    /// This option is used to overwrite the "sent-by" field of the Via header for outgoing messages with
    /// the same interface address as the one in the REGISTER request, as long as the request uses the same 
    /// transport instance as the previous REGISTER request.
    ///
    /// # Default
    /// 1 (yes)
    fn set_allow_via_rewrite(&self, value: bool);
    fn get_allow_via_rewrite(&self) -> bool;
    /// This option controls whether the IP address in SDP should be replaced with the IP address found
    /// in Via header of the REGISTER response, ONLY when STUN and ICE are not used. If the value is FALSE
    /// (the original behavior), then the local IP address will be used. If TRUE, and when STUN and ICE
    /// are disabled, then the IP address found in registration response will be used.
    ///
    /// # Default
    /// PJ_FALSE (no)
    fn set_allow_sdp_nat_rewrite(&self, value: bool);
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
    fn set_use_rfc5626(&self, value: u32);
    fn get_use_rfc5626(&self) -> u32;
    /// Specify SIP outbound (RFC 5626) instance ID to be used by this application. If empty,
    /// an instance ID will be generated based on the hostname of this agent. If application
    /// specifies this parameter, the value will
    /// look like "<urn:uuid:00000000-0000-1000-8000-AABBCCDDEEFF>" without the doublequote.
    ///
    /// # Default
    /// empty
    fn set_rfc5626_instance_id(&self, value: String);
    fn get_rfc5626_instance_id(&self) -> String;
    // Specify SIP outbound (RFC 5626) registration ID. The default value is empty,
    // which would cause the library to automatically generate a suitable value.
    //
    // # Default
    // empty
    fn set_rfc5626_reg_id(&self, value: String);
    fn get_rfc5626_reg_id(&self) -> String;
    /// Set the interval for periodic keep-alive transmission for this account. If this value is zero,
    /// keep-alive will be disabled for this account. The keep-alive transmission will be sent to the
    /// registrar's address, after successful registration.
    ///
    /// # Default
    /// 15 (seconds)
    fn set_ka_interval(&self, value: u32);
    fn get_ka_interval(&self) -> u32;
    /// Specify the data to be transmitted as keep-alive packets.
    ///
    /// # Default
    /// CR-LF
    fn set_ka_data(&self, value: String);
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
    fn set_rtp_cfg(&self, value: TransportConfig);
    // fn get_rtp_cfg(&self) -> TransportConfig;
    /// Specify NAT64 options.
    ///
    /// # Default
    /// PJSUA_NAT64_DISABLED
    fn set_nat64_opt(&self, value: pjsua_nat64_opt);
    fn get_nat64_opt(&self) -> pjsua_nat64_opt;
    /// Specify whether IPv6 should be used on media.
    fn set_ipv6_media_use(&self, value: pjsua_ipv6_use);
    fn get_ipv6_media_use(&self) -> pjsua_ipv6_use;
    /// Control the use of STUN for the SIP signaling.
    ///
    /// # Default
    /// PJSUA_STUN_USE_DEFAULT
    fn set_sip_stun_use(&self, value: pjsua_stun_use);
    fn get_sip_stun_use(&self) -> pjsua_stun_use;
    /// Control the use of STUN for the media transports.
    ///
    /// # Default
    /// PJSUA_STUN_RETRY_ON_FAILURE
    fn set_media_stun_use(&self, value: pjsua_stun_use);
    fn get_media_stun_use(&self) -> pjsua_stun_use;
    /// Use loopback media transport. This may be useful if application doesn't want PJSIP
    /// to create real media transports/sockets, such as when using third party media.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_use_loop_med_tp(&self, value: bool);
    fn get_use_loop_med_tp(&self) -> bool;
    /// Enable local loopback when loop_med_tp_use is set to PJ_TRUE. If enabled, packets sent
    /// to the transport will be sent back to the streams attached to the transport.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_enable_loopback(&self, value: bool);
    fn get_enable_loopback(&self) -> bool;
    /// Control the use of ICE in the account. By default, the settings in the
    /// pjsua_media_config will be used.
    ///
    /// # Default
    /// PJSUA_ICE_CONFIG_USE_DEFAULT
    fn set_ice_cfg_use(&self, value: pjsua_ice_config_use);
    fn get_ice_cfg_use(&self) -> pjsua_ice_config_use;
    /// The custom ICE setting for this account. This setting will only be used if ice_cfg_use
    /// is set to PJSUA_ICE_CONFIG_USE_CUSTOM
    fn set_ice_cfg(&self, value: pjsua_ice_config);
    // fn get_ice_cfg(&self) -> pjsua_ice_config;
    /// Control the use of TURN in the account. By default, the settings in the
    /// pjsua_media_config will be used
    ///
    /// # Default
    /// PJSUA_TURN_CONFIG_USE_DEFAULT
    fn set_turn_cfg_use(&self, value: pjsua_turn_config_use);
    fn get_turn_cfg_use(&self) -> pjsua_turn_config_use;
    /// The custom TURN setting for this account. This setting will only be used if turn_cfg_use
    /// is set to PJSUA_TURN_CONFIG_USE_CUSTOM
    fn set_turn_cfg(&self, value: pjsua_turn_config);
    // fn get_turn_cfg(&self) -> pjsua_turn_config;
    /// Specify whether secure media transport should be used for this account. Valid values are
    /// PJMEDIA_SRTP_DISABLED, PJMEDIA_SRTP_OPTIONAL, and PJMEDIA_SRTP_MANDATORY.
    ///
    /// # Default
    /// PJSUA_DEFAULT_USE_SRTP
    fn set_use_srtp(&self, value: pjmedia_srtp_use);
    fn get_use_srtp(&self) -> pjmedia_srtp_use;
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
    fn set_srtp_secure_signaling(&self, value: i32);
    fn get_srtp_secure_signaling(&self) -> i32;
    /// This setting has been deprecated and will be ignored.
    fn set_srtp_optional_dup_offer(&self, value: bool);
    fn get_srtp_optional_dup_offer(&self) -> bool;
    /// Specify SRTP transport setting. Application can initialize it with default values using
    /// pjsua_srtp_opt_default().
    fn set_srtp_opt(&self, value: pjsua_srtp_opt);
    fn get_srtp_opt(&self) -> pjsua_srtp_opt;
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
    fn set_reg_retry_interval(&self, value: u32);
    fn get_reg_retry_interval(&self) -> u32;
    /// This specifies the interval for the first registration retry. The registration retry is explained
    /// in reg_retry_interval. Note that the value here will also be randomized by some seconds
    /// (specified in reg_retry_random_interval) to avoid all clients re-registering at the same time.
    ///
    /// # Default
    /// 0
    fn set_reg_first_retry_interval(&self, value: u32);
    fn get_reg_first_retry_interval(&self) -> u32;
    /// This specifies maximum randomized value to be added/substracted to/from the registration
    /// retry interval specified in reg_retry_interval and reg_first_retry_interval, in second.
    /// This is useful to avoid all clients re-registering at the same time. For example, if the
    /// registration retry interval is set to 100 seconds and this is set to 10 seconds, the actual
    /// registration retry interval will be in the range of 90 to 110 seconds.
    ///
    /// # Default
    /// 10
    fn set_reg_retry_random_interval(&self, value: u32);
    fn get_reg_retry_random_interval(&self) -> u32;
    /// Specify whether calls of the configured account should be dropped after registration failure and
    /// an attempt of re-registration has also failed.
    ///
    /// # Default
    /// PJ_FALSE (disabled)
    fn set_drop_calls_on_reg_fail(&self, value: bool);
    fn get_drop_calls_on_reg_fail(&self) -> bool;
    // Specify how the registration uses the outbound and account proxy settings. This controls if
    /// and what Route headers will appear in the REGISTER request of this account. The value is bitmask
    /// combination of PJSUA_REG_USE_OUTBOUND_PROXY and PJSUA_REG_USE_ACC_PROXY bits. If the value is set to 0,
    /// the REGISTER request will not use any proxy (i.e. it will not have any Route headers).
    ///
    /// # Default
    /// 3 (PJSUA_REG_USE_OUTBOUND_PROXY | PJSUA_REG_USE_ACC_PROXY)
    fn set_reg_use_proxy(&self, value: u32);
    fn get_reg_use_proxy(&self) -> u32;
    /// Specify how to offer call hold to remote peer. Please see the documentation on
    /// pjsua_call_hold_type for more info.
    ///
    /// # Default
    /// PJSUA_CALL_HOLD_TYPE_DEFAULT
    fn set_call_hold_type(&self, value: pjsua_call_hold_type);
    fn get_call_hold_type(&self) -> u32;
    /// Specify whether the account should register as soon as it is added to the UA. Application can
    /// set this to PJ_FALSE and control the registration manually with pjsua_acc_set_registration().
    ///
    /// # Default
    /// PJ_TRUE
    fn set_register_on_acc_add(&self, value: bool);
    fn get_register_on_acc_add(&self) -> bool;
    /// Specify account configuration specific to IP address change used when calling
    /// pjsua_handle_ip_change().
    fn set_ip_change_cfg(&self, value: pjsua_ip_change_acc_cfg);
    // fn get_ip_change_cfg(&self) -> pjsua_ip_change_acc_cfg;
    /// Enable RTP and RTCP multiplexing.
    fn set_enable_rtcp_mux(&self, value: bool);
    fn get_enable_rtcp_mux(&self) -> bool;
    /// RTCP Feedback configuration.
    fn set_rtcp_fb_cfg(&self, value: pjmedia_rtcp_fb_setting);
    // fn get_rtcp_fb_cfg(&self) -> pjmedia_rtcp_fb_setting;
}
