use std::cell::RefCell;

// use pj_sys::*;
// use pjsip_simple_sys::*;
// use pjsua_sys::*;

use crate::pjproject::prelude::*;

use crate::pjproject::utils;
use crate::pjproject::pjsua;

// #[derive(Clone)]
pub struct SIPBuddy {
    id: i32,
    ctx: RefCell<BuddyConfig>,
}

pub trait SIPBuddyExt {

    /// Buddy URL or name address.
    fn set_uri(&self, value: String);
    fn get_uri(&self) -> String;

    /// Specify whether presence subscription should start immediately.
    fn set_subscribe(&self, value: bool);
    fn get_subscribe(&self) -> bool;

    // skiped
    // user_data: *mut c_void,
}

// unsigned 	pjsua_get_buddy_count (void)
// pj_status_t 	pjsua_enum_buddies (pjsua_buddy_id ids[], unsigned *count)
// pjsua_buddy_id 	pjsua_buddy_find (const pj_str_t *uri)
// void 	pjsua_pres_dump (pj_bool_t verbose)

// void 	pjsua_buddy_config_default (pjsua_buddy_config *cfg)
// pj_bool_t 	pjsua_buddy_is_valid (pjsua_buddy_id buddy_id)
// pj_status_t 	pjsua_buddy_get_info (pjsua_buddy_id buddy_id, pjsua_buddy_info *info)
// pj_status_t 	pjsua_buddy_add (const pjsua_buddy_config *buddy_cfg, pjsua_buddy_id *p_buddy_id)
// pj_status_t 	pjsua_buddy_del (pjsua_buddy_id buddy_id)
// pj_status_t 	pjsua_buddy_subscribe_pres (pjsua_buddy_id buddy_id, pj_bool_t subscribe)
// pj_status_t 	pjsua_buddy_update_pres (pjsua_buddy_id buddy_id)

// pj_status_t 	pjsua_buddy_set_user_data (pjsua_buddy_id buddy_id, void *user_data)
// void * 	pjsua_buddy_get_user_data (pjsua_buddy_id buddy_id)

// pj_status_t 	pjsua_pres_notify (pjsua_acc_id acc_id, pjsua_srv_pres *srv_pres, pjsip_evsub_state state, const pj_str_t *state_str, const pj_str_t *reason, pj_bool_t with_body, const pjsua_msg_data *msg_data)
// pj_status_t 	pjsua_im_send (pjsua_acc_id acc_id, const pj_str_t *to, const pj_str_t *mime_type, const pj_str_t *content, const pjsua_msg_data *msg_data, void *user_data)
// pj_status_t 	pjsua_im_typing (pjsua_acc_id acc_id, const pj_str_t *to, pj_bool_t is_typing, const pjsua_msg_data *msg_data)

impl SIPBuddy {

    /// create new SIPBuddy
    pub fn new() -> Self {
        SIPBuddy {
            id: -1,
            ctx: RefCell::new(pjsua_buddy_config::new()),
        }
    }

    /// construct new SIPBuddy from given buddy_id
    pub fn from(buddy_id: pjsua_buddy_id) -> Self {
        SIPBuddy {
            id: buddy_id,
            ctx: RefCell::new(pjsua_buddy_config::new())
        }
    }

    pub fn config_default(&self) {
        pjsua::buddy_config_default(&mut self.ctx.borrow_mut());
    }

    // Check if buddy ID is valid.
    pub fn is_valid(&self) -> bool {
        pjsua::buddy_is_valid(self.id)
    }

    // Get detailed buddy info.
    pub fn get_info(&self) -> Result<pjsua_buddy_info, i32> {

        let mut info = pjsua_buddy_info::new();

        if let Err(e)= pjsua::buddy_get_info( self.id, &mut info) {
            return Err(e);
        }

        Ok(info)
    }

    // add buddy to internal pjsua
    pub fn add(&mut self) {
        pjsua::buddy_add(&mut self.ctx.borrow_mut(), &mut self.id)
        .expect("SIPBuddy::pjsua_buddy_add");
    }

    // delete buddy from internal pjsua
    pub fn del(&self) {
        pjsua::buddy_del(self.id)
        .expect("SIPBuddy::pjsua_buddy_del");
    }

    // Enable/disable buddy's presence monitoring.
    pub fn subscribe_pres(&self, subscribe: bool) {
        pjsua::buddy_subscribe_pres(self.id, subscribe)
        .expect("SIPBuddy::pjsua_buddy_subscribe_pres");
    }

    // Update the presence information for the buddy.
    pub fn update_pres(&self) {
        pjsua::buddy_update_pres( self.id )
        .expect("SIPBuddy::pjsua_buddy_update_pres");
    }

}

pub struct SIPBuddys { }

impl SIPBuddys {

    pub fn new() -> Self {
        SIPBuddys { }
    }

    // Get total number of buddies.
    pub fn get_buddy_count(&self) -> u32 {
        pjsua::get_buddy_count()
    }

    // Enumerate all buddy IDs in the buddy list.
    // Application then can use pjsua_buddy_get_info()
    // to get the detail information for each buddy id.
    pub fn enum_buddies() -> Vec<pjsua_buddy_id> {

        let mut ret: Vec<pjsua_buddy_id> = Vec::new();

        let mut ids: [pjsua_buddy_id; PJSUA_MAX_BUDDIES as usize] = [-1; PJSUA_MAX_BUDDIES as usize];
        let mut count = 0u32;

        if let Ok(_) = pjsua::enum_buddies(&mut ids, &mut count) {
            
            for i in 0..count as usize {
                ret.push(ids[i])
            }
        } else {
            println!("ERR cant enumerate buddies.");
        }

        ret
    }

    // Find the buddy ID with the specified URI.
    pub fn buddy_find(&self, uri: String) -> i32 {
        pjsua::buddy_find(uri)
    }

    /// Dump presence subscriptions to log.
    pub fn pres_dump(&self, verbose: bool) {
        pjsua::pres_dump(verbose);
    }

}

impl SIPBuddyExt for SIPBuddy {

    fn set_uri(&self, value: String) {
        self.ctx.borrow_mut().uri = pj_str_t::from_string(value);
    }

    fn get_uri(&self) -> String {
        self.ctx.borrow().uri.to_string()
    }

    fn set_subscribe(&self, value: bool) {
        self.ctx.borrow_mut().subscribe = utils::boolean_to_pjbool(value);
    }

    fn get_subscribe(&self) -> bool {
        utils::check_boolean(self.ctx.borrow().subscribe)
    }
}
