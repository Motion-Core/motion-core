<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "$lib/utils/cn";

type ComponentProps = {
	class?: string;
	children?: Snippet;
	[prop: string]: unknown;
};

let { children, class: className = "", ...restProps }: ComponentProps = $props();

const isBlock = (
	classValue: string | undefined,
	dataTheme: unknown,
) => {
	if (dataTheme !== undefined) return true;
	if (!classValue) return false;

	return classValue.split(/\s+/).some((token) => token.startsWith("language-"));
};
</script>

<code
	{...restProps}
	class={cn(
		isBlock(typeof className === "string" ? className : undefined, restProps["data-theme"])
			? "block whitespace-pre font-mono text-sm leading-relaxed"
			: "rounded bg-card border border-border px-1.5 py-0.5 font-mono text-sm text-foreground font-normal",
		className,
	)}
>
	{@render children?.()}
</code>
