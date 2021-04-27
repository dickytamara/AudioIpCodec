extern crate bindgen;
use std::env;

fn main () {

    println!("cargo:rustc-link-lib=pjmedia_audiodev");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let pjmedia_audiodev = bindgen::Builder::default().header("wrapper.h")

        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("extern crate pj_sys;")
        .raw_line("extern crate pjmedia_sys;")
        .raw_line("use pj_sys::*;")
        .raw_line("use pjmedia_sys::*;")

        .allowlist_function("pjmedia_alsa_factory")
        .allowlist_function("pjmedia_audiodev_strerror")
        .allowlist_function("pjmedia_aud_register_factory")
        .allowlist_function("pjmedia_aud_subsys_get_pool_factory")
        .allowlist_function("pjmedia_aud_subsys_init")
        .allowlist_function("pjmedia_aud_subsys_shutdown")
        .allowlist_function("pjmedia_aud_test")
        .allowlist_function("pjmedia_aud_unregister_factory")
        
        .allowlist_type("pjmedia_aud_test_stat")
        .allowlist_type("pjmedia_aud_test_results")

        .allowlist_recursively(false)
        .translate_enum_integer_types(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = env::current_dir().unwrap();
    pjmedia_audiodev.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

}
