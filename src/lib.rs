//! # Postcard
//!
//! Postcard is a `#![no_std]` focused serializer and deserializer for Serde.
//!
//! Postcard aims to be convenient for developers in constrained environments, while
//! allowing for flexibility to customize behavior as needed.
//!
//! ## Design Goals
//!
//! 1. Design primarily for `#![no_std]` usage, in embedded or other constrained contexts
//! 2. Support a maximal set of `serde` features, so `postcard` can be used as a drop in replacement
//! 3. Avoid special differences in code between communication code written for a microcontroller or a desktop/server PC
//! 4. Be resource efficient - memory usage, code size, developer time, and CPU time; in that order
//! 5. Allow library users to customize the serialization and deserialization  behavior to fit their bespoke needs
//!
//! ## Format Stability
//!
//! As of v1.0.0, `postcard` has a documented and stable wire format. More information about this
//! wire format can be found in the `spec/` folder of the Postcard repository, or viewed online
//! at <https://postcard.jamesmunns.com>.
//!
//! Work towards the Postcard Specification and portions of the Postcard 1.0 Release
//! has been sponsored by Mozilla Corporation.
//!
//! ## Variable Length Data
//!
//! All signed and unsigned integers larger than eight bits are encoded using a [Varint].
//! This includes the length of array slices, as well as the discriminant of `enums`.
//!
//! For more information, see the [Varint] chapter of the wire specification.
//!
//! [Varint]: https://postcard.jamesmunns.com/wire-format.html#varint-encoded-integers
//!
//! ## Example - Serialization/Deserialization
//!
//! Postcard can serialize and deserialize messages similar to other `serde` formats.
//!
//! Using the default `heapless` feature to serialize to a `heapless::Vec<u8>`:
//!
//! ```rust
//! # #[cfg(feature = "heapless")] {
//! use core::ops::Deref;
//! use serde::{Serialize, Deserialize};
//! use postcard::{from_bytes, to_vec};
//! use heapless::Vec;
//!
//! #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
//! struct RefStruct<'a> {
//!     bytes: &'a [u8],
//!     str_s: &'a str,
//! }
//! let message = "hElLo";
//! let bytes = [0x01, 0x10, 0x02, 0x20];
//! let output: Vec<u8, 11> = to_vec(&RefStruct {
//!     bytes: &bytes,
//!     str_s: message,
//! }).unwrap();
//!
//! assert_eq!(
//!     &[0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',],
//!     output.deref()
//! );
//!
//! let out: RefStruct = from_bytes(output.deref()).unwrap();
//! assert_eq!(
//!     out,
//!     RefStruct {
//!         bytes: &bytes,
//!         str_s: message,
//!     }
//! );
//! # }
//! ```
//!
//! Or the optional `alloc` feature to serialize to an `alloc::vec::Vec<u8>`:
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! use core::ops::Deref;
//! use serde::{Serialize, Deserialize};
//! use postcard::{from_bytes, to_allocvec};
//! extern crate alloc;
//! use alloc::vec::Vec;
//!
//! #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
//! struct RefStruct<'a> {
//!     bytes: &'a [u8],
//!     str_s: &'a str,
//! }
//! let message = "hElLo";
//! let bytes = [0x01, 0x10, 0x02, 0x20];
//! let output: Vec<u8> = to_allocvec(&RefStruct {
//!     bytes: &bytes,
//!     str_s: message,
//! }).unwrap();
//!
//! assert_eq!(
//!     &[0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',],
//!     output.deref()
//! );
//!
//! let out: RefStruct = from_bytes(output.deref()).unwrap();
//! assert_eq!(
//!     out,
//!     RefStruct {
//!         bytes: &bytes,
//!         str_s: message,
//!     }
//! );
//! # }
//! ```
//!
//! ## Flavors
//!
//! `postcard` supports a system called `Flavors`, which are used to modify the way
//! postcard serializes or processes serialized data. These flavors act as "plugins" or "middlewares"
//! during the serialization or deserialization process, and can be combined to obtain complex protocol formats.
//!
//! See the documentation of the [Serialization Flavors][crate::ser_flavors] or [Deserialization Flavors][crate::de_flavors]
//! for more information on usage
//!
//! ## Setup - `Cargo.toml`
//!
//! Don't forget to add [the `no-std` subset](https://serde.rs/no-std.html) of `serde` along with `postcard` to the `[dependencies]` section of your `Cargo.toml`!
//!
//! ```toml
//! [dependencies]
//! postcard = "1.0.0"
//!
//! # By default, `serde` has the `std` feature enabled, which makes it unsuitable for embedded targets
//! # disabling default-features fixes this
//! serde = { version = "1.0.*", default-features = false }
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.

#![cfg_attr(not(any(test, feature = "use-std")), no_std)]
#![warn(missing_docs)]

pub mod accumulator;
mod de;
mod error;
pub mod fixint;
mod ser;
mod varint;

pub use de::deserializer::Deserializer;
pub use de::flavors as de_flavors;
pub use de::{from_bytes, from_bytes_cobs, take_from_bytes, take_from_bytes_cobs};
pub use error::{Error, Result};
pub use ser::flavors as ser_flavors;
pub use ser::{serialize_with_flavor, serializer::Serializer, to_slice, to_slice_cobs};

#[cfg(feature = "heapless")]
pub use ser::{to_vec, to_vec_cobs};

#[cfg(feature = "use-std")]
pub use ser::{to_stdvec, to_stdvec_cobs};

#[cfg(feature = "alloc")]
pub use ser::{to_allocvec, to_allocvec_cobs};

#[cfg(test)]
mod test {
    #[test]
    fn varint_boundary_canon() {
        let x = u32::MAX;
        let mut buf = [0u8; 5];
        let used = crate::to_slice(&x, &mut buf).unwrap();
        let deser: u32 = crate::from_bytes(used).unwrap();
        assert_eq!(deser, u32::MAX);
        assert_eq!(used, &mut [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
        let deser: Result<u32, crate::Error> = crate::from_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]);
        assert_eq!(deser, Err(crate::Error::DeserializeBadVarint));
    }
}
