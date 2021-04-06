
use std::cell::RefCell;

use super::pj_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsip_sys::*;
use super::pjmedia_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;
use super::pjsua::*;

use super::pjsua;

use std::ptr;
use std::ffi::CString;
use std::os::raw::c_uint;



#[derive(Clone)]
pub struct SIPAccount {
    id: i32,
    // info: pjsua_acc_info,
    ctx: RefCell<pjsua_acc_config>,
}


pub trait SIPAccountExt {
    // skiped
    // user_data: *mut c_void,

    fn set_priority(&self, value: i32);
    fn get_priority(&self) -> i32;

    fn set_id(&self, value: String);
    fn get_id(&self) -> String;

    fn set_reg_uri(&self, value: String);
    fn get_reg_uri(&self) -> String;

    fn set_reg_hdr_list(&self, value: pjsip_hdr);
    fn get_reg_hdr_list(&self) -> pjsip_hdr;

    fn set_reg_contact_params(&self, value: String);
    fn get_reg_contact_params(&self) -> String;

    fn set_sub_hdr_list(&self, value: pjsip_hdr);
    fn get_sub_hdr_list(&self) -> pjsip_hdr;

    fn set_mwi_enabled(&self, value: bool);
    fn get_mwi_enabled(&self) -> bool;

    fn set_mwi_expires(&self, value: u32);
    fn get_mwi_expires(&self) -> u32;

    fn set_publish_enabled(&self, value: bool);
    fn get_publish_enabled(&self) -> bool;

    fn set_publish_opt(&self, value: pjsip_publishc_opt);
    fn get_publish_opt(&self) -> pjsip_publishc_opt;

    fn set_unpublish_max_wait_time_msec(&self, value: u32);
    fn get_unpublish_max_wait_time_msec(&self) -> u32;

    fn set_auth_pref(&self, value: pjsip_auth_clt_pref);
    fn get_auth_pref(&self) -> pjsip_auth_clt_pref;

    fn set_pidf_tuple_id(&self, value: String);
    fn get_pidf_tuple_id(&self) -> String;

    fn set_force_contact(&self, value: String);
    fn get_force_contact(&self) -> String;

    fn set_contact_params(&self, value: String);
    fn get_contact_params(&self) -> String;

    fn set_contact_uri_params(&self, value: String);
    fn get_contact_uri_params(&self) -> String;

    fn set_require_100rel(&self, value: pjsua_100rel_use);
    fn get_require_100rel(&self) -> pjsua_100rel_use;

    fn set_use_timer(&self, value: pjsua_sip_timer_use);
    fn get_use_timer(&self) -> pjsua_sip_timer_use;

    fn set_timer_setting(&self, value: pjsip_timer_setting);
    fn get_timer_setting(&self) -> pjsip_timer_setting;

    fn set_proxy_cnt(&self, value: u32);
    fn get_proxy_cnt(&self) -> u32;

    fn set_proxy(&self, value: [String; 8usize]);
    fn get_proxy(&self) -> [String; 8usize];

    fn set_lock_codec(&self, value: u32);
    fn get_lock_codec(&self) -> u32;

    fn set_reg_timeout(&self, value: u32);
    fn get_reg_timeout(&self) -> u32;

    fn set_reg_delay_before_refresh(&self, value: u32);
    fn get_reg_delay_before_refresh(&self) -> u32;

    fn set_unreg_timeout(&self, value: u32);
    fn get_unreg_timeout(&self) -> u32;

    fn set_cred_count(&self, value: u32);
    fn get_cred_count(&self) -> u32;

    fn set_cred_info(&self, value: [pjsip_cred_info; 8usize]);
    fn get_cred_info(&self) -> [pjsip_cred_info; 8usize];

    fn set_transport_id(&self, value: pjsua_transport_id);
    fn get_transport_id(&self) -> pjsua_transport_id;

    fn set_allow_contact_rewrite(&self, value: bool);
    fn get_allow_contact_rewrite(&self) -> bool;

    fn set_contact_rewrite_method(&self, value: i32);
    fn get_contact_rewrite_method(&self) -> i32;

    fn set_contact_use_src_port(&self, value: bool);
    fn get_contact_use_src_port(&self) -> bool;

    fn set_allow_via_rewrite(&self, value: bool);
    fn get_allow_via_rewrite(&self) -> bool;

    fn set_allow_sdp_nat_rewrite(&self, value: bool);
    fn get_allow_sdp_nat_rewrite(&self) -> bool;

