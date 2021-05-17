use pjproject::{pjsip_ua::SIPInvState, pjsua::{buddy::UABuddy, call::UACall}, prelude::*, utils::{boolean_to_pjbool, check_boolean}};
use pjproject::prelude::AutoDefault;
use pjproject::pj::*;
use pjproject::pjsip_simple::*;
use pjproject::pjsip::*;
use pjproject::pjsua::*;
use pjproject::pjmedia::*;
use pjproject::pjnath::*;

use pjproject::pjsua::transport::*;
use pjproject::pjsua::account::*;
use pjproject::pjsua::media::*;

use pjproject::pjsua;



// use std::ptr;
// use std::ffi::{CString, CStr};
use std::{cell::{Cell, RefCell}, os::raw::{c_void, c_uint}, rc::Rc};
use std::convert::TryFrom;

// thread safe use Arc<Mutex<SIPCore>>
pub static mut SIP_CORE: Option<SIPCore> = None;
pub static mut CURRENT_CALL: Option<i32> = None;

pub struct SIPCore {
    pub ua_config: Rc<RefCell<UAConfig>>,
    pub log_config: Rc<RefCell<UALoggingConfig>>,
    pub media_config: Rc<RefCell<UAMediaConfig>>,
    pub default_transport_config: UATransportConfig,
    pub default_rtp_config: UATransportConfig,
    def_pool: Rc::<Cell<Option<Box::<*mut PJPool>>>>,
    module: Rc<RefCell<SIPModule>>,
    no_udp: Rc<Cell<bool>>,
    no_tcp: Rc<Cell<bool>>,
    use_ipv6: Rc<Cell<bool>>,
    transport_udp: Rc<Cell<Option<UATransport>>>,
    transport_tcp: Rc<Cell<Option<UATransport>>>,
    transport_udp6: Rc<Cell<Option<UATransport>>>,
    transport_tcp6: Rc<Cell<Option<UATransport>>>,
    redir_op: SIPRedirectOp,
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    auto_play_hangup: bool,
    duration: u32,
    current_call: Rc<Cell<Option<UACall>>>,
    auto_answer: u32,
    events: SIPCoreEvents,
    no_refersub: bool,
    compact_form: bool,
}
// struct to hold invite event function
struct SIPCoreEvents {
    invite: Box<dyn Fn(SIPInvState)>,
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
    fn connect_invite<F: Fn(SIPInvState) + 'static> (&mut self, f: F);
    fn connect_incoming_call<F: Fn() + 'static> (&mut self, f: F);
}

impl SIPCore {

    pub fn new() -> Self {
        let mut sipcore = Self {
            ua_config: Rc::new(RefCell::new(UAConfig::default())),
            log_config: Rc::new(RefCell::new(UALoggingConfig::default())),
            media_config: Rc::new(RefCell::new(UAMediaConfig::default())),
            default_transport_config: UATransportConfig::default(),
            default_rtp_config: UATransportConfig::default(),
            def_pool: Rc::new(Cell::new(None)),
            module: Rc::new(RefCell::new(SIPModule::new())),
            transport_udp: Rc::new(Cell::new(None)),
            transport_tcp: Rc::new(Cell::new(None)),
            transport_udp6: Rc::new(Cell::new(None)),
            transport_tcp6: Rc::new(Cell::new(None)),
            redir_op: SIPRedirectOp::AcceptReplace,
            input_dev: -1,
            output_dev: -2,
            input_latency: 100,
            output_latency: 140,
            auto_play_hangup: false,
            duration: 0,
            current_call: Rc::new(Cell::new(None)),
            auto_answer: 0,
            events: SIPCoreEvents::new(),
            no_refersub: false,
            compact_form: false,
            no_udp: Rc::new(Cell::new(false)),
            no_tcp: Rc::new(Cell::new(false)),
            use_ipv6: Rc::new(Cell::new(false)),
        };

        // default ua config
        sipcore.ua_config.borrow_mut().set_max_calls(1);
        sipcore.ua_config.borrow_mut().set_force_lr(true);
        sipcore.ua_config.borrow_mut().set_user_agent(String::from("IpCodec"));

        // set default event
        sipcore.ua_config.borrow_mut().cb.on_call_state = Some(on_call_state);
        sipcore.ua_config.borrow_mut().cb.on_stream_destroyed = Some(on_stream_destroyed);
        sipcore.ua_config.borrow_mut().cb.on_call_media_state = Some(on_call_media_state);
        sipcore.ua_config.borrow_mut().cb.on_incoming_call = Some(on_incoming_call);
        sipcore.ua_config.borrow_mut().cb.on_call_redirected = Some(on_call_redirected);
        sipcore.ua_config.borrow_mut().cb.on_dtmf_digit2 = Some(on_dtmf_digit2);
        sipcore.ua_config.borrow_mut().cb.on_reg_state = Some(on_reg_state);
        sipcore.ua_config.borrow_mut().cb.on_incoming_subscribe = Some(on_incoming_subscribe);
        sipcore.ua_config.borrow_mut().cb.on_buddy_state = Some(on_buddy_state);
        sipcore.ua_config.borrow_mut().cb.on_buddy_evsub_state = Some(on_buddy_evsub_state);
        sipcore.ua_config.borrow_mut().cb.on_pager = Some(on_pager);
        sipcore.ua_config.borrow_mut().cb.on_typing = Some(on_typing);
        sipcore.ua_config.borrow_mut().cb.on_call_transfer_status = Some(on_call_transfer_status);
        sipcore.ua_config.borrow_mut().cb.on_call_replaced = Some(on_call_replaced);
        sipcore.ua_config.borrow_mut().cb.on_nat_detect = Some(on_nat_detect);
        sipcore.ua_config.borrow_mut().cb.on_mwi_info = Some(on_mwi_info);
        sipcore.ua_config.borrow_mut().cb.on_transport_state = Some(on_transport_state);
        sipcore.ua_config.borrow_mut().cb.on_ice_transport_error = Some(on_ice_transport_error);
        sipcore.ua_config.borrow_mut().cb.on_snd_dev_operation = Some(on_snd_dev_operation);
        sipcore.ua_config.borrow_mut().cb.on_call_media_event = Some(on_call_media_event);
        sipcore.ua_config.borrow_mut().cb.on_ip_change_progress = Some(on_ip_change_progress);

        // default media config
        sipcore.media_config.borrow_mut().set_channel_count(ConfigChannel::Stereo);
        sipcore.media_config.borrow_mut().set_clock_rate(ClockRate::ClockRate48000);
        sipcore.media_config.borrow_mut().set_snd_clock_rate(ClockRate::ClockRate48000);
        sipcore.media_config.borrow_mut().set_quality(EncodingQuality::Level10);
        sipcore.media_config.borrow_mut().set_ec_options(MediaEchoFlag::Default);
        sipcore.media_config.borrow_mut().set_ec_tail_len(0);

        // default log level
        sipcore.log_config.borrow_mut().set_level(0);

        // default transport config
        sipcore.default_transport_config.set_port(5060);
        sipcore.default_rtp_config.set_port(4000);



        sipcore
    }

