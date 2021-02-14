#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]

// default
use super::pjdefault::AutoCreate;
use super::pjsip::PjsipModuleCallback;
use super::pjsua::PjsuaCallback;
use super::pjsua_sys::*;
use mut_static::MutStatic;
use std::ffi::CString;
use std::mem;
use std::ops::Drop;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

pub type SIPAccount = pjsua_acc_config;
pub type SIPBuddy = pjsua_buddy_config;

pub struct SIPAudio {}
pub struct SIPIMessages {}


pub struct SIPMedia {
    slot: i32,
    cnt: i32,
    port: pjmedia_port
}


impl SIPMedia {
    pub fn new () -> SIPMedia {
      SIPMedia {
        slot: -1,
        cnt: -1,
        port: pjmedia_port::new()
      }
    }
}

pub struct SIPPressence {}


#[derive(Copy, Clone)]
pub struct SIPCall {
    timer: pj_timer_entry,
    ringback_on: bool,
    ring_on: bool,
}

impl SIPCall {
    pub fn new() -> SIPCall {
        SIPCall {
            timer: pj_timer_entry::new(),
            ringback_on: false,
            ring_on: false,
        }
    }
}

trait Account {}

trait Audio {}

trait IMessages {}

trait Media {

}

trait Pressence {}

trait Call {}

// pjsua_call_dump
trait Dump {}

static mut SIP_CORE: Option<SIPCore> = None;
static mut CURRENT_CALL: Option<pjsua_call_id> = None;

pub struct SIPCore {
    pool: *mut pj_pool_t,
    app_config: pjsua_config,
    log_config: pjsua_logging_config,
    media_config: pjsua_media_config,
    udp_config: pjsua_transport_config,
    rtp_config: pjsua_transport_config,
    account: SIPAccount, // for now just set to 1 account
    buddy_list: Vec<SIPBuddy>,
    call_data: [SIPCall; 32],
    ringback: SIPMedia,
    ring: SIPMedia,
    default_handler: pjsip_module,
    redir_op: pjsip_redirect_op,
    wav_id: pjsua_player_id,
    rec_id: pjsua_recorder_id,
    wav_port: pjsua_conf_port_id,
    rec_port: pjsua_conf_port_id,
    input_level: f32,
    output_level: f32,
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    auto_play_hangup: bool,
    duration: u32,
    current_call: i32,
    aud_cnt: u32,
    auto_answer: u32
}

impl SIPCore {
    pub fn new() -> SIPCore {
        // create default data
        let ctx: *mut pj_pool_t;
        unsafe {
            pjsua_create();
            let pool_name = CString::new("ipcodec").expect("pool_name fail.");
            ctx = pjsua_pool_create(pool_name.as_ptr(), 1000, 1000);
        }

        let mut udp = pjsua_transport_config::new();
        let mut rtp = pjsua_transport_config::new();
        udp.port = 5060;
        rtp.port = 4000;

        SIPCore {
            pool: ctx,
            app_config: pjsua_config::new(),
            log_config: pjsua_logging_config::new(),
            media_config: pjsua_media_config::new(),
            udp_config: udp,
            rtp_config: rtp,
            account: SIPAccount::new(),
            buddy_list: Vec::<SIPBuddy>::new(),
            call_data: [SIPCall::new(); 32],
            ringback: SIPMedia::new(),
            ring: SIPMedia::new(),
            default_handler: pjsip_module::new(),
            redir_op: pjsip_redirect_op_PJSIP_REDIRECT_ACCEPT_REPLACE,
            wav_id: PJSUA_INVALID_ID,
            rec_id: PJSUA_INVALID_ID,
            wav_port: PJSUA_INVALID_ID,
            rec_port: PJSUA_INVALID_ID,
            input_level: 1.0,
            output_level: 1.0,
            input_dev: PJSUA_INVALID_ID,
            output_dev: PJSUA_INVALID_ID,
            input_latency: 100,
            output_latency: 140,
            auto_play_hangup: false,
            duration: 0,
            current_call: -1,
            aud_cnt: 0,
            auto_answer: 0
        }
    }

