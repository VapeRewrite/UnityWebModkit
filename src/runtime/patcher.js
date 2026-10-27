/* @ts-self-types="../../crates/patcher/pkg/patcher.d.ts" */
import * as wasm from "../../crates/patcher/pkg/patcher_bg.wasm?sync";
import { __wbg_set_wasm } from "../../crates/patcher/pkg/patcher_bg.js";

__wbg_set_wasm(wasm);
export {
	IndexMap,
	WasmExport,
	WasmFunction,
	WasmGlobal,
	WasmImport,
	WasmMemory,
	WasmParseResult,
	WasmTable,
	add_wasm_exports,
	find_function_by_pattern,
	parse_wasm_binary,
	patch_calls_in_wasm,
	patch_wasm_complete,
	patch_wasm_for_streaming,
	write_var_uint32_to_buffer,
} from "../../crates/patcher/pkg/patcher_bg";
