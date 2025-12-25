<script lang="ts">
	import { fade, fly } from "svelte/transition";
	import { page } from "$app/state";
	import DocsSidebar from "./DocsSidebar.svelte";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";

	let isOpen = $state(false);
	const pathname = $derived(page.url.pathname);

	function toggle() {
		isOpen = !isOpen;
	}

	function close() {
		isOpen = false;
	}

	$effect(() => {
		void pathname;
		close();
	});
</script>

<div
	class="lg:hidden flex items-center justify-between px-4 py-2 border-b border-border bg-card sticky top-0 z-50"
>
	<a href="/" class="flex items-center gap-2">
		<span
			class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
			aria-hidden="true"
		>
			{@html motionCoreLogo}
		</span>
		<span class="text-xl text-foreground font-display font-medium"
			>Motion Core</span
		>
	</a>
	<button
		onclick={toggle}
		class="p-2 -mr-2 text-foreground/70 hover:text-foreground"
		aria-label="Toggle menu"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="w-6 h-6"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
			/>
		</svg>
	</button>
</div>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm lg:hidden"
		transition:fade={{ duration: 200 }}
		onclick={close}
		role="button"
		tabindex="0"
		onkeydown={(e) => {
			if (e.key === "Escape") close();
		}}
		aria-label="Close sidebar"
	></div>

	<div
		class="fixed inset-y-0 right-0 z-50 w-3/4 max-w-sm border-l border-border bg-background shadow-xl lg:hidden"
		transition:fly={{ x: "100%", duration: 300, opacity: 1 }}
	>
		<div class="absolute top-0 right-0 flex justify-end p-4">
			<button onclick={close} aria-label="Close menu">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="size-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M6 18 18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>
		<DocsSidebar />
	</div>
{/if}
