use pj_sys::*;
use std::ffi::{CStr, CString};

use crate::prelude::{AutoCreate, FromString, ToString};

impl FromString<pj_str_t> for pj_str_t {
    fn from_string(value: String) -> pj_str_t {
        unsafe {
            pj_str(
                CString::new(value.as_str())
                .expect("invalid string")
                .into_raw() as *mut _
            )
        }
    }
}

impl ToString for pj_str_t {
    fn to_string(&self) -> String {
        // let mut tmp: Vec<i8> = Vec::new();
        // let mut ret: String = String::new();
        // unsafe {
        //     let pointer = self.ptr;
        //     for i in 0..self.slen {
        //         tmp.push(*pointer.offset(i as isize).clone());
        //     }
        //     let ret = CStr::from_ptr(tmp.as_ptr()).to_str().expect("error").to_string().clone();
        // }
        unsafe {
            CStr::from_ptr(self.ptr).to_str().expect("error convert to_string").to_string().clone()
        }
    }
}

impl AutoCreate<pj_str_t> for pj_str_t {}
impl AutoCreate<pj_timestamp__bindgen_ty_1> for pj_timestamp__bindgen_ty_1 {}
impl AutoCreate<pj_hash_table_t> for pj_hash_table_t {}
impl AutoCreate<pj_hash_entry> for pj_hash_entry {}
impl AutoCreate<pj_hash_iterator_t> for pj_hash_iterator_t {}
impl AutoCreate<pj_ioqueue_t> for pj_ioqueue_t {}
impl AutoCreate<pj_ioqueue_key_t> for pj_ioqueue_key_t {}
impl AutoCreate<pj_timer_heap_t> for pj_timer_heap_t {}
impl AutoCreate<pj_atomic_t> for pj_atomic_t {}
impl AutoCreate<pj_thread_t> for pj_thread_t {}
impl AutoCreate<pj_lock_t> for pj_lock_t {}
impl AutoCreate<pj_grp_lock_t> for pj_grp_lock_t {}
impl AutoCreate<pj_mutex_t> for pj_mutex_t {}
impl AutoCreate<pj_sem_t> for pj_sem_t {}
impl AutoCreate<pj_event_t> for pj_event_t {}
impl AutoCreate<pj_pipe_t> for pj_pipe_t {}
impl AutoCreate<pj_time_val> for pj_time_val {}
impl AutoCreate<pj_parsed_time> for pj_parsed_time {}
impl AutoCreate<pj_ioqueue_op_key_t> for pj_ioqueue_op_key_t {}
impl AutoCreate<pj_ioqueue_callback> for pj_ioqueue_callback {}
impl AutoCreate<in_addr> for in_addr {}
impl AutoCreate<in6_addr> for in6_addr {}
impl AutoCreate<pj_sockaddr_in> for pj_sockaddr_in {}
impl AutoCreate<pj_sockaddr_in6> for pj_sockaddr_in6 {}
impl AutoCreate<pj_addr_hdr> for pj_addr_hdr {}
impl AutoCreate<pj_ip_mreq> for pj_ip_mreq {}
impl AutoCreate<pj_sockopt_params> for pj_sockopt_params {}
impl AutoCreate<pj_sockopt_params__bindgen_ty_1> for pj_sockopt_params__bindgen_ty_1 {}
impl AutoCreate<pj_activesock_t> for pj_activesock_t {}
impl AutoCreate<pj_activesock_cb> for pj_activesock_cb {}
impl AutoCreate<pj_activesock_cfg> for pj_activesock_cfg {}
impl AutoCreate<pj_hostent> for pj_hostent {}
impl AutoCreate<pj_addrinfo> for pj_addrinfo {}
impl AutoCreate<pj_exception_state_t> for pj_exception_state_t {}
impl AutoCreate<pj_fifobuf_t> for pj_fifobuf_t {}
impl AutoCreate<pj_file_stat> for pj_file_stat {}
impl AutoCreate<pj_ip_route_entry__bindgen_ty_1> for pj_ip_route_entry__bindgen_ty_1 {}
impl AutoCreate<pj_enum_ip_option> for pj_enum_ip_option {}
impl AutoCreate<pj_list> for pj_list {}
impl AutoCreate<pj_grp_lock_config> for pj_grp_lock_config {}
impl AutoCreate<pj_math_stat> for pj_math_stat {}
impl AutoCreate<pj_sys_info> for pj_sys_info {}
impl AutoCreate<pj_symbianos_params> for pj_symbianos_params {}
impl AutoCreate<pj_rwmutex_t> for pj_rwmutex_t {}
impl AutoCreate<pj_pool_block> for pj_pool_block {}
impl AutoCreate<pj_pool_t> for pj_pool_t {}
impl AutoCreate<pj_pool_factory_policy> for pj_pool_factory_policy {}
impl AutoCreate<pj_pool_factory> for pj_pool_factory {}
impl AutoCreate<pj_caching_pool> for pj_caching_pool {}
impl AutoCreate<pj_rbtree_node> for pj_rbtree_node {}
impl AutoCreate<pj_rbtree> for pj_rbtree {}
impl AutoCreate<pj_qos_params> for pj_qos_params {}
impl AutoCreate<pj_fd_set_t> for pj_fd_set_t {}
impl AutoCreate<pj_ssl_sock_t> for pj_ssl_sock_t {}
impl AutoCreate<pj_ssl_cert_t> for pj_ssl_cert_t {}
impl AutoCreate<pj_ssl_cert_info> for pj_ssl_cert_info {}
impl AutoCreate<pj_ssl_cert_info__bindgen_ty_1> for pj_ssl_cert_info__bindgen_ty_1 {}
impl AutoCreate<pj_ssl_cert_info__bindgen_ty_2> for pj_ssl_cert_info__bindgen_ty_2 {}
impl AutoCreate<pj_ssl_cert_info__bindgen_ty_3> for pj_ssl_cert_info__bindgen_ty_3 {}
impl AutoCreate<pj_ssl_cert_info__bindgen_ty_4> for pj_ssl_cert_info__bindgen_ty_4 {}
impl AutoCreate<pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1> for pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1 {}
impl AutoCreate<pj_ssl_cert_info__bindgen_ty_5> for pj_ssl_cert_info__bindgen_ty_5 {}
impl AutoCreate<pj_ssl_sock_cb> for pj_ssl_sock_cb {}
impl AutoCreate<pj_ssl_sock_info> for pj_ssl_sock_info {}
impl AutoCreate<pj_ssl_sock_param> for pj_ssl_sock_param {}
impl AutoCreate<pj_ssl_start_connect_param> for pj_ssl_start_connect_param {}
impl AutoCreate<pj_timer_entry> for pj_timer_entry {}