use crate::models::*;

/// Parse a WASM binary and extract all sections
pub fn parse_wasm_binary(data: &[u8]) -> Result<WasmParseResult, String> {
	if data.len() < 8 {
		return Err("WASM binary too short".to_string());
	}

	// Check magic number
	let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
	if magic != WASM_MAGIC {
		return Err(format!("Invalid WASM magic: 0x{:08x}", magic));
	}

	// Check version
	let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
	if version != WASM_VERSION {
		return Err(format!("Invalid WASM version: {}", version));
	}

	let mut offset: usize = 8;
	let mut types: Vec<Vec<u8>> = Vec::new();
	let mut imports: Vec<WasmImport> = Vec::new();
	let mut functions: Vec<u32> = Vec::new();
	let mut tables: Vec<WasmTable> = Vec::new();
	let mut memories: Vec<WasmMemory> = Vec::new();
	let mut globals: Vec<WasmGlobal> = Vec::new();
	let mut exports: Vec<WasmExport> = Vec::new();
	let mut function_codes: Vec<WasmFunction> = Vec::new();

	// Parse all sections
	while offset < data.len() {
		let section_id = data[offset];
		offset += 1;

		if offset >= data.len() {
			break;
		}

		let payload_len = read_var_uint32(data, &mut offset) as usize;

		// Sanity check
		if offset + payload_len > data.len() {
			return Err("WASM section payload extends beyond binary".to_string());
		}

		let section_start = offset;
		let section_end = offset + payload_len;

		match section_id {
			SECTION_TYPE => {
				// Parse type section
				let num_types = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_types {
					// Function type: params (value types) then results (value types)
					let param_count = read_var_uint32(data, &mut offset) as usize;
					let mut param_types: Vec<u8> = Vec::with_capacity(param_count);
					for _ in 0..param_count {
						param_types.push(data[offset]);
						offset += 1;
					}
					let result_count = read_var_uint32(data, &mut offset) as usize;
					let mut result_types: Vec<u8> = Vec::with_capacity(result_count);
					for _ in 0..result_count {
						result_types.push(data[offset]);
						offset += 1;
					}
					// Combine params and results into a single vec for simplicity
					let type_entry: Vec<u8> = param_types
						.into_iter()
						.chain(result_types.into_iter())
						.collect();
					types.push(type_entry);
				}
			}
			SECTION_IMPORT => {
				// Parse import section
				let num_imports = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_imports {
					let module_len = read_var_uint32(data, &mut offset) as usize;
					let module = read_utf8_string(data, &mut offset, module_len);

					let field_len = read_var_uint32(data, &mut offset) as usize;
					let field = read_utf8_string(data, &mut offset, field_len);

					let kind = data[offset];
					offset += 1;

					let type_index = read_var_uint32(data, &mut offset);

					imports.push(WasmImport {
						module,
						field,
						kind,
						type_index,
					});
				}
			}
			SECTION_FUNCTION => {
				// Parse function section
				let num_functions = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_functions {
					let func_type_index = read_var_uint32(data, &mut offset);
					functions.push(func_type_index);
				}
			}
			SECTION_TABLE => {
				// Parse table section
				let num_tables = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_tables {
					let table_type = data[offset];
					offset += 1;
					let minimum = read_var_uint32(data, &mut offset);
					let maximum = read_var_uint32(data, &mut offset);
					let element_type = data[offset];
					offset += 1;

					tables.push(WasmTable {
						minimum,
						maximum,
						element_type,
					});
				}
			}
			SECTION_MEMORY => {
				// Parse memory section
				let num_memories = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_memories {
					let minimum = read_var_uint32(data, &mut offset);
					let maximum = read_var_uint32(data, &mut offset);

					memories.push(WasmMemory { minimum, maximum });
				}
			}
			SECTION_GLOBAL => {
				// Parse global section
				let num_globals = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_globals {
					let value_type = data[offset];
					offset += 1;
					let mutable = data[offset] != 0;
					offset += 1;

					globals.push(WasmGlobal {
						value_type,
						mutable,
					});
				}
			}
			SECTION_EXPORT => {
				// Parse export section
				let num_exports = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_exports {
					let field_len = read_var_uint32(data, &mut offset) as usize;
					let field = read_utf8_string(data, &mut offset, field_len);
					offset += field_len;

					let kind = data[offset];
					offset += 1;

					let index = read_var_uint32(data, &mut offset);

					exports.push(WasmExport { field, index });
				}
			}
			SECTION_START => {
				// Parse start section - just read the function index
				let _start_func = read_var_uint32(data, &mut offset);
				// We'll ignore the start function for now
			}
			SECTION_ELEMENT => {
				// Parse element section
				let num_elements = read_var_uint32(data, &mut offset) as usize;
				for _ in 0..num_elements {
					let type_ = data[offset];
					offset += 1;
					let num_funcs = read_var_uint32(data, &mut offset) as usize;
					let func_index_base = read_var_uint32(data, &mut offset);
					// Skip function indices
					for _ in 0..num_funcs {
						let _func_idx = read_var_uint32(data, &mut offset);
					}
				}
			}
			SECTION_CODE => {
				// Parse code section - function bodies
				let num_functions = read_var_uint32(data, &mut offset) as usize;

				for _ in 0..num_functions {
					// Skip type index varuint32 (first thing in code section entry)
					let _type_idx = read_var_uint32(data, &mut offset);

					// Read local declarations count
					// read_var_uint32 already advances the offset, so no need to add 4
					let local_count = read_var_uint32(data, &mut offset) as u32;

					// Skip local types
					for _ in 0..local_count {
						let _local_type = data[offset];
						offset += 1;
					}

					// Store function code info
					let func = WasmFunction {
						type_index: 0, // Will be set if needed
						code: Vec::new(),
						num_locals: 0,
						num_locals_mut: 0,
					};
					function_codes.push(func);
				}

				// After processing CODE section, jump to section end
				offset = section_end;
				continue;
			}
			_ => {
				// Custom or unknown section - skip payload
				offset = section_end;
				continue;
			}
		}

		// After each section, set offset to section_end
		offset = section_end;
	}

	Ok(WasmParseResult {
		types,
		imports,
		functions,
		tables,
		memories,
		globals,
		exports,
		function_codes,
		original_data: data.to_vec(),
	})
}

/// Parse a modified UTF-8 string from the WASM binary
fn read_utf8_string(data: &[u8], offset: &mut usize, len: usize) -> String {
	let start = *offset;
	let string_bytes = &data[start..start + len];
	*offset += len;
	// Decode as UTF-8, replacing invalid bytes
	String::from_utf8_lossy(string_bytes).to_string()
}

/// Read a varuint32 from the data at the given offset, advancing the offset
pub fn read_var_uint32(data: &[u8], offset: &mut usize) -> u32 {
	let mut result: u32 = 0;
	let mut shift: u32 = 0;
	let mut i = *offset;

	loop {
		if i >= data.len() {
			break;
		}
		let byte = data[i] as u32;
		result |= (byte & 0x7f) << shift;
		shift += 7;
		i += 1;
		if (byte & 0x80) == 0 {
			break;
		}
	}

	*offset = i;
	result
}
