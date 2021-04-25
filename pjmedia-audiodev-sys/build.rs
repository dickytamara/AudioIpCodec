#![allow(deprecated)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;



fn main () {

    println!("cargo:rustc-link-lib=pjmedia_audiodev");
    println!("cargo:rustc-link-search=native=/usr/lib");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let pjmedia_audiodev = bindgen::Builder::default()
        .header("wrapper.h")
        // .clang_arg("-I/usr/local/include")
        // .clang_arg("-I/usr/lib/llvm-11/lib/clang/11.0.1/include")
        // .clang_arg("-I/usr/include/x86_64-linux-gnu")
        // .clang_arg("-I/usr/include")
        // .clang_arg("-lstdc++")

        .allowlist_function("pjmedia_alsa_factory")
        .allowlist_function("pjmedia_audiodev_strerror")
        .allowlist_function("pjmedia_aud_register_factory")
        .allowlist_function("pjmedia_aud_subsys_get_pool_factory")
        .allowlist_function("pjmedia_aud_subsys_init")
        .allowlist_function("pjmedia_aud_subsys_shutdown")
        .allowlist_function("pjmedia_aud_test")
        .allowlist_function("pjmedia_aud_unregister_factory")

        .allowlist_recursively(true)
        .translate_enum_integer_types(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    pjmedia_audiodev.write_to_file(out_path.join("pjmedia_audiodev.rs")).expect("Error write pjmedia_audiodev.rs");

}
