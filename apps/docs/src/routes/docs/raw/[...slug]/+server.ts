import { error, type RequestHandler } from "@sveltejs/kit";

const docFiles = import.meta.glob("/src/routes/docs/**/+page.svx", {
	query: "?raw",
	import: "default",
	eager: true,
}) as Record<string, string>;

const docsBySlug = new Map<string, string>();
for (const [filePath, contents] of Object.entries(docFiles)) {
	const normalizedSlug = filePath
		.replace(/^\/src\/routes\/docs\//, "")
		.replace(/\/\+page\.svx$/, "");
	docsBySlug.set(normalizedSlug, contents);
}

const normalizeSlug = (slug: string | undefined) => {
	if (!slug) return "";
	return slug
		.split("/")
		.map((segment) => segment.trim())
		.filter(Boolean)
		.join("/");
};

export const GET: RequestHandler = ({ params }) => {
	const slug = normalizeSlug(params.slug);

	if (!slug) {
		throw error(404, "Doc not found");
	}

	const contents = docsBySlug.get(slug);

	if (!contents) {
		throw error(404, "Doc not found");
	}

	return new Response(contents, {
		headers: {
			"content-type": "text/markdown; charset=utf-8",
			"cache-control": "public, max-age=60",
		},
	});
};
