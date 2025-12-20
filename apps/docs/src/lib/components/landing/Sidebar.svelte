<script lang="ts">
	import ThemeToggle from "./ThemeToggle.svelte";
	import type { SocialLink } from "./types";
	import gsap from "gsap";
	import SplitText from "gsap/SplitText";

	gsap.registerPlugin(SplitText);

	const props = $props<{
		title?: string;
		description?: string;
		socialLinks?: SocialLink[];
	}>();

	const title = $derived(props.title ?? "Motion Core");
	const description = $derived(props.description ?? "");
	const socialLinks = $derived(props.socialLinks ?? []);

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
		timelineKey;
		if (typeof window === "undefined" || !containerRef) return;

		let splitParent: SplitText | null = null;
		let splitText: SplitText | null = null;

		const ctx = gsap.context(() => {
			if (descriptionRef) {
				splitParent = new SplitText(descriptionRef, {
					type: "lines",
					linesClass: "sidebar-desc-parent",
				});
				splitText = new SplitText(descriptionRef, {
					type: "lines",
					linesClass: "sidebar-desc-line",
				});
			}

			const timeline = gsap.timeline({
				defaults: {
					ease: "power3.out",
					duration: 0.75
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
					"-=0.35",
				);
			}

			if (linksRef) {
				timeline.fromTo(
					linksRef,
					{ autoAlpha: 0, filter: "blur(16px)" },
					{ autoAlpha: 1, filter: "blur(0px)", clearProps: "filter" },
					"-=0.35",
				);
			}

			if (toggleRef) {
				timeline.fromTo(
					toggleRef,
					{ autoAlpha: 0, filter: "blur(16px)" },
					{ autoAlpha: 1, filter: "blur(0px)", clearProps: "filter" },
					"-=0.75",
				);
			}
		}, containerRef);

		return () => {
			ctx.revert();
			splitText?.revert();
			splitParent?.revert();
		};
	});
</script>

<aside
	class="flex flex-col justify-between gap-8 lg:sticky lg:top-4 lg:col-span-1 lg:h-[calc(100svh-2rem)] lg:self-start"
	bind:this={containerRef}
>
	<header class="flex items-center gap-2" bind:this={headerRef}>
		<span class="text-xl font-medium text-foreground">{title}</span>
	</header>
	<div class="space-y-4">
		<p
			class="text-xs leading-relaxed text-foreground/70 text-balance overflow-hidden"
			bind:this={descriptionRef}
		>
			{description}
		</p>
		<nav
			class="flex items-center gap-2 text-xs font-medium uppercase tracking-wide font-mono"
			bind:this={linksRef}
		>
			{#each socialLinks as link}
				<a class="text-foreground underline-offset-4 hover:underline" href={link.href}>
					{link.label}
				</a>
			{/each}
			<div class="ml-auto" bind:this={toggleRef}>
				<ThemeToggle />
			</div>
		</nav>
	</div>
</aside>
