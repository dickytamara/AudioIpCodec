
use pjsip_ua_sys::pjsip_timer_setting;
use crate::utils::{boolean_to_pjbool, check_boolean};

use super::*;

pub struct SIPOutboundProxyServerData {
    server: String,
    username: String,
    password: String
}

impl SIPOutboundProxyServerData {
    pub fn new(svr: String, user: String, pass: String) -> Self {
        SIPOutboundProxyServerData {
            server: svr,
            username: user,
            password: pass
        }
    }
}

pub trait UAConfigExt {
    /// Set Maximum calls to support (default: 4). The value specified here must be smaller
    /// than the compile time maximum settings PJSUA_MAX_CALLS, which by default is 32.
    /// To increase this limit, the library must be recompiled with new PJSUA_MAX_CALLS value.
    fn set_max_calls (&mut self, value: u32);
    fn get_max_calls (&self) -> u32;

    /// Number of worker threads. Normally application will want to have at least one worker thread,
    /// unless when it wants to poll the library periodically,
    /// which in this case the worker thread can be set to zero.
    fn set_thread_cnt(&mut self, value: u32);
    fn get_thread_cnt(&self) -> u32;

    /// Number of nameservers. If no name server is configured,
    /// the SIP SRV resolution would be disabled,
    /// and domain will be resolved with standard pj_gethostbyname() function.
    fn set_nameserver_count(&mut self, value: u32);
    fn get_nameserver_count(&self) -> u32;

    /// Array of nameservers to be used by the SIP resolver subsystem.
    /// The order of the name server specifies the priority
    /// (first name server will be used first, unless it is not reachable).
    fn set_nameserver(&mut self, ns1: Option<String>, ns2: Option<String>, ns3: Option<String>, ns4: Option<String>) -> Result<(), i32>;
    fn get_nameserver(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>);

    /// Force loose-route to be used in all route/proxy URIs (outbound_proxy and account's proxy settings).
    /// When this setting is enabled, the library will check all the route/proxy
    /// URIs specified in the settings and append ";lr" parameter to the URI if the parameter is not present.
    fn set_force_lr(&mut self, value: bool);
    fn get_force_lr(&self) -> bool;

    /// Number of outbound proxies in the outbound_proxy array.
    fn set_outbound_proxy_cnt(&mut self, value: u32);
    fn get_outbound_proxy_cnt(&self) -> u32;

    /// Specify the URL of outbound proxies to visit for all outgoing requests.
    /// The outbound proxies will be used for all accounts,
    /// and it will be used to build the route set for outgoing requests.
    /// The final route set for outgoing requests will consists of the outbound
    /// proxies and the proxy configured in the account.
    // fn set_outbound_proxy(&self, value: Vec<SIPOutboundProxyServerData>) -> Result<(), i32>;
    fn set_outbound_proxy(&self, proxy1: Option<String>, proxy2: Option<String>, proxy3: Option<String>, proxy4: Option<String>);
    fn get_outbound_proxy(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>);

    /// Get Number of STUN server entries in stun_srv array.
    fn set_stun_srv_cnt(&mut self, value: u32);
    fn get_stun_srv_cnt(&self) -> u32;

    /// Array of STUN servers to try. The library will try to resolve and contact each of the
    /// STUN server entry until it finds one that is usable. Each entry may be a domain name,
    /// host name, IP address, and it may contain an optional port number. For example:
    ///
    /// - "pjsip.org" (domain name)
    /// - "sip.pjsip.org" (host name)
    /// - "pjsip.org:33478" (domain name and a non-standard port number)
    /// - "10.0.0.1:3478" (IP address and port number)
    ///
    /// When nameserver is configured in the pjsua_config.nameserver field,
    /// if entry is not an IP address, it will be resolved with DNS SRV resolution first,
    /// and it will fallback to use DNS A resolution if this fails.
    /// Port number may be specified even if the entry is a domain name,
    /// in case the DNS SRV resolution should fallback to a non-standard port.
    ///
    /// When nameserver is not configured, entries will be resolved with pj_gethostbyname()
    /// if it's not an IP address. Port number may be specified
    /// if the server is not listening in standard STUN port.
    fn set_stun_srv(&mut self, stun1: Option<String>, stun2: Option<String>, stun3: Option<String>, stun4: Option<String>);
    fn get_stun_srv(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>);

