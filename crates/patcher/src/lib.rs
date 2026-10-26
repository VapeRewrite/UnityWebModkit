//! WebAssembly binary patcher that can patch function calls in WASM binaries
//! while they are streaming in or have been fully loaded.
//!
//! Based on the UnityWebModkit runtime which uses WAIL (WebAssembly Instruction
//! Array Language) for parsing and patching.
//!
//! # Examples
//!
//! ```
//! use patcher::{parse_wasm_binary, patch_calls_in_wasm, make_call_patch};
//!
//! let binary: Vec<u8> = vec![];
//! ```
//!
//! ## Modules
//!
//! - `parser` - WASM binary section parsing
//! - `writer` - varuint32 encoding/decoding
//! - `patcher` - OP_CALL instruction patching
//! - `models` - Data structures for parsed WASM

/// WebAssembly magic number: 0x6d736100 ("\0asm")
pub const WASM_MAGIC: u32 = 0x6d736100;
/// WASM version: 1
pub const WASM_VERSION: u32 = 1;

/// Section IDs (matching JavaScript WAIL constants)
pub use crate::models::*;
/// OP codes
pub use crate::models::{OP_CALL, OP_CALL_INDIRECT, VALUE_TYPE_I32};

/// Parse a WASM binary and extract all sections
pub use crate::parser::parse_wasm_binary;
/// Create a WASM instruction patch for OP_CALL
pub use crate::patcher::make_call_patch;
/// Apply OP_CALL patching to a WASM binary
pub use crate::patcher::patch_calls_in_wasm;
// Re-export commonly used types
pub use crate::models::WasmExport;
pub use crate::models::WasmFunction;
pub use crate::models::WasmGlobal;
pub use crate::models::WasmImport;
pub use crate::models::WasmMemory;
pub use crate::models::WasmParseResult;
pub use crate::models::WasmString;
pub use crate::models::WasmTable;

/// Read a varuint32 from the data at the given offset, advancing the offset
pub use crate::parser::read_var_uint32;
/// Write a varuint32 to a buffer
pub use crate::writer::write_var_uint32_to_buffer;

// === Internal module re-exports for crate users ===

mod models;
mod parser;
mod patcher;
mod writer;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_parsing() {
		let mut wasm_binary = Vec::new();

		// Magic number
		wasm_binary.extend_from_slice(&WASM_MAGIC.to_le_bytes());
		// Version
		wasm_binary.extend_from_slice(&WASM_VERSION.to_le_bytes());

