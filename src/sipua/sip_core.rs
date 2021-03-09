
use super::pj_sys::*;
use super::pjsip_sys::*;
use super::pjmedia_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;

use super::pjsip::PjsipModuleCallback;
use super::pjsua::*;
use super::pjdefault::*;

use super::sip_account::*;
use super::sip_buddy::*;
use super::sip_calls::*;
use super::sip_media::*;
use super::sip_presence::*;
use super::sip_tones::*;
use super::sip_transport::*;
use super::sip_wav::*;

use std::ptr;
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void, c_uint, c_char};


pub static mut SIP_CORE: Option<SIPCore> = None;
pub static mut CURRENT_CALL: Option<pjsua_call_id> = None;

pub struct SIPCore {
    pool: *mut pj_pool_t,
    app_config: pjsua_config,
    log_config: pjsua_logging_config,
    pub media_config: SIPMedia,
    no_udp: bool,
    no_tcp: bool,
    use_ipv6: bool,
    transports: SIPTransports,
    accounts: SIPAccounts,
    presence: SIPPresence,
    calls: SIPCalls,
    tones: Vec<SIPTones>,
    ringback: SIPRingback,
    ringtone: SIPRingtone,
    wav_player: Option<SIPWavPlayer>,
    wav_recorder: Option<SIPWavRecorder>,
    default_handler: pjsip_module,
    redir_op: pjsip_redirect_op,
    // input_level: f32,
    // output_level: f32,
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    auto_play_hangup: bool,
    duration: u32,
    current_call: i32,
    aud_cnt: u32,
    auto_answer: u32,
    invite_event: SIPCoreInviteEvents
}
// struct to hold invite event function
struct SIPCoreInviteEvents {
    calling: Box<dyn FnMut()>,
    incoming: Box<dyn Fn()>,
    early: Box<dyn Fn()>,
    connecting: Box<dyn Fn()>,
    confirmed: Box<dyn Fn()>,
    disconnected: Box<dyn Fn()>,
    null: Box<dyn Fn()>,
    failure: Box<dyn Fn()>,

}

impl SIPCoreInviteEvents {
    fn new() -> SIPCoreInviteEvents {
        SIPCoreInviteEvents {
            calling: Box::new(|| {}),
            incoming: Box::new(|| {}),
            early: Box::new(|| {}),
            connecting: Box::new(|| {}),
            confirmed: Box::new(|| {}),
            disconnected: Box::new(|| {}),
            null: Box::new(|| {}),
            failure: Box::new(|| {}),
        }
    }
}

pub trait SIPCoreInviteExt {
    fn connect_invite_calling<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_incoming<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_early<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_connecting<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_confirmed<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_disconnected<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_null<F: Fn() + 'static> (&mut self, f: F);
    fn connect_invite_failure<F: Fn() + 'static> (&mut self, f: F);
}

impl SIPCore {

    pub fn new() -> SIPCore {
        // create default data
        unsafe {
            pjsua_create();
        }

        let sip_core = SIPCore {
            pool: ptr::null_mut(),
            app_config: pjsua_config::new(),
            log_config: pjsua_logging_config::new(),
            media_config: SIPMedia::new(),
            no_udp: false,
            no_tcp: false,
            use_ipv6: false,
            transports: SIPTransports::new(),
            accounts: SIPAccounts::new(), // only reg acc not local
            presence: SIPPresence::new(),
            calls: SIPCalls::new(),
            tones: Vec::new(),
            ringback: SIPRingback::new(),
            ringtone: SIPRingtone::new(),
            wav_player: None,
            wav_recorder: None,
            default_handler: pjsip_module::new(),
            redir_op: PJSIP_REDIRECT_ACCEPT_REPLACE,
            input_dev: PJSUA_INVALID_ID,
            output_dev: PJSUA_INVALID_ID,
            input_latency: 100,
            output_latency: 140,
            auto_play_hangup: false,
            duration: 0,
            current_call: -1,
            aud_cnt: 1,
            auto_answer: 0,
            invite_event: SIPCoreInviteEvents::new()
        };

        sip_core
    }

