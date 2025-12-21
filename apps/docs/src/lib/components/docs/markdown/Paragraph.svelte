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

<p
	{...restProps}
	class={cn(
		"mt-4 text-sm leading-relaxed text-foreground/70 first:mt-0",
		className,
	)}
>
	{@render children?.()}
</p>
