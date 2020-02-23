# Rust bindings for SLEEF

[![Travis-CI Status]][travis] [![Appveyor Status]][appveyor] [![Latest Version]][crates.io] [![docs]][docs.rs]

> [SLEEF: SIMD Library for Evaluating Elementary Functions](https://sleef.org/) 

Note: Rust does not support `long double` (80-bit precision floating-point
types) and the SLEEF APIs that use it are not included in these bindings _yet_
(they _could_ probably be added to just handle `long double` as a bag of bytes**.

### Cargo features

* **dft** (default: disabled - equivalent to SLEEF's
  [`BUILD_DFT`](https://sleef.org/compile.xhtml) option): SLEEF also includes
  SIMD-vectorized and parallelized subroutines for discrete Fourier transform
  (DFT) with an API similar to FFTW.
* **all_extensions** (default: disabled): Generate sse and avx instructions even if the corresponding target feature flags are not set. This is useful for crates doing runtime detection of SIMD capabilites.

### Platform support

This wrapper supports the following platforms, but CI is not properly set up for
all of them yet.

| Linux targets:                  | build     | run     |
|---------------------------------|-----------|---------|
| `x86_64-unknown-linux-gnu`      | ✓         | ✓       |
| `aarch64-unknown-linux-gnu`     | ✗         | ✗       |
| `powerpc64le-unknown-linux-gnu` | ✗         | ✗       |
| **MacOSX targets:**             | **build** | **run** |
| `x86_64-apple-darwin`           | ✓         | ✓       |
| **Windows targets:**            | **build** | **run** |
| `x86_64-pc-windows-msvc`        | ✓         | ✓       |

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) or 
 * Boost license ([LICENSE-BOOST](LICENSE-BOOST) or
   http://www.boost.org/LICENSE_1_0.txt)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sleef-sys` by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.

[travis]: https://travis-ci.org/gnzlbg/sleef-sys
[Travis-CI Status]: https://travis-ci.org/gnzlbg/sleef-sys.svg?branch=master
[appveyor]: https://ci.appveyor.com/project/gnzlbg/sleef-sys
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/hd7v9dvr442hgdix?svg=true
[Latest Version]: https://img.shields.io/crates/v/sleef-sys.svg
[crates.io]: https://crates.io/crates/sleef-sys
[docs]: https://docs.rs/sleef-r/badge.svg
[docs.rs]: https://docs.rs/sleef-sys/
[master_docs]: https://gnzlbg.github.io/sleef-sys/sleef-sys/

