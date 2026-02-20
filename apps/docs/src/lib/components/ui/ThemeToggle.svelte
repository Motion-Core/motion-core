<script lang="ts">
	import { onMount } from "svelte";
	import { browser } from "$app/environment";
	import type { ThemeMode } from "./types";
	import { gsap } from "gsap/dist/gsap";

	type ConcreteTheme = Exclude<ThemeMode, "system">;
	const themeStorageKey = "motion-core-theme";
	let theme = $state<ThemeMode>("system");
	let systemPreference: ConcreteTheme = "light";
	let mediaQuery: MediaQueryList | null = null;

	const resolveTheme = (value: ThemeMode): ConcreteTheme =>
		value === "system" ? systemPreference : value;

	const applyTheme = (value: ThemeMode) => {
		if (!browser) return;
		const nextTheme = resolveTheme(value);
		document.documentElement.classList.toggle("dark", nextTheme === "dark");
	};

	const applyThemeWithVT = (value: ThemeMode) => {
		if (!browser) return;

		if (!document.startViewTransition) {
			applyTheme(value);
			return;
		}

		const html = document.documentElement;
		html.classList.add("theme-vt");

		const vt = document.startViewTransition(() => {
			applyTheme(value);
		});

		vt.finished.finally(() => {
			html.classList.remove("theme-vt");
		});
	};

	const setTheme = (value: ThemeMode) => {
		theme = value;
		if (!browser) return;

		applyThemeWithVT(value);
		window.localStorage.setItem(themeStorageKey, value);
	};

	const buttonBaseClasses =
		"relative z-10 inline-flex size-6 items-center justify-center rounded-full border border-transparent text-[10px] transition-colors duration-250 ease-out";

	const getThemeButtonClasses = (mode: ThemeMode) =>
		mode === theme
			? `${buttonBaseClasses} text-fixed-light`
			: `${buttonBaseClasses} text-foreground/70`;

	let containerRef: HTMLDivElement | null = null;
	let lightButton: HTMLButtonElement | null = null;
	let systemButton: HTMLButtonElement | null = null;
	let darkButton: HTMLButtonElement | null = null;
	let highlightTween: gsap.core.Tween | null = null;

	const getButtonForMode = (mode: ThemeMode) => {
		if (mode === "light") return lightButton;
		if (mode === "system") return systemButton;
		return darkButton;
	};

	const applyHighlight = (options: { immediate?: boolean } = {}) => {
		if (!browser || !containerRef) return;
		const button = getButtonForMode(theme);
		if (!button) return;

		const x = button.offsetLeft;
		const y = button.offsetTop;
		const width = button.offsetWidth;
		const height = button.offsetHeight;

		const vars = {
			"--toggle-highlight-x": `${x}px`,
			"--toggle-highlight-y": `${y}px`,
			"--toggle-highlight-width": `${width}px`,
			"--toggle-highlight-height": `${height}px`,
		};

		if (options.immediate) {
			gsap.set(containerRef, vars);
			return;
		}

		highlightTween?.kill();
		highlightTween = gsap.to(containerRef, {
			...vars,
			duration: 0.45,
			ease: "power3.out",
		});
	};

	onMount(() => {
		if (!browser) return;

		const stored = window.localStorage.getItem(themeStorageKey);
		if (stored === "dark" || stored === "light" || stored === "system") {
			theme = stored;
		} else {
			theme = "system";
		}

		mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

		const syncSystemPreference = (matches: boolean) => {
			systemPreference = matches ? "dark" : "light";
			if (theme === "system") {
				applyTheme("system");
			}
		};

		const handlePreferenceChange = (event: MediaQueryListEvent) => {
			syncSystemPreference(event.matches);
		};

		syncSystemPreference(mediaQuery.matches);
		applyTheme(theme);
		mediaQuery.addEventListener("change", handlePreferenceChange);

		applyHighlight({ immediate: true });

		return () => {
			mediaQuery?.removeEventListener("change", handlePreferenceChange);
		};
	});

	$effect(() => {
		void theme;
		applyHighlight();
	});
</script>

