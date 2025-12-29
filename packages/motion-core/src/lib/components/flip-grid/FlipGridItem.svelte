<script lang="ts">
	import { cn } from "../../utils/cn";

	import type { Snippet } from "svelte";

	interface Props {
		children?: Snippet;
		class?: string;
		id: string;
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
