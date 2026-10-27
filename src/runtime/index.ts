import { Logger } from "../logger";
import { UnresolvedMetadataError } from "../errors";
import { createIl2CppContext, createMetadata, Il2CppContext, Il2CppMetadata } from "../il2cpp";
import { WebData } from "../web-data";
import { preload } from "../preloader";
import { bufToHex, waitFor, writeUint8ArrayAtOffset } from "../utils";
import { BinaryReader, BinaryWriter } from "../utils/binary";
import { dataTypeSizes } from "../extras";
import init, { patch_wasm_complete, find_function_by_pattern } from "../../crates/patcher/pkg/patcher";

// Function index mapping: old index -> new index for WASM call patching
// Built from plugin hook mappings (old -> new redirects)
let funcIndexMap: [number, number][] = [];

function getGame() {
	return "game" in window &&
		window.game != null &&
		typeof window.game === "object" &&
		"instance" in window.game
		? window.game.instance
		: "unityInstance" in window
			? window.unityInstance
			: (undefined as never);
}

export class Runtime {
	public tableName: string | undefined;
	private logger: Logger;
	private plugins: ModkitPlugin[] = [];
	private startedInitializing = false;
	private allReferencedAssemblies: string[] = [];
	private globalMetadata: Il2CppMetadata | null | undefined;
	private il2CppContext: Il2CppContext | undefined;
	private resolvedIl2CppFunctions: Record<string, number> = {};
	private instantiateStreaming: any;
	private internalMappings: any;

	get game(): any {
		return getGame();
	}

	public constructor() {
		this.logger = new Logger("UnityWebModkit");
	}

	public createPlugin(opts: ModkitPluginOptions): ModkitPlugin {
		if (!this.startedInitializing) this.initialize();
		const plugin = new ModkitPlugin(opts.name, opts.version, opts.referencedAssemblies, this);
		this.plugins.push(plugin);
		return plugin;
	}

	private async initialize(): Promise<void> {
		if (typeof window === "undefined") {
			console.log(
				"\x1b[37m[UnityWebModkit]\x1b[0m \x1b[33m[WARN]\x1b[0m Not running in a browser environment! Nothing will be executed.",
			);
			return;
		}
		this.startedInitializing = true;
		this.hookWasmInstantiate();
		const webData = await preload();
		this.logger.debug("Parsed web data into %d node(s)", webData.nodes.length);
		if (webData.unityVersion) {
			this.logger.info("Running under Unity %s", webData.unityVersion);
		} else {
			this.logger.warn("Unable to determine Unity version from web data!");
		}
		this.readGlobalMetadataFromStorage(webData).catch(() => {
			window.indexedDB.deleteDatabase("UnityWebModkit");
			this.loadGlobalMetadata(webData);
		});
	}

	private async loadGlobalMetadata(webData: WebData) {
		const metadataNode = webData.getNode("Il2CppData/Metadata/global-metadata.dat");
		if (!metadataNode || !metadataNode.data) {
			this.logger.error(
				new UnresolvedMetadataError(
					"Unable to find global-metadata.dat! The game may be encrypted, corrupt or unsupported.",
				).print(),
			);
			return;
		}
		this.allReferencedAssemblies = this.plugins.flatMap(
			(plugin) => plugin.referencedAssemblies,
		);
		const globalMetadata = await createMetadata(
			metadataNode.data,
			this.allReferencedAssemblies,
		);
		if (globalMetadata.isErr()) {
			this.logger.error(globalMetadata.error.print());
			return;
		}
		this.globalMetadata = globalMetadata.value;
		this.saveGlobalMetadata();
	}

	private saveGlobalMetadata() {
		const request = window.indexedDB.open("UnityWebModkit", 2);
		request.onupgradeneeded = () => {
			const db = request.result;
			const objectStore = db.createObjectStore("storage", {
				keyPath: "name",
			});
			objectStore.createIndex("name", "name", { unique: true });
			objectStore.transaction.oncomplete = () => {
				const storageObjectStore = db
					.transaction("storage", "readwrite")
					.objectStore("storage");
				storageObjectStore.add(this.globalMetadata);
			};
		};
	}

