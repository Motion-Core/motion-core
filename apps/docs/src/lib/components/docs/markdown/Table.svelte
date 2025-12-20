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

<div class="my-8 w-full rounded-lg border border-border bg-card">
	<div class="overflow-x-auto">
		<table {...restProps} class={cn("w-full text-xs [&_code]:text-xs", className)}>
			{@render children?.()}
		</table>
	</div>
</div>
