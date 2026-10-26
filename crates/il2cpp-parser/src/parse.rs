use crate::error::{Error, Result};
use crate::io::BinaryStream;
use crate::metadata::{METADATA_MAGIC, Metadata, MetadataVariant};
use crate::structures::*;
use alloc::string::String;
use alloc::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ParsedMetadata {
	#[wasm_bindgen(getter_with_clone)]
	pub metadata: Metadata,
}

impl ParsedMetadata {
	pub fn parse(data: Vec<u8>) -> Result<Self> {
		let mut stream = BinaryStream::new(data.clone());
		if stream.len() < 16 {
			return Err(Error::InvalidMetadata("File too small".into()));
		}

		let sanity = stream.read_u32()?;
		if sanity != METADATA_MAGIC {
			return Err(Error::InvalidMetadata("Invalid magic".into()));
		}
		let version_int = stream.read_i32()?;
		let version = version_int as f64;

		let variant = MetadataVariant::Standard;

		stream.set_position(0);
		let header = match variant {
			MetadataVariant::Standard => Il2CppGlobalMetadataHeader::read(&mut stream, version)?,
			MetadataVariant::Codm => Il2CppGlobalMetadataHeader::read_codm(&mut stream)?,
		};

		let metadata = Metadata {
			header: header.clone(),
			version,
			version_int,
			variant,
			data: data.clone(),
			stream: BinaryStream::new(data),
			images: Vec::new(),
			assemblies: Vec::new(),
			type_defs: Vec::new(),
			method_defs: Vec::new(),
			parameter_defs: Vec::new(),
			field_defs: Vec::new(),
			property_defs: Vec::new(),
			event_defs: Vec::new(),
			generic_containers: Vec::new(),
			generic_parameters: Vec::new(),
			string_literals: Vec::new(),
			field_default_values: Vec::new(),
			parameter_default_values: Vec::new(),
			field_refs: Vec::new(),
		};

		Ok(ParsedMetadata { metadata })
	}

	pub fn get_string(&self, index: i32) -> Result<String> {
		if index < 0 {
			return Ok(String::new());
		}
		Ok(self
			.metadata
			.stream
			.peek_string_to_null_at(self.metadata.header.string_offset as u64 + index as u64)?)
	}
}
