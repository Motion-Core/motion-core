<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";
	import ScrollArea from "$lib/components/ui/ScrollArea.svelte";

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

<div class="inset-shadow my-8 rounded-lg bg-background-inset p-1.5">
	<div class="card relative rounded-md bg-background">
		<ScrollArea
			mode="horizontal"
			class="w-full rounded-md"
			viewportClass="rounded-md"
		>
			<table
				{...restProps}
				class={cn("w-full text-lg [&_code]:text-base", className)}
			>
				{@render children?.()}
			</table>
		</ScrollArea>
	</div>
</div>
