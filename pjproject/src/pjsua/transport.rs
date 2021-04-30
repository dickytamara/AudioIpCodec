

pub trait TransportConfigExt {

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