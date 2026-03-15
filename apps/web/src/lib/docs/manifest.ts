import type { DocItem } from "../types/doc";
import { docsNavigation } from "$lib/config/navigation";

const flattenNavigationToManifest = (
	items: DocItem[],
	parentCategory?: string,
): DocItem[] => {
	const manifest: DocItem[] = [];

	for (const item of items) {
		const effectiveCategory = item.category ?? parentCategory;

		if (item.items?.length) {
			const childCategory = effectiveCategory ?? item.name;
			manifest.push(...flattenNavigationToManifest(item.items, childCategory));
			continue;
		}

		manifest.push({
			slug: item.slug,
			name: item.name,
			category: effectiveCategory,
		});
	}

	return manifest;
};

export const docsManifest: DocItem[] =
	flattenNavigationToManifest(docsNavigation);

export const getDocBySlug = (slug: string) => {
	return docsManifest.find((doc) => doc.slug === slug);
};

export const getDocHref = (slug: string) => {
	return slug ? `/docs/${slug}` : "/docs";
};

export const getAdjacentDocs = (slug: string) => {
	const index = docsManifest.findIndex((doc) => doc.slug === slug);
	if (index === -1) {
		return { previous: null, next: null };
	}
	const previous = index > 0 ? docsManifest[index - 1] : null;
	const next = index < docsManifest.length - 1 ? docsManifest[index + 1] : null;
	return { previous, next };
};
