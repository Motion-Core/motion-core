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

<li
	{...restProps}
	class={cn(
		"text-base leading-relaxed font-normal tracking-normal text-pretty text-foreground-muted",
		className,
	)}
>
	{@render children?.()}
</li>