    /// This specifies if the library should try to do an IPv6 resolution of
    /// the STUN servers if the IPv4 resolution fails. It can be useful in an
    /// IPv6-only environment, including on NAT64.
    ///
    /// #Default
    /// PJ_FALSE
    fn set_stun_try_ipv6(&mut self, value: bool);
    fn get_stun_try_ipv6(&self) -> bool;

    /// This specifies if the library should ignore failure with the STUN servers.
    /// If this is set to PJ_FALSE, the library will refuse to start if it fails
    /// to resolve or contact any of the STUN servers.
    ///
    /// This setting will also determine what happens if STUN servers are unavailable
    /// during runtime (if set to PJ_FALSE, calls will directly fail, otherwise
    /// (if PJ_TRUE) call medias will fallback to proceed as though not using STUN servers.
    ///
    /// # Default
    /// PJ_TRUE
    fn set_stun_ignore_failure(&mut self, value: bool);
    fn get_stun_ignore_failure(&self) -> bool;

    /// This specifies whether STUN requests for resolving socket mapped address should use the new format,
    /// i.e: having STUN magic cookie in its transaction ID.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_stun_map_use_stun2(&mut self, value: bool);
    fn get_stun_map_use_stun2(&self) -> bool;

    /// Support for adding and parsing NAT type in the SDP to assist troubleshooting. The valid values are:
    ///
    /// - 0: no information will be added in SDP, and parsing is disabled.
    /// - 1: only the NAT type number is added.
    /// - 2: add both NAT type number and name.
    ///
    /// # Default
    /// 1
    fn set_nat_type_in_sdp(&mut self, value: i32);
    fn get_nat_type_in_sdp(&self) -> i32;

    /// Specify how the support for reliable provisional response (100rel/ PRACK) should be used by default.
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// [Default] PJSUA_100REL_NOT_USED
    fn set_require_100rel(&mut self, value: UAConfig100relUse);
    fn get_require_100rel(&self) -> UAConfig100relUse;

    /// Specify the usage of Session Timers for all sessions. See the pjsua_sip_timer_use for possible values.
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// # Default
    /// PJSUA_SIP_TIMER_OPTIONAL
    fn set_use_timer(&mut self, value: UAConfigSipTimerUse);
    fn get_use_timer(&self) -> UAConfigSipTimerUse;

    /// Handle unsolicited NOTIFY requests containing message waiting indication (MWI) info.
    /// Unsolicited MWI is incoming NOTIFY requests which are not requested by client with SUBSCRIBE request.
    ///
    /// If this is enabled, the library will respond 200/OK
    /// to the NOTIFY request and forward the request to on_mwi_info() callback.
    ///
    /// See also mwi_enabled field #on pjsua_acc_config.
    ///
    /// # Default
    /// PJ_TRUE
    fn set_enable_unsolicited_mwi(&mut self, value: bool);
    fn get_enable_unsolicited_mwi(&self) -> bool;

    /// Specify Session Timer settings, see pjsip_timer_setting.
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    fn set_timer_setting(&mut self, value: pjsip_timer_setting);
    fn get_timer_setting(&self) -> pjsip_timer_setting;

    /// Number of credentials in the credential array.
    fn set_cred_count(&mut self, value: u32);
    fn get_cred_count(&self) -> u32;

    /// Array of credentials. These credentials will be used by all accounts,
    /// and can be used to authenticate against outbound proxies.
    /// If the credential is specific to the account, then application should
    /// set the credential in the pjsua_acc_config rather than the credential here.
    fn set_cred_info(&mut self, value: [CredentialInfo; 8usize]);
    fn get_cred_info(&self) -> [CredentialInfo; 8usize];

    // TODO create pjsua_callback.
    // Application callback to receive various event notifications from the library.
    // pub cb: pjsua_callback,

