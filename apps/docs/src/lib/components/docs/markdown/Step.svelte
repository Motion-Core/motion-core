<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import type { Snippet } from "svelte";

	let {
		class: className,
		children,
		title,
	} = $props<{
		class?: string;
		children?: Snippet;
		title?: string;
	}>();
</script>

<div class={cn("relative pb-10 pl-8", className)}>
	{#if title}
		<div class="mb-4 flex h-8 items-center">
			<span
				class="absolute -left-4 flex size-8 items-center justify-center rounded-full bg-card text-xs font-medium text-foreground [counter-increment:step] before:content-[counter(step)]"
			>
				<span class="card-highlight absolute z-0 size-8 rounded-full shadow-sm"
				></span>
			</span>
			<h3 class="relative z-10 text-base leading-none font-medium">
				{title}
			</h3>
		</div>
	{:else}
		<span
			class="absolute top-1 -left-4 flex size-8 items-center justify-center rounded-full border border-border bg-card text-xs font-medium text-foreground [counter-increment:step] before:content-[counter(step)]"
		></span>
	{/if}
	<div class="text-base leading-relaxed text-foreground/70">
		{@render children?.()}
	</div>
</div>
