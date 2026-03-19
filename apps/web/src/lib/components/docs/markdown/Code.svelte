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
		class={cn(
			"block font-mono text-sm leading-relaxed whitespace-pre",
			className,
		)}
	>
		{@render children?.()}
	</code>
{:else}
	<span
		class="inset-shadow relative inline-block w-fit rounded-sm bg-background-inset px-0.75 py-1 font-mono text-sm font-medium whitespace-nowrap text-foreground"
	>
		<code
			{...restProps}
			class={cn(
				"card rounded-[calc(var(--radius-base)*1.25)] bg-background px-1.5 py-0.5",
				className,
			)}
		>
			{@render children?.()}
		</code>
	</span>
{/if}