    fn set_use_rfc5626(&self, value: u32);
    fn get_use_rfc5626(&self) -> u32;

    fn set_rfc5626_instance_id(&self, value: String);
    fn get_rfc5626_instance_id(&self) -> String;

    fn set_rfc5626_reg_id(&self, value: String);
    fn get_rfc5626_reg_id(&self) -> String;

    fn set_ka_interval(&self, value: u32);
    fn get_ka_interval(&self) -> u32;

    fn set_ka_data(&self, value: String);
    fn get_ka_data(&self) -> String;

    // skiped AudioIpCodecs not support video
    // vid_in_auto_show: pj_bool_t,
    // vid_out_auto_transmit: pj_bool_t,
    // vid_wnd_flags: c_uint,
    // vid_cap_dev: pjmedia_vid_dev_index,
    // vid_rend_dev: pjmedia_vid_dev_index,
    // vid_stream_rc_cfg: pjmedia_vid_stream_rc_config,
    // vid_stream_sk_cfg: pjmedia_vid_stream_sk_config,

    fn set_rtp_cfg(&self, value: pjsua_transport_config);
    fn get_rtp_cfg(&self) -> pjsua_transport_config;

    fn set_nat64_opt(&self, value: pjsua_nat64_opt);
    fn get_nat64_opt(&self) -> pjsua_nat64_opt;

    fn set_ipv6_media_use(&self, value: pjsua_ipv6_use);
    fn get_ipv6_media_use(&self) -> pjsua_ipv6_use;

    fn set_sip_stun_use(&self, value: pjsua_stun_use);
    fn get_sip_stun_use(&self) -> pjsua_stun_use;

    fn set_media_stun_use(&self, value: pjsua_stun_use);
    fn get_media_stun_use(&self) -> pjsua_stun_use;

    fn set_use_loop_med_tp(&self, value: bool);
    fn get_use_loop_med_tp(&self) -> bool;

    fn set_enable_loopback(&self, value: bool);
    fn get_enable_loopback(&self) -> bool;

    fn set_ice_cfg_use(&self, value: pjsua_ice_config_use);
    fn get_ice_cfg_use(&self) -> pjsua_ice_config_use;

    fn set_ice_cfg(&self, value: pjsua_ice_config);
    fn get_ice_cfg(&self) -> pjsua_ice_config;

    fn set_turn_cfg_use(&self, value: pjsua_turn_config_use);
    fn get_turn_cfg_use(&self) -> pjsua_turn_config_use;

    fn set_turn_cfg(&self, value: pjsua_turn_config);
    fn get_turn_cfg(&self) -> pjsua_turn_config;

    fn set_use_srtp(&self, value: pjmedia_srtp_use);
    fn get_use_srtp(&self) -> pjmedia_srtp_use;

    fn set_srtp_secure_signaling(&self, value: i32);
    fn get_srtp_secure_signaling(&self) -> i32;

    fn set_srtp_optional_dup_offer(&self, value: bool);
    fn get_srtp_optional_dup_offer(&self) -> bool;

    fn set_srtp_opt(&self, value: pjsua_srtp_opt);
    fn get_srtp_opt(&self) -> pjsua_srtp_opt;

    fn set_reg_retry_interval(&self, value: u32);
    fn get_reg_retry_interval(&self) -> u32;

    fn set_reg_first_retry_interval(&self, value: u32);
    fn get_reg_first_retry_interval(&self) -> u32;

    fn set_reg_retry_random_interval(&self, value: u32);
    fn get_reg_retry_random_interval(&self) -> u32;

    fn set_drop_calls_on_reg_fail(&self, value: bool);
    fn get_drop_calls_on_reg_fail(&self) -> bool;

    fn set_reg_use_proxy(&self, value: u32);
    fn get_reg_use_proxy(&self) -> u32;

    fn set_call_hold_type(&self, value: pjsua_call_hold_type);
    fn get_call_hold_type(&self) -> u32;

    fn set_register_on_acc_add(&self, value: bool);
    fn get_register_on_acc_add(&self) -> bool;

    fn set_ip_change_cfg(&self, value: pjsua_ip_change_acc_cfg);
    fn get_ip_change_cfg(&self) -> pjsua_ip_change_acc_cfg;

    fn set_enable_rtcp_mux(&self, value: bool);
    fn get_enable_rtcp_mux(&self) -> bool;

    fn set_rtcp_fb_cfg(&self, value: pjmedia_rtcp_fb_setting);
    fn get_rtcp_fb_cfg(&self) -> pjmedia_rtcp_fb_setting;
}

