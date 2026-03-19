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

<div class="inset-shadow mt-6 rounded-lg bg-background-inset p-1.5">
	<blockquote
		{...restProps}
		class={cn(
			"card relative rounded-md border-l-2 border-l-accent bg-background px-5 py-3 text-base font-normal text-foreground-muted italic",
			className,
		)}
	>
		{@render children?.()}
	</blockquote>
</div>