    pub fn start(&mut self) {
        unsafe {
            self.app_config.cb.on_call_state = Some(SIPCore::on_call_state);
            self.app_config.cb.on_stream_destroyed = Some(SIPCore::on_stream_destroyed);
            self.app_config.cb.on_call_media_state = Some(SIPCore::on_call_media_state);
            self.app_config.cb.on_incoming_call = Some(SIPCore::on_incoming_call);
            self.app_config.cb.on_dtmf_digit2 = Some(SIPCore::on_dtmf_digit2);
            self.app_config.cb.on_call_redirected = Some(SIPCore::on_call_redirected);
            self.app_config.cb.on_reg_state = Some(SIPCore::on_reg_state);
            self.app_config.cb.on_incoming_subscribe = Some(SIPCore::on_incoming_subscribe);
            self.app_config.cb.on_buddy_state = Some(SIPCore::on_buddy_state);
            self.app_config.cb.on_buddy_evsub_state = Some(SIPCore::on_buddy_evsub_state);
            self.app_config.cb.on_pager = Some(SIPCore::on_pager);
            self.app_config.cb.on_typing = Some(SIPCore::on_typing);
            self.app_config.cb.on_call_transfer_status = Some(SIPCore::on_call_transfer_status);
            self.app_config.cb.on_call_replaced = Some(SIPCore::on_call_replaced);
            self.app_config.cb.on_nat_detect = Some(SIPCore::on_nat_detect);
            self.app_config.cb.on_mwi_info = Some(SIPCore::on_mwi_info);
            self.app_config.cb.on_transport_state = Some(SIPCore::on_transport_state);
            self.app_config.cb.on_ice_transport_error = Some(SIPCore::on_ice_transport_error);
            self.app_config.cb.on_snd_dev_operation = Some(SIPCore::on_snd_dev_operation);
            self.app_config.cb.on_call_media_event = Some(SIPCore::on_call_media_event);
            self.app_config.cb.on_ip_change_progress = Some(SIPCore::on_ip_change_progress);

            // init pjsua
            pjsua_init(
                &mut self.app_config as *mut _,
                &mut self.log_config as *mut _,
                &mut self.media_config as *mut _,
            );
            // pjsip endpoint for unhadled error
            self.default_handler.on_rx_request = Some(SIPCore::on_rx_request);
            pjsip_endpt_register_module(
                pjsua_get_pjsip_endpt(),
                &mut self.default_handler as *mut _,
            );
        }
    }

    pub fn ringback_start(&self, call_id: pjsua_call_id) {}

    // ring stop procedure
    pub fn ring_stop(&self, call_id: &pjsua_call_id) {}

    pub fn ring_start(&self, call_id: pjsua_call_id) {}

    pub fn find_next_call(&self) {}

    pub fn on_call_generic_media_state(&self) {}

    pub fn on_call_audio_state(&self, ci: &pjsua_call_info, mi: u32, has_error: &mut bool) {
        self.ring_stop(&ci.id);
    }

