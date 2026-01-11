import { error, type RequestHandler } from "@sveltejs/kit";

const docFiles = import.meta.glob("/src/routes/docs/**/+page.svx", {
	query: "?raw",
	import: "default",
	eager: true,
}) as Record<string, string>;

const libFiles = import.meta.glob(
	"../../../../../../../packages/motion-core/src/lib/**/*",
	{
		query: "?raw",
		import: "default",
		eager: true,
	},
) as Record<string, string>;

const demoFiles = import.meta.glob("/src/routes/docs/**/*", {
	query: "?raw",
	import: "default",
	eager: true,
}) as Record<string, string>;

const normalizeKey = (key: string) =>
	key.replace(/^\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/\.\.\//, "/../../");

const libFilesNormalized = Object.fromEntries(
	Object.entries(libFiles).map(([k, v]) => [normalizeKey(k), v]),
);

const demoFilesNormalized = Object.fromEntries(
	Object.entries(demoFiles).map(([k, v]) => [
		k.startsWith("/") ? k : `/${k}`,
		v,
	]),
);

const docsBySlug = new Map<string, { content: string; filePath: string }>();

for (const [filePath, contents] of Object.entries(docFiles)) {
	const normalizedSlug = filePath
		.replace(/^\/src\/routes\/docs\//, "")
		.replace(/\/\+page\.svx$/, "");
	docsBySlug.set(normalizedSlug, { content: contents, filePath });
}

const normalizeSlug = (slug: string | undefined) => {
	if (!slug) return "";
	return slug
		.split("/")
		.map((segment) => segment.trim())
		.filter(Boolean)
		.join("/");
};

function resolveImportPath(
	currentFilePath: string,
	importPath: string,
): string | null {
	const cleanPath = importPath.replace(/\?raw$/, "");

	if (cleanPath.startsWith("motion-core/")) {
		return `/../../packages/motion-core/src/lib/${cleanPath.replace("motion-core/", "")}`;
	}

	if (cleanPath.startsWith(".")) {
		const segments = currentFilePath.split("/");
		segments.pop();
		const pathSegments = cleanPath.split("/");

		for (const segment of pathSegments) {
			if (segment === "..") segments.pop();
			else if (segment !== ".") segments.push(segment);
		}
		const result = segments.join("/");
		return result.startsWith("/") ? result : `/${result}`;
	}

	return null;
}

export const GET: RequestHandler = ({ params }) => {
	const slug = normalizeSlug(params.slug);
	const docEntry = docsBySlug.get(slug);

	if (!docEntry) {
		throw error(404, "Doc not found");
	}

	let { content } = docEntry;
	const { filePath } = docEntry;
	const resolvedSources: string[] = [];

	const rawImportRegex = /import\s+(\w+)\s+from\s+["']([^"']+\?raw)["']/gs;
	let match;

	while ((match = rawImportRegex.exec(content)) !== null) {
		const [_, _varName, importPath] = match;
		const resolvedPath = resolveImportPath(filePath, importPath);

		if (resolvedPath) {
			const fileContent =
				libFilesNormalized[resolvedPath] || demoFilesNormalized[resolvedPath];

			if (fileContent) {
				const ext = resolvedPath.split(".").pop() || "txt";
				const lang =
					ext === "svelte" ? "svelte" : ext === "ts" ? "typescript" : ext;
				const fileName = importPath.replace("?raw", "").split("/").pop();

				resolvedSources.push(
					`### ${fileName}\n\`\`\`${lang}\n${fileContent}\n\`\`\``,
				);
			}
		}
	}

	if (resolvedSources.length > 0) {
		content +=
			"\n\n---\n## Included Source Code\n\n" + resolvedSources.join("\n\n");
	}

	return new Response(content, {
		headers: {
			"content-type": "text/markdown; charset=utf-8",
			"cache-control": "public, max-age=60",
		},
	});
};
