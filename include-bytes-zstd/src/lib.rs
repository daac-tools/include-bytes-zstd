//! Includes a file with zstd compression.
//!
//! This macro can be used like [`std::include_bytes`], but the byte array is compressed by the
//! [zstd crate](https://docs.rs/zstd/). The included data will be decompressed by the
//! [ruzstd](https://docs.rs/ruzstd) crate in runtime and returned as a [`Vec<u8>`].
//!
//! This macro performs the decompression each time it is called.
//!
//! # Examples
//!
//! `input.txt`:
//! ```plain
//! This is a test.
//! ```
//!
//! Rust code:
//! ```
//! let data = include_bytes_zstd::include_bytes_zstd!("test-resources/input.txt", 19);
//! assert_eq!(b"This is a test.\n", data.as_slice());
//! ```

use std::io::Read;

pub use include_bytes_zstd_macro::include_bytes_zstd;

/// Decodes zstd compressed data.
#[doc(hidden)]
pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut decoder = ruzstd::StreamingDecoder::new(data).unwrap();
    let mut buff = vec![];
    decoder.read_to_end(&mut buff).unwrap();
    buff
}
