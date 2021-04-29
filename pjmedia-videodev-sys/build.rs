extern crate bindgen;
use std::env;

fn main () {

    println!("cargo:rustc-link-lib=pjmedia-videodev");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let pjmedia_videodev = bindgen::Builder::default().header("wrapper.h")

        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("extern crate pj_sys;")
        .raw_line("extern crate pjmedia_sys;")
        .raw_line("use pj_sys::*;")
        .raw_line("use pjmedia_sys::*;")


        .allowlist_function("pjmedia_get_vid_subsys")
        .allowlist_function("pjmedia_vid_driver_init")
        .allowlist_function("pjmedia_vid_driver_deinit")
        .allowlist_function("pjmedia_vid_dev_switch_param_default")
        .allowlist_function("pjmedia_vid_dev_cap_name")
        .allowlist_function("pjmedia_vid_dev_param_set_cap")
        .allowlist_function("pjmedia_vid_dev_param_get_cap")
        .allowlist_function("pjmedia_vid_dev_refresh")
        .allowlist_function("pjmedia_vid_dev_count")
        .allowlist_function("pjmedia_vid_dev_get_info")
        .allowlist_function("pjmedia_vid_dev_lookup")
        .allowlist_function("pjmedia_vid_dev_default_param")
        .allowlist_function("pjmedia_vid_dev_stream_create")
        .allowlist_function("pjmedia_vid_dev_stream_get_param")
        .allowlist_function("pjmedia_vid_dev_stream_get_cap")
        .allowlist_function("pjmedia_vid_dev_stream_set_cap")
        .allowlist_function("pjmedia_vid_dev_stream_start")
        .allowlist_function("pjmedia_vid_dev_stream_is_running")
        .allowlist_function("pjmedia_vid_dev_stream_get_frame")
        .allowlist_function("pjmedia_vid_dev_stream_put_frame")
        .allowlist_function("pjmedia_vid_dev_stream_stop")
        .allowlist_function("pjmedia_vid_dev_stream_destroy")
        .allowlist_function("pjmedia_vid_dev_subsys_init")
        .allowlist_function("pjmedia_vid_dev_subsys_get_pool_factory")
        .allowlist_function("pjmedia_vid_dev_subsys_shutdown")
        .allowlist_function("pjmedia_vid_register_factory")
        .allowlist_function("pjmedia_vid_unregister_factory")

        .allowlist_function("pjmedia_vid.*")

        .allowlist_recursively(false)
        .translate_enum_integer_types(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = env::current_dir().unwrap();
    pjmedia_videodev.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

}