impl SIPAccount {

    pub fn new() -> Self {
        SIPAccount {
            id: -1,
            ctx: RefCell::new(pjsua_acc_config::new()),
        }
    }

    pub fn from(acc_id: pjsua_acc_id) -> Result<Self, i32> {

        let ret = SIPAccount {
            id: acc_id,
            ctx: RefCell::new(pjsua_acc_config::new())
        };

        if let Err(e) = pjsua::acc_get_config(acc_id, &mut ret.ctx.borrow_mut()) {
            return Err(e);
        }

        Ok(ret)
    }

    // configure for default setting.
    pub fn config_default(&mut self) {

        pjsua::acc_config_default(&mut self.ctx.borrow_mut());

        self.set_cred_count(1);
        self.set_reg_retry_interval(300);
        self.set_reg_first_retry_interval(60);


        let mut cred = [pjsip_cred_info::new(); 8usize];
        cred[0].data_type = 0;
        cred[0].scheme = pj_str_t::from_string(String::from("Digest"));

        self.set_cred_info(cred);
    }

    // set sip id for account
    pub fn set_id(&mut self, value: String) {
        self.ctx.borrow_mut().id = pj_str_t::from_string(value);

    }

    // set registrar uri
    pub fn set_reg_uri(&mut self, value: String) {

        self.ctx.borrow_mut().reg_uri = pj_str_t::from_string(value);

    }

    // set realm for account
    pub fn set_realm(&mut self, value: String) {

        self.ctx.borrow_mut().cred_info[0].realm = pj_str_t::from_string(value);

    }

    // set username
    pub fn set_username(&mut self, value: String) {
        self.ctx.borrow_mut().cred_info[0].username = pj_str_t::from_string(value);
    }

    // set password
    pub fn set_password(&mut self, value: String) {
        self.ctx.borrow_mut().cred_info[0].data = pj_str_t::from_string(value);

    }

    // check if this account valid or not
    pub fn is_valid(&self) -> bool {
        pjsua::acc_is_valid(self.id)
    }

    // set this account to be default
    pub fn set_default(&self) {

        pjsua::acc_set_default(self.id)
        .expect("SIPAccount::pjsua_acc_set_default");
    }

    // add account to internal pjsua
    pub fn add(&mut self, is_default: bool) {

        pjsua::acc_add(&mut self.ctx.borrow_mut(), is_default, &mut self.id)
        .expect("SIPAccount::pjsua_acc_add");
    }

    // remove account from internal pjsua
    pub fn del(&self) {
        pjsua::acc_del(self.id)
        .expect("SIPAccount::pjsua_acc_del");
    }

    // get inner config
    pub fn get_config(&self) -> Result<pjsua_acc_config, i32> {

        let mut acc_cfg = pjsua_acc_config::new();

        if let Err(e) = pjsua::acc_get_config(self.id, &mut acc_cfg) {
            return Err(e)
        }

        Ok(acc_cfg)
    }

    // modify acoount config
    pub fn modify(&self, acc_config: &mut pjsua_acc_config) {
        pjsua::acc_modify( self.id, acc_config)
        .expect("SIPAccount::pjsua_acc_modify");
    }

    // set online or ofline status
    pub fn set_online_status(&self, is_online: bool) {

        pjsua::acc_set_online_status( self.id, is_online)
        .expect("SIPAccount::pjsua_acc_set_online_status");
    }

    // this online status more like presence state for account
    pub fn set_online_status2(&self, is_online: bool, pr: &mut pjrpid_element) {
        pjsua::acc_set_online_status2( self.id, is_online, pr )
        .expect("SIPAccount::pjsua_acc_set_online_status2")
    }

    // set registration process
    pub fn set_registration(&self, renew: bool) {
        pjsua::acc_set_registration( self.id, renew)
        .expect("SIPAccount::pjsua_acc_set_registration");
    }

    // get inner account info
    pub fn get_info(&self) -> Result<pjsua_acc_info, i32> {

        let mut info = pjsua_acc_info::new();

        if let Err(e) = pjsua::acc_get_info( self.id, &mut info) {
            println!("ERR cant get account info");
            return Err(e);
        }

        Ok(info)
    }

    // Create arbitrary requests using the account.
    // Application should only use this function to create auxiliary requests outside dialog,
    // such as OPTIONS, and use the call or presence API to create dialog related requests.
    pub fn create_request(&self, method: &mut pjsip_method, target: String) -> Result<pjsip_tx_data, i32> {

        let mut rdata = pjsip_tx_data::new();

        if let Err(e) = pjsua::acc_create_request( self.id, method, target,
            &mut rdata) {

            println!("ERR cant create request for account");
            return Err(e);
        }

        Ok(rdata)
    }

