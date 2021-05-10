
use pjproject::{pjsip::SIPRedirectOp, pjsua::{UAConfig, UALoggingConfig, UAMediaConfig, transport::UATransport}};

use crate::pjproject::{prelude::*, utils::boolean_to_pjbool};

// use crate::pjproject::utils;
use crate::pjproject::pjsip;
use crate::pjproject::pjsua;
use crate::pjproject::pjmedia;

use std::ptr;
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void, c_uint};
use super::SIPInviteState;


pub static mut SIP_CORE: Option<SIPCore> = None;
pub static mut CURRENT_CALL: Option<i32> = None;


pub struct SIPCore {
    pub ua_config: UAConfig,
    pub log_config: UALoggingConfig,
    pub media_config: UAMediaConfig,
    module: SIPModule,
    no_udp: bool,
    no_tcp: bool,
    use_ipv6: bool,
    transport_udp: Option<UATransport>,
    transport_tcp: Option<UATransport>,
    transport_udp6: Option<UATransport>,
    transport_tcp6: Option<UATransport>,
    redir_op: SIPRedirectOp,
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    auto_play_hangup: bool,
    duration: u32,
    current_call: i32,
    auto_answer: u32,
    events: SIPCoreEvents,
    no_refersub: bool,
    compact_form: bool,
}
// struct to hold invite event function
struct SIPCoreEvents {
    invite: Box<dyn FnMut(SIPInviteState)>,
    incoming_call: Box<dyn FnMut()>
}

impl SIPCoreEvents {
    fn new() -> SIPCoreEvents {
        SIPCoreEvents {
            invite: Box::new(|s| {}),
            incoming_call: Box::new(|| {})
        }
    }
}

pub trait SIPCoreEventsExt {
    fn connect_invite<F: FnMut(SIPInviteState) + 'static> (&mut self, f: F);
    fn connect_incoming_call<F: Fn() + 'static> (&mut self, f: F);
}

impl SIPCore {

    pub fn new() -> Self {
        // pjsua::create().expect("SIPCore::pjsua_create");
        //SIPUa::create();
        SIPCore {
            ua_config: UAConfig::default(),
            log_config: UALoggingConfig::new(),
            media_config: UAMediaConfig::new(),
            module: SIPModule::new(),
            transport_udp: None,
            transport_tcp: None,
            transport_udp6: None,
            transport_tcp6: None,
            redir_op: PJSIP_REDIRECT_ACCEPT_REPLACE,
            input_dev: pjsua::PJSUA_INVALID_ID,
            output_dev: pjsua::PJSUA_INVALID_ID,
            input_latency: 100,
            output_latency: 140,
            auto_play_hangup: false,
            duration: 0,
            current_call: -1,
            auto_answer: 0,
            events: SIPCoreEvents::new(),
            no_refersub: false,
            compact_form: false,
            no_udp: false,
            no_tcp: true,
            use_ipv6: false,
        }
    }

