extern crate sleef_sys;
use sleef_sys::Sleef_powd2_u10;
use std::arch::x86_64::{_mm_loadu_pd, _mm_storeu_pd, __m128d};

#[test]
fn hello_world() {
    unsafe {
        let a = [2., 10.];
        let b = [3., 20.];

        let va = _mm_loadu_pd(a.as_ptr());
        let vb = _mm_loadu_pd(b.as_ptr());
        let vc = Sleef_powd2_u10(&va as *const __m128d as *mut __m128d as *mut f64,
                                 &vb as *const __m128d as *mut __m128d as *mut f64);

        let mut c = [0_f64; 2];

        _mm_storeu_pd(c.as_mut_ptr(), std::mem::transmute(vc));

        println!("pow({}, {}) = {}", a[0], b[0], c[0]);
        println!("pow({}, {}) = {}", a[1], b[1], c[1]);
    }
}
