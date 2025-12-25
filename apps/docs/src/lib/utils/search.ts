type DocSection = {
	title: string;
	slug: string;
	heading?: string;
	anchor?: string;
	matchType: "title" | "heading" | "content";
	score: number;
	level?: number;
};

type DocModule = {
	default: string;
	metadata?: {
		name?: string;
		title?: string;
		description?: string;
	};
};

const docModules = import.meta.glob<string>("/src/routes/docs/**/*.svx", {
	query: "?raw",
	eager: true,
	import: "default",
});

const metaModules = import.meta.glob<DocModule>("/src/routes/docs/**/*.svx", {
	eager: true,
});

const slugify = (value: string) =>
	value
		.normalize("NFD")
		.replace(/[\u0300-\u036f]/g, "")
		.toLowerCase()
		.trim()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");

function parseDocs() {
	const index: DocSection[] = [];

	for (const path in docModules) {
		const rawContent = docModules[path];
		const meta = metaModules[path]?.metadata ?? {};

		const cleanPath = path
			.replace("/src/routes/docs/", "")
			.replace("/+page.svx", "");
		const slug = `/docs/${cleanPath}`;

		const title = meta.name || meta.title || cleanPath;
		const description = meta.description || "";

		index.push({
			title: title,
			slug,
			matchType: "title",
			score: 0,
		});

		if (description) {
			index.push({
				title: title,
				slug,
				heading: "Description",
				matchType: "content",
				score: 0,
			});
		}

		const headingRegex = /^(#{2,4})\s+(.+)$/gm;
		let match;

		while ((match = headingRegex.exec(rawContent)) !== null) {
			const level = match[1].length;
			const text = match[2].trim();
			const anchor = slugify(text);

			index.push({
				title: title,
				slug,
				heading: text,
				anchor: `#${anchor}`,
				matchType: "heading",
				score: 0,
				level,
			});
		}
	}

	return index;
}

const searchIndex = parseDocs();

const pageLookup = new Map<string, string>();
searchIndex.forEach((item) => {
	if (item.matchType === "title") {
		pageLookup.set(item.slug, item.title);
	}
});

export function searchDocs(query: string): DocSection[] {
	if (!query) return [];

	const normalizedQuery = query.toLowerCase();

	const groups = new Map<
		string,
		{ parent: DocSection; children: DocSection[]; maxScore: number }
	>();

	for (const item of searchIndex) {
		let score = 0;
		const titleMatch = item.title.toLowerCase().includes(normalizedQuery);
		const headingMatch = item.heading?.toLowerCase().includes(normalizedQuery);

		if (item.matchType === "title" && titleMatch) {
			score += 10;
			if (item.title.toLowerCase().startsWith(normalizedQuery)) score += 5;
		} else if (item.matchType === "heading" && headingMatch) {
			score += 5;
		} else if (item.matchType === "content" && titleMatch) {
			score += 1;
		}

		if (score > 0) {
			if (!groups.has(item.slug)) {
				groups.set(item.slug, {
					parent: {
						title: pageLookup.get(item.slug) || item.title,
						slug: item.slug,
						matchType: "title",
						score: 0,
					},
					children: [],
					maxScore: 0,
				});
			}

			const group = groups.get(item.slug)!;

			if (item.matchType === "title") {
				group.parent = { ...item, score };
			} else if (item.matchType === "heading") {
				group.children.push({ ...item, score });
			}

			if (score > group.maxScore) {
				group.maxScore = score;
			}
		}
	}

	const sortedGroups = Array.from(groups.values())
		.sort((a, b) => {
			const scoreDiff = b.maxScore - a.maxScore;
			if (scoreDiff !== 0) return scoreDiff;
			return a.parent.title.localeCompare(b.parent.title);
		})
		.slice(0, 20);

	const flatResults: DocSection[] = [];

	for (const group of sortedGroups) {
		flatResults.push(group.parent);
		group.children.sort((a, b) => b.score - a.score);
		flatResults.push(...group.children);
	}

	return flatResults;
}
