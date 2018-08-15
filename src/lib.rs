//! Raw bindings for SLEEF: SIMD Library for Evaluating Elementary Functions .
//!
//! WARNING: only the SLEEF APIs that are supported by the set of target
//! features enabled while compiling this crate are exposed. Trying to call a
//! SLEEF function using a larger register type than the one supported by the
//! enabled target features is **undefined behavior** (see
//! https://github.com/rust-lang/rust/issues/53346).

#![feature(simd_ffi, stdsimd)]
#![allow(
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![no_std]

#[macro_use]
extern crate cfg_if;
extern crate libc;

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        #[cfg(target_arch = "x86")]
        use core::arch::x86;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64 as x86;

        pub use self::x86::{
            // MMX:
            __m64,
            // SSE:
            __m128, __m128i, __m128d,
            // AVX:
            __m256, __m256i, __m256d,
            // FIXME: AVX-512:
            //__m512, __m512i, __m512d,
        };
    } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        pub use core::arch::aarch64::{
            int8x8_t, uint8x8_t, poly8x8_t, int16x4_t, uint16x4_t,
            poly16x4_t, int32x2_t, uint32x2_t, float32x2_t, int64x1_t, uint64x1_t,
            float64x1_t, int8x16_t, uint8x16_t, poly8x16_t, int16x8_t,
            uint16x8_t, poly16x8_t, int32x4_t, uint32x4_t, float32x4_t, int64x2_t,
            uint64x2_t, float64x2_t
        };
    } else if #[cfg(all(target_arch = "powerpc64", target_feature = "vsx"))] {
        pub use core::arch::powerpc64::{
            vector_signed_char, vector_unsigned_char,
            vector_signed_short, vector_unsigned_short, vector_signed_int,
            vector_unsigned_int,  vector_float, vector_signed_long,
            vector_unsigned_long, vector_double, vector_bool_char,
            vector_bool_short, vector_bool_int, vector_bool_long
        };
    }
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
