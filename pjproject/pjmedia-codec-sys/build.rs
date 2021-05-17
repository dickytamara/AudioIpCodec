// extern crate bindgen;
// use std::env;


fn main () {

    // dynamic
    println!("cargo:rustc-link-lib=pjmedia-codec");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    // static
    // println!("cargo:rustc-link-lib=static=pjmedia-codec-x86_64-unknown-linux-gnu");
    // println!("cargo:rustc-link-search=native=/usr/local/lib");
    // println!("cargo:rerun-if-changed=wrapper.h");

    // generate lib.rs
    // create_bindgen();
}

// fn create_bindgen() {
//     let pjmedia_codec = bindgen::Builder::default()
//     .header("wrapper.h")
//     .raw_line("#![allow(non_upper_case_globals)]")
//     .raw_line("#![allow(non_camel_case_types)]")
//     .raw_line("#![allow(non_snake_case)]")
//     .raw_line("extern crate pj_sys;")
//     .raw_line("extern crate pjmedia_sys;")
//     .raw_line("use pj_sys::*;")
//     .raw_line("use pjmedia_sys::*;")


//     // pj-sys
//     // struct
//     .blocklist_type("pj_timestamp").blocklist_type("pj_sockaddr").blocklist_type("pj_str_t").blocklist_type("pj_timestamp__bindgen_ty_1")
//     .blocklist_type("pj_hash_table_t").blocklist_type("pj_hash_entry").blocklist_type("pj_hash_iterator_t").blocklist_type("pj_ioqueue_t")
//     .blocklist_type("pj_ioqueue_key_t").blocklist_type("pj_timer_heap_t").blocklist_type("pj_atomic_t").blocklist_type("pj_thread_t")
//     .blocklist_type("pj_lock_t").blocklist_type("pj_grp_lock_t").blocklist_type("pj_mutex_t").blocklist_type("pj_sem_t")
//     .blocklist_type("pj_event_t").blocklist_type("pj_pipe_t").blocklist_type("pj_time_val").blocklist_type("pj_parsed_time")
//     .blocklist_type("pj_ioqueue_op_key_t").blocklist_type("pj_ioqueue_callback").blocklist_type("__sigset_t").blocklist_type("in_addr.*")
//     .blocklist_type("in6_addr.*").blocklist_type("pj_sockaddr_in").blocklist_type("pj_sockaddr_in6").blocklist_type("pj_addr_hdr")
//     .blocklist_type("pj_ip_mreq").blocklist_type("pj_sockopt_params").blocklist_type("pj_sockopt_params__bindgen_ty_1").blocklist_type("pj_activesock_t")
//     .blocklist_type("pj_activesock_cb").blocklist_type("pj_activesock_cfg").blocklist_type("pj_hostent").blocklist_type("pj_addrinfo")
//     .blocklist_type("__jmp_buf_tag").blocklist_type("pj_exception_state_t").blocklist_type("pj_fifobuf_t").blocklist_type("pj_file_stat")
//     .blocklist_type("pj_ip_route_entry__bindgen_ty_1").blocklist_type("pj_enum_ip_option").blocklist_type("pj_list").blocklist_type("pj_grp_lock_config")
//     .blocklist_type("pj_math_stat").blocklist_type("pj_sys_info").blocklist_type("pj_symbianos_params").blocklist_type("pj_rwmutex_t")
//     .blocklist_type("pj_pool_block").blocklist_type("pj_pool_t").blocklist_type("pj_pool_factory_policy").blocklist_type("pj_pool_factory")
//     .blocklist_type("pj_caching_pool").blocklist_type("pj_rbtree_node").blocklist_type("pj_rbtree").blocklist_type("pj_qos_params")
//     .blocklist_type("pj_fd_set_t").blocklist_type("pj_ssl_sock_t").blocklist_type("pj_ssl_cert_t").blocklist_type("pj_ssl_cert_info")
//     .blocklist_type("pj_ssl_cert_info__bindgen_ty_1").blocklist_type("pj_ssl_cert_info__bindgen_ty_2").blocklist_type("pj_ssl_cert_info__bindgen_ty_3").blocklist_type("pj_ssl_cert_info__bindgen_ty_4")
//     .blocklist_type("pj_ssl_cert_info__bindgen_ty_4__bindgen_ty_1").blocklist_type("pj_ssl_cert_info__bindgen_ty_5").blocklist_type("pj_ssl_sock_cb").blocklist_type("pj_ssl_sock_info")
//     .blocklist_type("pj_ssl_sock_param").blocklist_type("pj_ssl_start_connect_param").blocklist_type("pj_timer_entry").blocklist_type("__va_list_tag")

