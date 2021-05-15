// extern crate bindgen;
// use std::env;


fn main () {
    // dynamic
    println!("cargo:rustc-link-lib=pjsip");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    // dynamic
    // println!("cargo:rustc-link-lib=static=pjsip-ua-x86_64-unknown-linux-gnu");
    // println!("cargo:rustc-link-search=native=/usr/local/lib");
    // println!("cargo:rerun-if-changed=wrapper.h");

    // generate lib.rs
    // crate_bindgen();
}

// fn create_bindgen() {
//     let pjsip_ua = bindgen::Builder::default().header("wrapper.h")

//     .raw_line("#![allow(non_upper_case_globals)]")
//     .raw_line("#![allow(non_camel_case_types)]")
//     .raw_line("#![allow(non_snake_case)]")
//     .raw_line("extern crate pj_sys;")
//     .raw_line("extern crate pjsip_sys;")
//     .raw_line("extern crate pjsip_simple_sys;")
//     .raw_line("extern crate pjmedia_sys;")
//     .raw_line("use pj_sys::*;")
//     .raw_line("use pjsip_sys::*;")
//     .raw_line("use pjsip_simple_sys::*;")
//     .raw_line("use pjmedia_sys::*;")
//     // .parse_callbacks(Box::new(bindgen::CargoCallbacks))

//     .allowlist_function("pjsip_100rel_attach")
//     .allowlist_function("pjsip_100rel_create_prack")
//     .allowlist_function("pjsip_100rel_end_session")
//     .allowlist_function("pjsip_100rel_init_module")
//     .allowlist_function("pjsip_100rel_is_reliable")
//     .allowlist_function("pjsip_100rel_on_rx_prack")
//     .allowlist_function("pjsip_100rel_send_prack")
//     .allowlist_function("pjsip_100rel_tx_response")
//     .allowlist_function("pjsip_create_sdp_body")
//     .allowlist_function("pjsip_dlg_get_inv_session")
//     .allowlist_function("pjsip_get_prack_method")
//     .allowlist_function("pjsip_get_refer_method")

//     .allowlist_function("pjsip_inv_add_ref")
//     .allowlist_function("pjsip_inv_answer")
//     .allowlist_function("pjsip_inv_cancel_reinvite")
//     .allowlist_function("pjsip_inv_create_ack")
//     .allowlist_function("pjsip_inv_create_uac")
//     .allowlist_function("pjsip_inv_create_uas")
//     .allowlist_function("pjsip_inv_dec_ref")
//     .allowlist_function("pjsip_inv_end_session")
//     .allowlist_function("pjsip_inv_initial_answer")
//     .allowlist_function("pjsip_inv_invite")
//     .allowlist_function("pjsip_inv_process_redirect")
//     .allowlist_function("pjsip_inv_reinvite")
//     .allowlist_function("pjsip_inv_send_msg")
//     .allowlist_function("pjsip_inv_set_local_sdp")
//     .allowlist_function("pjsip_inv_set_sdp_answer")
//     .allowlist_function("pjsip_inv_state_name")
//     .allowlist_function("pjsip_inv_terminate")
//     .allowlist_function("pjsip_inv_uac_restart")
//     .allowlist_function("pjsip_inv_update")
//     .allowlist_function("pjsip_inv_usage_init")
//     .allowlist_function("pjsip_inv_usage_instance")
//     .allowlist_function("pjsip_inv_verify_request")
//     .allowlist_function("pjsip_inv_verify_request2")
//     .allowlist_function("pjsip_inv_verify_request3")
//     .allowlist_type("PJSIP_INV_.*")
//     .allowlist_type("pjsip_inv_.*")
//     .allowlist_var("PJSIP_INV_.*")
//     .allowlist_var("pjsip_inv_.*")

//     .allowlist_function("pjsip_min_se_hdr_create")
//     .allowlist_type("PJSIP_MIN_.*")
//     .allowlist_type("pjsip_min_.*")
//     .allowlist_var("PJSIP_MIN_.*")
//     .allowlist_var("pjsip_min_.*")

//     .allowlist_function("pjsip_rdata_get_sdp_info")
//     .allowlist_function("pjsip_rdata_get_sdp_info2")
//     .allowlist_type("PJSIP_RDATA_.*")
//     .allowlist_type("pjsip_rdata_.*")
//     .allowlist_var("PJSIP_RDATA_.*")
//     .allowlist_var("pjsip_rdata_.*")

