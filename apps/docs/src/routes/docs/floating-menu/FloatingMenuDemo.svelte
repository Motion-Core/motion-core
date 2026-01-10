<script lang="ts">
	import { FloatingMenu } from "motion-core";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";

	const menuGroups = [
		{
			title: "Platform",
			variant: "muted" as const,
			links: [
				{ label: "Home", href: "#" },
				{ label: "Components", href: "#" },
				{ label: "Showcase", href: "#" },
			],
		},
		{
			title: "Resources",
			variant: "default" as const,
			links: [
				{ label: "Documentation", href: "#" },
				{ label: "API Reference", href: "#" },
				{ label: "Community", href: "#" },
			],
		},
		{
			title: "Company",
			variant: "muted" as const,
			links: [
				{ label: "About Us", href: "#" },
				{ label: "Blog", href: "#" },
				{ label: "Careers", href: "#" },
				{ label: "Contact", href: "#" },
			],
		},
	];
	let demoContainer: HTMLElement;
</script>

<div
	bind:this={demoContainer}
	class="relative flex h-full min-h-150 w-full scale-[1] transform items-center justify-center overflow-hidden p-4"
>
	<p class="text-center font-display">
		Due to the nature of the component, testing in full-screen mode is
		recommended.
	</p>
	<FloatingMenu
		portalTarget={demoContainer}
		{menuGroups}
		primaryButton={{ label: "Get Started", href: "#" }}
		secondaryButton={{ label: "Log In", href: "#" }}
	>
		{#snippet logo()}
			<a href="/" class="flex items-center">
				<span
					class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
					aria-hidden="true"
				>
					{@html motionCoreLogo}
				</span>
			</a>
		{/snippet}
	</FloatingMenu>
</div>
