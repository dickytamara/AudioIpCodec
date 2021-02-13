#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]

// default
use super::pjdefault::AutoCreate;
use super::pjsip::PjsipModuleCallback;
use super::pjsua::PjsuaCallback;
use super::pjsua_sys::*;
use std::ffi::CString;
use std::ops::Drop;
use std::os::raw::{c_void, c_int, c_uint, c_char};
use std::ptr;


pub type SIPAccount = pjsua_acc_config;
pub type SIPBuddy = pjsua_buddy_config;

pub struct SIPAudio {}
pub struct SIPIMessages {}
pub struct SIPMedia {}
pub struct SIPPressence {}
pub struct SIPCall {}

// pjsua_acc_get_count
// pjsua_acc_is_valid
// pjsua_acc_set_default
// pjsua_acc_get_default
// pjsua_acc_config_dup
// pjsua_acc_add
// pjsua_acc_add_local
// pjsua_acc_set_user_data
// pjsua_acc_get_user_data
// pjsua_acc_del
// pjsua_acc_get_config
// pjsua_acc_modify
// pjsua_acc_set_online_status
// pjsua_acc_set_online_status2
// pjsua_acc_set_registration
// pjsua_acc_get_info
// pjsua_enum_accs
// pjsua_acc_enum_info
// pjsua_acc_find_for_outgoing
// pjsua_acc_find_for_incoming
// pjsua_acc_create_request
// pjsua_acc_create_uac_contact
// pjsua_acc_create_uas_contact
// pjsua_acc_set_transport
//
trait Account {
    fn get_count() -> u32;
}

// pjsua_call_has_media
// pjsua_call_get_conf_port
// pjsua_call_get_stream_info
// pjsua_call_get_stream_stat
// pjsua_call_dial_dtmf
//
// pjsua_snd_dev_param_default
// pjsua_conf_connect_param_default
// pjsua_conf_get_max_port
// pjsua_conf_get_active_ports
// pjsua_enum_conf_ports
// pjsua_conf_get_port_info
// pjsua_conf_add_port
// pjsua_conf_remove_port
// pjsua_conf_connect
// pjsua_conf_connectpjsua_acc_info
// pjsua_conf_disconnect
// pjsua_conf_adjust_tx_level
// pjsua_conf_adjust_rx_level
// pjsua_conf_get_signal_level
// pjsua_player_create
// pjsua_playlist_create
// pjsua_player_get_conf_port
// pjsua_player_get_info
// pjsua_player_get_pos
// pjsua_player_set_pos
// pjsua_player_destroy
// pjsua_recorder_create
// pjsua_recorder_get_conf_port
// pjsua_recorder_get_port
// pjsua_recorder_destroy
//
// sound devices
// pjsua_enum_aud_devs
// pjsua_enum_snd_devs
// pjsua_set_snd_dev
// pjsua_set_snd_dev2
// pjsua_get_snd_dev
// pjsua_set_null_snd_dev
// pjsua_set_no_snd_dev
// pjsua_set_ec
// pjsua_set_ec_tail
// pjsua_get_ec_stat
// pjsua_snd_is_active
// pjsua_snd_get_setting
// pjsua_ext_snd_dev_create
// pjsua_ext_snd_dev_destroy
// pjsua_ext_snd_dev_get_snd_port
// pjsua_ext_snd_dev_get_conf_port
//
trait Audio {}

// pjsua_im_send
// pjsua_im_typing
trait IMessages {}

// pjsua_enum_codecs
// pjsua_codec_set_priority
// pjsua_codec_get_param
// pjsua_codec_set_param
trait Media {}

// pjsua_get_buddy_count
// pjsua_buddy_find
// pjsua_buddy_is_valid
// pjsua_enum_buddies
// pjsua_buddy_get_info
// pjsua_buddy_set_user_data
// pjsua_buddy_get_user_data
// pjsua_buddy_add
// pjsua_buddy_del
// pjsua_buddy_subscribe_pres
// pjsua_pres_dump
// pjsua_pres_notify
trait Pressence {}

