#![allow(unused)]

extern crate sleef_sys;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    use super::*;

    #[test]
    fn v128() {
        use sleef_sys::Sleef_powd2_u10;
        unsafe {
            let a = [2., 10.];
            let b = [3., 20.];

            let vc = Sleef_powd2_u10(std::mem::transmute(a), std::mem::transmute(b));

            let mut c: [f64; 2] = std::mem::transmute(vc);

            assert_eq!(c[0], 2_f64.powf(3.));
            assert_eq!(c[1], 10_f64.powf(20.));
        }
    }

    #[test]
    #[cfg(target_feature = "avx")] // UB otherwise: https://github.com/rust-lang/rust/issues/53346
    fn v256() {
        use sleef_sys::Sleef_powd4_u10;
        unsafe {
            let a = [2., 10., 3., 4.];
            let b = [3., 20., 3., 5.];

            let vc = Sleef_powd4_u10(std::mem::transmute(a), std::mem::transmute(b));

            let mut c: [f64; 4] = std::mem::transmute(vc);

            assert_eq!(c[0], 2_f64.powf(3.));
            assert_eq!(c[1], 10_f64.powf(20.));
            assert_eq!(c[2], 3_f64.powf(3.));
            assert_eq!(c[3], 4_f64.powf(5.));
        }
    }
}
