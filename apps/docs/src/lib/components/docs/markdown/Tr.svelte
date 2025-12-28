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

<tr
	{...restProps}
	class={cn(
		"transition-[background-color] duration-150 ease-out hover:bg-card-muted/60 data-[state=selected]:bg-card-muted/60",
		className,
	)}
>
	{@render children?.()}
</tr>
