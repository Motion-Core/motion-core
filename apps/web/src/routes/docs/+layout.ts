import type { LayoutLoad } from "./$types";
import { getAdjacentDocs, getDocBySlug, getDocMetadata } from "$lib";
import { docsManifest as generatedDocsManifest } from "$lib/docs/generated-manifest";

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
	const componentDoc = generatedDocsManifest.find((doc) => doc.slug === slug);
	const componentDependencies = Object.entries(componentDoc?.dependencies ?? {})
		.map(([name]) => ({
			name,
			label: name,
			href: `https://www.npmjs.com/package/${name}`,
		}))
		.sort((a, b) => a.name.localeCompare(b.name));

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
		componentDependencies,
		docOrigin: url.origin,
	};
};
