

use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;
use std::mem;


impl AutoCreate<pjsip_cred_info> for pjsip_cred_info {
    fn new() -> pjsip_cred_info {
        let extmem: pjsip_cred_info__bindgen_ty_1 = unsafe { mem::zeroed() };
        pjsip_cred_info {
            realm: pj_str_t::new(),
            scheme: pj_str_t::new(),
            username: pj_str_t::new(),
            data_type: 0,
            data: pj_str_t::new(),
            ext: extmem,
        }
    }
}