    pub fn init(&self) {

        pjsua::create().unwrap();
        self.def_pool.set(Some(Box::new(pjsua::pool_create("ipcodec_app"))));

        // register sub module for unhandeled error
        // self.module.set_priority((PJSIP_MOD_PRIORITY_APPLICATION + 99) as i32);
        let mut module = self.module.borrow_mut();
        module.set_priority(SIPModulePriority::Application);
        module.set_name(String::from("mod-default-handler"));
        module.connect_on_rx_request(Some(on_rx_request));
        SIPModule::register(&mut module);

        pjsua::init(
            &mut self.ua_config.borrow_mut(),
            &mut self.log_config.borrow_mut(),
            &mut self.media_config.borrow_mut())
        .unwrap();

        // check if setting not have at least 1 protocol
        if self.no_udp.get() & self.no_tcp.get() {
            self.no_udp.set(false);
        }

        // Initialize UDP Transport and local account
        if !self.no_udp.get() {
            let tp = UATransport::new(SIPTransportType::Udp, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_udp.set(Some(tp));
        }

        // initialize TCP transport and local account
        if !self.no_tcp.get() {
            let tp = UATransport::new(SIPTransportType::Tcp, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_tcp.set(Some(tp));
        }

        // initialize UDPv6 and local account
        if self.use_ipv6.get() & !self.no_udp.get() {
            let tp = UATransport::new(SIPTransportType::UdpV6, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            config.set_ipv6_media_use(self.use_ipv6.get());
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_udp6.set(Some(tp));
        }

        // initialize TCPv6 and local account
        if self.use_ipv6.get() & !self.no_tcp.get() {
            let tp = UATransport::new(SIPTransportType::TcpV6, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            config.set_ipv6_media_use(self.use_ipv6.get());
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_tcp6.set(Some(tp));
        }

        // self.media_config.init();
        UASound::default().set_snd_dev(self.input_dev, self.output_dev).unwrap();
        pjsua::start().unwrap();
    }

    pub fn restart(&self) {
        self.stop();
        self.init();
    }

    pub fn stop(&self) {
        pjsua::destroy().unwrap();
        // self.ua_config.deinit();
    }

    pub fn call(&self, call_addr: &str) {
        let account = UAAccount::default();
        if account.is_valid() {
            match account.call(call_addr.to_string(), None, None) {
                Ok(call) => self.current_call.set(Some(call)),
                Err(e) => ()
            }
        }
    }

    pub fn call_hangup(&self) {

        // if self.current_call == -1 {
        //     return;
        // }

        // let calls = SIPCalls::new();

        // if calls.get_count() > 1 {
        //     calls.hangup_all();
        // } else {
        //     let call = SIPCall::from(self.current_call);
        //     call.hangup(0, None, None);
        // }

    }

    pub fn call_answer(&self) {

        // if self.current_call == -1 {
        //     println!("call_answer: no active call");
        //     return;
        // }

        // let calls = SIPCalls::new();
        // let current_call = SIPCall::from(self.current_call);
        // let mut msg_data = pjsua_msg_data::new();
        // let mut call_opt = calls.get_call_opt();

        // current_call.answer2(&mut call_opt, 200, None, Some(&mut msg_data));
    }

    pub fn ringback_start(&self, call_id: &i32) {
        // start procedure ringback
    }

    pub fn ring_stop(&self, call_id: &i32) {
        // ring stop on incomming call
    }

    pub fn ring_start(&self, call_id: &i32) {
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

    pub fn set_compact_form(&mut self, value: bool) {
        self.compact_form = value;
    }

    pub fn on_call_audio_state(&self, ci: &UACallInfo, mi: u32, has_error: &mut bool) {

        let media = &ci.media[mi as usize];

        match media.get_status() {
            CallMediaStatus::None => {}
            CallMediaStatus::LocalHold => {}
            CallMediaStatus::Error => {}
            CallMediaStatus::Active |
            CallMediaStatus::RemoteHold => {
                unsafe {
                    let call_conf_slot = media.stream.aud.as_ref().conf_slot;

                    for call in UACall::enum_calls().unwrap().iter() {
                        if ci.get_id() == call.get_info().unwrap().get_id() { continue; }
                        if call.has_media() {
                            UAConf::connect(call_conf_slot, call.get_info().unwrap().get_id()).unwrap();
                            UAConf::connect(call.get_info().unwrap().get_id(), call_conf_slot).unwrap();
                        }
                    }

                    UAConf::connect(call_conf_slot, 0).unwrap();
                    UAConf::connect(0, call_conf_slot).unwrap();
                }
            }
        };

        // if media.status == PJSUA_CALL_MEDIA_ACTIVE ||
        // media.status == PJSUA_CALL_MEDIA_REMOTE_HOLD {
        //     let call_conf_slot: pjsua_conf_port_id;



        //     let mut call_ids: [pjsua_call_id; 32] = [-1; 32];
        //     let mut call_cnt = 32u32;

        //     pjsua::enum_calls(&mut call_ids, &mut call_cnt).unwrap();

        //     unsafe {
        //         call_conf_slot = media.stream.aud.as_ref().conf_slot;

        //         for idx in 0..call_cnt as usize {
        //             if call_ids[idx] == ci.id { continue; }
        //             if  pjsua_call_has_media(call_ids[idx].clone()) == PJ_FALSE as pj_bool_t { continue; }

        //             // connect rx
        //             pjsua_conf_connect(call_conf_slot,
        //                 pjsua_call_get_conf_port(call_ids[idx])
        //             );

        //             // connect tx
        //             pjsua_conf_connect(pjsua_call_get_conf_port(call_ids[idx]),
        //                 call_conf_slot
        //             );
        //         }
        //     }
        //     pjsua::conf_connect(call_conf_slot, 0).unwrap();
        //     pjsua::conf_connect(0, call_conf_slot).unwrap();

        // }

    }

    // pub fn callback_on_call_state(&self, call_id: i32, e: *mut SIPEvent) {
    pub fn callback_on_call_state(&self, call: UACall, e: *mut SIPEvent) {

        let state_message: String = match call.get_info().unwrap().get_state() {
            SIPInvState::Null => { String::from("NULL") }
            SIPInvState::Calling => { String::from("CALLING") }
            SIPInvState::Incoming => { String::from("INCOMING") }
            SIPInvState::Early => { String::from("EARLY") }
            SIPInvState::Connecting => { String::from("CONNECTING") }
            SIPInvState::Confirmed => { String::from("CONFIRMED") }
            SIPInvState::Disconnected => { String::from("DISCONNECTED") }
        };

        println!("INVSTATE [{}]", state_message);
        // todo ringing mecanism

        // call event for non internal
        (self.events.invite)(call.get_info().unwrap().get_state());

        // match call_info.state {
        //     PJSIP_INV_STATE_DISCONNECTED => {
        //         if self.current_call == call_id  {
        //             if call_info.state == PJSIP_INV_STATE_DISCONNECTED {
        //                 self.current_call = -1;
        //             }
        //         } else if self.current_call != -1 {
        //             let call = SIPCall::from(call_id);
        //             call.hangup(0, None, None);
        //         }
        //     },
        //     _ => {
        //         if self.current_call == -1 {
        //             self.current_call = call_id;
        //         }
        //     }
        // }
    }

    pub fn callback_on_stream_destroyed(&self, call: UACall, strm: *mut MediaStream, stream_idx: c_uint) {
        // println!("OnStreamDestroyed: CallID {}, StreamIDX {}", call_id, stream_idx);
    }

    pub fn callback_on_call_media_state(&self, call: UACall) {
        println!("OnCallMediaState");

        let mut has_error = false;
        let call_info = call.get_info().unwrap();

        for mi in 0..call_info.media_cnt {

            let call_media_info = &call_info.media[mi as usize];
            match call_media_info.get_type_() {
                MediaType::Audio => {
                    self.on_call_audio_state(&call_info, mi, &mut has_error);
                },
                _ => has_error = true
            }
            // match call_media_info.type_ {
            //     PJMEDIA_TYPE_NONE |
            //     PJMEDIA_TYPE_VIDEO |
            //     PJMEDIA_TYPE_APPLICATION |
            //     PJMEDIA_TYPE_UNKNOWN => has_error = true ,
            //     PJMEDIA_TYPE_AUDIO => self.on_call_audio_state(&call_info, mi, &mut has_error) ,
            //     _ => has_error = true
            // }

            // println!("sipua.rs Call {} media {} [type={}], status is {}",
            //     call_info.id, mi, media_type, status_name);
        }

        if has_error {
            let reason = format!("Media failed");
            call.hangup(SIPStatusCode::IntenalServerError, Some(reason), None).unwrap();
        }
    }

    pub fn callback_on_incomming_call(&self, account: UAAccount, call_id: UACall, rdata: *mut SIPRxData) {

        // let sip_account = SIPAccount::from(acc_id).unwrap();
        // let call = SIPCall::from(call_id);

        // self.current_call = call_id;

        // let calls = SIPCalls::new();
        // let mut opt = calls.get_call_opt();

        // // outer level
        // println!("INVSTATE [INCOMING]");
        // (self.events.invite)(SIPInviteState::Incoming);

        // if self.auto_answer > 0 {
        //     call.answer2(&mut opt, 200, None, None);
        // }

    }

    pub fn callback_on_dtmf_digit2(&self, call: UACall, info: *const UADtmfInfo) {
        // unsafe {
        //     let info = info.as_ref().unwrap();
        //     let dtmf = match info.method {
        //         PJSUA_DTMF_METHOD_RFC2833 => "RFC2833",
        //         PJSUA_DTMF_METHOD_SIP_INFO => "SIP INFO",
        //         _ => "Unknown dtmf method",
        //     };

        //     println!("Incomming DTMF on call using method {}", dtmf);
        // }
    }

    pub fn callback_on_call_redirected(&self, call: UACall, target: *const SIPUri, e: *const SIPEvent) -> u32 {
        // println!("Call {} is being redirected", call_id);
        self.redir_op.into()
    }

    pub fn callback_on_reg_state(&self, acc_id: UAAccount) {
        // println!("reg_state {}", acc_id);
    }

    pub fn callback_on_incoming_subscribe(
        &self,
        acc: UAAccount,
        srv_pres: *mut UASrvPres,
        buddy: UABuddy,
        from: String,
        rdata: *mut SIPRxData,
        code: &mut SIPStatusCode,
        reason: &mut String,
        msg_data: *mut UAMsgData)
    {
        // Todo
        println!("On_incomming_subscribe")
    }

    pub fn callback_on_buddy_state(&self, buddy: UABuddy) {
        // let mut info: pjsua_buddy_info = pjsua_buddy_info::new();
        // pjsua::buddy_get_info(buddy_id, &mut info).unwrap();
    }

    pub fn callback_on_buddy_evsub_state(
        &self,
        buddy: UABuddy,
        sub: *mut SIPEvsub,
        event: *mut SIPEvent,
    ) {
        println!("Buddy subscription state");
    }

    pub fn callback_on_pager(
        &self,
        call: UACall,
        from: String,
        to: String ,
        contact: String,
        mime_type: String,
        body: String,
    ) {
        println!("OnPager");
    }

    pub fn callback_on_typing(
        &self,
        call: UACall,
        from: String,
        to: String,
        contact: String,
        is_typing: bool,
    ) {
        println!("IM indication.");
    }

    pub fn callback_on_call_transfer_status(
        &self,
        call: UACall,
        st_code: i32,
        st_text: String,
        final_: bool,
        p_cont: &mut bool,
    ) {
        // unsafe {
        //     println!("Call {} transfer status={}", call_id, st_code);
        //     if st_code / 100 == 2 {
        //         pjsua_call_hangup(
        //             call_id,
        //             PJSIP_SC_GONE,
        //             ptr::null() as *const _,
        //             ptr::null() as *const _,
        //         );
        //         *p_cont = PJ_FALSE as pj_bool_t;
        //     }
        // }
    }

    pub fn callback_on_call_replaced(
        &self,
        old_call: UACall,
        new_call: UACall,
    ) {
        // println!( "Call {} is being replaced by call {}", old_call_id, new_call_id);
    }

    pub fn callback_on_nat_detect(&self, res: *const STUNNatDetectResult) {
        // logging nat detect result this only for trouble shooting
        // unsafe {

        //     let nat_result = &*res;
        //     let status_str;
        //     let status_text;
        //     let nat_type_name;

        //     if nat_result.status == PJ_SUCCESS as pj_status_t {
        //         status_str = String::from("SUCCESS");
        //     } else {
        //         status_str = String::from("FAILED");
        //     }

        //     // get status text from nat result
        //     status_text = CStr::from_ptr(nat_result.status_text)
        //         .to_owned()
        //         .into_string()
        //         .expect("nat status_text fail");

        //     // get nat_type_name from nat result
        //     nat_type_name = CStr::from_ptr(nat_result.nat_type_name)
        //         .to_owned()
        //         .into_string()
        //         .expect("nat_type_name fail");

        //     println!("NAT detected result: [{}], status [{}], type: [{}]",
        //         status_str, status_text, nat_type_name
        //     );
        // }
    }

    pub fn callback_on_mwi_info(&self, account: UAAccount, mwi_info: *mut UAMwiInfo) {
        // unsafe {
        //     println!("Received MWI for acc_id {}", acc_id);
        //     let ctype = (*(*mwi_info).rdata).msg_info.ctype;

        //     if !ctype.is_null() {
        //         println!(
        //             "Content-Type: {} {}/ {} {}",
        //             (*ctype).media.type_.slen,
        //             CString::from_raw((*ctype).media.type_.ptr)
        //                 .into_string()
        //                 .expect("error"),
        //             (*ctype).media.subtype.slen,
        //             CString::from_raw((*ctype).media.subtype.ptr)
        //                 .into_string()
        //                 .expect("error")
        //         );
        //     }
        // }
    }

    pub fn callback_on_transport_state(
        &self,
        tp: *mut SIPTransport,
        state: SIPTransportState,
        info: *const SIPTransportStateInfo,
    ) {
        // unsafe {
        //     // transport
        //     let tp = &*tp;
        //     // transport type name TCP or UDP
        //     let type_name = CStr::from_ptr(tp.type_name)
        //             .to_owned()
        //             .into_string()
        //             .expect("tp.transport null info");

        //     // remote address name it can be server or ptp client
        //     let remote_name = tp.remote_name.host.to_string();

        //     // remote port
        //     let remote_port = tp.remote_name.port;

        //     let op_name: String = match state {
        //         PJSIP_TP_STATE_CONNECTED => String::from("CONNECTED"),
        //         PJSIP_TP_STATE_DISCONNECTED => String::from("DISCONNECTED"),
        //         PJSIP_TP_STATE_SHUTDOWN => String::from("SHUTDOWN"),
        //         PJSIP_TP_STATE_DESTROY => String::from("DESTROY"),
        //         _ => String::from("unknown")
        //     };

        //     println!("SIP [{}] transport is {} to {}:{}",
        //         type_name, op_name, remote_name, remote_port
        //     );
        // }
    }

    pub fn callback_on_ice_transport_error(
        &self,
        index: i32,
        op: IceStransOp,
        status: i32,
        param: *mut c_void,
    ) {
        // this only log print
        // let ice_trans_op_text: String;
        // let ice_status_text: String;

        // if status == PJ_SUCCESS as pj_status_t {
        //     ice_status_text = String::from("SUCCESS");
        // } else {
        //     ice_status_text = String::from("FAILURE");
        // }

        // match op {
        //     PJ_ICE_STRANS_OP_INIT => ice_trans_op_text = String::from("INIT"),
        //     PJ_ICE_STRANS_OP_NEGOTIATION => ice_trans_op_text = String::from("NEGOTIATION"),
        //     PJ_ICE_STRANS_OP_KEEP_ALIVE => ice_trans_op_text = String::from("KEEPALIVE"),
        //     PJ_ICE_STRANS_OP_ADDR_CHANGE => ice_trans_op_text = String::from("ADDRCHANGE"),
        //     _ => ice_trans_op_text = String::from("UNKNOWN OP")
        // }

        // println!("ICE [{}] status [{}], operation [{}]",
        //     index, ice_status_text, ice_trans_op_text,
        // );
    }

    pub fn callback_on_snd_dev_operation(&self, operation: i32) -> i32 {
        // println!("OnSndDevOperation");
        // let mut cap_dev = -1;
        // let mut play_dev = -1;
        // let op: String;

        // pjsua::get_snd_dev(&mut cap_dev, &mut play_dev).unwrap();

        // if operation > 0 {
        //     op = String::from("ON");
        // } else {
        //     op = String::from("OFF");
        // }

        // println!( "Turning sound device input {} output {} : {}",
        //     cap_dev, play_dev, op
        // );

        // PJ_SUCCESS as pj_status_t
        1
    }

    pub fn callback_on_call_media_event(
        &self,
        call: UACall,
        med_idx: u32,
        event: *mut MediaEvent)
    {
        // if pjsua has support video
        println!("Event {}", "skip");
    }

    pub fn callback_on_ip_change_progress(
        &self,
        op: UAIpChangeOp,
        status: i32,
        info: *const UAIpChangeOpInfo)
    {

        // unsafe {
        //     let mut acc_info: pjsua_acc_info = pjsua_acc_info::new();
        //     let mut tp_info: pjsua_transport_info = pjsua_transport_info::new();


        //     let status_text: String = match status as u32 {
        //         PJ_SUCCESS => String::from("SUCCESS"),
        //         _ => String::from("FAILURE")
        //     };

        //     if !info.is_null() {
        //         let info = &*info;
        //         pjsua_transport_get_info(info.lis_restart.as_ref().transport_id,
        //             &mut tp_info as *mut _
        //         );

        //         pjsua_acc_get_info(info.acc_shutdown_tp.as_ref().acc_id,
        //             &mut acc_info as *mut _
        //         );
        //     }

        //     let op_text = match op {
        //         PJSUA_IP_CHANGE_OP_NULL => String::from("NULL"),
        //         PJSUA_IP_CHANGE_OP_RESTART_LIS => String::from("RESTART_LISTENER"),
        //         PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP => String::from("ACC_SHUTDOWN_TP"),
        //         PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT => String::from("ACC_UPDATE_CONTACT"),
        //         PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS => String::from("ACC_HANGUP_CALLS"),
        //         PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS => String::from("ACC_REINVITE_CALLS"),
        //         PJSUA_IP_CHANGE_OP_COMPLETED => String::from("COMPLETED"),
        //         _ => String::from("UNKNOWN")
        //     };

        //     let base_str = format!("IPCHANGE [{}] op: [{}]",
        //         status_text, op_text
        //     );

        //     let mut log_str: String = String::new();

        //     if op == PJSUA_IP_CHANGE_OP_RESTART_LIS {
        //         log_str = format!("{} : restart transport {}", base_str, tp_info.info.to_string());
        //     }

        //     if op == PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP {
        //         log_str = format!("{} : transport shutdown for account {}",
        //             base_str,
        //             acc_info.acc_uri.to_string()
        //         );
        //     }

        //     if op == PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT {
        //         if (*info).acc_update_contact.as_ref().code != 0 {
        //             log_str = format!("{} : update contact for account {} code: [{}]",
        //                 base_str,
        //                 acc_info.acc_uri.to_string(),
        //                 (*info).acc_update_contact.as_ref().code
        //             );
        //         } else {
        //             log_str = format!("{} : update contact for account {}",
        //                 base_str,
        //                 acc_info.acc_uri.to_string(),
        //             );
        //         }
        //     }

        //     if op == PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS {
        //         log_str = format!("{} : hangup call for account {}, call_id[{}]",
        //             base_str,
        //             acc_info.acc_uri.to_string(),
        //             (*info).acc_hangup_calls.as_ref().call_id
        //         );
        //     }

        //     if op == PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS {
        //         log_str = format!("{} : reinvite call for account {}, call_id[{}]",
        //             base_str,
        //             acc_info.acc_uri.to_string(),
        //             (*info).acc_reinvite_calls.as_ref() .call_id
        //         );
        //     }

        //     if op == PJSUA_IP_CHANGE_OP_COMPLETED {
        //         log_str = format!("{} : done", base_str);
        //     }

        //     println!("{}", log_str);
        // }
    }

}

// trait to hold external event
// impl SIPCoreEventsExt for SIPCore {

//     fn connect_invite <F: FnMut(SIPInviteState) + 'static> (&mut self, f: F) {
//         self.events.invite = Box::new(f);
//     }

//     fn connect_incoming_call <F: FnMut() + 'static> (&mut self, f: F) {
//         self.events.incoming_call = Box::new(f);
//     }
// }

// fn simple_registrar(rdata: *mut pjsip_rx_data) {
//     println!("ON Simple Registrar");
//     unsafe {
//         let tdata: *const pjsip_tx_data = ptr::null();
//         let str_null: *const pj_str_t = ptr::null();
//         let status: pj_status_t;
//         let mut cnt: c_uint = 0;

//         status = pjsip_endpt_create_response(
//             pjsua_get_pjsip_endpt(),
//             rdata as *const _,
//             200,
//             str_null as *const _,
//             tdata as *mut _,
//         );
//         if status != PJ_SUCCESS as i32 {
//             return;
//         }

//         let exp: *const pjsip_expires_hdr = pjsip_msg_find_hdr(
//             (*rdata).msg_info.msg,
//             PJSIP_H_EXPIRES,
//             ptr::null_mut(),
//         ) as *const _;

//         // let rdata_ = &*rdata;
//         // let llist: pjsip_hdr = (*(*rdata).msg_info.msg).hdr;
//         // let mut h: *mut pjsip_hdr = (*(*rdata).msg_info.msg).hdr.next;
//         let llist = &(*(&*rdata).msg_info.msg).hdr;
//         let mut h= &*((*(&*rdata).msg_info.msg).hdr).next;

//         while h != &*llist.next {
//             if (*h as pjsip_hdr).type_ == (PJSIP_H_CONTACT as pjsip_hdr_e) {
//                 let c: *const pjsip_contact_hdr = h as *const pjsip_contact_hdr;
//                 let mut e: c_uint = (*c).expires;

//                 if e != 0xffffffff {
//                     if !exp.is_null() {
//                         e = (*exp).ivalue;
//                     } else {
//                         e = 3600;
//                     }
//                 }

//                 if e > 0 {
//                     let nc: *mut pjsip_contact_hdr =
//                         pjsip_hdr_clone((*tdata).pool, h as *const _) as *mut pjsip_contact_hdr;

//                     (*nc).expires = e;
//                     pj_list_insert_before((*tdata).msg as *mut _, nc as *mut _);
//                     cnt = cnt + 1;
//                 }
//                 h = (*h).next;
//             }
//         }

//         // todo review c code for this. it's c clasic problem
//         let srv: *mut pjsip_generic_string_hdr =
//             pjsip_generic_string_hdr_create((*tdata).pool, str_null, str_null);
//         // create server name
//         let tmp: CString = CString::new("Server").expect("cant create Server string");
//         (*srv).name = pj_str(tmp.as_ptr() as *mut c_char);
//         // create add description
//         let tmp: CString =
//             CString::new("IpCodec simple registrar").expect("cant create simple registrar");
//         (*srv).hvalue = pj_str(tmp.as_ptr() as *mut c_char);

//         pj_list_insert_before((*tdata).msg as *mut _, srv as *mut _);
//         let cb: pjsip_send_callback = None;
//         pjsip_endpt_send_response2(
//             pjsua::get_pjsip_endpt(),
//             rdata,
//             tdata as *mut _,
//             ptr::null_mut(),
//             None,
//         );
//     }
// }

unsafe extern "C" fn on_rx_request(rdata: *mut SIPRxData) -> i32 {
    println!("OnRxRequest");
    // base rx request handle undefined state.
    // let mut tdata: *mut pjsip_tx_data = &mut pjsip_tx_data::new() as *mut _;
    // let status_code: pjsip_status_code;
    // let status: pj_status_t;

    // // let mut rdata = rdata;
    // let msg = (*rdata).msg_info.msg;
    // let method = &(*msg).line.req.as_ref().method;

    // if pjsip::method_cmp(&method, &pjsip_ack_method) == 0 {
    //     return PJ_TRUE as pj_status_t;
    // }

    // if pjsip::method_cmp(&method , &pjsip_register_method) == 0 {
    //     // call simple registrar pjsip_tx_data
    //     simple_registrar(rdata as *mut _);
    //     return PJ_TRUE as pj_status_t;
    // }

    // let mmethod = &pjsip_simple_sys::pjsip_notify_method;
    // if pjsip::method_cmp(&method, &mmethod) == 0 {
    //     status_code = PJSIP_SC_BAD_REQUEST;
    // } else {
    //     status_code = PJSIP_SC_METHOD_NOT_ALLOWED;
    // }

    // status = pjsip_endpt_create_response(
    //     pjsua_get_pjsip_endpt(),
    //     rdata,
    //     status_code as c_int,
    //     ptr::null_mut() as *const _,
    //     &mut tdata as *mut _,
    // );

    // if status != PJ_SUCCESS as pj_status_t {
    //     return PJ_TRUE as pj_status_t;
    // }

    // let msg = (*tdata).msg;
    // let ahdr: *mut pjsip_hdr = &mut (*msg).hdr as *mut _;

    // if status_code == PJSIP_SC_METHOD_NOT_ALLOWED {


    //     let cap_hdr = pjsip_endpt_get_capability(
    //         pjsua_get_pjsip_endpt(),
    //         PJSIP_H_ALLOW as i32,
    //         ptr::null() as *const _,
    //     );

    //     if !cap_hdr.is_null() {
    //         let hdr_clone = pjsip_hdr_clone((*tdata).pool, cap_hdr as *const _) as *const pjsip_hdr;

    //         pj_list_insert_before(
    //             ahdr as *mut _ ,
    //             hdr_clone as *mut _,
    //         );
    //     }
    // }

    // // add user-agent header
    // let mut ua = pj_str_t::from_string(String::from("User-Agent"));
    // let mut agent = pj_str_t::from_string(String::from("IpCodec"));

    // let h = pjsip_generic_string_hdr_create(
    //     (*tdata).pool,
    //     &mut ua as *const _,
    //     &mut agent as *const _,
    // ) as *mut pjsip_hdr;

    // pj_list_insert_before(ahdr as *mut _, h as *mut _);

    // pjsip_endpt_send_response2(pjsua::get_pjsip_endpt(),
    //     rdata,
    //     tdata,
    //     ptr::null_mut(),
    //     None
    // );

    // PJ_TRUE as pj_status_t
    0
}

/// on_call_state(pjsua_call_id, pjsip_event)
unsafe extern "C" fn on_call_state(call_id: i32, e: *mut SIPEvent) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_call_state(UACall::from(call_id), e);
}

/// on_stream_destroyed(pjsua_call_id, pjmedia_stream, stream_idx)
unsafe extern "C" fn on_stream_destroyed(call_id: i32, strm: *mut MediaStream, stream_idx: u32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_stream_destroyed(UACall::from(call_id), strm, stream_idx);
}

/// on_incoming_call(acc_id, call_id, rdata)
unsafe extern "C" fn on_incoming_call( acc_id: i32, call_id: i32, rdata: *mut SIPRxData) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_incomming_call(UAAccount::from(acc_id), UACall::from(call_id), rdata)
}

/// on_call_media_state(call_id)
unsafe extern "C" fn on_call_media_state(call_id: i32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_call_media_state(UACall::from(call_id));
}

/// on_dtmf_digit2(call_id, info)
unsafe extern "C" fn on_dtmf_digit2(call_id: i32, info: *const UADtmfInfo) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_dtmf_digit2(UACall::from(call_id), info)
}

/// on_call_redirected(call_id, target, e) -> pjsip_redirect_op
unsafe extern "C" fn on_call_redirected( call_id: i32, target: *const SIPUri, e: *const SIPEvent) -> u32 {
    SIP_CORE.as_ref().unwrap()
    .callback_on_call_redirected(UACall::from(call_id), target, e)
}

/// on_reg_state(acc_id)
unsafe extern "C" fn on_reg_state(acc_id: i32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_reg_state(UAAccount::from(acc_id));
}

// On Incomming Subscribe
unsafe extern "C" fn on_incoming_subscribe(
    acc_id: i32,
    srv_pres: *mut UASrvPres,
    buddy_id: i32,
    from: *const PJStr,
    rdata: *mut SIPRxData,
    code: *mut u32,
    reason: *mut PJStr,
    msg_data: *mut UAMsgData,
) {
    // convert to enum
    let mut sip_code = SIPStatusCode::try_from(*code.clone()).expect("Error Convert SIPStatusCode");
    let mut reason_str = from.as_ref().unwrap().to_string();


    SIP_CORE.as_ref().unwrap()
    .callback_on_incoming_subscribe(
        UAAccount::from(acc_id),
        srv_pres,
        UABuddy::from(buddy_id),
        from.as_ref().unwrap().to_string(),
        rdata,
        &mut sip_code,
        &mut reason_str,
        msg_data
    );

    // convert back and return.
    *code = sip_code.into();
    *reason = PJStr::from_string(reason_str);
}

/// on_buddy_state(buddy_id)
unsafe extern "C" fn on_buddy_state(buddy_id: i32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_buddy_state(UABuddy::from(buddy_id));
}

/// on_buddy_evsub_state(buddy_id, sub, event)
unsafe extern "C" fn on_buddy_evsub_state(buddy_id: i32, sub: *mut SIPEvsub, event: *mut SIPEvent) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_buddy_evsub_state(UABuddy::from(buddy_id), sub, event);
}

// On Pager
unsafe extern "C" fn on_pager(
    call_id: i32,
    from: *const PJStr,
    to: *const PJStr,
    contact: *const PJStr,
    mime_type: *const PJStr,
    body: *const PJStr,
) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_pager(
        UACall::from(call_id),
        from.as_ref().unwrap().to_string(),
        to.as_ref().unwrap().to_string(),
        contact.as_ref().unwrap().to_string(),
        mime_type.as_ref().unwrap().to_string(),
        body.as_ref().unwrap().to_string()
    );
}

