
use utils::{FromString, ToString};
use std::convert::TryFrom;

use crate::pjsua;

use super::*;


pub trait SIPModuleExt {
    // To allow chaining of modules in the endpoint.
    // pub prev: *mut pjsip_module,
    // pub next: *mut pjsip_module,

    /// Module name to identify the module.
    ///
    /// This field MUST be initialized before registering the module.
    fn set_name(&mut self, value: String);
    fn get_name(&self) -> String;
    /// Module ID. Application must initialize this field with -1 before registering the
    /// module to PJSIP. After the module is registered, this field will contain a unique ID
    /// to identify the module.
    fn set_id(&mut self, value: i32);
    fn get_id(&self) -> i32;
    /// Integer number to identify module initialization and start order with regard to other
    /// modules. Higher number will make the module gets initialized later.
    ///
    /// This field MUST be initialized before registering the module.
    fn set_priority(&mut self, value: SIPModulePriority);
    fn get_priority(&self) -> SIPModulePriority;
    /// Optional function to be called to initialize the module. This function will be called
    /// by endpoint during module registration. If the value is NULL, then it's equal to
    /// returning PJ_SUCCESS.
    ///
    /// # Parameters
    /// endpt The endpoint instance.
    fn connect_load(&mut self, callback: Option<unsafe extern "C" fn(endpt: *mut SIPEndpoint) -> i32>);
    /// Optional function to be called to start the module. This function will be called by
    /// endpoint during module registration. If the value is NULL, then it's equal to
    /// returning PJ_SUCCESS.
    ///
    /// # Returns
    /// Module should return zero to indicate success.
    fn connect_start(&mut self, callback: Option<unsafe extern "C" fn() -> i32>);
    /// Optional function to be called to deinitialize the module before it is unloaded. This
    /// function will be called by endpoint during module unregistration. If the value is
    /// NULL, then it's equal to returning PJ_SUCCESS.
    ///
    /// # Returns
    /// Module should return PJ_SUCCESS to indicate success.
    fn connect_stop(&mut self, callback: Option<unsafe extern "C" fn() -> i32>);
    /// Optional function to be called to deinitialize the module before it is unloaded. This
    /// function will be called by endpoint during module unregistration. If the value is NULL,
    /// then it's equal to returning PJ_SUCCESS.
    ///
    /// # Parameters
    /// mod	The module.
    fn connect_unload(&mut self, callback: Option<unsafe extern "C" fn() -> i32>);
    /// Optional function to be called to process incoming request message.
    ///
    /// # Parameters
    /// rdata	The incoming message.
    ///
    /// # Returns
    ///     Module should return PJ_TRUE if it handles the request, or otherwise it should
    /// return PJ_FALSE to allow other modules to handle the request.
    fn connect_on_rx_request(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> i32>);
    /// Optional function to be called to process incoming response message.
    ///
    /// # Parameters
    /// rdata    The incoming message.
    ///
    /// # Returns
    ///      Module should return PJ_TRUE if it handles the response, or otherwise it should
    /// return PJ_FALSE to allow other modules to handle the response.
    fn connect_on_rx_response(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> i32>);
    /// Optional function to be called when transport layer is about to transmit outgoing request message.
    ///
    /// # Parameters
    /// tdata	The outgoing request message.
    ///
    /// # Returns
    ///     Module should return PJ_SUCCESS in all cases. If non-zero (or PJ_FALSE) is returned,
    /// the message will not be sent.
    fn connect_on_tx_request(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> i32>);
    /// Optional function to be called when transport layer is about to transmit outgoing response message.
    ///
    /// # Parameters
    /// tdata	The outgoing response message.
    ///
    /// # Returns
    ///     Module should return PJ_SUCCESS in all cases. If non-zero (or PJ_FALSE) is returned, the message will not be sent.
    fn connect_on_tx_response(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> i32>);
    /// Optional function to be called when this module is acting as transaction user for the specified
    /// transaction, when the transaction's state has changed.
    ///
    /// # Parameters
    /// - tsx	The transaction.
    /// - event	The event which has caused the transaction state to change.
    fn connect_on_tsx_state(&mut self, callback: Option<unsafe extern "C" fn(tsx: *mut pjsip_transaction, event: *mut pjsip_event)>);
}