// pjsua_call_get_max_count
// pjsua_call_get_count
// pjsua_enum_calls
// pjsua_call_setting_default
// pjsua_call_send_dtmf_param_default
// pjsua_call_make_call
// pjsua_call_is_active
// pjsua_call_get_info
// pjsua_call_remote_has_cap
// pjsua_call_set_user_data
// pjsua_call_get_user_data
// pjsua_call_get_rem_nat_type
// pjsua_call_get_med_transport_info
// pjsua_call_answer
// pjsua_call_answer2
// pjsua_call_answer_with_sdp
// pjsua_call_hangup
// pjsua_call_process_redirect
// pjsua_call_set_hold
// pjsua_call_set_hold2
// pjsua_call_reinvite
// pjsua_call_reinvite2
// pjsua_call_update
// pjsua_call_update2
// pjsua_call_xfer
// pjsua_call_xfer_replaces
// pjsua_call_send_dtmf
// pjsua_call_send_im
// pjsua_call_send_typing_ind
// pjsua_call_send_request
// pjsua_call_hangup_all
trait Call {}

// pjsua_call_dump
trait Dump {}

// pjsua_get_var
// pjsua_perror
// pjsua_logging_config_default
// pjsua_logging_config_dup
// pjsua_config_default
// pjsua_config_dup
// pjsua_msg_data_init
// pjsua_msg_data_clone
// pjsua_transport_config_default
// pjsua_transport_config_dup
// pjsua_ice_config_from_media_config
// pjsua_ice_config_dup
// pjsua_turn_config_from_media_config
// pjsua_turn_config_dup
// pjsua_srtp_opt_default
// pjsua_srtp_opt_dup
// pjsua_acc_config_default
// pjsua_buddy_config_default
// pjsua_media_config_default
// pjsua_reconfigure_logging
// pjsua_stop_worker_threads
// pjsua_create
// pjsua_init
// pjsua_update_stun_servers
// pjsua_resolve_stun_servers
// pjsua_cancel_stun_resolution
// pjsua_destroy2
// pjsua_destroy
// pjsua_start
// pjsua_handle_events
// pjsua_pool_create
// pjsua_get_pjsip_endpt
// pjsua_get_pjmedia_endpt
// pjsua_get_pool_factory
// pjsua_transport_create
// pjsua_transport_register
// pjsua_tpfactory_register
// pjsua_enum_transports
// pjsua_transport_get_info
// pjsua_transport_set_enable
// pjsua_transport_close
// pjsua_transport_lis_start
// pjsua_ip_change_param_default
// pjsua_detect_nat_type
// pjsua_get_nat_type
// pjsua_verify_url
// pjsua_verify_sip_url
// pjsua_schedule_timer_dbg
// pjsua_schedule_timer
// pjsua_schedule_timer2_dbg
// pjsua_schedule_timer2
// pjsua_cancel_timer
// pjsua_dump
// pjsua_handle_ip_change
pub struct SIPUserAgent {
    /// hold internal pjsua data
    pool: *mut pj_pool_t,
    app_config: pjsua_config,
    log_config: pjsua_logging_config,
    media_config: pjsua_media_config,
    udp_config: pjsua_transport_config,
    rtp_config: pjsua_transport_config,
    account: SIPAccount, // for now just set to 1 account
    buddy_list: Vec<SIPBuddy>,
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
    ringback_slot: i32,
    ring_slot: i32,
}

//type SIPConfig = pjsua_config;

pub const PJSUA_INVALID_ID: i32 = -1;

impl SIPUserAgent {
    /// create sip user sip user agent with default value
    ///
    pub fn new() -> SIPUserAgent {
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

        SIPUserAgent {
            pool: ctx,
            app_config: pjsua_config::new(),
            log_config: pjsua_logging_config::new(),
            media_config: pjsua_media_config::new(),
            udp_config: udp,
            rtp_config: rtp,
            account: SIPAccount::new(),
            buddy_list: Vec::<SIPBuddy>::new(),
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
            ringback_slot: PJSUA_INVALID_ID,
            ring_slot: PJSUA_INVALID_ID,
        }
    }

