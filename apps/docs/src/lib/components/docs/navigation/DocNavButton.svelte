<script lang="ts">
	import { cn } from "$lib/utils/cn";

	type ComponentProps = {
		label: string;
		title: string;
		href: string;
		align?: "left" | "right";
		forceSecondColumn?: boolean;
	};

	const {
		label,
		title,
		href,
		align = "left",
		forceSecondColumn = false,
		...rest
	}: ComponentProps = $props();
</script>

<a
	{href}
	{...rest}
	class={cn(
		"group relative flex flex-col rounded-md border border-border bg-card px-4 py-3 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted/60",
		align === "right" && "sm:text-right",
		forceSecondColumn && "sm:col-start-2",
	)}
>
	<span class="text-xs tracking-wide text-foreground/45 uppercase">
		{label}
	</span>
	<span class="text-lg font-medium text-foreground">
		{title}
	</span>
</a>