    /// Optional user agent string (default empty).
    /// If it's empty, no User-Agent header will be sent with outgoing requests.
    fn set_user_agent(&mut self, value: String);
    fn get_user_agent(&self) -> String;

    /// Specify default value of secure media transport usage.
    /// Valid values are PJMEDIA_SRTP_DISABLED, PJMEDIA_SRTP_OPTIONAL, and PJMEDIA_SRTP_MANDATORY.
    ///
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// # Default
    /// PJSUA_DEFAULT_USE_SRTP`
    fn set_use_srtp(&mut self, value: UAConfigSrtpUse);
    fn get_use_srtp(&self) -> UAConfigSrtpUse;

    /// Specify whether SRTP requires secure signaling to be used. This option is only used when use_srtp option above is non-zero.
    ///
    /// Valid values are:
    /// - 0: SRTP does not require secure signaling
    /// - 1: SRTP requires secure transport such as TLS
    /// - 2: SRTP requires secure end-to-end transport (SIPS)
    ///
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// # Default
    /// PJSUA_DEFAULT_SRTP_SECURE_SIGNALING
    fn set_srtp_secure_signaling(&mut self, value: i32);
    fn get_srtp_secure_signaling(&self) -> i32;

    /// This setting has been deprecated and will be ignored.
    fn set_srtp_optional_dup_offer(&mut self, value: bool);
    fn get_srtp_optional_dup_offer(&self) -> bool;

    /// Specify SRTP transport setting. Application can initialize it with
    /// default values using pjsua_srtp_opt_default().
    fn set_srtp_opt(&mut self, value: SRTPOption);
    fn get_srtp_opt(&self) -> SRTPOption;

    /// Disconnect other call legs when more than one 2xx responses for outgoing INVITE
    /// are received due to forking. Currently the library is not able to handle simultaneous
    /// forked media, so disconnecting the other call legs is necessary.
    ///
    /// With this setting enabled, the library will handle only one of the connected call leg,
    /// and the other connected call legs will be disconnected.
    ///
    /// # Default
    /// PJ_TRUE (only disable this setting for testing purposes).
    fn set_hangup_forked_call(&mut self, value: bool);
    fn get_hangup_forked_call(&self) -> bool;
}

pub trait CredentialInfoExt {
    fn set_realm(&mut self, value: String);
    fn get_realm(&self) -> String;

    fn set_scheme(&mut self, value: String);
    fn get_scheme(&self) -> String;

    fn set_username(&mut self, value: String);
    fn get_username(&self) -> String;

    fn set_data_type(&mut self, value: i32);
    fn get_data_type(&self) -> i32;

    fn set_data(&mut self, value: String);
    fn get_data(&self) -> String;
}


impl UAConfigExt for UAConfig {
    fn set_max_calls (&mut self, value: u32) {
        self.max_calls = value;
    }

    fn get_max_calls (&self) -> u32 {
        self.max_calls
    }

    fn set_thread_cnt(&mut self, value: u32) {
        self.thread_cnt = value;
    }

    fn get_thread_cnt(&self) -> u32 {
        self.thread_cnt
    }

    fn set_nameserver_count(&mut self, value: u32) {
        self.nameserver_count = value;
    }

    fn get_nameserver_count(&self) -> u32 {
        self.nameserver_count
    }

    fn set_nameserver(&mut self, ns1: Option<String>, ns2: Option<String>, ns3: Option<String>, ns4: Option<String>) -> Result<(), i32> {
        todo!()
    }

    fn get_nameserver(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        todo!()
    }

    fn set_force_lr(&mut self, value: bool) {
        self.force_lr = boolean_to_pjbool(value);
    }

    fn get_force_lr(&self) -> bool {
        check_boolean(self.force_lr)
    }

    fn set_outbound_proxy_cnt(&mut self, value: u32) {
        self.outbound_proxy_cnt = value;
    }

    fn get_outbound_proxy_cnt(&self) -> u32 {
        self.outbound_proxy_cnt
    }

    fn set_outbound_proxy(&self, proxy1: Option<String>, proxy2: Option<String>, proxy3: Option<String>, proxy4: Option<String>) {
        todo!()
    }

    fn get_outbound_proxy(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        todo!()
    }

