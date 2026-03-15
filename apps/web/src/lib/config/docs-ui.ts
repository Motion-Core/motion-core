/**
 * Supported package manager tabs shown in installation examples.
 */
export const availablePackageManagers = ["npm", "pnpm", "bun", "yarn"] as const;

/**
 * Union of package manager keys derived from `availablePackageManagers`.
 */
export type PackageManagerOption = (typeof availablePackageManagers)[number];

/**
 * Global, strongly-typed settings for interactive documentation UI elements.
 * Adjust defaults here to tune behavior across the entire docs experience.
 */
export type DocsUiConfig = {
	search: {
		enabled: boolean;
		triggerPlaceholder: string;
		dialogPlaceholder: string;
		noResultsLabel: string;
		submitHintLabel: string;
		hotkey: {
			enabled: boolean;
			key: string;
			metaOrCtrl: boolean;
			label: string;
		};
		maxGroups: number;
		maxChildrenPerGroup: number;
	};
	sidebar: {
		navigationLabel: string;
		showThemeToggle: boolean;
		showRepositoryLink: boolean;
		repositoryAriaLabel: string;
	};
	toc: {
		enabled: boolean;
		title: string;
		emptyLabel: string;
		minViewportWidth: number;
		defaultSelector: string;
		selectorOverrides: Array<{
			slugPrefix: string;
			selector: string;
		}>;
	};
	docActions: {
		enabled: boolean;
		showCopyMarkdown: boolean;
		showRepositoryLink: boolean;
		repositoryLinkLabel: string;
		repositoryBranch: string;
		moreActionsAriaLabel: string;
		copyLabels: {
			desktopIdle: string;
			mobileIdle: string;
			copying: string;
			success: string;
			error: string;
		};
		assistantPromptTemplate: string;
		assistants: {
			chatgpt: {
				enabled: boolean;
				label: string;
				hrefTemplate: string;
			};
			claude: {
				enabled: boolean;
				label: string;
				hrefTemplate: string;
			};
		};
	};
	pagination: {
		enabled: boolean;
		previousLabel: string;
		nextLabel: string;
	};
	packageManager: {
		enabled: PackageManagerOption[];
		default: PackageManagerOption;
		storageKey: string;
	};
	theme: {
		storageKey: string;
		defaultMode: "light" | "dark" | "system";
	};
};

/**
 * Centralized interactive docs configuration used by route components and helpers.
 */
export const docsUiConfig: DocsUiConfig = {
	search: {
		enabled: true,
		triggerPlaceholder: "Search...",
		dialogPlaceholder: "Search documentation...",
		noResultsLabel: "No results found.",
		submitHintLabel: "Go to page",
		hotkey: {
			enabled: true,
			key: "k",
			metaOrCtrl: true,
			label: "⌘K",
		},
		maxGroups: 20,
		maxChildrenPerGroup: 5,
	},
	sidebar: {
		navigationLabel: "Docs",
		showThemeToggle: true,
		showRepositoryLink: true,
		repositoryAriaLabel: "Open project repository",
	},
	toc: {
		enabled: true,
		title: "On this page",
		emptyLabel: "No headings",
		minViewportWidth: 1280,
		defaultSelector: "[data-doc-content] h2, [data-doc-content] h3",
		selectorOverrides: [
			{ slugPrefix: "changelog/", selector: "[data-doc-content] h2" },
		],
	},
	docActions: {
		enabled: true,
		showCopyMarkdown: true,
		showRepositoryLink: true,
		repositoryLinkLabel: "Open in GitHub",
		repositoryBranch: "master",
		moreActionsAriaLabel: "More actions",
		copyLabels: {
			desktopIdle: "Copy as Markdown",
			mobileIdle: "Copy Markdown",
			copying: "Copying…",
			success: "Copied!",
			error: "Copy failed",
		},
		assistantPromptTemplate:
			"I'm currently viewing the documentation at {url}. Please assist me in learning how to work with it. I may need clarification on concepts, sample code demonstrations, or troubleshooting guidance related to this documentation.",
		assistants: {
			chatgpt: {
				enabled: true,
				label: "Open in ChatGPT",
				hrefTemplate: "https://chatgpt.com/?hints=search&prompt={prompt}",
			},
			claude: {
				enabled: true,
				label: "Open in Claude",
				hrefTemplate: "https://claude.ai/new?q={prompt}",
			},
		},
	},
	pagination: {
		enabled: true,
		previousLabel: "Previous",
		nextLabel: "Next",
	},
	packageManager: {
		enabled: ["npm", "pnpm", "bun", "yarn"],
		default: "npm",
		storageKey: "docs-package-manager",
	},
	theme: {
		storageKey: "docs-theme",
		defaultMode: "system",
	},
};

/**
 * Replaces `{token}` placeholders in a template string with provided values.
 *
 * @param template String that may contain `{key}` placeholders.
 * @param variables Object with replacement values indexed by key.
 * @returns Template with placeholders replaced; missing keys resolve to an empty string.
 */
function interpolateTemplate(
	template: string,
	variables: Record<string, string>,
) {
	return template.replace(
		/\{([a-zA-Z0-9_]+)\}/g,
		(_, key: string) => variables[key] ?? "",
	);
}

/**
 * Resolves the heading selector for table-of-contents generation based on a doc slug.
 *
 * @param slug Relative documentation slug (for example `changelog/1.0.0`).
 * @returns CSS selector used to extract headings from page content.
 */
export function resolveTocSelector(slug?: string | null) {
	const normalizedSlug = slug ?? "";
	const override = docsUiConfig.toc.selectorOverrides.find((item) =>
		normalizedSlug.startsWith(item.slugPrefix),
	);
	return override?.selector ?? docsUiConfig.toc.defaultSelector;
}

/**
 * Builds AI assistant links (ChatGPT/Claude) for the current documentation URL.
 *
 * @param rawUrl Absolute URL of the current docs page.
 * @returns Encoded assistant URLs when enabled; otherwise `null` values.
 */
export function resolveDocAssistantUrls(rawUrl?: string | null) {
	if (!rawUrl) {
		return {
			chatGptUrl: null,
			claudeUrl: null,
		};
	}

	const prompt = interpolateTemplate(
		docsUiConfig.docActions.assistantPromptTemplate,
		{
			url: rawUrl,
		},
	);
	const encodedPrompt = encodeURIComponent(prompt);
	const encodedUrl = encodeURIComponent(rawUrl);
	const templateVars = {
		prompt: encodedPrompt,
		url: rawUrl,
		encodedUrl,
	};

	return {
		chatGptUrl: docsUiConfig.docActions.assistants.chatgpt.enabled
			? interpolateTemplate(
					docsUiConfig.docActions.assistants.chatgpt.hrefTemplate,
					templateVars,
				)
			: null,
		claudeUrl: docsUiConfig.docActions.assistants.claude.enabled
			? interpolateTemplate(
					docsUiConfig.docActions.assistants.claude.hrefTemplate,
					templateVars,
				)
			: null,
	};
}

/**
 * Creates a deep link to a documentation file in the source repository.
 *
 * @param repositoryBaseUrl Repository root URL (for example `https://github.com/org/repo`).
 * @param repositoryRelativePath File path prefixed with `/` from repository root.
 * @returns URL pointing to the configured branch and target file.
 */
export function resolveRepositoryDocUrl(
	repositoryBaseUrl: string,
	repositoryRelativePath: string,
) {
	const branch = docsUiConfig.docActions.repositoryBranch.trim();
	const safeBranch = branch.length > 0 ? branch : "main";
	return `${repositoryBaseUrl}/blob/${safeBranch}${repositoryRelativePath}`;
}
