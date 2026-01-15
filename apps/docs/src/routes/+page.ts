import type { PageLoad } from "./$types";
import { componentManifest } from "$lib/docs/manifest";

export const load: PageLoad = ({ data }) => {
	return {
		...data,
		components: componentManifest,
	};
};
