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

<th {...restProps} class={cn("h-12 px-6 text-left align-middle font-medium text-foreground tracking-tight", className)}>
	{@render children?.()}
</th>