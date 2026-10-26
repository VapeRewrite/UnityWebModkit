/// WebAssembly binary patcher that can patch function calls in WASM binaries
/// while they are streaming in or have been fully loaded.
///
/// Based on the UnityWebModkit runtime (src/runtime/index.ts) which uses WAIL
/// (WebAssembly Instruction Array Language) for parsing and patching.

/// WASM magic number: 0x6d736100 ("\0asm")
pub const WASM_MAGIC: u32 = 0x6d736100;
/// WASM version: 1
pub const WASM_VERSION: u32 = 1;

/// Section IDs (matching JavaScript WAIL constants)
pub const SECTION_CUSTOM: u8 = 0;
pub const SECTION_TYPE: u8 = 1;
pub const SECTION_IMPORT: u8 = 2;
pub const SECTION_FUNCTION: u8 = 3;
pub const SECTION_TABLE: u8 = 4;
pub const SECTION_MEMORY: u8 = 5;
pub const SECTION_GLOBAL: u8 = 6;
pub const SECTION_EXPORT: u8 = 7;
pub const SECTION_START: u8 = 8;
pub const SECTION_ELEMENT: u8 = 9;
pub const SECTION_CODE: u8 = 10;
pub const SECTION_DATA: u8 = 11;
pub const SECTION_DATACOUNT: u8 = 12;

