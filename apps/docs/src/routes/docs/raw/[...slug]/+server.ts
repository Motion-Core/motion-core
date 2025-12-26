import { error, type RequestHandler } from "@sveltejs/kit";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = fileURLToPath(new URL(".", import.meta.url));
const DOCS_ROOT = path.resolve(currentDir, "..", "..");

const buildDocPath = (slug: string) => {
	const segments = slug
		.split("/")
		.map((segment) => segment.trim())
		.filter(Boolean);

	if (segments.length === 0) {
		throw error(404, "Doc not found");
	}

	const targetPath = path.resolve(DOCS_ROOT, ...segments, "+page.svx");
	const relativePath = path.relative(DOCS_ROOT, targetPath);

	if (relativePath.startsWith("..") || path.isAbsolute(relativePath)) {
		throw error(404, "Doc not found");
	}

	return targetPath;
};

export const GET: RequestHandler = async ({ params }) => {
	const slug = params.slug;

	if (!slug) {
		throw error(404, "Doc not found");
	}

	const docPath = buildDocPath(slug);

	try {
		const contents = await readFile(docPath, "utf8");
		return new Response(contents, {
			headers: {
				"content-type": "text/markdown; charset=utf-8",
				"cache-control": "public, max-age=60",
			},
		});
	} catch {
		throw error(404, "Doc not found");
	}
};
