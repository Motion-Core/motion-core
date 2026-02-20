#!/usr/bin/env bun

import { mkdir, readFile, readdir, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

type ComponentFileEntry = {
	path: string;
	kind?: string;
	target?: string;
	typeExports?: string[];
};

type ComponentMetadata = {
	name: string;
	slug?: string;
	description: string;
	category: string;
	preview?: {
		video?: string;
		poster?: string;
	};
	dependencies?: Record<string, string>;
	devDependencies?: Record<string, string>;
	internalDependencies?: string[];
	files: ComponentFileEntry[];
};

type RegistryComponent = ComponentMetadata & {
	slug: string;
	files: ComponentFileEntry[];
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
const tokensRoot = path.join(
	rootDir,
	"packages",
	"motion-core",
	"src",
	"lib",
	"tokens",
);
const registryOutputDir = path.join(
	rootDir,
	"apps",
	"docs",
	"static",
	"registry",
);
const schemaOutputDir = path.join(registryOutputDir, "schema");
const generatedDocsManifestPath = path.join(
	rootDir,
	"apps",
	"docs",
	"src",
	"lib",
	"docs",
	"generated-manifest.ts",
);

const REGISTRY_NAME = "Motion Core Registry";
const REGISTRY_DESCRIPTION = "Curated Motion Core Svelte components";
const CONFIG_SCHEMA_URL =
	"https://motion-core.dev/registry/schema/config-schema.json";

const requirements = {
	svelte: "^5.0.0",
	tailwindcss: "^4.1.0",
};

type PackageJson = {
	version: string;
	dependencies?: Record<string, string>;
	devDependencies?: Record<string, string>;
};

async function main() {
	const pkgJson = JSON.parse(
		await readFile(
			path.join(rootDir, "packages", "motion-core", "package.json"),
			"utf8",
		),
	) as PackageJson;

	const componentDirs = (await readdir(componentRoot, { withFileTypes: true }))
		.filter((entry) => entry.isDirectory())
		.map((entry) => entry.name)
		.filter((dir) => dir !== "index.ts")
		.sort();

	const components: Record<string, RegistryComponent> = {};
	const assetPayload = new Map<string, Buffer>();

	for (const dir of componentDirs) {
		const metadataPath = path.join(componentRoot, dir, "component.json");
		const metadataExists = await fileExists(metadataPath);
		if (!metadataExists) {
			console.warn(`Skipping ${dir}: missing component.json`);
			continue;
		}

		const metadata = JSON.parse(
			await readFile(metadataPath, "utf8"),
		) as ComponentMetadata;

		const slug = metadata.slug ?? dir;
		const files = await Promise.all(
			metadata.files.map(async (entry) => {
				const relativePath = entry.path
					.replace(/\\/g, "/")
					.replace(/^\.\//, "");
				const sourcePath = path.join(componentRoot, dir, relativePath);
				const registryPath = toPosix(
					path.join("components", dir, relativePath),
				);
				const contents = await readFile(sourcePath);
				const transformedContents = adjustComponentImports(
					registryPath,
					contents,
				);
				assetPayload.set(registryPath, transformedContents);

				return {
					path: registryPath,
					kind: entry.kind,
					target: entry.target,
					typeExports: entry.typeExports,
				};
			}),
		);

		components[slug] = {
			slug,
			name: metadata.name,
			description: metadata.description,
			category: metadata.category,
			preview: metadata.preview,
			dependencies: metadata.dependencies ?? {},
			devDependencies: metadata.devDependencies ?? {},
			internalDependencies: metadata.internalDependencies ?? [],
			files,
		};
	}

	await appendCssTokens(assetPayload);

	await mkdir(registryOutputDir, { recursive: true });
	await mkdir(schemaOutputDir, { recursive: true });

	const baseDependencies = pkgJson.dependencies ?? {};
	const baseDevDependencies = pkgJson.devDependencies ?? {};

	const registry = {
		name: REGISTRY_NAME,
		description: REGISTRY_DESCRIPTION,
		version: pkgJson.version,
		requirements,
		baseDependencies,
		baseDevDependencies,
		components: Object.fromEntries(
			Object.entries(components)
				.sort(([a], [b]) => a.localeCompare(b))
				.map(([slug, component]) => {
					const { preview, ...rest } = component;
					return [slug, rest];
				}),
		),
	};

	const registryPath = path.join(registryOutputDir, "registry.json");
	await writeFile(registryPath, stringify(registry));

	const componentsEncoded: Record<string, string> = {};
	for (const [filePath, buffer] of assetPayload.entries()) {
		componentsEncoded[filePath] = buffer.toString("base64");
	}
	const componentsManifestPath = path.join(
		registryOutputDir,
		"components.json",
	);
	await writeFile(componentsManifestPath, stringify(componentsEncoded));

	const schemaPath = path.join(schemaOutputDir, "config-schema.json");
	await writeFile(schemaPath, stringify(buildConfigSchema()));

	await writeGeneratedDocsManifest(components);

	// eslint-disable-next-line no-console
	console.log(
		`Registry built with ${Object.keys(components).length} components.`,
	);
}

async function fileExists(filePath: string) {
	try {
		await stat(filePath);
		return true;
	} catch {
		return false;
	}
}

async function appendCssTokens(assetPayload: Map<string, Buffer>) {
	const tokensDirExists = await fileExists(tokensRoot);
	if (!tokensDirExists) {
		return;
	}

	for (const filePath of await collectCssFiles(tokensRoot)) {
		const relative = path.relative(tokensRoot, filePath);
		const registryPath = toPosix(path.join("tokens", relative));
		const buffer = await readFile(filePath);
		assetPayload.set(registryPath, buffer);
	}
}

async function collectCssFiles(dir: string): Promise<string[]> {
	const entries = await readdir(dir, { withFileTypes: true });
	const files: string[] = [];

	for (const entry of entries) {
		const entryPath = path.join(dir, entry.name);
		if (entry.isDirectory()) {
			files.push(...(await collectCssFiles(entryPath)));
		} else if (
			entry.isFile() &&
			path.extname(entry.name).toLowerCase() === ".css"
		) {
			files.push(entryPath);
		}
	}

	return files;
}

function stringify(value: unknown) {
	return `${JSON.stringify(value, null, 2)}\n`;
}

function toPosix(value: string) {
	return value.split(path.sep).join("/");
}

function adjustComponentImports(filePath: string, contents: Buffer) {
	if (!filePath.startsWith("components/")) {
		return contents;
	}

	const source = contents.toString("utf8");
	const replacements = [
		{ pattern: /(from\s+["'])\.\.\/\.\.\/utils\//g, target: "$1../utils/" },
		{
			pattern: /(from\s+["'])\.\.\/\.\.\/helpers\//g,
			target: "$1../helpers/",
		},
		{
			pattern: /(from\s+["'])\.\.\/\.\.\/assets\//g,
			target: "$1../assets/",
		},
	];

	let updated = source;
	for (const { pattern, target } of replacements) {
		updated = updated.replace(pattern, target);
	}

	if (updated === source) {
		return contents;
	}

	return Buffer.from(updated, "utf8");
}

function buildConfigSchema() {
	const aliasDefinition = {
		type: "object",
		properties: {
			filesystem: { type: "string" },
			import: { type: "string" },
		},
		required: ["filesystem", "import"],
		additionalProperties: false,
	};

	return {
		$schema: "http://json-schema.org/draft-07/schema#",
		$title: "Motion Core Config",
		$type: "object",
		properties: {
			$schema: { type: "string", const: CONFIG_SCHEMA_URL },
			tailwind: {
				type: "object",
				properties: {
					css: { type: "string", default: "src/routes/layout.css" },
				},
				additionalProperties: false,
			},
			aliases: {
				type: "object",
				properties: {
					components: {
						...aliasDefinition,
						default: {
							filesystem: "src/lib/motion-core",
							import: "$lib/motion-core",
						},
					},
					helpers: {
						...aliasDefinition,
						default: {
							filesystem: "src/lib/motion-core/helpers",
							import: "$lib/motion-core/helpers",
						},
					},
					utils: {
						...aliasDefinition,
						default: {
							filesystem: "src/lib/motion-core/utils",
							import: "$lib/motion-core/utils",
						},
					},
					assets: {
						...aliasDefinition,
						default: {
							filesystem: "src/lib/motion-core/assets",
							import: "$lib/motion-core/assets",
						},
					},
				},
				additionalProperties: false,
			},
			aliasPrefixes: {
				type: "object",
				properties: {
					components: {
						type: "string",
						default: "$lib/motion-core",
					},
				},
				additionalProperties: false,
			},
			exports: {
				type: "object",
				properties: {
					components: {
						type: "object",
						properties: {
							barrel: {
								type: "string",
								default: "src/lib/motion-core/index.ts",
							},
							strategy: {
								type: "string",
								enum: ["named"],
								default: "named",
							},
						},
						additionalProperties: false,
					},
				},
				additionalProperties: false,
			},
		},
		required: ["aliases", "aliasPrefixes", "exports"],
		additionalProperties: false,
	};
}

async function writeGeneratedDocsManifest(
	components: Record<string, RegistryComponent>,
) {
	const manifestEntries = Object.values(components)
		.map(({ slug, name, category, preview, dependencies }) => ({
			slug,
			name,
			category,
			video: preview?.video,
			poster: preview?.poster,
			dependencies,
		}))
		.sort((a, b) => a.name.localeCompare(b.name));

	const source = `import type { ComponentInfo } from "../components/landing";

export const docsManifest: ComponentInfo[] = ${JSON.stringify(
		manifestEntries,
		null,
		2,
	)};

export const getAdjacentDocs = (slug: string) => {
  const index = docsManifest.findIndex((doc) => doc.slug === slug);
  if (index === -1) {
    return { previous: null, next: null };
  }
  const previous = index > 0 ? docsManifest[index - 1] : null;
  const next = index < docsManifest.length - 1 ? docsManifest[index + 1] : null;
  return { previous, next };
};
`;

	await writeFile(generatedDocsManifestPath, source);
}

await main().catch((error) => {
	console.error("Failed to build registry:", error);
	process.exitCode = 1;
});
