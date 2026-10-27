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
/// This function takes a WASM binary and a mapping of old function indices to new ones,
/// and returns a new WASM binary with the function calls patched.
#[wasm_bindgen]
pub fn patch_calls_in_wasm(data: &[u8], func_index_map: Vec<IndexMap>) -> Result<Vec<u8>, String> {
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
	for IndexMap { old_idx, new_idx } in func_index_map {
		index_map.insert(old_idx, new_idx);
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