// on Typing
unsafe extern "C" fn on_typing(
    call_id: i32,
    from: *const PJStr,
    to: *const PJStr,
    contact: *const PJStr,
    is_typing: i32,
) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_typing(
        UACall::from(call_id),
        from.as_ref().unwrap().to_string(),
        to.as_ref().unwrap().to_string(),
        contact.as_ref().unwrap().to_string(),
        check_boolean(is_typing)
    );
}

// On Call transfer status
unsafe extern "C" fn on_call_transfer_status(
    call_id: i32,
    st_code: i32,
    st_text: *const PJStr,
    final_: i32,
    p_cont: *mut i32,
) {
    let mut pcont = check_boolean(*p_cont);

    SIP_CORE.as_ref().unwrap()
    .callback_on_call_transfer_status(
        UACall::from(call_id),
        st_code,
        st_text.as_ref().unwrap().to_string(),
        check_boolean(final_),
        &mut pcont
    );

    *p_cont = boolean_to_pjbool(pcont);
}

// (old_call_id: pjsua_call_id, new_call_id: pjsua_call_id)
unsafe extern "C" fn on_call_replaced(old_call_id: i32, new_call_id: i32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_call_replaced(
        UACall::from(old_call_id),
        UACall::from(new_call_id)
    );
}

// On NAT detect
unsafe extern "C" fn on_nat_detect(res: *const STUNNatDetectResult) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_nat_detect(res);
}

