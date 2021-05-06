use num_enum::*;

/// pub type pjsip_evsub_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPEvsubState {
    Null = pjsip_simple_sys::PJSIP_EVSUB_STATE_NULL,
    Sent = pjsip_simple_sys::PJSIP_EVSUB_STATE_SENT,
    Accepted = pjsip_simple_sys::PJSIP_EVSUB_STATE_ACCEPTED,
    Pending = pjsip_simple_sys::PJSIP_EVSUB_STATE_PENDING,
    Active = pjsip_simple_sys::PJSIP_EVSUB_STATE_ACTIVE,
    Terminated = pjsip_simple_sys::PJSIP_EVSUB_STATE_TERMINATED,
    Unknown = pjsip_simple_sys::PJSIP_EVSUB_STATE_UNKNOWN,
}

/// pub type pjrpid_activity = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPRpidActivity {
    Unknown = pjsip_simple_sys::PJRPID_ACTIVITY_UNKNOWN,
    Away = pjsip_simple_sys::PJRPID_ACTIVITY_AWAY,
    Busy = pjsip_simple_sys::PJRPID_ACTIVITY_BUSY,
}