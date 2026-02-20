<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";

	type ComponentProps = {
		class?: string;
		children?: Snippet;
		[prop: string]: unknown;
	};

	const {
		children,
		class: className = "",
		...restProps
	}: ComponentProps = $props();
</script>

<h4
	{...restProps}
	class={cn(
		"mt-8 scroll-m-24 text-lg font-medium text-foreground font-display",
		className,
	)}
>
	{@render children?.()}
</h4>