    /// start application
    pub fn start(&mut self) {
        unsafe {
            self.app_config.cb.on_call_state = Some(SIPUserAgent::on_call_state);
            self.app_config.cb.on_stream_destroyed = Some(SIPUserAgent::on_stream_destroyed);
            self.app_config.cb.on_call_media_state = Some(SIPUserAgent::on_call_media_state);
            self.app_config.cb.on_incoming_call = Some(SIPUserAgent::on_incoming_call);
            self.app_config.cb.on_dtmf_digit2 = Some(SIPUserAgent::on_dtmf_digit2);
            self.app_config.cb.on_call_redirected = Some(SIPUserAgent::on_call_redirected);
            self.app_config.cb.on_reg_state = Some(SIPUserAgent::on_reg_state);
            self.app_config.cb.on_incoming_subscribe = Some(SIPUserAgent::on_incoming_subscribe);
            self.app_config.cb.on_buddy_state = Some(SIPUserAgent::on_buddy_state);
            self.app_config.cb.on_buddy_evsub_state = Some(SIPUserAgent::on_buddy_evsub_state);
            self.app_config.cb.on_pager = Some(SIPUserAgent::on_pager);
            self.app_config.cb.on_typing = Some(SIPUserAgent::on_typing);
            self.app_config.cb.on_call_transfer_status =
                Some(SIPUserAgent::on_call_transfer_status);
            self.app_config.cb.on_call_replaced = Some(SIPUserAgent::on_call_replaced);
            self.app_config.cb.on_nat_detect = Some(SIPUserAgent::on_nat_detect);
            self.app_config.cb.on_mwi_info = Some(SIPUserAgent::on_mwi_info);
            self.app_config.cb.on_transport_state = Some(SIPUserAgent::on_transport_state);
            self.app_config.cb.on_ice_transport_error = Some(SIPUserAgent::on_ice_transport_error);
            self.app_config.cb.on_snd_dev_operation = Some(SIPUserAgent::on_snd_dev_operation);
            self.app_config.cb.on_call_media_event = Some(SIPUserAgent::on_call_media_event);
            self.app_config.cb.on_ip_change_progress = Some(SIPUserAgent::on_ip_change_progress);

            // init pjsua
            pjsua_init(
                &mut self.app_config as *mut _,
                &mut self.log_config as *mut _,
                &mut self.media_config as *mut _,
            );
            // pjsip endpoint for unhadled error
            self.default_handler.on_rx_request = Some(SIPUserAgent::on_rx_request);
            pjsip_endpt_register_module(
                pjsua_get_pjsip_endpt(),
                &mut self.default_handler as *mut _,
            );
        }
    }
}

//binding clike code patern with destructor
impl Drop for SIPUserAgent {
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
    
      status = pjsip_endpt_create_response(pjsua_get_pjsip_endpt(), rdata as *const _, 200, str_null as *const _, tdata as *mut _);
      if status != pj_constants__PJ_SUCCESS as i32 {
          return;
      }
      #[allow(unused_assignments)]
      let void: *const c_void = ptr::null();
    
      let exp: *const pjsip_expires_hdr = pjsip_msg_find_hdr((*rdata).msg_info.msg,
                  pjsip_hdr_e_PJSIP_H_EXPIRES, void) as *const _;

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
                    let nc: *mut pjsip_contact_hdr = pjsip_hdr_clone((*tdata).pool, 
                                        h as *const _) as *mut pjsip_contact_hdr;

                    (*nc).expires = e;
                    pj_list_insert_before((*tdata).msg as *mut _, nc as *mut _);
                    cnt = cnt +1;
                }
                h = (*h).next;
            }
        }
        
        // todo review c code for this. it's c clasic problem
        let srv: *mut pjsip_generic_string_hdr = pjsip_generic_string_hdr_create (
                      (*tdata).pool, str_null, str_null);
        // create server name
        let tmp: CString = CString::new("Server").expect("cant create Server string");
        (*srv).name = pj_str(tmp.as_ptr()  as *mut c_char);
        // create add description 
        let tmp: CString = CString::new("IpCodec simple registrar").expect("cant create simple registrar");
        (*srv).hvalue = pj_str(tmp.as_ptr() as *mut c_char);

        pj_list_insert_before((*tdata).msg as *mut _, srv as *mut _);
        let cb: pjsip_send_callback = None;
        pjsip_endpt_send_response2(pjsua_get_pjsip_endpt(), rdata, tdata as *mut _, void as *mut _, None);
    }
}


