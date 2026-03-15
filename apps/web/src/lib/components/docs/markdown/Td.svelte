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

<td
	{...restProps}
	class={cn(
		"px-4 py-2 align-middle font-normal tracking-normal text-foreground-muted",
		className,
	)}
>
	{@render children?.()}
</td>
