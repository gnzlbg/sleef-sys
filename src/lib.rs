//! Bindings for SLEEF: SIMD Library for Evaluating Elementary Functions

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![no_std]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
