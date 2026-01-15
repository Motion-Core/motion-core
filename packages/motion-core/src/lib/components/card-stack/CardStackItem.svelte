<script lang="ts">
	import { cn } from "../../utils/cn";
	import type { Snippet } from "svelte";

	interface Props {
		/**
		 * The content of the card.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes.
		 */
		class?: string;
		/**
		 * Additional inline styles.
		 */
		style?: string;
		[key: string]: unknown;
	}

	let { children, class: className, style, ...props }: Props = $props();
</script>

<div
	class={cn(
		"card-stack-item relative flex flex-col will-change-transform",
		className,
	)}
	{style}
	{...props}
>
	{@render children?.()}
</div>
