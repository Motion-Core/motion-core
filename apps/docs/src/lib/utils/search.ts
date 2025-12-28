type DocSection = {
	title: string;
	slug: string;
	heading?: string;
	anchor?: string;
	matchType: "title" | "heading" | "content";
	score: number;
	level?: number;
	content?: string;
	snippet?: string;
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

function stripMdx(content: string): string {
	return content
		.replace(/import\s+.*?;/g, "")
		.replace(/<script[\s\S]*?>[\s\S]*?<\/script>/g, "")
		.replace(/<style[\s\S]*?>[\s\S]*?<\/style>/g, "")
		.replace(/<[^>]+>/g, " ")
		.replace(/```[\s\S]*?```/g, "")
		.replace(/`([^`]+)`/g, "$1")
		.replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
		.replace(/[*_]{1,2}([^*_]+)[*_]{1,2}/g, "$1")
		.replace(/#{1,6}\s+/g, "")
		.replace(/\s+/g, " ")
		.trim();
}

function getSnippet(content: string, query: string, maxLength = 100): string {
	const lowerContent = content.toLowerCase();
	const lowerQuery = query.toLowerCase();
	const index = lowerContent.indexOf(lowerQuery);

	if (index === -1) return content.slice(0, maxLength);

	const start = Math.max(0, index - maxLength / 2);
	const end = Math.min(content.length, index + query.length + maxLength / 2);

	let snippet = content.slice(start, end);

	if (start > 0) snippet = "..." + snippet;
	if (end < content.length) snippet = snippet + "...";

	return snippet;
}

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
				anchor: "",
				matchType: "content",
				content: description,
				score: 0,
			});
		}

		const contentBody = rawContent.replace(/^---\n[\s\S]+?\n---\n/, "");

		const lines = contentBody.split("\n");
		let currentHeading: string | undefined = undefined;
		let currentAnchor = "";
		let currentContentBuffer: string[] = [];

		const flushBuffer = () => {
			if (currentContentBuffer.length > 0) {
				const text = stripMdx(currentContentBuffer.join(" "));
				if (text.length > 10) {
					index.push({
						title: title,
						slug,
						heading: currentHeading ?? title,
						anchor: currentAnchor,
						matchType: "content",
						content: text,
						score: 0,
					});
				}
				currentContentBuffer = [];
			}
		};

		for (const line of lines) {
			const headingMatch = line.match(/^(#{2,4})\s+(.+)$/);

			if (headingMatch) {
				flushBuffer();

				const level = headingMatch[1].length;
				const text = headingMatch[2].trim();
				const anchor = `#${slugify(text)}`;

				currentHeading = text;
				currentAnchor = anchor;

				index.push({
					title: title,
					slug,
					heading: text,
					anchor,
					matchType: "heading",
					score: 0,
					level,
				});
			} else {
				if (
					line.trim() &&
					!line.trim().startsWith("import") &&
					!line.trim().startsWith("---")
				) {
					currentContentBuffer.push(line);
				}
			}
		}
		flushBuffer();
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
		let snippet: string | undefined;

		const titleMatch = item.title.toLowerCase().includes(normalizedQuery);
		const headingMatch = item.heading?.toLowerCase().includes(normalizedQuery);
		const contentMatch = item.content?.toLowerCase().includes(normalizedQuery);

		if (item.matchType === "title" && titleMatch) {
			score += 10;
			if (item.title.toLowerCase().startsWith(normalizedQuery)) score += 5;
		} else if (item.matchType === "heading" && headingMatch) {
			score += 5;
		} else if (item.matchType === "content" && contentMatch) {
			score += 1;
			if (item.content) {
				snippet = getSnippet(item.content, query);
			}
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
			} else {
				group.children.push({ ...item, score, snippet });
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

		const topChildren = group.children.slice(0, 5);

		flatResults.push(...topChildren);
	}

	return flatResults;
}