    // Create a suitable Contact header value,
    // based on the specified target URI for the specified account.
    pub fn create_uac_contact(&self, contact: String, uri: String) {
        pjsua::acc_create_uac_contact( contact, self.id, uri)
        .expect("SIPAccount::pjsua_acc_create_uac_contact");
    }

    // Create a suitable Contact header value, based on the information in the incoming request.
    pub fn create_uas_contact(&self, contact: String, rdata: &mut pjsip_rx_data) {
        pjsua::acc_create_uas_contact( contact, self.id, rdata)
        .expect("SIPAccount::pjsua_acc_create_uas_contact");
    }

    // set transport by given id transport id for account
    pub fn set_transport(&self, tp_id: pjsua_transport_id) {

        pjsua::acc_set_transport( self.id, tp_id)
        .expect("SIPAccount::pjsua_acc_set_transport");
    }

    // Send NOTIFY to inform account presence status or to terminate server side presence subscription.
    // If application wants to reject the incoming request,
    // it should set the state to PJSIP_EVSUB_STATE_TERMINATED.
    pub fn pres_notify (&self,
        srv_pres: &mut pjsua_srv_pres,
        state: pjsip_evsub_state,
        state_str: String,
        reason: String,
        with_body: bool,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::pres_notify(self.id, srv_pres, state, state_str, reason, with_body, msg_data)
        .expect("SIPAccount::pjsua_pres_notify");
    }

    // Send instant messaging outside dialog, using the specified account for route set and authentication
    pub fn im_send(&self,
        to: String,
        mime_type: String,
        content: String,
        msg_data: Option<&mut pjsua_msg_data>
    ) {
        pjsua::im_send(self.id, to, mime_type, content, msg_data)
        .expect("SIPAccount::pjsua_im_send");
    }

    // Send typing indication outside dialog.
    pub fn im_typing(&self, to: String, is_typing: bool, msg_data: Option<&mut pjsua_msg_data>) {
        pjsua::im_typing(self.id, to, is_typing, msg_data)
        .expect("SIPAccount::pjsua_im_typing");
    }

    pub fn call(
        &self,
        dst_uri: String,
        opt: Option<&mut pjsua_call_setting>,
        msg_data: Option<&mut pjsua_msg_data>,
        p_call_id: Option<&mut pjsua_call_id>
    ) {
        pjsua::call_make_call(self.id, dst_uri, opt, msg_data, p_call_id)
        .expect("SIPAccount::pjsua_call_make_call");
    }

}

// true interface for managing accounts
pub struct SIPAccounts {  }

// please remember default pjsua account
// was 8 see pjsua sample application at
// pjproject.
impl SIPAccounts {

    // create new accounts
    pub fn new() -> SIPAccounts {
        SIPAccounts { }
    }

    // get default account
    pub fn get_default(&self) -> pjsua_acc_id {
        pjsua::acc_get_default()
    }

    // set default rtp config
    pub fn set_rtp_config(&self, rtp_config: pjsua_transport_config) {

        let ids = self.enum_accs_id();

        ids.iter().for_each(|acc_id| {

            let mut acc = SIPAccount::from(acc_id.clone());

            match acc {
                Ok(ref mut sipacc) => {
                    sipacc.set_rtp_cfg(rtp_config);
                },
                Err(err) => println!("print error ={}", err)
            }
        });
    }

    // set register retry interval
    pub fn set_reg_retry_interval(&self, value: u32) {

        let ids = self.enum_accs_id();

        ids.iter().for_each(|acc_id| {

            let mut acc = SIPAccount::from(acc_id.clone());

            match acc {
                Ok(ref mut sipacc) => {
                    sipacc.set_reg_retry_interval(value);
                },
                Err(err) => println!("print error ={}", err)
            }
        });
    }

    // set first register retry interval
    pub fn set_reg_first_retry_interval(&self, value: u32) {

        let ids = self.enum_accs_id();

        ids.iter().for_each(|acc_id| {

            let mut acc = SIPAccount::from(acc_id.clone());

            match acc {
                Ok(ref mut sipacc) => {
                    // sipacc.ctx.reg_first_retry_interval = value;
                    sipacc.set_reg_first_retry_interval(value);
                },
                Err(err) => println!("print error ={}", err)
            }
        });
    }

