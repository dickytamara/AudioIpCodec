
use std::cell::{RefCell, RefMut};


use pj_sys::*;
use pjsip_sys::*;
use pjmedia_sys::*;
use pjsua_sys::*;

use crate::pjproject::pjdefault::{self, AutoCreate, FromString, ToString};
use crate::pjproject::pjsua;


// high layer API
// this API provide error message and
// memory safety when interact with pjsua basic api

pub struct SIPUa {
    // give mutable interior for ensure ctx not moved to anywhere.
    ctx: RefCell<pjsua_config>
}

/// this trait handle all pjsua_config struct fields
pub trait SIPUaExt {
    /// Set Maximum calls to support (default: 4). The value specified here must be smaller
    /// than the compile time maximum settings PJSUA_MAX_CALLS, which by default is 32.
    /// To increase this limit, the library must be recompiled with new PJSUA_MAX_CALLS value.
    fn set_max_calls (&self, max_calls: u32);
    fn get_max_calls (&self) -> u32;
    /// Number of worker threads. Normally application will want to have at least one worker thread,
    /// unless when it wants to poll the library periodically,
    /// which in this case the worker thread can be set to zero.
    fn set_thread_cnt(&self, value: u32);
    fn get_thread_cnt(&self) -> u32;
    /// Number of nameservers. If no name server is configured,
    /// the SIP SRV resolution would be disabled,
    /// and domain will be resolved with standard pj_gethostbyname() function.
    fn set_nameserver_count(&self, value: u32);
    fn get_nameserver_count(&self) -> u32;
    /// Array of nameservers to be used by the SIP resolver subsystem.
    /// The order of the name server specifies the priority
    /// (first name server will be used first, unless it is not reachable).
    fn set_nameserver(&self, nameserver: [String; 4usize]);
    fn get_nameserver(&self) -> [String; 4usize];
    /// Force loose-route to be used in all route/proxy URIs (outbound_proxy and account's proxy settings).
    /// When this setting is enabled, the library will check all the route/proxy
    /// URIs specified in the settings and append ";lr" parameter to the URI if the parameter is not present.
    fn set_force_lr(&self, value: bool);
    fn get_force_lr(&self) -> bool;
    /// Number of outbound proxies in the outbound_proxy array.
    fn set_outbound_proxy_cnt(&self, value: u32);
    fn get_outbound_proxy_cnt(&self) -> u32;
    /// Specify the URL of outbound proxies to visit for all outgoing requests.
    /// The outbound proxies will be used for all accounts,
    /// and it will be used to build the route set for outgoing requests.
    /// The final route set for outgoing requests will consists of the outbound
    /// proxies and the proxy configured in the account.
    fn set_outbound_proxy(&self, value: [String; 4usize]);
    fn get_outbound_proxy(&self) -> [String; 4usize];
    /// Warning: deprecated, please use stun_srv field instead.
    /// To maintain backward compatibility, if stun_srv_cnt is zero then
    /// the value of this field will be copied to stun_srv field, if present.
    ///
    /// Get Specify domain name to be resolved with DNS SRV
    /// resolution to get the address of the STUN server.
    /// Alternatively application may specify stun_host instead.
    ///
    /// If DNS SRV resolution failed for this domain, then DNS A resolution will be
    /// performed only if stun_host is specified.
    fn set_stun_domain(&self, value: String);
    fn get_stun_domain(&self) -> String;
    /// Warning: deprecated, please use stun_srv field instead.
    /// To maintain backward compatibility, if stun_srv_cnt is zero then the
    /// value of this field will be copied to stun_srv field, if present.
    ///
    /// Get Specify STUN server to be used, in "HOST\[:PORT\]" format.
    /// If port is not specified, default port 3478 will be used.
    fn set_stun_host(&self, value: String);
    fn get_stun_host(&self) -> String;
    /// Get Number of STUN server entries in stun_srv array.
    fn set_stun_srv_cnt(&self, value: u32);
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
    fn set_stun_srv(&self, value: [String; 8usize]);
    fn get_stun_srv(&self) -> [String; 8usize];
    /// This specifies if the library should try to do an IPv6 resolution of
    /// the STUN servers if the IPv4 resolution fails. It can be useful in an
    /// IPv6-only environment, including on NAT64.
    ///
    /// #Default
    /// PJ_FALSE
    fn set_stun_try_ipv6(&self, value: bool);
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
    fn set_stun_ignore_failure(&self, value: bool);
    fn get_stun_ignore_failure(&self) -> bool;
    /// This specifies whether STUN requests for resolving socket mapped address should use the new format,
    /// i.e: having STUN magic cookie in its transaction ID.
    ///
    /// # Default
    /// PJ_FALSE
    fn set_stun_map_use_stun2(&self, value: bool);
    fn get_stun_map_use_stun2(&self) -> bool;
    /// Support for adding and parsing NAT type in the SDP to assist troubleshooting. The valid values are:
    ///
    /// - 0: no information will be added in SDP, and parsing is disabled.
    /// - 1: only the NAT type number is added.
    /// - 2: add both NAT type number and name.
    ///
    /// # Default
    /// 1
    fn set_nat_type_in_sdp(&self, value: i32);
    fn get_nat_type_in_sdp(&self) -> i32;
    /// Specify how the support for reliable provisional response (100rel/ PRACK) should be used by default.
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// [Default] PJSUA_100REL_NOT_USED
    fn set_require_100rel(&self, value: pjsua_100rel_use);
    fn get_require_100rel(&self) -> pjsua_100rel_use;
    /// Specify the usage of Session Timers for all sessions. See the pjsua_sip_timer_use for possible values.
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// # Default
    /// PJSUA_SIP_TIMER_OPTIONAL
    fn set_use_timer(&self, value: pjsua_sip_timer_use);
    fn get_use_timer(&self) -> pjsua_sip_timer_use;
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
    fn set_enable_unsolicited_mwi(&self, value: bool);
    fn get_enable_unsolicited_mwi(&self) -> bool;
    /// Specify Session Timer settings, see pjsip_timer_setting.
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    fn set_timer_setting(&self, value: pjsip_timer_setting);
    fn get_timer_setting(&self) -> pjsip_timer_setting;
    /// Number of credentials in the credential array.
    fn set_cred_count(&self, value: u32);
    fn get_cred_count(&self) -> u32;
    /// Array of credentials. These credentials will be used by all accounts,
    /// and can be used to authenticate against outbound proxies.
    /// If the credential is specific to the account, then application should
    /// set the credential in the pjsua_acc_config rather than the credential here.
    fn set_cred_info(&self, value: [pjsip_cred_info; 8usize]);
    fn get_cred_info(&self) -> [pjsip_cred_info; 8usize];