// OP codes (matching JavaScript WAIL constants)
pub const OP_UNREACHABLE: u8 = 0x00;
pub const OP_NOP: u8 = 0x01;
pub const OP_BLOCK: u8 = 0x02;
pub const OP_LOOP: u8 = 0x03;
pub const OP_IF: u8 = 0x04;
pub const OP_ELSE: u8 = 0x05;
pub const OP_END: u8 = 0x0b;
pub const OP_BR: u8 = 0x0c;
pub const OP_BR_IF: u8 = 0x0d;
pub const OP_BR_TABLE: u8 = 0x0e;
pub const OP_RETURN: u8 = 0x0f;
pub const OP_CALL: u8 = 0x10;
pub const OP_CALL_INDIRECT: u8 = 0x11;
pub const OP_DROP: u8 = 0x1a;
pub const OP_SELECT: u8 = 0x1b;
pub const OP_GET_LOCAL: u8 = 0x20;
pub const OP_SET_LOCAL: u8 = 0x21;
pub const OP_TEE_LOCAL: u8 = 0x22;
pub const OP_GET_GLOBAL: u8 = 0x23;
pub const OP_SET_GLOBAL: u8 = 0x24;
pub const OP_I32_LOAD: u8 = 0x28;
pub const OP_I64_LOAD: u8 = 0x29;
pub const OP_F32_LOAD: u8 = 0x2a;
pub const OP_F64_LOAD: u8 = 0x2b;
pub const OP_I32_LOAD8_S: u8 = 0x2c;
pub const OP_I32_LOAD8_U: u8 = 0x2d;
pub const OP_I32_LOAD16_S: u8 = 0x2e;
pub const OP_I32_LOAD16_U: u8 = 0x2f;
pub const OP_I64_LOAD8_S: u8 = 0x30;
pub const OP_I64_LOAD8_U: u8 = 0x31;
pub const OP_I64_LOAD16_S: u8 = 0x32;
pub const OP_I64_LOAD16_U: u8 = 0x33;
pub const OP_I64_LOAD32_S: u8 = 0x34;
pub const OP_I64_LOAD32_U: u8 = 0x35;
pub const OP_I32_STORE: u8 = 0x36;
pub const OP_I64_STORE: u8 = 0x37;
pub const OP_F32_STORE: u8 = 0x38;
pub const OP_F64_STORE: u8 = 0x39;
pub const OP_I32_STORE8: u8 = 0x3a;
pub const OP_I32_STORE16: u8 = 0x3b;
pub const OP_I64_STORE8: u8 = 0x3c;
pub const OP_I64_STORE16: u8 = 0x3d;
pub const OP_I64_STORE32: u8 = 0x3e;
pub const OP_MEMORY_SIZE: u8 = 0x3f;
pub const OP_MEMORY_GROW: u8 = 0x40;
pub const OP_I32_CONST: u8 = 0x41;
pub const OP_I64_CONST: u8 = 0x42;
pub const OP_F32_CONST: u8 = 0x43;
pub const OP_F64_CONST: u8 = 0x44;
pub const OP_I32_EQZ: u8 = 0x45;
pub const OP_I32_EQ: u8 = 0x46;
pub const OP_I32_NE: u8 = 0x47;
pub const OP_I32_LT_S: u8 = 0x48;
pub const OP_I32_LT_U: u8 = 0x49;
pub const OP_I32_GT_S: u8 = 0x4a;
pub const OP_I32_GT_U: u8 = 0x4b;
pub const OP_I32_LE_S: u8 = 0x4c;
pub const OP_I32_LE_U: u8 = 0x4d;
pub const OP_I32_GE_S: u8 = 0x4e;
pub const OP_I32_GE_U: u8 = 0x4f;
pub const OP_I64_EQZ: u8 = 0x50;
pub const OP_I64_EQ: u8 = 0x51;
pub const OP_I64_NE: u8 = 0x52;
pub const OP_I64_LT_S: u8 = 0x53;
pub const OP_I64_LT_U: u8 = 0x54;
pub const OP_I64_GT_S: u8 = 0x55;
pub const OP_I64_GT_U: u8 = 0x56;
pub const OP_I64_LE_S: u8 = 0x57;
pub const OP_I64_LE_U: u8 = 0x58;
pub const OP_I64_GE_S: u8 = 0x59;
pub const OP_I64_GE_U: u8 = 0x5a;
pub const OP_F32_EQ: u8 = 0x5b;
pub const OP_F32_NE: u8 = 0x5c;
pub const OP_F32_LT: u8 = 0x5d;
pub const OP_F32_GT: u8 = 0x5e;
pub const OP_F32_LE: u8 = 0x5f;
pub const OP_F32_GE: u8 = 0x60;
pub const OP_F64_EQ: u8 = 0x61;
pub const OP_F64_NE: u8 = 0x62;
pub const OP_F64_LT: u8 = 0x63;
pub const OP_F64_GT: u8 = 0x64;
pub const OP_F64_LE: u8 = 0x65;
pub const OP_F64_GE: u8 = 0x66;
pub const OP_I32_CLZ: u8 = 0x67;
pub const OP_I32_CTZ: u8 = 0x68;
pub const OP_I32_POPCNT: u8 = 0x69;
pub const OP_I32_ADD: u8 = 0x6a;
pub const OP_I32_SUB: u8 = 0x6b;
pub const OP_I32_MUL: u8 = 0x6c;
pub const OP_I32_DIV_S: u8 = 0x6d;
pub const OP_I32_DIV_U: u8 = 0x6e;
pub const OP_I32_REM_S: u8 = 0x6f;
pub const OP_I32_REM_U: u8 = 0x70;
pub const OP_I32_AND: u8 = 0x71;
pub const OP_I32_OR: u8 = 0x72;
pub const OP_I32_XOR: u8 = 0x73;
pub const OP_I32_SHL: u8 = 0x74;
pub const OP_I32_SHR_S: u8 = 0x75;
pub const OP_I32_SHR_U: u8 = 0x76;
pub const OP_I32_ROTL: u8 = 0x77;
pub const OP_I32_ROTR: u8 = 0x78;
pub const OP_I64_CLZ: u8 = 0x79;
pub const OP_I64_CTZ: u8 = 0x7a;
pub const OP_I64_POPCNT: u8 = 0x7b;
pub const OP_I64_ADD: u8 = 0x7c;
pub const OP_I64_SUB: u8 = 0x7d;
pub const OP_I64_MUL: u8 = 0x7e;
pub const OP_I64_DIV_S: u8 = 0x7f;
pub const OP_I64_DIV_U: u8 = 0x80;
pub const OP_I64_REM_S: u8 = 0x81;
pub const OP_I64_REM_U: u8 = 0x82;
pub const OP_I64_AND: u8 = 0x83;
pub const OP_I64_OR: u8 = 0x84;
pub const OP_I64_XOR: u8 = 0x85;
pub const OP_I64_SHL: u8 = 0x86;
pub const OP_I64_SHR_S: u8 = 0x87;
pub const OP_I64_SHR_U: u8 = 0x88;
pub const OP_I64_ROTL: u8 = 0x89;
pub const OP_I64_ROTR: u8 = 0x8a;
pub const OP_F32_ABS: u8 = 0x8b;
pub const OP_F32_NEG: u8 = 0x8c;
pub const OP_F32_CEIL: u8 = 0x8d;
pub const OP_F32_FLOOR: u8 = 0x8e;
pub const OP_F32_TRUNC: u8 = 0x8f;
pub const OP_F32_NEAREST: u8 = 0x90;
pub const OP_F32_SQRT: u8 = 0x91;
pub const OP_F32_ADD: u8 = 0x92;
pub const OP_F32_SUB: u8 = 0x93;
pub const OP_F32_MUL: u8 = 0x94;
pub const OP_F32_DIV: u8 = 0x95;
pub const OP_F32_MIN: u8 = 0x96;
pub const OP_F32_MAX: u8 = 0x97;
pub const OP_F32_COPYSIGN: u8 = 0x98;
pub const OP_F64_ABS: u8 = 0x99;
pub const OP_F64_NEG: u8 = 0x9a;
pub const OP_F64_CEIL: u8 = 0x9b;
pub const OP_F64_FLOOR: u8 = 0x9c;
pub const OP_F64_TRUNC: u8 = 0x9d;
pub const OP_F64_NEAREST: u8 = 0x9e;
pub const OP_F64_ADD: u8 = 0xa0;
pub const OP_F64_SUB: u8 = 0xa1;
pub const OP_F64_MUL: u8 = 0xa2;
pub const OP_F64_DIV: u8 = 0xa3;
pub const OP_F64_MIN: u8 = 0xa4;
pub const OP_F64_MAX: u8 = 0xa5;
pub const OP_F64_COPYSIGN: u8 = 0xa6;
pub const OP_I32_WRAP_I64: u8 = 0xa7;
pub const OP_I32_TRUNC_S_F32: u8 = 0xa8;
pub const OP_I32_TRUNC_U_F32: u8 = 0xa9;
pub const OP_I32_TRUNC_S_F64: u8 = 0xaa;
pub const OP_I32_TRUNC_U_F64: u8 = 0xab;
pub const OP_I64_EXTEND_S_I32: u8 = 0xac;
pub const OP_I64_EXTEND_U_I32: u8 = 0xad;
pub const OP_I64_TRUNC_S_F32: u8 = 0xae;
pub const OP_I64_TRUNC_U_F32: u8 = 0xaf;
pub const OP_I64_TRUNC_S_F64: u8 = 0xb0;
pub const OP_I64_TRUNC_U_F64: u8 = 0xb1;
pub const OP_F32_CONVERT_S_I32: u8 = 0xb2;
pub const OP_F32_CONVERT_U_I32: u8 = 0xb3;
pub const OP_F32_CONVERT_S_I64: u8 = 0xb4;
pub const OP_F32_CONVERT_U_I64: u8 = 0xb5;
pub const OP_F32_DEMOTE_F64: u8 = 0xb6;
pub const OP_F64_CONVERT_S_I32: u8 = 0xb7;
pub const OP_F64_CONVERT_U_I32: u8 = 0xb8;
pub const OP_F64_CONVERT_S_I64: u8 = 0xb9;
pub const OP_F64_CONVERT_U_I64: u8 = 0xba;
pub const OP_F64_PROMOTE_F32: u8 = 0xbb;
pub const OP_I32_REINTERPRET_F32: u8 = 0xbc;
pub const OP_I64_REINTERPRET_F64: u8 = 0xbd;
pub const OP_F32_REINTERPRET_I32: u8 = 0xbe;
pub const OP_F64_REINTERPRET_I64: u8 = 0xbf;
pub const OP_I32_EXTEND8_S: u8 = 0xc0;
pub const OP_I32_EXTEND16_S: u8 = 0xc1;
pub const OP_I64_EXTEND8_S: u8 = 0xc2;
pub const OP_I64_EXTEND16_S: u8 = 0xc3;
pub const OP_I64_EXTEND32_S: u8 = 0xc4;
pub const OP_BULK_MEMORY: u8 = 0xfc;
pub const OP_SIMD: u8 = 0xfd;
pub const OP_ATOMIC: u8 = 0xfe;