    // get number of registered to pjsua library
    pub fn get_count(&self) -> u32 {
        pjsua::acc_get_count()
    }

    // enumerate current registered account id
    pub fn enum_accs_id(&self) -> Vec<pjsua_acc_id> {

        let mut ret: Vec<pjsua_acc_id> = Vec::new();

        let mut acc_id: [pjsua_acc_id; PJSUA_MAX_ACC as usize] = [-1; PJSUA_MAX_ACC as usize];
        let mut count = 8u32;

        if let Ok(_)= pjsua::enum_accs(&mut acc_id, &mut count) {

            for i in 0..count as usize {
                ret.push(acc_id[i]);
            }
        } else {
            println!("ERR cant enumerate accounts id.");
        }

        ret
    }

    // enumerate current registered account info
    pub fn enum_info(&self) -> Vec<pjsua_acc_info> {

        let mut ret: Vec<pjsua_acc_info> = Vec::new();

        let mut acc_info: [pjsua_acc_info; PJSUA_MAX_ACC as usize] = [pjsua_acc_info::new(); PJSUA_MAX_ACC as usize];
        let mut count = 0u32;

        if let Ok(_) = pjsua::acc_enum_info( &mut acc_info, &mut count) {

            for i in 0..count as usize {
                ret.push(acc_info[i]);
            }

        } else {
            println!("ERR cant enumerate account info.");
        }

        ret
    }

    // This is an internal function to find the most appropriate account to used to reach to the specified URL.
    pub fn find_for_outgoing(&self, value: String) -> pjsua_acc_id {
        pjsua::acc_find_for_outgoing(value)
    }

    // This is an internal function to find the most appropriate account to be used to handle incoming calls.
    pub fn on_acc_find_for_incoming (&self, rdata: &mut pjsip_rx_data) -> pjsua_acc_id {
        pjsua::acc_find_for_incoming(rdata)
    }


}


impl SIPAccountExt for SIPAccount {

    fn set_priority(&self, value: i32) {
        self.ctx.borrow_mut().priority = value;
    }

    fn get_priority(&self) -> i32 {
        self.ctx.borrow().priority
    }

    fn set_id(&self, value: String) {
        self.ctx.borrow_mut().id = pj_str_t::from_string(value);
    }

    fn get_id(&self) -> String {
        self.ctx.borrow().id.to_string()
    }

    fn set_reg_uri(&self, value: String) {
        self.ctx.borrow_mut().reg_uri = pj_str_t::from_string(value);
    }

    fn get_reg_uri(&self) -> String {
        self.ctx.borrow().reg_uri.to_string()
    }

    fn set_reg_hdr_list(&self, value: pjsip_hdr) {
        self.ctx.borrow_mut().reg_hdr_list = value;
    }

    fn get_reg_hdr_list(&self) -> pjsip_hdr {
        self.ctx.borrow().reg_hdr_list
    }

    fn set_reg_contact_params(&self, value: String) {
        self.ctx.borrow_mut().reg_contact_params = pj_str_t::from_string(value);
    }

    fn get_reg_contact_params(&self) -> String {
        self.ctx.borrow().reg_contact_params.to_string()
    }

    fn set_sub_hdr_list(&self, value: pjsip_hdr) {
        self.ctx.borrow_mut().sub_hdr_list = value;
    }

    fn get_sub_hdr_list(&self) -> pjsip_hdr {
        self.ctx.borrow().sub_hdr_list
    }

    fn set_mwi_enabled(&self, value: bool) {
        self.ctx.borrow_mut().mwi_enabled = boolean_to_pjbool(value);
    }

    fn get_mwi_enabled(&self) -> bool {
        check_boolean(self.ctx.borrow().mwi_enabled)
    }

    fn set_mwi_expires(&self, value: u32) {
        self.ctx.borrow_mut().mwi_expires = value;
    }

    fn get_mwi_expires(&self) -> u32 {
        self.ctx.borrow().mwi_expires
    }

    fn set_publish_enabled(&self, value: bool) {
        self.ctx.borrow_mut().publish_enabled = boolean_to_pjbool(value);
    }

    fn get_publish_enabled(&self) -> bool {
        check_boolean(self.ctx.borrow().publish_enabled)
    }

    fn set_publish_opt(&self, value: pjsip_publishc_opt) {
        self.ctx.borrow_mut().publish_opt = value;
    }

    fn get_publish_opt(&self) -> pjsip_publishc_opt {
        self.ctx.borrow().publish_opt
    }

