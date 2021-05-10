
use std::convert::TryFrom;
use crate::{pj::QOSType, pjsip::{SIPTransportFlags, SIPTransportType}, utils::AutoDefault};

use super::*;

pub trait UATransportConfigExt {

    /// UDP port number to bind locally. This setting MUST be specified even when default
    /// port is desired. If the value is zero, the transport will be bound to any
    /// available port, and application can query the port by querying the transport info.
    fn set_port(&mut self, value: u32);
    fn get_port(&self) -> u32;

    /// Specify the port range for socket binding, relative to the start port number
    /// specified in port. Note that this setting is only applicable when the start port
    /// number is non zero.
    ///
    /// # default
    /// value zero.
    fn set_port_range(&mut self, value: u32);
    fn get_port_range(&self) -> u32;

    /// Optional address to advertise as the address of this transport. Application can
    /// specify any address or hostname for this field, for example it can point to one of
    /// the interface address in the system, or it can point to the public address of a NAT
    /// router where port mappings have been configured for the application.
    ///
    /// # note:
    ///
    /// this option can be used for both UDP and TCP as well!
    fn set_public_addr(&mut self, value: String);
    fn get_public_addr(&self) -> String;


    /// Optional address where the socket should be bound to. This option SHOULD only be
    /// used to selectively bind the socket to particular interface (instead of 0.0.0.0),
    /// and SHOULD NOT be used to set the published address of a transport (the public_addr
    /// field should be used for that purpose).
    ///
    /// # note:
    ///
    /// that unlike public_addr field, the address (or hostname) here MUST correspond to the
    /// actual interface address in the host, since this address will be specified as bind()
    /// argument.
    fn set_bound_addr(&mut self, value: String);
    fn get_bound_addr(&self) -> String;

    /// QoS traffic type to be set on this transport. When application wants to apply QoS
    /// tagging to the transport, it's preferable to set this field rather than qos_param
    /// fields since this is more portable.
    ///
    /// # default
    /// QoS not set.
    ///
    fn set_qos_type(&mut self, value: QOSType);
    fn get_qos_type(&self) -> QOSType;

    // fn get_tls_setting(&self) -> pjsip_tls_setting;
    // fn get_qos_params(&self) -> pj_qos_params,
    // fn get_sockopt_params(&self) -> pj_sockopt_params,


}

impl AutoDefault<UATransportConfig> for UATransportConfig {
    fn default() -> Self {
        unsafe {
            let mut cfg = UATransportConfig::new();
            pjsua_sys::pjsua_transport_config_default(&mut cfg as *mut _);
            cfg
        }
    }
}

// read only implementation
pub trait UATransportInfoExt {

    /// PJSUA transport identification.
    fn get_id (&self) -> i32;

    /// Transport type.
    fn get_type (&self) -> SIPTransportType;

    /// Transport type name.
    fn get_type_name (&self) -> String;

    /// Transport string info/description.
    fn get_info (&self) -> String;

    /// Transport flag (see #TrasportFlags).
    fn get_flag (&self) -> SIPTransportFlags;

    /// Local address length.
    fn get_addr_len (&self) -> u32;

    // local_addr represented by TranportType
    // fn get_local_addr(&self) -> pj_sockaddr;

    /// Published address (or transport address name).
    fn get_local_name (&self) -> (String, i32);

    /// Current number of objects currently referencing this transport.
    fn get_usage_count (&self) -> u32;
}

impl UATransportConfigExt for UATransportConfig {

    fn set_port(&mut self, value: u32) {
        self.port = value;
    }

    fn get_port(&self) -> u32 {
        self.port
    }

    fn set_port_range(&mut self, value: u32) {
        self.port_range = value;
    }

    fn get_port_range(&self) -> u32 {
        self.port_range
    }

    fn set_public_addr(&mut self, value: String) {
        self.public_addr = pj_str_t::from_string(value);
    }

    fn get_public_addr(&self) -> String {
        self.public_addr.to_string()
    }

    fn set_bound_addr(&mut self, value: String) {
        self.bound_addr = pj_str_t::from_string(value);
    }

    fn get_bound_addr(&self) -> String {
        self.bound_addr.to_string()
    }

    fn set_qos_type(&mut self, value: QOSType) {
        self.qos_type = value.into()
    }

    fn get_qos_type(&self) -> QOSType {
        QOSType::try_from(self.qos_type)
        .expect("Error TrasportConfig get qos_type")
    }
}

