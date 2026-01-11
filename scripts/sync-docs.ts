#!/usr/bin/env bun
/* eslint-disable no-console */

import { readFile, readdir, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, "..");
const componentRoot = path.join(
	rootDir,
	"packages",
	"motion-core",
	"src",
	"lib",
	"components",
);
const docsRoot = path.join(rootDir, "apps", "docs", "src", "routes", "docs");

async function main() {
	const componentDirs = (await readdir(componentRoot, { withFileTypes: true }))
		.filter((entry) => entry.isDirectory())
		.map((entry) => entry.name)
		.filter((dir) => dir !== "index.ts")
		.sort();

	let updatedCount = 0;

	for (const dir of componentDirs) {
		const metadataPath = path.join(componentRoot, dir, "component.json");
		const metadataExists = await fileExists(metadataPath);

		if (!metadataExists) {
			continue;
		}

		const metadata = JSON.parse(await readFile(metadataPath, "utf8")) as {
			slug?: string;
			description: string;
		};

		const slug = metadata.slug ?? dir;
		const description = metadata.description;

		if (!description) {
			console.warn(`[${slug}] Missing description in component.json`);
			continue;
		}

		const docPath = path.join(docsRoot, slug, "+page.svx");
		const docExists = await fileExists(docPath);

		if (!docExists) {
			// Docs might not exist yet for a component
			continue;
		}

		const docContent = await readFile(docPath, "utf8");
		const updatedContent = updateDescription(docContent, description);

		if (docContent !== updatedContent) {
			await writeFile(docPath, updatedContent);
			console.log(`[${slug}] Updated description in docs.`);
			updatedCount++;
		}
	}

	if (updatedCount > 0) {
		console.log(`\nSuccessfully synced ${updatedCount} documentation files.`);
	} else {
		console.log("\nAll documentation files are up to date.");
	}

	await syncChangelog();
	await syncCliChangelog();
}

async function syncChangelog() {
	const changelogPath = path.join(rootDir, "CHANGELOG.md");
	const docsChangelogDir = path.join(docsRoot, "changelog", "registry");
	const docsChangelogPath = path.join(docsChangelogDir, "+page.svx");

	try {
		const content = await readFile(changelogPath, "utf8");

		const cleanContent = content.replace(/^[\s\S]*?(?=## \[)/, "");

		const frontmatter = `---
title: Registry Changelog
description: All notable changes to Motion Core Component Registry.
---

`;
		const finalContent = frontmatter + cleanContent;

		try {
			await stat(docsChangelogDir);
		} catch {
			await import("node:fs/promises").then((fs) =>
				fs.mkdir(docsChangelogDir, { recursive: true }),
			);
		}

		await writeFile(docsChangelogPath, finalContent);
		console.log("\n[changelog] Successfully synced CHANGELOG.md to docs.");
	} catch (error) {
		console.error("\n[changelog] Failed to sync changelog:", error);
	}
}

async function syncCliChangelog() {
	const changelogPath = path.join(
		rootDir,
		"motion-core-cli",
		"js",
		"CHANGELOG.md",
	);
	const docsChangelogDir = path.join(docsRoot, "changelog", "cli");
	const docsChangelogPath = path.join(docsChangelogDir, "+page.svx");

	try {
		const content = await readFile(changelogPath, "utf8");

		// Remove header content until the first version header
		const cleanContent = content.replace(/^[\s\S]*?(?=## \[)/, "");

		const frontmatter = `---
title: CLI Changelog
description: All notable changes to Motion Core CLI.
---

`;

		const finalContent = frontmatter + cleanContent;

		try {
			await stat(docsChangelogDir);
		} catch {
			await import("node:fs/promises").then((fs) =>
				fs.mkdir(docsChangelogDir, { recursive: true }),
			);
		}

		await writeFile(docsChangelogPath, finalContent);
		console.log(
			"\n[cli-changelog] Successfully synced CLI CHANGELOG.md to docs.",
		);
	} catch (error) {
		console.error("\n[cli-changelog] Failed to sync CLI changelog:", error);
	}
}

function updateDescription(content: string, newDescription: string): string {
	const regex = /(^---\n[\s\S]*?\ndescription:\s*)(.*)(\n[\s\S]*?---)/m;

	if (regex.test(content)) {
		return content.replace(regex, `$1${newDescription}$3`);
	} else {
		return content;
	}
}

async function fileExists(filePath: string) {
	try {
		await stat(filePath);
		return true;
	} catch {
		return false;
	}
}

await main().catch((error) => {
	console.error("Failed to sync docs:", error);
	process.exitCode = 1;
});
