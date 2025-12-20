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

<a
	{...restProps}
	class={cn(
		"font-medium text-foreground underline decoration-dotted underline-offset-4 hover:text-foreground/70 transition-[color] duration-150 ease-out",
		className,
	)}
>
	{@render children?.()}
</a>