impl UATransportInfoExt for UATransportInfo {
    fn get_id (&self) -> i32 {
        self.id
    }

    fn get_type (&self) -> SIPTransportType {
        SIPTransportType::try_from(self.type_)
        .expect("Error TransportInfo get type_")
    }

    fn get_type_name (&self) -> String {
        self.type_name.to_string()
    }

    fn get_info (&self) -> String {
        self.info.to_string()
    }

    fn get_flag (&self) -> SIPTransportFlags {
        SIPTransportFlags::try_from(self.flag)
        .expect("Error TransportInfo get flag")
    }

    fn get_addr_len (&self) -> u32 {
        self.addr_len
    }

    fn get_local_name (&self) -> (String, i32) {
        (
            self.local_name.host.to_string(),
            self.local_name.port
        )
    }

    fn get_usage_count (&self) -> u32 {
        self.usage_count
    }
}

pub struct UATransport { id: i32 }

impl From<i32> for UATransport {
    fn from (id: i32) -> Self {
        Self { id }
    }
}

impl UATransport {

    pub fn new(type_: SIPTransportType, cfg: &UATransportConfig) -> Self {
        UATransport::transport_create(type_, cfg).unwrap()
    }

    pub fn get_info(&self) -> Result<UATransportInfo, i32> {
        unsafe {
            let mut info = UATransportInfo::new();
            match utils::check_status(pjsua_sys::pjsua_transport_get_info(self.id, &mut info as *mut _)) {
                Ok(()) => { return Ok(info); },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn set_enable(&self, enabled: bool) -> Result<(), i32> {
        unsafe {
            utils::check_status(
                pjsua_sys::pjsua_transport_set_enable(self.id, utils::boolean_to_pjbool(enabled))
            )
        }
    }

    pub fn close (&self, force: bool) -> Result<(), i32> {
        unsafe {
            utils::check_status(
                pjsua_sys::pjsua_transport_close (self.id, utils::boolean_to_pjbool(force))
            )
        }
    }

    pub fn enum_transports() -> Result<Vec<UATransport>, i32> {
        unsafe {
            let mut id: [i32; PJSIP_MAX_TRANSPORTS as usize] = [-1; PJSIP_MAX_TRANSPORTS as usize];
            let mut count = 0_u32;
            let status = pjsua_sys::pjsua_enum_transports( id.as_mut_ptr(), &mut count as *mut _);

            match utils::check_status(status) {
                Ok(()) => {
                    let mut vec = Vec::<UATransport>::new();

                    for i in 0..count as usize {
                        vec.push(UATransport::from(id[i]));
                    }

                    return Ok(vec);
                },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn transport_lis_start(&self, cfg: &mut UATransportConfig) -> Result<(), i32> {
        unsafe { utils::check_status(pjsua_sys::pjsua_transport_lis_start(self.id, cfg as *const _)) }
    }

    pub fn transport_create(type_: SIPTransportType, cfg: &UATransportConfig) -> Result<UATransport, i32> {
        unsafe {
            let mut p_id = -1_i32;
            let status = pjsua_sys::pjsua_transport_create(
                type_.into(),
                cfg as *const _,
                &mut p_id as *mut _
            );

            match utils::check_status(status) {
                Ok(()) => { return Ok(UATransport::from(p_id)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn config_dup(dst: &mut UATransportConfig, src: &mut UATransportConfig) {
        let pool = pool_create("tmp-pool");

        unsafe {
            pjsua_sys::pjsua_transport_config_dup(
                pool,
                dst as *mut _,
                src as *mut _
            );
        }

        pool_release(pool)
    }

    pub fn transport_register(tp: &mut pjsip_transport) -> Result<UATransport, i32> {
        unsafe {
            let mut p_id = -1_i32;
            let status = pjsua_sys::pjsua_transport_register( tp as *mut _, &mut p_id as *mut _);

            match utils::check_status(status) {
                Ok(()) => { return Ok(UATransport::from(p_id)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn tpfactory_register(tf: &mut pjsip_tpfactory) -> Result<UATransport, i32> {
        unsafe {
            let mut p_id = -1_i32;
            let status = pjsua_sys::pjsua_tpfactory_register( tf as *mut _, &mut p_id as *mut _);

            match utils::check_status(status) {
                Ok(()) => { return Ok(UATransport::from(p_id)); },
                Err(e) => { return Err(e); }
            }

        }
    }

}