// On MWI info
unsafe extern "C" fn on_mwi_info(acc_id: i32, mwi_info: *mut UAMwiInfo) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_mwi_info(UAAccount::from(acc_id), mwi_info);
}

// On Transport state
unsafe extern "C" fn on_transport_state(tp: *mut SIPTransport, state: u32, info: *const SIPTransportStateInfo) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_transport_state(
        tp,
        SIPTransportState::try_from(state).unwrap(),
        info
    );
}

// On ICE transport error
unsafe extern "C" fn on_ice_transport_error(index: i32, op: u32, status: i32, param: *mut c_void) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_ice_transport_error(
        index,
        IceStransOp::try_from(op).unwrap(),
        status,
        param
    );
}

// Sound device operation
unsafe extern "C" fn on_snd_dev_operation(operation: i32) -> i32 {
    SIP_CORE.as_ref().unwrap()
    .callback_on_snd_dev_operation(operation)
}

// Call media event
unsafe extern "C" fn on_call_media_event(call_id: i32, med_idx: u32, event: *mut MediaEvent) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_call_media_event(
        UACall::from(call_id),
        med_idx,
        event
    );
}

// IP change progress
unsafe extern "C" fn on_ip_change_progress(op: u32, status: i32, info: *const UAIpChangeOpInfo) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_ip_change_progress(
        UAIpChangeOp::try_from(op).unwrap(),
        status,
        info
    );
}
