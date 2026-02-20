<script lang="ts">
	import ComponentCard from "./ComponentCard.svelte";
	import type { ComponentInfo } from "./types";
	import { gsap } from "gsap/dist/gsap";
	import { CustomEase } from "gsap/dist/CustomEase";
	import { onMount } from "svelte";

	const motionCoreEase = "motion-core-ease";
	const FEATURED_LAYOUT = [
		{ slug: "dithered-image", className: "md:col-span-1 md:row-span-1" },
		{ slug: "text-loop", className: "md:col-span-2 md:row-span-1" },
		{ slug: "water-ripple", className: "md:col-span-1 md:row-span-2" },
		{ slug: "image-trail", className: "md:col-span-2 md:row-span-1" },
		{ slug: "globe", className: "md:col-span-1 md:row-span-1" },
		{ slug: "magnetic", className: "md:col-span-2 md:row-span-1" },
		{ slug: "rubiks-cube", className: "md:col-span-2 md:row-span-1" },
	];

	const props = $props<{ components?: ComponentInfo[] }>();
	const components = $derived(props.components ?? []);

	const featuredComponents = $derived(
		FEATURED_LAYOUT.map((conf) => {
			const component = components.find(
				(entry: ComponentInfo) => entry.slug === conf.slug,
			);
			return component ? { component, className: conf.className } : null;
		}).filter((item) => item !== null),
	);

	let listRef: HTMLDivElement | null = null;

	onMount(() => {
		gsap.registerPlugin(CustomEase);
		CustomEase.create(motionCoreEase, "0.625, 0.05, 0, 1");
	});

	$effect(() => {
		void featuredComponents;

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
				},
			);
		}, listRef);

		return () => {
			ctx.revert();
		};
	});
</script>

<section class="input-highlight relative w-full rounded-2xl bg-background p-2">
	<div
		class="grid grid-cols-1 gap-2 md:auto-rows-[280px] md:grid-cols-4"
		bind:this={listRef}
	>
		{#each featuredComponents as item (item.component.slug)}
			<ComponentCard
				component={item.component}
				class={item.className}
				featured={true}
			/>
		{/each}
	</div>
</section>