    pub fn init(&mut self) {

        pjsua::create().unwrap();

        // set all default media event
        self.ua_config.connect_on_call_state(Some(on_call_state));
        self.ua_config.connect_on_stream_destroyed(Some(on_stream_destroyed));
        self.ua_config.connect_on_call_media_state(Some(on_call_media_state));
        self.ua_config.connect_on_incoming_call(Some(on_incoming_call));
        self.ua_config.connect_on_call_redirected(Some(on_call_redirected));
        self.ua_config.connect_on_dtmf_digit2(Some(on_dtmf_digit2));
        self.ua_config.connect_on_reg_state(Some(on_reg_state));
        self.ua_config.connect_on_incoming_subscribe(Some(on_incoming_subscribe));
        self.ua_config.connect_on_buddy_state(Some(on_buddy_state));
        self.ua_config.connect_on_buddy_evsub_state(Some(on_buddy_evsub_state));
        self.ua_config.connect_on_pager(Some(on_pager));
        self.ua_config.connect_on_typing(Some(on_typing));
        self.ua_config.connect_on_call_transfer_status(Some(on_call_transfer_status));
        self.ua_config.connect_on_call_replaced(Some(on_call_replaced));
        self.ua_config.connect_on_nat_detect(Some(on_nat_detect));
        self.ua_config.connect_on_mwi_info(Some(on_mwi_info));
        self.ua_config.connect_on_transport_state(Some(on_transport_state));
        self.ua_config.connect_on_ice_transport_error(Some(on_ice_transport_error));
        self.ua_config.connect_on_snd_dev_operation(Some(on_snd_dev_operation));
        self.ua_config.connect_on_call_media_event(Some(on_call_media_event));
        self.ua_config.connect_on_ip_change_progress(Some(on_ip_change_progress));

        // set only one call per seassons
        self.ua_config.set_max_calls(1);
        self.ua_config.set_force_lr(true);
        self.ua_config.set_user_agent(String::from("IpCodec"));

        // register sub module for unhandeled error
        self.module.set_priority((PJSIP_MOD_PRIORITY_APPLICATION + 99) as i32);
        self.module.set_name(String::from("mod-default-handler"));
        self.module.connect_on_rx_request(Some(on_rx_request));
        self.module.register();


        // SIPUa::init(&self.ua_config, &self.log_config, &self.media_config);
        self.ua_config.init(&self.log_config, &self.media_config);

        // add optional tones
        // TODO fix code bellow
        // for _ in 0..32 {
        //     let mut tones = SIPTones::new();
        //     tones.init(self.ua_config.get_pool(), 440, 480);
        //     self.tones.push(tones);
        // }

        // check if setting not have atleast 1 protocol
        if self.no_udp & self.no_tcp {
            self.no_udp = false;
        }

        // Initialize UDP Transport
        if !self.no_udp {
            let tp = UATransport::new();
            
        }

        // initialize TCP transport
        if !self.no_tcp {

        }

        // TODO create local account
        for tp in self.transports.list.iter() {
            let tid = tp.get_id();
            let mut acc_id: pjsua_acc_id = -1;

            pjsua::acc_add_local( tid, true, &mut acc_id)
            .expect("SIPTransport::acc_add_local");

            let mut acc_cfg = pjsua_acc_config::new();
            pjsua::acc_get_config(acc_id, &mut acc_cfg).unwrap();

            acc_cfg.rtp_cfg.port = 4000;
            acc_cfg.ipv6_media_use = boolean_to_pjbool(self.use_ipv6) as u32;


            pjsua::acc_modify(acc_id, &mut acc_cfg).unwrap();
            pjsua::acc_set_online_status(pjsua::acc_get_default(), true).unwrap();
        }

        self.media_config.init();


        pjsua::start();
    }

    pub fn restart(&mut self) {
        self.stop();
        self.init();
    }

    pub fn stop(&mut self) {
        SIPUa::destroy();
        // self.ua_config.deinit();
    }

    pub fn call(&self, call_addr: &str) {

        let calls = SIPCalls::new();
        let accounts = SIPAccounts::new();

        let default_acc_id = accounts.get_default();
        let default_acc = SIPAccount::from(default_acc_id).unwrap();

        let mut msg_data = pjsua_msg_data::new();

        default_acc.call(String::from(call_addr), None, Some(&mut msg_data), None);
    }

    pub fn call_hangup(&self) {

        if self.current_call == -1 {
            return;
        }

        let calls = SIPCalls::new();

        if calls.get_count() > 1 {
            calls.hangup_all();
        } else {
            let call = SIPCall::from(self.current_call);
            call.hangup(0, None, None);
        }

    }

    pub fn call_answer(&self) {

        if self.current_call == -1 {
            println!("call_answer: no active call");
            return;
        }

        let calls = SIPCalls::new();
        let current_call = SIPCall::from(self.current_call);
        let mut msg_data = pjsua_msg_data::new();
        let mut call_opt = calls.get_call_opt();

        current_call.answer2(&mut call_opt, 200, None, Some(&mut msg_data));
    }

    pub fn ringback_start(&self, call_id: pjsua_call_id) {
        // start procedure ringback
    }

    pub fn ring_stop(&self, call_id: &pjsua_call_id) {
        // ring stop on incomming call
    }