    fn set_unpublish_max_wait_time_msec(&self, value: u32) {
        self.ctx.borrow_mut().unpublish_max_wait_time_msec = value;
    }

    fn get_unpublish_max_wait_time_msec(&self) -> u32 {
        self.ctx.borrow().unpublish_max_wait_time_msec
    }

    fn set_auth_pref(&self, value: pjsip_auth_clt_pref) {
        self.ctx.borrow_mut().auth_pref = value;
    }

    fn get_auth_pref(&self) -> pjsip_auth_clt_pref {
        self.ctx.borrow().auth_pref
    }

    fn set_pidf_tuple_id(&self, value: String) {
        self.ctx.borrow_mut().pidf_tuple_id = pj_str_t::from_string(value);
    }

    fn get_pidf_tuple_id(&self) -> String {
        self.ctx.borrow().pidf_tuple_id.to_string()
    }

    fn set_force_contact(&self, value: String) {
        self.ctx.borrow_mut().force_contact = pj_str_t::from_string(value);
    }

    fn get_force_contact(&self) -> String {
        self.ctx.borrow().force_contact.to_string()
    }

    fn set_contact_params(&self, value: String) {
        self.ctx.borrow_mut().contact_params = pj_str_t::from_string(value);
    }

    fn get_contact_params(&self) -> String {
        self.ctx.borrow().contact_params.to_string()
    }

    fn set_contact_uri_params(&self, value: String) {
        self.ctx.borrow_mut().contact_uri_params = pj_str_t::from_string(value);
    }

