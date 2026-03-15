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

<th
	{...restProps}
	class={cn(
		"px-4 py-2 text-left align-middle text-base font-medium text-foreground",
		className,
	)}
>
	{@render children?.()}
</th>
