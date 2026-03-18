#!/usr/bin/env bun
/* eslint-disable no-console */

import { readFile, readdir, stat } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

type ComponentManifest = {
	name: string;
	slug?: string;
	category: string;
};

type ManifestComponent = {
	slug: string;
	name: string;
	category: string;
	dir: string;
};

type DocItem = {
	slug: string;
	name: string;
	items?: DocItem[];
};

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
const navigationPath = path.join(
	rootDir,
	"apps",
	"web",
	"src",
	"lib",
	"config",
	"navigation.ts",
);

const IGNORED_NAV_SECTIONS = new Set(["getting-started", "resources"]);

async function main() {
	console.log("Validating docs navigation...");

	const [manifestComponents, docsNavigation] = await Promise.all([
		loadManifestComponents(),
		loadDocsNavigation(),
	]);

	const errors = validateNavigation(manifestComponents, docsNavigation);

	if (errors.length > 0) {
		for (const error of errors) {
			console.error(`- ${error}`);
		}

		console.error(
			`\nNavigation integrity check failed with ${errors.length} issue${errors.length === 1 ? "" : "s"}.`,
		);
		process.exit(1);
	}

	console.log("Docs navigation check passed.");
}

async function loadManifestComponents(): Promise<ManifestComponent[]> {
	const entries = await readdir(componentRoot, { withFileTypes: true });
	const components: ManifestComponent[] = [];

	for (const entry of entries) {
		if (!entry.isDirectory()) {
			continue;
		}

		const dir = entry.name;
		const manifestPath = path.join(componentRoot, dir, "component.json");
		const manifestExists = await fileExists(manifestPath);

		if (!manifestExists) {
			continue;
		}

		const raw = await readFile(manifestPath, "utf8");
		let manifest: ComponentManifest;

		try {
			manifest = JSON.parse(raw) as ComponentManifest;
		} catch {
			throw new Error(`[${dir}] Invalid JSON in component.json`);
		}

		if (!manifest.name || !manifest.category) {
			throw new Error(
				`[${dir}] component.json must include "name" and "category".`,
			);
		}

		components.push({
			slug: manifest.slug ?? dir,
			name: manifest.name,
			category: manifest.category,
			dir,
		});
	}

	return components;
}

async function loadDocsNavigation(): Promise<DocItem[]> {
	const navigationModule = (await import(
		pathToFileURL(navigationPath).href
	)) as {
		docsNavigation?: DocItem[];
	};

	if (!Array.isArray(navigationModule.docsNavigation)) {
		throw new Error(
			`Expected "${navigationPath}" to export docsNavigation as an array.`,
		);
	}

	return navigationModule.docsNavigation;
}

function validateNavigation(
	manifestComponents: ManifestComponent[],
	docsNavigation: DocItem[],
) {
	const errors: string[] = [];

	const manifestBySlug = new Map<string, ManifestComponent>();
	const manifestCategories = new Set<string>();

	for (const component of manifestComponents) {
		const duplicate = manifestBySlug.get(component.slug);
		if (duplicate) {
			errors.push(
				`Duplicate manifest slug "${component.slug}" in "${duplicate.dir}" and "${component.dir}".`,
			);
			continue;
		}

		manifestBySlug.set(component.slug, component);
		manifestCategories.add(component.category);
	}

	const navCategories = new Set<string>();
	const navBySlug = new Map<string, { category: string; item: DocItem }>();

	for (const section of docsNavigation) {
		if (IGNORED_NAV_SECTIONS.has(section.slug)) {
			continue;
		}

		if (navCategories.has(section.slug)) {
			errors.push(`Duplicate navigation category "${section.slug}".`);
			continue;
		}

		navCategories.add(section.slug);

		const items = section.items ?? [];
		const expectedOrder = [...items].sort(compareByNameThenSlug);
		if (!hasSameOrder(items, expectedOrder)) {
			errors.push(
				`Category "${section.slug}" is not sorted alphabetically. Expected order: ${expectedOrder
					.map((item) => item.name)
					.join(", ")}.`,
			);
		}

		const seenSlugs = new Set<string>();
		for (const item of items) {
			if (seenSlugs.has(item.slug)) {
				errors.push(
					`Duplicate component slug "${item.slug}" in navigation category "${section.slug}".`,
				);
			}
			seenSlugs.add(item.slug);

			const existing = navBySlug.get(item.slug);
			if (existing) {
				errors.push(
					`Component slug "${item.slug}" appears in multiple navigation categories: "${existing.category}" and "${section.slug}".`,
				);
				continue;
			}

			navBySlug.set(item.slug, {
				category: section.slug,
				item,
			});

			const manifest = manifestBySlug.get(item.slug);
			if (!manifest) {
				errors.push(
					`Navigation item "${item.slug}" in category "${section.slug}" has no matching component manifest.`,
				);
				continue;
			}

			if (manifest.category !== section.slug) {
				errors.push(
					`Component "${item.slug}" is under navigation category "${section.slug}" but manifest category is "${manifest.category}".`,
				);
			}

			if (manifest.name !== item.name) {
				errors.push(
					`Component "${item.slug}" name mismatch: navigation has "${item.name}", manifest has "${manifest.name}".`,
				);
			}
		}
	}

	for (const category of manifestCategories) {
		if (!navCategories.has(category)) {
			errors.push(
				`Missing navigation category "${category}" for component manifests.`,
			);
		}
	}

	for (const category of navCategories) {
		if (!manifestCategories.has(category)) {
			errors.push(
				`Navigation category "${category}" has no matching component manifests.`,
			);
		}
	}

	for (const component of manifestComponents) {
		if (!navBySlug.has(component.slug)) {
			errors.push(
				`Missing component "${component.slug}" (${component.name}) in navigation category "${component.category}".`,
			);
		}
	}

	return errors;
}

function compareByNameThenSlug(
	a: Pick<DocItem, "name" | "slug">,
	b: Pick<DocItem, "name" | "slug">,
) {
	const byName = a.name.localeCompare(b.name, undefined, {
		numeric: true,
		sensitivity: "base",
	});

	if (byName !== 0) {
		return byName;
	}

	return a.slug.localeCompare(b.slug, undefined, {
		numeric: true,
		sensitivity: "base",
	});
}

function hasSameOrder(current: DocItem[], expected: DocItem[]) {
	if (current.length !== expected.length) {
		return false;
	}

	return current.every((item, index) => item.slug === expected[index]?.slug);
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
	console.error("Failed to validate docs navigation:", error);
	process.exit(1);
});
