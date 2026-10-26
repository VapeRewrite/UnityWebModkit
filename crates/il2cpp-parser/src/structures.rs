use wasm_bindgen::prelude::wasm_bindgen;

use crate::error::Result;
use crate::io::BinaryStream;

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppGlobalMetadataHeader {
	pub sanity: u32,
	pub version: i32,
	pub string_literal_offset: i32,
	pub string_literal_size: i32,
	pub string_literal_data_offset: i32,
	pub string_literal_data_size: i32,
	pub string_offset: i32,
	pub string_size: i32,
	pub events_offset: i32,
	pub events_size: i32,
	pub properties_offset: i32,
	pub properties_size: i32,
	pub methods_offset: i32,
	pub methods_size: i32,
	pub parameter_default_values_offset: i32,
	pub parameter_default_values_size: i32,
	pub field_default_values_offset: i32,
	pub field_default_values_size: i32,
	pub field_and_parameter_default_value_data_offset: i32,
	pub field_and_parameter_default_value_data_size: i32,
	pub field_marshaled_sizes_offset: i32,
	pub field_marshaled_sizes_size: i32,
	pub parameters_offset: i32,
	pub parameters_size: i32,
	pub fields_offset: i32,
	pub fields_size: i32,
	pub generic_parameters_offset: i32,
	pub generic_parameters_size: i32,
	pub generic_parameter_constraints_offset: i32,
	pub generic_parameter_constraints_size: i32,
	pub generic_containers_offset: i32,
	pub generic_containers_size: i32,
	pub nested_types_offset: i32,
	pub nested_types_size: i32,
	pub interfaces_offset: i32,
	pub interfaces_size: i32,
	pub vtable_methods_offset: i32,
	pub vtable_methods_size: i32,
	pub interface_offsets_offset: i32,
	pub interface_offsets_size: i32,
	pub type_definitions_offset: i32,
	pub type_definitions_size: i32,
	pub rgctx_entries_offset: i32,
	pub rgctx_entries_count: i32,
	pub images_offset: i32,
	pub images_size: i32,
	pub assemblies_offset: i32,
	pub assemblies_size: i32,
	pub metadata_usage_lists_offset: i32,
	pub metadata_usage_lists_count: i32,
	pub metadata_usage_pairs_offset: i32,
	pub metadata_usage_pairs_count: i32,
	pub field_refs_offset: i32,
	pub field_refs_size: i32,
	pub referenced_assemblies_offset: i32,
	pub referenced_assemblies_size: i32,
	pub attributes_info_offset: i32,
	pub attributes_info_count: i32,
	pub attribute_types_offset: i32,
	pub attribute_types_count: i32,
	pub attribute_data_offset: i32,
	pub attribute_data_size: i32,
	pub attribute_data_range_offset: i32,
	pub attribute_data_range_size: i32,
	pub unresolved_virtual_call_parameter_types_offset: i32,
	pub unresolved_virtual_call_parameter_types_size: i32,
	pub unresolved_virtual_call_parameter_ranges_offset: i32,
	pub unresolved_virtual_call_parameter_ranges_size: i32,
	pub windows_runtime_type_names_offset: i32,
	pub windows_runtime_type_names_size: i32,
	pub windows_runtime_strings_offset: i32,
	pub windows_runtime_strings_size: i32,
	pub exported_type_definitions_offset: i32,
	pub exported_type_definitions_size: i32,
	pub type_definitions_count: i32,
	pub generic_containers_count: i32,
	pub interface_offsets_count: i32,
	pub parameters_count: i32,
	pub events_count: i32,
	pub properties_count: i32,
	pub nested_types_count: i32,
	pub methods_count: i32,
	pub generic_parameters_count: i32,
	pub fields_count: i32,
	pub field_and_parameter_default_value_data_count: i32,
}

