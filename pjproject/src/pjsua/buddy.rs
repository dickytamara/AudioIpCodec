
use crate::{pjsip_simple::SIPEvsubState, utils::{boolean_to_pjbool, check_boolean}};
use std::convert::TryFrom;
use super::*;

pub trait BuddyConfigExt {

    /// Buddy URL or name address.
    fn set_uri(&mut self, value: String);
    fn get_uri(&self) -> String;

    /// Specify whether presence subscription should start immediately.
    fn set_subscribe(&mut self, value: bool);
    fn get_subscribe(&self) -> bool;

    // fn set_user_data(&mut self);
    // fn get_user_data(&self) -> *mut ::std::os::raw::c_void;
}

// readonly
pub trait BuddyInfoExt {

    /// The buddy ID.
    fn get_id (&self) ->  i32;

    /// The full URI of the buddy, as specified in the configuration.
    fn get_uri (&self) ->  String;

    /// Buddy's Contact, only available when presence subscription has been established
    /// to the buddy.
    fn get_contact (&self) ->  String;

    /// Buddy's online status.
    fn get_status (&self) ->  BuddyStatus;

    /// Text to describe buddy's online status.
    fn get_status_text (&self) ->  String;

    /// Flag to indicate that we should monitor the presence information for this buddy
    /// (normally yes, unless explicitly disabled).
    fn get_monitor_pres (&self) ->  bool;

    /// If monitor_pres is enabled, this specifies the last state of the presence subscription.
    /// If presence subscription session is currently active, the value will be
    /// PJSIP_EVSUB_STATE_ACTIVE. If presence subscription request has been rejected, the value
    /// will be PJSIP_EVSUB_STATE_TERMINATED, and the termination reason will be specified in
    /// sub_term_reason.
    fn get_sub_state (&self) ->  SIPEvsubState;

    // fn get_sub_state_name (&self) ->  *const ::std::os::raw::c_char;

    /// Specifies the last presence subscription termination code. This would return the last
    /// status of the SUBSCRIBE request. If the subscription is terminated with NOTIFY by the
    /// server, this value will be set to 200, and subscription termination reason will be given
    /// in the sub_term_reason field.
    fn get_sub_term_code (&self) ->  u32;

    /// Specifies the last presence subscription termination reason. If presence subscription is
    /// currently active, the value will be empty.
    fn get_sub_term_reason (&self) ->  String;

    // Extended RPID information about the person.
    // fn get_rpid (&self) ->  pjrpid_element;

    // Extended presence info.
    // fn get_pres_status (&self) ->  pjsip_pres_status;
    // fn get_buf_ (&self) ->  [::std::os::raw::c_char; 512usize];
}

impl BuddyConfigExt for UABuddyConfig {

    fn set_uri(&mut self, value: String) {
        self.uri = pj_str_t::from_string(value);
    }

    fn get_uri(&self) -> String {
        self.uri.to_string()
    }

    fn set_subscribe(&mut self, value: bool) {
        self.subscribe = boolean_to_pjbool(value);
    }

    fn get_subscribe(&self) -> bool {
        check_boolean(self.subscribe)
    }
}

impl BuddyInfoExt for BuddyInfo {
    fn get_id (&self) ->  i32 {
        self.id
    }

    fn get_uri (&self) ->  String {
        self.uri.to_string()
    }

    fn get_contact (&self) ->  String {
        self.contact.to_string()
    }

    fn get_status (&self) ->  BuddyStatus {
        BuddyStatus::try_from(self.status)
        .expect("Error BuddyInfo get status")
    }

    fn get_status_text (&self) ->  String {
        self.status_text.to_string()
    }

    fn get_monitor_pres (&self) ->  bool {
        check_boolean(self.monitor_pres)
    }

    fn get_sub_state (&self) ->  SIPEvsubState {
        SIPEvsubState::try_from(self.sub_state)
        .expect("Error BuddyInfo get sub_state")
    }

    fn get_sub_term_code (&self) ->  u32 {
        self.sub_term_code
    }

    fn get_sub_term_reason (&self) ->  String {
        self.sub_term_reason.to_string()
    }
}


pub struct UABuddy { id: i32 }

impl From<i32> for UABuddy {
    fn from(id: i32) -> Self {
        Self { id }
    }
}

impl UABuddy {

