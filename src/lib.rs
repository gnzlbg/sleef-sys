//! Bindings for SLEEF: SIMD Library for Evaluating Elementary Functions
#![feature(simd_ffi)]
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
        mod x86 {
            #[cfg(target_arch = "x86")]
            pub use core::arch::x86::*;

            #[cfg(target_arch = "x86_64")]
            pub use core::arch::x86_64::*;

        }
        pub use self::x86::{__m128d};
    }
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