    // TODO create pjsua_callback.
    // Application callback to receive various event notifications from the library.
    // pub cb: pjsua_callback,

    /// Optional user agent string (default empty).
    /// If it's empty, no User-Agent header will be sent with outgoing requests.
    fn set_user_agent(&self, value: String);
    fn get_user_agent(&self) -> String;
    /// Specify default value of secure media transport usage.
    /// Valid values are PJMEDIA_SRTP_DISABLED, PJMEDIA_SRTP_OPTIONAL, and PJMEDIA_SRTP_MANDATORY.
    ///
    /// Note that this setting can be further customized in account configuration (pjsua_acc_config).
    ///
    /// # Default
    /// PJSUA_DEFAULT_USE_SRTP`
    fn set_use_srtp(&self, value: pjmedia_srtp_use);
    fn get_use_srtp(&self) -> pjmedia_srtp_use;
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
    fn set_srtp_secure_signaling(&self, value: i32);
    fn get_srtp_secure_signaling(&self) -> i32;
    /// This setting has been deprecated and will be ignored.
    fn set_srtp_optional_dup_offer(&self, value: bool);
    fn get_srtp_optional_dup_offer(&self) -> bool;
    /// Specify SRTP transport setting. Application can initialize it with
    /// default values using pjsua_srtp_opt_default().
    fn set_srtp_opt(&self, value: pjsua_srtp_opt);
    fn get_srtp_opt(&self) -> pjsua_srtp_opt;
    /// Disconnect other call legs when more than one 2xx responses for outgoing INVITE
    /// are received due to forking. Currently the library is not able to handle simultaneous
    /// forked media, so disconnecting the other call legs is necessary.
    ///
    /// With this setting enabled, the library will handle only one of the connected call leg,
    /// and the other connected call legs will be disconnected.
    ///
    /// # Default
    /// PJ_TRUE (only disable this setting for testing purposes).
    fn set_hangup_forked_call(&self, value: bool);
    fn get_hangup_forked_call(&self) -> bool;
}

