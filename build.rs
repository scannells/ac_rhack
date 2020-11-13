extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    // tell rustc to link with libGL. Needed for drawing
    println!("cargo:rustc-link-lib=GL");

    // tell cargo to re-build if the gl_bindings.h file changes
    println!("cargo:rerun-if-changed=src/esp/gl_bindings.h");

    // generate Rust bindings for libGL
    let bindings = bindgen::Builder::default()
        .header("src/esp/gl_bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file. We will include this file later
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("gl_bindings.rs"))
        .expect("Couldn't write bindings!");


    // In the second part of the build process, compile a CPP trampoline
    println!("cargo:rerun-if-changed=src/aimbot/bot_trampoline.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/aimbot/bot_trampoline.cpp")
        .compile("bot_trampoline");
}
