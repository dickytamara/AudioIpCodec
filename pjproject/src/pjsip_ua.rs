
// use super::prelude::*;
// use pjsip_sys::pjsip_generic_int_hdr;
// use pjsip_ua_sys::*;

use num_enum::*;

pub mod auto;


/// pub type pjsip_inv_state = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPInvState {
    Null = pjsip_ua_sys::PJSIP_INV_STATE_NULL,
    Calling = pjsip_ua_sys::PJSIP_INV_STATE_CALLING,
    Incoming = pjsip_ua_sys::PJSIP_INV_STATE_INCOMING,
    Early = pjsip_ua_sys::PJSIP_INV_STATE_EARLY,
    Connecting = pjsip_ua_sys::PJSIP_INV_STATE_CONNECTING,
    Confirmed = pjsip_ua_sys::PJSIP_INV_STATE_CONFIRMED,
    Disconnected = pjsip_ua_sys::PJSIP_INV_STATE_DISCONNECTED,
}

/// pub type pjsip_inv_option = u32;
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SIPInvOption {
    Support100rel = pjsip_ua_sys::PJSIP_INV_SUPPORT_100REL,
    SupportTimer = pjsip_ua_sys::PJSIP_INV_SUPPORT_TIMER,
    SupportUpdate = pjsip_ua_sys::PJSIP_INV_SUPPORT_UPDATE,
    SupportIce = pjsip_ua_sys::PJSIP_INV_SUPPORT_ICE,
    RequireIce = pjsip_ua_sys::PJSIP_INV_REQUIRE_ICE,
    Require100rel = pjsip_ua_sys::PJSIP_INV_REQUIRE_100REL,
    RequireTimer = pjsip_ua_sys::PJSIP_INV_REQUIRE_TIMER,
    AlwaysUseTimer = pjsip_ua_sys::PJSIP_INV_ALWAYS_USE_TIMER,
    SupportTrickleIce = pjsip_ua_sys::PJSIP_INV_SUPPORT_TRICKLE_ICE,
    RequireTrickleIce = pjsip_ua_sys::PJSIP_INV_REQUIRE_TRICKLE_ICE,
}