impl SIPUa {

    pub fn new() -> Self {
        SIPUa {
            ctx: RefCell::new(pjsua_config::new())
        }
    }

    pub fn get_context(&self) -> RefMut<pjsua_config> {
        self.ctx.borrow_mut()
    }

    /// Use this function to initialize logging config.
    pub fn logging_config_default(&self, cfg: &mut pjsua_logging_config) {
        pjsua::logging_config_default(cfg);
    }

    /// Use this function to duplicate logging config.
    pub fn logging_config_dup(&self, dst: &mut pjsua_logging_config, src: &mut pjsua_logging_config) {
        pjsua::logging_config_dup(dst, src);
    }

    /// Use this function to initialize pjsua config.
    pub fn config_default (&self, cfg: &mut pjsua_config) {
        pjsua::config_default(cfg)
    }

    /// Duplicate pjsua_config.
    pub fn config_dup(&self, dst: &mut pjsua_config, src: &mut pjsua_config) {
        pjsua::config_dup(dst, src)
    }

    /// Initialize message data.
    pub fn msg_data_init(&self, msg_data: &mut pjsua_msg_data) {
        pjsua::msg_data_init(msg_data);
    }

    /// Clone message data.
    pub fn msg_data_clone(&self, rhs: &mut pjsua_msg_data) -> Option<pjsua_msg_data> {
        let ret = pjsua::msg_data_clone(rhs);

        if ret.is_null() {
            return None;
        }

        Some(unsafe {*ret} )
    }

    /// Instantiate pjsua application. Application must call this function
    /// before calling any other functions, to make sure that the underlying
    /// libraries are properly initialized. Once this function has returned success,
    /// application must call destroy() before quitting.
    pub fn create() {
        pjsua::create().expect("SIPUa::pjsua_create");
    }

    /// Initialize pjsua with the specified settings. All the settings are optional,
    /// and the default values will be used when the config is not specified.
    /// Note that create() MUST be called before calling this function.
    pub fn init(&self, log_cfg: &mut pjsua_logging_config, media_cfg: &mut pjsua_media_config) {
        pjsua::init(
            &mut self.ctx.borrow_mut(),
            log_cfg,
            media_cfg
        ).expect("SIPUa::pjsua_init");
    }

    /// Application is recommended to call this function after all initialization is done,
    /// so that the library can do additional checking set up additional
    /// Application may call this function anytime after init().
    pub fn start(&self) {
        pjsua::start().expect("SIPUa::pjsua_start");
    }

    /// Destroy pjsua. Application is recommended to perform graceful shutdown before calling
    /// this function (such as unregister the account from the SIP server,
    /// terminate presense subscription, and hangup active calls), however,
    /// this function will do all of these if it finds there are active sessions that need to be terminated.
    /// This function will approximately block for one second to wait for replies from remote.\
    ///
    /// Application may safely call this function more than once if it doesn't keep track of it's state.
    ///
    /// See also
    ///
    /// [`SIPUa::destroy2`]
    pub fn destroy(&self) {
        pjsua::destroy().expect("SIPUa::pjsua_destroy");
    }

    /// Retrieve pjsua state.
    pub fn get_state(&self) -> pjsua_state {
        pjsua::get_state()
    }