impl Il2CppGlobalMetadataHeader {
	pub fn read(stream: &mut BinaryStream, version: f64) -> Result<Self> {
		let v38 = version >= 38.0;
		macro_rules! sec {
			() => {{
				let o = stream.read_i32()?;
				let s = stream.read_i32()?;
				let c = if v38 { stream.read_i32()? } else { 0 };
				(o, s, c)
			}};
		}
		let sanity = stream.read_u32()?;
		let version_field = stream.read_i32()?;
		let (string_literal_offset, string_literal_size, _) = sec!();
		let (string_literal_data_offset, string_literal_data_size, _) = sec!();
		let (string_offset, string_size, _) = sec!();
		let (events_offset, events_size, events_count) = sec!();
		let (properties_offset, properties_size, properties_count) = sec!();
		let (methods_offset, methods_size, methods_count) = sec!();
		let (parameter_default_values_offset, parameter_default_values_size, _) = sec!();
		let (field_default_values_offset, field_default_values_size, _) = sec!();
		let (
			field_and_parameter_default_value_data_offset,
			field_and_parameter_default_value_data_size,
			field_and_parameter_default_value_data_count,
		) = sec!();
		let (field_marshaled_sizes_offset, field_marshaled_sizes_size, _) = sec!();
		let (parameters_offset, parameters_size, parameters_count) = sec!();
		let (fields_offset, fields_size, fields_count) = sec!();
		let (generic_parameters_offset, generic_parameters_size, generic_parameters_count) = sec!();
		let (generic_parameter_constraints_offset, generic_parameter_constraints_size, _) = sec!();
		let (generic_containers_offset, generic_containers_size, generic_containers_count) = sec!();
		let (nested_types_offset, nested_types_size, nested_types_count) = sec!();
		let (interfaces_offset, interfaces_size, _) = sec!();
		let (vtable_methods_offset, vtable_methods_size, _) = sec!();
		let (interface_offsets_offset, interface_offsets_size, interface_offsets_count) = sec!();
		let (type_definitions_offset, type_definitions_size, type_definitions_count) = sec!();
		let (rgctx_entries_offset, rgctx_entries_count) = if version <= 24.15 {
			let (o, s, _) = sec!();
			(o, s)
		} else {
			(0, 0)
		};
		let (images_offset, images_size, _) = sec!();
		let (assemblies_offset, assemblies_size, _) = sec!();
		let (metadata_usage_lists_offset, metadata_usage_lists_count) = if version < 27.0 {
			let (o, s, _) = sec!();
			(o, s)
		} else {
			(0, 0)
		};
		let (metadata_usage_pairs_offset, metadata_usage_pairs_count) = if version < 27.0 {
			let (o, s, _) = sec!();
			(o, s)
		} else {
			(0, 0)
		};
		let (field_refs_offset, field_refs_size, _) = sec!();
		let (referenced_assemblies_offset, referenced_assemblies_size, _) = sec!();
		let (
			attributes_info_offset,
			attributes_info_count,
			attribute_types_offset,
			attribute_types_count,
			attribute_data_offset,
			attribute_data_size,
			attribute_data_range_offset,
			attribute_data_range_size,
		) = if version < 29.0 {
			let (ao, ac, _) = sec!();
			let (to, tc, _) = sec!();
			(ao, ac, to, tc, 0, 0, 0, 0)
		} else {
			let (ddo, dds, _) = sec!();
			let (dro, drs, _) = sec!();
			(0, 0, 0, 0, ddo, dds, dro, drs)
		};
		let (
			unresolved_virtual_call_parameter_types_offset,
			unresolved_virtual_call_parameter_types_size,
			_,
		) = sec!();
		let (
			unresolved_virtual_call_parameter_ranges_offset,
			unresolved_virtual_call_parameter_ranges_size,
			_,
		) = sec!();
		let (windows_runtime_type_names_offset, windows_runtime_type_names_size, _) = sec!();
		let (windows_runtime_strings_offset, windows_runtime_strings_size) = if version >= 27.0 {
			let (o, s, _) = sec!();
			(o, s)
		} else {
			(0, 0)
		};
		let (exported_type_definitions_offset, exported_type_definitions_size) = if version >= 24.0
		{
			let (o, s, _) = sec!();
			(o, s)
		} else {
			(0, 0)
		};
		Ok(Self {
			sanity,
			version: version_field,
			string_literal_offset,
			string_literal_size,
			string_literal_data_offset,
			string_literal_data_size,
			string_offset,
			string_size,
			events_offset,
			events_size,
			properties_offset,
			properties_size,
			methods_offset,
			methods_size,
			parameter_default_values_offset,
			parameter_default_values_size,
			field_default_values_offset,
			field_default_values_size,
			field_and_parameter_default_value_data_offset,
			field_and_parameter_default_value_data_size,
			field_marshaled_sizes_offset,
			field_marshaled_sizes_size,
			parameters_offset,
			parameters_size,
			fields_offset,
			fields_size,
			generic_parameters_offset,
			generic_parameters_size,
			generic_parameter_constraints_offset,
			generic_parameter_constraints_size,
			generic_containers_offset,
			generic_containers_size,
			nested_types_offset,
			nested_types_size,
			interfaces_offset,
			interfaces_size,
			vtable_methods_offset,
			vtable_methods_size,
			interface_offsets_offset,
			interface_offsets_size,
			type_definitions_offset,
			type_definitions_size,
			rgctx_entries_offset,
			rgctx_entries_count,
			images_offset,
			images_size,
			assemblies_offset,
			assemblies_size,
			metadata_usage_lists_offset,
			metadata_usage_lists_count,
			metadata_usage_pairs_offset,
			metadata_usage_pairs_count,
			field_refs_offset,
			field_refs_size,
			referenced_assemblies_offset,
			referenced_assemblies_size,
			attributes_info_offset,
			attributes_info_count,
			attribute_types_offset,
			attribute_types_count,
			attribute_data_offset,
			attribute_data_size,
			attribute_data_range_offset,
			attribute_data_range_size,
			unresolved_virtual_call_parameter_types_offset,
			unresolved_virtual_call_parameter_types_size,
			unresolved_virtual_call_parameter_ranges_offset,
			unresolved_virtual_call_parameter_ranges_size,
			windows_runtime_type_names_offset,
			windows_runtime_type_names_size,
			windows_runtime_strings_offset,
			windows_runtime_strings_size,
			exported_type_definitions_offset,
			exported_type_definitions_size,
			type_definitions_count,
			generic_containers_count,
			interface_offsets_count,
			parameters_count,
			events_count,
			properties_count,
			nested_types_count,
			methods_count,
			generic_parameters_count,
			fields_count,
			field_and_parameter_default_value_data_count,
		})
	}

	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let sanity = stream.read_u32()?;
		let version_field = stream.read_i32()?;
		let string_literal_offset = stream.read_i32()?;
		let string_literal_size = stream.read_i32()?;
		let string_literal_data_offset = stream.read_i32()?;
		let string_literal_data_size = stream.read_i32()?;
		let string_offset = stream.read_i32()?;
		let string_size = stream.read_i32()?;
		let events_offset = stream.read_i32()?;
		let events_size = stream.read_i32()?;
		let properties_offset = stream.read_i32()?;
		let properties_size = stream.read_i32()?;
		let methods_offset = stream.read_i32()?;
		let methods_size = stream.read_i32()?;
		let parameter_default_values_offset = stream.read_i32()?;
		let parameter_default_values_size = stream.read_i32()?;
		let field_default_values_offset = stream.read_i32()?;
		let field_default_values_size = stream.read_i32()?;
		let field_and_parameter_default_value_data_offset = stream.read_i32()?;
		let field_and_parameter_default_value_data_size = stream.read_i32()?;
		let field_marshaled_sizes_offset = stream.read_i32()?;
		let field_marshaled_sizes_size = stream.read_i32()?;
		let parameters_offset = stream.read_i32()?;
		let parameters_size = stream.read_i32()?;
		let fields_offset = stream.read_i32()?;
		let fields_size = stream.read_i32()?;
		let generic_parameters_offset = stream.read_i32()?;
		let generic_parameters_size = stream.read_i32()?;
		let generic_parameter_constraints_offset = stream.read_i32()?;
		let generic_parameter_constraints_size = stream.read_i32()?;
		let generic_containers_offset = stream.read_i32()?;
		let generic_containers_size = stream.read_i32()?;
		let nested_types_offset = stream.read_i32()?;
		let nested_types_size = stream.read_i32()?;
		let interfaces_offset = stream.read_i32()?;
		let interfaces_size = stream.read_i32()?;
		let vtable_methods_offset = stream.read_i32()?;
		let vtable_methods_size = stream.read_i32()?;
		let interface_offsets_offset = stream.read_i32()?;
		let interface_offsets_size = stream.read_i32()?;
		let type_definitions_offset = stream.read_i32()?;
		let type_definitions_size = stream.read_i32()?;
		let rgctx_entries_offset = stream.read_i32()?;
		let rgctx_entries_count = stream.read_i32()?;
		let images_offset = stream.read_i32()?;
		let images_size = stream.read_i32()?;
		let assemblies_offset = stream.read_i32()?;
		let assemblies_size = stream.read_i32()?;
		let metadata_usage_lists_offset = stream.read_i32()?;
		let metadata_usage_lists_count = stream.read_i32()?;
		let metadata_usage_pairs_offset = stream.read_i32()?;
		let metadata_usage_pairs_count = stream.read_i32()?;
		let field_refs_offset = stream.read_i32()?;
		let field_refs_size = stream.read_i32()?;
		let referenced_assemblies_offset = stream.read_i32()?;
		let referenced_assemblies_size = stream.read_i32()?;
		let attributes_info_offset = stream.read_i32()?;
		let attributes_info_count = stream.read_i32()?;
		let attribute_types_offset = stream.read_i32()?;
		let attribute_types_count = stream.read_i32()?;
		let unresolved_virtual_call_parameter_types_offset = stream.read_i32()?;
		let unresolved_virtual_call_parameter_types_size = stream.read_i32()?;
		let unresolved_virtual_call_parameter_ranges_offset = stream.read_i32()?;
		let unresolved_virtual_call_parameter_ranges_size = stream.read_i32()?;
		let windows_runtime_type_names_offset = stream.read_i32()?;
		let windows_runtime_type_names_size = stream.read_i32()?;
		Ok(Self {
			sanity,
			version: version_field,
			string_literal_offset,
			string_literal_size,
			string_literal_data_offset,
			string_literal_data_size,
			string_offset,
			string_size,
			events_offset,
			events_size,
			properties_offset,
			properties_size,
			methods_offset,
			methods_size,
			parameter_default_values_offset,
			parameter_default_values_size,
			field_default_values_offset,
			field_default_values_size,
			field_and_parameter_default_value_data_offset,
			field_and_parameter_default_value_data_size,
			field_marshaled_sizes_offset,
			field_marshaled_sizes_size,
			parameters_offset,
			parameters_size,
			fields_offset,
			fields_size,
			generic_parameters_offset,
			generic_parameters_size,
			generic_parameter_constraints_offset,
			generic_parameter_constraints_size,
			generic_containers_offset,
			generic_containers_size,
			nested_types_offset,
			nested_types_size,
			interfaces_offset,
			interfaces_size,
			vtable_methods_offset,
			vtable_methods_size,
			interface_offsets_offset,
			interface_offsets_size,
			type_definitions_offset,
			type_definitions_size,
			rgctx_entries_offset,
			rgctx_entries_count,
			images_offset,
			images_size,
			assemblies_offset,
			assemblies_size,
			metadata_usage_lists_offset,
			metadata_usage_lists_count,
			metadata_usage_pairs_offset,
			metadata_usage_pairs_count,
			field_refs_offset,
			field_refs_size,
			referenced_assemblies_offset,
			referenced_assemblies_size,
			attributes_info_offset,
			attributes_info_count,
			attribute_types_offset,
			attribute_types_count,
			attribute_data_offset: 0,
			attribute_data_size: 0,
			attribute_data_range_offset: 0,
			attribute_data_range_size: 0,
			unresolved_virtual_call_parameter_types_offset,
			unresolved_virtual_call_parameter_types_size,
			unresolved_virtual_call_parameter_ranges_offset,
			unresolved_virtual_call_parameter_ranges_size,
			windows_runtime_type_names_offset,
			windows_runtime_type_names_size,
			windows_runtime_strings_offset: 0,
			windows_runtime_strings_size: 0,
			exported_type_definitions_offset: 0,
			exported_type_definitions_size: 0,
			..Default::default()
		})
	}
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppImageDefinition {
	pub name_index: i32,
	pub assembly_index: i32,
	pub type_start: i32,
	pub type_count: i32,
	pub exported_type_start: i32,
	pub exported_type_count: i32,
	pub entry_point_index: i32,
	pub token: u32,
	pub custom_attribute_start: i32,
	pub custom_attribute_count: i32,
}

