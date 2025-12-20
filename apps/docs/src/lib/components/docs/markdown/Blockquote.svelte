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

<blockquote
	{...restProps}
	class={cn(
		"mt-6 rounded-r-xl border-l-4 border-border bg-card px-5 py-3 text-base italic text-foreground/70",
		className,
	)}
>
	{@render children?.()}
</blockquote>
