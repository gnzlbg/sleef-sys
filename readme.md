# Rust bindings for SLEEF: SIMD Library for Evaluating Elementary Functions

[![Travis-CI Status]][travis] [![Appveyor Status]][appveyor] [![Latest Version]][crates.io] [![docs]][docs.rs]

> https://sleef.org/

Note: Rust does not support `long double` (80-bit precision floating-point) and
the SLEEF APIs that use it are not included in these bindings _yet_ (they
_could_ probably be added to just handle `long double` as a bag of bytes).

### Cargo features

> None

## Platform support

TBD.

### License

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
for inclusion in `sleef-rs` by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.

[travis]: https://travis-ci.org/gnzlbg/sleef-rs
[Travis-CI Status]: https://travis-ci.org/gnzlbg/sleef-rs.svg?branch=master
[appveyor]: https://ci.appveyor.com/project/gnzlbg/sleef-rs
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/hd7v9dvr442hgdix?svg=true
[Latest Version]: https://img.shields.io/crates/v/sleef-rs.svg
[crates.io]: https://crates.io/crates/sleef-rs
[docs]: https://docs.rs/sleef-r/badge.svg
[docs.rs]: https://docs.rs/sleef-rs/
[master_docs]: https://gnzlbg.github.io/sleef-rs/sleef-rs/

