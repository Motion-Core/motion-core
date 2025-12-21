import type { LayoutLoad } from "./$types";
import { getAdjacentDocs } from "$lib/docs/manifest";
import { getDocMetadata } from "$lib/docs/metadata";

const extractSlug = (pathname: string) => {
	const normalized = pathname.replace(/\/$/, "");
	const slug = normalized.replace(/^\/docs\/?/, "").split("/")[0] ?? "";
	return slug;
};

export const load: LayoutLoad = ({ url }) => {
	const slug = extractSlug(url.pathname);
	const { previous, next } = slug
		? getAdjacentDocs(slug)
		: { previous: null, next: null };

	const metadata = getDocMetadata(url.pathname);

	return {
		previousLink: previous
			? { title: previous.name, href: `/docs/${previous.slug}` }
			: null,
		nextLink: next ? { title: next.name, href: `/docs/${next.slug}` } : null,
		metadata,
	};
};
