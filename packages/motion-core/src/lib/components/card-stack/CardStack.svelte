<script lang="ts">
	import { onMount } from "svelte";
	import { gsap } from "gsap/dist/gsap";
	import { ScrollTrigger } from "gsap/dist/ScrollTrigger";
	import { cn } from "../../utils/cn";
	import type { Snippet } from "svelte";

	interface Props {
		/**
		 * The cards to stack. Use the `Card` component for best results.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * The scale difference between stacked cards.
		 * @default 0.05
		 */
		scaleFactor?: number;
		/**
		 * The vertical offset (in pixels) between stacked cards.
		 * @default 10
		 */
		offset?: number;
		/**
		 * The vertical distance from the top of the screen where the first card stops.
		 * @default 0
		 */
		topOffset?: number;
		/**
		 * The element to use as the scroller. Defaults to window.
		 */
		scrollElement?: string | HTMLElement | null;
	}

	let {
		children,
		class: className,
		scaleFactor = 0.05,
		offset = 10,
		topOffset = 0,
		scrollElement,
	}: Props = $props();

	let container: HTMLElement;

	onMount(() => {
		gsap.registerPlugin(ScrollTrigger);
	});

	$effect(() => {
		if (!container) return;

		const cards = Array.from(
			container.querySelectorAll(".card-stack-item"),
		) as HTMLElement[];

		if (cards.length === 0) return;

		const ctx = gsap.context(() => {
			const lastCard = cards[cards.length - 1];
			const scroller =
				typeof scrollElement === "string"
					? document.querySelector(scrollElement)
					: scrollElement instanceof HTMLElement
						? scrollElement
						: window;

			const scrollerHeight =
				scroller === window
					? window.innerHeight
					: (scroller as HTMLElement).clientHeight;

			const lastCardHeight = lastCard.offsetHeight;
			const targetPos = topOffset + (cards.length - 1) * offset;
			const extraPadding = Math.max(
				0,
				scrollerHeight - lastCardHeight - targetPos,
			);

			if (extraPadding > 0) {
				gsap.set(container, { paddingBottom: extraPadding });
			}

			cards.forEach((card, index) => {
				gsap.set(card, {
					transformOrigin: "top center",
					zIndex: index,
				});

				const tl = gsap.timeline({
					scrollTrigger: {
						trigger: card,
						start: `top top+=${topOffset + index * offset}`,
						endTrigger: container,
						end: "bottom bottom",
						pin: true,
						pinSpacing: false,
						scrub: true,
						scroller: scrollElement || window,
						invalidateOnRefresh: true,
					},
				});

				const targetScale = 1 - (cards.length - 1 - index) * scaleFactor;

				if (index < cards.length - 1) {
					tl.to(card, {
						scale: targetScale,
						ease: "none",
					});
				}
			});
		}, container);

		return () => {
			ctx.revert();
		};
	});
</script>

<div bind:this={container} class={cn("relative w-full", className)}>
	{@render children?.()}
</div>
