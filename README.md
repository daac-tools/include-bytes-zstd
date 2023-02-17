# `include_bytes_zstd!()`

This library provides a macro to include a file with zstd compression.

This macro can be used like `std::include_bytes!`, but the byte array is compressed by the [zstd
crate](https://docs.rs/zstd/). The included data will be decompressed by the [ruzstd
crate](https://docs.rs/ruzstd/) in runtime and returned as a `Vec<u8>`.

This macro performs the decompression each time it is called.

## Examples

`input.txt`:

```plain
This is a test.
```

Rust code:

```rust
let data = include_bytes_zstd::include_bytes_zstd!("test-resources/input.txt", 19);
assert_eq!(b"This is a test.\n", data.as_slice());
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

See [the guidelines](CONTRIBUTING.md).
