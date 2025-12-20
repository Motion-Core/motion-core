import type { PageLoad } from "./$types";
import { docsManifest } from "$lib/docs/manifest";

export const load: PageLoad = () => {
	return {
		components: docsManifest,
	};
};
