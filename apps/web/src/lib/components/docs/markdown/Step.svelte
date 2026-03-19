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

<div class={cn("relative pl-8", className)}>
	{#if title}
		<div class="mb-4 flex h-10 items-center">
			<div
				class="inset-shadow absolute -left-5 flex size-10 rounded-full bg-background-inset p-1.5"
			>
				<span
					class="card flex h-full w-full items-center justify-center rounded-full bg-background text-xs font-medium text-foreground [counter-increment:step] before:content-[counter(step)]"
				>
				</span>
			</div>
			<h3 class="relative z-10 text-lg leading-none font-medium tracking-tight">
				{title}
			</h3>
		</div>
	{:else}
		<div
			class="inset-shadow absolute -left-5 flex size-10 rounded-full bg-background-inset p-1.5"
		>
			<span
				class="flex h-full w-full items-center justify-center rounded-full bg-background text-xs font-medium text-foreground [counter-increment:step] before:content-[counter(step)]"
			>
			</span>
		</div>
	{/if}
	<div
		class={cn(
			"text-base leading-relaxed tracking-normal text-foreground-muted",
			!title && "pt-2",
		)}
	>
		{@render children?.()}
	</div>
</div>