// handle for callback PjsipModule
impl PjsipModuleCallback for SIPUserAgent {
    unsafe extern "C" fn on_rx_request(rdata: *mut pjsip_rx_data) -> pj_status_t {
        // base rx request handle undefined state. 
        let tdata: *const pjsip_tx_data = ptr::null();
        let status_code: pjsip_status_code;
        let status: pj_status_t;


        let mut rdata = *rdata;
        let msg = *rdata.msg_info.msg;
        let mut method = msg.line.req.method;
        // let msg_info = method.msg_info;
        if pjsip_method_cmp(&mut method as *const _, &pjsip_ack_method as *const _) == 0{
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
        status = pjsip_endpt_create_response(pjsua_get_pjsip_endpt(),
                  &mut rdata as *const _, 
                  status_code as c_int, 
                  null_ptr,
                  tdata as *mut *mut  _);

        if status != (pj_constants__PJ_SUCCESS as pj_status_t) {
          return pj_constants__PJ_TRUE as pj_status_t; 
        }

        if status_code == pjsip_status_code_PJSIP_SC_METHOD_NOT_ALLOWED {
          #[allow(unused_assignments)]
          let mut cap_hdr: *const pjsip_hdr = ptr::null();

          cap_hdr = pjsip_endpt_get_capability(pjsua_get_pjsip_endpt(),
                          pjsip_hdr_e_PJSIP_H_ALLOW as i32, null_ptr);

          if !cap_hdr.is_null() {
              //pjsip_msg_add_hdr(msg, pjsip_hdr_clone(tdata.pool, cap_hdr));
              pj_list_insert_before((*tdata).msg as *mut _, 
                          pjsip_hdr_clone((*tdata).pool as *mut _, cap_hdr as *const _));
          }
        }


        // add user-agent header
        #[allow(unused_assignments)]
        let mut h: *const pjsip_hdr = ptr::null(); 
        
        let ua_str = CString::new("User-Agent").expect("cant create str User-Agent.");
        let mut ua: pj_str_t = pj_str_t {ptr: ua_str.as_ptr() as *mut _, slen: 10};
        let agent_str = CString::new("AudioIP 0.1").expect("cant create str AudioIP 0.1"); 
        let mut agent = pj_str_t{ptr: agent_str.as_ptr() as *mut _, slen: 11};

        h = pjsip_generic_string_hdr_create(
          (*tdata).pool as *mut _, &mut ua as _, &mut agent as *mut _) as *mut _;
        
        pj_list_insert_before((*tdata).msg as *mut _, h as *mut _);
         
        pj_constants__PJ_TRUE as pj_status_t
    }
}

impl PjsuaCallback for SIPUserAgent {
    // Call status event
    unsafe extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {
      // call info data  
      let mut call_info: pjsua_call_info = pjsua_call_info::new();
    
    }

    // Stream Destroyed;
    unsafe extern "C" fn on_stream_destroyed(
        call_id: pjsua_call_id,
        strm: *mut pjmedia_stream,
        stream_idx: c_uint,
    ) {
        // todo here
    }

    // Call media satate
    unsafe extern "C" fn on_call_media_state(call_id: pjsua_call_id) {}

    // Incoming Call
    unsafe extern "C" fn on_incoming_call(
        acc_id: pjsua_acc_id,
        call_id: pjsua_call_id,
        rdata: *mut pjsip_rx_data,
    ) {
        // todo here
    }

    // DTMF Digit2
    unsafe extern "C" fn on_dtmf_digit2(call_id: pjsua_call_id, info: *const pjsua_dtmf_info) {
        // todo here
    }

    // Call Redirected
    unsafe extern "C" fn on_call_redirected(
        call_id: pjsua_call_id,
        target: *const pjsip_uri,
        e: *const pjsip_event,
    ) -> pjsip_redirect_op {
        // todo here
        0x0
    }

    // REG state
    unsafe extern "C" fn on_reg_state(acc_id: pjsua_acc_id) {
        // todo here
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
        // todo here
    }

    // Buddy State
    unsafe extern "C" fn on_buddy_state(buddy_id: pjsua_buddy_id) {}

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
        // todo here
        // let info_str: [c_char; 128];
        let mut acc_info: pjsua_acc_info = pjsua_acc_info::new();
        let mut tp_info: pjsua_transport_info = pjsua_transport_info::new();

        if status == pj_constants__PJ_SUCCESS as pj_status_t {
            match op {
              pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_RESTART_LIS => {
                  pjsua_transport_get_info((*info).lis_restart.transport_id,
                    &mut tp_info as *mut _);
                  println!("restart transport.");
              },
              pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_SHUTDOWN_TP => {
                  pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id,
                    &mut acc_info as *mut _);
                  println!("transport shutdown for account.");
              },
              pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_UPDATE_CONTACT => {
                  pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id,
                    &mut acc_info as *mut _);
                  println!("update contact for account.");
              },
              pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_HANGUP_CALLS => {
                  pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id,
                    &mut acc_info as *mut _);
                  println!("hangup call for account.");
              },
              pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_ACC_REINVITE_CALLS => {
                  pjsua_acc_get_info((*info).acc_shutdown_tp.acc_id,
                    &mut acc_info as *mut _);
                  println!("reinvite call for account.");
              },
              pjsua_ip_change_op_PJSUA_IP_CHANGE_OP_COMPLETED => {
                  println!("done");
              },

                _ => println!("warn validate c code.") 
            }
        } else {
            println!("IP change progress fail.");
        }
    }
}
