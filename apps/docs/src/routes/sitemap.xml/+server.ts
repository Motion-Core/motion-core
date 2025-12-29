import type { RequestHandler } from "@sveltejs/kit";
import { docsManifest } from "$lib/docs/manifest";

type SitemapEntry = {
	path: string;
	changefreq?: string;
	priority?: string;
};

const staticPages: SitemapEntry[] = [
	{ path: "/", changefreq: "weekly", priority: "1.0" },
];

const buildTimestamp = new Date().toISOString();

const toAbsoluteUrl = (origin: string, path: string) =>
	new URL(path, origin).href;

const createUrlEntry = (origin: string, entry: SitemapEntry) => {
	const loc = toAbsoluteUrl(origin, entry.path);
	const changefreqTag = entry.changefreq
		? `<changefreq>${entry.changefreq}</changefreq>`
		: "";
	const priorityTag = entry.priority
		? `<priority>${entry.priority}</priority>`
		: "";

	return `<url><loc>${loc}</loc><lastmod>${buildTimestamp}</lastmod>${changefreqTag}${priorityTag}</url>`;
};

const dedupeEntries = (entries: SitemapEntry[]) => {
	const map = new Map<string, SitemapEntry>();
	for (const entry of entries) {
		if (!map.has(entry.path)) {
			map.set(entry.path, entry);
		}
	}
	return Array.from(map.values());
};

export const GET: RequestHandler = ({ url }) => {
	const docEntries: SitemapEntry[] = docsManifest.map((doc) => ({
		path: `/docs/${doc.slug}`,
		changefreq: "weekly",
		priority: "0.8",
	}));

	const uniqueEntries = dedupeEntries([...staticPages, ...docEntries]);
	const body =
		`<?xml version="1.0" encoding="UTF-8"?>` +
		`<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">` +
		uniqueEntries.map((entry) => createUrlEntry(url.origin, entry)).join("") +
		`</urlset>`;

	return new Response(body, {
		headers: {
			"content-type": "application/xml",
			"cache-control": "public, max-age=3600",
		},
	});
};