//     .allowlist_function("pjsip_regc_add_headers")
//     .allowlist_function("pjsip_regc_add_ref")
//     .allowlist_function("pjsip_regc_create")
//     .allowlist_function("pjsip_regc_dec_ref")
//     .allowlist_function("pjsip_regc_destroy")
//     .allowlist_function("pjsip_regc_get_info")
//     .allowlist_function("pjsip_regc_get_pool")
//     .allowlist_function("pjsip_regc_init")
//     .allowlist_function("pjsip_regc_register")
//     .allowlist_function("pjsip_regc_release_transport")
//     .allowlist_function("pjsip_regc_send")
//     .allowlist_function("pjsip_regc_set_credentials")
//     .allowlist_function("pjsip_regc_set_delay_before_refresh")
//     .allowlist_function("pjsip_regc_set_prefs")
//     .allowlist_function("pjsip_regc_set_reg_tsx_cb")
//     .allowlist_function("pjsip_regc_set_route_set")
//     .allowlist_function("pjsip_regc_set_transport")
//     .allowlist_function("pjsip_regc_set_via_sent_by")
//     .allowlist_function("pjsip_regc_unregister")
//     .allowlist_function("pjsip_regc_unregister_all")
//     .allowlist_function("pjsip_regc_update_contact")
//     .allowlist_function("pjsip_regc_update_expires")
//     .allowlist_type("PJSIP_REGC.*")
//     .allowlist_type("pjsip_regc.*")
//     .allowlist_var("PJSIP_REGC.*")
//     .allowlist_var("pjsip_regc.*")

//     .allowlist_function("pjsip_replaces_hdr_create")
//     .allowlist_function("pjsip_replaces_init_module")
//     .allowlist_function("pjsip_replaces_verify_request")
//     .allowlist_type("PJSIP_REPLACES_.*")
//     .allowlist_type("pjsip_replaces_.*")
//     .allowlist_var("PJSIP_REPLACES_.*")
//     .allowlist_var("pjsip_replaces_.*")

//     .allowlist_function("pjsip_sess_expires_hdr_create")
//     .allowlist_type("PJSIP_SESS_.*")
//     .allowlist_type("pjsip_sess_.*")
//     .allowlist_var("PJSIP_SESS_.*")
//     .allowlist_var("pjsip_sess_.*")

//     .allowlist_function("pjsip_timer_end_session")
//     .allowlist_function("pjsip_timer_handle_refresh_error")
//     .allowlist_function("pjsip_timer_init_module")
//     .allowlist_function("pjsip_timer_init_session")
//     .allowlist_function("pjsip_timer_process_req")
//     .allowlist_function("pjsip_timer_process_resp")
//     .allowlist_function("pjsip_timer_setting_default")
//     .allowlist_function("pjsip_timer_update_req")
//     .allowlist_function("pjsip_timer_update_resp")
//     .allowlist_type("PJSIP_TIMER.*")
//     .allowlist_type("pjsip_timer.*")
//     .allowlist_var("PJSIP_TIMER.*")
//     .allowlist_var("pjsip_timer.*")

//     .allowlist_function("pjsip_xfer_accept")
//     .allowlist_function("pjsip_xfer_create_uac")
//     .allowlist_function("pjsip_xfer_create_uas")
//     .allowlist_function("pjsip_xfer_current_notify")
//     .allowlist_function("pjsip_xfer_initiate")
//     .allowlist_function("pjsip_xfer_init_module")
//     .allowlist_function("pjsip_xfer_notify")
//     .allowlist_function("pjsip_xfer_send_request")

//     .allowlist_var("pjsip_prack_method")
//     .allowlist_var("pjsip_refer_method")
//     .allowlist_var("pjsip_update_method")


//     // .whitelisted_type("PJSIP_.*")
//     // .whitelisted_type("pjsip_.*")
//     // .whitelisted_function("PJSIP_.*")
//     // .whitelisted_function("pjsip_.*")
//     // .whitelisted_var("PJSIP_.*")
//     // .whitelisted_var("pjsip_.*")



//     .allowlist_recursively(false)
//     .translate_enum_integer_types(true)
//     .layout_tests(false)
//     .prepend_enum_name(false)
//     .layout_tests(false)
//     .generate()
//     // Unwrap the Result and panic on failure.
//     .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = env::current_dir().unwrap();
//     pjsip_ua.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

// }