//     // pj-sys
//     // type
//     .blocklist_type("pj_int64_t").blocklist_type("pj_uint64_t").blocklist_type("size_t").blocklist_type("pj_int32_t").blocklist_type("pj_uint32_t")
//     .blocklist_type("pj_int16_t").blocklist_type("pj_uint16_t").blocklist_type("pj_int8_t").blocklist_type("pj_uint8_t").blocklist_type("pj_size_t")
//     .blocklist_type("pj_ssize_t").blocklist_type("pj_status_t").blocklist_type("pj_bool_t").blocklist_type("pj_char_t").blocklist_type("pj_constants_")
//     .blocklist_type("pj_off_t").blocklist_type("pj_list_type").blocklist_type("pj_atomic_value_t").blocklist_type("pj_oshandle_t").blocklist_type("pj_sock_t")
//     .blocklist_type("pj_sockaddr_t").blocklist_type("pj_color_t").blocklist_type("pj_exception_id_t").blocklist_type("pj_exit_callback").blocklist_type("pj_ioqueue_operation_e")
//     .blocklist_type("__uint8_t").blocklist_type("__uint16_t").blocklist_type("__uint32_t").blocklist_type("in_addr_t").blocklist_type("pj_socket_sd_type")
//     .blocklist_type("pj_in_addr").blocklist_type("pj_in6_addr").blocklist_type("pj_os_err_type").blocklist_type("va_list").blocklist_type("pj_error_callback")
//     .blocklist_type("__jmp_buf").blocklist_type("jmp_buf").blocklist_type("pj_jmp_buf").blocklist_type("pj_log_decoration").blocklist_type("pj_log_func")
//     .blocklist_type("pj_file_access").blocklist_type("pj_file_seek_type").blocklist_type("pj_hash_entry_buf").blocklist_type("pj_highprec_t").blocklist_type("pj_sys_info_flag")
//     .blocklist_type("pj_thread_create_flags").blocklist_type("pj_thread_proc").blocklist_type("pj_thread_desc").blocklist_type("pj_mutex_type_e").blocklist_type("pj_main_func_ptr")
//     .blocklist_type("pj_pool_callback").blocklist_type("pj_rbcolor_t").blocklist_type("pj_rbtree_comp").blocklist_type("pj_qos_type").blocklist_type("pj_qos_flag")
//     .blocklist_type("pj_qos_wmm_prio").blocklist_type("pj_ssl_cert_verify_flag_t").blocklist_type("pj_ssl_cert_name_type").blocklist_type("pj_ssl_cert_buffer").blocklist_type("pj_ssl_cipher")
//     .blocklist_type("pj_ssl_curve").blocklist_type("pj_ssl_entropy").blocklist_type("pj_ssl_sock_proto").blocklist_type("pj_timer_id_t").blocklist_type("pj_timer_heap_callback")
//     .blocklist_type("__builtin_va_list")

//     // pj-sys
//     // pub static
//     .blocklist_item("PJ_VERSION").blocklist_item("PJ_AF_UNSPEC").blocklist_item("PJ_AF_UNIX").blocklist_item("PJ_AF_INET").blocklist_item("PJ_AF_INET6")
//     .blocklist_item("PJ_AF_PACKET").blocklist_item("PJ_AF_IRDA").blocklist_item("PJ_SOCK_STREAM").blocklist_item("PJ_SOCK_DGRAM").blocklist_item("PJ_SOCK_RAW")
//     .blocklist_item("PJ_SOCK_RDM").blocklist_item("PJ_SOL_SOCKET").blocklist_item("PJ_SOL_IP").blocklist_item("PJ_SOL_TCP").blocklist_item("PJ_SOL_UDP")
//     .blocklist_item("PJ_SOL_IPV6").blocklist_item("PJ_IP_TOS").blocklist_item("PJ_IPTOS_LOWDELAY").blocklist_item("PJ_IPTOS_THROUGHPUT").blocklist_item("PJ_IPTOS_RELIABILITY")
//     .blocklist_item("PJ_IPTOS_MINCOST").blocklist_item("PJ_IPV6_TCLASS").blocklist_item("PJ_SO_TYPE").blocklist_item("PJ_SO_RCVBUF").blocklist_item("PJ_SO_SNDBUF")
//     .blocklist_item("PJ_TCP_NODELAY").blocklist_item("PJ_SO_REUSEADDR").blocklist_item("PJ_SO_NOSIGPIPE").blocklist_item("PJ_SO_PRIORITY").blocklist_item("PJ_IP_MULTICAST_IF")
//     .blocklist_item("PJ_IP_MULTICAST_TTL").blocklist_item("PJ_IP_MULTICAST_LOOP").blocklist_item("PJ_IP_ADD_MEMBERSHIP").blocklist_item("PJ_IP_DROP_MEMBERSHIP").blocklist_item("PJ_MSG_OOB")
//     .blocklist_item("PJ_MSG_PEEK").blocklist_item("PJ_MSG_DONTROUTE").blocklist_item("PJ_GUID_STRING_LENGTH").blocklist_item("pj_pool_factory_default_policy")

