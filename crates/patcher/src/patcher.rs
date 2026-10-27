use wasm_bindgen::prelude::wasm_bindgen;

use crate::models::*;
use crate::parser::read_var_uint32;
use crate::writer::write_var_uint32_to_buffer;
use std::collections::HashMap;

// Remove the duplicate constants and functions from lib.rs
// This file contains the patching logic that was originally in lib.rs

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct IndexMap {
	pub old_idx: u32,
	pub new_idx: u32
}

/// Apply OP_CALL patching to a WASM binary
///
/// This function takes a WASM binary and parallel arrays of old and new function indices,
/// and returns a new WASM binary with the function calls patched.
#[wasm_bindgen]
pub fn patch_calls_in_wasm(data: &[u8], old_indices: Vec<u32>, new_indices: Vec<u32>) -> Result<Vec<u8>, String> {
	if data.len() < 8 {
		return Err("WASM binary too short".to_string());
	}

	// Check magic number
	let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
	if magic != WASM_MAGIC {
		return Err(format!("Invalid WASM magic: 0x{:08x}", magic));
	}

	// Build a lookup map: old_index -> new_index
	let mut index_map: HashMap<u32, u32> = HashMap::new();
	for i in 0..old_indices.len().min(new_indices.len()) {
		index_map.insert(old_indices[i], new_indices[i]);
	}

	// Build the patched binary
	let mut result = data.to_vec();
	let mut offset: usize = 8; // Skip magic + version

	while offset < result.len() {
		let section_id = result[offset];
		offset += 1;

		if offset >= result.len() {
			break;
		}

		let payload_len = read_var_uint32(&result, &mut offset) as usize;

		if offset + payload_len > result.len() {
			break;
		}

		let section_start = offset;
		let section_end_val = offset + payload_len; // Save end value

		match section_id {
			SECTION_CODE => {
				// Parse function bodies and patch OP_CALL instructions
				let num_functions = read_var_uint32(&result, &mut offset) as usize;

				for _ in 0..num_functions {
					// Skip type index varuint32 (first thing in code section entry)
					let _type_idx = read_var_uint32(&result, &mut offset);

					// Read local declarations count
					// read_var_uint32 already advances the offset, so no need to add 4
					let local_count = read_var_uint32(&result, &mut offset) as u32;

					// Skip local types
					for _ in 0..local_count {
						let _local_type = result[offset];
						offset += 1;
					}

					// Now we're at the function code
					// The code is a sequence of opcodes ending with OP_END
					let code_start = offset;
					let code_end = section_end_val;

					// Process the code to find and patch OP_CALL instructions
					let mut code_offset = code_start;
					while code_offset < code_end {
						if code_offset >= result.len() {
							break;
						}

						let opcode = result[code_offset];

						if opcode == OP_CALL {
							// OP_CALL format: 0x10 followed by varuint32 target
							// Read the varuint32 target starting at code_offset + 1
							let target_start = code_offset + 1;
							if target_start >= result.len() {
								code_offset += 1;
								continue;
							}

							// Read varuint32 from the result buffer
							let mut temp_offset = target_start;
							let mut call_target: u32 = 0;
							let mut read_shift: u32 = 0;

							while temp_offset < result.len() {
								let byte = result[temp_offset] as u32;
								call_target |= (byte & 0x7f) << read_shift;
								read_shift += 7;
								temp_offset += 1;
								if (byte & 0x80) == 0 {
									break;
								}
							}

							// Check if this target is in our patch map
							if let Some(&new_idx) = index_map.get(&call_target) {
								// Patch the call target
								// We need to write the new varuint32 in place
								let mut write_offset = target_start;
								let value = new_idx;

								if value == 0 {
									result[write_offset] = 0;
									write_offset += 1;
								} else {
									let mut bytes: Vec<u8> = Vec::new();
									let mut v = value;
									while v > 0 {
										bytes.push(((v & 0x7f) | 0x80) as u8);
										v >>= 7;
									}
									// Last byte: clear continuation bit
									*bytes.last_mut().unwrap() &= 0x7f;
									bytes.reverse();
									for (i, b) in bytes.iter().enumerate() {
										if write_offset + i < result.len() {
											result[write_offset + i] = *b;
										}
									}
								}
							}

							// Advance past the OP_CALL and its target
							code_offset = temp_offset;
						} else if opcode == OP_CALL_INDIRECT {
							// OP_CALL_INDIRECT: 0x11 followed by table index (varuint32)
							// We'll skip this for now as it's less common
							let mut indirect_offset = code_offset + 1;
							if indirect_offset < result.len() {
								// Read varuint32 for table index
								let mut table_idx: u32 = 0;
								let mut read_shift: u32 = 0;
								while indirect_offset < result.len() {
									let byte = result[indirect_offset] as u32;
									table_idx |= (byte & 0x7f) << read_shift;
									read_shift += 7;
									indirect_offset += 1;
									if (byte & 0x80) == 0 {
										break;
									}
								}
							}
							code_offset = indirect_offset;
						} else {
							// Regular opcode, advance by 1
							code_offset += 1;
						}
					}
				}

				// After processing CODE section, jump to section end
				offset = section_end_val;
				continue;
			}
			_ => {
				// Skip to end of section
				offset = section_end_val;
			}
		}
	}

	Ok(result)
}

