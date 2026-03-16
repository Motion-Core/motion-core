import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";

const workerPath = path.join(process.cwd(), ".svelte-kit", "cloudflare", "_worker.js");
const cloudflareDir = path.dirname(workerPath);
const require = createRequire(import.meta.url);

const resvgWasmImportLine =
	'import __docsOgResvgWasmModule from "./resvg-index_bg.wasm";';
const resvgWasmGlobalLine = "globalThis.__docsOgResvgWasmModule = __docsOgResvgWasmModule;";
const yogaWasmFileName = "satori-yoga.wasm";
const yogaWasmImportLine = `import __docsOgYogaWasmModule from "./${yogaWasmFileName}";`;
const yogaWasmGlobalLine = "globalThis.__docsOgYogaWasmModule = __docsOgYogaWasmModule;";
const anchor = 'import { env } from "cloudflare:workers";';

if (!fs.existsSync(workerPath)) {
	throw new Error(`Worker entry not found: ${workerPath}`);
}

const yogaSourcePath = require.resolve("satori/yoga.wasm");
const yogaDestPath = path.join(cloudflareDir, yogaWasmFileName);
if (!fs.existsSync(yogaDestPath)) {
	fs.copyFileSync(yogaSourcePath, yogaDestPath);
}

const source = fs.readFileSync(workerPath, "utf8");

if (source.includes(resvgWasmGlobalLine) && source.includes(yogaWasmGlobalLine)) {
	process.exit(0);
}

if (!source.includes(anchor)) {
	throw new Error(`Could not find import anchor in ${workerPath}`);
}

const injectedLines = [
	anchor,
	resvgWasmImportLine,
	resvgWasmGlobalLine,
	yogaWasmImportLine,
	yogaWasmGlobalLine,
].join("\n");
const patched = source.replace(anchor, injectedLines);
fs.writeFileSync(workerPath, patched, "utf8");