    pub fn callback_on_call_state(&mut self, call_id: pjsua_call_id, e: *mut pjsip_event) {
        unsafe {
            let mut call_info: pjsua_call_info = pjsua_call_info::new();

            pjsua_call_get_info(call_id, &mut call_info as *mut _);

            if call_info.state == pjsip_inv_state_PJSIP_INV_STATE_DISCONNECTED {
                // todo
                self.ring_stop(&call_id);
                let cd: *mut SIPCall = &mut self.call_data[call_id as usize] as *mut _;
                let endpt = pjsua_get_pjsip_endpt();

                (*cd).timer.id = PJSUA_INVALID_ID;
                pjsip_endpt_cancel_timer(endpt, &mut (*cd).timer as *mut _);

                if self.auto_play_hangup {
                    pjsua_player_set_pos(self.wav_id, 0);
                }

                if call_id == self.current_call {
                    self.find_next_call();
                }

                println!("Call disconnected.");
            } else {
                if self.duration == 0x7FFFFFFF
                    && call_info.state == pjsip_inv_state_PJSIP_INV_STATE_CONFIRMED
                {
                    let cd: *mut SIPCall = &mut self.call_data[call_id as usize] as *mut _;
                    let endpt = pjsua_get_pjsip_endpt();
                    let mut delay: pj_time_val = pj_time_val::new();

                    (*cd).timer.id = call_id;
                    delay.sec = self.duration as i64;
                    delay.msec = 0;
                    pjsip_endpt_schedule_timer_dbg(
                        endpt,
                        &mut (*cd).timer as *mut _,
                        &delay as *const _,
                        &mut mem::zeroed() as *mut _,
                        0,
                    );
                }

                if call_info.state == pjsip_inv_state_PJSIP_INV_STATE_EARLY {
                    println!("Call state changed.");
                }

                if self.current_call == PJSUA_INVALID_ID {
                    self.current_call = call_id;
                }
            }
        }
    }

    pub fn callback_on_call_media_state(&self, call_id: pjsua_call_id) {
        unsafe {
            let mut call_info: pjsua_call_info = pjsua_call_info::new();
            //let mi: u32 = 0;
            let mut has_error = false;

            pjsua_call_get_info(call_id, &mut call_info as *mut _);

            for mi in 0..call_info.media_cnt {
                self.on_call_generic_media_state();
                match call_info.media[mi as usize].type_ {
                    pjmedia_type_PJMEDIA_TYPE_AUDIO => {
                        self.on_call_audio_state(&call_info, mi, &mut has_error);
                    }
                    _ => println!("unsuported pjmedia type"),
                }
            }

            if has_error {
                let reason = CString::new("Media Failed").expect("cant create str");
                pjsua_call_hangup(
                    call_id,
                    500,
                    &pj_str(reason.as_ptr() as *mut _) as *const _,
                    ptr::null(),
                );
            }
        }
    }

    pub fn callback_on_incomming_call(
        &mut self,
        acc_id: pjsua_acc_id,
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
    ) {
        unsafe {
            let mut call_info: pjsua_call_info = pjsua_call_info::new();

            pjsua_call_get_info(call_id, &mut call_info as *mut _);

            self.current_call = call_id;

            self.ring_start(call_id);

            let mut opt: pjsua_call_setting = pjsua_call_setting::new();
            pjsua_call_setting_default(&mut opt as *mut _);
            opt.aud_cnt = self.aud_cnt;

            pjsua_call_answer2(call_id, &opt as *const _, self.auto_answer,
              &mut mem::zeroed() as *const _,
              &mut mem::zeroed() as *const _);
        }
    }

    pub fn callback_on_dtmf_digit2(&self, call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        unsafe {
              let mut dtmf: &str = "None";
              match (*info).method {
                   pjsua_dtmf_method_PJSUA_DTMF_METHOD_RFC2833 => {
                      dtmf = "RFC2833";
                   },
                   pjsua_dtmf_method_PJSUA_DTMF_METHOD_SIP_INFO => {
                      dtmf = "SIP INFO";  
                   },
                   _ => println!("Unknown dtmf method")
              }

              println!("Incomming DTMF on call using method {}", dtmf);
        }
    } 


    pub fn callback_on_call_redirected( 
        &self, 
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        println!("Call {} is being redirected", call_id);
        self.redir_op
    }
 
    pub fn callback_on_reg_state(&self, acc_id: pjsua_acc_id) {
        println!("reg_state {}", acc_id);
    }


    pub fn callback_on_incoming_subscribe( &self,
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data,
    ) {
        // Todo
    }