//     .blocklist_item("pj_ssl_entropy").blocklist_item("pj_ssl_entropy_t")

//     // pjlib-util-sys
//     .blocklist_type("pj_dns_type")
//     .blocklist_type("pj_dns_callback")
//     .blocklist_type("pj_dns_srv_resolver_cb")
//     .blocklist_type("pj_cis_elem_t")
//     .blocklist_type("pj_syn_err_func_ptr")
//     .blocklist_type("pj_json_val_type")
//     .blocklist_type("pj_json_writer")
//     .blocklist_type("pjstun_attr_type")
//     .blocklist_type("pj_pcap_link_type")
//     .blocklist_type("pj_pcap_proto_type")
//     .blocklist_type("pj_cli_cmd_id")
//     .blocklist_type("pj_cli_arg_id")
//     .blocklist_type("pj_cli_get_dyn_choice")
//     .blocklist_type("pj_cli_cmd_handler")
//     .blocklist_type("pj_cli_front_end_type")
//     .blocklist_type("pj_cli_telnet_on_started")

//     // pjnath-sys
//     // pub type
//     .blocklist_type("pj_stun_method_e").blocklist_type("pj_stun_msg_class_e").blocklist_type("pj_stun_msg_type").blocklist_type("pj_stun_attr_type")
//     .blocklist_type("pj_stun_status").blocklist_type("pj_stun_fingerprint_attr").blocklist_type("pj_stun_realm_attr").blocklist_type("pj_stun_nonce_attr")
//     .blocklist_type("pj_stun_mapped_addr_attr").blocklist_type("pj_stun_xor_mapped_addr_attr").blocklist_type("pj_stun_software_attr").blocklist_type("pj_stun_alt_server_attr")
//     .blocklist_type("pj_stun_refresh_interval_attr").blocklist_type("pj_stun_response_addr_attr").blocklist_type("pj_stun_changed_addr_attr").blocklist_type("pj_stun_change_request_attr")
//     .blocklist_type("pj_stun_src_addr_attr").blocklist_type("pj_stun_reflected_from_attr").blocklist_type("pj_stun_username_attr").blocklist_type("pj_stun_password_attr")
//     .blocklist_type("pj_stun_channel_number_attr").blocklist_type("pj_stun_lifetime_attr").blocklist_type("pj_stun_bandwidth_attr").blocklist_type("pj_stun_xor_peer_addr_attr")
//     .blocklist_type("pj_stun_data_attr").blocklist_type("pj_stun_xor_relayed_addr_attr").blocklist_type("pj_stun_req_addr_type_attr").blocklist_type("pj_stun_even_port_attr")
//     .blocklist_type("pj_stun_req_transport_attr").blocklist_type("pj_stun_dont_fragment_attr").blocklist_type("pj_stun_res_token_attr").blocklist_type("pj_stun_xor_reflected_from_attr")
//     .blocklist_type("pj_stun_priority_attr").blocklist_type("pj_stun_use_candidate_attr").blocklist_type("pj_stun_timer_val_attr").blocklist_type("pj_stun_ice_controlling_attr")
//     .blocklist_type("pj_stun_ice_controlled_attr").blocklist_type("pj_stun_icmp_attr").blocklist_type("pj_stun_decode_options").blocklist_type("pj_stun_auth_type")
//     .blocklist_type("pj_stun_auth_cred_type").blocklist_type("pj_stun_passwd_type").blocklist_type("pj_stun_sess_msg_log_flag").blocklist_type("pj_ice_cand_type")
//     .blocklist_type("pj_ice_sess_check_state").blocklist_type("pj_ice_sess_checklist_state").blocklist_type("pj_ice_sess_role").blocklist_type("pj_ice_sess_trickle")
//     .blocklist_type("pj_dns_rcode").blocklist_type("pj_dns_dup_options").blocklist_type("pj_stun_sock_op").blocklist_type("pj_turn_tp_type")
//     .blocklist_type("pj_turn_state_t").blocklist_type("pj_ice_strans_op").blocklist_type("pj_ice_strans_state").blocklist_type("pj_stun_nat_type")
//     .blocklist_type("pj_stun_nat_detect_cb")

