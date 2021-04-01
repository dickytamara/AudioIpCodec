
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

use super::pjsua;
use super::pjmedia;
use super::pjsip;
use std::ptr;
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void, c_uint, c_char};
use super::SIPInviteState;


pub static mut SIP_CORE: Option<SIPCore> = None;
pub static mut CURRENT_CALL: Option<pjsua_call_id> = None;

// todo create tune parameter for codec
// because default pjsua only support mono channels
// so we need create opus tune parameter.
// opus have bad inbandfec (forward error correction)
// fix with disable inbandfec

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
    input_dev: i32,
    output_dev: i32,
    input_latency: u32,
    output_latency: u32,
    auto_play_hangup: bool,
    duration: u32,
    current_call: i32,
    aud_cnt: u32,
    auto_answer: u32,
    events: SIPCoreEvents
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
        // create default data
        pjsua::create().unwrap();

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
            events: SIPCoreEvents::new()
        };

        sip_core
    }

    pub fn start(&mut self) {

        self.pool = pjsua::pool_create("ipcodec");

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
        pjsua::init(
            &mut self.app_config,
            &mut self.log_config,
            &mut self.media_config.get_context()
        ).unwrap();

        // pjsip endpoint for unhadled error
        self.default_handler.priority = (PJSIP_MOD_PRIORITY_APPLICATION + 99) as i32;
        self.default_handler.name = pj_str_t::from_string(String::from("mod-default-handler"));
        self.default_handler.on_rx_request = Some(SIPCore::on_rx_request);
        unsafe {
            let status = pjsip_endpt_register_module(
                pjsua::get_pjsip_endpt(),
                &mut self.default_handler as *mut _,
            );
            if status != 0 {
                panic!("cant register module");
            }
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



        if let Err(e) = pjsua::start() {
            pjsua::perror("sip_core.rs", "can't start pjsua.", e);
        }

        // we don't need add account for this state
        // so we create dynamicaly in addition
        self.accounts.set_rtp_config(self.transports.get_rtp_config());
        self.accounts.set_reg_retry_interval(300);
        self.accounts.set_reg_first_retry_interval(60);
    }

    pub fn deinit(&mut self) {
        pjsua::pool_release(self.pool);
        pjsua::destroy().unwrap();
    }

    pub fn call(&self, call_addr: &str) {
        unsafe{

            let mut msg_data = pjsua_msg_data::new();
            pjsua::msg_data_init(&mut msg_data);

            let default_acc = self.accounts.get_default();
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

    pub fn hangup(&self) {
        self.calls.hangup_all();
    }

    pub fn call_account(&self) {

    }



    pub fn ringback_start(&self, call_id: pjsua_call_id) {}

    pub fn ring_stop(&self, call_id: &pjsua_call_id) {
        // ring stop on incomming call
    }

    pub fn ring_start(&self, call_id: pjsua_call_id) {
        // ring start on incomming call
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
    }

    pub fn callback_on_stream_destroyed(&self, call_id: pjsua_call_id, strm: *mut pjmedia_stream, stream_idx: c_uint) {

    }

    pub fn callback_on_call_media_state(&mut self, call_id: pjsua_call_id) {
        println!("OnCallMediaState");

        let mut has_error = false;

        let call = SIPCall::from(call_id);
        let call_info = call.get_info().unwrap();

        for mi in 0..call_info.media_cnt {

            let call_media_info = call_info.media[mi as usize];
            let media_type = pjmedia::type_name(call_media_info.type_);

            let status_name = match call_media_info.status {
                PJSUA_CALL_MEDIA_NONE => "None",
                PJSUA_CALL_MEDIA_ACTIVE => "Active",
                PJSUA_CALL_MEDIA_LOCAL_HOLD => "Local hold",
                PJSUA_CALL_MEDIA_REMOTE_HOLD => "Remote hold",
                PJSUA_CALL_MEDIA_ERROR => "Error",
                _ => "Error"
            };

            match call_media_info.type_ {
                PJMEDIA_TYPE_NONE |
                PJMEDIA_TYPE_VIDEO |
                PJMEDIA_TYPE_APPLICATION |
                PJMEDIA_TYPE_UNKNOWN => has_error = true ,
                PJMEDIA_TYPE_AUDIO => self.on_call_audio_state(&call_info, mi, &mut has_error) ,
                _ => has_error = true
            }

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

        let mut opt = self.calls.get_call_opt();
        opt.aud_cnt = self.aud_cnt;

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
            pjsua::get_pjsip_endpt(),
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
        let mut tdata: *mut pjsip_tx_data = &mut pjsip_tx_data::new() as *mut _;
        let status_code: pjsip_status_code;
        let status: pj_status_t;

        // let mut rdata = rdata;
        let msg = (*rdata).msg_info.msg;
        let method = (*msg).line.req.method;

        if pjsip::method_cmp(&method, &pjsip_ack_method) == 0 {
            return PJ_TRUE as pj_status_t;
        }

        if pjsip::method_cmp(&method , &pjsip_register_method) == 0 {
            // call simple registrar pjsip_tx_data
            simple_registrar(rdata as *mut _);
            return PJ_TRUE as pj_status_t;
        }

        let mmethod = pjsip_notify_method;
        if pjsip::method_cmp(&method, &mmethod) == 0 {
            status_code = pjsip_status_code_PJSIP_SC_BAD_REQUEST;
        } else {
            status_code = pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED;
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

        if status_code == pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED {


            let cap_hdr = pjsip_endpt_get_capability(
                pjsua_get_pjsip_endpt(),
                pjsip_hdr_e_PJSIP_H_ALLOW as i32,
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

}

