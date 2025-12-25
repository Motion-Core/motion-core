import { docsManifest as componentManifest } from "./generated-manifest";
import type { ComponentInfo } from "../components/landing/types";

export const gettingStartedManifest: ComponentInfo[] = [
	{
		slug: "introduction",
		name: "Introduction",
	},
	{
		slug: "cli-guide",
		name: "CLI Guide",
	},
];

export { componentManifest };

export const docsManifest: ComponentInfo[] = [
	...gettingStartedManifest,
	...componentManifest,
];

export const getAdjacentDocs = (slug: string) => {
	const index = docsManifest.findIndex((doc) => doc.slug === slug);
	if (index === -1) {
		return { previous: null, next: null };
	}
	const previous = index > 0 ? docsManifest[index - 1] : null;
	const next = index < docsManifest.length - 1 ? docsManifest[index + 1] : null;
	return { previous, next };
};
