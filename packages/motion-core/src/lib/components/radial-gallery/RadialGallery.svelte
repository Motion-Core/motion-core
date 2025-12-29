<script lang="ts" generics="T">
	import { onMount, onDestroy } from "svelte";
	import type { Snippet } from "svelte";
	import gsap from "gsap";
	import { cn } from "../../utils/cn";

	interface Props<T> {
		/**
		 * Array of items to display in the gallery.
		 */
		items: T[];
		/**
		 * Snippet to render each item. Receives the item and its index.
		 */
		children: Snippet<[T, number]>;
		/**
		 * Radius of the circular gallery in pixels.
		 * @default 600
		 */
		radius?: number;
		/**
		 * Duration of one full rotation in seconds.
		 * @default 20
		 */
		duration?: number;
		/**
		 * Whether to rotate in the opposite direction.
		 * @default false
		 */
		reversed?: boolean;
		/**
		 * Vertical offset of the circle center from the bottom in pixels.
		 * @default 0
		 */
		offset?: number;
		/**
		 * Gap between items in pixels.
		 * @default 0
		 */
		gap?: number;
		/**
		 * Estimated size of each element (width) for calculation.
		 * @default 100
		 */
		elementSize?: number;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
	}

	let {
		items,
		children,
		radius = 600,
		duration = 20,
		reversed = false,
		offset = 0,
		gap = 0,
		elementSize = 100,
		class: className,
	}: Props<T> = $props();

	let container = $state<HTMLDivElement>();
	let tween: gsap.core.Tween;

	let displayItems = $derived.by(() => {
		const circumference = 2 * Math.PI * radius;
		const spacePerItem = elementSize + gap;
		const neededItems = Math.ceil(circumference / spacePerItem);

		const repeats = Math.ceil(neededItems / items.length);
		return Array.from({ length: repeats }, (_, r) =>
			items.map((item, i) => ({ item, key: `${r}-${i}` })),
		).flat();
	});

	let angleStep = $derived(360 / displayItems.length);

	onMount(() => {
		if (!container) return;

		tween = gsap.to(container, {
			rotation: reversed ? -360 : 360,
			duration,
			repeat: -1,
			ease: "none",
		});

		return () => tween?.kill();
	});

	onDestroy(() => tween?.kill());

	$effect(() => {
		tween?.duration(duration);
	});
</script>

<div
	class={cn(
		"relative flex h-full w-full items-end justify-center overflow-hidden",
		className,
	)}
>
	<div
		bind:this={container}
		class="absolute flex items-center justify-center"
		style:width="{radius * 2}px"
		style:height="{radius * 2}px"
		style:bottom="{offset - radius}px"
	>
		{#each displayItems as { item, key }, i (key)}
			<div
				class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
				style:transform="rotate({i * angleStep}deg) translate(0, -{radius}px)
				rotate(90deg)"
			>
				<div style:transform="rotate(-90deg)">
					{@render children(item, i)}
				</div>
			</div>
		{/each}
	</div>
</div>
