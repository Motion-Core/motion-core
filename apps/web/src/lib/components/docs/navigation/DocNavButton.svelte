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
		"group inset-shadow rounded-lg bg-background-inset p-1.5",
		align === "right" && "sm:text-right",
		forceSecondColumn && "sm:col-start-2",
	)}
>
	<a
		{href}
		class={cn(
			"group card relative flex flex-col rounded-md bg-background px-4 py-3 transition-[background-color] duration-150 ease-out hover:bg-background-muted",
		)}
	>
		<span
			class="text-xs font-normal tracking-wide text-foreground-muted/70 uppercase"
		>
			{label}
		</span>
		<span class="text-lg font-medium tracking-tight text-foreground">
			{title}
		</span>
	</a>
</div>
