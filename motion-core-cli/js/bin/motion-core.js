#!/usr/bin/env node

import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";

const __dirname = dirname(fileURLToPath(import.meta.url));

const platform = process.platform;
const arch = process.arch;

const targets = {
	"darwin-arm64": "aarch64-apple-darwin",
	"darwin-x64": "x86_64-apple-darwin",
	"linux-arm64": "aarch64-unknown-linux-gnu",
	"linux-x64": "x86_64-unknown-linux-gnu",
	"win32-arm64": "aarch64-pc-windows-msvc",
	"win32-x64": "x86_64-pc-windows-msvc",
};

const variant = `${platform}-${arch}`;
const target = targets[variant];

if (!target) {
	console.error(
		`motion-core: unsupported platform/architecture combination: ${variant}`,
	);
	console.error(
		"Please open an issue at https://github.com/motion-core/motion-core/issues.",
	);
	process.exit(1);
}

const binaryName = platform === "win32" ? "motion-core.exe" : "motion-core";
const binaryPath = join(__dirname, "..", "dist", target, binaryName);

if (!existsSync(binaryPath)) {
	console.error(`motion-core: binary not found for target ${target}.`);
	console.error("Build the Rust CLI with:");
	console.error(`  cargo build --release --target ${target}`);
	console.error("Then copy the resulting binary into js/dist/<target>/.");
	process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
	stdio: "inherit",
});

if (result.error) {
	console.error(result.error);
	process.exit(result.status ?? 1);
}

process.exit(result.status ?? 0);
