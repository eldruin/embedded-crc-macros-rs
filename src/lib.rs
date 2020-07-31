//! This crate provides macros that define portable [CRC-8][crc8] algorithm implementations
//! with the parameters directly provided at compile time and without any dependencies.
//! Intended for use in `no_std`.
//!
//! [crc8]: https://en.wikipedia.org/wiki/CRC-8
//!
//! ## How this crate compares to others
//!
//! There is a number of crates implementing CRC algorithms but their intention is to
//! be configurable, generic, use acceleration via SIMD instructions, etc.
//!
//! This crate provides macros that define portable and non-configurable CRC-8 algorithm
//! implementations with the parameters provided at compile time (optionally using
//! a pre-calculated lookup table) and without any dependencies.
//!
//! This should allow the compiler to make good optimizations and allows for use of the
//! algorithm in any target architecture with minimal code bloat.
//!
//! Furthermore, this crate provides macros to generate the lookup tables on build time.
//!
//! This makes this crate specially well suited for use in `no_std` environments.
//!
//! # Usage
//!
//! The examples implement the System Management Bus (SMBus) Packet Error Code
//! calculation algorithm, also called CRC-8-ATM HEC.
//!
//! This algorithm uses the polynomial `x^8 + x^2 + x + 1` which corresponds to
//! the value `7` with an initial value of 0.
//!
//! See [here][smbus-pec] for more information.
//!
//! [smbus-pec]: https://en.wikipedia.org/wiki/System_Management_Bus#Packet_Error_Checking
//!
//! ## Define a function computing the SMBus PEC algorithm
//!
//! ```rust
//! use embedded_crc_macros::crc8;
//!
//! // 7 corresponds to the polynomial x^8 + x^2 + x + 1
//! crc8!(smbus_pec, 7, 0, "SMBus Packet Error Code");
//!
//! const ADDR: u8 = 0x5A;
//! let command = 0x06;
//! let value = 0xAB;
//!
//! let checksum = smbus_pec(&[ADDR << 1, command, value]);
//! println!("PEC: {}", checksum);
//! ```
//!
//! ## Define a function computing the SMBus PEC algorithm using a pre-calculated lookup table
//!
//! A lookup table must be defined in the same environment as `LOOKUP_TABLE`.
//! ```rust
//! use embedded_crc_macros::crc8_lookup_table;
//!
//! crc8_lookup_table!(smbus_pec, 0, "SMBus Packet Error Code");
//!
//! const ADDR: u8 = 0x5A;
//! let command = 0x06;
//! let value = 0xAB;
//!
//! let checksum = smbus_pec(&[ADDR << 1, command, value]);
//! println!("PEC: {}", checksum);
//!
//! # // This can be generated on build time with the
//! # // `build_rs_lookup_table_file_generation` macro.
//! # const LOOKUP_TABLE: [u8; 256] = [0; 256];
//! ```
//!
//! ## Define structure implementing the SMBus PEC algorithm as a `core::hash::Hasher`
//!
//! ```rust
//! use core::hash::Hasher;
//! use embedded_crc_macros::crc8_hasher;
//!
//! crc8_hasher!(SmbusPec, 7 /* x^8+x^2+x+1 */, 0, "SMBus Packet Error Code");
//!
//! let mut hasher = SmbusPec::new();
//! hasher.write(&[0xAB, 0xCD]);
//! let pec = hasher.finish();
//!
//! println!("PEC: {}", pec);
//! ```
//!
//! ## `core::hash::Hasher` implementation using a pre-calculated lookup table.
//!
//! A lookup table must be defined in the same environment as `LOOKUP_TABLE`.
//! This can be generated in the `build.rs` file with the
//! [build_rs_lookup_table_file_generation](macro.build_rs_lookup_table_file_generation.html)
//! macro.
//! ```rust
//! use core::hash::Hasher;
//! use embedded_crc_macros::crc8_hasher_lookup_table;
//!
//! // include!(concat!(env!("OUT_DIR"), "/lookup_table.rs"));
//! crc8_hasher_lookup_table!(SmbusPec, 0, "SMBus Packet Error Code");
//!
//! let mut hasher = SmbusPec::new();
//! hasher.write(&[0xAB, 0xCD]);
//! let pec = hasher.finish();
//!
//! println!("PEC: {}", pec);
//! #
//! # // This can be generated on build time with the
//! # // `build_rs_lookup_table_file_generation` macro.
//! # const LOOKUP_TABLE: [u8; 256] = [0; 256];
//! ```

