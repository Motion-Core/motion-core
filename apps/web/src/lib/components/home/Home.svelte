<script lang="ts">
	import { brandLogoRaw } from "$lib";
	import { resolve } from "$app/paths";
	import Book from "carbon-icons-svelte/lib/Book.svelte";
	import LogoGithub from "carbon-icons-svelte/lib/LogoGithub.svelte";
	import Button from "../ui/Button.svelte";
	import Section from "./Section.svelte";
	import { siteConfig } from "$lib/config/site";

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
</script>

<Section
	variant="muted"
	id="home"
	class="flex min-h-[50vh] w-full flex-col items-center justify-center gap-4"
>
	<div
		class="flex w-full flex-col items-center justify-center gap-4 pt-16 sm:pt-10"
	>
		<span
			class="inline-flex shrink-0 items-center text-accent [&>svg]:size-16 [&>svg]:fill-current"
			aria-hidden="true"
		>
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html brandLogoRaw}
		</span>
		<h1
			class="max-w-3xl text-center text-3xl font-medium tracking-tight text-balance text-foreground sm:text-5xl"
		>
			A High-quality <span class="text-accent">motion components</span>
			for Svelte.
		</h1>
		<p
			class="max-w-xl text-center text-base font-normal text-pretty text-foreground-muted sm:text-lg"
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
</Section>
