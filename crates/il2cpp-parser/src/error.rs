use alloc::string::String;
use core::fmt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub struct OutOfBoundsErr {
	pub offset: u64,
	pub size: usize,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct InvalidArraySizeErr {
	pub count: u64,
	pub elem_size: usize,
	#[wasm_bindgen(getter_with_clone)]
	pub context: String,
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum Error {
	InvalidMetadata(String),
	UnsupportedVersion(i32),
	OutOfBounds(OutOfBoundsErr),
	InvalidArraySize(InvalidArraySizeErr),
	Io(String),
	Other(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::InvalidMetadata(s) => write!(f, "Invalid metadata: {s}"),
			Error::UnsupportedVersion(v) => write!(f, "Unsupported metadata version: {v}"),
			Error::OutOfBounds(OutOfBoundsErr { offset, size }) => {
				write!(f, "Read out of bounds: offset 0x{offset:x}, size {size}")
			}
			Error::InvalidArraySize(InvalidArraySizeErr {
				count,
				elem_size,
				context,
			}) => write!(
				f,
				"Invalid array size: count={count}, element_size={elem_size} ({context})"
			),
			Error::Io(s) => write!(f, "IO error: {s}"),
			Error::Other(s) => write!(f, "{s}"),
		}
	}
}

pub type Result<T> = core::result::Result<T, Error>;