// Value types
pub const VALUE_TYPE_I32: u8 = 0x7f;
pub const VALUE_TYPE_I64: u8 = 0x7e;
pub const VALUE_TYPE_F32: u8 = 0x7d;
pub const VALUE_TYPE_F64: u8 = 0x7c;
pub const VALUE_TYPE_ANYFUNC: u8 = 0x70;
pub const VALUE_TYPE_FUNC: u8 = 0x60;
pub const VALUE_TYPE_BLOCK: u8 = 0x40;

// A parsed WASM function entry with its type index and code
#[derive(Debug, Clone)]
pub struct WasmFunction {
	/// Index into the type section
	pub type_index: u32,
	/// Raw code bytes (locals + body)
	pub code: Vec<u8>,
	/// Number of local declarations
	pub num_locals: u32,
	/// Number of local declarations (mutable)
	pub num_locals_mut: u32,
}

/// A parsed WASM import entry
#[derive(Debug, Clone)]
pub struct WasmImport {
	/// Module name
	pub module: String,
	/// Field/field name
	pub field: String,
	/// Kind (0=func, 1=table, 2=memory, 3=global)
	pub kind: u8,
	/// Type index (for func imports)
	pub type_index: u32,
}

/// A parsed WASM export entry
#[derive(Debug, Clone)]
pub struct WasmExport {
	/// Field/field name
	pub field: String,
	/// Index
	pub index: u32,
}