    /// Variant of destroy with additional flags.
    pub fn destroy2(&self, flags: u32) {
        pjsua::destroy2(flags).expect("SIPUa::pjsua_destroy2");
    }

    /// Poll pjsua for events, and if necessary block the caller thread for the specified
    /// maximum interval (in miliseconds).
    ///
    /// Application doesn't normally need to call this function if it has configured worker
    /// thread (thread_cnt field) in pjsua_config structure, because polling then will be done
    /// by these worker threads instead.
    pub fn handle_events(&self, msec_timeout: u32) -> i32 {
        pjsua::handle_events(msec_timeout)
    }

    /// Signal all worker threads to quit. This will only wait until internal threads are done.
    pub fn stop_worker_threads(&self) {
        pjsua::stop_worker_threads();
    }

    /// Create memory pool to be used by the application. Once application finished using the pool,
    /// it must be released with pj_pool_release().
    pub fn pool_create(&self, name: &str) -> *mut pj_pool_t {
        pjsua::pool_create(name)
    }

    /// Application can call this function at any time (after create(), of course)
    /// to change logging settings.
    pub fn reconfigure_logging(&self, c: &mut pjsua_logging_config) {
        pjsua::reconfigure_logging(c).expect("SIPUa::pjsua_reconfigure_logging");
    }

    /// Internal function to get SIP endpoint instance of pjsua, which is needed for
    /// example to register module, create transports, etc. Only valid after init() is called.
    pub fn get_pjsip_endpt(&self) -> *mut pjsip_endpoint {
        pjsua::get_pjsip_endpt()
    }

    /// Internal function to get media endpoint instance. Only valid after init() is called.
    pub fn get_pjmedia_endpt(&self) -> *mut pjmedia_endpt {
        pjsua::get_pjmedia_endpt()
    }

    /// Internal function to get PJSUA pool factory.
    /// Only valid after pjsua_create() is called.
    pub fn get_pool_factory(&self) -> *mut pj_pool_factory {
        pjsua::get_pool_factory()
    }

    /// Call this function to initialize pjsua_ip_change_param with default values.
    pub fn ip_change_param_default(&self, param: &mut pjsua_ip_change_param) {
        pjsua::ip_change_param_default(param);
    }

    /// This is a utility function to detect NAT type in front of this endpoint.
    /// Once invoked successfully, this function will complete asynchronously and
    /// report the result in on_nat_detect() callback of pjsua_callback.
    ///
    /// After NAT has been detected and the callback is called,
    /// application can get the detected NAT type by calling pjsua_get_nat_type().
    /// Application can also perform NAT detection by calling detect_nat_type()
    /// again at later time.
    ///
    /// Note that STUN must be enabled to run this function successfully.
    pub fn detect_nat_type(&self) {
        pjsua::detect_nat_type().expect("SIPUa::pjsua_detect_nat_type");
    }

    /// Get the NAT type as detected by detect_nat_type() function.
    /// This function will only return useful NAT type after pjsua_detect_nat_type()
    /// has completed successfully and on_nat_detect() callback has been called.
    pub fn get_nat_type(&self, type_: &mut pj_stun_nat_type) {
        pjsua::get_nat_type(type_).expect("SIPUa::pjsua_get_nat_type");
    }

    /// Update the STUN servers list. The init() must have been called before calling this function.
    pub fn update_stun_servers(&self, srv: Vec<String>, wait: bool) {

        let mut tmp = [pj_str_t::new(); 8];

        if srv.len() <= 8 {
            for (idx, val) in srv.iter().enumerate() {
                tmp[idx] = pj_str_t::from_string(val.clone());
            }
        }

        let count = srv.len() as u32;
        pjsua::update_stun_servers(count, &mut tmp, wait).expect("SIPUa::psjua_update_stun_servers");
    }

