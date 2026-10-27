import { ok, err, Result } from "neverthrow";
import { BinaryReader } from "../utils/binary";
import { MetadataParsingError } from "../errors";
import { Metadata, parse_from_bytes } from "./parser";

export type Il2CppMetadata = Metadata;

export type Il2CppContext = {
  codeGenModules: Il2CppCodeGenModuleCollection;
  codeGenModuleMethodPointers: Il2CppCodeGenModuleMethodPointers;
  scriptData: Il2CppScriptData;
  name: string;
};

type Il2CppCodeGenModule = {
  moduleName: number;
  methodPointerCount: number;
  methodPointers: number;
  adjustorThunkCount: number;
  adjustorThunks: number;
  invokerIndices: number;
  reversePInvokeWrapperCount: number;
  reversePInvokeWrapperIndices: number;
  rgctxRangesCount: number;
  rgctxRanges: number;
  rgctxsCount: number;
  rgctxs: number;
  debuggerMetadata: number;
  moduleInitializer: number;
  staticConstructorTypeIndices: number;
  metadataRegistration: number;
  codeRegistration: number;
};

type Il2CppCodeGenModuleCollection = {
  [moduleName: string]: Il2CppCodeGenModule;
};

type Il2CppCodeGenModuleMethodPointers = {
  [moduleName: string]: number[];
};

type Il2CppScriptData = {
  [typeName: string]: {
    [methodName: string]: number;
  };
};

export async function createMetadata(
  buffer: ArrayBuffer
): Promise<Result<Il2CppMetadata, MetadataParsingError>> {
  { // quick sanity check before calling the WASM for -1% performance boost ig
    const reader = new BinaryReader(buffer);
    const sanity = reader.readUint32();
    if (sanity !== 0xfab11baf) return err(new MetadataParsingError("Metadata file supplied is not a valid metadata file."));
    const version = reader.readUint32();
    if (version < 0 || version > 1000) return err(new MetadataParsingError("Metadata file supplied is not a valid metadata file."));
  }
    const { metadata } = parse_from_bytes(new Uint8Array(buffer));
  return ok(metadata);
}
