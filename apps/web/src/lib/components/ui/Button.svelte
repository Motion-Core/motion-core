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

<style>
	.btn-primary,
	.btn-secondary {
		position: relative;
		appearance: none;
	}

	.btn-primary {
		overflow: hidden;
		color: var(--color-white-fixed);
		text-shadow: 0 -1px 0 rgb(0 0 0 / 0.25);
		transition:
			filter 150ms ease-out,
			box-shadow 150ms ease-out;
		background:
			radial-gradient(
				ellipse at -20px top,
				rgb(255 255 255 / 0.22),
				rgb(255 255 255 / 0)
			),
			linear-gradient(
				180deg,
				var(--color-accent),
				var(--color-accent-secondary)
			);
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.22),
			var(--shadow-md);

		&::after {
			content: "";
			position: absolute;
			inset: 0;
			border-radius: inherit;
			pointer-events: none;
			border: 1.5px solid transparent;
			-webkit-mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			-webkit-mask-composite: xor;
			mask-composite: exclude;
			background: linear-gradient(
					180deg,
					rgb(255 255 255 / 0.72),
					rgb(0 0 0 / 0.24) 41%,
					rgb(0 0 0 / 0.24) 75%,
					rgb(255 255 255 / 0.28)
				)
				border-box;
			mix-blend-mode: overlay;
		}

		@media (hover: hover) {
			&:hover {
				filter: brightness(1.08) saturate(1.04);
			}
		}
	}

	:global(.dark) .btn-primary {
		overflow: hidden;
		color: var(--color-white-fixed);
		text-shadow: 0 -1px 0 rgb(0 0 0 / 0.25);
		transition:
			filter 150ms ease-out,
			box-shadow 150ms ease-out;
		background:
			radial-gradient(
				ellipse at -20px top,
				rgb(255 255 255 / 0.22),
				rgb(255 255 255 / 0)
			),
			linear-gradient(
				180deg,
				color-mix(
					in oklab,
					var(--color-accent) 80%,
					var(--color-background) 20%
				),
				color-mix(
					in oklab,
					var(--color-accent-secondary) 80%,
					var(--color-background) 20%
				)
			);
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.22),
			var(--shadow-md);

		&::after {
			content: "";
			position: absolute;
			inset: 0;
			border-radius: inherit;
			pointer-events: none;
			border: 1.5px solid transparent;
			-webkit-mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			-webkit-mask-composite: xor;
			mask-composite: exclude;
			background: linear-gradient(
					180deg,
					rgb(255 255 255 / 0.72),
					rgb(0 0 0 / 0.24) 41%,
					rgb(0 0 0 / 0.24) 75%,
					rgb(255 255 255 / 0.28)
				)
				border-box;
			mix-blend-mode: overlay;
		}

		@media (hover: hover) {
			&:hover {
				filter: brightness(1.08) saturate(1.04);
			}
		}
	}

	.btn-secondary {
		isolation: isolate;
		overflow: hidden;
		color: var(--color-foreground);
		text-shadow: 0 -1px 0 rgb(0 0 0 / 0.15);

		transition:
			filter 150ms ease-out,
			box-shadow 150ms ease-out;
		color: var(--color-background);
		transition:
			filter 150ms ease-out,
			box-shadow 150ms ease-out;
		background:
			radial-gradient(
				ellipse at -20px top,
				rgb(255 255 255 / 0.22),
				rgb(255 255 255 / 0)
			),
			linear-gradient(
				180deg,
				var(--color-foreground),
				var(--color-foreground-muted)
			);
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.22),
			var(--shadow-md);

		&::after {
			content: "";
			position: absolute;
			inset: 0;
			border-radius: inherit;
			pointer-events: none;
			border: 1.5px solid transparent;
			-webkit-mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			-webkit-mask-composite: xor;
			mask-composite: exclude;
			background: linear-gradient(
					180deg,
					rgb(255 255 255 / 0.72),
					rgb(0 0 0 / 0.24) 41%,
					rgb(0 0 0 / 0.24) 75%,
					rgb(255 255 255 / 0.28)
				)
				border-box;
			mix-blend-mode: overlay;
		}

		@media (hover: hover) {
			&:hover {
				filter: brightness(1.08) contrast(0.9);
			}
		}
	}

	:global(.dark) .btn-secondary {
		isolation: isolate;
		overflow: hidden;
		color: var(--color-foreground);
		text-shadow: 0 -1px 0 rgb(0 0 0 / 0.15);

		transition:
			filter 150ms ease-out,
			box-shadow 150ms ease-out;
		color: var(--color-background);
		transition:
			filter 150ms ease-out,
			box-shadow 150ms ease-out;
		background:
			radial-gradient(
				ellipse at -20px top,
				rgb(255 255 255 / 0.22),
				rgb(255 255 255 / 0)
			),
			linear-gradient(
				180deg,
				color-mix(
					in oklab,
					var(--color-foreground) 80%,
					var(--color-background) 20%
				),
				color-mix(
					in oklab,
					var(--color-foreground-muted) 80%,
					var(--color-background) 20%
				)
			);
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.22),
			var(--shadow-md);

		&::after {
			content: "";
			position: absolute;
			inset: 0;
			border-radius: inherit;
			pointer-events: none;
			border: 1.5px solid transparent;
			-webkit-mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			mask:
				linear-gradient(black, black) padding-box,
				linear-gradient(black, black);
			-webkit-mask-composite: xor;
			mask-composite: exclude;
			background: linear-gradient(
					180deg,
					rgb(255 255 255 / 0.72),
					rgb(0 0 0 / 0.24) 41%,
					rgb(0 0 0 / 0.24) 75%,
					rgb(255 255 255 / 0.28)
				)
				border-box;
			mix-blend-mode: overlay;
		}

		@media (hover: hover) {
			&:hover {
				filter: brightness(1.08) contrast(0.9);
			}
		}
	}
</style>