    pub fn ring_start(&self, call_id: pjsua_call_id) {
        // ring start on incomming call
    }

    pub fn auto_answer(&mut self, value: bool) {
        if value {
            self.auto_answer = 200;
        } else {
            self.auto_answer = 0;
        }
    }

    pub fn set_no_refersub(&mut self, value: bool) {
        self.no_refersub = value;
    }

    pub fn set_no_forcelr(&self, value: bool) {
        self.ua_config.set_force_lr(!value);
    }

    pub fn set_compact_form(&mut self, value: bool) {
        self.compact_form = value;
    }

    pub fn on_call_audio_state(&mut self, ci: &pjsua_call_info, mi: u32, has_error: &mut bool) {

        let media = &ci.media[mi as usize];

        if media.status == PJSUA_CALL_MEDIA_ACTIVE ||
        media.status == PJSUA_CALL_MEDIA_REMOTE_HOLD {
            let call_conf_slot: pjsua_conf_port_id;



            let mut call_ids: [pjsua_call_id; 32] = [-1; 32];
            let mut call_cnt = 32u32;

            pjsua::enum_calls(&mut call_ids, &mut call_cnt).unwrap();

            unsafe {
                call_conf_slot = media.stream.aud.as_ref().conf_slot;

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
            }
            pjsua::conf_connect(call_conf_slot, 0).unwrap();
            pjsua::conf_connect(0, call_conf_slot).unwrap();

        }

    }

    pub fn callback_on_call_state(&mut self, call_id: pjsua_call_id, e: *mut pjsip_event) {

        let call = SIPCall::from(call_id);
        let call_info = call.get_info().unwrap();

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
            PJSIP_INV_STATE_NULL => (self.events.invite)(SIPInviteState::Null),
            PJSIP_INV_STATE_CALLING => (self.events.invite)(SIPInviteState::Calling),
            PJSIP_INV_STATE_INCOMING => (self.events.invite)(SIPInviteState::Incoming),
            PJSIP_INV_STATE_EARLY => (self.events.invite)(SIPInviteState::Early),
            PJSIP_INV_STATE_CONNECTING => (self.events.invite)(SIPInviteState::Connecting),
            PJSIP_INV_STATE_CONFIRMED => (self.events.invite)(SIPInviteState::Confirmed),
            PJSIP_INV_STATE_DISCONNECTED => (self.events.invite)(SIPInviteState::Disconnected),
            _ => (self.events.invite)(SIPInviteState::Unknown)
        }

