<script lang="ts">
	import { cn } from "../../utils/cn";

	import type { Snippet } from "svelte";

	interface Props {
		/**
		 * Snippet to render the item content.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Unique identifier for FLIP animation.
		 */
		id: string;
		/**
		 * Additional inline styles.
		 */
		style?: string;
		[key: string]: unknown;
	}

	let {
		children,
		class: className = undefined,
		id,
		style = undefined,
		...props
	}: Props = $props();
</script>

<div
	class={cn("flip-grid-item", className)}
	data-flip-id={id}
	{style}
	{...props}
>
	{@render children?.()}
</div>
