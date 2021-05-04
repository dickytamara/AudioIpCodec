
use pjproject::pjsua::*;

use crate::pjproject::prelude::*;
use crate::pjproject::pjsua;

pub struct SIPBuddy {
    id: i32,
    config: pjsua::BuddyConfig,
}


impl SIPBuddy {

    /// create new SIPBuddy
    pub fn new() -> Self {
        Self {
            id: -1,
            config: BuddyConfig::new(),
        }
    }

    /// construct new SIPBuddy from given buddy_id
    pub fn from(buddy_id: i32) -> Self {
        SIPBuddy {
            id: buddy_id,
            config: BuddyConfig::new()
        }
    }

    pub fn config_default(&self) {
        pjsua::buddy_config_default(&mut self.config);
    }

    // Check if buddy ID is valid.
    pub fn is_valid(&self) -> bool {
        pjsua::buddy_is_valid(self.id)
    }

    // Get detailed buddy info.
    pub fn get_info(&self) -> Result<BuddyInfo, i32> {

        let mut info = BuddyInfo::new();

        if let Err(e)= pjsua::buddy_get_info( self.id, &mut info) {
            return Err(e);
        }

        Ok(info)
    }

    // add buddy to internal pjsua
    pub fn add(&mut self) {
        pjsua::buddy_add(&mut self.config, &mut self.id)
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
    pub fn enum_buddies() -> Vec<i32> {

        let mut ret: Vec<i32> = Vec::new();

        let mut ids: [i32; MAX_BUDDIES] = [-1; MAX_BUDDIES];
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
