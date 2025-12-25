import type { PageLoad } from "./$types";
import { componentManifest } from "$lib/docs/manifest";

export const load: PageLoad = () => {
	return {
		components: componentManifest,
	};
};
