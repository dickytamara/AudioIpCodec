
#![allow(deprecated)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;



fn main () {

    println!("cargo:rustc-link-lib=pjsua");
    println!("cargo:rustc-link-search=native=/usr/lib");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let pjsua = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // -lpjsua2 -lstdc++ 
        // -lpjsua -lpjsip-ua -lpjsip-simple -lpjsip 
        // -lpjmedia-codec -lpjmedia -lpjmedia-videodev 
        // -lpjmedia-audiodev -lpjmedia -lpjnath -lpjlib-util -lpj
        // -lsrtp -lresample -lilbccodec -lg7221codec  -lopus 
        // -lssl -lcrypto -lm -lrt -lpthread  -lasound 
        // -lopencore-amrnb -lopencore-amrwb -lvo-amrwbenc
        .header("wrapper.h")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/usr/lib/llvm-11/lib/clang/11.0.1/include")
        .clang_arg("-I/usr/include/x86_64-linux-gnu")
        .clang_arg("-I/usr/include")
        .clang_arg("-lstdc++")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelisted_type("PJ.*")
        .whitelisted_type("pj.*")
        .whitelisted_function("PJ.*")
        .whitelisted_function("pj.*")
        .whitelisted_var("PJ.*")
        .whitelisted_var("pj.*")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    pjsua.write_to_file(out_path.join("pjsua.rs")).expect("Error write pjsua.rs");

}
