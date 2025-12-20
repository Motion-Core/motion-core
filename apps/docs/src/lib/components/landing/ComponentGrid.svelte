<script lang="ts">
	import ComponentCard from "./ComponentCard.svelte";
	import type { ComponentInfo } from "./types";
	import gsap from "gsap";

	const props = $props<{ components?: ComponentInfo[] }>();
	const components = $derived(props.components ?? []);

	let listRef: HTMLDivElement | null = null;
	const animationKey = $derived(
		components.map((component: ComponentInfo) => component.slug).join("|"),
	);

	$effect(() => {
		animationKey;

		if (typeof window === "undefined" || !listRef) {
			return;
		}

		const cards = listRef.querySelectorAll<HTMLElement>("[data-component-card]");
		if (!cards.length) {
			return;
		}

		const ctx = gsap.context(() => {
			gsap.fromTo(
				cards,
				{
					autoAlpha: 0,
					filter: "blur(18px)",
					y: 20
				},
				{
					autoAlpha: 1,
					scale: 1,
					y: 0,
					delay: 0.1,
					duration: 1,
					stagger: 0.05,
					ease: "power3.out",
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
	class="relative lg:col-span-3 lg:max-h-[calc(100svh-2rem)] rounded-xl bg-background border border-border card-highlight"
>
	<div
		class="w-full h-full p-4 lg:overflow-y-auto"
		style="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
	>
		<div
			class="columns-1 gap-4 space-y-4 sm:columns-2 lg:columns-2 [column-fill:balance]"
			bind:this={listRef}
		>
			{#each components as component}
				<ComponentCard {component} />
			{/each}
		</div>
	</div>
</section>