/// A parsed WASM table entry
#[derive(Debug, Clone)]
pub struct WasmTable {
	/// Minimum initial element count
	pub minimum: u32,
	/// Maximum element count (0 for fixed)
	pub maximum: u32,
	/// Element type (funcref or anyfuncref)
	pub element_type: u8,
}

/// A parsed WASM memory entry
#[derive(Debug, Clone)]
pub struct WasmMemory {
	/// Minimum initial page count
	pub minimum: u32,
	/// Maximum page count (0 for fixed)
	pub maximum: u32,
}

/// A parsed WASM global entry
#[derive(Debug, Clone)]
pub struct WasmGlobal {
	/// Value type (i32, f32, etc.)
	pub value_type: u8,
	/// Whether mutable
	pub mutable: bool,
}

/// Result of parsing a WASM binary
#[derive(Debug)]
pub struct WasmParseResult {
	/// Parsed type section entries
	pub types: Vec<Vec<u8>>, // Each entry is (params, results) flattened
	/// Parsed import entries
	pub imports: Vec<WasmImport>,
	/// Parsed function entries (type indices)
	pub functions: Vec<u32>,
	/// Parsed table entries
	pub tables: Vec<WasmTable>,
	/// Parsed memory entries
	pub memories: Vec<WasmMemory>,
	/// Parsed global entries
	pub globals: Vec<WasmGlobal>,
	/// Parsed export entries
	pub exports: Vec<WasmExport>,
	/// Parsed function code entries
	pub function_codes: Vec<WasmFunction>,
	/// Original binary data (for reference)
	pub original_data: Vec<u8>,
}

/// A parsed modified UTF-8 string from the WASM binary
pub struct WasmString {
	/// The string content
	pub string: String,
}
