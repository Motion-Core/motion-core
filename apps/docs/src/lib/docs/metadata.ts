type DocMetadata = {
	href: `/docs/${string}`;
	slug: string;
	title: string;
	description?: string;
};

type DocMetadataMap = Record<string, DocMetadata>;

const docPageModules = import.meta.glob("/src/routes/docs/**/+page.svx", {
	eager: true,
	import: "default",
	query: "?raw",
}) as Record<string, string>;

const docsMetadata: DocMetadataMap = Object.entries(docPageModules).reduce(
	(acc, [path, contents]) => {
		const route = resolveRoutePath(path);

		if (!route) {
			return acc;
		}

		acc[route] = parseDocMetadata(route, contents);
		return acc;
	},
	{} as DocMetadataMap,
);

export function getDocMetadata(path: string) {
	return docsMetadata[normalizePath(path)] ?? null;
}

function parseDocMetadata(route: `/docs/${string}`, raw: string): DocMetadata {
	const slug = normalizePath(route).replace(/^\/docs\//, "");
	const sanitized = stripBlocks(raw);
	const headingMatch = sanitized.match(/^\s*#\s+(.+?)\s*$/m);
	const headingSource = headingMatch?.[0];
	const rawTitle = headingMatch?.[1];
	const title = cleanupText(rawTitle ?? slugToTitle(slug));
	const description = extractDescription(sanitized, headingSource);

	return {
		href: route,
		slug,
		title,
		description,
	};
}

function resolveRoutePath(filePath: string) {
	const match = filePath.match(/\/src\/routes(\/docs\/.+)\/\+page\.svx$/);
	if (!match) {
		return null;
	}

	const normalized = normalizePath(match[1] ?? "");
	if (!normalized.startsWith("/docs/")) {
		return null;
	}

	return normalized as `/docs/${string}`;
}

function stripBlocks(value: string) {
	return value
		.replace(/<script[\s\S]*?<\/script>/g, "")
		.replace(/<style[\s\S]*?<\/style>/g, "")
		.replace(/<!--[\s\S]*?-->/g, "")
		.trim();
}

function extractDescription(text: string, firstHeading?: string) {
	let remainder = text;

	if (firstHeading) {
		const idx = text.indexOf(firstHeading);
		if (idx >= 0) {
			remainder = text.slice(idx + firstHeading.length);
		}
	}

	const blocks = remainder
		.split(/\n{2,}/)
		.map((block) => block.trim())
		.filter(Boolean);

	for (const block of blocks) {
		if (!block) continue;
		if (block.startsWith("#")) break;
		if (block.startsWith("```")) continue;
		if (block.startsWith("<")) continue;
		if (block.startsWith("{@html")) continue;

		const cleaned = cleanupText(block)
			.replace(/```[\s\S]*?```/g, "")
			.replace(/\s+/g, " ")
			.trim();

		if (cleaned) {
			return truncate(cleaned, 180);
		}
	}

	return undefined;
}

function cleanupText(value: string) {
	return value
		.replace(/!\[[^\]]*?\]\([^)]+\)/g, "")
		.replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
		.replace(/`([^`]+)`/g, "$1")
		.replace(/[*_]{1,3}/g, "")
		.replace(/<\/?[^>]+>/g, "")
		.replace(/\{#.+?\}/g, "")
		.replace(/\{\/.+?\}/g, "")
		.replace(/&nbsp;/g, " ")
		.trim();
}

function truncate(value: string, limit: number) {
	if (value.length <= limit) {
		return value;
	}

	return `${value.slice(0, limit - 1).trimEnd()}…`;
}

function slugToTitle(slug: string) {
	return slug
		.split("/")
		.filter(Boolean)
		.map((segment) =>
			segment
				.replace(/[-_]/g, " ")
				.replace(/\b\w/g, (char) => char.toUpperCase()),
		)
		.join(" — ");
}

function normalizePath(path: string) {
	if (path === "/") return path;
	return path.replace(/\/+$/, "");
}

export type { DocMetadata };
