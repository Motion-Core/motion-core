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
		class="relative inline-flex w-fit rounded-sm bg-background-inset p-0.75 font-mono text-sm font-medium whitespace-nowrap inset-shadow"
	>
		<code
			{...restProps}
			class={cn(
				"rounded-[calc(var(--radius-base)*1.25)] bg-background px-1.5 py-0.5 text-foreground card",
				className,
			)}
		>
			{@render children?.()}
		</code>
	</span>
{/if}
