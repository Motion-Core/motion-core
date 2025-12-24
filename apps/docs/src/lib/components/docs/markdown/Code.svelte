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

	const isBlock = (classValue: string | undefined, dataTheme: unknown) => {
		if (dataTheme !== undefined) return true;
		if (!classValue) return false;

		return classValue
			.split(/\s+/)
			.some((token) => token.startsWith("language-"));
	};
</script>

{#if isBlock(typeof className === "string" ? className : undefined, restProps["data-theme"])}
	<code
		{...restProps}
		class={cn("block whitespace-pre mono text-sm leading-relaxed", className)}
	>
		{@render children?.()}
	</code>
{:else}
	<div
		class="relative inline-block w-fit rounded bg-card border border-border shadow-sm px-1.5 py-0.5 mono text-xs text-foreground"
	>
		<code {...restProps} class={cn("", className)}>
			{@render children?.()}
		</code>
	</div>
{/if}
