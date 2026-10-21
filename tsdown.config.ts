import { defineConfig } from "tsdown";
import { version } from "./package.json" with { type: "json" };

export default defineConfig({
	entry: "src/index.ts",
	target: "esnext",
	platform: "browser",
	define: {
		DEVELOPMENT: "true",
		VERSION: JSON.stringify(version),
	},
});