	private saveIl2CppContext() {
		const request = window.indexedDB.open("UnityWebModkit", 2);
		request.onsuccess = () => {
			const db = request.result;
			const storageObjectStore = db
				.transaction("storage", "readwrite")
				.objectStore("storage");
			storageObjectStore.add(this.il2CppContext);
		};
	}

	private readGlobalMetadataFromStorage(webData: WebData): Promise<void> {
		return new Promise<void>((resolve, reject) => {
			indexedDB.databases().then(async (databases) => {
				const uwmStore = databases.findIndex((d) => d.name === "UnityWebModkit");
				if (uwmStore == -1) {
					reject();
					return;
				}
				const request = window.indexedDB.open("UnityWebModkit", 2);
				request.onsuccess = () => {
					const transaction = request.result.transaction(["storage"]);
					const objectStore = transaction.objectStore("storage");
					const metadataRequest = objectStore.get("metadata");
					metadataRequest.onsuccess = async () => {
						const metadataNode = webData.getNode(
							"Il2CppData/Metadata/global-metadata.dat",
						);
						if (!metadataNode || !metadataNode.data) {
							reject();
							return;
						}
						this.allReferencedAssemblies = this.plugins.flatMap(
							(plugin) => plugin.referencedAssemblies,
						);
						const globalMetadata = metadataRequest.result;
						if (
							JSON.stringify(this.allReferencedAssemblies.sort()) !==
							JSON.stringify(globalMetadata.referencedAssemblies.sort())
						) {
							reject();
							return;
						}
						const currentHash = bufToHex(
							await window.crypto.subtle.digest("SHA-256", metadataNode.data),
						);
						if (currentHash !== globalMetadata.integrityHash) {
							reject();
							return;
						}
						this.globalMetadata = globalMetadata;
						resolve();
					};
				};
			});
		});
	}

	private readIl2CppContextFromStorage(): Promise<void> {
		return new Promise<void>((resolve, reject) => {
			indexedDB.databases().then(async (databases) => {
				const uwmStore = databases.findIndex((d) => d.name === "UnityWebModkit");
				if (uwmStore == -1) {
					reject();
					return;
				}
				const request = window.indexedDB.open("UnityWebModkit");
				request.onsuccess = () => {
					const transaction = request.result.transaction(["storage"]);
					const objectStore = transaction.objectStore("storage");
					const il2CppRequest = objectStore.get("il2cpp");
					il2CppRequest.onsuccess = async () => {
						this.il2CppContext = il2CppRequest.result;
						resolve();
					};
				};
			});
		});
	}

	private hookWasmInstantiate() {
		this.instantiateStreaming = WebAssembly.instantiateStreaming;
		WebAssembly.instantiateStreaming = this.onWebAssemblyInstantiateStreaming.bind(this);
	}