    pub fn start(&mut self) {
        unsafe {
            self.pool = pjsua_pool_create(
                CString::new("ipcodec").expect("error").into_raw(),
                1000,
                1000,
            );

            assert_ne!(self.pool.is_null(), true);

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
                &mut self.media_config.get_context() as *mut _,
            );

            // pjsip endpoint for unhadled error
            self.default_handler.on_rx_request = Some(SIPCore::on_rx_request);
            let status = pjsip_endpt_register_module(
                pjsua_get_pjsip_endpt(),
                &mut self.default_handler as *mut _,
            );

            if status != 0 {
                panic!("cant register module");
            }

            // add optional tones
            for _ in 0..32 {
                let mut tones = SIPTones::new();
                tones.init(self.pool, 440, 480);
                self.tones.push(tones);
            }

            // init ringback
            self.ringback.init(self.pool, self.media_config.get_context());

            // init ringtone
            self.ringtone.init(self.pool, self.media_config.get_context());

            // Initialize UDP Transport
            if !self.no_udp {
                self.transports
                    .add(pjsip_transport_type_e_PJSIP_TRANSPORT_UDP);
                if self.use_ipv6 == true {
                    self.transports
                        .add(pjsip_transport_type_e_PJSIP_TRANSPORT_UDP6);
                }
            }

            // initialize TCP transport
            if !self.no_tcp {
                self.transports
                    .add(pjsip_transport_type_e_PJSIP_TRANSPORT_TCP);
                if self.use_ipv6 {
                    self.transports
                        .add(pjsip_transport_type_e_PJSIP_TRANSPORT_TCP6);
                }
            }

            self.media_config.init();
            self.calls.set_audio_count(self.aud_cnt);

            // we don't need add account for this state
            // so we create dynamicaly in addition

            let status = pjsua_start();
            if status != PJ_SUCCESS as pj_status_t {
                pjsua_perror(CString::new("cant start").expect("error").into_raw() as *const _,
                 CString::new("cant start").expect("error").into_raw() as *const _, status);
            }

        }
    }

    pub fn deinit(&mut self) {
        unsafe {
            pj_pool_safe_release(&mut self.pool as *mut _);
            pjsua_destroy();
        }
    }

    pub fn call(&mut self, call_addr: &str) {
        unsafe{

            let mut msg_data = pjsua_msg_data::new();
            pjsua_msg_data_init(&mut msg_data as *mut _);

            let default_acc = pjsua_acc_get_default();
            println!("default accid : {}", default_acc);
            let mut call_addr = pj_str(CString::new(call_addr).expect("error").into_raw() as *mut _);
            let status = pjsua_call_make_call(
                        default_acc,
                        &mut call_addr as *const _,
                        &mut self.calls.get_call_opt() as *mut _,
                        // ptr::null_mut(),
                        // &mut msg_data as *mut _,
                        // self.current_call as *mut _
                        ptr::null_mut(),
                        ptr::null_mut(),
                        ptr::null_mut()
            );
        }
    }

    pub fn call_account(&self) {

    }

    pub fn get_input_level(&self) -> i32 {
        self.media_config.get_input_level()
    }

    pub fn get_output_level(&self) -> i32 {
        self.media_config.get_output_level()
    }

    pub fn set_input_level(&mut self, value: i32) {
        self.media_config.set_input_level(value);
    }

    pub fn set_output_level(&mut self, value: i32) {
        self.media_config.set_output_level(value);
    }

    pub fn get_signal_level(&self) -> (u32, u32, u32, u32) {
        self.media_config.get_signal_level()
    }

    pub fn ringback_start(&self, call_id: pjsua_call_id) {}

    pub fn ring_stop(&self, call_id: &pjsua_call_id) {}

    pub fn ring_start(&self, call_id: pjsua_call_id) {}

    pub fn find_next_call(&self) {}

    pub fn on_call_generic_media_state(&self, ci: &pjsua_call_info, mi: u32, has_error: &mut bool) {
        // println!("OnCallGenericMediaState");
        let status_name: [&str; 5] = [
            "None",
            "Active",
            "Local hold",
            "Remote hold",
            "Error"
        ];

        unsafe {
            let media_type = CStr::from_ptr(
                pjmedia_type_name(ci.media[mi as usize].type_)
            ).to_str().expect("Error string media type");

            println!("sipua.rs Call {} media {} [type={}], status is {}",
                ci.id,
                mi,
                media_type,
                status_name[ci.media[mi as usize].status as usize]
            );
        }

    }

    pub fn on_call_audio_state(&mut self, ci: &pjsua_call_info, mi: u32, has_error: &mut bool) {

        let media = ci.media[mi as usize];

        if media.status == PJSUA_CALL_MEDIA_ACTIVE ||
        media.status == PJSUA_CALL_MEDIA_REMOTE_HOLD {
            let call_conf_slot: pjsua_conf_port_id;

            unsafe {

                call_conf_slot = media.stream.aud.conf_slot;

                let mut call_ids: [pjsua_call_id; 32] = [-1; 32];
                let mut call_cnt = 32u32;

                pjsua_enum_calls(call_ids.as_mut_ptr(),
                    &mut call_cnt as *mut _);

                for idx in 0..call_cnt as usize {
                    if call_ids[idx] == ci.id { continue; }
                    if  pjsua_call_has_media(call_ids[idx].clone()) == PJ_FALSE as pj_bool_t { continue; }

                    // connect rx
                    pjsua_conf_connect(call_conf_slot,
                        pjsua_call_get_conf_port(call_ids[idx])
                    );

                    // connect tx
                    pjsua_conf_connect(pjsua_call_get_conf_port(call_ids[idx]),
                        call_conf_slot
                    );
                }

                pjsua_conf_connect(call_conf_slot, 0);
                pjsua_conf_connect(0, call_conf_slot);
            }
        }
    }

    pub fn callback_on_call_state(&mut self, call_id: pjsua_call_id, e: *mut pjsip_event) {
        unsafe {
            let mut call_info: pjsua_call_info = pjsua_call_info::new();

            let status = pjsua_call_get_info(call_id, &mut call_info as *mut _);
            if status != PJ_SUCCESS as pj_status_t {
                panic!("OnCallState : pjsua_call_get_info fail.");
            }

            let state_message: String = match call_info.state {
                PJSIP_INV_STATE_NULL => String::from("NULL"),
                PJSIP_INV_STATE_CALLING => String::from("CALLING"),
                PJSIP_INV_STATE_INCOMING => String::from("INCOMING"),
                PJSIP_INV_STATE_EARLY => String::from("EARLY"),
                PJSIP_INV_STATE_CONNECTING => String::from("CONNECTING"),
                PJSIP_INV_STATE_CONFIRMED => String::from("CONFIRMED"),
                PJSIP_INV_STATE_DISCONNECTED => String::from("DISCONNECTED"),
                _ => String::from("FAILURE")
            };

            println!("INVSTATE [{}]", state_message);
            // todo ringing mecanism

            // call event for non internal
            match call_info.state {
                PJSIP_INV_STATE_NULL => (self.invite_event.null)(),
                PJSIP_INV_STATE_CALLING => (self.invite_event.calling)(),
                PJSIP_INV_STATE_INCOMING => (self.invite_event.incoming)(),
                PJSIP_INV_STATE_EARLY => (self.invite_event.early)(),
                PJSIP_INV_STATE_CONNECTING => (self.invite_event.early)(),
                PJSIP_INV_STATE_CONFIRMED => (self.invite_event.confirmed)(),
                PJSIP_INV_STATE_DISCONNECTED => (self.invite_event.disconnected)(),
                _ => (self.invite_event.failure)()
            }

        }
    }

    pub fn callback_on_call_media_state(&mut self, call_id: pjsua_call_id) {
        println!("OnCallMediaState");
        unsafe {

            let mut call_info: pjsua_call_info = pjsua_call_info::new();
            let mut has_error = false;
            let mut media_type: &str = "Unknown";

            pjsua_call_get_info(call_id, &mut call_info as *mut _);

            for mi in 0..call_info.media_cnt {

                self.on_call_generic_media_state(&call_info, mi, &mut has_error);

                media_type = match call_info.media[mi as usize].type_ {

                    PJMEDIA_TYPE_NONE => {
                        has_error = true;
                        "None"
                    },
                    PJMEDIA_TYPE_AUDIO => {
                        self.on_call_audio_state(&call_info, mi, &mut has_error);
                        "Audio"
                    },

                    PJMEDIA_TYPE_VIDEO => {
                        has_error = true;
                        "Video"
                    },

                    PJMEDIA_TYPE_APPLICATION => {
                        has_error = true;
                        "Application"
                    },

                    PJMEDIA_TYPE_UNKNOWN => {
                        has_error = true;
                        "Unknown"
                    }
                    _ => {
                        has_error = true;
                        "Unsuported"
                    }
                };
            }

            if has_error {
                let reason = CString::new(format!("Media failed : {}", media_type)).expect("cant create str");
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

            pjsua_call_answer2(
                call_id,
                &opt as *const _,
                self.auto_answer,
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }

    pub fn callback_on_dtmf_digit2(&self, call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        unsafe {
            let mut dtmf: &str = "None";

            match (*info).method {
                pjsua_dtmf_method_PJSUA_DTMF_METHOD_RFC2833 => {
                    dtmf = "RFC2833";
                }
                pjsua_dtmf_method_PJSUA_DTMF_METHOD_SIP_INFO => {
                    dtmf = "SIP INFO";
                }
                _ => println!("Unknown dtmf method"),
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

    pub fn callback_on_incoming_subscribe(
        &self,
        acc_id: pjsua_acc_id,
        srv_pres: *mut pjsua_srv_pres,
        buddy_id: pjsua_buddy_id,
        from: *const pj_str_t,
        rdata: *mut pjsip_rx_data,
        code: *mut pjsip_status_code,
        reason: *mut pj_str_t,
        msg_data: *mut pjsua_msg_data)
    {
        // Todo
    }

    pub fn callback_on_buddy_state(&self, buddy_id: pjsua_buddy_id) {
        unsafe {
            let mut info: pjsua_buddy_info = pjsua_buddy_info::new();
            pjsua_buddy_get_info(buddy_id, &mut info as *mut _);
        }
    }

    pub fn callback_on_buddy_evsub_state(
        &self,
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
        unsafe {
            let rdata = (*event).body.tsx_state.src.rdata;
            // let astr = pjsip_rx_data_get_info(rdata);
            println!("Buddy subscription state");
        }
    }

    pub fn callback_on_pager(
        &self,
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        mime_type: *const pj_str_t,
        body: *const pj_str_t,
    ) {
        println!("OnPager");
    }

    pub fn callback_on_typing(
        &self,
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
        println!("IM indication.");
    }

    pub fn callback_on_call_transfer_status(
        &self,
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
        unsafe {
            println!("Call {} transfer status={}", call_id, st_code);
            if st_code / 100 == 2 {
                pjsua_call_hangup(
                    call_id,
                    pjsip_status_code_PJSIP_SC_GONE,
                    ptr::null() as *const _,
                    ptr::null() as *const _,
                );
                *p_cont = PJ_FALSE as pj_bool_t;
            }
        }
    }

    pub fn callback_on_call_replaced(
        &self,
        old_call_id: pjsua_call_id,
        new_call_id: pjsua_call_id,
    ) {
        unsafe {
            let mut old_ci: pjsua_call_info = pjsua_call_info::new();
            let mut new_ci: pjsua_call_info = pjsua_call_info::new();

            pjsua_call_get_info(old_call_id, &mut old_ci as *mut _);
            pjsua_call_get_info(new_call_id, &mut new_ci as *mut _);

            println!(
                "Call {} is being replaced by call {}",
                old_call_id, new_call_id
            );
        }
    }

    pub fn callback_on_nat_detect(&self, res: *const pj_stun_nat_detect_result) {
        // logging nat detect result this only for trouble shooting
        unsafe {

            let nat_result = *res;
            let status_str;
            let status_text;
            let nat_type_name;

            if nat_result.status == PJ_SUCCESS as pj_status_t {
                status_str = String::from("SUCCESS");
            } else {
                status_str = String::from("FAILED");
            }

            // get status text from nat result
            status_text = CStr::from_ptr(nat_result.status_text)
                .to_owned()
                .into_string()
                .expect("nat status_text fail");

            // get nat_type_name from nat result
            nat_type_name = CStr::from_ptr(nat_result.nat_type_name)
                .to_owned()
                .into_string()
                .expect("nat_type_name fail");

            println!("NAT detected result: [{}], status [{}], type: [{}]",
                status_str, status_text, nat_type_name
            );
        }
    }

    pub fn callback_on_mwi_info(&self, acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {
        unsafe {
            println!("Received MWI for acc_id {}", acc_id);
            let ctype = (*(*mwi_info).rdata).msg_info.ctype;

            if !ctype.is_null() {
                println!(
                    "Content-Type: {} {}/ {} {}",
                    (*ctype).media.type_.slen,
                    CString::from_raw((*ctype).media.type_.ptr)
                        .into_string()
                        .expect("error"),
                    (*ctype).media.subtype.slen,
                    CString::from_raw((*ctype).media.subtype.ptr)
                        .into_string()
                        .expect("error")
                );
            }
        }
    }

    pub fn callback_on_transport_state(
        &self,
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
        unsafe {
            // transport
            let tp = *tp;
            // transport type name TCP or UDP
            let type_name = CStr::from_ptr(tp.type_name)
                    .to_owned()
                    .into_string()
                    .expect("tp.transport null info");

            // remote address name it can be server or ptp client
            let remote_name = CStr::from_ptr(tp.remote_name.host.ptr)
                    .to_owned()
                    .into_string()
                    .expect("remote_name null info");

            // remote port
            let remote_port = tp.remote_name.port;

            let op_name: String = match state {
                PJSIP_TP_STATE_CONNECTED => String::from("CONNECTED"),
                PJSIP_TP_STATE_DISCONNECTED => String::from("DISCONNECTED"),
                PJSIP_TP_STATE_SHUTDOWN => String::from("SHUTDOWN"),
                PJSIP_TP_STATE_DESTROY => String::from("DESTROY"),
                _ => String::from("unknown")
            };

            println!("SIP [{}] transport is {} to {}:{}",
                type_name, op_name, remote_name, remote_port
            );
        }
    }

    pub fn callback_on_ice_transport_error(
        &self,
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
        let ice_trans_op_text: String;
        let ice_status_text: String;

        if status == PJ_SUCCESS as pj_status_t {
            ice_status_text = String::from("SUCCESS");
        } else {
            ice_status_text = String::from("FAILURE");
        }

        match op {
            PJ_ICE_STRANS_OP_INIT => ice_trans_op_text = String::from("INIT"),
            PJ_ICE_STRANS_OP_NEGOTIATION => ice_trans_op_text = String::from("NEGOTIATION"),
            PJ_ICE_STRANS_OP_KEEP_ALIVE => ice_trans_op_text = String::from("KEEPALIVE"),
            PJ_ICE_STRANS_OP_ADDR_CHANGE => ice_trans_op_text = String::from("ADDRCHANGE"),
            _ => ice_trans_op_text = String::from("UNKNOWN OP")
        }

        println!("ICE [{}] status [{}], operation [{}]",
            index, ice_status_text, ice_trans_op_text,
        );
    }

    pub fn callback_on_snd_dev_operation(&self, operation: c_int) -> pj_status_t {
        // println!("OnSndDevOperation");
        let mut cap_dev = -1;
        let mut play_dev = -1;
        let op: String;

        unsafe {
            pjsua_get_snd_dev(&mut cap_dev as *mut _, &mut play_dev as *mut _);
        }

        if operation > 0 {
            op = String::from("ON");
        } else {
            op = String::from("OFF");
        }

        println!( "Turning sound device input {} output {} : {}",
            cap_dev, play_dev, op
        );

        PJ_SUCCESS as pj_status_t
    }

    pub fn callback_on_call_media_event(
        &self,
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event)
    {


        println!("Event {}", "skip");
        // }
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


            let status_text: String = match status as u32 {
                PJ_SUCCESS => String::from("SUCCESS"),
                _ => String::from("FAILURE")
            };

            if !info.is_null() {
                let info = *info;
                pjsua_transport_get_info(info.lis_restart.transport_id,
                    &mut tp_info as *mut _
                );

                pjsua_acc_get_info(info.acc_shutdown_tp.acc_id,
                    &mut acc_info as *mut _
                );
            }

            let op_text = match op {
                PJSUA_IP_CHANGE_OP_NULL => String::from("NULL"),
                PJSUA_IP_CHANGE_OP_RESTART_LIS => String::from("RESTART_LISTENER"),
                PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP => String::from("ACC_SHUTDOWN_TP"),
                PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT => String::from("ACC_UPDATE_CONTACT"),
                PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS => String::from("ACC_HANGUP_CALLS"),
                PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS => String::from("ACC_REINVITE_CALLS"),
                PJSUA_IP_CHANGE_OP_COMPLETED => String::from("COMPLETED"),
                _ => String::from("UNKNOWN")
            };

            let base_str = format!("IPCHANGE [{}] op: [{}]",
                status_text, op_text
            );

            let mut log_str: String = String::new();

            if op == PJSUA_IP_CHANGE_OP_RESTART_LIS {
                log_str = format!("{} : restart transport {}", base_str, tp_info.info.to_string());
            }

            if op == PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP {
                log_str = format!("{} : transport shutdown for account {}",
                    base_str,
                    acc_info.acc_uri.to_string()
                );
            }

            if op == PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT {
                if (*info).acc_update_contact.code != 0 {
                    log_str = format!("{} : update contact for account {} code: [{}]",
                        base_str,
                        acc_info.acc_uri.to_string(),
                        (*info).acc_update_contact.code
                    );
                } else {
                    log_str = format!("{} : update contact for account {}",
                        base_str,
                        acc_info.acc_uri.to_string(),
                    );
                }
            }

            if op == PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS {
                log_str = format!("{} : hangup call for account {}, call_id[{}]",
                    base_str,
                    acc_info.acc_uri.to_string(),
                    (*info).acc_hangup_calls.call_id
                );
            }

            if op == PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS {
                log_str = format!("{} : reinvite call for account {}, call_id[{}]",
                    base_str,
                    acc_info.acc_uri.to_string(),
                    (*info).acc_reinvite_calls.call_id
                );
            }

            if op == PJSUA_IP_CHANGE_OP_COMPLETED {
                log_str = format!("{} : done", base_str);
            }

            println!("{}", log_str);
        }
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
            }
            _ => panic!("Panic OnDtmfDigit2"),
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
            Some(ref mut sipcore) => sipcore.callback_on_call_redirected(call_id, target, e),
            _ => panic!("Panic OnCallRedirected"),
        }
    }

    // REG state
    unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_reg_state(acc_id);
            }
            _ => panic!("Panic OnRegState"),
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
                sipcore.callback_on_incoming_subscribe(
                    acc_id, srv_pres, buddy_id, from, rdata, code, reason, msg_data,
                );
            }
            _ => panic!("Panic OnIncomingSubscribe"),
        }
    }

    // Buddy State
    unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_buddy_state(buddy_id);
            }
            _ => panic!("Panic OnBuddyState"),
        }
    }

    // Buddy evsub state
    unsafe extern "C" fn on_buddy_evsub_state(
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_buddy_evsub_state(buddy_id, sub, event);
            }
            _ => panic!("Panic OnBuddyEvsubState"),
        }
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
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_pager(call_id, from, to, contact, mime_type, body);
            }
            _ => panic!("Panic OnPager"),
        }
    }

    // Typing event
    unsafe extern "C" fn on_typing(
        call_id: pjsua_call_id,
        from: *const pj_str_t,
        to: *const pj_str_t,
        contact: *const pj_str_t,
        is_typing: pj_bool_t,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_typing(call_id, from, to, contact, is_typing);
            }
            _ => panic!("Panic OnTyping"),
        }
    }

    // Call transfer status
    unsafe extern "C" fn on_call_transfer_status(
        call_id: pjsua_call_id,
        st_code: c_int,
        st_text: *const pj_str_t,
        final_: pj_bool_t,
        p_cont: *mut pj_bool_t,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_transfer_status(call_id, st_code, st_text, final_, p_cont);
            }
            _ => panic!("Panic OnCallTransferStatus"),
        }
    }

    // Call replaced
    unsafe extern "C" fn on_call_replaced(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_replaced(old_call_id, new_call_id);
            }
            _ => panic!("Panic OnCallReplaced"),
        }
    }

    // NAT detect
    unsafe extern "C" fn on_nat_detect(res: *const pj_stun_nat_detect_result) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_nat_detect(res);
            }
            _ => panic!("Panic OnNatDetect"),
        }
    }

    // MWI info
    unsafe extern "C" fn on_mwi_info(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_mwi_info(acc_id, mwi_info);
            }
            _ => panic!("Panic OnMwiInfo"),
        }
    }

    // Transport state
    unsafe extern "C" fn on_transport_state(
        tp: *mut pjsip_transport,
        state: pjsip_transport_state,
        info: *const pjsip_transport_state_info,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_transport_state(tp, state, info);
            }
            _ => panic!("Panic OnTransportState"),
        }
    }

    // ICE transport error
    unsafe extern "C" fn on_ice_transport_error(
        index: c_int,
        op: pj_ice_strans_op,
        status: pj_status_t,
        param: *mut c_void,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_ice_transport_error(index, op, status, param);
            }
            _ => panic!("Panic OnTransportError"),
        }
    }

    // Sound device operation
    unsafe extern "C" fn on_snd_dev_operation(operation: c_int) -> pj_status_t {
        match SIP_CORE {
            Some(ref mut sipcore) => sipcore.callback_on_snd_dev_operation(operation),
            _ => panic!("Panic OnSndDevOperation"),
        }
    }

    // Call media event
    unsafe extern "C" fn on_call_media_event(
        call_id: pjsua_call_id,
        med_idx: c_uint,
        event: *mut pjmedia_event,
    ) {
        match SIP_CORE {
            Some(ref mut sipcore) => {
                sipcore.callback_on_call_media_event(call_id, med_idx, event);
            }
            _ => panic!("Panic OnCallMediaEvent"),
        }
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
            }
            _ => panic!("Panic OnIpChangeProgress"),
        }
    }
}

