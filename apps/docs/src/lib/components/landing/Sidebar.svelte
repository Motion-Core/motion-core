<script lang="ts">
	import ThemeToggle from "./ThemeToggle.svelte";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";
	import { SplitHover } from "motion-core";
	import { motionCoreEase } from "motion-core";
	import type { SocialLink } from "./types";
	import gsap from "gsap";
	import SplitText from "gsap/SplitText";

	const props = $props<{
		title?: string;
		description?: string;
		socialLinks?: SocialLink[];
	}>();

	const title = $derived(props.title ?? "Motion Core");
	const description = $derived(props.description ?? "");
	const socialLinks = $derived(
		(props.socialLinks ?? []).map((link: SocialLink) => ({
			...link,
			ref: null as HTMLAnchorElement | null,
		})),
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

		const ctx = gsap.context(() => {
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

		return () => {
			ctx.revert();
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
			class="inline-flex shrink-0 items-center text-foreground [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
			aria-hidden="true"
		>
			{@html motionCoreLogo}
		</span>
		<span class="text-xl text-foreground">{title}</span>
	</header>
	<div class="space-y-4">
		<p
			class="text-xs leading-relaxed text-foreground/70 text-balance overflow-hidden"
			bind:this={descriptionRef}
		>
			{description}
		</p>
		<nav
			class="flex items-center gap-2 text-xs uppercase tracking-wide font-mono"
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
