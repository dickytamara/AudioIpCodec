
use std::cell::RefCell;

use super::pj_sys::*;
use super::pjsip_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::{AutoCreate, FromString};
use super::pjsua;


// high layer API
// this API provide error message and
// memory safety when interact with pjsua basic api

pub struct SIPUa {
    // give mutable interior for ensure ctx not moved to anywhere.
    ctx: RefCell<pjsua_config>
}

impl SIPUa {

    pub fn new() -> Self {
        SIPUa {
            ctx: RefCell::new(pjsua_config::new())
        }
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
    pub fn init(&mut self, log_cfg: &mut pjsua_logging_config, media_cfg: &mut pjsua_media_config) {
        pjsua::init(
            &mut self.ctx.try_borrow_mut().expect("SIPUa::pjsua_init ctx value"),
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
    /// Application.may safely call this function more than once if it doesn't keep track of it's state.
    ///
    /// See also
    /// ```code
    /// destroy2()
    /// ```
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
    /// - [Restart] the listener (this step is configurable via pjsua_ip_change_param.restart_listener).
    /// - [Shutdown] the transport used by account registration
    ///              (this step is configurable via pjsua_acc_config.ip_change_cfg.shutdown_tp).
    /// - [Update] contact URI by sending re-Registration (this step is configurable via
    ///            pjsua_acc_config.allow_contact_rewrite and pjsua_acc_config.contact_rewrite_method)
    /// - [Hangup] active calls (this step is configurable via a\ pjsua_acc_config.ip_change_cfg.hangup_calls)
    ///            or continue the call by sending re-INVITE (configurable via pjsua_acc_config.ip_change_cfg.reinvite_flags).
    pub fn handle_ip_change(&self, param: &mut pjsua_ip_change_param) {
        pjsua::handle_ip_change(param).expect("SIPUa::pjsua_handle_ip_change");
    }
}

