#!/usr/bin/env bun

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

        const metadata = JSON.parse(
            await readFile(metadataPath, "utf8"),
        ) as { slug?: string; description: string };

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
}

function updateDescription(content: string, newDescription: string): string {
    // Regex to match description in frontmatter
    // Matches: description: followed by anything until newline
    const regex = /(^---\n[\s\S]*?\ndescription:\s*)(.*)(\n[\s\S]*?---)/m;

    if (regex.test(content)) {
        return content.replace(regex, `$1${newDescription}$3`);
    } else {
        // Fallback: If description key doesn't exist but frontmatter does, we could add it.
        // For now, let's assume standard template structure.
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
