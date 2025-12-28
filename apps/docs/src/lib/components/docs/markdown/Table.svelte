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

<div
	class="relative my-8 w-full rounded-xl border border-border bg-card shadow-sm"
>
	<div class="overflow-x-auto rounded-xl">
		<table
			{...restProps}
			class={cn("w-full text-base [&_code]:text-sm", className)}
		>
			{@render children?.()}
		</table>
	</div>
</div>