        match call_info.state {
            PJSIP_INV_STATE_DISCONNECTED => {
                if self.current_call == call_id  {
                    if call_info.state == PJSIP_INV_STATE_DISCONNECTED {
                        self.current_call = -1;
                    }
                } else if self.current_call != -1 {
                    let call = SIPCall::from(call_id);
                    call.hangup(0, None, None);
                }
            },
            _ => {
                if self.current_call == -1 {
                    self.current_call = call_id;
                }
            }
        }
    }

    pub fn callback_on_stream_destroyed(&self, call_id: pjsua_call_id, strm: *mut pjmedia_stream, stream_idx: c_uint) {
        println!("OnStreamDestroyed: CallID {}, StreamIDX {}", call_id, stream_idx);
    }

    pub fn callback_on_call_media_state(&mut self, call_id: pjsua_call_id) {
        println!("OnCallMediaState");

        let mut has_error = false;

        let call = SIPCall::from(call_id);
        let call_info = call.get_info().unwrap();

        for mi in 0..call_info.media_cnt {

            let call_media_info = &call_info.media[mi as usize];
            let media_type = pjmedia::type_name(call_media_info.type_);

            let status_name = match call_media_info.status {
                PJSUA_CALL_MEDIA_NONE => "None",
                PJSUA_CALL_MEDIA_ACTIVE => "Active",
                PJSUA_CALL_MEDIA_LOCAL_HOLD => "Local hold",
                PJSUA_CALL_MEDIA_REMOTE_HOLD => "Remote hold",
                PJSUA_CALL_MEDIA_ERROR => "Error",
                _ => "Error"
            };

            // match call_media_info.type_ {
            //     PJMEDIA_TYPE_NONE |
            //     PJMEDIA_TYPE_VIDEO |
            //     PJMEDIA_TYPE_APPLICATION |
            //     PJMEDIA_TYPE_UNKNOWN => has_error = true ,
            //     PJMEDIA_TYPE_AUDIO => self.on_call_audio_state(&call_info, mi, &mut has_error) ,
            //     _ => has_error = true
            // }

            println!("sipua.rs Call {} media {} [type={}], status is {}",
                call_info.id, mi, media_type, status_name);
        }

        if has_error {
            let reason = format!("Media failed");
            call.hangup(500, Some(reason), None);
        }
    }

    pub fn callback_on_incomming_call(&mut self, acc_id: pjsua_acc_id, call_id: pjsua_call_id, rdata: *mut pjsip_rx_data) {

        let sip_account = SIPAccount::from(acc_id).unwrap();
        let call = SIPCall::from(call_id);

        self.current_call = call_id;

        let calls = SIPCalls::new();
        let mut opt = calls.get_call_opt();

        // outer level
        println!("INVSTATE [INCOMING]");
        (self.events.invite)(SIPInviteState::Incoming);

        if self.auto_answer > 0 {
            call.answer2(&mut opt, 200, None, None);
        }

    }

    pub fn callback_on_dtmf_digit2(&self, call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        unsafe {
            let info = info.as_ref().unwrap();
            let dtmf = match info.method {
                PJSUA_DTMF_METHOD_RFC2833 => "RFC2833",
                PJSUA_DTMF_METHOD_SIP_INFO => "SIP INFO",
                _ => "Unknown dtmf method",
            };

            println!("Incomming DTMF on call using method {}", dtmf);
        }
    }

    pub fn callback_on_call_redirected(&self, call_id: pjsua_call_id, target: *const pjsip_uri, e: *const pjsip_event) -> pjsip_redirect_op {
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
        println!("On_incomming_subscribe")
    }

    pub fn callback_on_buddy_state(&self, buddy_id: pjsua_buddy_id) {
        let mut info: pjsua_buddy_info = pjsua_buddy_info::new();
        pjsua::buddy_get_info(buddy_id, &mut info).unwrap();
    }

    pub fn callback_on_buddy_evsub_state(
        &self,
        buddy_id: pjsua_buddy_id,
        sub: *mut pjsip_evsub,
        event: *mut pjsip_event,
    ) {

        // let rdata = (*event).body.tsx_state.as_ref().src.rdata;
        // let astr = pjsip_rx_data_get_info(rdata);
        println!("Buddy subscription state");
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
                    PJSIP_SC_GONE,
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
        println!( "Call {} is being replaced by call {}", old_call_id, new_call_id);
    }

    pub fn callback_on_nat_detect(&self, res: *const pj_stun_nat_detect_result) {
        // logging nat detect result this only for trouble shooting
        unsafe {

            let nat_result = &*res;
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
            let tp = &*tp;
            // transport type name TCP or UDP
            let type_name = CStr::from_ptr(tp.type_name)
                    .to_owned()
                    .into_string()
                    .expect("tp.transport null info");

            // remote address name it can be server or ptp client
            let remote_name = tp.remote_name.host.to_string();

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
        // this only log print
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

        pjsua::get_snd_dev(&mut cap_dev, &mut play_dev).unwrap();

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
        // if pjsua has support video
        println!("Event {}", "skip");
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
                let info = &*info;
                pjsua_transport_get_info(info.lis_restart.as_ref().transport_id,
                    &mut tp_info as *mut _
                );

                pjsua_acc_get_info(info.acc_shutdown_tp.as_ref().acc_id,
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
                if (*info).acc_update_contact.as_ref().code != 0 {
                    log_str = format!("{} : update contact for account {} code: [{}]",
                        base_str,
                        acc_info.acc_uri.to_string(),
                        (*info).acc_update_contact.as_ref().code
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
                    (*info).acc_hangup_calls.as_ref().call_id
                );
            }

            if op == PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS {
                log_str = format!("{} : reinvite call for account {}, call_id[{}]",
                    base_str,
                    acc_info.acc_uri.to_string(),
                    (*info).acc_reinvite_calls.as_ref() .call_id
                );
            }

            if op == PJSUA_IP_CHANGE_OP_COMPLETED {
                log_str = format!("{} : done", base_str);
            }

            println!("{}", log_str);
        }
    }

}

