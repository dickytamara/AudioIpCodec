
use super::pj_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;

#[derive(Clone, Copy)]
pub struct SIPBuddy {
    id: i32,
    ctx: pjsua_buddy_config,
}

impl SIPBuddy {

    pub fn new() -> SIPBuddy {
        SIPBuddy {
            id: -1,
            ctx: pjsua_buddy_config::new(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let status = pjsua_buddy_add(&mut self.ctx as *const _, &mut self.id as *mut _);

            if status != PJ_SUCCESS as pj_status_t {
                panic!("Panic SIPBuddy");
            }

            assert_ne!(self.id, -1);
        }
    }

    // Check if buddy ID is valid.
    pub fn is_valid(&self) -> bool {

        let result = PJ_FALSE as pj_bool_t;

        unsafe {
            let result = pjsua_buddy_is_valid(self.id);
        }

        if result == PJ_TRUE as pj_bool_t {
            true
        } else {
            false
        }
    }

    // Get detailed buddy info.
    pub fn get_info(&self) -> Result<pjsua_buddy_info, i32> {

        let mut info = pjsua_buddy_info::new();

        unsafe {

            let status = pjsua_buddy_get_info(
                    self.id,
                    &mut info as *mut _
                );

            if status == PJ_SUCCESS as pj_status_t {
                Ok(info)
            } else {
                println!("Err cant get buddy info.");
                Err(status)
            }
        }

    }

    // add buddy to internal pjsua
    pub fn add(&mut self) {

        unsafe {

            let status = pjsua_buddy_add(
                    &mut self.ctx as *const _,
                    &mut self.id as *mut _
                );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant add buddy to internal pjsua.")
            }
        }
    }

    // delete buddy from internal pjsua
    pub fn del(&self) {

        unsafe {

            let status = pjsua_buddy_del( self.id );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant delete buddy from pjsua");
            }
        }
    }

    // Enable/disable buddy's presence monitoring.
    pub fn subscribe_pres(&self, subscribe: bool) {

        let mut subs = PJ_FALSE as pj_bool_t;

        if subscribe {
            subs = PJ_TRUE as pj_bool_t;
        }

        unsafe {

            let status = pjsua_buddy_subscribe_pres(
                self.id,
                subs as pj_bool_t
            );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant subscribe presents");
            }
        }
    }

    // Update the presence information for the buddy.
    pub fn update_pres(&self) {
        unsafe {

            let status = pjsua_buddy_update_pres( self.id );

            if status != PJ_SUCCESS as pj_status_t {
                println!("ERR cant update presents.");
            }
        }
    }

}

pub struct SIPBuddys {
    buddy_list: [SIPBuddy; PJSUA_MAX_BUDDIES as usize]
}

impl SIPBuddys {

    pub fn new() -> SIPBuddys {
        SIPBuddys {
            buddy_list: [SIPBuddy::new(); PJSUA_MAX_BUDDIES as usize]
        }
    }

    // Get total number of buddies.
    pub fn get_buddy_count(&self) -> u32 {
        unsafe {
            pjsua_get_buddy_count()
        }
    }

    // Enumerate all buddy IDs in the buddy list.
    // Application then can use pjsua_buddy_get_info()
    // to get the detail information for each buddy id.
    pub fn enum_buddies() -> Vec<pjsua_buddy_id> {

        let mut ret: Vec<pjsua_buddy_id> = Vec::new();

        unsafe {
            let mut ids: [pjsua_buddy_id; PJSUA_MAX_BUDDIES as usize] = [-1; PJSUA_MAX_BUDDIES as usize];
            let mut count = 0u32;

            let status = pjsua_enum_buddies(
                    ids.as_mut_ptr(),
                    &mut count as *mut _
                );

            if status == PJ_SUCCESS as pj_status_t {
                for i in 0..count as usize {
                    ret.push(ids[i])
                }
            } else {
                println!("ERR cant enumerate buddies.");
            }
        }

        ret
    }

    // Find the buddy ID with the specified URI.
    pub fn buddy_find(&self, uri: String) -> i32 {

        unsafe {
            pjsua_buddy_find(
                &mut pj_str_t::from_string(uri) as *const _
            )
        }
    }


}