//     // pjnath-sys
//     // pub struct
//     .blocklist_type("pj_stun_msg_hdr").blocklist_type("pj_stun_attr_hdr").blocklist_type("pj_stun_sockaddr_attr").blocklist_type("pj_stun_empty_attr")
//     .blocklist_type("pj_stun_string_attr").blocklist_type("pj_stun_uint_attr").blocklist_type("pj_stun_uint64_attr").blocklist_type("pj_stun_binary_attr")
//     .blocklist_type("pj_stun_msgint_attr").blocklist_type("pj_stun_errcode_attr").blocklist_type("pj_stun_unknown_attr").blocklist_type("pj_stun_msg")
//     .blocklist_type("pj_stun_auth_cred").blocklist_type("pj_stun_req_cred_info").blocklist_type("pj_stun_config").blocklist_type("pj_stun_client_tsx")
//     .blocklist_type("pj_stun_tsx_cb").blocklist_type("pj_stun_session").blocklist_type("pj_stun_session_cb").blocklist_type("pj_stun_rx_data")
//     .blocklist_type("pj_stun_tx_data").blocklist_type("pj_ice_sess_comp").blocklist_type("pj_ice_msg_data").blocklist_type("pj_ice_msg_data_data_request_data")
//     .blocklist_type("pj_ice_sess_cand").blocklist_type("pj_ice_sess_check").blocklist_type("pj_ice_sess_checklist").blocklist_type("pj_ice_sess_cb")
//     .blocklist_type("pj_ice_rx_check").blocklist_type("pj_ice_sess_options").blocklist_type("pj_ice_sess").blocklist_type("pj_dns_hdr")
//     .blocklist_type("pj_dns_parsed_query").blocklist_type("pj_dns_parsed_rr").blocklist_type("pj_dns_parsed_rr_rdata").blocklist_type("pj_dns_parsed_rr_rdata_srv")
//     .blocklist_type("pj_dns_parsed_rr_rdata_cname").blocklist_type("pj_dns_parsed_rr_rdata_ns").blocklist_type("pj_dns_parsed_rr_rdata_ptr").blocklist_type("pj_dns_parsed_rr_rdata_a")
//     .blocklist_type("pj_dns_parsed_rr_rdata_aaaa").blocklist_type("pj_dns_parsed_packet").blocklist_type("pj_dns_resolver").blocklist_type("pj_dns_async_query")
//     .blocklist_type("pj_dns_settings").blocklist_type("pj_dns_a_record").blocklist_type("pj_dns_addr_record").blocklist_type("pj_stun_sock")
//     .blocklist_type("pj_stun_sock_cb").blocklist_type("pj_stun_sock_info").blocklist_type("pj_stun_sock_cfg").blocklist_type("pj_turn_session")
//     .blocklist_type("pj_turn_channel_data").blocklist_type("pj_turn_session_cb").blocklist_type("pj_turn_alloc_param").blocklist_type("pj_turn_session_info")
//     .blocklist_type("pj_turn_session_on_rx_pkt_param").blocklist_type("pj_turn_sock").blocklist_type("pj_turn_sock_cb").blocklist_type("pj_turn_sock_tls_cfg")
//     .blocklist_type("pj_turn_sock_cfg").blocklist_type("pj_ice_strans").blocklist_type("pj_ice_strans_cb").blocklist_type("pj_ice_strans_stun_cfg")
//     .blocklist_type("pj_ice_strans_turn_cfg").blocklist_type("pj_ice_strans_cfg").blocklist_type("pj_stun_nat_detect_result")

//     .blocklist_type("pj_stun_auth_cred__bindgen_ty_1")
//     .blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_1")
//     .blocklist_type("pj_stun_auth_cred__bindgen_ty_1__bindgen_ty_2")
//     .blocklist_type("pj_dns_addr_record__bindgen_ty_1")
//     .blocklist_type("pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1")
//     .blocklist_type("pj_ice_strans_cfg__bindgen_ty_1")

//     .blocklist_type("pj_ice_msg_data_data")
//     .blocklist_type("pj_ice_sess__bindgen_ty_1")


