use crate::io::BinaryStream;
use crate::structures::*;
use alloc::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;

pub const METADATA_MAGIC: u32 = 0xFAB11BAF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[wasm_bindgen]
pub enum MetadataVariant {
	Standard,
	Codm,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Metadata {
	pub header: Il2CppGlobalMetadataHeader,
	pub version: f64,
	pub version_int: i32,
	pub variant: MetadataVariant,
	#[wasm_bindgen(getter_with_clone)]
	pub data: Vec<u8>,
	#[wasm_bindgen(getter_with_clone)]
	pub stream: BinaryStream,

	// Core tables
	#[wasm_bindgen(getter_with_clone)]
	pub images: Vec<Il2CppImageDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub assemblies: Vec<Il2CppAssemblyDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub type_defs: Vec<Il2CppTypeDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub method_defs: Vec<Il2CppMethodDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub parameter_defs: Vec<Il2CppParameterDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub field_defs: Vec<Il2CppFieldDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub property_defs: Vec<Il2CppPropertyDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub event_defs: Vec<Il2CppEventDefinition>,
	#[wasm_bindgen(getter_with_clone)]
	pub generic_containers: Vec<Il2CppGenericContainer>,
	#[wasm_bindgen(getter_with_clone)]
	pub generic_parameters: Vec<Il2CppGenericParameter>,
	#[wasm_bindgen(getter_with_clone)]
	pub string_literals: Vec<Il2CppStringLiteral>,
	#[wasm_bindgen(getter_with_clone)]
	pub field_default_values: Vec<Il2CppFieldDefaultValue>,
	#[wasm_bindgen(getter_with_clone)]
	pub parameter_default_values: Vec<Il2CppParameterDefaultValue>,
	#[wasm_bindgen(getter_with_clone)]
	pub field_refs: Vec<Il2CppFieldRef>,
}
