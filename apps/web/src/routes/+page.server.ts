import type { PageServerLoad } from "./$types";
import { fetchGitHubStars } from "$lib/server/github";

export const load: PageServerLoad = async () => {
	const githubStars = await fetchGitHubStars();

	return {
		githubStars,
	};
};
