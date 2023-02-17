//! Macro definition of the include-bytes-zstd crate.

extern crate proc_macro;

use std::io::Write;
use std::path::PathBuf;

use proc_macro::TokenStream;
use proc_macro_crate::FoundCrate;
use syn::{parse::Parse, parse_macro_input, LitInt, LitStr, Token};

struct Arguments {
    filename: LitStr,
    _comma: Token![,],
    level: LitInt,
}

impl Parse for Arguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            filename: input.parse()?,
            _comma: input.parse()?,
            level: input.parse()?,
        })
    }
}

/// Includes a file with zstd compression.
///
/// # Arguments
///
/// * `filename` - File name relative to the project root.
/// * `level`: Compression level (1-21).
#[proc_macro]
pub fn include_bytes_zstd(input: TokenStream) -> TokenStream {
    // parses arguments
    let args = parse_macro_input!(input as Arguments);
    let filename = PathBuf::from(args.filename.value());
    let level = args.level.base10_parse::<i32>().unwrap();

    // resolves file path
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let filename = if filename.is_relative() {
        manifest_dir.join(filename)
    } else {
        filename
    };

    // compression
    let bytes = std::fs::read(filename).unwrap();
    let mut compressed = vec![];
    let mut encoder = zstd::Encoder::new(&mut compressed, level).unwrap();
    encoder.write_all(&bytes).unwrap();
    encoder.finish().unwrap();

    // generates code
    let crate_name = proc_macro_crate::crate_name("include-bytes-zstd").unwrap();
    let crate_name = match crate_name {
        FoundCrate::Itself => "include_bytes_zstd".to_string(),
        FoundCrate::Name(crate_name) => crate_name,
    };
    format!("{{ {crate_name}::decode(&{:?}) }}", compressed.as_slice())
        .parse()
        .unwrap()
}
