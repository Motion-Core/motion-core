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
		class={cn("block text-sm leading-relaxed whitespace-pre mono", className)}
	>
		{@render children?.()}
	</code>
{:else}
	<div
		class="card-highlight relative inline-block w-fit rounded-lg border border-border bg-card px-1.5 py-0.5 text-xs font-medium text-foreground shadow-sm mono"
	>
		<code {...restProps} class={cn("", className)}>
			{@render children?.()}
		</code>
	</div>
{/if}
