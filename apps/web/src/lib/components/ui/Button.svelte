<script lang="ts">
	import { cva, type VariantProps } from "class-variance-authority";
	import type { Snippet } from "svelte";
	import { cn } from "$lib/utils/cn";

	const buttonVariants = cva(
		"inline-flex items-center justify-center gap-2 whitespace-nowrap tracking-tight font-medium text-sm transition-colors duration-150 ease-out rounded-sm disabled:pointer-events-none disabled:opacity-50",
		{
			variants: {
				variant: {
					default: "btn-primary",
					secondary: "btn-secondary",
					ghost: "text-foreground hover:bg-background-inset",
				},
				size: {
					sm: "h-7 px-2.5 text-xs",
					md: "h-8 px-2.5",
					lg: "h-9 px-2.5",
					none: "",
				},
			},
			defaultVariants: {
				variant: "default",
				size: "md",
			},
		},
	);

	type ButtonVariants = VariantProps<typeof buttonVariants>;

	interface Props extends ButtonVariants {
		href?: string;
		type?: "button" | "submit" | "reset";
		disabled?: boolean;
		class?: string;
		children?: Snippet;
		[key: string]: unknown;
	}

	let {
		href,
		type = "button",
		disabled = false,
		variant = "default",
		size = "md",
		class: className = "",
		children,
		...rest
	}: Props = $props();

	const classes = $derived(cn(buttonVariants({ variant, size }), className));
</script>

{#if href}
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
	<a
		href={disabled ? undefined : href}
		class={classes}
		aria-disabled={disabled || undefined}
		tabindex={disabled ? -1 : undefined}
		{...rest}
	>
		{@render children?.()}
	</a>
{:else}
	<button {type} class={classes} {disabled} {...rest}>
		{@render children?.()}
	</button>
{/if}
