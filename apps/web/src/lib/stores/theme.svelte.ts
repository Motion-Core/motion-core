import { browser } from "$app/environment";
import { docsUiConfig } from "$lib/config/docs-ui";

export const themes = ["light", "dark"] as const;
export type Theme = (typeof themes)[number];

const storageKey = docsUiConfig.theme.storageKey;

function isTheme(value: string | null): value is Theme {
	return value === "light" || value === "dark";
}

function getPreferredTheme(): Theme {
	if (!browser) {
		return docsUiConfig.theme.defaultMode === "dark" ? "dark" : "light";
	}

	if (
		docsUiConfig.theme.defaultMode === "light" ||
		docsUiConfig.theme.defaultMode === "dark"
	) {
		return docsUiConfig.theme.defaultMode;
	}

	return window.matchMedia("(prefers-color-scheme: dark)").matches
		? "dark"
		: "light";
}

function applyTheme(theme: Theme) {
	if (!browser) {
		return;
	}

	const isDark = theme === "dark";
	document.documentElement.classList.toggle("dark", isDark);
	document.documentElement.style.colorScheme = isDark ? "dark" : "light";
}

function createThemeStore() {
	let current = $state<Theme>("light");

	if (browser) {
		const storedTheme = localStorage.getItem(storageKey);
		const initialTheme = isTheme(storedTheme)
			? storedTheme
			: getPreferredTheme();
		current = initialTheme;
		applyTheme(initialTheme);

		const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
		const handlePreferredThemeChange = () => {
			const savedTheme = localStorage.getItem(storageKey);
			if (isTheme(savedTheme)) {
				return;
			}

			const preferredTheme = mediaQuery.matches ? "dark" : "light";
			if (current !== preferredTheme) {
				current = preferredTheme;
				applyTheme(preferredTheme);
			}
		};

		mediaQuery.addEventListener("change", handlePreferredThemeChange);
	}

	const set = (theme: Theme) => {
		current = theme;
		if (browser) {
			localStorage.setItem(storageKey, theme);
		}
		applyTheme(theme);
	};

	const toggle = () => {
		set(current === "dark" ? "light" : "dark");
	};

	return {
		get current() {
			return current;
		},
		get isDark() {
			return current === "dark";
		},
		set,
		toggle,
	};
}

export const themeStore = createThemeStore();
