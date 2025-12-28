<script lang="ts">
	import ComponentCard from "./ComponentCard.svelte";
	import type { ComponentInfo } from "./types";
	import { gsap } from "gsap/dist/gsap";
	import { CustomEase } from "gsap/dist/CustomEase";
	import ScrollArea from "$lib/components/ui/ScrollArea.svelte";
	import { onMount } from "svelte";

	const motionCoreEase = "motion-core-ease";

	const props = $props<{ components?: ComponentInfo[] }>();
	const components = $derived(props.components ?? []);

	let listRef: HTMLDivElement | null = null;

	onMount(() => {
		gsap.registerPlugin(CustomEase);
		CustomEase.create(motionCoreEase, "0.625, 0.05, 0, 1");
	});
	const animationKey = $derived(
		components.map((component: ComponentInfo) => component.slug).join("|"),
	);

	$effect(() => {
		void animationKey;

		if (typeof window === "undefined" || !listRef) {
			return;
		}

		const cards = listRef.querySelectorAll<HTMLElement>(
			"[data-component-card]",
		);
		if (!cards.length) {
			return;
		}

		const ctx = gsap.context(() => {
			gsap.fromTo(
				cards,
				{
					autoAlpha: 0,
					filter: "blur(16px)",
					y: 20,
				},
				{
					autoAlpha: 1,
					scale: 1,
					y: 0,
					delay: 0.1,
					duration: 1,
					stagger: 0.1,
					ease: motionCoreEase,
					filter: "blur(0px)",
					clearProps: "all",
				},
			);
		}, listRef);

		return () => {
			ctx.revert();
		};
	});
</script>

<section
	class="relative rounded-2xl border border-border bg-background lg:col-span-3 lg:max-h-[calc(100svh-2rem)]"
>
	<ScrollArea
		class="h-full w-full"
		viewportClass="h-full w-full p-2 md:p-4"
		viewportStyle="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
	>
		<div
			class="columns-1 gap-4 space-y-4 [column-fill:balance] sm:columns-2 lg:columns-3"
			bind:this={listRef}
		>
			{#each components as component (component.slug)}
				<ComponentCard {component} />
			{/each}
		</div>
	</ScrollArea>
</section>
