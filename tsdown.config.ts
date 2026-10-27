import { defineConfig } from "tsdown";
import { version } from "./package.json" with { type: "json" };
import { wasm } from "rolldown-plugin-wasm";

export default defineConfig({
	entry: "src/index.ts",
	target: "esnext",
	platform: "browser",
	plugins: [],
	define: {
		DEVELOPMENT: "true",
		VERSION: JSON.stringify(version),
	},
});
