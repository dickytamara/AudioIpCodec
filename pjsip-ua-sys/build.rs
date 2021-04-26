
extern crate bindgen;
use std::env;


fn main () {

    println!("cargo:rustc-link-lib=pjsip");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let pjsip_ua = bindgen::Builder::default().header("wrapper.h")

        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("extern crate pj_sys;")
        .raw_line("extern crate pjsip_sys;")
        .raw_line("extern crate pjmedia_sys;")
        .raw_line("use pj_sys::*;")
        .raw_line("use pjsip_sys::*;")
        .raw_line("use pjmedia_sys::*;")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))

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

        .blocklist_type("pj_stun_msg_hdr").blocklist_type("pj_stun_attr_hdr").blocklist_type("pj_stun_sockaddr_attr").blocklist_type("pj_stun_empty_attr")
        .blocklist_type("pj_stun_string_attr").blocklist_type("pj_stun_uint_attr").blocklist_type("pj_stun_uint64_attr").blocklist_type("pj_stun_binary_attr")
        .blocklist_type("pj_stun_msgint_attr").blocklist_type("pj_stun_errcode_attr").blocklist_type("pj_stun_unknown_attr").blocklist_type("pj_stun_msg")
        .blocklist_type("pj_stun_auth_cred").blocklist_type("pj_stun_auth_cred__bindgen_ty_1").blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1").blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2")
        .blocklist_type("pj_stun_req_cred_info").blocklist_type("pj_stun_config").blocklist_type("pj_stun_client_tsx").blocklist_type("pj_stun_tsx_cb")
        .blocklist_type("pj_stun_session").blocklist_type("pj_stun_session_cb").blocklist_type("pj_stun_rx_data").blocklist_type("pj_stun_tx_data")
        .blocklist_type("pj_ice_sess_comp").blocklist_type("pj_ice_msg_data").blocklist_type("pj_ice_msg_data_data_request_data").blocklist_type("pj_ice_sess_cand")
        .blocklist_type("pj_ice_sess_check").blocklist_type("pj_ice_sess_checklist").blocklist_type("pj_ice_sess_cb").blocklist_type("pj_ice_rx_check")
        .blocklist_type("pj_ice_sess_options").blocklist_type("pj_ice_sess").blocklist_type("pj_dns_hdr").blocklist_type("pj_dns_parsed_query")
        .blocklist_type("pj_dns_parsed_rr").blocklist_type("pj_dns_parsed_rr_rdata").blocklist_type("pj_dns_parsed_rr_rdata_srv").blocklist_type("pj_dns_parsed_rr_rdata_cname")
        .blocklist_type("pj_dns_parsed_rr_rdata_ns").blocklist_type("pj_dns_parsed_rr_rdata_ptr").blocklist_type("pj_dns_parsed_rr_rdata_a").blocklist_type("pj_dns_parsed_rr_rdata_aaaa")
        .blocklist_type("pj_dns_parsed_packet").blocklist_type("pj_dns_resolver").blocklist_type("pj_dns_async_query").blocklist_type("pj_dns_settings")
        .blocklist_type("pj_dns_a_record").blocklist_type("pj_dns_addr_record").blocklist_type("pj_dns_addr_record__bindgen_ty_1").blocklist_type("pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pj_stun_sock").blocklist_type("pj_stun_sock_cb").blocklist_type("pj_stun_sock_info").blocklist_type("pj_stun_sock_cfg")
        .blocklist_type("pj_turn_session").blocklist_type("pj_turn_channel_data").blocklist_type("pj_turn_session_cb").blocklist_type("pj_turn_alloc_param")
        .blocklist_type("pj_turn_session_info").blocklist_type("pj_turn_session_on_rx_pkt_param").blocklist_type("pj_turn_sock").blocklist_type("pj_turn_sock_cb")
        .blocklist_type("pj_turn_sock_tls_cfg").blocklist_type("pj_turn_sock_cfg").blocklist_type("pj_ice_strans").blocklist_type("pj_ice_strans_cb")
        .blocklist_type("pj_ice_strans_stun_cfg").blocklist_type("pj_ice_strans_turn_cfg").blocklist_type("pj_ice_strans_cfg").blocklist_type("pj_ice_strans_cfg__bindgen_ty_1")
        .blocklist_type("pj_stun_nat_detect_result")

        .blocklist_type("pj_stun_auth_cred__bindgen_ty_1")
        .blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2")
        .blocklist_type("pj_dns_addr_record__bindgen_ty_1")
        .blocklist_type("pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pj_ice_strans_cfg__bindgen_ty_1")

        .blocklist_type("pj_ice_msg_data_data")
        .blocklist_type("pj_ice_sess__bindgen_ty_1")

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

        // pjmedia-sys
        .blocklist_type("pjmedia_endpt").blocklist_type("pjmedia_stream").blocklist_type("pjmedia_ratio").blocklist_type("pjmedia_coord")
        .blocklist_type("pjmedia_rect_size").blocklist_type("pjmedia_rect").blocklist_type("pjmedia_clock_src").blocklist_type("pjmedia_clock")
        .blocklist_type("pjmedia_clock_param").blocklist_type("pjmedia_audio_format_detail").blocklist_type("pjmedia_video_format_detail").blocklist_type("pjmedia_format")
        .blocklist_type("pjmedia_format__bindgen_ty_1").blocklist_type("pjmedia_video_apply_fmt_param").blocklist_type("pjmedia_video_format_info").blocklist_type("pjmedia_video_format_mgr")
        .blocklist_type("pjmedia_frame").blocklist_type("pjmedia_frame_ext").blocklist_type("pjmedia_frame_ext_subframe").blocklist_type("pjmedia_aud_stream")
        .blocklist_type("pjmedia_aud_dev_factory").blocklist_type("pjmedia_aud_driver").blocklist_type("pjmedia_aud_subsys").blocklist_type("pjmedia_aud_dev_info")
        .blocklist_type("pjmedia_aud_param").blocklist_type("pjmedia_rtcp_xr_rb_header").blocklist_type("pjmedia_rtcp_xr_rb_rr_time").blocklist_type("pjmedia_rtcp_xr_rb_dlrr_item")
        .blocklist_type("pjmedia_rtcp_xr_rb_dlrr").blocklist_type("pjmedia_rtcp_xr_rb_stats").blocklist_type("pjmedia_rtcp_xr_rb_voip_mtc").blocklist_type("pjmedia_rtcp_xr_pkt")
        .blocklist_type("pjmedia_rtcp_xr_pkt__bindgen_ty_1").blocklist_type("pjmedia_rtcp_xr_stream_stat").blocklist_type("pjmedia_rtcp_xr_stream_stat__bindgen_ty_1").blocklist_type("pjmedia_rtcp_xr_stream_stat__bindgen_ty_2")
        .blocklist_type("pjmedia_rtcp_xr_stat").blocklist_type("pjmedia_rtcp_xr_session").blocklist_type("pjmedia_rtcp_xr_session__bindgen_ty_1").blocklist_type("pjmedia_rtp_hdr")
        .blocklist_type("pjmedia_rtp_ext_hdr").blocklist_type("pjmedia_rtp_dec_hdr").blocklist_type("pjmedia_rtp_dtmf_event").blocklist_type("pjmedia_rtp_seq_session")
        .blocklist_type("pjmedia_rtp_session").blocklist_type("pjmedia_rtp_status").blocklist_type("pjmedia_rtp_status__bindgen_ty_1").blocklist_type("pjmedia_rtp_status__bindgen_ty_1_flag")
        .blocklist_type("pjmedia_rtp_session_setting").blocklist_type("pjmedia_rtcp_sr").blocklist_type("pjmedia_rtcp_rr").blocklist_type("pjmedia_rtcp_common")
        .blocklist_type("pjmedia_rtcp_sr_pkt").blocklist_type("pjmedia_rtcp_rr_pkt").blocklist_type("pjmedia_rtcp_sdes").blocklist_type("pjmedia_rtcp_ntp_rec")
        .blocklist_type("pjmedia_rtcp_stream_stat").blocklist_type("pjmedia_rtcp_stream_stat__bindgen_ty_1").blocklist_type("pjmedia_rtcp_stat").blocklist_type("pjmedia_rtcp_session")
        .blocklist_type("pjmedia_rtcp_session_setting").blocklist_type("pjmedia_sdp_attr").blocklist_type("pjmedia_sdp_rtpmap").blocklist_type("pjmedia_sdp_fmtp")
        .blocklist_type("pjmedia_sdp_rtcp_attr").blocklist_type("pjmedia_sdp_ssrc_attr").blocklist_type("pjmedia_sdp_conn").blocklist_type("pjmedia_sdp_bandw")
        .blocklist_type("pjmedia_sdp_media").blocklist_type("pjmedia_sdp_media__bindgen_ty_1").blocklist_type("pjmedia_sdp_session").blocklist_type("pjmedia_sdp_session__bindgen_ty_1")
        .blocklist_type("pjmedia_sdp_session__bindgen_ty_2").blocklist_type("pjmedia_rtcp_fb_cap").blocklist_type("pjmedia_rtcp_fb_info").blocklist_type("pjmedia_rtcp_fb_setting")
        .blocklist_type("pjmedia_rtcp_fb_nack").blocklist_type("pjmedia_rtcp_fb_sli").blocklist_type("pjmedia_rtcp_fb_rpsi").blocklist_type("pjmedia_event_rx_rtcp_fb_data")
        .blocklist_type("pjmedia_event_rx_rtcp_fb_data__bindgen_ty_1").blocklist_type("pjmedia_vid_dev_hwnd").blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_1").blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_2")
        .blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_3").blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_4").blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_5").blocklist_type("pjmedia_vid_dev_switch_param")
        .blocklist_type("pjmedia_vid_dev_info").blocklist_type("pjmedia_vid_dev_cb").blocklist_type("pjmedia_vid_dev_param").blocklist_type("pjmedia_vid_driver")
        .blocklist_type("pjmedia_vid_subsys").blocklist_type("pjmedia_event_fmt_changed_data").blocklist_type("pjmedia_event_dummy_data").blocklist_type("pjmedia_event_wnd_resized_data")
        .blocklist_type("pjmedia_event_wnd_closing_data").blocklist_type("pjmedia_event_aud_dev_err_data").blocklist_type("pjmedia_event_vid_dev_err_data").blocklist_type("pjmedia_event_media_tp_err_data")
        .blocklist_type("pjmedia_event").blocklist_type("pjmedia_event__bindgen_ty_1").blocklist_type("pjmedia_event_mgr").blocklist_type("pjmedia_port_info")
        .blocklist_type("pjmedia_port").blocklist_type("pjmedia_port_port_data").blocklist_type("pjmedia_avi_streams").blocklist_type("pjmedia_circ_buf")
        .blocklist_type("pjmedia_codec_info").blocklist_type("pjmedia_codec_fmtp").blocklist_type("pjmedia_codec_fmtp_param").blocklist_type("pjmedia_codec_param")
        .blocklist_type("pjmedia_codec_param__bindgen_ty_1").blocklist_type("pjmedia_codec_param__bindgen_ty_2").blocklist_type("pjmedia_codec_op").blocklist_type("pjmedia_codec")
        .blocklist_type("pjmedia_codec_factory_op").blocklist_type("pjmedia_codec_factory").blocklist_type("pjmedia_codec_default_param").blocklist_type("pjmedia_codec_desc")
        .blocklist_type("pjmedia_codec_mgr").blocklist_type("pjmedia_conf").blocklist_type("pjmedia_conf_port_info").blocklist_type("pjmedia_conversion_param")
        .blocklist_type("pjmedia_converter_factory").blocklist_type("pjmedia_converter").blocklist_type("pjmedia_converter_factory_op").blocklist_type("pjmedia_converter_op")
        .blocklist_type("pjmedia_converter_mgr").blocklist_type("pjmedia_delay_buf").blocklist_type("pjmedia_echo_state").blocklist_type("pjmedia_echo_stat")
        .blocklist_type("pjmedia_sock_info").blocklist_type("pjmedia_transport_op").blocklist_type("pjmedia_transport").blocklist_type("pjmedia_transport_specific_info")
        .blocklist_type("pjmedia_transport_info").blocklist_type("pjmedia_tp_cb_param").blocklist_type("pjmedia_transport_attach_param").blocklist_type("pjmedia_jb_state")
        .blocklist_type("pjmedia_jbuf").blocklist_type("pjmedia_master_port").blocklist_type("pjmedia_plc").blocklist_type("pjmedia_resample")
        .blocklist_type("pjmedia_sdp_neg").blocklist_type("pjmedia_silence_det").blocklist_type("pjmedia_snd_stream").blocklist_type("pjmedia_snd_dev_info")
        .blocklist_type("pjmedia_snd_stream_info").blocklist_type("pjmedia_snd_port_param").blocklist_type("pjmedia_snd_port").blocklist_type("pjmedia_vid_encode_opt")
        .blocklist_type("pjmedia_vid_codec_info").blocklist_type("pjmedia_vid_codec_param").blocklist_type("pjmedia_vid_codec_op").blocklist_type("pjmedia_vid_codec")
        .blocklist_type("pjmedia_vid_codec_factory_op").blocklist_type("pjmedia_vid_codec_factory").blocklist_type("pjmedia_vid_codec_mgr").blocklist_type("pjmedia_stream_rtp_sess_info")
        .blocklist_type("pjmedia_channel").blocklist_type("pjmedia_stream_info").blocklist_type("pjmedia_stream_dtmf_event").blocklist_type("pjmedia_tone_desc")
        .blocklist_type("pjmedia_tone_digit").blocklist_type("pjmedia_tone_digit_map").blocklist_type("pjmedia_tone_digit_map__bindgen_ty_1").blocklist_type("pjmedia_ice_cb")
        .blocklist_type("pjmedia_ice_transport_info").blocklist_type("pjmedia_ice_transport_info__bindgen_ty_1").blocklist_type("pjmedia_loop_tp_setting").blocklist_type("pjmedia_srtp_crypto")
        .blocklist_type("pjmedia_srtp_cb").blocklist_type("pjmedia_srtp_setting").blocklist_type("pjmedia_srtp_info").blocklist_type("pjmedia_srtp_dtls_nego_param")
        .blocklist_type("pjmedia_vid_conf").blocklist_type("pjmedia_vid_conf_setting").blocklist_type("pjmedia_vid_conf_port_info").blocklist_type("pjmedia_vid_port_param")
        .blocklist_type("pjmedia_vid_port").blocklist_type("pjmedia_vid_stream_rc_config").blocklist_type("pjmedia_vid_stream_sk_config").blocklist_type("pjmedia_vid_stream_info")
        .blocklist_type("pjmedia_vid_stream").blocklist_type("pjmedia_wav_player_info").blocklist_type("pjmedia_wave_hdr").blocklist_type("pjmedia_wave_hdr__bindgen_ty_1")
        .blocklist_type("pjmedia_wave_hdr__bindgen_ty_2").blocklist_type("pjmedia_wave_hdr__bindgen_ty_3").blocklist_type("pjmedia_wave_subchunk").blocklist_type("pjmedia_wsola")
        .blocklist_type("pjmedia_vid_dev_factory_op").blocklist_type("pjmedia_vid_dev_factory").blocklist_type("pjmedia_vid_dev_factory__bindgen_ty_1").blocklist_type("pjmedia_vid_dev_stream_op")
        .blocklist_type("pjmedia_vid_dev_stream").blocklist_type("pjmedia_vid_dev_stream__bindgen_ty_1").blocklist_type("pjmedia_avi_dev_param")

        // pjsip-sys
        .blocklist_type("pjsip_transport_type_e")
        .blocklist_type("pjsip_user_agent")
        .blocklist_type("pjsip_role_e")
        .blocklist_type("pjsip_uri_context_e")
        .blocklist_type("pjsip_method_e")
        .blocklist_type("pjsip_hdr_e")
        .blocklist_type("pjsip_status_code")
        .blocklist_type("pjsip_msg_type_e")
        .blocklist_type("pjsip_accept_hdr")
        .blocklist_type("pjsip_allow_hdr")
        .blocklist_type("pjsip_expires_hdr")
        .blocklist_type("pjsip_from_hdr")
        .blocklist_type("pjsip_to_hdr")
        .blocklist_type("pjsip_max_fwd_hdr")
        .blocklist_type("pjsip_min_expires_hdr")
        .blocklist_type("pjsip_rr_hdr")
        .blocklist_type("pjsip_route_hdr")
        .blocklist_type("pjsip_require_hdr")
        .blocklist_type("pjsip_supported_hdr")
        .blocklist_type("pjsip_unsupported_hdr")
        .blocklist_type("pjsip_warning_hdr")
        .blocklist_type("pjsip_parse_hdr_func")
        .blocklist_type("pjsip_parse_uri_func")
        .blocklist_type("pjsip_event_id_e")
        .blocklist_type("pjsip_resolver_callback")
        .blocklist_type("pjsip_tpselector_type")
        .blocklist_type("pjsip_transport_callback")
        .blocklist_type("pjsip_transport_dir")
        .blocklist_type("pjsip_rx_callback")
        .blocklist_type("pjsip_tx_callback")
        .blocklist_type("pjsip_tp_send_callback")
        .blocklist_type("pjsip_transport_state")
        .blocklist_type("pjsip_tp_state_listener_key")
        .blocklist_type("pjsip_tp_state_callback")
        .blocklist_type("pjsip_tp_on_rx_dropped_cb")
        .blocklist_type("pjsip_endpt_exit_callback")
        .blocklist_type("pjsip_send_callback")
        .blocklist_type("pjsip_endpt_send_callback")
        .blocklist_type("pjsip_ssl_method")
        .blocklist_type("pjsip_proxy_authorization_hdr")
        .blocklist_type("pjsip_proxy_authenticate_hdr")
        .blocklist_type("pjsip_auth_qop_type")
        .blocklist_type("pjsip_cred_cb")
        .blocklist_type("pjsip_auth_lookup_cred")
        .blocklist_type("pjsip_auth_lookup_cred2")
        .blocklist_type("pjsip_tsx_state_e")
        .blocklist_type("pjsip_dialog_state")
        .blocklist_type("pjsip_dialog_cap_status")

        .blocklist_type("pjsip_cfg_t")
        .blocklist_type("pjsip_cfg_t__bindgen_ty_1")
        .blocklist_type("pjsip_cfg_t__bindgen_ty_2")
        .blocklist_type("pjsip_cfg_t__bindgen_ty_3")
        .blocklist_type("pjsip_cfg_t__bindgen_ty_4")
        .blocklist_type("pjsip_cfg_t__bindgen_ty_5")
        .blocklist_type("pjsip_tpmgr")
        .blocklist_type("pjsip_endpoint")
        .blocklist_type("pjsip_resolver_t")
        .blocklist_type("pjsip_buffer")
        .blocklist_type("pjsip_host_port")
        .blocklist_type("pjsip_host_info")
        .blocklist_type("pj_cis_t")
        .blocklist_type("pj_scanner")
        .blocklist_type("pjsip_param")
        .blocklist_type("pjsip_uri_vptr")
        .blocklist_type("pjsip_uri")
        .blocklist_type("pjsip_sip_uri")
        .blocklist_type("pjsip_name_addr")
        .blocklist_type("pjsip_other_uri")
        .blocklist_type("pjsip_tel_uri")
        .blocklist_type("pjsip_method")
        .blocklist_type("pjsip_hdr_vptr")
        .blocklist_type("pjsip_hdr")
        .blocklist_type("pjsip_request_line")
        .blocklist_type("pjsip_status_line")
        .blocklist_type("pjsip_media_type")
        .blocklist_type("pjsip_msg_body")
        .blocklist_type("pjsip_msg")
        .blocklist_type("pjsip_msg__bindgen_ty_1")
        .blocklist_type("pjsip_generic_string_hdr")
        .blocklist_type("pjsip_generic_int_hdr")
        .blocklist_type("pjsip_generic_array_hdr")
        .blocklist_type("pjsip_cid_hdr")
        .blocklist_type("pjsip_clen_hdr")
        .blocklist_type("pjsip_cseq_hdr")
        .blocklist_type("pjsip_contact_hdr")
        .blocklist_type("pjsip_ctype_hdr")
        .blocklist_type("pjsip_fromto_hdr")
        .blocklist_type("pjsip_routing_hdr")
        .blocklist_type("pjsip_retry_after_hdr")
        .blocklist_type("pjsip_via_hdr")
        .blocklist_type("pjsip_multipart_part")
        .blocklist_type("pjsip_parser_err_report")
        .blocklist_type("pjsip_parse_ctx")
        .blocklist_type("pjsip_parser_const_t")
        .blocklist_type("pjsip_event")
        .blocklist_type("pjsip_event__bindgen_ty_1")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_2")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_3")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_4")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_5")
        .blocklist_type("pjsip_event__bindgen_ty_1__bindgen_ty_6")
        .blocklist_type("pjsip_module")
        .blocklist_type("pjsip_server_addresses")
        .blocklist_type("pjsip_server_addresses__bindgen_ty_1")
        .blocklist_type("pjsip_ext_resolver")
        .blocklist_type("pjsip_tpselector")
        .blocklist_type("pjsip_rx_data_op_key")
        .blocklist_type("pjsip_rx_data")
        .blocklist_type("pjsip_rx_data__bindgen_ty_1")
        .blocklist_type("pjsip_rx_data__bindgen_ty_2")
        .blocklist_type("pjsip_rx_data__bindgen_ty_3")
        .blocklist_type("pjsip_rx_data__bindgen_ty_4")
        .blocklist_type("pjsip_tx_data_op_key")
        .blocklist_type("pjsip_tx_data")
        .blocklist_type("pjsip_tx_data__bindgen_ty_1")
        .blocklist_type("pjsip_tx_data__bindgen_ty_2")
        .blocklist_type("pjsip_transport_key")
        .blocklist_type("pjsip_transport")
        .blocklist_type("pjsip_tpfactory")
        .blocklist_type("pjsip_tpmgr_fla2_param")
        .blocklist_type("pjsip_transport_state_info")
        .blocklist_type("pjsip_tp_dropped_data")
        .blocklist_type("pjsip_process_rdata_param")
        .blocklist_type("pjsip_target")
        .blocklist_type("pjsip_target_set")
        .blocklist_type("pjsip_send_state")
        .blocklist_type("pjsip_response_addr")
        .blocklist_type("pjsip_udp_transport_cfg")
        .blocklist_type("pjsip_tcp_transport_cfg")
        .blocklist_type("pjsip_tls_on_accept_fail_param")
        .blocklist_type("pjsip_tls_setting")
        .blocklist_type("pjsip_common_credential")
        .blocklist_type("pjsip_digest_credential")
        .blocklist_type("pjsip_pgp_credential")
        .blocklist_type("pjsip_oauth_credential")
        .blocklist_type("pjsip_authorization_hdr")
        .blocklist_type("pjsip_authorization_hdr__bindgen_ty_1")
        .blocklist_type("pjsip_common_challenge")
        .blocklist_type("pjsip_digest_challenge")
        .blocklist_type("pjsip_pgp_challenge")
        .blocklist_type("pjsip_www_authenticate_hdr")
        .blocklist_type("pjsip_www_authenticate_hdr__bindgen_ty_1")
        .blocklist_type("pjsip_cred_info")
        .blocklist_type("pjsip_cred_info__bindgen_ty_1")
        .blocklist_type("pjsip_cred_info__bindgen_ty_1__bindgen_ty_1")
        .blocklist_type("pjsip_cached_auth")
        .blocklist_type("pjsip_auth_clt_pref")
        .blocklist_type("pjsip_auth_clt_sess")
        .blocklist_type("pjsip_auth_lookup_cred_param")
        .blocklist_type("pjsip_auth_srv")
        .blocklist_type("pjsip_auth_srv_init_param")
        .blocklist_type("pjsip_transaction")
        .blocklist_type("pjsip_ua_init_param")
        .blocklist_type("pjsip_dlg_party")
        .blocklist_type("pjsip_dialog")
        .blocklist_type("pjsip_dlg_create_uac_param")

        .allowlist_function("pjsip_100rel_attach")
        .allowlist_function("pjsip_100rel_create_prack")
        .allowlist_function("pjsip_100rel_end_session")
        .allowlist_function("pjsip_100rel_init_module")
        .allowlist_function("pjsip_100rel_is_reliable")
        .allowlist_function("pjsip_100rel_on_rx_prack")
        .allowlist_function("pjsip_100rel_send_prack")
        .allowlist_function("pjsip_100rel_tx_response")
        .allowlist_function("pjsip_create_sdp_body")
        .allowlist_function("pjsip_dlg_get_inv_session")
        .allowlist_function("pjsip_get_prack_method")
        .allowlist_function("pjsip_get_refer_method")
        .allowlist_function("pjsip_inv_add_ref")
        .allowlist_function("pjsip_inv_answer")
        .allowlist_function("pjsip_inv_cancel_reinvite")
        .allowlist_function("pjsip_inv_create_ack")
        .allowlist_function("pjsip_inv_create_uac")
        .allowlist_function("pjsip_inv_create_uas")
        .allowlist_function("pjsip_inv_dec_ref")
        .allowlist_function("pjsip_inv_end_session")
        .allowlist_function("pjsip_inv_initial_answer")
        .allowlist_function("pjsip_inv_invite")
        .allowlist_function("pjsip_inv_process_redirect")
        .allowlist_function("pjsip_inv_reinvite")
        .allowlist_function("pjsip_inv_send_msg")
        .allowlist_function("pjsip_inv_set_local_sdp")
        .allowlist_function("pjsip_inv_set_sdp_answer")
        .allowlist_function("pjsip_inv_state_name")
        .allowlist_function("pjsip_inv_terminate")
        .allowlist_function("pjsip_inv_uac_restart")
        .allowlist_function("pjsip_inv_update")
        .allowlist_function("pjsip_inv_usage_init")
        .allowlist_function("pjsip_inv_usage_instance")
        .allowlist_function("pjsip_inv_verify_request")
        .allowlist_function("pjsip_inv_verify_request2")
        .allowlist_function("pjsip_inv_verify_request3")
        .allowlist_function("pjsip_min_se_hdr_create")
        .allowlist_function("pjsip_rdata_get_sdp_info")
        .allowlist_function("pjsip_rdata_get_sdp_info2")
        .allowlist_function("pjsip_regc_add_headers")
        .allowlist_function("pjsip_regc_add_ref")
        .allowlist_function("pjsip_regc_create")
        .allowlist_function("pjsip_regc_dec_ref")
        .allowlist_function("pjsip_regc_destroy")
        .allowlist_function("pjsip_regc_get_info")
        .allowlist_function("pjsip_regc_get_pool")
        .allowlist_function("pjsip_regc_init")
        .allowlist_function("pjsip_regc_register")
        .allowlist_function("pjsip_regc_release_transport")
        .allowlist_function("pjsip_regc_send")
        .allowlist_function("pjsip_regc_set_credentials")
        .allowlist_function("pjsip_regc_set_delay_before_refresh")
        .allowlist_function("pjsip_regc_set_prefs")
        .allowlist_function("pjsip_regc_set_reg_tsx_cb")
        .allowlist_function("pjsip_regc_set_route_set")
        .allowlist_function("pjsip_regc_set_transport")
        .allowlist_function("pjsip_regc_set_via_sent_by")
        .allowlist_function("pjsip_regc_unregister")
        .allowlist_function("pjsip_regc_unregister_all")
        .allowlist_function("pjsip_regc_update_contact")
        .allowlist_function("pjsip_regc_update_expires")
        .allowlist_function("pjsip_replaces_hdr_create")
        .allowlist_function("pjsip_replaces_init_module")
        .allowlist_function("pjsip_replaces_verify_request")
        .allowlist_function("pjsip_sess_expires_hdr_create")
        .allowlist_function("pjsip_timer_end_session")
        .allowlist_function("pjsip_timer_handle_refresh_error")
        .allowlist_function("pjsip_timer_init_module")
        .allowlist_function("pjsip_timer_init_session")
        .allowlist_function("pjsip_timer_process_req")
        .allowlist_function("pjsip_timer_process_resp")
        .allowlist_function("pjsip_timer_setting_default")
        .allowlist_function("pjsip_timer_update_req")
        .allowlist_function("pjsip_timer_update_resp")
        .allowlist_function("pjsip_xfer_accept")
        .allowlist_function("pjsip_xfer_create_uac")
        .allowlist_function("pjsip_xfer_create_uas")
        .allowlist_function("pjsip_xfer_current_notify")
        .allowlist_function("pjsip_xfer_initiate")
        .allowlist_function("pjsip_xfer_init_module")
        .allowlist_function("pjsip_xfer_notify")
        .allowlist_function("pjsip_xfer_send_request")

        .allowlist_var("pjsip_prack_method")
        .allowlist_var("pjsip_refer_method")
        .allowlist_var("pjsip_update_method")


        // .whitelisted_type("PJSIP_.*")
        // .whitelisted_type("pjsip_.*")
        // .whitelisted_function("PJSIP_.*")
        // .whitelisted_function("pjsip_.*")
        // .whitelisted_var("PJSIP_.*")
        // .whitelisted_var("pjsip_.*")



        .allowlist_recursively(true)
        .translate_enum_integer_types(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .layout_tests(false)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = env::current_dir().unwrap();
    pjsip_ua.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

}