/// Create a WASM instruction patch for OP_CALL
///
/// This creates a patch pattern to replace function call targets.
pub fn make_call_patch(old_index: u32, new_index: u32) -> (Vec<u8>, Vec<u8>) {
	// OP_CALL byte: 0x10
	// Followed by varuint32 target

	// Pattern: [0x10] + varuint32(old_index)
	let mut pattern: Vec<u8> = Vec::new();
	pattern.push(OP_CALL);
	write_var_uint32_to_buffer(&mut pattern, old_index);

	// Replacement: [0x10] + varuint32(new_index)
	let mut replacement: Vec<u8> = Vec::new();
	replacement.push(OP_CALL);
	write_var_uint32_to_buffer(&mut replacement, new_index);

	(pattern, replacement)
}

/// Patch a WASM binary for streaming instantiation.
///
/// This function is designed to be used with `WebAssembly.instantiateStreaming`.
/// It takes the full WASM binary data and parallel arrays of old and new function indices,
/// and returns a new WASM binary with the function calls patched.
///
/// This is the streaming variant of `patch_calls_in_wasm` - it accepts the same parameters
/// but is intended to be called on accumulated stream chunks rather than the full response
/// up front.
#[wasm_bindgen]
pub fn patch_wasm_for_streaming(data: &[u8], old_indices: Vec<u32>, new_indices: Vec<u32>) -> Result<Vec<u8>, String> {
	patch_calls_in_wasm(data, old_indices, new_indices)
}

/// Find a function index by searching function bodies for a byte pattern.
///
/// Searches the CODE section for function bodies containing the given byte pattern
/// and returns the function index (relative to the start of defined functions in
/// the CODE section) of the first match.
#[wasm_bindgen]
pub fn find_function_by_pattern(data: &[u8], pattern: &[u8]) -> Option<u32> {
	let mut offset = 8;

	while offset < data.len() {
		let section_id = data[offset];
		offset += 1;

		if offset >= data.len() {
			break;
		}

		let payload_len = read_var_uint32(data, &mut offset) as usize;

		if offset + payload_len > data.len() {
			break;
		}

		let section_start = offset;
		let section_end = offset + payload_len;

		if section_id == SECTION_CODE {
			let mut code_offset = section_start;
			let num_functions = read_var_uint32(data, &mut code_offset) as usize;

			for func_idx in 0..num_functions {
				let body_size = read_var_uint32(data, &mut code_offset) as usize;
				let body_start = code_offset;
				let body_end = code_offset + body_size;

				// Search for pattern in this function's body
				let mut search_offset = body_start;
				while search_offset + pattern.len() <= body_end {
					let mut match_found = true;
					for i in 0..pattern.len() {
						if data[search_offset + i] != pattern[i] {
							match_found = false;
							break;
						}
					}
					if match_found {
						return Some(func_idx as u32);
					}
					search_offset += 1;
				}

				code_offset = body_end;
			}
		}

		offset = section_end;
	}

	None
}