	private async onWebAssemblyInstantiateStreaming(
		source: Response | PromiseLike<Response>,
		importObject?: WebAssembly.Imports | undefined,
	): Promise<WebAssembly.WebAssemblyInstantiatedSource> {
		// Wait for the Il2Cpp metadata to be resolved before continuing
		await waitFor(() => this.globalMetadata);
		if (this.globalMetadata?.imageDefs.length === 0)
			return this.instantiateStreaming(source, importObject);

		let response: Response;
		if (source instanceof Promise) {
			response = await source;
		} else if (source instanceof Response) {
			response = source;
		} else {
			this.logger.error(
				"TypeError: Got an unexpected object type as the first argument to WebAssembly.instantiateStreaming",
			);
			return Promise.reject();
		}

		// Build function index map from plugin hook mappings
		// (old -> new redirects for OP_CALL patching)
		// Identity mappings are skipped - they're a no-op for the patcher

		const oldIndices = new Uint32Array(funcIndexMap.map(([old]) => old));
		const newIndices = new Uint32Array(funcIndexMap.map(([, new_]) => new_));

		// Read chunks from the response body stream instead of buffering with arrayBuffer()
		// This allows the patcher to work on streamed data rather than waiting for the full response
		const chunks: Uint8Array[] = [];
		const reader = response.body?.getReader();
		if (!reader) {
			this.logger.error("No readable body in WASM response");
			return this.instantiateStreaming(response, importObject);
		}

		while (true) {
			const { done, value } = await reader.read();
			if (done) break;
			if (value) chunks.push(value);
		}

		// Concatenate all chunks into a single buffer
		const totalLength = chunks.reduce((acc, chunk) => acc + chunk.length, 0);
		const fullBytes = new Uint8Array(totalLength);
		let byteOffset = 0;
		for (const chunk of chunks) {
			fullBytes.set(chunk, byteOffset);
			byteOffset += chunk.length;
		}

		this.resolveIl2CppFunctions(fullBytes, importObject!);

		const exportIndices: number[] = [];
		const exportNames: string[] = [];
		for (const [key, value] of Object.entries(this.resolvedIl2CppFunctions)) {
			exportIndices.push(value);
			exportNames.push(key);
		}

		let patchedBytes: Uint8Array;
		try {
			patchedBytes = patch_wasm_complete(
				fullBytes,
				oldIndices,
				newIndices,
				new Uint32Array(exportIndices),
				exportNames,
			);
		} catch (e) {
			this.logger.error(`WASM complete patch failed: ${e}`);
			patchedBytes = fullBytes;
		}

		// Create a new Response with the patched bytes for WebAssembly.instantiateStreaming
		const patchedResponse = new Response(patchedBytes as unknown as BodyInit, {
			status: response.status,
			statusText: response.statusText,
			headers: response.headers,
		});

		// Use WebAssembly.instantiateStreaming with the patched response
		// This enables streaming instantiation instead of waiting for the full buffer
		const instantiatedSource = await WebAssembly.instantiateStreaming(
			patchedResponse,
			importObject,
		);

		// Fallback for hooking functions that are invoked indirectly
		const unappliedHooks = this.getUnappliedHooks();
		const tableName: string =
			this.tableName || this.resolveTableName(instantiatedSource.instance.exports);
		unappliedHooks.forEach((hook) => {
			// @ts-ignore
			var originalFunc = instantiatedSource.instance.exports[tableName].get(hook.tableIndex);
			const hookResults = hook.returnType ? [hook.returnType] : [];
			let injectFunc = null;
			if (!hook.kind) {
				// @ts-ignore
				injectFunc = new WebAssembly.Function(
					{
						parameters: hook.params,
						results: hookResults,
					},
					(...args: number[]) => {
						if (!hook.enabled) {
							if (hook.returnType) {
								return originalFunc(...args);
							}
							originalFunc(...args);
							return;
						}
						const wrappedArgs = args.map((arg) => new ValueWrapper(arg));
						const result = hook.callback(...wrappedArgs);
						// Unwrap arguments in case they were changed in the callback function
						args = wrappedArgs.map((arg) => arg.val());
						if (result === undefined || result === true) {
							if (hook.returnType) {
								return originalFunc(...args);
							}
							originalFunc(...args);
						}
					},
				);
			} else {
				// @ts-ignore
				injectFunc = new WebAssembly.Function(
					{
						parameters: hook.params,
						results: hookResults,
					},
					(...args: number[]) => {
						let originalResult = originalFunc(...args);
						if (!hook.enabled) return hook.returnType ? originalResult : undefined;
						if (originalResult !== undefined)
							originalResult = new ValueWrapper(originalResult);
						const wrappedArgs = args.map((arg) => new ValueWrapper(arg));
						hook.callback(originalResult, ...wrappedArgs);
						return originalResult?.val();
					},
				);
			}
			// @ts-ignore
			instantiatedSource.instance.exports[tableName].set(hook.tableIndex, injectFunc);
			hook.applied = true;
		});
		this.logger.message("Chainloader startup complete");

		return instantiatedSource;
	}