impl Il2CppImageDefinition {
	pub fn read(stream: &mut BinaryStream, version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let assembly_index = stream.read_i32()?;
		let type_start = stream.read_i32()?;
		let type_count = stream.read_u32()?;
		let exported_type_start = if version >= 24.0 {
			stream.read_i32()?
		} else {
			0
		};
		let exported_type_count = if version >= 24.0 {
			stream.read_u32()?
		} else {
			0
		};
		let entry_point_index = stream.read_i32()?;
		let token = if version >= 19.0 {
			stream.read_u32()?
		} else {
			0
		};
		let custom_attribute_start = if version >= 24.1 {
			stream.read_i32()?
		} else {
			0
		};
		let custom_attribute_count = if version >= 24.1 {
			stream.read_i32()?
		} else {
			0
		};
		Ok(Self {
			name_index,
			assembly_index,
			type_start,
			type_count: type_count as i32,
			exported_type_start,
			exported_type_count: exported_type_count as i32,
			entry_point_index,
			token,
			custom_attribute_start,
			custom_attribute_count,
		})
	}

	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_u32()? as i32;
		let assembly_index = stream.read_i32()?;
		let type_start = stream.read_i32()?;
		let type_count = stream.read_u32()? as i32;
		let entry_point_index = stream.read_i32()?;
		Ok(Self {
			name_index,
			assembly_index,
			type_start,
			type_count,
			exported_type_start: 0,
			exported_type_count: 0,
			entry_point_index,
			token: 1,
			custom_attribute_start: 0,
			custom_attribute_count: 0,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 20;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppAssemblyNameDefinition {
	pub name_index: i32,
	pub culture_index: i32,
	pub hash_value_index: i32,
	pub public_key_index: i32,
	pub hash_alg: i32,
	pub hash_len: i32,
	pub flags: u32,
	pub major: i32,
	pub minor: i32,
	pub build: i32,
	pub revision: i32,
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppAssemblyDefinition {
	pub image_index: i32,
	pub token: u32,
	pub module_token: u32,
	pub custom_attribute_index: i32,
	pub referenced_assembly_start: i32,
	pub referenced_assembly_count: i32,
	pub aname: Il2CppAssemblyNameDefinition,
}

impl Il2CppAssemblyDefinition {
	pub fn read(stream: &mut BinaryStream, version: f64) -> Result<Self> {
		let image_index = stream.read_i32()?;
		let token = if version >= 24.1 {
			stream.read_u32()?
		} else {
			0
		};
		let module_token = if version >= 38.0 {
			stream.read_u32()?
		} else {
			0
		};
		let custom_attribute_index = if version <= 24.0 {
			stream.read_i32()?
		} else {
			0
		};
		let referenced_assembly_start = if version >= 20.0 {
			stream.read_i32()?
		} else {
			0
		};
		let referenced_assembly_count = if version >= 20.0 {
			stream.read_i32()?
		} else {
			0
		};
		let mut aname = Il2CppAssemblyNameDefinition::default();
		aname.name_index = stream.read_i32()?;
		aname.culture_index = stream.read_i32()?;
		if version <= 24.3 {
			aname.hash_value_index = stream.read_i32()?;
		} else {
			aname.hash_value_index = 0;
		}
		aname.public_key_index = stream.read_i32()?;
		aname.hash_alg = stream.read_i32()?;
		aname.hash_len = stream.read_i32()?;
		aname.flags = stream.read_u32()?;
		aname.major = stream.read_i32()?;
		aname.minor = stream.read_i32()?;
		aname.build = stream.read_i32()?;
		aname.revision = stream.read_i32()?;
		Ok(Self {
			image_index,
			token,
			module_token,
			custom_attribute_index,
			referenced_assembly_start,
			referenced_assembly_count,
			aname,
		})
	}

	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let image_index = stream.read_i32()?;
		let custom_attribute_index = stream.read_i16()? as i32;
		let referenced_assembly_start = stream.read_i32()?;
		let referenced_assembly_count = stream.read_i32()?;
		let mut aname = Il2CppAssemblyNameDefinition::default();
		aname.name_index = stream.read_i32()?;
		aname.culture_index = stream.read_i32()?;
		aname.public_key_index = stream.read_i32()?;
		aname.hash_alg = stream.read_i32()?;
		aname.hash_len = stream.read_i32()?;
		aname.flags = stream.read_u32()?;
		aname.major = stream.read_i32()?;
		aname.minor = stream.read_i32()?;
		aname.build = stream.read_i32()?;
		aname.revision = stream.read_i32()?;
		Ok(Self {
			image_index,
			token: 0,
			module_token: 0,
			custom_attribute_index,
			referenced_assembly_start,
			referenced_assembly_count,
			aname,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 4 + 2 + 4 + 4 + 4 * 11 + 8;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppTypeDefinition {
	pub name_index: i32,
	pub namespace_index: i32,
	pub custom_attribute_index: i32,
	pub byval_type_index: i32,
	pub byref_type_index: i32,
	pub declared_type_index: i32,
	pub parent_index: i32,
	pub element_type_index: i32,
	pub generic_container_index: i32,
	pub flags: u32,
	pub field_start: i32,
	pub method_start: i32,
	pub event_start: i32,
	pub property_start: i32,
	pub nested_types_start: i32,
	pub interfaces_start: i32,
	pub vtable_start: i32,
	pub interface_offsets_start: i32,
	pub method_count: u16,
	pub property_count: u16,
	pub field_count: u16,
	pub event_count: u16,
	pub nested_type_count: u16,
	pub vtable_count: u16,
	pub interfaces_count: u16,
	pub interface_offsets_count: u16,
	pub bitfield: u32,
	pub token: u32,
}

impl Il2CppTypeDefinition {
	pub fn read(stream: &mut BinaryStream, version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let namespace_index = stream.read_i32()?;
		let custom_attribute_index = stream.read_i32()?;
		let byval_type_index = stream.read_i32()?;
		let byref_type_index = stream.read_i32()?;
		let declared_type_index = stream.read_i32()?;
		let parent_index = stream.read_i32()?;
		let element_type_index = stream.read_i32()?;
		let generic_container_index = stream.read_i32()?;
		let flags = stream.read_u32()?;
		let field_start = stream.read_i32()?;
		let method_start = stream.read_i32()?;
		let event_start = stream.read_i32()?;
		let property_start = stream.read_i32()?;
		let nested_types_start = stream.read_i32()?;
		let interfaces_start = stream.read_i32()?;
		let vtable_start = stream.read_i32()?;
		let interface_offsets_start = stream.read_i32()?;
		let method_count = stream.read_u16()?;
		let property_count = stream.read_u16()?;
		let field_count = stream.read_u16()?;
		let event_count = stream.read_u16()?;
		let nested_type_count = stream.read_u16()?;
		let vtable_count = stream.read_u16()?;
		let interfaces_count = stream.read_u16()?;
		let interface_offsets_count = stream.read_u16()?;
		let bitfield = if version >= 19.0 {
			stream.read_u32()?
		} else {
			0
		};
		let token = if version >= 19.0 {
			stream.read_u32()?
		} else {
			0
		};
		Ok(Self {
			name_index,
			namespace_index,
			custom_attribute_index,
			byval_type_index,
			byref_type_index,
			declared_type_index,
			parent_index,
			element_type_index,
			generic_container_index,
			flags,
			field_start,
			method_start,
			event_start,
			property_start,
			nested_types_start,
			interfaces_start,
			vtable_start,
			interface_offsets_start,
			method_count,
			property_count,
			field_count,
			event_count,
			nested_type_count,
			vtable_count,
			interfaces_count,
			interface_offsets_count,
			bitfield,
			token,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let namespace_index = stream.read_i32()?;
		let custom_attribute_index = stream.read_i32()?;
		let byval_type_index = stream.read_i32()?;
		let byref_type_index = stream.read_i32()?;
		let declared_type_index = stream.read_i32()?;
		let parent_index = stream.read_i32()?;
		let element_type_index = stream.read_i32()?;
		let generic_container_index = stream.read_i32()?;
		let flags = stream.read_u32()?;
		let field_start = stream.read_i32()?;
		let method_start = stream.read_i32()?;
		let event_start = stream.read_i32()?;
		let property_start = stream.read_i32()?;
		let nested_types_start = stream.read_i32()?;
		let interfaces_start = stream.read_i32()?;
		let vtable_start = stream.read_i32()?;
		let interface_offsets_start = stream.read_i32()?;
		let method_count = stream.read_u16()?;
		let property_count = stream.read_u16()?;
		let field_count = stream.read_u16()?;
		let event_count = stream.read_u16()?;
		let nested_type_count = stream.read_u16()?;
		let vtable_count = stream.read_u16()?;
		let interfaces_count = stream.read_u16()?;
		let interface_offsets_count = stream.read_u16()?;
		let bitfield = stream.read_u32()?;
		let token = 1;
		Ok(Self {
			name_index,
			namespace_index,
			custom_attribute_index,
			byval_type_index,
			byref_type_index,
			declared_type_index,
			parent_index,
			element_type_index,
			generic_container_index,
			flags,
			field_start,
			method_start,
			event_start,
			property_start,
			nested_types_start,
			interfaces_start,
			vtable_start,
			interface_offsets_start,
			method_count,
			property_count,
			field_count,
			event_count,
			nested_type_count,
			vtable_count,
			interfaces_count,
			interface_offsets_count,
			bitfield,
			token,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 32 * 4 + 8 * 2 + 4 + 4;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppMethodDefinition {
	pub name_index: i32,
	pub return_type: i32,
	pub declaring_type: i32,
	pub parameter_start: i32,
	pub generic_container_index: i32,
	pub token: u32,
	pub flags: u16,
	pub iflags: u16,
	pub slot: u16,
	pub parameter_count: u16,
}

impl Il2CppMethodDefinition {
	pub fn read(stream: &mut BinaryStream, version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let return_type = stream.read_i32()?;
		let declaring_type = if version >= 16.0 {
			stream.read_i32()?
		} else {
			-1
		};
		let parameter_start = stream.read_i32()?;
		let generic_container_index = if version >= 16.0 {
			stream.read_i32()?
		} else {
			-1
		};
		let token = stream.read_u32()?;
		let flags = stream.read_u16()?;
		let iflags = stream.read_u16()?;
		let slot = stream.read_u16()?;
		let parameter_count = stream.read_u16()?;
		Ok(Self {
			name_index,
			return_type,
			declaring_type,
			parameter_start,
			generic_container_index,
			token,
			flags,
			iflags,
			slot,
			parameter_count,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let return_type = stream.read_i32()?;
		let declaring_type = stream.read_i32()?;
		let parameter_start = stream.read_i32()?;
		let generic_container_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		let flags = stream.read_u16()?;
		let iflags = stream.read_u16()?;
		let slot = stream.read_u16()?;
		let parameter_count = stream.read_u16()?;
		Ok(Self {
			name_index,
			return_type,
			declaring_type,
			parameter_start,
			generic_container_index,
			token,
			flags,
			iflags,
			slot,
			parameter_count,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 4 * 5 + 4 + 2 * 4;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppParameterDefinition {
	pub name_index: i32,
	pub token: u32,
	pub custom_attribute_index: i32,
	pub type_index: i32,
}
impl Il2CppParameterDefinition {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		let custom_attribute_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		Ok(Self {
			name_index,
			token,
			custom_attribute_index,
			type_index,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		let custom_attribute_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		Ok(Self {
			name_index,
			token,
			custom_attribute_index,
			type_index,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 16;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppFieldDefinition {
	pub name_index: i32,
	pub type_index: i32,
	pub custom_attribute_index: i32,
	pub token: u32,
}
impl Il2CppFieldDefinition {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		let custom_attribute_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		Ok(Self {
			name_index,
			type_index,
			custom_attribute_index,
			token,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		let custom_attribute_index = stream.read_i32()?;
		Ok(Self {
			name_index,
			type_index,
			custom_attribute_index,
			token: 0,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 12;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppPropertyDefinition {
	pub name_index: i32,
	pub get: i32,
	pub set: i32,
	pub attrs: u32,
	pub custom_attribute_index: i32,
	pub token: u32,
}
impl Il2CppPropertyDefinition {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let get = stream.read_i32()?;
		let set = stream.read_i32()?;
		let attrs = stream.read_u32()?;
		let custom_attribute_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		Ok(Self {
			name_index,
			get,
			set,
			attrs,
			custom_attribute_index,
			token,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let get = stream.read_i32()?;
		let set = stream.read_i32()?;
		let attrs = stream.read_u32()?;
		let custom_attribute_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		Ok(Self {
			name_index,
			get,
			set,
			attrs,
			custom_attribute_index,
			token,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 4 * 6;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppEventDefinition {
	pub name_index: i32,
	pub type_index: i32,
	pub add: i32,
	pub remove: i32,
	pub raise: i32,
	pub custom_attribute_index: i32,
	pub token: u32,
}
impl Il2CppEventDefinition {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		let add = stream.read_i32()?;
		let remove = stream.read_i32()?;
		let raise = stream.read_i32()?;
		let custom_attribute_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		Ok(Self {
			name_index,
			type_index,
			add,
			remove,
			raise,
			custom_attribute_index,
			token,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let name_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		let add = stream.read_i32()?;
		let remove = stream.read_i32()?;
		let raise = stream.read_i32()?;
		let custom_attribute_index = stream.read_i32()?;
		let token = stream.read_u32()?;
		Ok(Self {
			name_index,
			type_index,
			add,
			remove,
			raise,
			custom_attribute_index,
			token,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 4 * 7;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppGenericContainer {
	pub owner_index: i32,
	pub type_argc: i32,
	pub is_method: i32,
	pub generic_parameter_start: i32,
}
impl Il2CppGenericContainer {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let owner_index = stream.read_i32()?;
		let type_argc = stream.read_i32()?;
		let is_method = stream.read_i32()?;
		let generic_parameter_start = stream.read_i32()?;
		Ok(Self {
			owner_index,
			type_argc,
			is_method,
			generic_parameter_start,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let owner_index = stream.read_i32()?;
		let type_argc = stream.read_i32()?;
		let is_method = stream.read_i32()?;
		let generic_parameter_start = stream.read_i32()?;
		Ok(Self {
			owner_index,
			type_argc,
			is_method,
			generic_parameter_start,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 16;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppGenericParameter {
	pub owner_index: i32,
	pub name_index: i32,
	pub constraints_start: i16,
	pub constraints_count: i16,
	pub num: u16,
	pub flags: u16,
}
impl Il2CppGenericParameter {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let owner_index = stream.read_i32()?;
		let name_index = stream.read_i32()?;
		let constraints_start = stream.read_i16()?;
		let constraints_count = stream.read_i16()?;
		let num = stream.read_u16()?;
		let flags = stream.read_u16()?;
		Ok(Self {
			owner_index,
			name_index,
			constraints_start,
			constraints_count,
			num,
			flags,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let owner_index = stream.read_i32()?;
		let name_index = stream.read_i32()?;
		let constraints_start = stream.read_i16()?;
		let constraints_count = stream.read_i16()?;
		let num = stream.read_u16()?;
		let flags = stream.read_u16()?;
		Ok(Self {
			owner_index,
			name_index,
			constraints_start,
			constraints_count,
			num,
			flags,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 16;
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppStringLiteral {
	pub length: u32,
	pub data_index: i32,
}
impl Il2CppStringLiteral {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let length = stream.read_u32()?;
		let data_index = stream.read_i32()?;
		Ok(Self { length, data_index })
	}
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppFieldDefaultValue {
	pub field_index: i32,
	pub type_index: i32,
	pub data_index: i32,
}
impl Il2CppFieldDefaultValue {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let field_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		let data_index = stream.read_i32()?;
		Ok(Self {
			field_index,
			type_index,
			data_index,
		})
	}
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppParameterDefaultValue {
	pub parameter_index: i32,
	pub type_index: i32,
	pub data_index: i32,
}
impl Il2CppParameterDefaultValue {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let parameter_index = stream.read_i32()?;
		let type_index = stream.read_i32()?;
		let data_index = stream.read_i32()?;
		Ok(Self {
			parameter_index,
			type_index,
			data_index,
		})
	}
}

#[derive(Debug, Clone, Default, Copy)]
#[wasm_bindgen]
pub struct Il2CppFieldRef {
	pub type_index: i32,
	pub field_index: i32,
}
impl Il2CppFieldRef {
	pub fn read(stream: &mut BinaryStream, _version: f64) -> Result<Self> {
		let type_index = stream.read_i32()?;
		let field_index = stream.read_i32()?;
		Ok(Self {
			type_index,
			field_index,
		})
	}
	pub fn read_codm(stream: &mut BinaryStream) -> Result<Self> {
		let type_index = stream.read_i32()?;
		let field_index = stream.read_i32()?;
		Ok(Self {
			type_index,
			field_index,
		})
	}
	pub const CODM_BYTE_SIZE: usize = 8;
}
