extern crate bindgen;
use std::env;

fn main () {

    println!("cargo:rustc-link-lib=pjnath");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let pjnath = bindgen::Builder::default().header("wrapper.h")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("extern crate pj_sys;")
        .raw_line("extern crate pjlib_util_sys;")
        .raw_line("use pj_sys::*;")
        .raw_line("use pjlib_util_sys::*;")

        // pj-sys
        // struct
        .blocklist_type("pj_timestamp")
        .blocklist_type("pj_sockaddr")
        .blocklist_type("pj_str_t")
        .blocklist_type("pj_timestamp__bindgen_ty_1")
        .blocklist_type("pj_hash_table_t")
        .blocklist_type("pj_hash_entry")
        .blocklist_type("pj_hash_iterator_t")
        .blocklist_type("pj_ioqueue_t")
        .blocklist_type("pj_ioqueue_key_t")
        .blocklist_type("pj_timer_heap_t")
        .blocklist_type("pj_atomic_t")
        .blocklist_type("pj_thread_t")
        .blocklist_type("pj_lock_t")
        .blocklist_type("pj_grp_lock_t")
        .blocklist_type("pj_mutex_t")
        .blocklist_type("pj_sem_t")
        .blocklist_type("pj_event_t")
        .blocklist_type("pj_pipe_t")
        .blocklist_type("pj_time_val")
        .blocklist_type("pj_parsed_time")
        .blocklist_type("pj_ioqueue_op_key_t")
        .blocklist_type("pj_ioqueue_callback")
        .blocklist_type("__sigset_t")
        .blocklist_type("in_addr.*")
        .blocklist_type("in6_addr.*")
        .blocklist_type("pj_sockaddr_in")
        .blocklist_type("pj_sockaddr_in6")
        .blocklist_type("pj_addr_hdr")
        .blocklist_type("pj_ip_mreq")
        .blocklist_type("pj_sockopt_params")
        .blocklist_type("pj_sockopt_params__bindgen_ty_1")
        .blocklist_type("pj_activesock_t")
        .blocklist_type("pj_activesock_cb")
        .blocklist_type("pj_activesock_cfg")
        .blocklist_type("pj_hostent")
        .blocklist_type("pj_addrinfo")
        .blocklist_type("__jmp_buf_tag")
        .blocklist_type("pj_exception_state_t")
        .blocklist_type("pj_fifobuf_t")
        .blocklist_type("pj_file_stat")
        .blocklist_type("pj_ip_route_entry__bindgen_ty_1")
        .blocklist_type("pj_enum_ip_option")
        .blocklist_type("pj_list")
        .blocklist_type("pj_grp_lock_config")
        .blocklist_type("pj_math_stat")
        .blocklist_type("pj_sys_info")
        .blocklist_type("pj_symbianos_params")
        .blocklist_type("pj_rwmutex_t")
        .blocklist_type("pj_pool_block")
        .blocklist_type("pj_pool_t")
        .blocklist_type("pj_pool_factory_policy")
        .blocklist_type("pj_pool_factory")
        .blocklist_type("pj_caching_pool")
        .blocklist_type("pj_rbtree_node")
        .blocklist_type("pj_rbtree")
        .blocklist_type("pj_qos_params")
        .blocklist_type("pj_fd_set_t")
        .blocklist_type("pj_ssl_sock_t")
        .blocklist_type("pj_ssl_cert_t")
        .blocklist_type("pj_ssl_cert_info")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_1")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_2")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_3")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_4")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_5")
        .blocklist_type("pj_ssl_sock_cb")
        .blocklist_type("pj_ssl_sock_info")
        .blocklist_type("pj_ssl_sock_param")
        .blocklist_type("pj_ssl_start_connect_param")
        .blocklist_type("pj_timer_entry")
        .blocklist_type("__va_list_tag")

        // pj-sys
        // type
        .blocklist_type("pj_int64_t")
        .blocklist_type("pj_uint64_t")
        .blocklist_type("size_t")
        .blocklist_type("pj_int32_t")
        .blocklist_type("pj_uint32_t")
        .blocklist_type("pj_int16_t")
        .blocklist_type("pj_uint16_t")
        .blocklist_type("pj_int8_t")
        .blocklist_type("pj_uint8_t")
        .blocklist_type("pj_size_t")
        .blocklist_type("pj_ssize_t")
        .blocklist_type("pj_status_t")
        .blocklist_type("pj_bool_t")
        .blocklist_type("pj_char_t")
        .blocklist_type("pj_constants_")
        .blocklist_type("pj_off_t")
        .blocklist_type("pj_list_type")
        .blocklist_type("pj_atomic_value_t")
        .blocklist_type("pj_oshandle_t")
        .blocklist_type("pj_sock_t")
        .blocklist_type("pj_sockaddr_t")
        .blocklist_type("pj_color_t")
        .blocklist_type("pj_exception_id_t")
        .blocklist_type("pj_exit_callback")
        .blocklist_type("pj_ioqueue_operation_e")
        .blocklist_type("__uint8_t")
        .blocklist_type("__uint16_t")
        .blocklist_type("__uint32_t")
        .blocklist_type("in_addr_t")
        .blocklist_type("pj_socket_sd_type")
        .blocklist_type("pj_in_addr")
        .blocklist_type("pj_in6_addr")
        .blocklist_type("pj_os_err_type")
        .blocklist_type("va_list")
        .blocklist_type("pj_error_callback")
        .blocklist_type("__jmp_buf")
        .blocklist_type("jmp_buf")
        .blocklist_type("pj_jmp_buf")
        .blocklist_type("pj_log_decoration")
        .blocklist_type("pj_log_func")
        .blocklist_type("pj_file_access")
        .blocklist_type("pj_file_seek_type")
        .blocklist_type("pj_hash_entry_buf")
        .blocklist_type("pj_highprec_t")
        .blocklist_type("pj_sys_info_flag")
        .blocklist_type("pj_thread_create_flags")
        .blocklist_type("pj_thread_proc")
        .blocklist_type("pj_thread_desc")
        .blocklist_type("pj_mutex_type_e")
        .blocklist_type("pj_main_func_ptr")
        .blocklist_type("pj_pool_callback")
        .blocklist_type("pj_rbcolor_t")
        .blocklist_type("pj_rbtree_comp")
        .blocklist_type("pj_qos_type")
        .blocklist_type("pj_qos_flag")
        .blocklist_type("pj_qos_wmm_prio")
        .blocklist_type("pj_ssl_cert_verify_flag_t")
        .blocklist_type("pj_ssl_cert_name_type")
        .blocklist_type("pj_ssl_cert_buffer")
        .blocklist_type("pj_ssl_cipher")
        .blocklist_type("pj_ssl_curve")
        .blocklist_type("pj_ssl_entropy")
        .blocklist_type("pj_ssl_sock_proto")
        .blocklist_type("pj_timer_id_t")
        .blocklist_type("pj_timer_heap_callback")
        .blocklist_type("__builtin_va_list")

        // pj-sys
        // pub static
        .blocklist_item("PJ_VERSION")
        .blocklist_item("PJ_AF_UNSPEC")
        .blocklist_item("PJ_AF_UNIX")
        .blocklist_item("PJ_AF_INET")
        .blocklist_item("PJ_AF_INET6")
        .blocklist_item("PJ_AF_PACKET")
        .blocklist_item("PJ_AF_IRDA")
        .blocklist_item("PJ_SOCK_STREAM")
        .blocklist_item("PJ_SOCK_DGRAM")
        .blocklist_item("PJ_SOCK_RAW")
        .blocklist_item("PJ_SOCK_RDM")
        .blocklist_item("PJ_SOL_SOCKET")
        .blocklist_item("PJ_SOL_IP")
        .blocklist_item("PJ_SOL_TCP")
        .blocklist_item("PJ_SOL_UDP")
        .blocklist_item("PJ_SOL_IPV6")
        .blocklist_item("PJ_IP_TOS")
        .blocklist_item("PJ_IPTOS_LOWDELAY")
        .blocklist_item("PJ_IPTOS_THROUGHPUT")
        .blocklist_item("PJ_IPTOS_RELIABILITY")
        .blocklist_item("PJ_IPTOS_MINCOST")
        .blocklist_item("PJ_IPV6_TCLASS")
        .blocklist_item("PJ_SO_TYPE")
        .blocklist_item("PJ_SO_RCVBUF")
        .blocklist_item("PJ_SO_SNDBUF")
        .blocklist_item("PJ_TCP_NODELAY")
        .blocklist_item("PJ_SO_REUSEADDR")
        .blocklist_item("PJ_SO_NOSIGPIPE")
        .blocklist_item("PJ_SO_PRIORITY")
        .blocklist_item("PJ_IP_MULTICAST_IF")
        .blocklist_item("PJ_IP_MULTICAST_TTL")
        .blocklist_item("PJ_IP_MULTICAST_LOOP")
        .blocklist_item("PJ_IP_ADD_MEMBERSHIP")
        .blocklist_item("PJ_IP_DROP_MEMBERSHIP")
        .blocklist_item("PJ_MSG_OOB")
        .blocklist_item("PJ_MSG_PEEK")
        .blocklist_item("PJ_MSG_DONTROUTE")
        .blocklist_item("PJ_GUID_STRING_LENGTH")
        .blocklist_item("pj_pool_factory_default_policy")

        .blocklist_item("pj_ssl_entropy")
        .blocklist_item("pj_ssl_entropy_t")

        .blocklist_type("pj_dns_type")
        .blocklist_type("pj_dns_callback")
        .blocklist_type("pj_dns_srv_resolver_cb")
        .blocklist_type("pj_cis_elem_t")
        .blocklist_type("pj_syn_err_func_ptr")
        .blocklist_type("pj_json_val_type")
        .blocklist_type("pj_json_writer")
        .blocklist_type("pjstun_attr_type")
        .blocklist_type("pj_pcap_link_type")
        .blocklist_type("pj_pcap_proto_type")
        .blocklist_type("pj_cli_cmd_id")
        .blocklist_type("pj_cli_arg_id")
        .blocklist_type("pj_cli_get_dyn_choice")
        .blocklist_type("pj_cli_cmd_handler")
        .blocklist_type("pj_cli_front_end_type")
        .blocklist_type("pj_cli_telnet_on_started")

        // pjlib_util_sys
        .blocklist_type("pj_getopt_option")
        .blocklist_type("pj_crc32_context")
        .blocklist_type("pj_md5_context")
        .blocklist_type("pj_hmac_md5_context")
        .blocklist_type("pj_sha1_context")
        .blocklist_type("pj_hmac_sha1_context")
        .blocklist_type("in6_addr__bindgen_ty_1")
        .blocklist_type("pj_dns_hdr")
        .blocklist_type("pj_dns_parsed_query")
        .blocklist_type("pj_dns_parsed_rr")
        .blocklist_type("pj_dns_parsed_rr_rdata")
        .blocklist_type("pj_dns_parsed_rr_rdata_srv")
        .blocklist_type("pj_dns_parsed_rr_rdata_cname")
        .blocklist_type("pj_dns_parsed_rr_rdata_ns")
        .blocklist_type("pj_dns_parsed_rr_rdata_ptr")
        .blocklist_type("pj_dns_parsed_rr_rdata_a")
        .blocklist_type("pj_dns_parsed_rr_rdata_aaaa")
        .blocklist_type("pj_dns_parsed_packet")
        .blocklist_type("pj_dns_resolver")
        .blocklist_type("pj_dns_async_query")
        .blocklist_type("pj_dns_settings")
        .blocklist_type("pj_dns_a_record")
        .blocklist_type("pj_dns_addr_record")
        .blocklist_type("pj_dns_addr_record__bindgen_ty_1")
        .blocklist_type("pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pj_dns_srv_record")
        .blocklist_type("pj_dns_srv_record__bindgen_ty_1")
        .blocklist_type("pj_dns_srv_async_query")
        .blocklist_type("pj_dns_server")
        .blocklist_type("pj_cis_buf_t")
        .blocklist_type("pj_cis_t")
        .blocklist_type("pj_scanner")
        .blocklist_type("pj_scan_state")
        .blocklist_type("pj_xml_attr")
        .blocklist_type("pj_xml_node_head")
        .blocklist_type("pj_xml_node")
        .blocklist_type("pj_json_list")
        .blocklist_type("pj_json_elem")
        .blocklist_type("pj_json_elem__bindgen_ty_1")
        .blocklist_type("pj_json_err_info")
        .blocklist_type("pjstun_msg_hdr")
        .blocklist_type("pjstun_attr_hdr")
        .blocklist_type("pjstun_msg")
        .blocklist_type("pjstun_setting")
        .blocklist_type("pj_pcap_udp_hdr")
        .blocklist_type("pj_pcap_filter")
        .blocklist_type("pj_pcap_file")
        .blocklist_type("pj_http_req")
        .blocklist_type("pj_http_header_elmt")
        .blocklist_type("pj_http_headers")
        .blocklist_type("pj_http_auth_cred")
        .blocklist_type("pj_http_req_param")
        .blocklist_type("pj_http_req_param_pj_http_reqdata")
        .blocklist_type("pj_http_auth_chal")
        .blocklist_type("pj_http_resp")
        .blocklist_type("pj_http_url")
        .blocklist_type("pj_http_req_callback")
        .blocklist_type("pj_cli_t")
        .blocklist_type("pj_cli_cfg")
        .blocklist_type("pj_cli_cmd_spec")
        .blocklist_type("pj_cli_cmd_val")
        .blocklist_type("pj_cli_hint_info")
        .blocklist_type("pj_cli_exec_info")
        .blocklist_type("pj_cli_arg_choice_val")
        .blocklist_type("pj_cli_dyn_choice_param")
        .blocklist_type("pj_cli_front_end_op")
        .blocklist_type("pj_cli_front_end")
        .blocklist_type("pj_cli_sess_op")
        .blocklist_type("pj_cli_sess")
        .blocklist_type("pj_cli_console_cfg")
        .blocklist_type("pj_cli_telnet_info")
        .blocklist_type("pj_cli_telnet_cfg")


        .allowlist_function("discard_check")
        .allowlist_function("get_check_foundation_idx")
        .allowlist_function("pj_ice_calc_foundation")
        .allowlist_function("pj_ice_get_cand_type_name")
        .allowlist_function("pj_ice_sess_add_cand")
        .allowlist_function("pj_ice_sess_change_role")
        .allowlist_function("pj_ice_sess_create")
        .allowlist_function("pj_ice_sess_create_check_list")
        .allowlist_function("pj_ice_sess_destroy")
        .allowlist_function("pj_ice_sess_find_default_cand")
        .allowlist_function("pj_ice_sess_get_options")
        .allowlist_function("pj_ice_sess_on_rx_pkt")
        .allowlist_function("pj_ice_sess_options_default")
        .allowlist_function("pj_ice_sess_role_name")
        .allowlist_function("pj_ice_sess_send_data")
        .allowlist_function("pj_ice_sess_set_options")
        .allowlist_function("pj_ice_sess_set_prefs")
        .allowlist_function("pj_ice_sess_start_check")
        .allowlist_function("pj_ice_sess_update_check_list")
        .allowlist_function("pj_ice_strans_cfg_copy")
        .allowlist_function("pj_ice_strans_cfg_default")
        .allowlist_function("pj_ice_strans_change_role")
        .allowlist_function("pj_ice_strans_create")
        .allowlist_function("pj_ice_strans_destroy")
        .allowlist_function("pj_ice_strans_enum_cands")
        .allowlist_function("pj_ice_strans_get_cands_count")
        .allowlist_function("pj_ice_strans_get_def_cand")
        .allowlist_function("pj_ice_strans_get_grp_lock")
        .allowlist_function("pj_ice_strans_get_options")
        .allowlist_function("pj_ice_strans_get_role")
        .allowlist_function("pj_ice_strans_get_running_comp_cnt")
        .allowlist_function("pj_ice_strans_get_state")
        .allowlist_function("pj_ice_strans_get_ufrag_pwd")
        .allowlist_function("pj_ice_strans_get_user_data")
        .allowlist_function("pj_ice_strans_get_valid_pair")
        .allowlist_function("pj_ice_strans_has_sess")
        .allowlist_function("pj_ice_strans_init_ice")
        .allowlist_function("pj_ice_strans_sendto")
        .allowlist_function("pj_ice_strans_sendto2")
        .allowlist_function("pj_ice_strans_sess_is_complete")
        .allowlist_function("pj_ice_strans_sess_is_running")
        .allowlist_function("pj_ice_strans_set_options")
        .allowlist_function("pj_ice_strans_start_ice")
        .allowlist_function("pj_ice_strans_state_name")
        .allowlist_function("pj_ice_strans_stop_ice")
        .allowlist_function("pj_ice_strans_stun_cfg_default")
        .allowlist_function("pj_ice_strans_turn_cfg_default")
        .allowlist_function("pj_ice_strans_update_check_list")
        .allowlist_function("pj_ice_strans_update_comp_cnt")
        .allowlist_function("pjnath_init")
        .allowlist_function("pjnath_perror")
        .allowlist_function("pj_stun_attr_clone")
        .allowlist_function("pj_stun_auth_cred_dup")
        .allowlist_function("pj_stun_authenticate_request")
        .allowlist_function("pj_stun_authenticate_response")
        .allowlist_function("pj_stun_auth_valid_for_msg")
        .allowlist_function("pj_stun_binary_attr_create")
        .allowlist_function("pj_stun_binary_attr_init")
        .allowlist_function("pj_stun_client_tsx_create")
        .allowlist_function("pj_stun_client_tsx_destroy")
        .allowlist_function("pj_stun_client_tsx_get_data")
        .allowlist_function("pj_stun_client_tsx_is_complete")
        .allowlist_function("pj_stun_client_tsx_on_rx_msg")
        .allowlist_function("pj_stun_client_tsx_retransmit")
        .allowlist_function("pj_stun_client_tsx_schedule_destroy")
        .allowlist_function("pj_stun_client_tsx_send_msg")
        .allowlist_function("pj_stun_client_tsx_set_data")
        .allowlist_function("pj_stun_client_tsx_stop")
        .allowlist_function("pj_stun_create_key")
        .allowlist_function("pj_stun_detect_nat_type")
        .allowlist_function("pj_stun_detect_nat_type2")
        .allowlist_function("pj_stun_empty_attr_create")
        .allowlist_function("pj_stun_errcode_attr_create")
        .allowlist_function("pj_stun_get_attr_name")
        .allowlist_function("pj_stun_get_class_name")
        .allowlist_function("pj_stun_get_err_reason")
        .allowlist_function("pj_stun_get_method_name")
        .allowlist_function("pj_stun_get_nat_name")
        .allowlist_function("pj_stun_msg_add_attr")
        .allowlist_function("pj_stun_msg_add_binary_attr")
        .allowlist_function("pj_stun_msg_add_empty_attr")
        .allowlist_function("pj_stun_msg_add_errcode_attr")
        .allowlist_function("pj_stun_msg_add_msgint_attr")
        .allowlist_function("pj_stun_msg_add_sockaddr_attr")
        .allowlist_function("pj_stun_msg_add_string_attr")
        .allowlist_function("pj_stun_msg_add_uint64_attr")
        .allowlist_function("pj_stun_msg_add_uint_attr")
        .allowlist_function("pj_stun_msg_add_unknown_attr")
        .allowlist_function("pj_stun_msg_check")
        .allowlist_function("pj_stun_msg_clone")
        .allowlist_function("pj_stun_msg_create")
        .allowlist_function("pj_stun_msg_create_response")
        .allowlist_function("pj_stun_msg_decode")
        .allowlist_function("pj_stun_msg_destroy_tdata")
        .allowlist_function("pj_stun_msg_dump")
        .allowlist_function("pj_stun_msg_encode")
        .allowlist_function("pj_stun_msg_find_attr")
        .allowlist_function("pj_stun_msg_init")
        .allowlist_function("pj_stun_msgint_attr_create")
        .allowlist_function("pj_stun_req_cred_info_dup")
        .allowlist_function("pj_stun_session_cancel_req")
        .allowlist_function("pj_stun_session_create")
        .allowlist_function("pj_stun_session_create_ind")
        .allowlist_function("pj_stun_session_create_req")
        .allowlist_function("pj_stun_session_create_res")
        .allowlist_function("pj_stun_session_destroy")
        .allowlist_function("pj_stun_session_get_grp_lock")
        .allowlist_function("pj_stun_session_get_user_data")
        .allowlist_function("pj_stun_session_on_rx_pkt")
        .allowlist_function("pj_stun_session_respond")
        .allowlist_function("pj_stun_session_retransmit_req")
        .allowlist_function("pj_stun_session_send_msg")
        .allowlist_function("pj_stun_session_set_credential")
        .allowlist_function("pj_stun_session_set_log")
        .allowlist_function("pj_stun_session_set_software_name")
        .allowlist_function("pj_stun_session_set_user_data")
        .allowlist_function("pj_stun_session_use_fingerprint")
        .allowlist_function("pj_stun_set_padding_char")
        .allowlist_function("pj_stun_sockaddr_attr_create")
        .allowlist_function("pj_stun_sockaddr_attr_init")
        .allowlist_function("pj_stun_sock_cfg_default")
        .allowlist_function("pj_stun_sock_create")
        .allowlist_function("pj_stun_sock_destroy")
        .allowlist_function("pj_stun_sock_get_grp_lock")
        .allowlist_function("pj_stun_sock_get_info")
        .allowlist_function("pj_stun_sock_get_user_data")
        .allowlist_function("pj_stun_sock_op_name")
        .allowlist_function("pj_stun_sock_sendto")
        .allowlist_function("pj_stun_sock_set_user_data")
        .allowlist_function("pj_stun_sock_start")
        .allowlist_function("pj_stun_string_attr_create")
        .allowlist_function("pj_stun_string_attr_init")
        .allowlist_function("pj_stun_uint64_attr_create")
        .allowlist_function("pj_stun_uint_attr_create")
        .allowlist_function("pj_stun_unknown_attr_create")
        .allowlist_function("pj_turn_alloc_param_copy")
        .allowlist_function("pj_turn_alloc_param_default")
        .allowlist_function("pj_turn_session_alloc")
        .allowlist_function("pj_turn_session_bind_channel")
        .allowlist_function("pj_turn_session_connection_bind")
        .allowlist_function("pj_turn_session_create")
        .allowlist_function("pj_turn_session_destroy")
        .allowlist_function("pj_turn_session_get_grp_lock")
        .allowlist_function("pj_turn_session_get_info")
        .allowlist_function("pj_turn_session_get_user_data")
        .allowlist_function("pj_turn_session_on_rx_pkt")
        .allowlist_function("pj_turn_session_on_rx_pkt2")
        .allowlist_function("pj_turn_session_sendto")
        .allowlist_function("pj_turn_session_set_credential")
        .allowlist_function("pj_turn_session_set_log")
        .allowlist_function("pj_turn_session_set_perm")
        .allowlist_function("pj_turn_session_set_server")
        .allowlist_function("pj_turn_session_set_software_name")
        .allowlist_function("pj_turn_session_set_user_data")
        .allowlist_function("pj_turn_session_shutdown")
        .allowlist_function("pj_turn_sock_alloc")
        .allowlist_function("pj_turn_sock_bind_channel")
        .allowlist_function("pj_turn_sock_cfg_default")
        .allowlist_function("pj_turn_sock_create")
        .allowlist_function("pj_turn_sock_destroy")
        .allowlist_function("pj_turn_sock_get_grp_lock")
        .allowlist_function("pj_turn_sock_get_info")
        .allowlist_function("pj_turn_sock_get_user_data")
        .allowlist_function("pj_turn_sock_lock")
        .allowlist_function("pj_turn_sock_sendto")
        .allowlist_function("pj_turn_sock_set_log")
        .allowlist_function("pj_turn_sock_set_perm")
        .allowlist_function("pj_turn_sock_set_software_name")
        .allowlist_function("pj_turn_sock_set_user_data")
        .allowlist_function("pj_turn_sock_tls_cfg_default")
        .allowlist_function("pj_turn_sock_tls_cfg_dup")
        .allowlist_function("pj_turn_sock_tls_cfg_wipe_keys")
        .allowlist_function("pj_turn_sock_unlock")
        .allowlist_function("pj_turn_state_name")
        .allowlist_function("remove_check")

        .allowlist_type("PJ.*")
        .allowlist_type("pj.*")
        .allowlist_recursively(true)
        .translate_enum_integer_types(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = env::current_dir().unwrap();
    pjnath.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

}
