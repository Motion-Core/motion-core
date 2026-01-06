#!/usr/bin/env node
/* eslint-disable no-console */

import {
	existsSync,
	mkdtempSync,
	cpSync,
	rmSync,
	mkdirSync,
	readFileSync,
	copyFileSync,
} from "node:fs";
import { fileURLToPath } from "node:url";
import { tmpdir } from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "..", "..");
const fixtureArg = process.argv[2] ?? "fixtures/sveltekit-minimal";
const fixturePath = path.resolve(repoRoot, fixtureArg);

if (!existsSync(fixturePath)) {
	console.error(`Fixture not found at ${fixturePath}`);
	process.exit(1);
}

const registryRoot = path.join(repoRoot, "apps", "docs", "static", "registry");
const requiredRegistryFiles = ["registry.json", "components.json"];
for (const file of requiredRegistryFiles) {
	if (!existsSync(path.join(registryRoot, file))) {
		console.error(`Missing registry artifact ${file} in ${registryRoot}`);
		process.exit(1);
	}
}

prepareCliBinary();

const tempDir = mkdtempSync(path.join(tmpdir(), "motion-core-cli-"));
const workspaceDir = path.join(tempDir, "app");
const cacheDir = path.join(tempDir, "cache");
cpSync(fixturePath, workspaceDir, { recursive: true });
mkdirSync(cacheDir, { recursive: true });

const cleanup = () => {
	rmSync(tempDir, { recursive: true, force: true });
};
const shouldCleanup = process.env.KEEP_FIXTURE !== "1";
let failed = false;

const registryUrl =
	process.env.MOTION_CORE_REGISTRY_URL_FOR_TESTS ??
	"https://motion-core.dev/registry";
seedRegistryCache(registryUrl, cacheDir, registryRoot);
const npmCmd = commandFor("npm");
const cliEnv = buildCliEnv(cacheDir);

try {
	run(npmCmd, ["ci"], { cwd: workspaceDir });

	const cliEntry = path.join(
		repoRoot,
		"motion-core-cli",
		"js",
		"bin",
		"motion-core.js",
	);

	run(process.execPath, [cliEntry, "init", "--registry-url", registryUrl], {
		cwd: workspaceDir,
		env: cliEnv,
	});

	run(
		process.execPath,
		[cliEntry, "add", "glass-pane", "--yes", "--registry-url", registryUrl],
		{
			cwd: workspaceDir,
			env: cliEnv,
		},
	);

	assertWorkspaceState(workspaceDir);
	console.log(`CLI fixture scenario completed successfully in ${workspaceDir}`);
} catch (err) {
	failed = true;
	console.error(err);
	process.exitCode = 1;
} finally {
	if (shouldCleanup && !failed) {
		cleanup();
	} else {
		console.log(`Workspace preserved at ${workspaceDir}`);
		if (shouldCleanup && failed) {
			console.log(
				"Cleanup skipped because the scenario failed. Remove the directory manually once inspected.",
			);
		}
	}
}

function run(command, args, options = {}) {
	console.log(`\n$ ${command} ${args.join(" ")}`);
	const spawnOptions = {
		stdio: "inherit",
		...options,
	};
	if (process.platform === "win32" && command.toLowerCase().endsWith(".cmd")) {
		spawnOptions.shell = true;
	}
	const result = spawnSync(command, args, spawnOptions);
	if (result.error) {
		throw result.error;
	}
	if (result.status !== 0) {
		throw new Error(`Command failed: ${command} ${args.join(" ")}`);
	}
}

function commandFor(bin) {
	if (process.platform === "win32") {
		return `${bin}.cmd`;
	}
	return bin;
}

function targetInfo() {
	const variant = `${process.platform}-${process.arch}`;
	const targets = {
		"darwin-arm64": "aarch64-apple-darwin",
		"darwin-x64": "x86_64-apple-darwin",
		"linux-arm64": "aarch64-unknown-linux-gnu",
		"linux-x64": "x86_64-unknown-linux-gnu",
		"win32-arm64": "aarch64-pc-windows-msvc",
		"win32-x64": "x86_64-pc-windows-msvc",
	};

	const target = targets[variant];
	if (!target) {
		throw new Error(`Unsupported host platform for tests: ${variant}`);
	}
	const binaryName =
		process.platform === "win32" ? "motion-core-cli.exe" : "motion-core-cli";
	return { target, binaryName };
}

function prepareCliBinary() {
	const { target, binaryName } = targetInfo();
	const distDir = path.join(repoRoot, "motion-core-cli", "js", "dist", target);
	const distBinary = path.join(distDir, binaryName);
	if (existsSync(distBinary)) {
		return;
	}
	const builtBinary = path.join(
		repoRoot,
		"motion-core-cli",
		"target",
		target,
		"release",
		binaryName,
	);
	if (!existsSync(builtBinary)) {
		throw new Error(
			`CLI binary missing. Run "cargo build --release -p motion-core-cli --target ${target}" first.`,
		);
	}
	mkdirSync(distDir, { recursive: true });
	copyFileSync(builtBinary, distBinary);
}

function buildCliEnv(cacheDir) {
	return {
		...process.env,
		MOTION_CORE_CACHE_DIR: cacheDir,
		MOTION_CORE_CACHE_TTL_MS: String(1000 * 60 * 60 * 24 * 7), // 7 days
		MOTION_CORE_ASSET_CACHE_TTL_MS: String(1000 * 60 * 60 * 24 * 7),
		HTTP_PROXY: "",
		http_proxy: "",
		HTTPS_PROXY: "",
		https_proxy: "",
		NO_PROXY: "127.0.0.1,localhost",
		no_proxy: "127.0.0.1,localhost",
	};
}

function seedRegistryCache(registryUrl, cacheDir, registryRoot) {
	const encoded = Buffer.from(registryUrl).toString("base64url");
	const namespace = `registry-${encoded}`;
	const targetDir = path.join(cacheDir, namespace);
	mkdirSync(targetDir, { recursive: true });
	for (const file of ["registry.json", "components.json"]) {
		copyFileSync(path.join(registryRoot, file), path.join(targetDir, file));
	}
}

function assertWorkspaceState(workspace) {
	const expectExists = (p) => {
		if (!existsSync(p)) {
			throw new Error(`Expected file missing: ${path.relative(workspace, p)}`);
		}
	};

	const configPath = path.join(workspace, "motion-core.json");
	expectExists(configPath);
	const config = JSON.parse(readFileSync(configPath, "utf8"));

	const componentsDir =
		config?.aliases?.components?.filesystem ||
		path.join("src", "lib", "motion-core");
	const utilsDir =
		config?.aliases?.utils?.filesystem ||
		path.join("src", "lib", "motion-core", "utils");

	expectExists(path.join(workspace, utilsDir, "cn.ts"));
	expectExists(
		path.join(workspace, componentsDir, "glass-pane", "GlassPane.svelte"),
	);

	const pkgPath = path.join(workspace, "package.json");
	const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
	const deps = { ...pkg.dependencies, ...pkg.devDependencies };
	for (const dep of ["clsx", "tailwind-merge"]) {
		if (!deps[dep]) {
			throw new Error(`Expected dependency ${dep} to be installed`);
		}
	}
}
