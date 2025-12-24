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

<h3
	{...restProps}
	class={cn("mt-10 scroll-m-24 text-lg text-foreground font-medium", className)}
>
	{@render children?.()}
</h3>
