

use super::sip_buddy::*;

pub struct SIPPresence {
    buddy_list: Vec<SIPBuddy>,
}


impl SIPPresence {
    pub fn new() -> Self {
        SIPPresence {
            buddy_list: Vec::<SIPBuddy>::new(),
        }
    }

    pub fn add_buddy() {}

    pub fn delete() {}

    pub fn send_im() {}

    pub fn subscribe() {}

    pub fn unsubscribe() {}

    pub fn toggle_state() {}

    pub fn text() {}

    pub fn list() {}
}
