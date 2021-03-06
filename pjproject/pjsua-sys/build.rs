
// extern crate bindgen;
// use std::env;


fn main () {

    // dynamic
    println!("cargo:rustc-link-lib=pjsua");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    // static
    // println!("cargo:rustc-link-lib=static=pjsua-x86_64-unknown-linux-gnu");
    // println!("cargo:rustc-link-search=native=/usr/local/lib");
    // println!("cargo:rerun-if-changed=wrapper.h");


    // generate lib.rs
    // create_bindgen();

}

// fn create_bindgen() {
//     let pjsua = bindgen::Builder::default().header("wrapper.h")

//     .raw_line("#![allow(non_upper_case_globals)]")
//     .raw_line("#![allow(non_camel_case_types)]")
//     .raw_line("#![allow(non_snake_case)]")
//     .raw_line("extern crate pj_sys;")
//     // .raw_line("extern crate pjlib_util_sys;")

//     .raw_line("extern crate pjnath_sys;")
//     .raw_line("extern crate pjmedia_sys;")
//     .raw_line("extern crate pjsip_sys;")
//     .raw_line("extern crate pjsip_simple_sys;")
//     .raw_line("extern crate pjsip_ua_sys;")
//     .raw_line("use pj_sys::*;")

//     // .raw_line("use pjlib_util_sys::*;")

//     .raw_line("use pjnath_sys::*;")
//     .raw_line("use pjmedia_sys::*;")
//     .raw_line("use pjsip_sys::*;")
//     .raw_line("use pjsip_simple_sys::*;")
//     .raw_line("use pjsip_ua_sys::*;")

