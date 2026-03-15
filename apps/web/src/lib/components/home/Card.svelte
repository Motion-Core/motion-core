<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";

	interface Props {
		number?: string;
		title: string;
		description: string;
		class?: string;
		icon?: Snippet;
	}

	let {
		number,
		title,
		description,
		class: className = "",
		icon,
	}: Props = $props();
</script>

<div
	class="inset-shadow relative overflow-hidden rounded-lg border border-border bg-background-inset p-1"
>
	<article
		class={cn(
			"grid h-54 rounded-md border border-border bg-background p-4 shadow-md sm:h-72 sm:p-6",
			className,
		)}
	>
		<div
			class={cn(
				"flex items-start",
				number ? "justify-between" : "justify-start",
			)}
		>
			<div
				class="inset-shadow grid size-12 place-items-center rounded-sm border border-border bg-background-inset text-accent"
			>
				{@render icon?.()}
			</div>
			{#if number}
				<div class="font-mono text-base font-medium text-foreground-muted">
					{number}
				</div>
			{/if}
		</div>

		<div class="mt-auto grid gap-4">
			<h3 class="text-xl font-medium tracking-tight text-foreground">
				{title}
			</h3>
			<p class="text-base font-normal text-foreground-muted">{description}</p>
		</div>
	</article>
</div>
