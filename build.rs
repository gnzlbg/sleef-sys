//! Builds the sleef library from source.
extern crate cmake;
extern crate bindgen;
extern crate env_logger;

use std::{path::PathBuf, env};

fn main() {
    env_logger::init();
    let target = env::var("TARGET").expect("TARGET was not set");

    // Parse target features, this is required for ABI compatibility.
    let mut features = std::collections::HashSet::<String>::default();
    if let Ok(rustflags) = env::var("CARGO_CFG_TARGET_FEATURE") {
        for v in rustflags.split(',') {
            features.insert(v.to_string());
        }
    }

    let dst = cmake::Config::new("sleef")
        .very_verbose(true)
        // no DFT libraries (should be behind a feature flag):
        .define("BUILD_DFT", "FALSE")
        // no tests (should build and run the tests behind a feature flag):
        .define("BUILD_TESTS", "FALSE")
        .define("BUILD_SHARED_LIBS", "TRUE")
        .build();

    println!("cargo:rustc-link-lib=sleef");
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR was not set"));
    let sleef_header = out_dir.join("include").join("sleef.h");
    assert!(sleef_header.exists(),
            "error sleef.h header not found in OUT_DIR={}",
            out_dir.display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
        .header(sleef_header.to_str().expect("failed to convert header path to string"))
    // Rust does not support 80-bit precision floats:
        .opaque_type("Sleef_longdouble2")
    // The bindings should be no_std:
        .use_core()
    // The bindings should use the ctypes from libc, not std::os::raw:
        .ctypes_prefix("libc")
    // Generate inline functions:
        .generate_inline_functions(true)
    // Only target nightly Rust for the time being:
        .rust_target(bindgen::RustTarget::Nightly);

    // Blacklist vector types:
    if target.contains("86") && (features.contains("sse") || features.contains("avx")) {
        // x86 targets: i386,i586,i686,x86,x86_64
        let vs = [
            // MMX:
            "__m64",
            // SSE:
            "__m128", "__m128i", "__m128d",
            // AVX
            "__m256", "__m256i", "__m256d",
            // FIXME: AVX-512
            //"__m512", "__m512i", "__m512d",
        ];

        for v in &vs {
            bindings = bindings.blacklist_type(v).opaque_type(v);
        }

        let x86_features = {
            let mut features = std::collections::HashMap::<String, String>::default();
            features.insert("sse2".to_string(), "__SSE2__".to_string());
            features.insert("avx".to_string(), "__AVX__".to_string());
            // FIXME: AVX-512
            // features.insert("avx512f".to_string(), "__AVX512F__".to_string());
            features
        };
        let use_all_extensions = env::var("CARGO_FEATURE_ALL_EXTENSIONS").is_ok();

        if use_all_extensions {
            for def in x86_features.values() {
                bindings = bindings.clang_arg(format!("-D{}", def));
            }
        } else {
            for f in &features {
                if let Some(def) = x86_features.get(f) {
                    bindings = bindings.clang_arg(format!("-D{}", def));
                }
            }
        }
    } else if target.contains("aarch") && features.contains("neon") {
        let vs = [
            "int8x8_t", "uint8x8_t", "poly8x8_t", "int16x4_t", "uint16x4_t",
            "poly16x4_t", "int32x2_t", "uint32x2_t", "float32x2_t", "int64x1_t", "uint64x1_t",
            "float64x1_t", "int8x16_t", "uint8x16_t", "poly8x16_t", "int16x8_t",
            "uint16x8_t", "poly16x8_t", "int32x4_t", "uint32x4_t", "float32x4_t", "int64x2_t",
            "uint64x2_t", "float64x2_t"
        ];
        for v in &vs {
            bindings = bindings.blacklist_type(v).opaque_type(v);
        }

    } else if target.contains("powerpc64") && features.contains("vsx") {
        let vs = [
            "vector_signed_char", "vector_unsigned_char",
            "vector_signed_short", "vector_unsigned_short", "vector_signed_int",
            "vector_unsigned_int",  "vector_float", "vector_signed_long",
            "vector_unsigned_long", "vector_double", "vector_bool_char",
            "vector_bool_short", "vector_bool_int", "vector_bool_long"
        ];
        for v in &vs {
            bindings = bindings.blacklist_type(v).opaque_type(v);
        }
    } else {
        eprintln!("unsupported target: \"{}\" features: \"{:?}\"", target, features);
        std::process::abort();
    }

    let bindings = bindings
    // Finish the builder and generate the bindings.
        .generate()
    // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=sleef");
}
