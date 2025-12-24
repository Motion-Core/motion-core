<script lang="ts">
	import ThemeToggle from "./ThemeToggle.svelte";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";
	import { SplitHover } from "motion-core";
	import { motionCoreEase } from "motion-core";
	import type { SocialLink } from "./types";
	import gsap from "gsap";
	import SplitText from "gsap/SplitText";

	class LinkState {
		ref = $state<HTMLAnchorElement | null>(null);
		label: string;
		href: string;

		constructor(link: SocialLink) {
			this.label = link.label;
			this.href = link.href;
		}
	}

	const props = $props<{
		title?: string;
		description?: string;
		socialLinks?: SocialLink[];
	}>();

	const title = $derived(props.title ?? "Motion Core");
	const description = $derived(props.description ?? "");
	const socialLinks = $derived(
		(props.socialLinks ?? []).map((link: SocialLink) => new LinkState(link)),
	);

	let containerRef: HTMLElement | null = null;
	let descriptionRef: HTMLParagraphElement | null = null;
	let headerRef: HTMLElement | null = null;
	let linksRef: HTMLElement | null = null;
	let toggleRef: HTMLElement | null = null;

	const timelineKey = $derived(
		`${title}-${description}-${socialLinks
			.map((link: SocialLink) => link.href)
			.join("|")}`,
	);

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
						ease: motionCoreEase,
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
		<span class="text-xl text-foreground font-display font-medium">{title}</span
		>
	</header>
	<div class="space-y-4">
		<p
			class="text-sm leading-relaxed text-foreground/70 text-balance overflow-hidden"
			bind:this={descriptionRef}
		>
			{description}
		</p>
		<nav
			class="flex items-center gap-2 text-sm uppercase tracking-wide mono font-medium"
			bind:this={linksRef}
		>
			{#each socialLinks as link (link.href)}
				<a
					class="text-foreground underline-offset-4 hover:underline"
					href={link.href}
					bind:this={link.ref}
				>
					<SplitHover hoverTarget={link.ref}>{link.label}</SplitHover>
				</a>
			{/each}
			<div class="ml-auto" bind:this={toggleRef}>
				<ThemeToggle />
			</div>
		</nav>
	</div>
</aside>