//     // pjmedia-sys
//     // pub type
//     .blocklist_type("pjmedia_type")
//     .blocklist_type("pjmedia_tp_proto")
//     .blocklist_type("pjmedia_dir")
//     .blocklist_type("pjmedia_coord_base")
//     .blocklist_type("pjmedia_orient")
//     .blocklist_type("pjmedia_clock_options")
//     .blocklist_type("pjmedia_clock_callback")
//     .blocklist_type("pjmedia_format_id")
//     .blocklist_type("pjmedia_format_detail_type")
//     .blocklist_type("pjmedia_color_model")
//     .blocklist_type("pjmedia_frame_type")
//     .blocklist_type("pjmedia_aud_dev_index")
//     .blocklist_type("pjmedia_aud_dev_factory_create_func_ptr")
//     .blocklist_type("pjmedia_aud_dev_cap")
//     .blocklist_type("pjmedia_aud_dev_route")
//     .blocklist_type("pjmedia_aud_play_cb")
//     .blocklist_type("pjmedia_aud_rec_cb")
//     .blocklist_type("pjmedia_rtcp_xr_type")
//     .blocklist_type("pjmedia_rtcp_xr_info")
//     .blocklist_type("pjmedia_rtcp_xr_plc_type")
//     .blocklist_type("pjmedia_rtcp_xr_jb_type")
//     .blocklist_type("pjmedia_rtcp_fb_type")
//     .blocklist_type("pjmedia_obj_sig")
//     .blocklist_type("pjmedia_vid_dev_index")
//     .blocklist_type("pjmedia_vid_dev_hwnd_type")
//     .blocklist_type("pjmedia_vid_dev_wnd_flag")
//     .blocklist_type("pjmedia_vid_dev_std_index")
//     .blocklist_type("pjmedia_vid_dev_cap")
//     .blocklist_type("pjmedia_vid_dev_factory_create_func_ptr")
//     .blocklist_type("pjmedia_event_type")
//     .blocklist_type("pjmedia_event_wnd_closed_data")
//     .blocklist_type("pjmedia_event_mouse_btn_down_data")
//     .blocklist_type("pjmedia_event_keyframe_found_data")
//     .blocklist_type("pjmedia_event_keyframe_missing_data")
//     .blocklist_type("pjmedia_event_user_data")
//     .blocklist_type("pjmedia_event_cb")
//     .blocklist_type("pjmedia_event_publish_flag")
//     .blocklist_type("pjmedia_event_mgr_flag")
//     .blocklist_type("pjmedia_port_op")
//     .blocklist_type("pjmedia_avi_file_player_option")
//     .blocklist_type("pjmedia_avi_stream")
//     .blocklist_type("pjmedia_rtp_pt")
//     .blocklist_type("pjmedia_codec_priority")
//     .blocklist_type("pjmedia_codec_id")
//     .blocklist_type("pjmedia_conf_option")
//     .blocklist_type("pjmedia_converter_priority_guide")
//     .blocklist_type("pjmedia_converter_convert_setting")
//     .blocklist_type("pjmedia_delay_buf_flag")
//     .blocklist_type("pjmedia_echo_flag")
//     .blocklist_type("pjmedia_tranport_media_option")
//     .blocklist_type("pjmedia_transport_type")
//     .blocklist_type("pjmedia_endpt_flag")
//     .blocklist_type("pjmedia_endpt_exit_callback")
//     .blocklist_type("pjmedia_audio_pt")
//     .blocklist_type("pjmedia_video_pt")
//     .blocklist_type("pjmedia_jb_frame_type")
//     .blocklist_type("pjmedia_jb_discard_algo")
//     .blocklist_type("pjmedia_mem_player_option")
//     .blocklist_type("pjmedia_resample_port_options")
//     .blocklist_type("pjmedia_sdp_neg_state")
//     .blocklist_type("pjmedia_mod_offer_flag")
//     .blocklist_type("pjmedia_sdp_neg_fmt_match_flag")
//     .blocklist_type("pjmedia_sdp_neg_fmt_match_cb")
//     .blocklist_type("pjmedia_snd_play_cb")
//     .blocklist_type("pjmedia_snd_rec_cb")
//     .blocklist_type("pjmedia_snd_port_option")
//     .blocklist_type("pjmedia_stereo_port_options")
//     .blocklist_type("pjmedia_vid_packing")
//     .blocklist_type("pjmedia_vid_frm_bit_info")
//     .blocklist_type("pjmedia_stream_dtmf_event_flags")
//     .blocklist_type("_bindgen_ty_12")
//     .blocklist_type("pjmedia_transport_ice_options")
//     .blocklist_type("pjmedia_srtp_crypto_option")
//     .blocklist_type("pjmedia_srtp_use")
//     .blocklist_type("pjmedia_srtp_keying_method")
//     .blocklist_type("pjmedia_transport_udp_options")
//     .blocklist_type("pjmedia_vid_conf_layout")
//     .blocklist_type("pjmedia_vid_stream_rc_method")
//     .blocklist_type("pjmedia_file_player_option")
//     .blocklist_type("pjmedia_file_writer_option")
//     .blocklist_type("pjmedia_wave_fmt_tag")
//     .blocklist_type("pjmedia_wsola_option")
//     // pjmedia-sys
//     // pub struct
//     .blocklist_type("pjmedia_endpt")
//     .blocklist_type("pjmedia_stream")
//     .blocklist_type("pjmedia_ratio")
//     .blocklist_type("pjmedia_coord")
//     .blocklist_type("pjmedia_rect_size")
//     .blocklist_type("pjmedia_rect")
//     .blocklist_type("pjmedia_clock_src")
//     .blocklist_type("pjmedia_clock")
//     .blocklist_type("pjmedia_clock_param")
//     .blocklist_type("pjmedia_audio_format_detail")
//     .blocklist_type("pjmedia_video_format_detail")
//     .blocklist_type("pjmedia_format")
//     .blocklist_type("pjmedia_format__bindgen_ty_1")
//     .blocklist_type("pjmedia_video_apply_fmt_param")
//     .blocklist_type("pjmedia_video_format_info")
//     .blocklist_type("pjmedia_video_format_mgr")
//     .blocklist_type("pjmedia_frame")
//     .blocklist_type("pjmedia_frame_ext")
//     .blocklist_type("pjmedia_frame_ext_subframe")
//     .blocklist_type("pjmedia_aud_stream")
//     .blocklist_type("pjmedia_aud_dev_factory")
//     .blocklist_type("pjmedia_aud_driver")
//     .blocklist_type("pjmedia_aud_subsys")
//     .blocklist_type("pjmedia_aud_dev_info")
//     .blocklist_type("pjmedia_aud_param")
//     .blocklist_type("pjmedia_rtcp_xr_rb_header")
//     .blocklist_type("pjmedia_rtcp_xr_rb_rr_time")
//     .blocklist_type("pjmedia_rtcp_xr_rb_dlrr_item")
//     .blocklist_type("pjmedia_rtcp_xr_rb_dlrr")
//     .blocklist_type("pjmedia_rtcp_xr_rb_stats")
//     .blocklist_type("pjmedia_rtcp_xr_rb_voip_mtc")
//     .blocklist_type("pjmedia_rtcp_xr_pkt")
//     .blocklist_type("pjmedia_rtcp_xr_pkt__bindgen_ty_1")
//     .blocklist_type("pjmedia_rtcp_xr_stream_stat")
//     .blocklist_type("pjmedia_rtcp_xr_stream_stat__bindgen_ty_1")
//     .blocklist_type("pjmedia_rtcp_xr_stream_stat__bindgen_ty_2")
//     .blocklist_type("pjmedia_rtcp_xr_stat")
//     .blocklist_type("pjmedia_rtcp_xr_session")
//     .blocklist_type("pjmedia_rtcp_xr_session__bindgen_ty_1")
//     .blocklist_type("pjmedia_rtp_hdr")
//     .blocklist_type("pjmedia_rtp_ext_hdr")
//     .blocklist_type("pjmedia_rtp_dec_hdr")
//     .blocklist_type("pjmedia_rtp_dtmf_event")
//     .blocklist_type("pjmedia_rtp_seq_session")
//     .blocklist_type("pjmedia_rtp_session")
//     .blocklist_type("pjmedia_rtp_status")
//     .blocklist_type("pjmedia_rtp_status__bindgen_ty_1")
//     .blocklist_type("pjmedia_rtp_status__bindgen_ty_1_flag")
//     .blocklist_type("pjmedia_rtp_session_setting")
//     .blocklist_type("pjmedia_rtcp_sr")
//     .blocklist_type("pjmedia_rtcp_rr")
//     .blocklist_type("pjmedia_rtcp_common")
//     .blocklist_type("pjmedia_rtcp_sr_pkt")
//     .blocklist_type("pjmedia_rtcp_rr_pkt")
//     .blocklist_type("pjmedia_rtcp_sdes")
//     .blocklist_type("pjmedia_rtcp_ntp_rec")
//     .blocklist_type("pjmedia_rtcp_stream_stat")
//     .blocklist_type("pjmedia_rtcp_stream_stat__bindgen_ty_1")
//     .blocklist_type("pjmedia_rtcp_stat")
//     .blocklist_type("pjmedia_rtcp_session")
//     .blocklist_type("pjmedia_rtcp_session_setting")
//     .blocklist_type("pjmedia_sdp_attr")
//     .blocklist_type("pjmedia_sdp_rtpmap")
//     .blocklist_type("pjmedia_sdp_fmtp")
//     .blocklist_type("pjmedia_sdp_rtcp_attr")
//     .blocklist_type("pjmedia_sdp_ssrc_attr")
//     .blocklist_type("pjmedia_sdp_conn")
//     .blocklist_type("pjmedia_sdp_bandw")
//     .blocklist_type("pjmedia_sdp_media")
//     .blocklist_type("pjmedia_sdp_media__bindgen_ty_1")
//     .blocklist_type("pjmedia_sdp_session")
//     .blocklist_type("pjmedia_sdp_session__bindgen_ty_1")
//     .blocklist_type("pjmedia_sdp_session__bindgen_ty_2")
//     .blocklist_type("pjmedia_rtcp_fb_cap")
//     .blocklist_type("pjmedia_rtcp_fb_info")
//     .blocklist_type("pjmedia_rtcp_fb_setting")
//     .blocklist_type("pjmedia_rtcp_fb_nack")
//     .blocklist_type("pjmedia_rtcp_fb_sli")
//     .blocklist_type("pjmedia_rtcp_fb_rpsi")
//     .blocklist_type("pjmedia_event_rx_rtcp_fb_data")
//     .blocklist_type("pjmedia_event_rx_rtcp_fb_data__bindgen_ty_1")
//     .blocklist_type("pjmedia_vid_dev_hwnd")
//     .blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_1")
//     .blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_2")
//     .blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_3")
//     .blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_4")
//     .blocklist_type("pjmedia_vid_dev_hwnd__bindgen_ty_1__bindgen_ty_5")
//     .blocklist_type("pjmedia_vid_dev_switch_param")
//     .blocklist_type("pjmedia_vid_dev_info")
//     .blocklist_type("pjmedia_vid_dev_stream")
//     .blocklist_type("pjmedia_vid_dev_cb")
//     .blocklist_type("pjmedia_vid_dev_param")
//     .blocklist_type("pjmedia_vid_dev_factory")
//     .blocklist_type("pjmedia_vid_driver")
//     .blocklist_type("pjmedia_vid_subsys")
//     .blocklist_type("pjmedia_event_fmt_changed_data")
//     .blocklist_type("pjmedia_event_dummy_data")
//     .blocklist_type("pjmedia_event_wnd_resized_data")
//     .blocklist_type("pjmedia_event_wnd_closing_data")
//     .blocklist_type("pjmedia_event_aud_dev_err_data")
//     .blocklist_type("pjmedia_event_vid_dev_err_data")
//     .blocklist_type("pjmedia_event_media_tp_err_data")
//     .blocklist_type("pjmedia_event")
//     .blocklist_type("pjmedia_event__bindgen_ty_1")
//     .blocklist_type("pjmedia_event_mgr")
//     .blocklist_type("pjmedia_port_info")
//     .blocklist_type("pjmedia_port")
//     .blocklist_type("pjmedia_port_port_data")
//     .blocklist_type("pjmedia_avi_streams")
//     .blocklist_type("pjmedia_circ_buf")
//     .blocklist_type("pjmedia_codec_info")
//     .blocklist_type("pjmedia_codec_fmtp")
//     .blocklist_type("pjmedia_codec_fmtp_param")
//     .blocklist_type("pjmedia_codec_param")
//     .blocklist_type("pjmedia_codec_param__bindgen_ty_1")
//     .blocklist_type("pjmedia_codec_param__bindgen_ty_2")
//     .blocklist_type("pjmedia_codec_op")
//     .blocklist_type("pjmedia_codec")
//     .blocklist_type("pjmedia_codec_factory_op")
//     .blocklist_type("pjmedia_codec_factory")
//     .blocklist_type("pjmedia_codec_default_param")
//     .blocklist_type("pjmedia_codec_desc")
//     .blocklist_type("pjmedia_codec_mgr")
//     .blocklist_type("pjmedia_conf")
//     .blocklist_type("pjmedia_conf_port_info")
//     .blocklist_type("pjmedia_conversion_param")
//     .blocklist_type("pjmedia_converter_factory")
//     .blocklist_type("pjmedia_converter")
//     .blocklist_type("pjmedia_converter_factory_op")
//     .blocklist_type("pjmedia_converter_op")
//     .blocklist_type("pjmedia_converter_mgr")
//     .blocklist_type("pjmedia_delay_buf")
//     .blocklist_type("pjmedia_echo_state")
//     .blocklist_type("pjmedia_echo_stat")
//     .blocklist_type("pjmedia_sock_info")
//     .blocklist_type("pjmedia_transport_op")
//     .blocklist_type("pjmedia_transport")
//     .blocklist_type("pjmedia_transport_specific_info")
//     .blocklist_type("pjmedia_transport_info")
//     .blocklist_type("pjmedia_tp_cb_param")
//     .blocklist_type("pjmedia_transport_attach_param")
//     .blocklist_type("pjmedia_jb_state")
//     .blocklist_type("pjmedia_jbuf")
//     .blocklist_type("pjmedia_master_port")
//     .blocklist_type("pjmedia_plc")
//     .blocklist_type("pjmedia_resample")
//     .blocklist_type("pjmedia_sdp_neg")
//     .blocklist_type("pjmedia_silence_det")
//     .blocklist_type("pjmedia_snd_stream")
//     .blocklist_type("pjmedia_snd_dev_info")
//     .blocklist_type("pjmedia_snd_stream_info")
//     .blocklist_type("pjmedia_snd_port_param")
//     .blocklist_type("pjmedia_snd_port")
//     .blocklist_type("pjmedia_vid_encode_opt")
//     .blocklist_type("pjmedia_vid_codec_info")
//     .blocklist_type("pjmedia_vid_codec_param")
//     .blocklist_type("pjmedia_vid_codec_op")
//     .blocklist_type("pjmedia_vid_codec")
//     .blocklist_type("pjmedia_vid_codec_factory_op")
//     .blocklist_type("pjmedia_vid_codec_factory")
//     .blocklist_type("pjmedia_vid_codec_mgr")
//     .blocklist_type("pjmedia_stream_rtp_sess_info")
//     .blocklist_type("pjmedia_channel")
//     .blocklist_type("pjmedia_stream_info")
//     .blocklist_type("pjmedia_stream_dtmf_event")
//     .blocklist_type("pjmedia_tone_desc")
//     .blocklist_type("pjmedia_tone_digit")
//     .blocklist_type("pjmedia_tone_digit_map")
//     .blocklist_type("pjmedia_tone_digit_map__bindgen_ty_1")
//     .blocklist_type("pjmedia_ice_cb")
//     .blocklist_type("pjmedia_ice_transport_info")
//     .blocklist_type("pjmedia_ice_transport_info__bindgen_ty_1")
//     .blocklist_type("pjmedia_loop_tp_setting")
//     .blocklist_type("pjmedia_srtp_crypto")
//     .blocklist_type("pjmedia_srtp_cb")
//     .blocklist_type("pjmedia_srtp_setting")
//     .blocklist_type("pjmedia_srtp_info")
//     .blocklist_type("pjmedia_srtp_dtls_nego_param")
//     .blocklist_type("pjmedia_vid_conf")
//     .blocklist_type("pjmedia_vid_conf_setting")
//     .blocklist_type("pjmedia_vid_conf_port_info")
//     .blocklist_type("pjmedia_vid_port_param")
//     .blocklist_type("pjmedia_vid_port")
//     .blocklist_type("pjmedia_vid_stream_rc_config")
//     .blocklist_type("pjmedia_vid_stream_sk_config")
//     .blocklist_type("pjmedia_vid_stream_info")
//     .blocklist_type("pjmedia_vid_stream")
//     .blocklist_type("pjmedia_wav_player_info")
//     .blocklist_type("pjmedia_wave_hdr")
//     .blocklist_type("pjmedia_wave_hdr__bindgen_ty_1")
//     .blocklist_type("pjmedia_wave_hdr__bindgen_ty_2")
//     .blocklist_type("pjmedia_wave_hdr__bindgen_ty_3")
//     .blocklist_type("pjmedia_wave_subchunk")
//     .blocklist_type("pjmedia_wsola")