	private searchWasmBinary(bufferSource: ArrayBuffer) {
		if (!this.globalMetadata) return;
		const il2CppContext = createIl2CppContext(
			bufferSource,
			this.globalMetadata,
			this.allReferencedAssemblies,
		);
		if (il2CppContext.isErr()) {
			this.logger.error(il2CppContext.error.print());
			return;
		}
		this.il2CppContext = il2CppContext.value;
		this.saveIl2CppContext();
	}

	/// Resolve Il2Cpp function indices by searching WASM function bodies for byte patterns.
	/// Replaces WAIL's WailParser-based function index resolution.
	private resolveIl2CppFunctions(wasmBytes: Uint8Array, importObject: WebAssembly.Imports) {
		const importObjectSize = Object.keys(importObject.a).length;

		// Find il2cpp_string_new by searching for its instruction pattern
		// Pattern: GET_GLOBAL 0, I32_CONST 16, OP_CALL target, ...
		const stringNewPattern = new Uint8Array([
			35, 0, 65, 16, 107, 34, 2, 36, 0, 32, 2, 32, 0, 32, 1, 16,
		]);
		const stringNewIdx = find_function_by_pattern(wasmBytes, stringNewPattern);

		if (stringNewIdx !== undefined) {
			this.resolvedIl2CppFunctions["il2cpp_string_new"] = stringNewIdx + importObjectSize;
		}

		// TODO: This is a hack, but seems to work consistently with Unity 2021.3.15f1
		this.resolvedIl2CppFunctions["il2cpp_object_new"] = importObjectSize + 3;
	}

	public resolveTableName(asm: any): string {
		return Object.keys(asm).find((key) => asm[key].constructor.name == "Table") || "Unknown";
	}

	public createObject(typeInfo: number | ValueWrapper): number {
		console.log("creating object with typeinfo", typeInfo);
		const result = this.game.Module.asm.il2cpp_object_new(
			typeInfo instanceof ValueWrapper ? typeInfo.val() : typeInfo,
		);
		console.log("created object result at", result);
		return result;
	}

	public createMstr(char: string): number {
		const charAlloc = this.game.Module._malloc(char.length);
		writeUint8ArrayAtOffset(this.game.Module.HEAPU8, new TextEncoder().encode(char), charAlloc);
		return this.game.Module.asm.il2cpp_string_new(charAlloc, char.length);
	}

	public memory(address: number | ValueWrapper, size: number): Uint8Array {
		if (address instanceof ValueWrapper) address = address.val();
		return this.game.Module.HEAPU8.slice(address, address + size);
	}

	public malloc(size: number): number {
		return this.game.Module._malloc(size);
	}

	public free(block: number | ValueWrapper): void {
		this.game.Module._free(block instanceof ValueWrapper ? block.val() : block);
	}

	public getTableIndex(targetClass: string, targetMethod: string): number {
		if (!this.il2CppContext?.scriptData[targetClass]) return -1;
		const result = this.il2CppContext.scriptData[targetClass][targetMethod];
		if (!result) return -1;
		return result;
	}

	private getInternalIndex(tableIndex: number): number {
		return this.internalMappings[0].elements[tableIndex - 1];
	}

	private getHookByIndex(index: number): Hook | null {
		let totalHooksCount = 0;

		for (const plugin of this.plugins) {
			const hooksCount = plugin.hooks.length;

			// Check if the index is within the current plugin's hooks range
			if (index < totalHooksCount + hooksCount) {
				const hookIndex = index - totalHooksCount;
				return plugin.hooks[hookIndex];
			}

			totalHooksCount += hooksCount;
		}

		// If the index is out of range, return null
		return null;
	}

