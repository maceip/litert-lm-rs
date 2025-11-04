use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Get the LiteRT-LM root directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let litert_root = PathBuf::from(&manifest_dir).parent().unwrap().to_path_buf();

    // Path to the C header
    let c_header = litert_root.join("c/engine.h");
    println!("cargo:rerun-if-changed={}", c_header.display());

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header(c_header.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Allowlist the items we want to generate bindings for
        .allowlist_function("litert_lm_.*")
        .allowlist_type("LiteRtLm.*")
        .allowlist_type("InputData.*")
        .allowlist_var("kInput.*")
        // Generate comments from C code
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to OUT_DIR (standard Rust way)
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // === SIMPLE LINKING - Just link against libengine.so ===

    // 1. Add library search path
    let bazel_bin = litert_root.join("bazel-bin/c");
    if bazel_bin.exists() {
        println!("cargo:rustc-link-search=native={}", bazel_bin.display());
    } else {
        eprintln!("Warning: bazel-bin/c not found. Run: bazel build //c:engine");
    }

    // Also check LITERT_LM_LIB_PATH environment variable
    if let Ok(lib_path) = env::var("LITERT_LM_LIB_PATH") {
        println!("cargo:rustc-link-search=native={}", lib_path);
    }

    // 2. Link against libengine.so (the C library we built)
    // This single library should contain or link to everything else
    println!("cargo:rustc-link-lib=dylib=engine");

    // 3. Link C++ standard library (required for C++ code)
    // Different names on different platforms
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // That's it! No need to manually link 50+ libraries.
    // The shared library libengine.so should handle its own dependencies.

    println!("cargo:rerun-if-changed=../c/engine.h");
    println!("cargo:rerun-if-changed=../c/engine.cc");
    println!("cargo:rerun-if-env-changed=LITERT_LM_LIB_PATH");
}
