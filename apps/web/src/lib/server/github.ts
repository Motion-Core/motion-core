const GITHUB_REPO_URL = "https://api.github.com/repos/motion-core/motion-core";

type GitHubHeaders = {
	Accept: string;
	"User-Agent": string;
	Authorization?: string;
};

export const fetchGitHubStars = async (): Promise<number | null> => {
	const headers: GitHubHeaders = {
		Accept: "application/vnd.github+json",
		"User-Agent": "motion-core-docs",
	};

	const token = process.env.GITHUB_TOKEN;
	if (token) {
		headers.Authorization = `Bearer ${token}`;
	}

	try {
		const response = await fetch(GITHUB_REPO_URL, {
			headers,
		});

		if (!response.ok) {
			console.warn(
				"Failed to fetch GitHub stars:",
				response.status,
				response.statusText,
			);
			return null;
		}

		const data = (await response.json()) as { stargazers_count?: number };
		if (
			typeof data.stargazers_count === "number" &&
			Number.isFinite(data.stargazers_count)
		) {
			return data.stargazers_count;
		}

		return null;
	} catch (error) {
		console.error("GitHub stars API error:", error);
		return null;
	}
};
