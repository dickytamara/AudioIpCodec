

use super::pjsua_sys::*;
use super::pjdefault::AutoCreate;

impl AutoCreate<pjmedia_srtp_crypto>  for pjmedia_srtp_crypto {
    fn new() -> pjmedia_srtp_crypto {
        pjmedia_srtp_crypto {
            key: pj_str_t::new(),
            name: pj_str_t::new(),
            flags: 0
        }
    }
}


