# Rust Portable CRC Calculation Macros

[![crates.io](https://img.shields.io/crates/v/embedded-crc-macros.svg)](https://crates.io/crates/embedded-crc-macros)
[![Docs](https://docs.rs/embedded-crc-macros/badge.svg)](https://docs.rs/embedded-crc-macros)
[![Build Status](https://travis-ci.com/eldruin/embedded-crc-macros-rs.svg?branch=master)](https://travis-ci.com/eldruin/embedded-crc-macros-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/embedded-crc-macros-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/embedded-crc-macros-rs?branch=master)

This crate provides macros that define portable [CRC-8][crc8] algorithm implementations
with the parameters directly provided at compile time and without any dependencies.
Intended for use in `no_std`.

## How this crate compares to others
There is a number of crates implementing CRC algorithms but their intention is to
be configurable, generic, use acceleration via SIMD instructions, etc.

This crate provides macros that define portable and non-configurable CRC-8 algorithm
implementations with the parameters provided at compile time (optionally using
a pre-calculated lookup table) and without any dependencies.

This should allow the compiler to make good optimizations and allows for use of the
algorithm in any target architecture with minimal code bloat.

Furthermore, this crate provides macros to generate the lookup tables on build time.

This makes this crate specially well suited for use in `no_std` environments.

## Usage

```rust
use embedded_crc_macros::crc8;

crc8!(pec, 7 /* x^8+x^2+x+1 */, 0, "SMBus Packet Error Code");

const ADDRESS: u8 = 0x5A;
const REGISTER: u8 = 0x06;

fn main() {
    let crc = pec(&[ADDRESS << 1, REGISTER, 0xAB, 0xCD]);
    println!("PEC: {}", crc); // prints 95
}
```

## Support

For questions, issues, feature requests, other changes, or just feedback, please file an
[issue in the github project](https://github.com/eldruin/embedded-crc-macros-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[crc8]: https://en.wikipedia.org/wiki/CRC-8
