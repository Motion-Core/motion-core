#!/usr/bin/env bun
/* eslint-disable no-console */
import { readdir, readFile, stat } from "node:fs/promises";
import { join, extname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const COMPONENTS_DIR = join(
	__dirname,
	"../packages/motion-core/src/lib/components",
);

const IGNORED_FILES = new Set(["component.json", ".DS_Store", "README.md"]);
const TRACKED_EXTENSIONS = new Set([".svelte", ".ts", ".js"]);

interface ComponentManifest {
	name: string;
	files: Array<{ path: string }>;
}

async function validate() {
	console.log("Validating component manifests...");

	const entries = await readdir(COMPONENTS_DIR, { withFileTypes: true });
	const componentDirs = entries
		.filter((e) => e.isDirectory())
		.map((e) => e.name);

	let hasErrors = false;

	for (const compDir of componentDirs) {
		const dirPath = join(COMPONENTS_DIR, compDir);
		const manifestPath = join(dirPath, "component.json");

		try {
			await stat(manifestPath);
		} catch {
			continue;
		}

		const manifestContent = await readFile(manifestPath, "utf-8");
		let manifest: ComponentManifest;
		try {
			manifest = JSON.parse(manifestContent);
		} catch {
			console.error(`[${compDir}] Invalid JSON in component.json`);
			hasErrors = true;
			continue;
		}

		const manifestFiles = (manifest.files || []).map((f) => f.path);

		const definedLocalFiles = new Set(
			manifestFiles
				.filter((p) => !p.startsWith(".."))
				.map((p) => p.replace(/^\.\//, "")),
		);

		const seen = new Set<string>();
		for (const file of manifestFiles) {
			if (seen.has(file)) {
				console.error(`[${compDir}] Duplicate file in manifest: "${file}"`);
				hasErrors = true;
			}
			seen.add(file);
		}

		const diskFiles = await readdir(dirPath);
		for (const file of diskFiles) {
			if (IGNORED_FILES.has(file)) continue;
			if (!TRACKED_EXTENSIONS.has(extname(file))) continue;

			if (!definedLocalFiles.has(file)) {
				console.error(
					`[${compDir}] File exists on disk but missing in component.json: "${file}"`,
				);
				hasErrors = true;
			}
		}

		for (const file of definedLocalFiles) {
			try {
				await stat(join(dirPath, file));
			} catch {
				console.error(
					`[${compDir}] File listed in manifest but missing on disk: "${file}"`,
				);
				hasErrors = true;
			}
		}
	}

	if (hasErrors) {
		console.error("\nIntegrity checks failed. Please fix the errors above.");
		process.exit(1);
	} else {
		console.log("Component integrity check passed.");
	}
}

validate().catch((e) => {
	console.error(e);
	process.exit(1);
});
