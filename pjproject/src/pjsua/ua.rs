
use std::convert::TryFrom;
use crate::utils::{boolean_to_pjbool, check_boolean};

use super::*;

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
    fn set_nameserver(&mut self, ns1: Option<String>, ns2: Option<String>, ns3: Option<String>, ns4: Option<String>);
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
    fn set_outbound_proxy(&mut self, proxy1: Option<String>, proxy2: Option<String>, proxy3: Option<String>, proxy4: Option<String>);
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
    fn set_timer_setting(&mut self, min_se: u32, sess_expires: u32);
    fn get_timer_setting(&self) -> (u32, u32);

    /// Number of credentials in the credential array.
    fn set_cred_count(&mut self, value: u32);
    fn get_cred_count(&self) -> u32;

    /// Array of credentials. These credentials will be used by all accounts,
    /// and can be used to authenticate against outbound proxies.
    /// If the credential is specific to the account, then application should
    /// set the credential in the pjsua_acc_config rather than the credential here.
    fn set_cred_info(&mut self, value: Vec<CredentialInfo>);
    fn get_cred_info(&self) -> Vec<CredentialInfo>;

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
    fn set_srtp_secure_signaling(&mut self, value: UAConfigSrtpSecureSignaling);
    fn get_srtp_secure_signaling(&self) -> UAConfigSrtpSecureSignaling;

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

    /// Realm. Use "*" to make a credential that can be used to authenticate
    /// against any challenges.
    fn set_realm(&mut self, value: String);
    fn get_realm(&self) -> String;

    /// Scheme (e.g. "digest").
    fn set_scheme(&mut self, value: String);
    fn get_scheme(&self) -> String;

    /// User name.
    fn set_username(&mut self, value: String);
    fn get_username(&self) -> String;

    /// Type of data (0 for plaintext passwd).
    fn set_data_type(&mut self, value: CredentialInfoType);
    fn get_data_type(&self) -> CredentialInfoType;

    /// The data, which can be a plaintext password or a hashed digest.
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

    fn set_nameserver(&mut self, ns1: Option<String>, ns2: Option<String>, ns3: Option<String>, ns4: Option<String>) {
        // reset nameserver
        self.nameserver_count = 0;

        if ns1.is_some() {
            self.nameserver[self.nameserver_count as usize] = pj_str_t::from_string(ns1.unwrap());
            self.nameserver_count+= 1;
        }

        if ns2.is_some() {
            self.nameserver[self.nameserver_count as usize] = pj_str_t::from_string(ns2.unwrap());
            self.nameserver_count+= 1;
        }

        if ns3.is_some() {
            self.nameserver[self.nameserver_count as usize] = pj_str_t::from_string(ns3.unwrap());
            self.nameserver_count+= 1;
        }

        if ns4.is_some() {
            self.nameserver[self.nameserver_count as usize] = pj_str_t::from_string(ns4.unwrap());
            self.nameserver_count+= 1;
        }
    }

    fn get_nameserver(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        let mut nameserver = (None, None, None, None);

        if self.nameserver_count > 0 {
            nameserver.0 = Some(self.nameserver[0].to_string());
        }

        if self.nameserver_count > 1 {
            nameserver.1 = Some(self.nameserver[1].to_string());
        }

        if self.nameserver_count > 2 {
            nameserver.2 = Some(self.nameserver[2].to_string());
        }

        if self.nameserver_count > 3 {
            nameserver.3 = Some(self.nameserver[3].to_string());
        }

        nameserver
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

    fn set_outbound_proxy(&mut self, proxy1: Option<String>, proxy2: Option<String>, proxy3: Option<String>, proxy4: Option<String>) {
        // reset proxy
        self.outbound_proxy_cnt = 0;

        if proxy1.is_some() {
            self.outbound_proxy[self.outbound_proxy_cnt as usize] = pj_str_t::from_string(proxy1.unwrap());
            self.outbound_proxy_cnt+= 1;
        }

        if proxy2.is_some() {
            self.outbound_proxy[self.outbound_proxy_cnt as usize] = pj_str_t::from_string(proxy2.unwrap());
            self.outbound_proxy_cnt+= 1;
        }

        if proxy3.is_some() {
            self.outbound_proxy[self.outbound_proxy_cnt as usize] = pj_str_t::from_string(proxy3.unwrap());
            self.outbound_proxy_cnt+= 1;
        }

        if proxy4.is_some() {
            self.outbound_proxy[self.outbound_proxy_cnt as usize] = pj_str_t::from_string(proxy4.unwrap());
            self.outbound_proxy_cnt+= 1;
        }
    }

    fn get_outbound_proxy(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        let mut proxy = (None, None, None, None);

        if self.outbound_proxy_cnt > 0 {
            proxy.0 = Some(self.outbound_proxy[0].to_string());
        }

        if self.outbound_proxy_cnt > 1 {
            proxy.1 = Some(self.outbound_proxy[1].to_string());
        }

        if self.outbound_proxy_cnt > 2 {
            proxy.2 = Some(self.outbound_proxy[2].to_string());
        }

        if self.outbound_proxy_cnt > 3 {
            proxy.3 = Some(self.outbound_proxy[3].to_string());
        }

       proxy
    }

    fn set_stun_srv_cnt(&mut self, value: u32) {
        self.stun_srv_cnt = value;
    }

    fn get_stun_srv_cnt(&self) -> u32 {
        self.stun_srv_cnt
    }

    fn set_stun_srv(&mut self, stun1: Option<String>, stun2: Option<String>, stun3: Option<String>, stun4: Option<String>) {
        // reset stun
        self.stun_srv_cnt = 0;

        if stun1.is_some() {
            self.stun_srv[self.stun_srv_cnt as usize] = pj_str_t::from_string(stun1.unwrap());
            self.stun_srv_cnt+= 1;
        }

        if stun2.is_some() {
            self.stun_srv[self.stun_srv_cnt as usize] = pj_str_t::from_string(stun2.unwrap());
            self.stun_srv_cnt+= 1;
        }

        if stun3.is_some() {
            self.stun_srv[self.stun_srv_cnt as usize] = pj_str_t::from_string(stun3.unwrap());
            self.stun_srv_cnt+= 1;
        }

        if stun4.is_some() {
            self.stun_srv[self.stun_srv_cnt as usize] = pj_str_t::from_string(stun4.unwrap());
            self.stun_srv_cnt+= 1;
        }
    }

    fn get_stun_srv(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        let mut stun = (None, None, None, None);

        if self.stun_srv_cnt > 0 {
            stun.0 = Some(self.stun_srv[0].to_string());
        }

        if self.stun_srv_cnt > 1 {
            stun.1 = Some(self.stun_srv[1].to_string());
        }

        if self.stun_srv_cnt > 2 {
            stun.2 = Some(self.stun_srv[2].to_string());
        }

        if self.stun_srv_cnt > 3 {
            stun.3 = Some(self.stun_srv[3].to_string());
        }

       stun
    }

    fn set_stun_try_ipv6(&mut self, value: bool) {
        self.stun_try_ipv6 = boolean_to_pjbool(value);
    }

    fn get_stun_try_ipv6(&self) -> bool {
        check_boolean(self.stun_try_ipv6)
    }

    fn set_stun_ignore_failure(&mut self, value: bool) {
        self.stun_ignore_failure = boolean_to_pjbool(value);
    }

    fn get_stun_ignore_failure(&self) -> bool {
        check_boolean(self.stun_ignore_failure)
    }

    fn set_stun_map_use_stun2(&mut self, value: bool) {
        self.stun_map_use_stun2 = boolean_to_pjbool(value);
    }

    fn get_stun_map_use_stun2(&self) -> bool {
        check_boolean(self.stun_map_use_stun2)
    }

    fn set_nat_type_in_sdp(&mut self, value: i32) {
        self.nat_type_in_sdp = value;
    }

    fn get_nat_type_in_sdp(&self) -> i32 {
        self.nat_type_in_sdp
    }

    fn set_require_100rel(&mut self, value: UAConfig100relUse) {
        self.require_100rel = value.into();
    }

    fn get_require_100rel(&self) -> UAConfig100relUse {
        UAConfig100relUse::try_from(self.require_100rel)
        .expect("Error UAConfig get require_100rel")
    }

    fn set_use_timer(&mut self, value: UAConfigSipTimerUse) {
        self.use_timer = value.into();
    }

    fn get_use_timer(&self) -> UAConfigSipTimerUse {
        UAConfigSipTimerUse::try_from(self.use_timer)
        .expect("Error UAConfig get use_timer")
    }

    fn set_enable_unsolicited_mwi(&mut self, value: bool) {
        self.enable_unsolicited_mwi = boolean_to_pjbool(value);
    }

    fn get_enable_unsolicited_mwi(&self) -> bool {
        check_boolean(self.enable_unsolicited_mwi)
    }

    fn set_timer_setting(&mut self, min_se: u32, sess_expires: u32) {
        self.timer_setting.min_se = min_se;
        self.timer_setting.sess_expires = sess_expires;
    }

    fn get_timer_setting(&self) -> (u32, u32) {
        (self.timer_setting.min_se, self.timer_setting.sess_expires)
    }

    fn set_cred_count(&mut self, value: u32) {
        self.cred_count = value;
    }

    fn get_cred_count(&self) -> u32 {
        self.cred_count
    }

    fn set_cred_info(&mut self, value: Vec<CredentialInfo>) {
        todo!()
    }

    fn get_cred_info(&self) -> Vec<CredentialInfo> {
        todo!()
    }

    fn set_user_agent(&mut self, value: String) {
        self.user_agent = pj_str_t::from_string(value);
    }

    fn get_user_agent(&self) -> String {
        self.user_agent.to_string()
    }

    fn set_use_srtp(&mut self, value: UAConfigSrtpUse) {
        self.use_srtp = value.into();
    }

    fn get_use_srtp(&self) -> UAConfigSrtpUse {
        UAConfigSrtpUse::try_from(self.use_srtp)
        .expect("Error UAConfig get use_srtp")
    }

    fn set_srtp_secure_signaling(&mut self, value: UAConfigSrtpSecureSignaling) {
        self.srtp_secure_signaling = value.into();
    }

    fn get_srtp_secure_signaling(&self) -> UAConfigSrtpSecureSignaling {
        UAConfigSrtpSecureSignaling::try_from(self.srtp_secure_signaling)
        .expect("Error UAConfig get srtp_secure_signaling")
    }

    fn set_srtp_optional_dup_offer(&mut self, value: bool) {
        self.srtp_optional_dup_offer = boolean_to_pjbool(value);
    }

    fn get_srtp_optional_dup_offer(&self) -> bool {
        check_boolean(self.srtp_optional_dup_offer)
    }

    fn set_srtp_opt(&mut self, value: SRTPOption) {
        self.srtp_opt = value;
    }

    fn get_srtp_opt(&self) -> SRTPOption {
        todo!()
    }

    fn set_hangup_forked_call(&mut self, value: bool) {
        self.hangup_forked_call = boolean_to_pjbool(value);
    }

    fn get_hangup_forked_call(&self) -> bool {
        check_boolean(self.hangup_forked_call)
    }
}


impl CredentialInfoExt for CredentialInfo {
    fn set_realm(&mut self, value: String) {
        self.realm = pj_str_t::from_string(value);
    }

    fn get_realm(&self) -> String {
        self.realm.to_string()
    }

    fn set_scheme(&mut self, value: String) {
        self.scheme = pj_str_t::from_string(value);
    }

    fn get_scheme(&self) -> String {
        self.scheme.to_string()
    }

    fn set_username(&mut self, value: String) {
        self.username = pj_str_t::from_string(value);
    }

    fn get_username(&self) -> String {
        self.username.to_string()
    }

    fn set_data_type(&mut self, value: CredentialInfoType) {
        self.data_type = value.into()
    }

    fn get_data_type(&self) -> CredentialInfoType {
        CredentialInfoType::try_from(self.data_type)
        .expect("Error CredentialInfo get data_type")
    }

    fn set_data(&mut self, value: String) {
        self.data = pj_str_t::from_string(value);
    }

    fn get_data(&self) -> String {
        self.data.to_string()
    }
}