    /// Auxiliary function to resolve and contact each of the STUN server entries (sequentially)
    /// to find which is usable. The init() must have been called before calling this function.
    pub fn resolve_stun_servers(
        srv: Vec<String>,
        wait: bool,
        token: Option<&mut String>,
        cb: pj_stun_resolve_cb
    ) {
        let mut tmp = [pj_str_t::new(); 8];

        if srv.len() <= 8 {
            for (idx, val) in srv.iter().enumerate() {
                tmp[idx] = pj_str_t::from_string(val.clone());
            }
        }

        let count = srv.len() as u32;
        pjsua::resolve_stun_servers(count, &mut tmp, wait, token, cb).expect("SIPUa::pjsua_resolve_stun_servers");
    }

    /// Cancel pending STUN resolution which match the specified token.
    pub fn cancel_stun_resolution(token: Option<&mut i32>, notify_cb: bool) {
        pjsua::cancel_stun_resolution(token, notify_cb).expect("SIPUa::pjsua_cancel_stun_resolution");
    }

    /// This is a utility function to verify that valid SIP url is given.
    /// If the URL is a valid SIP/SIPS scheme, TRUE will be returned.
    pub fn verify_sip_url(&self, url: String) -> bool {
        if let Err(_) = pjsua::verify_sip_url(url) {
            false
        } else {
            true
        }
    }

    /// This is a utility function to verify that valid URI is given.
    /// Unlike verify_sip_url(), this function will return TRUE if tel: URI is given.
    pub fn verify_url(&self, url: String) -> bool {
        if let Err(_) = pjsua::verify_url(url) {
            false
        } else {
            true
        }
    }

    /// Schedule a timer entry. Note that the timer callback may be executed by different thread,
    /// depending on whether worker thread is enabled or not.
    pub fn schedule_timer(&self, entry: &mut pj_timer_entry, delay: &mut pj_time_val) {
        pjsua::schedule_timer(entry, delay).expect("SIPUa::pjsua_schedule_timer");
    }

    // pj_status_t 	pjsua_schedule_timer2 (void(*cb)(void *user_data), void *user_data, unsigned msec_delay)

    /// Cancel the previously scheduled timer.
    pub fn cancel_timer(&self, entry: &mut pj_timer_entry) {
        pjsua::cancel_timer(entry);
    }

    /// This is a utility function to display error message for the specified error code.
    /// The error message will be sent to the log.
    pub fn perror(&self, sender: String, title: String, status: pj_status_t) {
        pjsua::perror(sender.as_str(), title.as_str(), status);
    }

    /// This is a utility function to dump the stack states to log,
    /// using verbosity level 3.
    pub fn dump(&self, detail: bool) {
        pjsua::dump(detail);
    }

    /// Inform the stack that IP address change event was detected. The stack will:
    ///
    /// - `Restart` the listener (this step is configurable via pjsua_ip_change_param.restart_listener).
    /// - `Shutdown` the transport used by account registration
    ///              (this step is configurable via pjsua_acc_config.ip_change_cfg.shutdown_tp).
    /// - `Update` contact URI by sending re-Registration (this step is configurable via
    ///            pjsua_acc_config.allow_contact_rewrite and pjsua_acc_config.contact_rewrite_method)
    /// - `Hangup` active calls (this step is configurable via a\ pjsua_acc_config.ip_change_cfg.hangup_calls)
    ///            or continue the call by sending re-INVITE (configurable via pjsua_acc_config.ip_change_cfg.reinvite_flags).
    pub fn handle_ip_change(&self, param: &mut pjsua_ip_change_param) {
        pjsua::handle_ip_change(param).expect("SIPUa::pjsua_handle_ip_change");
    }
}


impl SIPUaExt for SIPUa {

    fn set_max_calls (&self, max_calls: u32) {
        self.ctx.borrow_mut().max_calls = max_calls;
    }

    fn get_max_calls (&self) -> u32 {
        self.ctx.borrow().max_calls
    }

    fn set_thread_cnt(&self, value: u32) {
        self.ctx.borrow_mut().thread_cnt = value;
    }

