

use pj_sys::*;
use pjsip_sys::*;
use pjsua_sys::*;

use crate::pjproject::utils::{AutoCreate, FromString, ToString, boolean_to_pjbool};
use crate::pjproject::pjsua;

use std::cell::{RefCell, RefMut};
use std::path::PathBuf;


#[derive(Clone)]
pub struct SIPTransportConfig {
    ctx: RefCell<pjsua_transport_config>
}

impl SIPTransportConfig {
    pub fn new() -> Self {
        SIPTransportConfig {
            ctx: RefCell::new(pjsua_transport_config::new())
        }
    }

    pub fn default(&self) {
        pjsua::transport_config_default(&mut self.ctx.borrow_mut());
        self.ctx.borrow_mut().port = 5060;
    }

    pub fn get_context(&self) -> RefMut<pjsua_transport_config> {
        self.ctx.borrow_mut()
    }
}

pub trait SIPTransportConfigExt {

    fn set_port(&self, value: u32);
    fn get_port(&self) -> u32;

    fn set_port_range(&self, value: u32);
    fn get_port_range(&self) -> u32;

    fn set_public_addr(&self, value: String);
    fn get_public_addr(&self) -> String;

    fn set_bound_addr(&self, value: String);
    fn get_bound_addr(&self) -> String;

    fn set_qos_type(&self, value: u32);
    fn get_qos_type(&self) -> u32;

    // pub tls_setting: pjsip_tls_setting,
    // pub qos_params: pj_qos_params,
    // pub sockopt_params: pj_sockopt_params,
}


// Transport wrapper
pub struct SIPTransport {
    id: i32,
    config: SIPTransportConfig
}

impl SIPTransport {

    pub fn new(
        type_: pjsip_transport_type_e,
        config: Option<&SIPTransportConfig>
    ) -> Self {

        let mut transport = SIPTransport{
            id: -1_i32,
            config: SIPTransportConfig::new()
        };

        transport.config.default();

        match config {
            Some(tp_cfg) => {
                transport.config = tp_cfg.clone();
            }, None => ()
        }

        pjsua::transport_create(type_, &mut transport.config.get_context(), Some(&mut transport.id))
        .expect("SIPTransport::transport_create");

        transport
    }

    // pub fn from()

    pub fn get_info(&self) -> Result<*const pjsua_transport_info, i32> {
        let mut info: pjsua_transport_info = pjsua_transport_info::new();

        pjsua::transport_get_info(self.id, &mut info)
        .expect("SIPTransport::pjsua_get_info");

        Ok(&mut info as *const _)
    }

    pub fn set_enable(&self, enabled: bool) {
        pjsua::transport_set_enable(self.id, enabled)
        .expect("SIPTransport::pjsua_set_enable");
    }


    pub fn get_id(&self) -> i32 {
        self.id
    }
}

// void 	pjsua_transport_config_default (pjsua_transport_config *cfg)
// pj_status_t 	pjsua_transport_set_enable (pjsua_transport_id id, pj_bool_t enabled)
// pj_status_t 	pjsua_transport_get_info (pjsua_transport_id id, pjsua_transport_info *info)
// pj_status_t 	pjsua_transport_close (pjsua_transport_id id, pj_bool_t force)
// void 	pjsua_transport_config_dup (pj_pool_t *pool, pjsua_transport_config *dst, const pjsua_transport_config *src)

// pj_status_t 	pjsua_transport_create (pjsip_transport_type_e type, const pjsua_transport_config *cfg, pjsua_transport_id *p_id)
// pj_status_t 	pjsua_transport_register (pjsip_transport *tp, pjsua_transport_id *p_id)
// pj_status_t 	pjsua_tpfactory_register (pjsip_tpfactory *tf, pjsua_transport_id *p_id)
// pj_status_t 	pjsua_enum_transports (pjsua_transport_id id[], unsigned *count)
// pj_status_t 	pjsua_transport_lis_start (pjsua_transport_id id, const pjsua_transport_config *cfg)

impl Drop for SIPTransport {
    fn drop(&mut self) {
        pjsua::transport_close(self.id, true)
        .expect("SIPTransport::pjsua_trasport_close");
    }
}

pub struct SIPTransports {
    pub list: Vec<SIPTransport>,
    config: SIPTransportConfig,
}

impl SIPTransports {

    pub fn new() -> Self {
        SIPTransports {
            list: Vec::<SIPTransport>::new(),
            config: SIPTransportConfig::new(),
        }
    }

    pub fn add(&mut self, transport_type: u32) {
        let transport = SIPTransport::new(
            transport_type,
            Some(&self.config)
        );
        self.list.push(transport);
    }

    pub fn delete(&mut self, transport_id: i32) {
        // TODO
    }


    pub fn set_ca_list_file(&self, path: PathBuf) {
        self.config.ctx.borrow_mut()
        .tls_setting.ca_list_file = pj_str_t::from_string(
            path.to_str().unwrap().to_string()
        );
    }

    pub fn set_cert_file(&self, path: PathBuf) {
        self.config.ctx.borrow_mut()
        .tls_setting.cert_file = pj_str_t::from_string(
            path.to_str().unwrap().to_string()
        );
    }

    pub fn set_privkey_file(&self, path: PathBuf) {
        self.config.ctx.borrow_mut()
        .tls_setting.privkey_file = pj_str_t::from_string(
            path.to_str().unwrap().to_string()
        );
    }

    pub fn set_password(&self, password: String) {
        self.config.ctx.borrow_mut()
            .tls_setting.password = pj_str_t::from_string(password);
    }

    pub fn set_tls_verify_server(&self, value: bool) {
        self.config.ctx.borrow_mut()
        .tls_setting.verify_server = boolean_to_pjbool(value);
    }

    pub fn set_tls_verify_client(&self, value: bool) {
        self.config.ctx.borrow_mut()
        .tls_setting.verify_client = boolean_to_pjbool(value);

        self.config.ctx.borrow_mut()
        .tls_setting.require_client_cert = boolean_to_pjbool(value);
    }
}


impl SIPTransportConfigExt for SIPTransportConfig {
    fn set_port(&self, value: u32) {
        self.ctx.borrow_mut().port = value;
    }

    fn get_port(&self) -> u32 {
        self.ctx.borrow().port
    }

    fn set_port_range(&self, value: u32) {
        self.ctx.borrow_mut().port_range = value;
    }

    fn get_port_range(&self) -> u32 {
        self.ctx.borrow().port_range
    }

    fn set_public_addr(&self, value: String) {
        self.ctx.borrow_mut().public_addr = pj_str_t::from_string(value);
    }

    fn get_public_addr(&self) -> String {
        self.ctx.borrow().public_addr.to_string()
    }

    fn set_bound_addr(&self, value: String) {
        self.ctx.borrow_mut().bound_addr = pj_str_t::from_string(value);
    }

    fn get_bound_addr(&self) -> String {
        self.ctx.borrow().bound_addr.to_string()
    }

    fn set_qos_type(&self, value: u32) {
        self.ctx.borrow_mut().qos_type = value;
    }

    fn get_qos_type(&self) -> u32 {
        self.ctx.borrow().qos_type
    }
}
