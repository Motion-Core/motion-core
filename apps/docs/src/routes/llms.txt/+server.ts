import type { RequestHandler } from "@sveltejs/kit";
import {
	componentManifest,
	gettingStartedManifest,
} from "$lib/docs/manifest";
import { getDocMetadata } from "$lib/docs/metadata";

const summary =
	"Motion Core is a Svelte-native motion toolkit with GSAP and Three.js powered components, demos, and CLI-driven workflows.";

const detailParagraphs = [
	"Use the Motion Core CLI to scaffold installations (`npx @motion-core/cli add <component-slug>`) or explore ready-made demos directly in the docs. Each example highlights dependencies, implementation snippets, and customization tips.",
	"LLM-friendly Markdown for every doc lives at `/docs/raw/<slug>`; this is the same content rendered on the site, but without UI chrome. Combine those endpoints with `/sitemap.xml` for discovery or `/robots.txt` for crawl guidance.",
	"Key resources:",
	"- Registry entries live under `/docs/<slug>` where `<slug>` comes from the manifest below.",
	"- `/apps/docs/src/lib/docs/generated-manifest.ts` in the repo exposes dependency data for each component if you need to script installs.",
];

type DocEntry = {
	slug: string;
	fallbackTitle: string;
};

const buildDocEntry = (origin: string, entry: DocEntry) => {
	const metadata = getDocMetadata(`/docs/${entry.slug}`);
	const title = metadata?.title ?? entry.fallbackTitle;
	const description =
		metadata?.description ?? `Documentation for ${title} component.`;
	const link = new URL(`/docs/raw/${entry.slug}`, origin).href;
	return `- [${title}](${link}): ${description}`;
};

const buildSection = (title: string, items: string[]) => {
	if (items.length === 0) return [];
	return [`## ${title}`, "", ...items];
};

const buildOptionalSection = (items: string[]) =>
	buildSection("Optional", items);

const optionalLinks = [
	"- [Motion Core GitHub](https://github.com/motion-core/motion-core): Full source, issues, and design discussions.",
	"- [Component manifest JSON](https://github.com/motion-core/motion-core/blob/master/apps/docs/src/lib/docs/generated-manifest.ts): Dependency metadata for each demo.",
];

export const GET: RequestHandler = ({ url }) => {
	const origin = url.origin;

	const gettingStartedEntries = gettingStartedManifest.map((doc) =>
		buildDocEntry(origin, { slug: doc.slug, fallbackTitle: doc.name }),
	);

	const componentEntries = componentManifest.map((doc) =>
		buildDocEntry(origin, { slug: doc.slug, fallbackTitle: doc.name }),
	);

	const lines = [
		"# Motion Core",
		"",
		`> ${summary}`,
		"",
		...detailParagraphs,
		"",
		...buildSection("Getting Started", gettingStartedEntries),
		"",
		...buildSection("Component Demos", componentEntries),
		"",
		...buildOptionalSection(optionalLinks),
		"",
	];

	const body = lines.join("\n").replace(/\n{3,}/g, "\n\n").trim() + "\n";

	return new Response(body, {
		headers: {
			"content-type": "text/plain; charset=utf-8",
			"cache-control": "public, max-age=3600",
		},
	});
};
