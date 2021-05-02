
use crate::utils::{boolean_to_pjbool, check_boolean};
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
    fn get_sub_state (&self) ->  BuddyEvsubState;

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

impl BuddyConfigExt for BuddyConfig {

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

    fn get_sub_state (&self) ->  BuddyEvsubState {
        BuddyEvsubState::try_from(self.sub_state)
        .expect("Error BuddyInfo get sub_state")
    }

    fn get_sub_term_code (&self) ->  u32 {
        self.sub_term_code
    }

    fn get_sub_term_reason (&self) ->  String {
        self.sub_term_reason.to_string()
    }
}