#![doc(html_root_url = "https://docs.rs/embedded-crc-macros/0.1.0")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

/// Define public function implementing the CRC-8 algorithm for the given polynomial and initial value.
///
/// A function name and some documentation for it must be provided. For example:
/// ```rust
/// use embedded_crc_macros::crc8;
/// crc8!(smbus_pec, 7 /* x^8+x^2+x+1 */, 0, "SMBus Packet Error Code");
/// ```
#[macro_export]
macro_rules! crc8 {
    ($function_name:ident, $poly:expr, $initial_value:expr, $doc:expr) => {
        #[doc=$doc]
        pub fn $function_name(data: &[u8]) -> u8 {
            let mut crc = $initial_value;
            for byte in data {
                crc ^= byte;
                for _ in 0..8 {
                    crc = if (crc & (1 << 7)) != 0 {
                        (crc << 1) ^ $poly
                    } else {
                        crc << 1
                    };
                }
            }
            crc
        }
    };
}

/// Define public function implementing the CRC-8 algorithm for the given polynomial
/// and initial value using a lookup table.
///
/// This implementation is much faster at the cost of some space.
/// A function name and some documentation for it must be provided.
///
/// The lookup table can be generated in the `build.rs` file with the
/// [build_rs_lookup_table_file_generation](macro.build_rs_lookup_table_file_generation.html)
/// macro and then included like in the following example.
/// ```rust
/// use embedded_crc_macros::crc8_lookup_table;
/// // include!(concat!(env!("OUT_DIR"), "/lookup_table.rs"));
/// crc8_lookup_table!(pec, 0, "SMBus Packet Error Code");
///
/// # // This can be generated on build time with the
/// # // `build_rs_lookup_table_file_generation` macro.
/// # const LOOKUP_TABLE: [u8; 256] = [0; 256];
/// ```
#[macro_export]
macro_rules! crc8_lookup_table {
    ($function_name:ident, $initial_value:expr, $doc:expr) => {
        #[doc=$doc]
        pub fn $function_name(data: &[u8]) -> u8 {
            let mut crc = $initial_value;
            for byte in data {
                crc = LOOKUP_TABLE[(crc ^ *byte) as usize];
            }
            crc
        }
    };
}

/// Define structure implementing the CRC-8 algorithm for the given polynomial and initial value
/// as a `core::hash::Hasher` trait implementation.
///
/// A struct name and some documentation for it must be provided. For example:
/// ```rust
/// use core::hash::Hasher;
/// use embedded_crc_macros::crc8_hasher;
///
/// crc8_hasher!(SmbusPec, 7 /* x^8+x^2+x+1 */, 0, "SMBus Packet Error Code");
///
/// let mut hasher = SmbusPec::new();
/// hasher.write(&[0xAB, 0xCD]);
/// let pec = hasher.finish();
///
/// println!("PEC: {}", pec);
/// ```
#[macro_export]
macro_rules! crc8_hasher {
    ($struct_name:ident, $poly:expr, $initial_value:expr, $doc:expr) => {
        #[doc=$doc]
        struct $struct_name {
            crc: u8,
        }

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name {
                    crc: $initial_value,
                }
            }
        }

        impl core::hash::Hasher for $struct_name {
            #[inline]
            fn write(&mut self, bytes: &[u8]) {
                for byte in bytes {
                    self.crc ^= byte;
                    for _ in 0..8 {
                        self.crc = if (self.crc & (1 << 7)) != 0 {
                            (self.crc << 1) ^ $poly
                        } else {
                            self.crc << 1
                        };
                    }
                }
            }

            #[inline]
            fn finish(&self) -> u64 {
                self.crc as u64
            }
        }
    };
}

