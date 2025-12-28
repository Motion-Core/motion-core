<script lang="ts">
	import { onMount } from "svelte";
	import gsap from "gsap";
	import { Flip } from "gsap/dist/Flip";
	import { cn } from "../../utils/cn";

	let {
		children,
		class: className = undefined,
		duration = 0.5,
		ease = "power2.inOut",
		stagger = 0,
		columns = undefined,
		style = undefined,
		...props
	} = $props();

	let container: HTMLElement;
	let state: Flip.FlipState | null = null;

	let computedStyle = $derived.by(() => {
		const baseStyle = style || "";
		if (columns) {
			const colStyle = `grid-template-columns: repeat(${columns}, minmax(0, 1fr))`;
			return baseStyle ? `${baseStyle}; ${colStyle}` : colStyle;
		}
		return baseStyle;
	});

	onMount(() => {
		gsap.registerPlugin(Flip);
	});

	$effect.pre(() => {
		void className;
		void computedStyle;

		if (container) {
			const items = container.querySelectorAll(".flip-grid-item");
			if (items.length > 0) {
				state = Flip.getState([...items, container]);
			}
		}
	});

	$effect(() => {
		void className;
		void computedStyle;

		if (state && container) {
			const items = container.querySelectorAll(".flip-grid-item");

			Flip.from(state, {
				targets: [...items, container],
				duration,
				ease,
				stagger,
				absolute: ".flip-grid-item",
				onEnter: (elements) => {
					gsap.fromTo(
						elements,
						{ opacity: 0, scale: 0 },
						{ opacity: 1, scale: 1, duration, ease },
					);
				},
				onLeave: (elements) => {
					gsap.to(elements, { opacity: 0, scale: 0, duration, ease });
				},
			});

			state = null;
		}
	});
</script>

<div
	bind:this={container}
	class={cn("relative grid", className)}
	style={computedStyle}
	{...props}
>
	{@render children?.()}
</div>
