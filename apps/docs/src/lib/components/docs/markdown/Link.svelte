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

<a
	{...restProps}
	class={cn(
		"text-foreground underline decoration-dotted underline-offset-4 transition-[color] duration-150 ease-out hover:text-foreground/70",
		className,
	)}
>
	{@render children?.()}
</a>
