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

<div class="relative my-8 w-full rounded-lg border border-border bg-card card-highlight shadow-sm">
	<div class="overflow-x-auto rounded-lg">
		<table {...restProps} class={cn("w-full text-sm [&_code]:text-sm", className)}>
			{@render children?.()}
		</table>
	</div>
</div>
