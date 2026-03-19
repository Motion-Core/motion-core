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

<tr
	{...restProps}
	class={cn(
		"text-base leading-loose font-normal transition-[background-color] duration-150 ease-out hover:bg-background data-[state=selected]:bg-background [&_code]:text-sm [&strong]:text-base",
		className,
	)}
>
	{@render children?.()}
</tr>