    fn get_contact_uri_params(&self) -> String {
        self.ctx.borrow().contact_uri_params.to_string()
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

    fn set_timer_setting(&self, value: pjsip_timer_setting) {
        self.ctx.borrow_mut().timer_setting = value;
    }

    fn get_timer_setting(&self) -> pjsip_timer_setting {
        self.ctx.borrow().timer_setting
    }

    fn set_proxy_cnt(&self, value: u32) {
        self.ctx.borrow_mut().proxy_cnt = value;
    }

    fn get_proxy_cnt(&self) -> u32 {
        self.ctx.borrow().proxy_cnt
    }

    fn set_proxy(&self, value: [String; 8usize]) {

        let mut tmp = [pj_str_t::new(); 8usize];

        for (idx, value) in value.iter().enumerate() {
            tmp[idx] = pj_str_t::from_string(value.clone());
        }

        self.ctx.borrow_mut().proxy = tmp;
    }

    fn get_proxy(&self) -> [String; 8usize] {

        let mut tmp = [
            String::new(), String::new(), String::new(), String::new(),
            String::new(), String::new(), String::new(), String::new()
        ];

        let val = self.ctx.borrow().proxy;

        for idx in 0..8usize {
            tmp[idx] = val[idx].to_string();
        }

        tmp
    }

    fn set_lock_codec(&self, value: u32) {
        self.ctx.borrow_mut().lock_codec = value;
    }

    fn get_lock_codec(&self) -> u32 {
        self.ctx.borrow().lock_codec
    }

    fn set_reg_timeout(&self, value: u32) {
        self.ctx.borrow_mut().reg_timeout = value;
    }

    fn get_reg_timeout(&self) -> u32 {
        self.ctx.borrow().reg_timeout
    }

    fn set_reg_delay_before_refresh(&self, value: u32) {
        self.ctx.borrow_mut().reg_delay_before_refresh = value;
    }

    fn get_reg_delay_before_refresh(&self) -> u32 {
        self.ctx.borrow_mut().reg_delay_before_refresh
    }

    fn set_unreg_timeout(&self, value: u32) {
        self.ctx.borrow_mut().unreg_timeout = value;
    }

    fn get_unreg_timeout(&self) -> u32 {
        self.ctx.borrow().unreg_timeout
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

    fn set_transport_id(&self, value: pjsua_transport_id) {
        self.ctx.borrow_mut().transport_id = value;
    }

    fn get_transport_id(&self) -> pjsua_transport_id {
        self.ctx.borrow().transport_id
    }

    fn set_allow_contact_rewrite(&self, value: bool) {
        self.ctx.borrow_mut().allow_contact_rewrite = boolean_to_pjbool(value);
    }

    fn get_allow_contact_rewrite(&self) -> bool {
        check_boolean(self.ctx.borrow().allow_contact_rewrite)
    }

    fn set_contact_rewrite_method(&self, value: i32) {
        self.ctx.borrow_mut().contact_rewrite_method = value;
    }

    fn get_contact_rewrite_method(&self) -> i32 {
        self.ctx.borrow().contact_rewrite_method
    }

    fn set_contact_use_src_port(&self, value: bool) {
        self.ctx.borrow_mut().contact_use_src_port = boolean_to_pjbool(value);
    }

    fn get_contact_use_src_port(&self) -> bool {
        check_boolean(self.ctx.borrow().contact_use_src_port)
    }

    fn set_allow_via_rewrite(&self, value: bool) {
        self.ctx.borrow_mut().allow_via_rewrite = boolean_to_pjbool(value);
    }

    fn get_allow_via_rewrite(&self) -> bool {
        check_boolean(self.ctx.borrow().allow_via_rewrite)
    }

    fn set_allow_sdp_nat_rewrite(&self, value: bool) {
        self.ctx.borrow_mut().allow_sdp_nat_rewrite = boolean_to_pjbool(value);
    }

    fn get_allow_sdp_nat_rewrite(&self) -> bool {
        check_boolean(self.ctx.borrow().allow_sdp_nat_rewrite)
    }

    fn set_use_rfc5626(&self, value: u32) {
        self.ctx.borrow_mut().use_rfc5626 = value;
    }

    fn get_use_rfc5626(&self) -> u32 {
        self.ctx.borrow().use_rfc5626
    }

    fn set_rfc5626_instance_id(&self, value: String) {
        self.ctx.borrow_mut().rfc5626_instance_id = pj_str_t::from_string(value);
    }

    fn get_rfc5626_instance_id(&self) -> String {
        self.ctx.borrow().rfc5626_instance_id.to_string()
    }

    fn set_rfc5626_reg_id(&self, value: String) {
        self.ctx.borrow_mut().rfc5626_reg_id = pj_str_t::from_string(value);
    }

    fn get_rfc5626_reg_id(&self) -> String {
        self.ctx.borrow().rfc5626_reg_id.to_string()
    }

    fn set_ka_interval(&self, value: u32) {
        self.ctx.borrow_mut().ka_interval = value;
    }

    fn get_ka_interval(&self) -> u32 {
        self.ctx.borrow().ka_interval
    }

    fn set_ka_data(&self, value: String) {
        self.ctx.borrow_mut().ka_data = pj_str_t::from_string(value);
    }

    fn get_ka_data(&self) -> String {
        self.ctx.borrow().ka_data.to_string()
    }

    fn set_rtp_cfg(&self, value: pjsua_transport_config) {
        self.ctx.borrow_mut().rtp_cfg = value;
    }

    fn get_rtp_cfg(&self) -> pjsua_transport_config {
        self.ctx.borrow().rtp_cfg
    }

    fn set_nat64_opt(&self, value: pjsua_nat64_opt) {
        self.ctx.borrow_mut().nat64_opt = value;
    }

    fn get_nat64_opt(&self) -> pjsua_nat64_opt {
        self.ctx.borrow().nat64_opt
    }

    fn set_ipv6_media_use(&self, value: pjsua_ipv6_use) {
        self.ctx.borrow_mut().ipv6_media_use = value;
    }

    fn get_ipv6_media_use(&self) -> pjsua_ipv6_use {
        self.ctx.borrow().ipv6_media_use
    }

    fn set_sip_stun_use(&self, value: pjsua_stun_use) {
        self.ctx.borrow_mut().sip_stun_use = value;
    }

    fn get_sip_stun_use(&self) -> pjsua_stun_use {
        self.ctx.borrow().sip_stun_use
    }

    fn set_media_stun_use(&self, value: pjsua_stun_use) {
        self.ctx.borrow_mut().media_stun_use = value;
    }

    fn get_media_stun_use(&self) -> pjsua_stun_use {
        self.ctx.borrow().media_stun_use
    }

    fn set_use_loop_med_tp(&self, value: bool) {
        self.ctx.borrow_mut().use_loop_med_tp = boolean_to_pjbool(value);
    }

    fn get_use_loop_med_tp(&self) -> bool {
        check_boolean(self.ctx.borrow().use_loop_med_tp)
    }

    fn set_enable_loopback(&self, value: bool) {
        self.ctx.borrow_mut().enable_loopback = boolean_to_pjbool(value);
    }

    fn get_enable_loopback(&self) -> bool {
        check_boolean(self.ctx.borrow().enable_loopback)
    }

    fn set_ice_cfg_use(&self, value: pjsua_ice_config_use) {
        self.ctx.borrow_mut().ice_cfg_use = value;
    }

    fn get_ice_cfg_use(&self) -> pjsua_ice_config_use {
        self.ctx.borrow().ice_cfg_use
    }

    fn set_ice_cfg(&self, value: pjsua_ice_config) {
        self.ctx.borrow_mut().ice_cfg = value;
    }

    fn get_ice_cfg(&self) -> pjsua_ice_config {
        self.ctx.borrow().ice_cfg
    }

    fn set_turn_cfg_use(&self, value: pjsua_turn_config_use) {
        self.ctx.borrow_mut().turn_cfg_use = value;
    }

    fn get_turn_cfg_use(&self) -> pjsua_turn_config_use {
        self.ctx.borrow().turn_cfg_use
    }

    fn set_turn_cfg(&self, value: pjsua_turn_config) {
        self.ctx.borrow_mut().turn_cfg = value;
    }

    fn get_turn_cfg(&self) -> pjsua_turn_config {
        self.ctx.borrow().turn_cfg
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
        self.ctx.borrow_mut().srtp_optional_dup_offer = boolean_to_pjbool(value);
    }

    fn get_srtp_optional_dup_offer(&self) -> bool {
        check_boolean(self.ctx.borrow().srtp_optional_dup_offer)
    }

    fn set_srtp_opt(&self, value: pjsua_srtp_opt) {
        self.ctx.borrow_mut().srtp_opt = value;
    }

    fn get_srtp_opt(&self) -> pjsua_srtp_opt {
        self.ctx.borrow().srtp_opt
    }

    fn set_reg_retry_interval(&self, value: u32) {
        self.ctx.borrow_mut().reg_retry_interval = value;
    }

    fn get_reg_retry_interval(&self) -> u32 {
        self.ctx.borrow().reg_retry_interval
    }

    fn set_reg_first_retry_interval(&self, value: u32) {
        self.ctx.borrow_mut().reg_first_retry_interval = value;
    }

    fn get_reg_first_retry_interval(&self) -> u32 {
        self.ctx.borrow_mut().reg_first_retry_interval
    }

    fn set_reg_retry_random_interval(&self, value: u32) {
        self.ctx.borrow_mut().reg_retry_random_interval = value
    }

    fn get_reg_retry_random_interval(&self) -> u32 {
        self.ctx.borrow().reg_retry_random_interval
    }

    fn set_drop_calls_on_reg_fail(&self, value: bool) {
        self.ctx.borrow_mut().drop_calls_on_reg_fail = boolean_to_pjbool(value);
    }

    fn get_drop_calls_on_reg_fail(&self) -> bool {
        check_boolean(self.ctx.borrow().drop_calls_on_reg_fail)
    }

    fn set_reg_use_proxy(&self, value: u32) {
        self.ctx.borrow_mut().reg_use_proxy = value;
    }

    fn get_reg_use_proxy(&self) -> u32 {
        self.ctx.borrow().reg_use_proxy
    }

    fn set_call_hold_type(&self, value: pjsua_call_hold_type) {
        self.ctx.borrow_mut().call_hold_type = value;
    }

    fn get_call_hold_type(&self) -> u32 {
        self.ctx.borrow().call_hold_type
    }

    fn set_register_on_acc_add(&self, value: bool) {
        self.ctx.borrow_mut().register_on_acc_add = boolean_to_pjbool(value);
    }

    fn get_register_on_acc_add(&self) -> bool {
        check_boolean(self.ctx.borrow().register_on_acc_add)
    }

    fn set_ip_change_cfg(&self, value: pjsua_ip_change_acc_cfg) {
        self.ctx.borrow_mut().ip_change_cfg = value;
    }

    fn get_ip_change_cfg(&self) -> pjsua_ip_change_acc_cfg {
        self.ctx.borrow().ip_change_cfg
    }

    fn set_enable_rtcp_mux(&self, value: bool) {
        self.ctx.borrow_mut().enable_rtcp_mux = boolean_to_pjbool(value);
    }

    fn get_enable_rtcp_mux(&self) -> bool {
        check_boolean(self.ctx.borrow().enable_rtcp_mux)
    }

    fn set_rtcp_fb_cfg(&self, value: pjmedia_rtcp_fb_setting) {
        self.ctx.borrow_mut().rtcp_fb_cfg = value;
    }

    fn get_rtcp_fb_cfg(&self) -> pjmedia_rtcp_fb_setting {
        self.ctx.borrow().rtcp_fb_cfg
    }
}