    pub fn callback_on_buddy_state(&self, buddy_id: pjsua_buddy_id) {
        unsafe { 
            let mut info: pjsua_buddy_info = pjsua_buddy_info::new();
            pjsua_buddy_get_info(buddy_id, &mut info as *mut _);
        }
    } 

    pub fn callback_on_call_transfer_status(
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
        unsafe {

        } 
    }

    pub fn callback_on_ip_change_progress(
        &self,
        op: pjsua_ip_change_op,
        status: pj_status_t,
        info: *const pjsua_ip_change_op_info,
    ) {
        unsafe {
            let mut acc_info: pjsua_acc_info = pjsua_acc_info::new();
            let mut tp_info: pjsua_transport_info = pjsua_transport_info::new();

            if status == pj_constants__PJ_SUCCESS as pj_status_t {
                match op {
                    pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_RESTART_LIS => {
                        pjsua_transport_get_info(
                            (*info).lis_restart.transport_id,
                            &mut tp_info as *mut _,
                        );
                        println!("restart transport.");
                    }
                    pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("transport shutdown for account.");
                    }
                    pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("update contact for account.");
                    }
                    pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("hangup call for account.");
                    }
                    pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS => {
                        pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id, &mut acc_info as *mut _);
                        println!("reinvite call for account.");
                    }
                    pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_COMPLETED => {
                        println!("done");
                    }

                    _ => println!("warn validate c code."),
                }
            } else {
                println!("IP change progress fail.");
            }
        }
    }
}

// SIPUserAgent
pub struct SIPUserAgent {
    /// hold internal pjsua data
    x: i32, // test only
}

//type SIPConfig = pjsua_config;

pub const PJSUA_INVALID_ID: i32 = -1;

trait SIPUserAgentInternal {
    fn get_config(&mut self) -> &mut pjsua_config;
    fn get_log_config(&mut self) -> &mut pjsua_logging_config;
    fn get_media_config(&mut self) -> &mut pjsua_media_config;
}

impl SIPUserAgent {
    // create sip user sip user agent with default ivalue
    pub fn new() -> SIPUserAgent {
        unsafe {
            SIP_CORE = Some(SIPCore::new());
            CURRENT_CALL = Some(PJSUA_INVALID_ID);
        }

        SIPUserAgent { x: 0 }
    }

    pub fn start(self) {
        unsafe {
            match SIP_CORE {
                Some(ref mut sipcore) => sipcore.start(),
                _ => panic!(""),
            }
        }
    }
}

//binding clike code patern with destructor
impl Drop for SIPCore {
    fn drop(&mut self) {
        unsafe {
            pj_pool_safe_release(&mut self.pool as *mut _);
            pjsua_destroy();
        }
    }
}

