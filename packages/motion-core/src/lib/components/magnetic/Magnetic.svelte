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

	let element: HTMLElement | undefined;
	let xTo: gsap.QuickToFunc;
	let yTo: gsap.QuickToFunc;

	const attachElement = (node: HTMLElement) => {
		element = node;
		return () => {
			if (element === node) {
				element = undefined;
			}
		};
	};

	onMount(() => {
		if (!element) return;
		const currentElement = element;

		xTo = gsap.quickTo(currentElement, "x", { duration, ease });
		yTo = gsap.quickTo(currentElement, "y", { duration, ease });

		const mouseMove = (e: MouseEvent) => {
			const { clientX, clientY } = e;
			const { height, width, left, top } =
				currentElement.getBoundingClientRect();
			const x = clientX - (left + width / 2);
			const y = clientY - (top + height / 2);
			xTo(x);
			yTo(y);
		};

		const mouseLeave = () => {
			xTo(0);
			yTo(0);
		};

		currentElement.addEventListener("mousemove", mouseMove);
		currentElement.addEventListener("mouseleave", mouseLeave);

		return () => {
			currentElement.removeEventListener("mousemove", mouseMove);
			currentElement.removeEventListener("mouseleave", mouseLeave);
		};
	});
</script>

<div {@attach attachElement} class={className} role="presentation">
	{@render children?.()}
</div>
