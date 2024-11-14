// build.rs

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Use pkg-config to find libwebp and libwebpdemux
    let webp_lib =
        pkg_config::probe_library("libwebp").expect("libwebp not found. Ensure it is installed.");
    let webpdemux_lib = pkg_config::probe_library("libwebpdemux")
        .expect("libwebpdemux not found. Ensure it is installed.");

    // Invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Initialize bindgen builder
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h") // Path to your wrapper header
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .allowlist_function("WebP.*")
        .allowlist_type("WebP.*")
        .allowlist_var("WEBP.*")
        .allowlist_var("WebP.*");

    // Add include paths from pkg-config to bindgen
    for include_path in webp_lib
        .include_paths
        .iter()
        .chain(webpdemux_lib.include_paths.iter())
    {
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
    }

    // Generate the bindings
    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Ensure both libraries are linked
    for lib in webp_lib.libs.iter().chain(webpdemux_lib.libs.iter()) {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Add any library paths
    for lib_path in webp_lib
        .link_paths
        .iter()
        .chain(webpdemux_lib.link_paths.iter())
    {
        println!("cargo:rustc-link-search={}", lib_path.display());
    }
}