		// Type section (id=1)
		wasm_binary.push(crate::models::SECTION_TYPE);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 5); // payload length
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // num_types = 1
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // param_count = 1
		wasm_binary.push(crate::models::VALUE_TYPE_I32); // param type
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // result_count = 1
		wasm_binary.push(crate::models::VALUE_TYPE_I32); // result type

		// Function section (id=3)
		wasm_binary.push(crate::models::SECTION_FUNCTION);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 2); // payload length
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // num_functions = 1
		wasm_binary.push(0); // function 0 uses type 0

		// Code section (id=10)
		wasm_binary.push(crate::models::SECTION_CODE);
		// Payload: 1 (num_functions) + 1 (type_index) + 1 (local_count) + 6 (code body) = 9
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 9);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // num_functions = 1

		// Function body: type index = 0, local count = 0, then code body
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 0); // type index = 0
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 0); // local count = 0

		// Code body: i32.const 42, i32.const 10, i32.add, end
		wasm_binary.push(crate::models::OP_I32_CONST);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 42);
		wasm_binary.push(crate::models::OP_I32_CONST);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 10);
		wasm_binary.push(crate::models::OP_I32_ADD);
		wasm_binary.push(crate::models::OP_END);

		// Parse the binary
		let parsed = parse_wasm_binary(&wasm_binary);
		assert!(parsed.is_ok(), "Parsing should succeed");
		let parsed = parsed.unwrap();
		assert_eq!(parsed.functions.len(), 1, "Should have 1 function");
		assert_eq!(parsed.function_codes.len(), 1, "Should have 1 code entry");
	}

	#[test]
	fn test_var_uint32_roundtrip() {
		let mut buf: Vec<u8> = Vec::new();
		write_var_uint32_to_buffer(&mut buf, 0);
		write_var_uint32_to_buffer(&mut buf, 1);
		write_var_uint32_to_buffer(&mut buf, 128);
		write_var_uint32_to_buffer(&mut buf, 256);
		write_var_uint32_to_buffer(&mut buf, 1000);
		write_var_uint32_to_buffer(&mut buf, 0xFFFFFFFF);

		// Should have written something
		assert!(!buf.is_empty(), "Buffer should not be empty");
		assert!(buf.len() > 0);
	}

	#[test]
	fn test_patch_call_instruction() {
		let mut wasm_binary: Vec<u8> = Vec::new();

		// Magic + version
		wasm_binary.extend_from_slice(&WASM_MAGIC.to_le_bytes());
		wasm_binary.extend_from_slice(&WASM_VERSION.to_le_bytes());

		// Type section - 1 type (i32 -> i32)
		wasm_binary.push(crate::models::SECTION_TYPE);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 5);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // 1 param
		wasm_binary.push(crate::models::VALUE_TYPE_I32);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1); // 1 result
		wasm_binary.push(crate::models::VALUE_TYPE_I32);

		// Function section - 1 function
		wasm_binary.push(crate::models::SECTION_FUNCTION);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 2);
		wasm_binary.push(0); // uses type 0

		// Code section
		wasm_binary.push(crate::models::SECTION_CODE);
		// Correct payload length: 6
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 6);

		// num_functions = 1
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 1);
		// type index varuint32 = 0
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 0); // type index = 0
		// Local declarations count = 0 (varuint32)
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 0); // local count = 0
		// OP_CALL = 0x10, followed by varuint32 target = 0
		wasm_binary.push(crate::models::OP_CALL);
		crate::writer::write_var_uint32_to_buffer(&mut wasm_binary, 0);
		wasm_binary.push(crate::models::OP_END);

		// Now patch: replace function index 0 -> 5
		let func_map = [(0, 5)];
		let patched = patch_calls_in_wasm(&wasm_binary, &func_map);

		match patched {
			Ok(data) => {
				// Verify the patch was applied
				let mut found_opcall = false;
				let mut found_new_index = false;

				for i in 0..data.len() - 1 {
					if data[i] == OP_CALL {
						found_opcall = true;
						let mut target: u32 = 0;
						let mut j = i + 1;
						let mut shift: u32 = 0;
						while j < data.len() {
							let byte = data[j] as u32;
							target |= (byte & 0x7f) << shift;
							shift += 7;
							j += 1;
							if (byte & 0x80) == 0 {
								break;
							}
						}
						if target == 5 {
							found_new_index = true;
							break;
						}
					}
				}

				assert!(found_opcall, "OP_CALL should be present in patched binary");
				assert!(found_new_index, "Function index should be patched to 5");
			}
			Err(e) => {
				panic!("Patch failed: {}", e);
			}
		}
	}

	#[test]
	fn test_make_call_patch() {
		let (pattern, replacement) = make_call_patch(0, 5);

		// Pattern should start with OP_CALL (0x10)
		assert_eq!(pattern[0], OP_CALL, "Pattern should start with OP_CALL");
		// The varuint32 after should be 0
		let mut shift: u32 = 0;
		let mut target: u32 = 0;
		let mut j = 1;
		while j < pattern.len() {
			let byte = pattern[j] as u32;
			target |= (byte & 0x7f) << shift;
			shift += 7;
			j += 1;
			if (byte & 0x80) == 0 {
				break;
			}
		}
		assert_eq!(target, 0, "Pattern target should be 0");

		// Replacement should start with OP_CALL (0x10)
		assert_eq!(
			replacement[0], OP_CALL,
			"Replacement should start with OP_CALL"
		);
		// The varuint32 after should be 5
		let mut shift2: u32 = 0;
		let mut target2: u32 = 0;
		let mut j = 1;
		while j < replacement.len() {
			let byte = replacement[j] as u32;
			target2 |= (byte & 0x7f) << shift2;
			shift2 += 7;
			j += 1;
			if (byte & 0x80) == 0 {
				break;
			}
		}
		assert_eq!(target2, 5, "Replacement target should be 5");
	}
}