// trait to hold external event
impl SIPCoreEventsExt for SIPCore {

    fn connect_invite <F: FnMut(SIPInviteState) + 'static> (&mut self, f: F) {
        self.events.invite = Box::new(f);
    }

    fn connect_incoming_call <F: FnMut() + 'static> (&mut self, f: F) {
        self.events.incoming_call = Box::new(f);
    }
}

fn simple_registrar(rdata: *mut pjsip_rx_data) {
    println!("ON Simple Registrar");
    // unsafe {
    //     let tdata: *const pjsip_tx_data = ptr::null();
    //     let str_null: *const pj_str_t = ptr::null();
    //     let status: pj_status_t;
    //     let mut cnt: c_uint = 0;

    //     status = pjsip_endpt_create_response(
    //         pjsua_get_pjsip_endpt(),
    //         rdata as *const _,
    //         200,
    //         str_null as *const _,
    //         tdata as *mut _,
    //     );
    //     if status != PJ_SUCCESS as i32 {
    //         return;
    //     }

    //     let exp: *const pjsip_expires_hdr = pjsip_msg_find_hdr(
    //         (*rdata).msg_info.msg,
    //         PJSIP_H_EXPIRES,
    //         ptr::null_mut(),
    //     ) as *const _;

    //     // let rdata_ = &*rdata;
    //     // let llist: pjsip_hdr = (*(*rdata).msg_info.msg).hdr;
    //     // let mut h: *mut pjsip_hdr = (*(*rdata).msg_info.msg).hdr.next;
    //     let llist = &(*(&*rdata).msg_info.msg).hdr;
    //     let mut h= &*((*(&*rdata).msg_info.msg).hdr).next;

    //     while h != &*llist.next {
    //         if (*h as pjsip_hdr).type_ == (PJSIP_H_CONTACT as pjsip_hdr_e) {
    //             let c: *const pjsip_contact_hdr = h as *const pjsip_contact_hdr;
    //             let mut e: c_uint = (*c).expires;

    //             if e != 0xffffffff {
    //                 if !exp.is_null() {
    //                     e = (*exp).ivalue;
    //                 } else {
    //                     e = 3600;
    //                 }
    //             }

    //             if e > 0 {
    //                 let nc: *mut pjsip_contact_hdr =
    //                     pjsip_hdr_clone((*tdata).pool, h as *const _) as *mut pjsip_contact_hdr;

    //                 (*nc).expires = e;
    //                 pj_list_insert_before((*tdata).msg as *mut _, nc as *mut _);
    //                 cnt = cnt + 1;
    //             }
    //             h = (*h).next;
    //         }
    //     }

    //     // todo review c code for this. it's c clasic problem
    //     let srv: *mut pjsip_generic_string_hdr =
    //         pjsip_generic_string_hdr_create((*tdata).pool, str_null, str_null);
    //     // create server name
    //     let tmp: CString = CString::new("Server").expect("cant create Server string");
    //     (*srv).name = pj_str(tmp.as_ptr() as *mut c_char);
    //     // create add description
    //     let tmp: CString =
    //         CString::new("IpCodec simple registrar").expect("cant create simple registrar");
    //     (*srv).hvalue = pj_str(tmp.as_ptr() as *mut c_char);

    //     pj_list_insert_before((*tdata).msg as *mut _, srv as *mut _);
    //     let cb: pjsip_send_callback = None;
    //     pjsip_endpt_send_response2(
    //         pjsua::get_pjsip_endpt(),
    //         rdata,
    //         tdata as *mut _,
    //         ptr::null_mut(),
    //         None,
    //     );
    // }
}

unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_status_t {
    println!("OnRxRequest");
    // base rx request handle undefined state.
    let mut tdata: *mut pjsip_tx_data = &mut pjsip_tx_data::new() as *mut _;
    let status_code: pjsip_status_code;
    let status: pj_status_t;

    // let mut rdata = rdata;
    let msg = (*rdata).msg_info.msg;
    let method = &(*msg).line.req.as_ref().method;

    if pjsip::method_cmp(&method, &pjsip_ack_method) == 0 {
        return PJ_TRUE as pj_status_t;
    }

    if pjsip::method_cmp(&method , &pjsip_register_method) == 0 {
        // call simple registrar pjsip_tx_data
        simple_registrar(rdata as *mut _);
        return PJ_TRUE as pj_status_t;
    }

    let mmethod = &pjsip_simple_sys::pjsip_notify_method;
    if pjsip::method_cmp(&method, &mmethod) == 0 {
        status_code = PJSIP_SC_BAD_REQUEST;
    } else {
        status_code = PJSIP_SC_METHOD_NOT_ALLOWED;
    }

    status = pjsip_endpt_create_response(
        pjsua_get_pjsip_endpt(),
        rdata,
        status_code as c_int,
        ptr::null_mut() as *const _,
        &mut tdata as *mut _,
    );

    if status != PJ_SUCCESS as pj_status_t {
        return PJ_TRUE as pj_status_t;
    }

    let msg = (*tdata).msg;
    let ahdr: *mut pjsip_hdr = &mut (*msg).hdr as *mut _;

    if status_code == PJSIP_SC_METHOD_NOT_ALLOWED {


        let cap_hdr = pjsip_endpt_get_capability(
            pjsua_get_pjsip_endpt(),
            PJSIP_H_ALLOW as i32,
            ptr::null() as *const _,
        );

        if !cap_hdr.is_null() {
            let hdr_clone = pjsip_hdr_clone((*tdata).pool, cap_hdr as *const _) as *const pjsip_hdr;

            pj_list_insert_before(
                ahdr as *mut _ ,
                hdr_clone as *mut _,
            );
        }
    }

    // add user-agent header
    let mut ua = pj_str_t::from_string(String::from("User-Agent"));
    let mut agent = pj_str_t::from_string(String::from("IpCodec"));

    let h = pjsip_generic_string_hdr_create(
        (*tdata).pool,
        &mut ua as *const _,
        &mut agent as *const _,
    ) as *mut pjsip_hdr;

    pj_list_insert_before(ahdr as *mut _, h as *mut _);

    pjsip_endpt_send_response2(pjsua::get_pjsip_endpt(),
        rdata,
        tdata,
        ptr::null_mut(),
        None
    );

    PJ_TRUE as pj_status_t
}

// On Call State
unsafe extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_call_state(call_id, e),
        _ => panic!("panic on_call_state"),
    }
}

// On Stream Destroyed
unsafe extern "C" fn on_stream_destroyed(call_id: pjsua_call_id, strm: *mut pjmedia_stream, stream_idx: c_uint) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_stream_destroyed(call_id, strm, stream_idx),
        _ => panic!("panic on_call_stream_destroyed")
    }
}

// On Incoming Call
unsafe extern "C" fn on_incoming_call( acc_id: pjsua_acc_id, call_id: pjsua_call_id, rdata: *mut pjsip_rx_data) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_incomming_call(acc_id, call_id, rdata),
        _ => panic!("panic on_incoming_call")
    }
}

// On Call Media State
unsafe extern "C" fn on_call_media_state(call_id: pjsua_call_id) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_call_media_state(call_id),
        _ => panic!("panic on_call_media_state")
    }
}

// On DTMF digit
unsafe extern "C" fn on_dtmf_digit2(call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_dtmf_digit2(call_id, info),
        _ => panic!("panic on_dtmf_digit2")
    }
}

// On Call Redirected
unsafe extern "C" fn on_call_redirected( call_id: pjsua_call_id, target: *const pjsip_uri, e: *const pjsip_event) -> pjsip_redirect_op {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_call_redirected(call_id, target, e),
        _ => panic!("panic on_call_redirected"),
    }
}