    fn set_stun_srv_cnt(&mut self, value: u32) {
        self.stun_srv_cnt = value;
    }

    fn get_stun_srv_cnt(&self) -> u32 {
        self.stun_srv_cnt
    }

    fn set_stun_srv(&mut self, stun1: Option<String>, stun2: Option<String>, stun3: Option<String>, stun4: Option<String>) {
        todo!()
    }

    fn get_stun_srv(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        todo!()
    }

    fn set_stun_try_ipv6(&mut self, value: bool) {
        todo!()
    }

    fn get_stun_try_ipv6(&self) -> bool {
        todo!()
    }

    fn set_stun_ignore_failure(&mut self, value: bool) {
        todo!()
    }

    fn get_stun_ignore_failure(&self) -> bool {
        todo!()
    }

    fn set_stun_map_use_stun2(&mut self, value: bool) {
        todo!()
    }

    fn get_stun_map_use_stun2(&self) -> bool {
        todo!()
    }

    fn set_nat_type_in_sdp(&mut self, value: i32) {
        todo!()
    }

    fn get_nat_type_in_sdp(&self) -> i32 {
        todo!()
    }

    fn set_require_100rel(&mut self, value: UAConfig100relUse) {
        todo!()
    }

    fn get_require_100rel(&self) -> UAConfig100relUse {
        todo!()
    }

    fn set_use_timer(&mut self, value: UAConfigSipTimerUse) {
        todo!()
    }

    fn get_use_timer(&self) -> UAConfigSipTimerUse {
        todo!()
    }

    fn set_enable_unsolicited_mwi(&mut self, value: bool) {
        todo!()
    }

    fn get_enable_unsolicited_mwi(&self) -> bool {
        todo!()
    }

    fn set_timer_setting(&mut self, value: pjsip_timer_setting) {
        todo!()
    }

    fn get_timer_setting(&self) -> pjsip_timer_setting {
        todo!()
    }

    fn set_cred_count(&mut self, value: u32) {
        todo!()
    }

    fn get_cred_count(&self) -> u32 {
        todo!()
    }

    fn set_cred_info(&mut self, value: [CredentialInfo; 8usize]) {
        todo!()
    }

    fn get_cred_info(&self) -> [CredentialInfo; 8usize] {
        todo!()
    }

    fn set_user_agent(&mut self, value: String) {
        todo!()
    }

    fn get_user_agent(&self) -> String {
        todo!()
    }

    fn set_use_srtp(&mut self, value: UAConfigSrtpUse) {
        todo!()
    }

    fn get_use_srtp(&self) -> UAConfigSrtpUse {
        todo!()
    }

    fn set_srtp_secure_signaling(&mut self, value: i32) {
        todo!()
    }

    fn get_srtp_secure_signaling(&self) -> i32 {
        todo!()
    }

    fn set_srtp_optional_dup_offer(&mut self, value: bool) {
        todo!()
    }

    fn get_srtp_optional_dup_offer(&self) -> bool {
        todo!()
    }

    fn set_srtp_opt(&mut self, value: pjsua_srtp_opt) {
        todo!()
    }

    fn get_srtp_opt(&self) -> pjsua_srtp_opt {
        todo!()
    }

    fn set_hangup_forked_call(&mut self, value: bool) {
        todo!()
    }

    fn get_hangup_forked_call(&self) -> bool {
        todo!()
    }
}


impl CredentialInfoExt for CredentialInfo {
    fn set_realm(&mut self, value: String) {
        todo!()
    }

    fn get_realm(&self) -> String {
        todo!()
    }

    fn set_scheme(&mut self, value: String) {
        todo!()
    }

    fn get_scheme(&self) -> String {
        todo!()
    }

    fn set_username(&mut self, value: String) {
        todo!()
    }

    fn get_username(&self) -> String {
        todo!()
    }

    fn set_data_type(&mut self, value: i32) {
        todo!()
    }

    fn get_data_type(&self) -> i32 {
        todo!()
    }

    fn set_data(&mut self, value: String) {
        todo!()
    }

    fn get_data(&self) -> String {
        todo!()
    }
}