impl SIPCoreInviteExt for SIPCore {

    fn connect_invite_calling <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.calling = Box::new(f);
    }

    fn connect_invite_incoming <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.incoming = Box::new(f);
    }

    fn connect_invite_early <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.early = Box::new(f);
    }

    fn connect_invite_connecting <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.connecting = Box::new(f);
    }

    fn connect_invite_confirmed <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.confirmed = Box::new(f);
    }

    fn connect_invite_disconnected <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.disconnected = Box::new(f);
    }

    fn connect_invite_null <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.null = Box::new(f);
    }

    fn connect_invite_failure <F: Fn() + 'static> (&mut self, f: F) {
        self.invite_event.failure = Box::new(f);
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
        if status != PJ_SUCCESS as i32 {
            return;
        }

        let exp: *const pjsip_expires_hdr = pjsip_msg_find_hdr(
            (*rdata).msg_info.msg,
            pjsip_hdr_e_PJSIP_H_EXPIRES,
            ptr::null_mut(),
        ) as *const _;

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
            ptr::null_mut(),
            None,
        );
    }
}

// handle for callback PjsipModule
impl PjsipModuleCallback for SIPCore {

    unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_status_t {
        println!("OnRxRequest");
        // base rx request handle undefined state.
        let tdata: *const pjsip_tx_data = ptr::null();
        let status_code: pjsip_status_code;
        let status: pj_status_t;

        let mut rdata = *rdata;
        let msg = *rdata.msg_info.msg;
        let mut method = msg.line.req.method;
        // let msg_info = method.msg_info;
        if pjsip_method_cmp(&mut method as *const _, &pjsip_ack_method as *const _) == 0 {
            return PJ_TRUE as pj_status_t;
        }

        if pjsip_method_cmp(&mut method as *const _, &pjsip_register_method as *const _) == 0 {
            // call simple registrar pjsip_tx_data
            simple_registrar(&mut rdata as *mut _);
            return PJ_TRUE as pj_status_t;
        }

        if pjsip_method_cmp(&mut method as *const _, &pjsip_notify_method as *const _) == 0 {
            status_code = pjsip_status_code_PJSIP_SC_BAD_REQUEST as pjsip_status_code;
        } else {
            status_code = pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED;
        }

        status = pjsip_endpt_create_response(
            pjsua_get_pjsip_endpt(),
            &mut rdata as *const _,
            status_code as c_int,
            ptr::null() as *const _,
            tdata as *mut *mut _,
        );

        if status != (PJ_SUCCESS as pj_status_t) {
            return PJ_TRUE as pj_status_t;
        }

        if status_code == pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED {
            #[allow(unused_assignments)]
            let mut cap_hdr: *const pjsip_hdr = ptr::null();

            cap_hdr = pjsip_endpt_get_capability(
                pjsua_get_pjsip_endpt(),
                pjsip_hdr_e_PJSIP_H_ALLOW as i32,
                ptr::null() as *const _,
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

        PJ_TRUE as pj_status_t
    }
}

