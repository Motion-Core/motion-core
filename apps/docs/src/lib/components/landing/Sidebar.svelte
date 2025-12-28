<script lang="ts">
	import ThemeToggle from "./ThemeToggle.svelte";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";
	import type { SocialLink } from "./types";
	import { gsap } from "gsap/dist/gsap";
	import { SplitText } from "gsap/dist/SplitText";
	import { CustomEase } from "gsap/dist/CustomEase";
	import { onMount } from "svelte";

	const props = $props<{
		title?: string;
		description?: string;
		socialLinks?: SocialLink[];
	}>();
	const DEFAULT_GITHUB_URL = "https://github.com/motion-core/motion-core";

	const title = $derived(props.title ?? "Motion Core");
	const description = $derived(props.description ?? "");
	const githubHref = $derived(
		(props.socialLinks ?? []).find(
			(link: SocialLink) => link.label.toLowerCase() === "github",
		)?.href ?? DEFAULT_GITHUB_URL,
	);

	let containerRef: HTMLElement | null = null;
	let descriptionRef: HTMLParagraphElement | null = null;
	let headerRef: HTMLElement | null = null;
	let linksRef: HTMLElement | null = null;
	let toggleRef: HTMLElement | null = null;

	const timelineKey = $derived(`${title}-${description}-${githubHref}`);

	onMount(() => {
		gsap.registerPlugin(CustomEase);
		CustomEase.create("motion-core-ease", "0.625, 0.05, 0, 1");
	});

	$effect(() => {
		void timelineKey;
		if (typeof window === "undefined" || !containerRef) return;

		let splitText: SplitText | null = null;
		let ctx: gsap.Context | null = null;

		const init = async () => {
			await document.fonts.ready;
			if (!containerRef) return;

			ctx = gsap.context(() => {
				if (descriptionRef) {
					splitText = SplitText.create(descriptionRef, {
						type: "lines",
						linesClass: "inline-block",
					});
				}

				const timeline = gsap.timeline({
					defaults: {
						ease: "motion-core-ease",
						duration: 0.5,
					},
				});

				if (headerRef) {
					timeline.fromTo(
						headerRef,
						{ autoAlpha: 0, filter: "blur(16px)" },
						{ autoAlpha: 1, filter: "blur(0px)", clearProps: "filter" },
					);
				}

				if (splitText?.lines?.length) {
					timeline.fromTo(
						splitText.lines,
						{ autoAlpha: 0, filter: "blur(16px)" },
						{
							autoAlpha: 1,
							filter: "blur(0px)",
							stagger: 0.08,
							clearProps: "all",
						},
						"-=0.25",
					);
				}

				if (linksRef) {
					timeline.fromTo(
						linksRef,
						{ autoAlpha: 0, filter: "blur(16px)" },
						{ autoAlpha: 1, filter: "blur(0px)", clearProps: "filter" },
						"-=0.25",
					);
				}

				if (toggleRef) {
					timeline.fromTo(
						toggleRef,
						{ autoAlpha: 0, filter: "blur(16px)" },
						{ autoAlpha: 1, filter: "blur(0px)", clearProps: "filter" },
						"-=0.5",
					);
				}
			}, containerRef);
		};

		void init();

		return () => {
			ctx?.revert();
			splitText?.revert();
		};
	});
</script>

<aside
	class="flex flex-col justify-between gap-8 lg:sticky lg:top-4 lg:col-span-1 lg:h-[calc(100svh-2rem)] lg:self-start"
	bind:this={containerRef}
>
	<header class="flex items-center gap-2" bind:this={headerRef}>
		<span
			class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
			aria-hidden="true"
		>
			{@html motionCoreLogo}
		</span>
		<span class="text-xl font-medium text-foreground font-display">{title}</span
		>
	</header>
	<div class="space-y-4">
		<p
			class="overflow-hidden text-sm leading-relaxed text-balance text-foreground/70"
			bind:this={descriptionRef}
		>
			{description}
		</p>
		<div class="flex items-center gap-2" bind:this={linksRef}>
			<a
				class="inline-flex size-7 items-center justify-center gap-2 rounded-lg text-sm font-medium text-foreground transition-colors duration-150 ease-out hover:bg-card-muted hover:text-foreground"
				href={DEFAULT_GITHUB_URL}
				target="_blank"
				rel="noreferrer"
				aria-label="Open Motion Core on GitHub"
			>
				<svg
					role="img"
					viewBox="0 0 24 24"
					fill="currentColor"
					aria-hidden="true"
					class="size-4 flex-none"
				>
					<title>GitHub</title>
					<path
						d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
					/>
				</svg>
			</a>
			<div class="ml-auto" bind:this={toggleRef}>
				<ThemeToggle />
			</div>
		</div>
	</div>
</aside>
