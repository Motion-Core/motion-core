<script lang="ts">
	import { page } from "$app/state";
	import DocsSidebar from "./DocsSidebar.svelte";
	import { brandingConfig } from "$lib/config/branding";
	import Menu from "carbon-icons-svelte/lib/Menu.svelte";
	import Close from "carbon-icons-svelte/lib/Close.svelte";

	let isOpen = $state(false);
	let isVisible = $state(false);
	const pathname = $derived(page.url.pathname);

	function open() {
		if (isVisible) {
			isOpen = true;
			return;
		}

		isVisible = true;
		requestAnimationFrame(() => {
			isOpen = true;
		});
	}

	function toggle() {
		if (isOpen) {
			close();
			return;
		}

		open();
	}

	function close() {
		isOpen = false;
	}

	function handleSidebarTransitionEnd(event: TransitionEvent) {
		if (
			event.target !== event.currentTarget ||
			event.propertyName !== "transform"
		)
			return;
		if (!isOpen) isVisible = false;
	}

	$effect(() => {
		void pathname;
		close();
	});
</script>

<div
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-between border-b border-border bg-background px-4 py-1.5 lg:hidden"
>
	<a
		href="/"
		class="inline-flex items-center gap-1 px-2 py-2 text-sm tracking-tight text-foreground transition-colors duration-150 ease-out hover:text-foreground"
	>
		<span
			class="inline-flex shrink-0 items-center text-accent [&>svg]:size-4 [&>svg]:fill-current"
			aria-hidden="true"
		>
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html brandingConfig.logoRaw}
		</span>
		<span class="font-medium tracking-tight text-foreground"
			>{brandingConfig.name}</span
		>
	</a>
	<button
		onclick={toggle}
		class="-mr-2 inline-flex size-10 items-center justify-center gap-2 text-sm whitespace-nowrap text-foreground transition-colors duration-150 ease-out hover:bg-background-inset lg:hidden"
		aria-label="Toggle menu"
	>
		<Menu size={20} />
	</button>
</div>

{#if isVisible}
	<div
		class="overlay fixed inset-0 z-50 bg-background-inset/80 backdrop-blur-sm lg:hidden"
		class:active={isOpen}
		onclick={close}
		role="button"
		tabindex="-1"
		onkeydown={(e) => {
			if (e.key === "Escape") close();
		}}
		aria-label="Close sidebar"
		aria-hidden={!isOpen}
	></div>

	<div
		class="sidebar fixed inset-y-0 right-0 z-50 w-3/4 max-w-sm overflow-hidden border-l border-border bg-background-inset text-foreground-muted shadow-xl lg:hidden"
		class:active={isOpen}
		ontransitionend={handleSidebarTransitionEnd}
	>
		<div class="absolute top-0 right-0 flex justify-end p-4">
			<button onclick={close} aria-label="Close menu">
				<Close size={32} class="size-6" />
			</button>
		</div>
		<DocsSidebar />
	</div>
{/if}

<style>
	.overlay {
		opacity: 0;
		pointer-events: none;
		transition: opacity 200ms ease-out;
		will-change: opacity;
	}

	.overlay.active {
		opacity: 1;
		pointer-events: auto;
	}

	.sidebar {
		transform: translateX(100%);
		transition: transform 200ms ease-out;
		will-change: transform;
	}

	.sidebar.active {
		transform: translateX(0);
	}
</style>
