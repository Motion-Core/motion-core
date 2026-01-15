import { fetchGitHubStars } from "$lib/server/github";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ setHeaders }) => {
	const stars = await fetchGitHubStars();

	setHeaders({
		"cache-control": "public, max-age=3600, s-maxage=3600",
	});

	return {
		stars,
	};
};
