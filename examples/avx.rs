fn main(){
    #[cfg(feature = "all_extensions")]
    {
        use std::arch::x86_64::_mm256_set1_pd;
        use std::mem::transmute;
        use std::f64::consts::FRAC_PI_2;

        if is_x86_feature_detected!("avx"){
            unsafe {
                let results = sleef_sys::Sleef_sind4_u35avx(
                    _mm256_set1_pd(FRAC_PI_2)
                );
                let results_arr: [f64; 4] = transmute(results);

                for r in results_arr.iter(){
                    assert!((1.0 - r).abs() < 0.000000000001);
                }
            }

            println!("Success");

            return;
        }
    }

    println!("Failure");
}
