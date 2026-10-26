extern crate alloc;

mod enums;
mod error;
mod io;
mod metadata;
mod parse;
mod structures;

pub use enums::*;
pub use error::{Error, Result};
pub use metadata::Metadata;
pub use metadata::MetadataVariant;
pub use parse::ParsedMetadata;
pub use structures::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_from_bytes(data: Vec<u8>) -> Result<ParsedMetadata> {
	ParsedMetadata::parse(data)
}
