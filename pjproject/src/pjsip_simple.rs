use pjsip_simple_sys::*;
use num_enum::*;


pub type SIPEvsub = pjsip_evsub;
// pub type pjsip_evsub_user = pjsip_evsub_user;
// pub type pjsip_event_hdr = pjsip_event_hdr;
// pub type pjsip_sub_state_hdr = pjsip_sub_state_hdr;
// pub type pjpidf_status_op = pjpidf_status_op;
// pub type pjpidf_tuple_op = pjpidf_tuple_op;
// pub type pjpidf_pres_op = pjpidf_pres_op;
// pub type pjpidf_op_desc = pjpidf_op_desc;
// pub type pjrpid_element = pjrpid_element;
// pub type pjsip_pres_status = pjsip_pres_status;
// pub type pjsip_pres_status__bindgen_ty_1 = pjsip_pres_status__bindgen_ty_1;
// pub type pjsip_publishc = pjsip_publishc;
// pub type pjsip_publishc_opt = pjsip_publishc_opt;
// pub type pjsip_publishc_cbparam = pjsip_publishc_cbparam;


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