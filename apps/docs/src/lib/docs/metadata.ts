type DocMetadata = {
	href: `/docs/${string}`;
	slug: string;
	title: string;
	description?: string;
};

type DocModule = {
	metadata?: {
		title?: string;
		name?: string;
		description?: string;
	};
};

const docModules = import.meta.glob<DocModule>("/src/routes/docs/**/+page.svx", {
	eager: true,
});

export function getDocMetadata(path: string): DocMetadata | null {
	const normalizedPath = normalizePath(path);
	const filePath = `/src/routes${normalizedPath}/+page.svx`;
	const module = docModules[filePath];

	if (!module || !module.metadata) {
		return null;
	}

	const slug = normalizedPath.replace(/^\/docs\//, "");
	const { title, name, description } = module.metadata;

	return {
		href: normalizedPath as `/docs/${string}`,
		slug,
		title: name || title || slugToTitle(slug),
		description,
	};
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
