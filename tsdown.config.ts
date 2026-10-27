import { defineConfig } from "tsdown";
import { version } from "./package.json" with { type: "json" };
import { wasm } from "@vape-rewrite/rolldown-plugin-wasm";

export default defineConfig({
	entry: "src/index.ts",
	target: "esnext",
	platform: "browser",
	plugins: [wasm({targetEnv: "auto-inline", forceSync: true})],
	define: {
		DEVELOPMENT: "true",
		VERSION: JSON.stringify(version),
	},
	sourcemap: true
});
