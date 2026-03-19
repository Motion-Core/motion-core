<script lang="ts">
	import { cva, type VariantProps } from "class-variance-authority";
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";
	const sectionVariants = cva("w-full border-t border-border border-dashed", {
		variants: {
			variant: {
				default: "",
				muted: "",
				full: "",
			},
		},
		defaultVariants: {
			variant: "default",
		},
	});
	interface Props extends VariantProps<typeof sectionVariants> {
		id?: string;
		class?: string;
		children?: Snippet;
	}
	let {
		id,
		variant = "default",
		class: className = "",
		children,
	}: Props = $props();
</script>

<section
	{id}
	data-reveal-section
	data-section-variant={variant}
	class={sectionVariants({ variant })}
>
	<div
		class={cn(
			"w-full",
			variant === "muted" &&
				"grid items-stretch sm:grid-cols-[minmax(0,1fr)_minmax(0,64rem)_minmax(0,1fr)]",
		)}
	>
		{#if variant === "muted"}
			<div
				class="hidden h-full min-h-20 items-center justify-start bg-dashed px-3 sm:flex"
			></div>
		{/if}
		<div
			class={cn(
				"relative mx-auto flex w-full flex-col justify-start px-4 py-8 sm:px-8 sm:py-16",
				variant === "default" &&
					"max-w-5xl border-x border-dashed border-border",
				variant === "muted" && "max-w-5xl border-x border-border",
				className,
			)}
		>
			{@render children?.()}
		</div>
		{#if variant === "muted"}
			<div
				class="hidden h-full min-h-20 items-center justify-start bg-dashed px-3 sm:flex"
			></div>
		{/if}
	</div>
</section>