fn simple_registrar(rdata: *mut pjsip_rx_data) {
    unsafe {
        let tdata: *const pjsip_tx_data = ptr::null();
        let str_null: *const pj_str_t = ptr::null();
        let status: pj_status_t;
        let mut cnt: c_uint = 0;

        status = pjsip_endpt_create_response(
            pjsua_get_pjsip_endpt(),
            rdata as *const _,
            200,
            str_null as *const _,
            tdata as *mut _,
        );
        if status != pj_constants__PJ_SUCCESS as i32 {
            return;
        }
        #[allow(unused_assignments)]
        let void: *const c_void = ptr::null();

        let exp: *const pjsip_expires_hdr =
            pjsip_msg_find_hdr((*rdata).msg_info.msg, pjsip_hdr_e_PJSIP_H_EXPIRES, void)
                as *const _;

        let llist: pjsip_hdr = (*(*rdata).msg_info.msg).hdr;
        let mut h: *mut pjsip_hdr = (*(*rdata).msg_info.msg).hdr.next;

        while h != llist.next {
            if (*h as pjsip_hdr).type_ == (pjsip_hdr_e_PJSIP_H_CONTACT as pjsip_hdr_e) {
                let c: *const pjsip_contact_hdr = h as *const pjsip_contact_hdr;
                let mut e: c_uint = (*c).expires;

                if e != 0xffffffff {
                    if !exp.is_null() {
                        e = (*exp).ivalue;
                    } else {
                        e = 3600;
                    }
                }

                if e > 0 {
                    let nc: *mut pjsip_contact_hdr =
                        pjsip_hdr_clone((*tdata).pool, h as *const _) as *mut pjsip_contact_hdr;

                    (*nc).expires = e;
                    pj_list_insert_before((*tdata).msg as *mut _, nc as *mut _);
                    cnt = cnt + 1;
                }
                h = (*h).next;
            }
        }

        // todo review c code for this. it's c clasic problem
        let srv: *mut pjsip_generic_string_hdr =
            pjsip_generic_string_hdr_create((*tdata).pool, str_null, str_null);
        // create server name
        let tmp: CString = CString::new("Server").expect("cant create Server string");
        (*srv).name = pj_str(tmp.as_ptr() as *mut c_char);
        // create add description
        let tmp: CString =
            CString::new("IpCodec simple registrar").expect("cant create simple registrar");
        (*srv).hvalue = pj_str(tmp.as_ptr() as *mut c_char);

        pj_list_insert_before((*tdata).msg as *mut _, srv as *mut _);
        let cb: pjsip_send_callback = None;
        pjsip_endpt_send_response2(
            pjsua_get_pjsip_endpt(),
            rdata,
            tdata as *mut _,
            void as *mut _,
            None,
        );
    }
}

// handle for callback PjsipModule
impl PjsipModuleCallback for SIPCore {
    unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_status_t {
        // base rx request handle undefined state.
        let tdata: *const pjsip_tx_data = ptr::null();
        let status_code: pjsip_status_code;
        let status: pj_status_t;

        let mut rdata = *rdata;
        let msg = *rdata.msg_info.msg;
        let mut method = msg.line.req.method;
        // let msg_info = method.msg_info;
        if pjsip_method_cmp(&mut method as *const _, &pjsip_ack_method as *const _) == 0 {
            return pj_constants__PJ_TRUE as pj_status_t;
        }

        if pjsip_method_cmp(&mut method as *const _, &pjsip_register_method as *const _) == 0 {
            // call simple registrar pjsip_tx_data
            simple_registrar(&mut rdata as *mut _);
            return pj_constants__PJ_TRUE as pj_status_t;
        }

        if pjsip_method_cmp(&mut method as *const _, &pjsip_notify_method as *const _) == 0 {
            status_code = pjsip_status_code_PJSIP_SC_BAD_REQUEST as pjsip_status_code;
        } else {
            status_code = pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED;
        }

        let null_ptr: *const pj_str_t = ptr::null();
        status = pjsip_endpt_create_response(
            pjsua_get_pjsip_endpt(),
            &mut rdata as *const _,
            status_code as c_int,
            null_ptr,
            tdata as *mut *mut _,
        );

        if status != (pj_constants__PJ_SUCCESS as pj_status_t) {
            return pj_constants__PJ_TRUE as pj_status_t;
        }

        if status_code == pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED {
            #[allow(unused_assignments)]
            let mut cap_hdr: *const pjsip_hdr = ptr::null();

            cap_hdr = pjsip_endpt_get_capability(
                pjsua_get_pjsip_endpt(),
                pjsip_hdr_e_PJSIP_H_ALLOW as i32,
                null_ptr,
            );

            if !cap_hdr.is_null() {
                //pjsip_msg_add_hdr(msg, pjsip_hdr_clone(tdata.pool, cap_hdr));
                pj_list_insert_before(
                    (*tdata).msg as *mut _,
                    pjsip_hdr_clone((*tdata).pool as *mut _, cap_hdr as *const _),
                );
            }
        }

        // add user-agent header
        #[allow(unused_assignments)]
        let mut h: *const pjsip_hdr = ptr::null();

        let ua_str = CString::new("User-Agent").expect("cant create str User-Agent.");
        let mut ua: pj_str_t = pj_str_t {
            ptr: ua_str.as_ptr() as *mut _,
            slen: 10,
        };
        let agent_str = CString::new("AudioIP 0.1").expect("cant create str AudioIP 0.1");
        let mut agent = pj_str_t {
            ptr: agent_str.as_ptr() as *mut _,
            slen: 11,
        };

        h = pjsip_generic_string_hdr_create(
            (*tdata).pool as *mut _,
            &mut ua as _,
            &mut agent as *mut _,
        ) as *mut _;

        pj_list_insert_before((*tdata).msg as *mut _, h as *mut _);

        pj_constants__PJ_TRUE as pj_status_t
    }
}

