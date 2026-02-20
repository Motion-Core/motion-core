<script lang="ts">
	import { cn } from "$lib/utils/cn";

	type ComponentProps = Record<string, unknown> & { class?: string };
	const { class: className = "", ...restProps }: ComponentProps = $props();
</script>

<hr
	{...restProps}
	class={cn(
		"my-12 h-px border-0 shadow-2xs dark:bg-black dark:shadow-border light:bg-black/15 light:shadow-white",
		className,
	)}
/>
