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
	class="inset-shadow my-8 rounded-lg border border-border bg-background-inset p-1"
>
	<div class="relative rounded-md border border-border bg-background shadow-md">
		<div class="w-full overflow-x-auto rounded-md">
			<table
				{...restProps}
				class={cn("w-full text-lg [&_code]:text-base", className)}
			>
				{@render children?.()}
			</table>
		</div>
	</div>
</div>
