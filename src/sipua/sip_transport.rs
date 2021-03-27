

use super::pj_sys::*;
use super::pjsip_sys::*;
use super::pjsua_sys::*;

use super::pjdefault::*;

use super::pjsua;
use std::ptr;
use std::ffi::CString;


// Transport wrapper
pub struct SIPTransport {
    id: i32,
    acc_id: i32,
}

impl SIPTransport {
    pub fn new() -> SIPTransport {
        SIPTransport { id: -1, acc_id: -1 }
    }

    // start create the transport
    pub fn init(
        &mut self,
        type_: pjsip_transport_type_e,
        config: &mut pjsua_transport_config,
        rtp_config: *const pjsua_transport_config,
    ) {

        let mut status = pjsua::transport_create(type_, config, Some(&mut self.id));
        if status != PJ_SUCCESS as i32 {
            panic!("cant create transport.")
        }

        assert_ne!(self.id, -1);

        status = pjsua::acc_add_local(
            self.id,
            true,
            &mut self.acc_id,
        );

        if status != PJ_SUCCESS as i32 {
            panic!("cant init transport");
        }

        assert_ne!(self.acc_id, -1);

        let mut acc_cfg = pjsua_acc_config::new();
        pjsua::acc_get_config(self.acc_id, &mut acc_cfg);

        unsafe {
            acc_cfg.rtp_cfg = *rtp_config;
            if type_ == pjsip_transport_type_e_PJSIP_TRANSPORT_TCP6
                || type_ == pjsip_transport_type_e_PJSIP_TRANSPORT_UDP6
            {
                acc_cfg.ipv6_media_use = pjsua_ipv6_use_PJSUA_IPV6_ENABLED;
            }
        }

        pjsua::acc_modify(self.acc_id, &mut acc_cfg);
        pjsua::acc_set_online_status(pjsua::acc_get_default(), true);
    }

    pub fn get_info(&self) -> Result<*const pjsua_transport_info, i32> {
        unsafe {
            let info: *mut pjsua_transport_info = ptr::null_mut();
            let status: pj_status_t = pjsua_transport_get_info(self.id, info);

            if status != PJ_SUCCESS as i32 {
                return Err(status);
            }

            Ok(info)
        }
    }

    pub fn set_enable(&self, enabled: bool) {
        unsafe {
            let mut e = PJ_FALSE;
            if enabled {
                e = PJ_TRUE;
            }

            let status = pjsua_transport_set_enable(self.id, e as i32);

            if status != PJ_SUCCESS as i32 {
                panic!("cant set enable transport");
            }
        }
    }
}

impl Drop for SIPTransport {
    fn drop(&mut self) {
        unsafe {
            pjsua_transport_close(self.id, PJ_TRUE as i32);
        }
    }
}

pub struct SIPTransports {
    transport_list: Vec<SIPTransport>,
    udp_cfg: pjsua_transport_config,
    rtp_cfg: pjsua_transport_config,
}

impl SIPTransports {

    pub fn new() -> SIPTransports {

        let mut sip_transports = SIPTransports {
            transport_list: Vec::<SIPTransport>::new(),
            udp_cfg: pjsua_transport_config::new(),
            rtp_cfg: pjsua_transport_config::new(),
        };

        sip_transports.udp_cfg.port = 5060;
        sip_transports.rtp_cfg.port = 4000;

        sip_transports
    }

    pub fn add(&mut self, transport_type: u32) {
        let mut transport = SIPTransport::new();
        transport.init(
            transport_type,
            &mut self.udp_cfg,
            &mut self.rtp_cfg as *const _,
        );
        self.transport_list.push(transport);
    }

    pub fn get_rtp_config(&self) -> pjsua_transport_config {
        self.rtp_cfg
    }

    pub fn delete(&mut self, transport_id: i32) {
        // TODO
    }

    pub fn set_ca_list_file(&mut self, file_path: &str) {
        unsafe {
            self.udp_cfg.tls_setting.ca_list_file =
                pj_str(CString::new(file_path).expect("error").into_raw());
        }
    }

    pub fn set_cert_file(&mut self, file_path: &str) {
        unsafe {
            self.udp_cfg.tls_setting.cert_file =
                pj_str(CString::new(file_path).expect("error").into_raw());
        }
    }

    pub fn set_privkey_file(&mut self, file_path: &str) {
        unsafe {
            self.udp_cfg.tls_setting.privkey_file =
                pj_str(CString::new(file_path).expect("error").into_raw());
        }
    }

    pub fn set_password(&mut self, password: &str) {
        unsafe {
            self.udp_cfg.tls_setting.password =
                pj_str(CString::new(password).expect("error").into_raw());
        }
    }

    pub fn set_tls_verify_server(&mut self, value: bool) {
        self.udp_cfg.tls_setting.verify_server = if value {
            PJ_TRUE as pj_bool_t
        } else {
            PJ_FALSE as pj_bool_t
        }
    }

    pub fn set_tls_verify_client(&mut self, value: bool) {
        let val = if value {
            PJ_TRUE as pj_bool_t
        } else {
            PJ_FALSE as pj_bool_t
        };

        self.udp_cfg.tls_setting.verify_client = val;
        self.udp_cfg.tls_setting.require_client_cert = val;
    }

}