	private getUnappliedHooks(): Hook[] {
		return this.plugins.flatMap((plugin) => plugin.hooks).filter((hook) => !hook.applied);
	}
}

type Hook = {
	index?: number;
	tableIndex?: number;
	typeName: string;
	methodName: string;
	params: string[];
	returnType?: string;
	applied: boolean;
	enabled: boolean;
	kind: number;
	callback: PrefixCallback | PostfixCallback;
};

export type HookInfo = {
	typeName: string;
	methodName: string;
	params: string[];
	returnType?: string;
};

type PrefixCallback = ((...args: any) => boolean) | (() => void);
type PostfixCallback = (...args: any) => void;

type ModkitPluginOptions = {
	name: string;
	version?: string;
	referencedAssemblies?: string[];
};

class ModkitPlugin {
	public readonly name: string;
	public readonly version: string;
	public readonly logger: Logger;
	public onLoaded: (() => void) | undefined = undefined;
	private _referencedAssemblies: string[] = [];
	private _hooks: Hook[] = [];
	private _runtime: Runtime;

  get game(): any {
    return getGame();
  }

	constructor(
		name: string,
		version: string | undefined,
		referencedAssemblies: string[] | undefined,
		runtime: Runtime,
	) {
		this.name = name;
		this.version = version || "1.0.0";
		this.logger = new Logger(name);
		this._referencedAssemblies = referencedAssemblies || [];
		this._runtime = runtime;
	}

	public get hooks(): Hook[] {
		return this._hooks;
	}

	public get referencedAssemblies(): string[] {
		return this._referencedAssemblies;
	}

	public hookPrefix(target: HookInfo, callback: PrefixCallback): Hook {
		return this.hook(target, callback, 0);
	}

	public hookPostfix(target: HookInfo, callback: PostfixCallback): Hook {
		return this.hook(target, callback, 1);
	}

	private hook(target: HookInfo, callback: PrefixCallback | PostfixCallback, kind: number): Hook {
		const hook = {
			typeName: target.typeName,
			methodName: target.methodName,
			params: target.params,
			returnType: target.returnType,
			applied: false,
			enabled: true,
			kind,
			callback,
		};
		this._hooks.push(hook);
		return hook;
	}

	public call(target: string, args: any[]): void;
	public call(targetClass: string, targetMethod: string, args: any[]): void;
	public call(target: string, targetMethodOrArgs?: string | any[], args?: any[]) {
		const tableName: string =
			this._runtime.tableName || this._runtime.resolveTableName(this.game.Module.asm);
		if (typeof targetMethodOrArgs === "string") {
			const tableIndex = this._runtime.getTableIndex(target, targetMethodOrArgs);
			if (tableIndex === -1)
				throw new Error(
					`Failed to invoke function! Could not find table index for ${target + "$$" + targetMethodOrArgs}`,
				);
			if (args) args = args.map((arg) => (arg instanceof ValueWrapper ? arg.val() : arg));
			const result = this.game.Module.asm[tableName].get(tableIndex)(...(args as any[]));
			return new ValueWrapper(result);
		} else if (
			typeof targetMethodOrArgs === "object" ||
			typeof targetMethodOrArgs === "undefined"
		) {
			const [typeName, methodName] = target.replace("::", "$$").split("$$");
			const tableIndex = this._runtime.getTableIndex(typeName, methodName);
			if (tableIndex === -1)
				throw new Error(
					`Failed to invoke function! Could not find table index for ${typeName + "$$" + methodName}`,
				);
			if (!targetMethodOrArgs) targetMethodOrArgs = [];
			targetMethodOrArgs = targetMethodOrArgs.map((arg) =>
				arg instanceof ValueWrapper ? arg.val() : arg,
			);
			const result = this.game.Module.asm[tableName].get(tableIndex)(
				...(targetMethodOrArgs as any[]),
			);
			return new ValueWrapper(result);
		}
	}