//     .allowlist_function("acquire_call")
//     .allowlist_function("call_media_on_event")
//     .allowlist_function("call_update_contact")
//     .allowlist_function("create_temp_sdp")
//     .allowlist_function("good_number")
//     .allowlist_function("normalize_route_uri")
//     .allowlist_function("on_dlg_forked")
//     .allowlist_function("on_media_event")
//     .allowlist_function("pjsua_acc_add")
//     .allowlist_function("pjsua_acc_add_local")
//     .allowlist_function("pjsua_acc_config_default")
//     .allowlist_function("pjsua_acc_config_dup")
//     .allowlist_function("pjsua_acc_create_request")
//     .allowlist_function("pjsua_acc_create_uac_contact")
//     .allowlist_function("pjsua_acc_create_uas_contact")
//     .allowlist_function("pjsua_acc_del")
//     .allowlist_function("pjsua_acc_end_ip_change")
//     .allowlist_function("pjsua_acc_enum_info")
//     .allowlist_function("pjsua_acc_find_for_incoming")
//     .allowlist_function("pjsua_acc_find_for_outgoing")
//     .allowlist_function("pjsua_acc_get_config")
//     .allowlist_function("pjsua_acc_get_count")
//     .allowlist_function("pjsua_acc_get_default")
//     .allowlist_function("pjsua_acc_get_info")
//     .allowlist_function("pjsua_acc_get_uac_addr")
//     .allowlist_function("pjsua_acc_get_user_data")
//     .allowlist_function("pjsua_acc_handle_call_on_ip_change")
//     .allowlist_function("pjsua_acc_is_valid")
//     .allowlist_function("pjsua_acc_modify")
//     .allowlist_function("pjsua_acc_on_tp_state_changed")
//     .allowlist_function("pjsua_acc_set_default")
//     .allowlist_function("pjsua_acc_set_online_status")
//     .allowlist_function("pjsua_acc_set_online_status2")
//     .allowlist_function("pjsua_acc_set_registration")
//     .allowlist_function("pjsua_acc_set_transport")
//     .allowlist_function("pjsua_acc_set_user_data")
//     .allowlist_function("pjsua_acc_update_contact_on_ip_change")
//     .allowlist_function("pjsua_aud_channel_update")
//     .allowlist_function("pjsua_aud_stop_stream")
//     .allowlist_function("pjsua_aud_subsys_destroy")
//     .allowlist_function("pjsua_aud_subsys_init")
//     .allowlist_function("pjsua_aud_subsys_start")
//     .allowlist_function("pjsua_buddy_add")
//     .allowlist_function("pjsua_buddy_config_default")
//     .allowlist_function("pjsua_buddy_del")
//     .allowlist_function("pjsua_buddy_find")
//     .allowlist_function("pjsua_buddy_get_info")
//     .allowlist_function("pjsua_buddy_get_user_data")
//     .allowlist_function("pjsua_buddy_is_valid")
//     .allowlist_function("pjsua_buddy_set_user_data")
//     .allowlist_function("pjsua_buddy_subscribe_pres")
//     .allowlist_function("pjsua_buddy_update_pres")
//     .allowlist_function("pjsua_call_answer")
//     .allowlist_function("pjsua_call_answer2")
//     .allowlist_function("pjsua_call_answer_with_sdp")
//     .allowlist_function("pjsua_call_cleanup_flag")
//     .allowlist_function("pjsua_call_dial_dtmf")
//     .allowlist_function("pjsua_call_dump")
//     .allowlist_function("pjsua_call_get_conf_port")
//     .allowlist_function("pjsua_call_get_count")
//     .allowlist_function("pjsua_call_get_info")
//     .allowlist_function("pjsua_call_get_max_count")
//     .allowlist_function("pjsua_call_get_med_transport_info")
//     .allowlist_function("pjsua_call_get_rem_nat_type")
//     .allowlist_function("pjsua_call_get_stream_info")
//     .allowlist_function("pjsua_call_get_stream_stat")
//     .allowlist_function("pjsua_call_get_user_data")
//     .allowlist_function("pjsua_call_hangup")
//     .allowlist_function("pjsua_call_hangup_all")
//     .allowlist_function("pjsua_call_has_media")
//     .allowlist_function("pjsua_call_is_active")
//     .allowlist_function("pjsua_call_make_call")
//     .allowlist_function("pjsua_call_media_init")
//     .allowlist_function("pjsua_call_media_is_changing")
//     .allowlist_function("pjsua_call_on_incoming")
//     .allowlist_function("pjsua_call_process_redirect")
//     .allowlist_function("pjsua_call_reinvite")
//     .allowlist_function("pjsua_call_reinvite2")
//     .allowlist_function("pjsua_call_remote_has_cap")
//     .allowlist_function("pjsua_call_schedule_reinvite_check")
//     .allowlist_function("pjsua_call_send_dtmf")
//     .allowlist_function("pjsua_call_send_dtmf_param_default")
//     .allowlist_function("pjsua_call_send_im")
//     .allowlist_function("pjsua_call_send_request")
//     .allowlist_function("pjsua_call_send_typing_ind")
//     .allowlist_function("pjsua_call_set_hold")
//     .allowlist_function("pjsua_call_set_hold2")
//     .allowlist_function("pjsua_call_setting_default")
//     .allowlist_function("pjsua_call_set_user_data")
//     .allowlist_function("pjsua_call_subsys_init")
//     .allowlist_function("pjsua_call_subsys_start")
//     .allowlist_function("pjsua_call_update")
//     .allowlist_function("pjsua_call_update2")
//     .allowlist_function("pjsua_call_xfer")
//     .allowlist_function("pjsua_call_xfer_replaces")
//     .allowlist_function("pjsua_cancel_stun_resolution")
//     .allowlist_function("pjsua_cancel_timer")
//     .allowlist_function("pjsua_check_snd_dev_idle")
//     .allowlist_function("pjsua_codec_get_param")
//     .allowlist_function("pjsua_codec_set_param")
//     .allowlist_function("pjsua_codec_set_priority")
//     .allowlist_function("pjsua_conf_add_port")
//     .allowlist_function("pjsua_conf_adjust_rx_level")
//     .allowlist_function("pjsua_conf_adjust_tx_level")
//     .allowlist_function("pjsua_conf_connect")
//     .allowlist_function("pjsua_conf_connect2")
//     .allowlist_function("pjsua_conf_connect_param_default")
//     .allowlist_function("pjsua_conf_disconnect")
//     .allowlist_function("pjsua_conf_get_active_ports")
//     .allowlist_function("pjsua_conf_get_max_ports")
//     .allowlist_function("pjsua_conf_get_msignal_level")
//     .allowlist_function("pjsua_conf_get_port_info")
//     .allowlist_function("pjsua_conf_get_signal_level")
//     .allowlist_function("pjsua_config_default")
//     .allowlist_function("pjsua_config_dup")
//     .allowlist_function("pjsua_conf_remove_port")
//     .allowlist_function("pjsua_create")
//     .allowlist_function("pjsua_destroy")
//     .allowlist_function("pjsua_destroy2")
//     .allowlist_function("pjsua_detect_nat_type")
//     .allowlist_function("pjsua_dump")
//     .allowlist_function("pjsua_enum_accs")
//     .allowlist_function("pjsua_enum_aud_devs")
//     .allowlist_function("pjsua_enum_buddies")
//     .allowlist_function("pjsua_enum_calls")
//     .allowlist_function("pjsua_enum_codecs")
//     .allowlist_function("pjsua_enum_conf_ports")
//     .allowlist_function("pjsua_enum_snd_devs")
//     .allowlist_function("pjsua_enum_transports")
//     .allowlist_function("pjsua_ext_snd_dev_create")
//     .allowlist_function("pjsua_ext_snd_dev_destroy")
//     .allowlist_function("pjsua_ext_snd_dev_get_conf_port")
//     .allowlist_function("pjsua_ext_snd_dev_get_snd_port")
//     .allowlist_function("pjsua_get_buddy_count")
//     .allowlist_function("pjsua_get_ec_stat")
//     .allowlist_function("pjsua_get_ec_tail")
//     .allowlist_function("pjsua_get_nat_type")
//     .allowlist_function("pjsua_get_pjmedia_endpt")
//     .allowlist_function("pjsua_get_pjsip_endpt")
//     .allowlist_function("pjsua_get_pool_factory")
//     .allowlist_function("pjsua_get_snd_dev")
//     .allowlist_function("pjsua_get_state")
//     .allowlist_function("pjsua_get_var")
//     .allowlist_function("pjsua_handle_events")
//     .allowlist_function("pjsua_handle_ip_change")
//     .allowlist_function("pjsua_ice_check_start_trickling")
//     .allowlist_function("pjsua_ice_config_dup")
//     .allowlist_function("pjsua_ice_config_from_media_config")
//     .allowlist_function("pjsua_im_accept_pager")
//     .allowlist_function("pjsua_im_create_accept")
//     .allowlist_function("pjsua_im_init")
//     .allowlist_function("pjsua_im_process_pager")
//     .allowlist_function("pjsua_im_send")
//     .allowlist_function("pjsua_im_typing")
//     .allowlist_function("pjsua_init")
//     .allowlist_function("pjsua_init_tpselector")
//     .allowlist_function("pjsua_ip_change_param_default")
//     .allowlist_function("pjsua_logging_config_default")
//     .allowlist_function("pjsua_logging_config_dup")
//     .allowlist_function("pjsua_media_acc_is_using_stun")
//     .allowlist_function("pjsua_media_apply_xml_control")
//     .allowlist_function("pjsua_media_channel_create_sdp")
//     .allowlist_function("pjsua_media_channel_deinit")
//     .allowlist_function("pjsua_media_channel_init")
//     .allowlist_function("pjsua_media_channel_update")
//     .allowlist_function("pjsua_media_config_default")
//     .allowlist_function("pjsua_media_prov_clean_up")
//     .allowlist_function("pjsua_media_prov_revert")
//     .allowlist_function("pjsua_media_subsys_destroy")
//     .allowlist_function("pjsua_media_subsys_init")
//     .allowlist_function("pjsua_media_subsys_start")
//     .allowlist_function("pjsua_msg_data_clone")
//     .allowlist_function("pjsua_msg_data_init")
//     .allowlist_function("pjsua_parse_media_type")
//     .allowlist_function("pjsua_perror")
//     .allowlist_function("pjsua_player_create")
//     .allowlist_function("pjsua_player_destroy")
//     .allowlist_function("pjsua_player_get_conf_port")
//     .allowlist_function("pjsua_player_get_info")
//     .allowlist_function("pjsua_player_get_port")
//     .allowlist_function("pjsua_player_get_pos")
//     .allowlist_function("pjsua_player_set_pos")
//     .allowlist_function("pjsua_playlist_create")
//     .allowlist_function("pjsua_pool_create")
//     .allowlist_function("pjsua_pres_delete_acc")
//     .allowlist_function("pjsua_pres_dump")
//     .allowlist_function("pjsua_pres_init")
//     .allowlist_function("pjsua_pres_init_acc")
//     .allowlist_function("pjsua_pres_init_publish_acc")
//     .allowlist_function("pjsua_pres_notify")
//     .allowlist_function("pjsua_pres_shutdown")
//     .allowlist_function("pjsua_pres_start")
//     .allowlist_function("pjsua_pres_unpublish")
//     .allowlist_function("pjsua_pres_update_acc")
//     .allowlist_function("pjsua_process_msg_data")
//     .allowlist_function("pjsua_reconfigure_logging")
//     .allowlist_function("pjsua_recorder_create")
//     .allowlist_function("pjsua_recorder_destroy")
//     .allowlist_function("pjsua_recorder_get_conf_port")
//     .allowlist_function("pjsua_recorder_get_port")
//     .allowlist_function("pjsua_resolve_stun_servers")
//     .allowlist_function("pjsua_schedule_timer2_dbg")
//     .allowlist_function("pjsua_schedule_timer_dbg")
//     .allowlist_function("pjsua_set_ec")
//     .allowlist_function("pjsua_set_media_tp_state")
//     .allowlist_function("pjsua_set_msg_route_set")
//     .allowlist_function("pjsua_set_no_snd_dev")
//     .allowlist_function("pjsua_set_null_snd_dev")
//     .allowlist_function("pjsua_set_snd_dev")
//     .allowlist_function("pjsua_set_snd_dev2")
//     .allowlist_function("pjsua_set_state")
//     .allowlist_function("pjsua_sip_acc_is_using_ipv6")
//     .allowlist_function("pjsua_sip_acc_is_using_stun")
//     .allowlist_function("pjsua_snd_dev_param_default")
//     .allowlist_function("pjsua_snd_get_setting")
//     .allowlist_function("pjsua_snd_is_active")
//     .allowlist_function("pjsua_snd_set_setting")
//     .allowlist_function("pjsua_srtp_opt_default")
//     .allowlist_function("pjsua_srtp_opt_dup")
//     .allowlist_function("pjsua_start")
//     .allowlist_function("pjsua_start_mwi")
//     .allowlist_function("pjsua_stop_worker_threads")
//     .allowlist_function("pjsua_tpfactory_register")
//     .allowlist_function("pjsua_transport_close")
//     .allowlist_function("pjsua_transport_config_default")
//     .allowlist_function("pjsua_transport_config_dup")
//     .allowlist_function("pjsua_transport_create")
//     .allowlist_function("pjsua_transport_get_info")
//     .allowlist_function("pjsua_transport_lis_start")
//     .allowlist_function("pjsua_transport_register")
//     .allowlist_function("pjsua_transport_set_enable")
//     .allowlist_function("pjsua_turn_config_dup")
//     .allowlist_function("pjsua_turn_config_from_media_config")
//     .allowlist_function("pjsua_update_stun_servers")
//     .allowlist_function("pjsua_verify_sip_url")
//     .allowlist_function("pjsua_verify_url")
//     .allowlist_function("print_call")
//     .allowlist_function("resolve_stun_server")
//     .allowlist_type("PJ_STUN_RESOLVE_.*")
//     .allowlist_type("pj_stun_resolve_.*")
//     .allowlist_var("PJ_STUN_RESOLVE_.*")
//     .allowlist_var("pj_stun_resolve_.*")

//     .allowlist_type("PJSUA_.*")
//     .allowlist_type("pjsua_.*")
//     .allowlist_var("PJSUA_.*")
//     .allowlist_var("pjsua_.*")

//     .allowlist_recursively(false)
//     .translate_enum_integer_types(true)
//     .layout_tests(false)
//     .prepend_enum_name(false)
//     .layout_tests(false)
//     .generate()
//     // Unwrap the Result and panic on failure.
//     .expect("Unable to generate bindings");

// // Write the bindings to the $OUT_DIR/bindings.rs file.
// let out_path = env::current_dir().unwrap();
// pjsua.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

// }