/// Add export entries to a WASM binary's export section.
///
/// If no export section exists, creates one. Each export entry maps a function
/// index to a name string. Kind is always 0 (func).
#[wasm_bindgen]
pub fn add_wasm_exports(
	data: &[u8],
	export_indices: Vec<u32>,
	export_names: Vec<String>,
) -> Result<Vec<u8>, String> {
	if data.len() < 8 {
		return Err("WASM binary too short".to_string());
	}

	let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
	if magic != WASM_MAGIC {
		return Err(format!("Invalid WASM magic: 0x{:08x}", magic));
	}

	// Parse all sections into (id, payload) tuples
	let mut sections: Vec<(u8, Vec<u8>)> = Vec::new();
	let mut offset = 8;

	while offset < data.len() {
		let section_id = data[offset];
		offset += 1;

		if offset >= data.len() {
			break;
		}

		let payload_len = read_var_uint32(data, &mut offset) as usize;

		if offset + payload_len > data.len() {
			break;
		}

		let payload = data[offset..offset + payload_len].to_vec();
		sections.push((section_id, payload));
		offset += payload_len;
	}

	// Build new export entries bytes
	let mut new_entries_payload = Vec::new();
	let num_new = export_indices.len().min(export_names.len());
	for i in 0..num_new {
		let name_bytes = export_names[i].as_bytes();
		write_var_uint32_to_buffer(&mut new_entries_payload, name_bytes.len() as u32);
		new_entries_payload.extend_from_slice(name_bytes);
		new_entries_payload.push(0x00); // kind = func
		write_var_uint32_to_buffer(&mut new_entries_payload, export_indices[i]);
	}

	// Find or create export section
	let export_section_idx = sections.iter().position(|(id, _)| *id == SECTION_EXPORT);

	if let Some(idx) = export_section_idx {
		// Parse existing exports and add new ones
		let mut payload = sections[idx].1.clone();
		let mut p_offset = 0;
		let num_exports = read_var_uint32(&payload, &mut p_offset);

		// Build new payload
		let mut new_payload = Vec::new();
		write_var_uint32_to_buffer(&mut new_payload, num_exports + num_new as u32);

		// Copy existing exports (everything after the count varuint32)
		new_payload.extend_from_slice(&payload[p_offset..]);

		// Append new entries
		new_payload.extend_from_slice(&new_entries_payload);

		sections[idx] = (SECTION_EXPORT, new_payload);
	} else {
		// Create new export section
		let mut payload = Vec::new();
		write_var_uint32_to_buffer(&mut payload, num_new as u32);
		payload.extend_from_slice(&new_entries_payload);
		sections.push((SECTION_EXPORT, payload));
	}

	// Rebuild binary
	let mut result = Vec::new();
	result.extend_from_slice(&WASM_MAGIC.to_le_bytes());
	result.extend_from_slice(&WASM_VERSION.to_le_bytes());

	for (section_id, payload) in &sections {
		result.push(*section_id);
		write_var_uint32_to_buffer(&mut result, payload.len() as u32);
		result.extend_from_slice(payload);
	}

	Ok(result)
}

/// Complete WASM patching: OP_CALL targets + export entries in one pass.
///
/// This function takes a WASM binary, parallel arrays of old/new function indices
/// for OP_CALL patching, and parallel arrays of export indices/names for adding
/// export entries. It returns a single patched binary with both modifications
/// applied, suitable for passing directly to WebAssembly.instantiate or
/// WebAssembly.instantiateStreaming.
#[wasm_bindgen]
pub fn patch_wasm_complete(
	data: &[u8],
	old_indices: Vec<u32>,
	new_indices: Vec<u32>,
	export_indices: Vec<u32>,
	export_names: Vec<String>,
) -> Result<Vec<u8>, String> {
	// First patch OP_CALL targets
	let step1 = patch_calls_in_wasm(data, old_indices, new_indices)?;

	// Then add exports
	add_wasm_exports(&step1, export_indices, export_names)
}
