import type { LayoutLoad } from "./$types";
import { getAdjacentDocs, getDocBySlug, getDocMetadata } from "$lib";

function pathToSlug(pathname: string): string {
	const normalized = pathname.replace(/\/+$/, "");
	if (normalized === "/docs" || normalized === "") return "";
	return normalized.replace(/^\/docs\//, "");
}

export const load: LayoutLoad = ({ url }) => {
	const slug = pathToSlug(url.pathname);
	const currentDoc = getDocBySlug(slug);
	const { previous, next } = getAdjacentDocs(slug);
	const metadata = getDocMetadata(url.pathname);

	return {
		slug,
		metadata,
		currentDoc: {
			...currentDoc,
			title: metadata?.title ?? currentDoc?.name ?? "Untitled",
			description: metadata?.description ?? "",
		},
		previousDoc: previous,
		nextDoc: next,
		docOrigin: url.origin,
	};
};
