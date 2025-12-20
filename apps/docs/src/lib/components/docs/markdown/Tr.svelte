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

<tr {...restProps} class={cn("transition-colors hover:bg-card-muted/30 data-[state=selected]:bg-card-muted", className)}>
	{@render children?.()}
</tr>