// On REG state
unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_reg_state(acc_id),
        _ => panic!("panic on_reg_state"),
    }
}

// On Incomming Subscribe
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
        Some(ref mut sipcore) => sipcore.callback_on_incoming_subscribe(acc_id, srv_pres, buddy_id, from, rdata, code, reason, msg_data),
        _ => panic!("panic on_incoming_subscribe"),
    }
}

// On Buddy State
unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_buddy_state(buddy_id),
        _ => panic!("panic on_buddy_state"),
    }
}

// On Buddy evsub state
unsafe extern "C" fn on_buddy_evsub_state(buddy_id: pjsua_buddy_id, sub: *mut pjsip_evsub, event: *mut pjsip_event) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_buddy_evsub_state(buddy_id, sub, event),
        _ => panic!("panic on_buddy_evsub_state"),
    }
}

// On Pager
unsafe extern "C" fn on_pager(
    call_id: pjsua_call_id,
    from: *const pj_str_t,
    to: *const pj_str_t,
    contact: *const pj_str_t,
    mime_type: *const pj_str_t,
    body: *const pj_str_t,
) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_pager(call_id, from, to, contact, mime_type, body),
        _ => panic!("panic on_pager"),
    }
}

// On Typing
unsafe extern "C" fn on_typing(
    call_id: pjsua_call_id,
    from: *const pj_str_t,
    to: *const pj_str_t,
    contact: *const pj_str_t,
    is_typing: pj_bool_t,
) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_typing(call_id, from, to, contact, is_typing),
        _ => panic!("panic on_typing"),
    }
}

// On Call transfer status
unsafe extern "C" fn on_call_transfer_status(
    call_id: pjsua_call_id,
    st_code: c_int,
    st_text: *const pj_str_t,
    final_: pj_bool_t,
    p_cont: *mut pj_bool_t,
) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_call_transfer_status(call_id, st_code, st_text, final_, p_cont),
        _ => panic!("panic on_call_transfer_status"),
    }
}

// On Call replaced
unsafe extern "C" fn on_call_replaced(old_call_id: pjsua_call_id, new_call_id: pjsua_call_id) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_call_replaced(old_call_id, new_call_id),
        _ => panic!("panic on_call_replaced"),
    }
}

// On NAT detect
unsafe extern "C" fn on_nat_detect(res: *const pj_stun_nat_detect_result) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_nat_detect(res),
        _ => panic!("panic on_nat_detect"),
    }
}

// On MWI info
unsafe extern "C" fn on_mwi_info(acc_id: pjsua_acc_id, mwi_info: *mut pjsua_mwi_info) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_mwi_info(acc_id, mwi_info),
        _ => panic!("panic on_mwi_info"),
    }
}

// On Transport state
unsafe extern "C" fn on_transport_state(tp: *mut pjsip_transport, state: pjsip_transport_state, info: *const pjsip_transport_state_info) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_transport_state(tp, state, info),
        _ => panic!("panic on_transport_state"),
    }
}

// On ICE transport error
unsafe extern "C" fn on_ice_transport_error(index: c_int, op: pj_ice_strans_op, status: pj_status_t, param: *mut c_void) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_ice_transport_error(index, op, status, param),
        _ => panic!("panic on_ice_transport_error"),
    }
}

// Sound device operation
unsafe extern "C" fn on_snd_dev_operation(operation: c_int) -> pj_status_t {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_snd_dev_operation(operation),
        _ => panic!("pnaic on_snd_dev_operation"),
    }
}

// Call media event
unsafe extern "C" fn on_call_media_event(call_id: pjsua_call_id, med_idx: c_uint, event: *mut pjmedia_event) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_call_media_event(call_id, med_idx, event),
        _ => panic!("panic on_call_media_event"),
    }
}

// IP change progress
unsafe extern "C" fn on_ip_change_progress(op: pjsua_ip_change_op,status: pj_status_t,info: *const pjsua_ip_change_op_info) {
    match SIP_CORE {
        Some(ref mut sipcore) => sipcore.callback_on_ip_change_progress(op, status, info),
        _ => panic!("panic on_ip_change_progress"),
    }
}


