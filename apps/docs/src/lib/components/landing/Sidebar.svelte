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

	$effect(() => {
		if (typeof window === "undefined" || !containerRef) return;

		const targets =
			containerRef.querySelectorAll<HTMLElement>("[data-sidebar-anim]");
		if (!targets.length) return;

		const ctx = gsap.context(() => {
			gsap.fromTo(
				targets,
				{ autoAlpha: 0, filter: "blur(14px)" },
				{
					autoAlpha: 1,
					filter: "blur(0px)",
					duration: 1,
					stagger: 0.05,
					ease: "power3.out",
					clearProps: "all",
				},
			);
		}, containerRef);

		return () => ctx.revert();
	});

	const descriptionKey = $derived(description);

	$effect(() => {
		descriptionKey;
		if (typeof window === "undefined" || !descriptionRef) return;

		let splitParent: SplitText | null = null;
		let splitText: SplitText | null = null;

		const ctx = gsap.context(() => {
			splitParent = new SplitText(descriptionRef, {
				type: "lines",
				linesClass: "sidebar-desc-parent",
			});
			splitText = new SplitText(descriptionRef, {
				type: "lines",
				linesClass: "sidebar-desc-line",
			});

			gsap.fromTo(
				splitText.lines,
				{ autoAlpha: 0, filter: "blur(14px)" },
				{
					autoAlpha: 1,
					filter: "blur(0px)",
					duration: 1,
					stagger: 0.05,
					delay: 0.15,
					ease: "power3.out",
					clearProps: "all",
				},
			);
		}, descriptionRef);

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
	<header class="flex items-center gap-2" data-sidebar-anim="true">
		<span class="text-xl font-medium text-foreground">{title}</span>
	</header>
	<div class="space-y-4" data-sidebar-anim="true">
		<p
			class="text-xs leading-relaxed text-foreground/70 text-balance overflow-hidden"
			bind:this={descriptionRef}
		>
			{description}
		</p>
		<nav class="flex items-center gap-2 text-xs font-medium uppercase tracking-wide font-mono">
			{#each socialLinks as link}
				<a
					class="text-foreground underline-offset-4 hover:underline"
					href={link.href}
					data-sidebar-anim
				>
					{link.label}
				</a>
			{/each}
			<div data-sidebar-anim class="ml-auto">
				<ThemeToggle />
			</div>
		</nav>
	</div>
</aside>
