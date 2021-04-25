extern crate bindgen;
use std::env;


fn main() {
    println!("cargo:rustc-link-lib=pjmedia");
    println!("cargo:rustc-link-search=native=/usr/lib");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let pjmedia = bindgen::Builder::default().header("wrapper.h")
        // .dynamic_library_name("pjmedia")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("extern crate pj_sys;")
        .raw_line("extern crate pjnath_sys;")
        .raw_line("use pj_sys::*;")
        .raw_line("use pjnath_sys::*;")

        // pj-sys
        // struct
        .blocklist_type("pj_timestamp").blocklist_type("pj_sockaddr").blocklist_type("pj_str_t").blocklist_type("pj_timestamp__bindgen_ty_1")
        .blocklist_type("pj_hash_table_t").blocklist_type("pj_hash_entry").blocklist_type("pj_hash_iterator_t").blocklist_type("pj_ioqueue_t")
        .blocklist_type("pj_ioqueue_key_t").blocklist_type("pj_timer_heap_t").blocklist_type("pj_atomic_t").blocklist_type("pj_thread_t")
        .blocklist_type("pj_lock_t").blocklist_type("pj_grp_lock_t").blocklist_type("pj_mutex_t").blocklist_type("pj_sem_t")
        .blocklist_type("pj_event_t").blocklist_type("pj_pipe_t").blocklist_type("pj_time_val").blocklist_type("pj_parsed_time")
        .blocklist_type("pj_ioqueue_op_key_t").blocklist_type("pj_ioqueue_callback").blocklist_type("__sigset_t").blocklist_type("in_addr.*")
        .blocklist_type("in6_addr.*").blocklist_type("pj_sockaddr_in").blocklist_type("pj_sockaddr_in6").blocklist_type("pj_addr_hdr")
        .blocklist_type("pj_ip_mreq").blocklist_type("pj_sockopt_params").blocklist_type("pj_sockopt_params__bindgen_ty_1").blocklist_type("pj_activesock_t")
        .blocklist_type("pj_activesock_cb").blocklist_type("pj_activesock_cfg").blocklist_type("pj_hostent").blocklist_type("pj_addrinfo")
        .blocklist_type("__jmp_buf_tag").blocklist_type("pj_exception_state_t").blocklist_type("pj_fifobuf_t").blocklist_type("pj_file_stat")
        .blocklist_type("pj_ip_route_entry__bindgen_ty_1").blocklist_type("pj_enum_ip_option").blocklist_type("pj_list").blocklist_type("pj_grp_lock_config")
        .blocklist_type("pj_math_stat").blocklist_type("pj_sys_info").blocklist_type("pj_symbianos_params").blocklist_type("pj_rwmutex_t")
        .blocklist_type("pj_pool_block").blocklist_type("pj_pool_t").blocklist_type("pj_pool_factory_policy").blocklist_type("pj_pool_factory")
        .blocklist_type("pj_caching_pool").blocklist_type("pj_rbtree_node").blocklist_type("pj_rbtree").blocklist_type("pj_qos_params")
        .blocklist_type("pj_fd_set_t").blocklist_type("pj_ssl_sock_t").blocklist_type("pj_ssl_cert_t").blocklist_type("pj_ssl_cert_info")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_1").blocklist_type("pj_ssl_cert_info__bindgen_ty_2").blocklist_type("pj_ssl_cert_info__bindgen_ty_3").blocklist_type("pj_ssl_cert_info__bindgen_ty_4")
        .blocklist_type("pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1").blocklist_type("pj_ssl_cert_info__bindgen_ty_5").blocklist_type("pj_ssl_sock_cb").blocklist_type("pj_ssl_sock_info")
        .blocklist_type("pj_ssl_sock_param").blocklist_type("pj_ssl_start_connect_param").blocklist_type("pj_timer_entry").blocklist_type("__va_list_tag")

        // pj-sys
        // type
        .blocklist_type("pj_int64_t").blocklist_type("pj_uint64_t").blocklist_type("size_t").blocklist_type("pj_int32_t").blocklist_type("pj_uint32_t")
        .blocklist_type("pj_int16_t").blocklist_type("pj_uint16_t").blocklist_type("pj_int8_t").blocklist_type("pj_uint8_t").blocklist_type("pj_size_t")
        .blocklist_type("pj_ssize_t").blocklist_type("pj_status_t").blocklist_type("pj_bool_t").blocklist_type("pj_char_t").blocklist_type("pj_constants_")
        .blocklist_type("pj_off_t").blocklist_type("pj_list_type").blocklist_type("pj_atomic_value_t").blocklist_type("pj_oshandle_t").blocklist_type("pj_sock_t")
        .blocklist_type("pj_sockaddr_t").blocklist_type("pj_color_t").blocklist_type("pj_exception_id_t").blocklist_type("pj_exit_callback").blocklist_type("pj_ioqueue_operation_e")
        .blocklist_type("__uint8_t").blocklist_type("__uint16_t").blocklist_type("__uint32_t").blocklist_type("in_addr_t").blocklist_type("pj_socket_sd_type")
        .blocklist_type("pj_in_addr").blocklist_type("pj_in6_addr").blocklist_type("pj_os_err_type").blocklist_type("va_list").blocklist_type("pj_error_callback")
        .blocklist_type("__jmp_buf").blocklist_type("jmp_buf").blocklist_type("pj_jmp_buf").blocklist_type("pj_log_decoration").blocklist_type("pj_log_func")
        .blocklist_type("pj_file_access").blocklist_type("pj_file_seek_type").blocklist_type("pj_hash_entry_buf").blocklist_type("pj_highprec_t").blocklist_type("pj_sys_info_flag")
        .blocklist_type("pj_thread_create_flags").blocklist_type("pj_thread_proc").blocklist_type("pj_thread_desc").blocklist_type("pj_mutex_type_e").blocklist_type("pj_main_func_ptr")
        .blocklist_type("pj_pool_callback").blocklist_type("pj_rbcolor_t").blocklist_type("pj_rbtree_comp").blocklist_type("pj_qos_type").blocklist_type("pj_qos_flag")
        .blocklist_type("pj_qos_wmm_prio").blocklist_type("pj_ssl_cert_verify_flag_t").blocklist_type("pj_ssl_cert_name_type").blocklist_type("pj_ssl_cert_buffer").blocklist_type("pj_ssl_cipher")
        .blocklist_type("pj_ssl_curve").blocklist_type("pj_ssl_entropy").blocklist_type("pj_ssl_sock_proto").blocklist_type("pj_timer_id_t").blocklist_type("pj_timer_heap_callback")
        .blocklist_type("__builtin_va_list")

        // pj-sys
        // pub static
        .blocklist_item("PJ_VERSION").blocklist_item("PJ_AF_UNSPEC").blocklist_item("PJ_AF_UNIX").blocklist_item("PJ_AF_INET").blocklist_item("PJ_AF_INET6")
        .blocklist_item("PJ_AF_PACKET").blocklist_item("PJ_AF_IRDA").blocklist_item("PJ_SOCK_STREAM").blocklist_item("PJ_SOCK_DGRAM").blocklist_item("PJ_SOCK_RAW")
        .blocklist_item("PJ_SOCK_RDM").blocklist_item("PJ_SOL_SOCKET").blocklist_item("PJ_SOL_IP").blocklist_item("PJ_SOL_TCP").blocklist_item("PJ_SOL_UDP")
        .blocklist_item("PJ_SOL_IPV6").blocklist_item("PJ_IP_TOS").blocklist_item("PJ_IPTOS_LOWDELAY").blocklist_item("PJ_IPTOS_THROUGHPUT").blocklist_item("PJ_IPTOS_RELIABILITY")
        .blocklist_item("PJ_IPTOS_MINCOST").blocklist_item("PJ_IPV6_TCLASS").blocklist_item("PJ_SO_TYPE").blocklist_item("PJ_SO_RCVBUF").blocklist_item("PJ_SO_SNDBUF")
        .blocklist_item("PJ_TCP_NODELAY").blocklist_item("PJ_SO_REUSEADDR").blocklist_item("PJ_SO_NOSIGPIPE").blocklist_item("PJ_SO_PRIORITY").blocklist_item("PJ_IP_MULTICAST_IF")
        .blocklist_item("PJ_IP_MULTICAST_TTL").blocklist_item("PJ_IP_MULTICAST_LOOP").blocklist_item("PJ_IP_ADD_MEMBERSHIP").blocklist_item("PJ_IP_DROP_MEMBERSHIP").blocklist_item("PJ_MSG_OOB")
        .blocklist_item("PJ_MSG_PEEK").blocklist_item("PJ_MSG_DONTROUTE").blocklist_item("PJ_GUID_STRING_LENGTH").blocklist_item("pj_pool_factory_default_policy")

        .blocklist_item("pj_ssl_entropy").blocklist_item("pj_ssl_entropy_t")

        // pjlib-util-sys
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

        // pjnath-sys
        // pub type
        .blocklist_type("pj_stun_method_e").blocklist_type("pj_stun_msg_class_e").blocklist_type("pj_stun_msg_type").blocklist_type("pj_stun_attr_type")
        .blocklist_type("pj_stun_status").blocklist_type("pj_stun_fingerprint_attr").blocklist_type("pj_stun_realm_attr").blocklist_type("pj_stun_nonce_attr")
        .blocklist_type("pj_stun_mapped_addr_attr").blocklist_type("pj_stun_xor_mapped_addr_attr").blocklist_type("pj_stun_software_attr").blocklist_type("pj_stun_alt_server_attr")
        .blocklist_type("pj_stun_refresh_interval_attr").blocklist_type("pj_stun_response_addr_attr").blocklist_type("pj_stun_changed_addr_attr").blocklist_type("pj_stun_change_request_attr")
        .blocklist_type("pj_stun_src_addr_attr").blocklist_type("pj_stun_reflected_from_attr").blocklist_type("pj_stun_username_attr").blocklist_type("pj_stun_password_attr")
        .blocklist_type("pj_stun_channel_number_attr").blocklist_type("pj_stun_lifetime_attr").blocklist_type("pj_stun_bandwidth_attr").blocklist_type("pj_stun_xor_peer_addr_attr")
        .blocklist_type("pj_stun_data_attr").blocklist_type("pj_stun_xor_relayed_addr_attr").blocklist_type("pj_stun_req_addr_type_attr").blocklist_type("pj_stun_even_port_attr")
        .blocklist_type("pj_stun_req_transport_attr").blocklist_type("pj_stun_dont_fragment_attr").blocklist_type("pj_stun_res_token_attr").blocklist_type("pj_stun_xor_reflected_from_attr")
        .blocklist_type("pj_stun_priority_attr").blocklist_type("pj_stun_use_candidate_attr").blocklist_type("pj_stun_timer_val_attr").blocklist_type("pj_stun_ice_controlling_attr")
        .blocklist_type("pj_stun_ice_controlled_attr").blocklist_type("pj_stun_icmp_attr").blocklist_type("pj_stun_decode_options").blocklist_type("pj_stun_auth_type")
        .blocklist_type("pj_stun_auth_cred_type").blocklist_type("pj_stun_passwd_type").blocklist_type("pj_stun_sess_msg_log_flag").blocklist_type("pj_ice_cand_type")
        .blocklist_type("pj_ice_sess_check_state").blocklist_type("pj_ice_sess_checklist_state").blocklist_type("pj_ice_sess_role").blocklist_type("pj_ice_sess_trickle")
        .blocklist_type("pj_dns_rcode").blocklist_type("pj_dns_dup_options").blocklist_type("pj_stun_sock_op").blocklist_type("pj_turn_tp_type")
        .blocklist_type("pj_turn_state_t").blocklist_type("pj_ice_strans_op").blocklist_type("pj_ice_strans_state").blocklist_type("pj_stun_nat_type")
        .blocklist_type("pj_stun_nat_detect_cb")

        // pjnath-sys
        // pub struct
        .blocklist_type("pj_stun_msg_hdr").blocklist_type("pj_stun_attr_hdr").blocklist_type("pj_stun_sockaddr_attr").blocklist_type("pj_stun_empty_attr")
        .blocklist_type("pj_stun_string_attr").blocklist_type("pj_stun_uint_attr").blocklist_type("pj_stun_uint64_attr").blocklist_type("pj_stun_binary_attr")
        .blocklist_type("pj_stun_msgint_attr").blocklist_type("pj_stun_errcode_attr").blocklist_type("pj_stun_unknown_attr").blocklist_type("pj_stun_msg")
        .blocklist_type("pj_stun_auth_cred").blocklist_type("pj_stun_req_cred_info").blocklist_type("pj_stun_config").blocklist_type("pj_stun_client_tsx")
        .blocklist_type("pj_stun_tsx_cb").blocklist_type("pj_stun_session").blocklist_type("pj_stun_session_cb").blocklist_type("pj_stun_rx_data")
        .blocklist_type("pj_stun_tx_data").blocklist_type("pj_ice_sess_comp").blocklist_type("pj_ice_msg_data").blocklist_type("pj_ice_msg_data_data_request_data")
        .blocklist_type("pj_ice_sess_cand").blocklist_type("pj_ice_sess_check").blocklist_type("pj_ice_sess_checklist").blocklist_type("pj_ice_sess_cb")
        .blocklist_type("pj_ice_rx_check").blocklist_type("pj_ice_sess_options").blocklist_type("pj_ice_sess").blocklist_type("pj_dns_hdr")
        .blocklist_type("pj_dns_parsed_query").blocklist_type("pj_dns_parsed_rr").blocklist_type("pj_dns_parsed_rr_rdata").blocklist_type("pj_dns_parsed_rr_rdata_srv")
        .blocklist_type("pj_dns_parsed_rr_rdata_cname").blocklist_type("pj_dns_parsed_rr_rdata_ns").blocklist_type("pj_dns_parsed_rr_rdata_ptr").blocklist_type("pj_dns_parsed_rr_rdata_a")
        .blocklist_type("pj_dns_parsed_rr_rdata_aaaa").blocklist_type("pj_dns_parsed_packet").blocklist_type("pj_dns_resolver").blocklist_type("pj_dns_async_query")
        .blocklist_type("pj_dns_settings").blocklist_type("pj_dns_a_record").blocklist_type("pj_dns_addr_record").blocklist_type("pj_stun_sock")
        .blocklist_type("pj_stun_sock_cb").blocklist_type("pj_stun_sock_info").blocklist_type("pj_stun_sock_cfg").blocklist_type("pj_turn_session")
        .blocklist_type("pj_turn_channel_data").blocklist_type("pj_turn_session_cb").blocklist_type("pj_turn_alloc_param").blocklist_type("pj_turn_session_info")
        .blocklist_type("pj_turn_session_on_rx_pkt_param").blocklist_type("pj_turn_sock").blocklist_type("pj_turn_sock_cb").blocklist_type("pj_turn_sock_tls_cfg")
        .blocklist_type("pj_turn_sock_cfg").blocklist_type("pj_ice_strans").blocklist_type("pj_ice_strans_cb").blocklist_type("pj_ice_strans_stun_cfg")
        .blocklist_type("pj_ice_strans_turn_cfg").blocklist_type("pj_ice_strans_cfg").blocklist_type("pj_stun_nat_detect_result")

        .blocklist_type("pj_stun_auth_cred__bindgen_ty_1")
        .blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2")
        .blocklist_type("pj_dns_addr_record__bindgen_ty_1")
        .blocklist_type("pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pj_ice_strans_cfg__bindgen_ty_1")

        .blocklist_type("pj_ice_msg_data_data")
        .blocklist_type("pj_ice_sess__bindgen_ty_1")




        .allowlist_function("echo_supp_cancel_echo")
        .allowlist_function("echo_supp_create")
        .allowlist_function("echo_supp_destroy")
        .allowlist_function("echo_supp_get_stat")
        .allowlist_function("echo_supp_reset")
        .allowlist_function("echo_supp_soft_reset")
        .allowlist_function("get_libsrtp_errstr")
        .allowlist_function("pjmedia_aud_dev_cap_name")
        .allowlist_function("pjmedia_aud_dev_count")
        .allowlist_function("pjmedia_aud_dev_default_param")
        .allowlist_function("pjmedia_aud_dev_get_info")
        .allowlist_function("pjmedia_aud_dev_lookup")
        .allowlist_function("pjmedia_aud_dev_refresh")
        .allowlist_function("pjmedia_aud_driver_deinit")
        .allowlist_function("pjmedia_aud_driver_init")
        .allowlist_function("pjmedia_aud_param_get_cap")
        .allowlist_function("pjmedia_aud_param_set_cap")
        .allowlist_function("pjmedia_aud_stream_create")
        .allowlist_function("pjmedia_aud_stream_destroy")
        .allowlist_function("pjmedia_aud_stream_get_cap")
        .allowlist_function("pjmedia_aud_stream_get_param")
        .allowlist_function("pjmedia_aud_stream_set_cap")
        .allowlist_function("pjmedia_aud_stream_start")
        .allowlist_function("pjmedia_aud_stream_stop")
        .allowlist_function("pjmedia_bidirectional_port_create")
        .allowlist_function("pjmedia_calc_avg_signal")
        .allowlist_function("pjmedia_clock_create")
        .allowlist_function("pjmedia_clock_create2")
        .allowlist_function("pjmedia_clock_destroy")
        .allowlist_function("pjmedia_clock_modify")
        .allowlist_function("pjmedia_clock_src_get_current_timestamp")
        .allowlist_function("pjmedia_clock_src_get_time_msec")
        .allowlist_function("pjmedia_clock_src_init")
        .allowlist_function("pjmedia_clock_src_update")
        .allowlist_function("pjmedia_clock_start")
        .allowlist_function("pjmedia_clock_stop")
        .allowlist_function("pjmedia_clock_wait")
        .allowlist_function("pjmedia_codec_info_to_id")
        .allowlist_function("pjmedia_codec_mgr_alloc_codec")
        .allowlist_function("pjmedia_codec_mgr_dealloc_codec")
        .allowlist_function("pjmedia_codec_mgr_destroy")
        .allowlist_function("pjmedia_codec_mgr_enum_codecs")
        .allowlist_function("pjmedia_codec_mgr_find_codecs_by_id")
        .allowlist_function("pjmedia_codec_mgr_get_codec_info")
        .allowlist_function("pjmedia_codec_mgr_get_default_param")
        .allowlist_function("pjmedia_codec_mgr_init")
        .allowlist_function("pjmedia_codec_mgr_register_factory")
        .allowlist_function("pjmedia_codec_mgr_set_codec_priority")
        .allowlist_function("pjmedia_codec_mgr_set_default_param")
        .allowlist_function("pjmedia_codec_mgr_unregister_factory")
        .allowlist_function("pjmedia_codec_param_clone")
        .allowlist_function("pjmedia_conf_add_port")
        .allowlist_function("pjmedia_conf_adjust_conn_level")
        .allowlist_function("pjmedia_conf_adjust_rx_level")
        .allowlist_function("pjmedia_conf_adjust_tx_level")
        .allowlist_function("pjmedia_conf_configure_port")
        .allowlist_function("pjmedia_conf_connect_port")
        .allowlist_function("pjmedia_conf_create")
        .allowlist_function("pjmedia_conf_destroy")
        .allowlist_function("pjmedia_conf_disconnect_port")
        .allowlist_function("pjmedia_conf_disconnect_port_from_sinks")
        .allowlist_function("pjmedia_conf_disconnect_port_from_sources")
        .allowlist_function("pjmedia_conf_enum_ports")
        .allowlist_function("pjmedia_conf_get_connect_count")
        .allowlist_function("pjmedia_conf_get_master_port")
        .allowlist_function("pjmedia_conf_get_msignal_level")
        .allowlist_function("pjmedia_conf_get_port_count")
        .allowlist_function("pjmedia_conf_get_port_info")
        .allowlist_function("pjmedia_conf_get_ports_info")
        .allowlist_function("pjmedia_conf_get_signal_level")
        .allowlist_function("pjmedia_conf_remove_port")
        .allowlist_function("pjmedia_conf_set_port0_name")
        .allowlist_function("pjmedia_converter_convert")
        .allowlist_function("pjmedia_converter_convert2")
        .allowlist_function("pjmedia_converter_create")
        .allowlist_function("pjmedia_converter_destroy")
        .allowlist_function("pjmedia_converter_mgr_create")
        .allowlist_function("pjmedia_converter_mgr_destroy")
        .allowlist_function("pjmedia_converter_mgr_instance")
        .allowlist_function("pjmedia_converter_mgr_register_factory")
        .allowlist_function("pjmedia_converter_mgr_set_instance")
        .allowlist_function("pjmedia_converter_mgr_unregister_factory")
        .allowlist_function("pjmedia_delay_buf_create")
        .allowlist_function("pjmedia_delay_buf_destroy")
        .allowlist_function("pjmedia_delay_buf_get")
        .allowlist_function("pjmedia_delay_buf_put")
        .allowlist_function("pjmedia_delay_buf_reset")
        .allowlist_function("pjmedia_echo_cancel")
        .allowlist_function("pjmedia_echo_capture")
        .allowlist_function("pjmedia_echo_create")
        .allowlist_function("pjmedia_echo_create2")
        .allowlist_function("pjmedia_echo_destroy")
        .allowlist_function("pjmedia_echo_get_stat")
        .allowlist_function("pjmedia_echo_playback")
        .allowlist_function("pjmedia_echo_port_create")
        .allowlist_function("pjmedia_echo_reset")
        .allowlist_function("pjmedia_echo_stat_default")
        .allowlist_function("pjmedia_endpt_atexit")
        .allowlist_function("pjmedia_endpt_create2")
        .allowlist_function("pjmedia_endpt_create_audio_sdp")
        .allowlist_function("pjmedia_endpt_create_base_sdp")
        .allowlist_function("pjmedia_endpt_create_pool")
        .allowlist_function("pjmedia_endpt_create_sdp")
        .allowlist_function("pjmedia_endpt_destroy2")
        .allowlist_function("pjmedia_endpt_dump")
        .allowlist_function("pjmedia_endpt_get_codec_mgr")
        .allowlist_function("pjmedia_endpt_get_flag")
        .allowlist_function("pjmedia_endpt_get_ioqueue")
        .allowlist_function("pjmedia_endpt_get_thread")
        .allowlist_function("pjmedia_endpt_get_thread_count")
        .allowlist_function("pjmedia_endpt_set_flag")
        .allowlist_function("pjmedia_endpt_stop_threads")
        .allowlist_function("pjmedia_event_init")
        .allowlist_function("pjmedia_event_mgr_create")
        .allowlist_function("pjmedia_event_mgr_destroy")
        .allowlist_function("pjmedia_event_mgr_instance")
        .allowlist_function("pjmedia_event_mgr_set_instance")
        .allowlist_function("pjmedia_event_publish")
        .allowlist_function("pjmedia_event_subscribe")
        .allowlist_function("pjmedia_event_unsubscribe")
        .allowlist_function("pjmedia_format_copy")
        .allowlist_function("pjmedia_format_get_audio_format_detail")
        .allowlist_function("pjmedia_get_aud_subsys")
        .allowlist_function("pjmedia_get_type")
        .allowlist_function("pjmedia_ice_add_ice_cb")
        .allowlist_function("pjmedia_ice_create")
        .allowlist_function("pjmedia_ice_create2")
        .allowlist_function("pjmedia_ice_create3")
        .allowlist_function("pjmedia_ice_get_grp_lock")
        .allowlist_function("pjmedia_ice_remove_ice_cb")
        .allowlist_function("pjmedia_ice_sdp_has_trickle")
        .allowlist_function("pjmedia_ice_trickle_decode_sdp")
        .allowlist_function("pjmedia_ice_trickle_encode_sdp")
        .allowlist_function("pjmedia_ice_trickle_has_new_cand")
        .allowlist_function("pjmedia_ice_trickle_send_local_cand")
        .allowlist_function("pjmedia_ice_trickle_update")
        .allowlist_function("pjmedia_jbuf_create")
        .allowlist_function("pjmedia_jbuf_destroy")
        .allowlist_function("pjmedia_jbuf_get_frame")
        .allowlist_function("pjmedia_jbuf_get_frame2")
        .allowlist_function("pjmedia_jbuf_get_frame3")
        .allowlist_function("pjmedia_jbuf_get_state")
        .allowlist_function("pjmedia_jbuf_is_full")
        .allowlist_function("pjmedia_jbuf_peek_frame")
        .allowlist_function("pjmedia_jbuf_put_frame")
        .allowlist_function("pjmedia_jbuf_put_frame2")
        .allowlist_function("pjmedia_jbuf_put_frame3")
        .allowlist_function("pjmedia_jbuf_remove_frame")
        .allowlist_function("pjmedia_jbuf_reset")
        .allowlist_function("pjmedia_jbuf_set_adaptive")
        .allowlist_function("pjmedia_jbuf_set_discard")
        .allowlist_function("pjmedia_jbuf_set_fixed")
        .allowlist_function("pjmedia_jbuf_set_ptime")
        .allowlist_function("pjmedia_loop_tp_setting_default")
        .allowlist_function("pjmedia_master_port_create")
        .allowlist_function("pjmedia_master_port_destroy")
        .allowlist_function("pjmedia_master_port_get_dport")
        .allowlist_function("pjmedia_master_port_get_uport")
        .allowlist_function("pjmedia_master_port_set_dport")
        .allowlist_function("pjmedia_master_port_set_uport")
        .allowlist_function("pjmedia_master_port_start")
        .allowlist_function("pjmedia_master_port_stop")
        .allowlist_function("pjmedia_master_port_wait")
        .allowlist_function("pjmedia_mem_capture_create")
        .allowlist_function("pjmedia_mem_capture_get_size")
        .allowlist_function("pjmedia_mem_capture_set_eof_cb")
        .allowlist_function("pjmedia_mem_capture_set_eof_cb2")
        .allowlist_function("pjmedia_mem_player_create")
        .allowlist_function("pjmedia_mem_player_set_eof_cb")
        .allowlist_function("pjmedia_mem_player_set_eof_cb2")
        .allowlist_function("pjmedia_null_port_create")
        .allowlist_function("pjmedia_plc_create")
        .allowlist_function("pjmedia_plc_generate")
        .allowlist_function("pjmedia_plc_save")
        .allowlist_function("pjmedia_port_destroy")
        .allowlist_function("pjmedia_port_get_clock_src")
        .allowlist_function("pjmedia_port_get_frame")
        .allowlist_function("pjmedia_port_info_init")
        .allowlist_function("pjmedia_port_info_init2")
        .allowlist_function("pjmedia_port_put_frame")
        .allowlist_function("pjmedia_resample_create")
        .allowlist_function("pjmedia_resample_destroy")
        .allowlist_function("pjmedia_resample_get_input_size")
        .allowlist_function("pjmedia_resample_port_create")
        .allowlist_function("pjmedia_resample_run")
        .allowlist_function("pjmedia_rtcp_build_rtcp")
        .allowlist_function("pjmedia_rtcp_build_rtcp_bye")
        .allowlist_function("pjmedia_rtcp_build_rtcp_sdes")
        .allowlist_function("pjmedia_rtcp_enable_xr")
        .allowlist_function("pjmedia_rtcp_fb_build_nack")
        .allowlist_function("pjmedia_rtcp_fb_build_pli")
        .allowlist_function("pjmedia_rtcp_fb_build_rpsi")
        .allowlist_function("pjmedia_rtcp_fb_build_sli")
        .allowlist_function("pjmedia_rtcp_fb_decode_sdp")
        .allowlist_function("pjmedia_rtcp_fb_decode_sdp2")
        .allowlist_function("pjmedia_rtcp_fb_encode_sdp")
        .allowlist_function("pjmedia_rtcp_fb_info_dup")
        .allowlist_function("pjmedia_rtcp_fb_parse_nack")
        .allowlist_function("pjmedia_rtcp_fb_parse_pli")
        .allowlist_function("pjmedia_rtcp_fb_parse_rpsi")
        .allowlist_function("pjmedia_rtcp_fb_parse_sli")
        .allowlist_function("pjmedia_rtcp_fb_setting_default")
        .allowlist_function("pjmedia_rtcp_fb_setting_dup")
        .allowlist_function("pjmedia_rtcp_fini")
        .allowlist_function("pjmedia_rtcp_get_ntp_time")
        .allowlist_function("pjmedia_rtcp_init")
        .allowlist_function("pjmedia_rtcp_init2")
        .allowlist_function("pjmedia_rtcp_init_stat")
        .allowlist_function("pjmedia_rtcp_rx_rtcp")
        .allowlist_function("pjmedia_rtcp_rx_rtp")
        .allowlist_function("pjmedia_rtcp_rx_rtp2")
        .allowlist_function("pjmedia_rtcp_session_setting_default")
        .allowlist_function("pjmedia_rtcp_tx_rtp")
        .allowlist_function("pjmedia_rtp_decode_rtp")
        .allowlist_function("pjmedia_rtp_decode_rtp2")
        .allowlist_function("pjmedia_rtp_encode_rtp")
        .allowlist_function("pjmedia_rtp_seq_init")
        .allowlist_function("pjmedia_rtp_seq_update")
        .allowlist_function("pjmedia_rtp_session_init")
        .allowlist_function("pjmedia_rtp_session_init2")
        .allowlist_function("pjmedia_rtp_session_update")
        .allowlist_function("pjmedia_rtp_session_update2")
        .allowlist_function("pjmedia_sdp_attr_add")
        .allowlist_function("pjmedia_sdp_attr_clone")
        .allowlist_function("pjmedia_sdp_attr_create")
        .allowlist_function("pjmedia_sdp_attr_create_rtcp")
        .allowlist_function("pjmedia_sdp_attr_create_ssrc")
        .allowlist_function("pjmedia_sdp_attr_find")
        .allowlist_function("pjmedia_sdp_attr_find2")
        .allowlist_function("pjmedia_sdp_attr_get_fmtp")
        .allowlist_function("pjmedia_sdp_attr_get_rtcp")
        .allowlist_function("pjmedia_sdp_attr_get_rtpmap")
        .allowlist_function("pjmedia_sdp_attr_get_ssrc")
        .allowlist_function("pjmedia_sdp_attr_remove")
        .allowlist_function("pjmedia_sdp_attr_remove_all")
        .allowlist_function("pjmedia_sdp_attr_to_rtpmap")
        .allowlist_function("pjmedia_sdp_bandw_clone")
        .allowlist_function("pjmedia_sdp_conn_clone")
        .allowlist_function("pjmedia_sdp_conn_cmp")
        .allowlist_function("pjmedia_sdp_media_add_attr")
        .allowlist_function("pjmedia_sdp_media_clone")
        .allowlist_function("pjmedia_sdp_media_clone_deactivate")
        .allowlist_function("pjmedia_sdp_media_cmp")
        .allowlist_function("pjmedia_sdp_media_deactivate")
        .allowlist_function("pjmedia_sdp_media_find_attr")
        .allowlist_function("pjmedia_sdp_media_find_attr2")
        .allowlist_function("pjmedia_sdp_media_remove_all_attr")
        .allowlist_function("pjmedia_sdp_media_remove_attr")
        .allowlist_function("pjmedia_sdp_neg_cancel_offer")
        .allowlist_function("pjmedia_sdp_neg_create_w_local_offer")
        .allowlist_function("pjmedia_sdp_neg_create_w_remote_offer")
        .allowlist_function("pjmedia_sdp_neg_fmt_match")
        .allowlist_function("pjmedia_sdp_neg_get_active_local")
        .allowlist_function("pjmedia_sdp_neg_get_active_remote")
        .allowlist_function("pjmedia_sdp_neg_get_neg_local")
        .allowlist_function("pjmedia_sdp_neg_get_neg_remote")
        .allowlist_function("pjmedia_sdp_neg_get_state")
        .allowlist_function("pjmedia_sdp_neg_has_local_answer")
        .allowlist_function("pjmedia_sdp_neg_modify_local_offer")
        .allowlist_function("pjmedia_sdp_neg_modify_local_offer2")
        .allowlist_function("pjmedia_sdp_neg_negotiate")
        .allowlist_function("pjmedia_sdp_neg_register_fmt_match_cb")
        .allowlist_function("pjmedia_sdp_neg_send_local_offer")
        .allowlist_function("pjmedia_sdp_neg_set_answer_multiple_codecs")
        .allowlist_function("pjmedia_sdp_neg_set_local_answer")
        .allowlist_function("pjmedia_sdp_neg_set_prefer_remote_codec_order")
        .allowlist_function("pjmedia_sdp_neg_set_remote_answer")
        .allowlist_function("pjmedia_sdp_neg_set_remote_offer")
        .allowlist_function("pjmedia_sdp_neg_state_str")
        .allowlist_function("pjmedia_sdp_neg_was_answer_remote")
        .allowlist_function("pjmedia_sdp_parse")
        .allowlist_function("pjmedia_sdp_print")
        .allowlist_function("pjmedia_sdp_rtpmap_to_attr")
        .allowlist_function("pjmedia_sdp_session_add_attr")
        .allowlist_function("pjmedia_sdp_session_clone")
        .allowlist_function("pjmedia_sdp_session_cmp")
        .allowlist_function("pjmedia_sdp_transport_cmp")
        .allowlist_function("pjmedia_sdp_transport_get_proto")
        .allowlist_function("pjmedia_sdp_validate")
        .allowlist_function("pjmedia_sdp_validate2")
        .allowlist_function("pjmedia_session_check_dtmf")
        .allowlist_function("pjmedia_session_create")
        .allowlist_function("pjmedia_session_destroy")
        .allowlist_function("pjmedia_session_dial_dtmf")
        .allowlist_function("pjmedia_session_enum_streams")
        .allowlist_function("pjmedia_session_get_dtmf")
        .allowlist_function("pjmedia_session_get_info")
        .allowlist_function("pjmedia_session_get_port")
        .allowlist_function("pjmedia_session_get_stream_stat")
        .allowlist_function("pjmedia_session_get_stream_stat_jbuf")
        .allowlist_function("pjmedia_session_get_user_data")
        .allowlist_function("pjmedia_session_info_from_sdp")
        .allowlist_function("pjmedia_session_pause")
        .allowlist_function("pjmedia_session_pause_stream")
        .allowlist_function("pjmedia_session_reset_stream_stat")
        .allowlist_function("pjmedia_session_resume")
        .allowlist_function("pjmedia_session_resume_stream")
        .allowlist_function("pjmedia_session_send_rtcp_bye")
        .allowlist_function("pjmedia_session_send_rtcp_sdes")
        .allowlist_function("pjmedia_session_set_dtmf_callback")
        .allowlist_function("pjmedia_silence_det_apply")
        .allowlist_function("pjmedia_silence_det_create")
        .allowlist_function("pjmedia_silence_det_detect")
        .allowlist_function("pjmedia_silence_det_disable")
        .allowlist_function("pjmedia_silence_det_set_adaptive")
        .allowlist_function("pjmedia_silence_det_set_fixed")
        .allowlist_function("pjmedia_silence_det_set_name")
        .allowlist_function("pjmedia_silence_det_set_params")
        .allowlist_function("pjmedia_snd_get_dev_info")
        .allowlist_function("pjmedia_snd_open")
        .allowlist_function("pjmedia_snd_open_player")
        .allowlist_function("pjmedia_snd_open_rec")
        .allowlist_function("pjmedia_snd_port_connect")
        .allowlist_function("pjmedia_snd_port_create")
        .allowlist_function("pjmedia_snd_port_create2")
        .allowlist_function("pjmedia_snd_port_create_player")
        .allowlist_function("pjmedia_snd_port_create_rec")
        .allowlist_function("pjmedia_snd_port_destroy")
        .allowlist_function("pjmedia_snd_port_disconnect")
        .allowlist_function("pjmedia_snd_port_get_clock_src")
        .allowlist_function("pjmedia_snd_port_get_ec_stat")
        .allowlist_function("pjmedia_snd_port_get_ec_tail")
        .allowlist_function("pjmedia_snd_port_get_port")
        .allowlist_function("pjmedia_snd_port_get_snd_stream")
        .allowlist_function("pjmedia_snd_port_param_default")
        .allowlist_function("pjmedia_snd_port_set_ec")
        .allowlist_function("pjmedia_snd_set_latency")
        .allowlist_function("pjmedia_snd_stream_close")
        .allowlist_function("pjmedia_snd_stream_get_info")
        .allowlist_function("pjmedia_snd_stream_start")
        .allowlist_function("pjmedia_snd_stream_stop")
        .allowlist_function("pjmedia_splitcomb_create")
        .allowlist_function("pjmedia_splitcomb_create_rev_channel")
        .allowlist_function("pjmedia_splitcomb_set_channel")
        .allowlist_function("pjmedia_srtp_enum_crypto")
        .allowlist_function("pjmedia_srtp_enum_keying")
        .allowlist_function("pjmedia_srtp_init_lib")
        .allowlist_function("pjmedia_srtp_setting_default")
        .allowlist_function("pjmedia_stereo_port_create")
        .allowlist_function("pjmedia_stream_check_dtmf")
        .allowlist_function("pjmedia_stream_create")
        .allowlist_function("pjmedia_stream_destroy")
        .allowlist_function("pjmedia_stream_dial_dtmf")
        .allowlist_function("pjmedia_stream_get_dtmf")
        .allowlist_function("pjmedia_stream_get_info")
        .allowlist_function("pjmedia_stream_get_last_jb_frame_type")
        .allowlist_function("pjmedia_stream_get_port")
        .allowlist_function("pjmedia_stream_get_rtp_session_info")
        .allowlist_function("pjmedia_stream_get_stat")
        .allowlist_function("pjmedia_stream_get_stat_jbuf")
        .allowlist_function("pjmedia_stream_get_transport")
        .allowlist_function("pjmedia_stream_info_from_sdp")
        .allowlist_function("pjmedia_stream_info_parse_fmtp")
        .allowlist_function("pjmedia_stream_pause")
        .allowlist_function("pjmedia_stream_reset_stat")
        .allowlist_function("pjmedia_stream_resume")
        .allowlist_function("pjmedia_stream_send_rtcp_bye")
        .allowlist_function("pjmedia_stream_send_rtcp_sdes")
        .allowlist_function("pjmedia_stream_set_dtmf_callback")
        .allowlist_function("pjmedia_stream_set_dtmf_event_callback")
        .allowlist_function("pjmedia_stream_start")
        .allowlist_function("pjmedia_strerror")
        .allowlist_function("pjmedia_tonegen_create")
        .allowlist_function("pjmedia_tonegen_create2")
        .allowlist_function("pjmedia_tonegen_get_digit_map")
        .allowlist_function("pjmedia_tonegen_is_busy")
        .allowlist_function("pjmedia_tonegen_play")
        .allowlist_function("pjmedia_tonegen_play_digits")
        .allowlist_function("pjmedia_tonegen_rewind")
        .allowlist_function("pjmedia_tonegen_set_digit_map")
        .allowlist_function("pjmedia_tonegen_stop")
        .allowlist_function("pjmedia_tonegen_stop_loop")
        .allowlist_function("pjmedia_tp_adapter_create")
        .allowlist_function("pjmedia_transport_loop_create")
        .allowlist_function("pjmedia_transport_loop_create2")
        .allowlist_function("pjmedia_transport_loop_disable_rx")
        .allowlist_function("pjmedia_transport_srtp_create")
        .allowlist_function("pjmedia_transport_srtp_decrypt_pkt")
        .allowlist_function("pjmedia_transport_srtp_dtls_get_fingerprint")
        .allowlist_function("pjmedia_transport_srtp_dtls_start_nego")
        .allowlist_function("pjmedia_transport_srtp_get_member")
        .allowlist_function("pjmedia_transport_srtp_start")
        .allowlist_function("pjmedia_transport_srtp_stop")
        .allowlist_function("pjmedia_transport_udp_attach")
        .allowlist_function("pjmedia_transport_udp_create")
        .allowlist_function("pjmedia_transport_udp_create2")
        .allowlist_function("pjmedia_transport_udp_create3")
        .allowlist_function("pjmedia_type_name")
        .allowlist_function("pjmedia_wave_hdr_file_to_host")
        .allowlist_function("pjmedia_wave_hdr_host_to_file")
        .allowlist_function("pjmedia_wav_player_get_info")
        .allowlist_function("pjmedia_wav_player_get_len")
        .allowlist_function("pjmedia_wav_player_port_create")
        .allowlist_function("pjmedia_wav_player_port_get_pos")
        .allowlist_function("pjmedia_wav_player_port_set_pos")
        .allowlist_function("pjmedia_wav_player_set_eof_cb")
        .allowlist_function("pjmedia_wav_player_set_eof_cb2")
        .allowlist_function("pjmedia_wav_playlist_create")
        .allowlist_function("pjmedia_wav_playlist_set_eof_cb")
        .allowlist_function("pjmedia_wav_playlist_set_eof_cb2")
        .allowlist_function("pjmedia_wav_writer_port_create")
        .allowlist_function("pjmedia_wav_writer_port_get_pos")
        .allowlist_function("pjmedia_wav_writer_port_set_cb")
        .allowlist_function("pjmedia_wav_writer_port_set_cb2")
        .allowlist_function("pjmedia_wsola_create")
        .allowlist_function("pjmedia_wsola_destroy")
        .allowlist_function("pjmedia_wsola_discard")
        .allowlist_function("pjmedia_wsola_generate")
        .allowlist_function("pjmedia_wsola_reset")
        .allowlist_function("pjmedia_wsola_save")
        .allowlist_function("pjmedia_wsola_set_max_expand")

        .allowlist_var("pjmedia_add_bandwidth_tias_in_sdp")
        .allowlist_var("pjmedia_add_rtpmap_for_static_pt")
        .allowlist_var("rtcp_fb_type_name")


        .allowlist_var("pjmedia_alaw2linear_tab")
        .allowlist_var("pjmedia_linear2alaw_tab")
        .allowlist_var("pjmedia_linear2ulaw_tab")
        .allowlist_var("pjmedia_ulaw2linear_tab")


        // .whitelisted_var("PJ.*")
        // .whitelisted_var("pj.*")
        .allowlist_type("PJMEDIA.*")
        .allowlist_type("pjmedia.*")

        .allowlist_var("PJMEDIA.*")

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
    pjmedia.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");
}