	public createObject(typeInfo: ValueWrapper | number): ValueWrapper {
		return new ValueWrapper(this._runtime.createObject(typeInfo));
	}

	public createMstr(char: string): ValueWrapper {
		return new ValueWrapper(this._runtime.createMstr(char));
	}

	public slice(address: ValueWrapper | number, size: number = 256): Uint8Array {
		return this._runtime.memory(address, size);
	}

	public malloc(size: number): ValueWrapper {
		return new ValueWrapper(this._runtime.malloc(size));
	}

	public free(block: ValueWrapper | number): void {
		this._runtime.free(block);
	}

	public memcpy(dest: ValueWrapper | number, src: ValueWrapper | number, count: number): void {
		writeUint8ArrayAtOffset(
			this.game.Module.HEAPU8,
			this.slice(src, count),
			dest instanceof ValueWrapper ? dest.val() : dest,
		);
	}
}

export class ValueWrapper {
	private _result: number;

	get game(): any {
		return getGame();
	}
  static get game(): any {
		return getGame();
	}

	constructor(result: number) {
		this._result = result;
	}

	public set(value: ValueWrapper | number): void {
		this._result = value instanceof ValueWrapper ? value.val() : value;
	}

	public val(): number {
		return this._result;
	}

	public mstr(): string {
		return ValueWrapper.readUtf16Char(this._result + 12);
	}

	public deref(): ValueWrapper | undefined {
		const val = this.readField(0, "u32")?.val();
		return val ? new ValueWrapper(val) : undefined;
	}

	public getClassName(): string | null {
		try {
			// @ts-ignore
			const _game = window.game || game;
			const classPtr = new DataView(
				this.game.Module.HEAPU8.slice(this._result, this._result + 4).buffer,
			).getUint32(0, true);
			let classNamePtr = new DataView(
				this.game.Module.HEAPU8.slice(classPtr + 8, classPtr + 12).buffer,
			).getUint32(0, true);
			const classNameReader = new BinaryReader(
				this.game.Module.HEAPU8.slice(
					classNamePtr,
					classNamePtr + 128, // Assumed max length for a class name
				).buffer,
			);
			return classNameReader.readNullTerminatedUTF8String();
		} catch {
			return null;
		}
	}

	public readField(offset: number, type: string): ValueWrapper | undefined {
		const valAddress = this._result + offset;
		let valArray = this.game.Module.HEAPU8.slice(valAddress, valAddress + 4);
		const reader = new BinaryReader(valArray.buffer);
		switch (type) {
			case "i32":
				return new ValueWrapper(reader.readInt32());
			case "f32":
				return new ValueWrapper(reader.readFloat());
			case "u8":
				return new ValueWrapper(reader.readUint8());
			case "u32":
				return new ValueWrapper(reader.readUint32());
		}
	}

	public writeField(offset: number, type: string, value: ValueWrapper | number): void {
		let size = dataTypeSizes[type];
		const writer = new BinaryWriter(new ArrayBuffer(size));
		if (value instanceof ValueWrapper) value = value.val();
		switch (type) {
			case "u8":
				writer.writeUint8(value);
				break;
			case "i32":
				writer.writeInt32(value);
				break;
			case "u32":
				writer.writeUint32(value);
				break;
			case "f32":
				writer.writeFloat(value);
				break;
		}
		writeUint8ArrayAtOffset(this.game.Module.HEAPU8, writer.finalize(), this._result + offset);
	}

	private static readUtf16Char(ptr: number) {
		let buffer = new Uint16Array(this.game.Module.HEAPU8.buffer);
		let offset = ptr / 2; // divide by 2 to convert from byte offset to character offset
		let subarray = [];
		let charCode = buffer[offset];

		while (charCode !== 0) {
			subarray.push(charCode);
			offset++;
			charCode = buffer[offset];
		}

		let decoder = new TextDecoder("utf-16le");
		return decoder.decode(new Uint16Array(subarray));
	}
}