<div class="space-y-2">
	<div
		class="theme-toggle input-highlight relative inline-flex items-center gap-1 rounded-full bg-background p-1"
		bind:this={containerRef}
	>
		<button
			type="button"
			class={getThemeButtonClasses("light")}
			onclick={() => setTheme("light")}
			aria-label="Use light theme"
			aria-pressed={theme === "light"}
			bind:this={lightButton}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				aria-hidden="true"
				width="14"
				height="14"
				fill="currentColor"
				viewBox="0 0 256 256"
				><path
					d="M120,40V16a8,8,0,0,1,16,0V40a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.07,64.07,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.05,48.05,0,0,0,176,128ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-16-16A8,8,0,0,0,42.34,53.66Zm0,116.68-16,16a8,8,0,0,0,11.32,11.32l16-16a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l16-16a8,8,0,0,0-11.32-11.32l-16,16A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l16,16a8,8,0,0,0,11.32-11.32ZM48,128a8,8,0,0,0-8-8H16a8,8,0,0,0,0,16H40A8,8,0,0,0,48,128Zm80,80a8,8,0,0,0-8,8v24a8,8,0,0,0,16,0V216A8,8,0,0,0,128,208Zm112-88H216a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16Z"
				></path></svg
			>
		</button>
		<button
			type="button"
			class={getThemeButtonClasses("system")}
			onclick={() => setTheme("system")}
			aria-label="Use system theme"
			aria-pressed={theme === "system"}
			bind:this={systemButton}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				aria-hidden="true"
				width="14"
				height="14"
				fill="currentColor"
				viewBox="0 0 256 256"
				><path
					d="M232,168h-8V72a24,24,0,0,0-24-24H56A24,24,0,0,0,32,72v96H24a8,8,0,0,0-8,8v16a24,24,0,0,0,24,24H216a24,24,0,0,0,24-24V176A8,8,0,0,0,232,168ZM48,72a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8v96H48ZM224,192a8,8,0,0,1-8,8H40a8,8,0,0,1-8-8v-8H224ZM152,88a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h32A8,8,0,0,1,152,88Z"
				></path></svg
			>
		</button>
		<button
			type="button"
			class={getThemeButtonClasses("dark")}
			onclick={() => setTheme("dark")}
			aria-label="Use dark theme"
			aria-pressed={theme === "dark"}
			bind:this={darkButton}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				aria-hidden="true"
				width="14"
				height="14"
				fill="currentColor"
				viewBox="0 0 256 256"
				><path
					d="M233.54,142.23a8,8,0,0,0-8-2,88.08,88.08,0,0,1-109.8-109.8,8,8,0,0,0-10-10,104.84,104.84,0,0,0-52.91,37A104,104,0,0,0,136,224a103.09,103.09,0,0,0,62.52-20.88,104.84,104.84,0,0,0,37-52.91A8,8,0,0,0,233.54,142.23ZM188.9,190.34A88,88,0,0,1,65.66,67.11a89,89,0,0,1,31.4-26A106,106,0,0,0,96,56,104.11,104.11,0,0,0,200,160a106,106,0,0,0,14.92-1.06A89,89,0,0,1,188.9,190.34Z"
				></path></svg
			>
		</button>
	</div>
</div>

<style>
	:global(.theme-toggle)::after {
		content: "";
		position: absolute;
		left: var(--toggle-highlight-x);
		top: var(--toggle-highlight-y);
		width: var(--toggle-highlight-width);
		height: var(--toggle-highlight-height);
		z-index: 0;
		border-radius: 0.75rem;
		pointer-events: none;
		border: 1.5px solid transparent;
		background:
			radial-gradient(
					ellipse at -20px top,
					rgb(255 255 255 / 0.22),
					rgb(255 255 255 / 0)
				)
				padding-box,
			linear-gradient(
					180deg,
					var(--color-accent),
					var(--color-accent-secondary)
				)
				padding-box,
			linear-gradient(
					180deg,
					rgb(255 255 255 / 0.72),
					rgb(0 0 0 / 0.24) 41%,
					rgb(0 0 0 / 0.24) 75%,
					rgb(255 255 255 / 0.28)
				)
				border-box;
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.22),
			var(--shadow-sm),
			var(--shadow-lg);
		transition: opacity 0.2s ease;
	}

	:global(html.dark .theme-toggle)::after {
		background:
			radial-gradient(
					ellipse at -20px top,
					rgb(255 255 255 / 0.08),
					rgb(255 255 255 / 0)
				)
				padding-box,
			linear-gradient(
					180deg,
					color-mix(in oklab, var(--color-accent) 70%, var(--color-card) 30%),
					color-mix(
						in oklab,
						var(--color-accent-secondary) 60%,
						var(--color-card) 40%
					)
				)
				padding-box,
			linear-gradient(
					180deg,
					rgb(255 255 255 / 0.72),
					rgb(0 0 0 / 0.24) 41%,
					rgb(0 0 0 / 0.24) 75%,
					rgb(255 255 255 / 0.28)
				)
				border-box;
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.08),
			var(--shadow-sm),
			var(--shadow-xl);
	}
</style>
