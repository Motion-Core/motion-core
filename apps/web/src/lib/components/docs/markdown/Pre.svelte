<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";
	import CopyCodeButton from "./CopyCodeButton.svelte";

	type ComponentProps = {
		class?: string;
		children?: Snippet;
		code?: string;
		unstyled?: boolean;
		[prop: string]: unknown;
	};

	const props = $props();
	const className = $derived((props as ComponentProps).class ?? "");
	const code = $derived((props as ComponentProps).code ?? "");
	const unstyled = $derived((props as ComponentProps).unstyled ?? false);
	const children = $derived((props as ComponentProps).children);
	const restProps = $derived(() => {
		const {
			class: _class,
			children: _children,
			code: _code,
			unstyled: _unstyled,
			...rest
		} = props as ComponentProps;
		return rest;
	});
</script>

<div
	class="inset-shadow mt-8 rounded-lg border border-border bg-background-inset p-1"
>
	<div
		{...restProps}
		class={cn(
			unstyled
				? "group/pre relative font-mono text-base font-normal"
				: "group/pre relative rounded-md border border-border bg-background p-4 font-mono text-base font-normal text-foreground shadow-md",
			className,
		)}
	>
		<div class="overflow-x-auto">
			{@render children?.()}
		</div>
		{#if code}
			<div class="pointer-events-none absolute top-2 right-2">
				<CopyCodeButton {code} class="pointer-events-auto" />
			</div>
		{/if}
	</div>
</div>

<style>
	:global(.shiki) {
		background-color: transparent !important;
		font-size: 14px;
		font-weight: 400;
	}
</style>
