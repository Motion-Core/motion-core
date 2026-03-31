<script lang="ts">
	import { brandLogoRaw } from "$lib";
	import { resolve } from "$app/paths";
	import Book from "carbon-icons-svelte/lib/Book.svelte";
	import LogoGithub from "carbon-icons-svelte/lib/LogoGithub.svelte";
	import Button from "../ui/Button.svelte";
	import { siteConfig } from "$lib/config/site";
	import SpecularBand from "motion-core/components/specular-band/SpecularBand.svelte";
	import { onMount } from "svelte";

	type Props = {
		githubStars?: number | null;
	};

	let { githubStars = null }: Props = $props();
	const docsRoute = "/docs" as const;

	const formattedGithubStars = $derived(
		typeof githubStars === "number"
			? new Intl.NumberFormat("en-US", {
					notation: "compact",
					maximumFractionDigits: 1,
				}).format(githubStars)
			: "--",
	);

	const COLOR_PRESETS = {
		dark: {
			backgroundColor: "#17181A",
		},
		light: {
			backgroundColor: "#f0f0f2",
		},
	};

	let isDark = $state(false);

	onMount(() => {
		isDark = document.documentElement.classList.contains("dark");

		const observer = new MutationObserver(() => {
			isDark = document.documentElement.classList.contains("dark");
		});

		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class"],
		});

		return () => observer.disconnect();
	});
</script>

<div
	id="home"
	class="relative flex h-dvh w-full flex-col items-center justify-center gap-4 p-4"
>
	<SpecularBand
		class="absolute inset-0"
		intensity={isDark ? 1 : 5}
		backgroundColor={isDark
			? COLOR_PRESETS.dark.backgroundColor
			: COLOR_PRESETS.light.backgroundColor}
	/>
	<div
		class="absolute inset-x-0 bottom-0 z-10 h-200 bg-linear-to-t from-background-inset to-transparent"
	></div>
	<div
		class="relative z-10 flex h-full w-full flex-col items-center justify-center gap-4"
	>
		<span
			class="mb-4 inline-flex shrink-0 items-center text-accent [&>svg]:h-auto [&>svg]:w-32 [&>svg]:fill-current"
			aria-hidden="true"
		>
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html brandLogoRaw}
		</span>
		<h1
			class="max-w-3xl text-center text-4xl font-medium tracking-tight text-balance text-foreground sm:text-6xl"
		>
			High-quality <span class="text-accent">motion components</span>
			for Svelte.
		</h1>
		<p
			class="max-w-xl text-center text-base font-normal tracking-normal text-pretty text-foreground-muted sm:text-lg"
		>
			Animated Svelte component library powered by GSAP and Three.js. Drop-in
			solutions for motion design, 3D canvases, and interactive animations.
		</p>
		<div data-reveal="actions" class="flex items-center gap-2">
			<Button
				variant="default"
				href={resolve(docsRoute as "/")}
				size="lg"
				data-sveltekit-reload
				data-sveltekit-preload-data="off"
				data-sveltekit-preload-code="off"
			>
				<Book size={16} />
				<span>Documentation</span>
			</Button>
			<Button
				variant="secondary"
				href={siteConfig.links.github}
				target="_blank"
				rel="noreferrer"
				size="lg"
			>
				<LogoGithub size={16} />
				<span>GitHub</span>
				<span aria-hidden="true" class="text-background/40">|</span>
				<span>{formattedGithubStars}</span>
			</Button>
		</div>
	</div>
</div>