impl PjsuaCallback for SIPCore {
    // Call status event
    unsafe extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {
        // call info data
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_state(call_id, e);
            }
            _ => panic!("Panic OnCallState"),
        }
    }

    // Stream Destroyed;
    unsafe extern "C" fn on_stream_destroyed(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
    ) {
        println!("Call stream destroyed");
    }

    // Call media satate
    unsafe extern "C" fn on_call_media_state(call_id: pjsua_call_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_media_state(call_id);
            }
            _ => panic!("Panic OnCallMediaState"),
        }
    }

    // DTMF Digit2
    unsafe extern "C" fn on_dtmf_digit2(call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        // todo here
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_dtmf_digit2(call_id, info);
            },
            _ => panic!("Panic OnDtmfDigit2")
        }
    }

    // Call Redirected
    unsafe extern "C" fn on_call_redirected(
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        let result: pjsip_redirect_op;
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_redirected(call_id, target, e)
            },
            _ => panic!("Panic OnCallRedirected")
        }
    }

    // REG state
    unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_reg_state(acc_id);
            },
            _ => panic!("Panic OnRegState")
        }
    }

    // Incomming Subscribe
    unsafe extern "C" fn on_incoming_subscribe(
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_incoming_subscribe(acc_id, srv_pres,
                  buddy_id, from, rdata, code, reason, msg_data);
            },
            _ => panic!("Panic OnIncomingSubscribe")
        }
    }

    // Buddy State
    unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_buddy_state(buddy_id);
            },
            _ => panic!("Panic OnBuddyState")
        }
    }

    // Buddy evsub state
    unsafe extern "C" fn on_buddy_evsub_state(
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
        // todo here
    }

    // Pager
    unsafe extern "C" fn on_pager(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
    ) {
        // todo here
    }

    // Typing event
    unsafe extern "C" fn on_typing(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
        // todo here
    }

    // Call transfer status
    unsafe extern "C" fn on_call_transfer_status(
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
        // todo here
    }

    // Call replaced
    unsafe extern "C" fn on_call_replaced(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id) {
        // todo here
    }

    // NAT detect
    unsafe extern "C" fn on_nat_detect(res: *const pj_stun_nat_detect_result) {
        // todo here
    }

    // MWI info
    unsafe extern "C" fn on_mwi_info(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {
        // todo here
    }

    // Transport state
    unsafe extern "C" fn on_transport_state(
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
        // todo here
    }

    // ICE transport error
    unsafe extern "C" fn on_ice_transport_error(
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
        // todo here
    }

    // Sound device operation
    unsafe extern "C" fn on_snd_dev_operation(operation: c_int) -> pj_status_t {
        // todo here
        0
    }

    // Call media event
    unsafe extern "C" fn on_call_media_event(
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event,
    ) {
        // todo here
    }

    // IP change progress
    unsafe extern "C" fn on_ip_change_progress(
        op: pjsua_ip_change_op,
        status: pj_status_t,
        info: *const pjsua_ip_change_op_info,
    ) {
    
        match SIP_CORE {
          Some(ref mut sipcore) => {
            sipcore.callback_on_ip_change_progress(op, status, info);
          },
          _ => panic!("Panic OnIpChangeProgress")
        }

    }
}
