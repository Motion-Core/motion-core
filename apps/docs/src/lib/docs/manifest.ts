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
		items: [
			{
				slug: "cli-guide/introduction",
				name: "Introduction",
			},
			{
				slug: "cli-guide/init",
				name: "Init",
			},
			{
				slug: "cli-guide/add",
				name: "Add",
			},
			{
				slug: "cli-guide/list",
				name: "List",
			},
			{
				slug: "cli-guide/cache",
				name: "Cache",
			},
		],
	},
];

export { componentManifest };

function flattenManifest(
	items: ComponentInfo[],
	category?: string,
): ComponentInfo[] {
	return items.reduce<ComponentInfo[]>((acc, item) => {
		if (item.items) {
			acc.push(...flattenManifest(item.items, item.name));
		} else {
			acc.push({ ...item, category: item.category ?? category });
		}
		return acc;
	}, []);
}

export const docsManifest: ComponentInfo[] = [
	...flattenManifest(gettingStartedManifest),
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
