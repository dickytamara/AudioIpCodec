#![allow(deprecated)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;



fn main () {

    println!("cargo:rustc-link-lib=pjmedia_codec");
    println!("cargo:rustc-link-search=native=/usr/lib");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let pjmedia_codec = bindgen::Builder::default()
        .header("wrapper.h")
        // .clang_arg("-I/usr/local/include")
        // .clang_arg("-I/usr/lib/llvm-11/lib/clang/11.0.1/include")
        // .clang_arg("-I/usr/include/x86_64-linux-gnu")
        // .clang_arg("-I/usr/include")
        // .clang_arg("-lstdc++")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")

        .allowlist_function("pjmedia_audio_codec_config_default")
        .allowlist_function("pjmedia_codec_amr_match_sdp")
        .allowlist_function("pjmedia_codec_g7221_match_sdp")
        .allowlist_function("pjmedia_codec_opus_deinit")
        .allowlist_function("pjmedia_codec_opus_get_config")
        .allowlist_function("pjmedia_codec_opus_init")
        .allowlist_function("pjmedia_codec_opus_set_default_param")
        .allowlist_function("pjmedia_codec_register_audio_codecs")

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
    // let out_path = PathBuf::from(env::var("SRC_DIR").unwrap());
    let out_path = env::current_dir().unwrap();
    pjmedia_codec.write_to_file(out_path.join("src/lib.rs")).expect("Error write lib.rs");

}