    fn get_thread_cnt(&self) -> u32 {
        self.ctx.borrow().thread_cnt
    }

    fn set_nameserver_count(&self, value: u32) {
        self.ctx.borrow_mut().nameserver_count = value;
    }

    fn get_nameserver_count(&self) -> u32 {
        self.ctx.borrow().nameserver_count
    }

    fn set_nameserver(&self, nameserver: [String; 4usize]) {

        let mut tmp = [pj_str_t::new(); 4usize];

        for (idx, value) in nameserver.iter().enumerate() {
            tmp[idx] = pj_str_t::from_string(value.clone());
        }

        self.ctx.borrow_mut().nameserver = tmp;
    }

    fn get_nameserver(&self) -> [String; 4usize] {
        // something not good to see.
        let mut tmp= [String::new(), String::new(), String::new(), String::new()];
        let nameserver = self.ctx.borrow().nameserver;

        for idx in 0..4usize {
            tmp[idx] = nameserver[idx].to_string();
        }

        tmp
    }

    fn set_force_lr(&self, value: bool) {
        self.ctx.borrow_mut().force_lr = pjdefault::boolean_to_pjbool(value);
    }

    fn get_force_lr(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().force_lr)
    }

    fn set_outbound_proxy_cnt(&self, value: u32) {
        self.ctx.borrow_mut().outbound_proxy_cnt = value;
    }

    fn get_outbound_proxy_cnt(&self) -> u32 {
        self.ctx.borrow().outbound_proxy_cnt
    }

    fn set_outbound_proxy(&self, value: [String; 4usize]) {
        let mut tmp = [pj_str_t::new(); 4usize];

        for (idx, value) in value.iter().enumerate() {
            tmp[idx] = pj_str_t::from_string(value.clone());
        }

        self.ctx.borrow_mut().outbound_proxy = tmp;
    }

    fn get_outbound_proxy(&self) -> [String; 4usize] {
        // something not good to see
        let mut tmp= [String::new(), String::new(), String::new(), String::new()];
        let outbound_proxy = self.ctx.borrow().outbound_proxy;

        for idx in 0..4usize {
            tmp[idx] = outbound_proxy[idx].to_string();
        }

        tmp
    }

    fn set_stun_domain(&self, value: String) {
        self.ctx.borrow_mut().stun_domain = pj_str_t::from_string(value);
    }

    fn get_stun_domain(&self) -> String {
        self.ctx.borrow().stun_domain.to_string()
    }

    fn set_stun_host(&self, value: String) {
        self.ctx.borrow_mut().stun_host = pj_str_t::from_string(value);
    }

    fn get_stun_host(&self) -> String {
        self.ctx.borrow().stun_host.to_string()
    }

    fn set_stun_srv_cnt(&self, value: u32) {
        self.ctx.borrow_mut().stun_srv_cnt = value;
    }

    fn get_stun_srv_cnt(&self) -> u32 {
        self.ctx.borrow().stun_srv_cnt
    }

    fn set_stun_srv(&self, value: [String; 8usize]) {
        let mut tmp = [pj_str_t::new(); 8usize];

        for (idx, value) in value.iter().enumerate() {
            tmp[idx] = pj_str_t::from_string(value.clone());
        }

        self.ctx.borrow_mut().stun_srv = tmp;
    }

    fn get_stun_srv(&self) -> [String; 8usize] {
        // something not good to see.
        let mut tmp= [
            String::new(), String::new(), String::new(), String::new(),
            String::new(), String::new(), String::new(), String::new(),
        ];

        let stun_srv = self.ctx.borrow().stun_srv;

        for idx in 0..8usize {
            tmp[idx] = stun_srv[idx].to_string();
        }

        tmp
    }

    fn set_stun_try_ipv6(&self, value: bool) {
        self.ctx.borrow_mut().stun_try_ipv6 = pjdefault::boolean_to_pjbool(value);
    }

    fn get_stun_try_ipv6(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().stun_try_ipv6)
    }

    fn set_stun_ignore_failure(&self, value: bool) {
        self.ctx.borrow_mut().stun_ignore_failure = pjdefault::boolean_to_pjbool(value);
    }

    fn get_stun_ignore_failure(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().stun_ignore_failure)
    }

    fn set_stun_map_use_stun2(&self, value: bool) {
        self.ctx.borrow_mut().stun_map_use_stun2 = pjdefault::boolean_to_pjbool(value);
    }

    fn get_stun_map_use_stun2(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().stun_map_use_stun2)
    }

    fn set_nat_type_in_sdp(&self, value: i32) {
        self.ctx.borrow_mut().nat_type_in_sdp = value;
    }

    fn get_nat_type_in_sdp(&self) -> i32 {
        self.ctx.borrow().nat_type_in_sdp
    }

    fn set_require_100rel(&self, value: pjsua_100rel_use) {
        self.ctx.borrow_mut().require_100rel = value;
    }

    fn get_require_100rel(&self) -> pjsua_100rel_use {
        self.ctx.borrow().require_100rel
    }

    fn set_use_timer(&self, value: pjsua_sip_timer_use) {
        self.ctx.borrow_mut().use_timer = value;
    }

    fn get_use_timer(&self) -> pjsua_sip_timer_use {
        self.ctx.borrow().use_timer
    }

    fn set_enable_unsolicited_mwi(&self, value: bool) {
        self.ctx.borrow_mut().enable_unsolicited_mwi = pjdefault::boolean_to_pjbool(value);
    }

    fn get_enable_unsolicited_mwi(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().enable_unsolicited_mwi)
    }

    fn set_timer_setting(&self, value: pjsip_timer_setting) {
        self.ctx.borrow_mut().timer_setting = value;
    }

    fn get_timer_setting(&self) -> pjsip_timer_setting {
        self.ctx.borrow().timer_setting
    }

    fn set_cred_count(&self, value: u32) {
        self.ctx.borrow_mut().cred_count = value;
    }

    fn get_cred_count(&self) -> u32 {
        self.ctx.borrow().cred_count
    }

    fn set_cred_info(&self, value: [pjsip_cred_info; 8usize]) {
        self.ctx.borrow_mut().cred_info = value;
    }

    fn get_cred_info(&self) -> [pjsip_cred_info; 8usize] {
        self.ctx.borrow().cred_info
    }

    fn set_user_agent(&self, value: String) {
        self.ctx.borrow_mut().user_agent = pj_str_t::from_string(value);
    }

    fn get_user_agent(&self) -> String {
        self.ctx.borrow().user_agent.to_string()
    }

    fn set_use_srtp(&self, value: pjmedia_srtp_use) {
        self.ctx.borrow_mut().use_srtp = value;
    }

    fn get_use_srtp(&self) -> pjmedia_srtp_use {
        self.ctx.borrow().use_srtp
    }

    fn set_srtp_secure_signaling(&self, value: i32) {
        self.ctx.borrow_mut().srtp_secure_signaling = value;
    }

    fn get_srtp_secure_signaling(&self) -> i32 {
        self.ctx.borrow().srtp_secure_signaling
    }

    fn set_srtp_optional_dup_offer(&self, value: bool) {
        self.ctx.borrow_mut().srtp_optional_dup_offer = pjdefault::boolean_to_pjbool(value)
    }

    fn get_srtp_optional_dup_offer(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().srtp_optional_dup_offer)
    }

    fn set_srtp_opt(&self, value: pjsua_srtp_opt) {
        self.ctx.borrow_mut().srtp_opt = value;
    }

    fn get_srtp_opt(&self) -> pjsua_srtp_opt {
        self.ctx.borrow().srtp_opt
    }

    fn set_hangup_forked_call(&self, value: bool) {
        self.ctx.borrow_mut().hangup_forked_call = pjdefault::boolean_to_pjbool(value);
    }

    fn get_hangup_forked_call(&self) -> bool {
        pjdefault::check_boolean(self.ctx.borrow().hangup_forked_call)
    }
}

