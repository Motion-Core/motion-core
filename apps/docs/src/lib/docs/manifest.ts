import type { ComponentInfo } from "../components/landing";

type DocModule = {
	metadata?: {
		name?: string;
		thumbnail?: string;
		video?: string;
		poster?: string;
	};
};

const docModules = import.meta.glob<DocModule>(
	"/src/routes/docs/**/+page.svx",
	{
		eager: true,
	},
);

const formatName = (slug: string) =>
	slug
		.split("/")
		.pop()
		?.replace(/-/g, " ")
		.replace(/\b\w/g, (char) => char.toUpperCase()) ?? slug;

const normalizeSlug = (path: string) =>
	path
		.replace("/src/routes/docs/", "")
		.replace(/\/\+page\.svx$/, "")
		.replace(/\.svx$/, "");

export const docsManifest: ComponentInfo[] = Object.entries(docModules)
	.map(([path, module]) => {
		const slug = normalizeSlug(path);
		const metadata = module.metadata ?? {};

		return {
			slug,
			name: metadata.name ?? formatName(slug),
			image: metadata.thumbnail ?? "/og-image.jpg",
			video: metadata.video,
			poster: metadata.poster,
		};
	})
	.sort((a, b) => a.name.localeCompare(b.name));

export const getAdjacentDocs = (slug: string) => {
	const index = docsManifest.findIndex((doc) => doc.slug === slug);

	if (index === -1) {
		return { previous: null, next: null };
	}

	const previous = index > 0 ? docsManifest[index - 1] : null;
	const next = index < docsManifest.length - 1 ? docsManifest[index + 1] : null;

	return { previous, next };
};
