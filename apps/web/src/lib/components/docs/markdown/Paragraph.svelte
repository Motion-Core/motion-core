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

<p
	{...restProps}
	class={cn(
		"mt-4 text-base leading-relaxed font-normal tracking-normal text-pretty text-foreground-muted first:mt-0",
		className,
	)}
>
	{@render children?.()}
</p>