//     .allowlist_function("pjmedia_audio_codec_config_default")
//     .allowlist_function("pjmedia_codec_amr_match_sdp")
//     .allowlist_function("pjmedia_codec_g7221_match_sdp")
//     // .allowlist_function("pjmedia_codec_opus_deinit")
//     // .allowlist_function("pjmedia_codec_opus_get_config")
//     // .allowlist_function("pjmedia_codec_opus_init")
//     // .allowlist_function("pjmedia_codec_opus_set_default_param")
//     .allowlist_function("pjmedia_codec_passthrough.*")
//     .allowlist_function("pjmedia_codec_bcg729.*")
//     .allowlist_function("pjmedia_codec_ffmpeg.*")
//     .allowlist_function("pjmedia_codec_g722.*")
//     .allowlist_function("pjmedia_codec_g7221.*")
//     .allowlist_function("pjmedia_codec_gsm.*")
//     .allowlist_function("pjmedia_codec_ilbc.*")
//     .allowlist_function("pjmedia_codec_ipp.*")
//     .allowlist_function("pjmedia_codec_l16.*")
//     .allowlist_function("pjmedia_codec_opencore_amr.*")
//     .allowlist_function("pjmedia_codec_openh264.*")
//     .allowlist_function("pjmedia_codec_opus.*")
//     .allowlist_function("pjmedia_codec_silk.*")
//     .allowlist_function("pjmedia_codec_speex.*")
//     .allowlist_function("pjmedia_codec_vid.*")
//     .allowlist_function("pjmedia_codec_vpx.*")
//     .allowlist_function("pjmedia_codec_register_audio_codecs")
//     // .allowlist_function("pjmedia_codec_.*")

//     .allowlist_type("PJ.*")
//     .allowlist_type("pj.*")

//     .allowlist_recursively(true)
//     .translate_enum_integer_types(true)
//     .layout_tests(false)
//     .prepend_enum_name(false)
//     .generate()
//     // Unwrap the Result and panic on failure.
//     .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     // let out_path = PathBuf::from(env::var("SRC_DIR").unwrap());
//     let out_path = env::current_dir().unwrap();
//     pjmedia_codec.write_to_file(out_path.join("src/lib.rs")).expect("Error write lib.rs");
// }