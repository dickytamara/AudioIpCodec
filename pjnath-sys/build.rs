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
        .allowlist_type("PJ_ICE_.*")
        .allowlist_type("pj_ice_.*")
        .allowlist_var("PJ_ICE_.*")
        .allowlist_var("pj_ice_.*")

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
        .allowlist_type("PJ_STUN_.*")
        .allowlist_type("pj_stun_.*")
        .allowlist_var("PJ_STUN_.*")
        .allowlist_var("pj_stun_.*")

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
        .allowlist_type("PJ_TURN_.*")
        .allowlist_type("pj_turn_.*")
        .allowlist_var("PJ_TURN_.*")
        .allowlist_var("pj_turn_.*")



        .allowlist_function("remove_check")
        .allowlist_recursively(false)
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
