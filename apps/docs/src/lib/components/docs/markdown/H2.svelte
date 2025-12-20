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

<h2
	{...restProps}
	class={cn(
		"mt-12 scroll-m-24 text-xl font-medium tracking-tight text-foreground",
		className,
	)}
>
	{@render children?.()}
</h2>
