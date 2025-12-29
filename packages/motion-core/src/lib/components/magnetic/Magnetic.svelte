<script lang="ts">
	import { onMount } from "svelte";
	import { gsap } from "gsap/dist/gsap";
	import type { Snippet } from "svelte";

	interface Props {
		/**
		 * Snippet to render content.
		 */
		children?: Snippet;
		/**
		 * Animation duration in seconds.
		 * @default 1
		 */
		duration?: number;
		/**
		 * Animation easing function.
		 * @default "elastic.out(1, 0.3)"
		 */
		ease?: string;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
	}

	let {
		children,
		duration = 1,
		ease = "elastic.out(1, 0.3)",
		class: className = "",
	}: Props = $props();

	let element: HTMLElement;
	let xTo: gsap.QuickToFunc;
	let yTo: gsap.QuickToFunc;

	onMount(() => {
		if (!element) return;

		xTo = gsap.quickTo(element, "x", { duration, ease });
		yTo = gsap.quickTo(element, "y", { duration, ease });

		const mouseMove = (e: MouseEvent) => {
			const { clientX, clientY } = e;
			const { height, width, left, top } = element.getBoundingClientRect();
			const x = clientX - (left + width / 2);
			const y = clientY - (top + height / 2);
			xTo(x);
			yTo(y);
		};

		const mouseLeave = () => {
			xTo(0);
			yTo(0);
		};

		element.addEventListener("mousemove", mouseMove);
		element.addEventListener("mouseleave", mouseLeave);

		return () => {
			element.removeEventListener("mousemove", mouseMove);
			element.removeEventListener("mouseleave", mouseLeave);
		};
	});
</script>

<div bind:this={element} class={className} role="presentation">
	{@render children?.()}
</div>
