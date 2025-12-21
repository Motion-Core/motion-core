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

<blockquote
	{...restProps}
	class={cn(
		"mt-6 rounded-lg border border-border bg-card px-5 py-3 text-sm italic text-foreground/70 shadow-sm",
		className,
	)}
>
	{@render children?.()}
</blockquote>