impl SIPModuleExt for SIPModule {
    fn set_name(&mut self, value: String) {
        self.name = pj_str_t::from_string(value);
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_id(&mut self, value: i32) {
        self.id = value;
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn set_priority(&mut self, value: SIPModulePriority) {
        let val: u32 = value.into();
        self.priority = val as i32;
    }

    fn get_priority(&self) -> SIPModulePriority {
        SIPModulePriority::try_from(self.priority as u32)
        .expect("Error SIPModule as get_priority")
    }

    fn connect_load(&mut self, callback: Option<unsafe extern "C" fn(endpt: *mut pjsip_endpoint) -> pj_status_t>) {
        self.load = callback;
    }

    fn connect_start(&mut self, callback: Option<unsafe extern "C" fn() -> pj_status_t>) {
        self.start = callback;
    }

    fn connect_stop(&mut self, callback: Option<unsafe extern "C" fn() -> pj_status_t>) {
        self.stop = callback;
    }

    fn connect_unload(&mut self, callback: Option<unsafe extern "C" fn() -> pj_status_t>) {
        self.unload = callback;
    }

    fn connect_on_rx_request(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>) {
        self.on_rx_request = callback;
    }

    fn connect_on_rx_response(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>) {
        self.on_rx_response = callback;
    }

    fn connect_on_tx_request(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>) {
        self.on_tx_request = callback;
    }

    fn connect_on_tx_response(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>) {
        self.on_tx_response = callback;
    }

    fn connect_on_tsx_state(&mut self, callback: Option<unsafe extern "C" fn(tsx: *mut pjsip_transaction, event: *mut pjsip_event)>) {
        self.on_tsx_state = callback;
    }
}

pub trait SIPModuleOps {
        // pj_status_t 	pjsip_endpt_register_module (pjsip_endpoint *endpt, pjsip_module *module)
        fn register(module: &mut SIPModule) {
            endpt_register_module(pjsua::get_pjsip_endpt(), module)
            .expect("SIPModule::pjsip_endpt_register_module");
        }
        // pj_status_t 	pjsip_endpt_unregister_module (pjsip_endpoint *endpt, pjsip_module *module)
        fn unregister(module: &mut SIPModule) {
            endpt_unregister_module(pjsua::get_pjsip_endpt(), module)
            .expect("SIPModule::pjsip_endpt_unregister_module");
        }
    
        // void 	pjsip_process_rdata_param_default (pjsip_process_rdata_param *p)
        // pj_status_t 	pjsip_endpt_process_rx_data (pjsip_endpoint *endpt, pjsip_rx_data *rdata, pjsip_process_rdata_param *p, pj_bool_t *p_handled)
        // pj_pool_t * 	pjsip_endpt_create_pool (pjsip_endpoint *endpt, const char *pool_name, pj_size_t initial, pj_size_t increment)
        // void 	pjsip_endpt_release_pool (pjsip_endpoint *endpt, pj_pool_t *pool)
        // pjsip_transaction * 	pjsip_endpt_find_tsx (pjsip_endpoint *endpt, const pj_str_t *key)
        // void 	pjsip_endpt_register_tsx (pjsip_endpoint *endpt, pjsip_transaction *tsx)
        // void 	pjsip_endpt_destroy_tsx (pjsip_endpoint *endpt, pjsip_transaction *tsx)
        // pj_status_t 	pjsip_endpt_create_tdata (pjsip_endpoint *endpt, pjsip_tx_data **p_tdata)
        // pj_status_t 	pjsip_endpt_create_resolver (pjsip_endpoint *endpt, pj_dns_resolver **p_resv)
        // pj_status_t 	pjsip_endpt_set_resolver (pjsip_endpoint *endpt, pj_dns_resolver *resv)
        // pj_status_t 	pjsip_endpt_set_ext_resolver (pjsip_endpoint *endpt, pjsip_ext_resolver *ext_res)
        // pj_dns_resolver * 	pjsip_endpt_get_resolver (pjsip_endpoint *endpt)
        // void 	pjsip_endpt_resolve (pjsip_endpoint *endpt, pj_pool_t *pool, pjsip_host_info *target, void *token, pjsip_resolver_callback *cb)
        // pjsip_tpmgr * 	pjsip_endpt_get_tpmgr (pjsip_endpoint *endpt)
        // pj_ioqueue_t * 	pjsip_endpt_get_ioqueue (pjsip_endpoint *endpt)
        // pj_status_t 	pjsip_endpt_acquire_transport (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_transport **p_tp)
        // pj_status_t 	pjsip_endpt_acquire_transport2 (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_tx_data *tdata, pjsip_transport **p_tp)
        // const pjsip_hdr * 	pjsip_endpt_get_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname)
        // pj_bool_t 	pjsip_endpt_has_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname, const pj_str_t *token)
        // pj_status_t 	pjsip_endpt_add_capability (pjsip_endpoint *endpt, pjsip_module *mod, int htype, const pj_str_t *hname, unsigned count, const pj_str_t tags[])
        // const pjsip_hdr * 	pjsip_endpt_get_request_headers (pjsip_endpoint *e)
        // void 	pjsip_endpt_dump (pjsip_endpoint *endpt, pj_bool_t detail)
        // pj_status_t 	pjsip_endpt_atexit (pjsip_endpoint *endpt, pjsip_endpt_exit_callback func)
}

impl SIPModuleOps for SIPModule { }


