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
	}: ComponentProps = $props();
</script>

<div
	class={cn(
		"group inset-shadow rounded-lg border border-border bg-background-inset p-1",
		align === "right" && "sm:text-right",
		forceSecondColumn && "sm:col-start-2",
	)}
>
	<a
		{href}
		class={cn(
			"group relative flex flex-col rounded-md border border-border bg-background px-4 py-3 shadow-md transition-[background-color] duration-150 ease-out hover:bg-background-inset",
		)}
	>
		<span
			class="text-xs font-normal tracking-wide text-foreground/45 uppercase"
		>
			{label}
		</span>
		<span class="text-lg font-medium tracking-tight text-foreground">
			{title}
		</span>
	</a>
</div>