    pub fn buddy_is_valid(&self) -> bool {
        unsafe {utils::check_boolean(pjsua_sys::pjsua_buddy_is_valid(self.id)) }
    }

    pub fn buddy_find(uri: String) -> i32 {
        let uri: *const pj_str_t = &mut pj_str_t::from_string(uri) as *const _;
        unsafe { pjsua_sys::pjsua_buddy_find( uri ) }
    }

    pub fn buddy_get_info(buddy_id: i32, info: &mut BuddyInfo) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_buddy_get_info( buddy_id, info as *mut _ )) }
    }

    // skiped function this mosly for trasfer data
    // i32 	pjsua_buddy_set_user_data (pjsua_buddy_id buddy_id, void *user_data)
    // void * 	pjsua_buddy_get_user_data (pjsua_buddy_id buddy_id)

    pub fn buddy_add(buddy_cfg: &mut UABuddyConfig, p_buddy_id: *mut i32) -> Result<(), i32> {
        unsafe {
            let status = pjsua_sys::pjsua_buddy_add (
                buddy_cfg as *const _,
                p_buddy_id as *mut _
            );
            utils::check_status(status)
        }
    }

    pub fn buddy_del(buddy_id: i32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_buddy_del(buddy_id)) }
    }

    pub fn buddy_subscribe_pres(buddy_id: i32, subscribe: bool) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_buddy_subscribe_pres(buddy_id, utils::boolean_to_pjbool(subscribe)))
        }
    }

    pub fn buddy_update_pres(buddy_id: i32) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_buddy_update_pres(buddy_id)) }
    }

    pub fn get_buddy_count() -> u32 {
        unsafe { pjsua_sys::pjsua_get_buddy_count() }
    }

    pub fn buddy_config_default(cfg: &mut UABuddyConfig) {
        unsafe {
            pjsua_sys::pjsua_buddy_config_default(
                cfg as *mut _
            )
        }
    }

    pub fn enum_buddies(ids: &mut [i32; pjsua_sys::PJSUA_MAX_BUDDIES as usize], count: &mut u32) -> Result<(), i32> {
        unsafe {
            utils::check_status(pjsua_sys::pjsua_enum_buddies( ids.as_mut_ptr(), count as *mut _ ))
        }
    }
    
}

// JSUA-API Buddy, Presence, and Instant Messaging


pub fn pres_notify(
    acc_id: i32,
    srv_pres: &mut pjsua_srv_pres,
    state: pjsip_evsub_state,
    state_str: String,
    reason: String,
    with_body: bool,
    msg_data: Option<&mut UAMsgData>
) -> Result<(), i32> {

    let mut state_str = pj_str_t::from_string(state_str);
    let mut reason = pj_str_t::from_string(reason);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_pres_notify(
            acc_id,
            srv_pres,
            state,
            &mut state_str as *const _,
            &mut reason as *const _,
            utils::boolean_to_pjbool(with_body),
            msg_data
        );

        utils::check_status(status)
    }
}


pub fn pres_dump(verbose: bool) {
    unsafe { pjsua_sys::pjsua_pres_dump ( utils::boolean_to_pjbool(verbose))}
}

pub fn im_send(
    acc_id: i32,
    to: String,
    mime_type: String,
    content: String,
    msg_data: Option<&mut UAMsgData>
) -> Result<(), i32> {

    let mut to = pj_str_t::from_string(to);
    let mut mime_type = pj_str_t::from_string(mime_type);
    let mut content = pj_str_t::from_string(content);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_im_send(
            acc_id,
            &mut to as *const _,
            &mut mime_type as *const _,
            &mut content as *const _,
            msg_data,
            ptr::null_mut()
        );
        utils::check_status(status)
    }

}

pub fn im_typing(
    acc_id: i32,
    to:String,
    is_typing: bool,
    msg_data: Option<&mut UAMsgData>
) -> Result<(), i32> {

    let mut to = pj_str_t::from_string(to);

    let msg_data = match msg_data {
        Some(value) => value as *const _,
        None => ptr::null_mut()
    };

    unsafe {
        let status = pjsua_sys::pjsua_im_typing(
            acc_id,
            &mut to as *const _,
            utils::boolean_to_pjbool(is_typing),
            msg_data
        );

        utils::check_status(status)
    }

}