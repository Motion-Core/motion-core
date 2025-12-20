<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "$lib/utils/cn";

type ComponentProps = {
	class?: string;
	children?: Snippet;
	[prop: string]: unknown;
};

let { children, class: className = "", ...restProps }: ComponentProps = $props();
</script>

<h1
	{...restProps}
	class={cn(
		"scroll-m-24 text-2xl font-medium tracking-tight text-foreground",
		className,
	)}
>
	{@render children?.()}
</h1>
