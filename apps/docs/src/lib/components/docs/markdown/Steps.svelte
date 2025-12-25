<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import type { Snippet } from "svelte";

	let { class: className, children } = $props<{
		class?: string;
		children?: Snippet;
	}>();
</script>

<div
	class={cn(
		"relative mb-12 ml-4 border-l border-border [counter-reset:step]",
		className,
	)}
>
	{@render children?.()}
</div>
