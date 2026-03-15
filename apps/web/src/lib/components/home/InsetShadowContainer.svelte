<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";

	interface Props {
		class?: string;
		children?: Snippet;
		[key: string]: unknown;
	}

	let { class: className = "", children, ...rest }: Props = $props();
</script>

<div
	class={cn(
		"inset-shadow w-full overflow-hidden rounded-xl border border-border bg-background-inset p-2",
		className,
	)}
	{...rest}
>
	{@render children?.()}
</div>