/// Define structure implementing the CRC-8 algorithm as a `core::hash::Hasher`
/// trait implementation using a pre-calculated lookup table.
///
/// This implementation is much faster at the cost of some space.
/// A struct name and some documentation for it must be provided.
///
/// The lookup table can be generated in the `build.rs` file with the
/// [build_rs_lookup_table_file_generation](macro.build_rs_lookup_table_file_generation.html)
/// macro and then included like in the following example.
/// ```rust
/// use core::hash::Hasher;
/// use embedded_crc_macros::crc8_hasher_lookup_table;
///
/// // include!(concat!(env!("OUT_DIR"), "/lookup_table.rs"));
/// crc8_hasher_lookup_table!(SmbusPec, 0, "SMBus Packet Error Code");
///
/// let mut hasher = SmbusPec::new();
/// hasher.write(&[0xAB, 0xCD]);
/// let pec = hasher.finish();
///
/// println!("PEC: {}", pec);
/// #
/// # // This can be generated on build time with the
/// # // `build_rs_lookup_table_file_generation` macro.
/// # const LOOKUP_TABLE: [u8; 256] = [0; 256];
/// ```
#[macro_export]
macro_rules! crc8_hasher_lookup_table {
    ($name:ident, $initial_value:expr, $doc:expr) => {
        #[doc=$doc]
        struct $name {
            crc: u8,
        }

        impl $name {
            /// Create new instance
            pub fn new() -> Self {
                Self {
                    crc: $initial_value,
                }
            }
        }

        impl core::hash::Hasher for $name {
            #[inline]
            fn write(&mut self, bytes: &[u8]) {
                for byte in bytes {
                    self.crc = LOOKUP_TABLE[(self.crc ^ *byte) as usize];
                }
            }

            #[inline]
            fn finish(&self) -> u64 {
                self.crc as u64
            }
        }
    };
}

/// Code generation macro for use in `build.rs` files.
///
/// Generate file containing a `LOOKUP_TABLE` constant with all the values for the checksum function.
///
/// Example `build.rs` file:
/// ```no_run
/// use embedded_crc_macros::{crc8, build_rs_lookup_table_file_generation};
///
/// crc8!(smbus_pec, 7, 0, "");
/// build_rs_lookup_table_file_generation!(write_file, smbus_pec, "lookup_table.rs", u8, 256);
///
/// fn main() {
///     println!("cargo:rerun-if-changed=build.rs");
///     println!("cargo:rerun-if-changed=lib.rs");
///
///     write_file().expect("Couldn't write lookup table file!");
/// }
/// ```
#[macro_export]
macro_rules! build_rs_lookup_table_file_generation {
    ($function_name:ident, $checksum_function:ident, $lookup_table_file:expr, $t:ty, $size:expr) => {
        fn $function_name() -> std::io::Result<()> {
            use std::io::prelude::*;
            let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
            let out_path = out_path.join($lookup_table_file);
            let mut file = std::fs::File::create(out_path)?;
            file.write_all(
                concat!(
                    "const LOOKUP_TABLE: [",
                    stringify!($t),
                    ";",
                    stringify!($size),
                    "] = [\n"
                )
                .as_bytes(),
            )?;
            for i in 0..$size {
                if i % 16 == 0 {
                    file.write_all(b"    ")?;
                }
                file.write_all(format!("0x{:x}, ", $checksum_function(&[i as $t])).as_bytes())?;
                if i > 0 && (i + 1) % 16 == 0 {
                    file.write_all(b"\n")?;
                }
            }
            file.write_all(b"];\n")
        }
    };
}
