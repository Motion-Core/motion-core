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
		"relative group flex flex-col rounded-md border border-border bg-card px-4 py-3 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted card-highlight",
		align === "right" && "sm:text-right",
		forceSecondColumn && "sm:col-start-2",
	)}
	data-sveltekit-preload-data
>
	<span class="text-[10px] uppercase tracking-wide text-foreground/45">
		{label}
	</span>
	<span class="text-base font-normal text-foreground">
		{title}
	</span>
</a>
