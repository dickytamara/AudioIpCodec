

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

impl AutoCreate<pjsip_hdr_vptr> for pjsip_hdr_vptr {
    fn new () -> pjsip_hdr_vptr {
        pjsip_hdr_vptr {
            clone: None,
            shallow_clone: None,
            print_on: None
        }
    }
}

impl AutoCreate<pjsip_hdr> for pjsip_hdr {
    fn new () -> pjsip_hdr {
        unsafe {
            let mut pjsip_hdr_z: pjsip_hdr = mem::zeroed();
            let mut pjsip_hdr_vptr_z: pjsip_hdr_vptr = mem::zeroed();
            pjsip_hdr {
                prev: &mut pjsip_hdr_z as *mut _,
                next: &mut pjsip_hdr_z as *mut _,
                type_: 0,
                name: pj_str_t::new(),
                sname: pj_str_t::new(),
                vptr: &mut pjsip_hdr_vptr_z as *mut _,
            }
        }
    }
}


impl AutoCreate<pjsip_tls_setting> for pjsip_tls_setting {
    fn new () -> pjsip_tls_setting {
        unsafe {
            let mut pj_ssl_cipher_z: pj_ssl_cipher = mem::zeroed();
            let mut pj_ssl_curve_z: pj_ssl_curve = mem::zeroed();
            pjsip_tls_setting {
                ca_list_file: pj_str_t::new(),
                ca_list_path: pj_str_t::new(),
                cert_file: pj_str_t::new(),
                privkey_file: pj_str_t::new(),
                ca_buf: pj_str_t::new() as pj_ssl_cert_buffer,
                cert_buf: pj_str_t::new() as pj_ssl_cert_buffer,
                privkey_buf: pj_str_t::new() as pj_ssl_cert_buffer,
                password: pj_str_t::new(),
                method: 0,
                proto: 0,
                ciphers_num: 0,
                ciphers: &mut pj_ssl_cipher_z as *mut _,
                curves_num: 0,
                curves: &mut pj_ssl_curve_z as *mut _,
                sigalgs: pj_str_t::new(),
                entropy_type: 0,
                entropy_path: pj_str_t::new(),
                verify_server: pj_constants__PJ_FALSE as pj_bool_t,
                verify_client: pj_constants__PJ_FALSE as pj_bool_t,
                require_client_cert: pj_constants__PJ_FALSE as pj_bool_t,
                timeout: pj_time_val::new(),
                reuse_addr: pj_constants__PJ_FALSE as pj_bool_t,
                qos_type: 0,
                qos_params: pj_qos_params::new(),
                qos_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
                sockopt_params: pj_sockopt_params::new(),
                sockopt_ignore_error: pj_constants__PJ_FALSE as pj_bool_t,
                on_accept_fail_cb: None
            }
        }
    }
}
