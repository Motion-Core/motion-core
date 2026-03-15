import type { RequestHandler } from "./$types";
import { docsManifest, getDocMetadata, siteConfig } from "$lib";

type DocEntry = {
	slug: string;
	fallbackTitle: string;
	category: string;
};

const summary = `${siteConfig.name} is ${siteConfig.description}`;

const detailParagraphs = [
	"LLM-friendly Markdown for every page is available at `/docs/raw/<slug>`; this is the source doc content without navigation chrome.",
	"Use `/sitemap.xml` for URL discovery and `/robots.txt` for crawl guidance.",
];

const buildDocEntry = (origin: string, entry: DocEntry) => {
	const metadata = getDocMetadata(`/docs/${entry.slug}`);
	const title = metadata?.title ?? entry.fallbackTitle;
	const description = metadata?.description ?? `Documentation for ${title}.`;
	const link = new URL(`/docs/raw/${entry.slug}`, origin).href;
	return `- [${title}](${link}): ${description}`;
};

const dedupeDocs = (entries: DocEntry[]) => {
	const map = new Map<string, DocEntry>();
	for (const entry of entries) {
		if (!map.has(entry.slug)) {
			map.set(entry.slug, entry);
		}
	}
	return Array.from(map.values());
};

const groupByCategory = (entries: DocEntry[]) => {
	const grouped = new Map<string, DocEntry[]>();
	for (const entry of entries) {
		const list = grouped.get(entry.category);
		if (list) {
			list.push(entry);
		} else {
			grouped.set(entry.category, [entry]);
		}
	}
	return grouped;
};

const buildSection = (title: string, items: string[]) => {
	if (items.length === 0) return [];
	return [`## ${title}`, "", ...items];
};

export const GET: RequestHandler = () => {
	const canonicalOrigin = new URL(siteConfig.url).origin;
	const optionalLinks = [
		`- [GitHub](${siteConfig.links.github}): Source code, issues, and discussions.`,
		`- [Package](https://www.npmjs.com/package/${siteConfig.package.name}): Installation and release metadata.`,
	];

	const docs = dedupeDocs(
		docsManifest.map((doc) => ({
			slug: doc.slug,
			fallbackTitle: doc.name,
			category: doc.category ?? "Documentation",
		})),
	);
	const groupedDocs = groupByCategory(docs);

	const categorySections = Array.from(groupedDocs.entries()).flatMap(
		([category, entries]) =>
			buildSection(
				category,
				entries.map((entry) => buildDocEntry(canonicalOrigin, entry)),
			),
	);

	const lines = [
		`# ${siteConfig.name}`,
		"",
		`> ${summary}`,
		"",
		...detailParagraphs,
		"",
		...categorySections,
		"",
		...buildSection("Optional", optionalLinks),
		"",
	];

	const body =
		lines
			.join("\n")
			.replace(/\n{3,}/g, "\n\n")
			.trim() + "\n";

	return new Response(body, {
		headers: {
			"content-type": "text/plain; charset=utf-8",
			"cache-control": "public, max-age=3600",
		},
	});
};
