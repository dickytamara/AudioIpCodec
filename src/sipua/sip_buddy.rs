
use super::pj_sys::*;
use super::pjsip_simple_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;

use super::pjsua;

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

        if let Err(_)= pjsua::buddy_add(&mut self.ctx, &mut self.id) {
            panic!("Panic SIPBuddy");
        }

        assert_ne!(self.id, -1);
    }

    // Check if buddy ID is valid.
    pub fn is_valid(&self) -> bool {

        pjsua::buddy_is_valid(self.id)
    }

    // Get detailed buddy info.
    pub fn get_info(&self) -> Result<pjsua_buddy_info, i32> {

        let mut info = pjsua_buddy_info::new();

        if let Err(e)= pjsua::buddy_get_info( self.id, &mut info) {
            println!("Err cant get buddy info.");
            return Err(e);
        }

        Ok(info)
    }

    // add buddy to internal pjsua
    pub fn add(&mut self) {

        if let Err(_) = pjsua::buddy_add(&mut self.ctx, &mut self.id) {
            println!("ERR cant add buddy to internal pjsua.")
        }
    }

    // delete buddy from internal pjsua
    pub fn del(&self) {

        if let Err(_) = pjsua::buddy_del(self.id) {
            println!("ERR cant delete buddy from pjsua");
        }
    }

    // Enable/disable buddy's presence monitoring.
    pub fn subscribe_pres(&self, subscribe: bool) {

        if let Err(_) = pjsua::buddy_subscribe_pres(self.id, subscribe) {
            println!("ERR cant subscribe presents");
        }

    }

    // Update the presence information for the buddy.
    pub fn update_pres(&self) {

        if let Err(_) = pjsua::buddy_update_pres( self.id ) {
            println!("ERR cant update presents.");
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


}
