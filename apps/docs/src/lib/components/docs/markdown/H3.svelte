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

<h3
	{...restProps}
	class={cn(
		"mt-10 scroll-m-24 text-lg font-medium tracking-tight text-foreground",
		className,
	)}
>
	{@render children?.()}
</h3>
