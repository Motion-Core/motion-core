<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "$lib/utils/cn";
import CopyCodeButton from "./CopyCodeButton.svelte";

type ComponentProps = {
	class?: string;
	children?: Snippet;
	code?: string;
	[prop: string]: unknown;
};

const props = $props();
const className = $derived((props as ComponentProps).class ?? "");
const code = $derived((props as ComponentProps).code ?? "");
const children = $derived((props as ComponentProps).children);
const restProps = $derived(() => {
	const { class: _class, children: _children, code: _code, ...rest } =
		props as ComponentProps;
	return rest;
});
</script>

<div
	{...restProps}
	class={cn(
		"group/pre relative mt-8 rounded-md border border-border bg-card p-4 font-mono text-sm text-foreground",
		className,
	)}
>
	<div class="overflow-x-auto">
		{@render children?.()}
	</div>
	{#if code}
		<div class="pointer-events-none absolute right-2 top-2">
			<CopyCodeButton
				code={code}
				class="pointer-events-auto bg-background"
			/>
		</div>
	{/if}
</div>

<style>
	:global(.shiki) {
		background-color: transparent !important;
		font-size: 12px;
		font-weight: 400;
	}
	:global(.shiki-theme-dark) {
		display: none;
	}
	:global(.dark .shiki-theme-dark) {
		display: block;
	}
	:global(.dark .shiki-theme-light) {
		display: none;
	}
</style>
