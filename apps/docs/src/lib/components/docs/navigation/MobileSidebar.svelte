<script lang="ts">
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
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-between border-b border-border bg-card px-4 py-2 lg:hidden"
>
	<a href="/" class="flex items-center gap-2">
		<span
			class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
			aria-hidden="true"
		>
			{@html motionCoreLogo}
		</span>
		<span class="text-xl font-medium text-foreground font-display"
			>Motion Core</span
		>
	</a>
	<button
		onclick={toggle}
		class="-mr-2 p-2 text-foreground/70 hover:text-foreground"
		aria-label="Toggle menu"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="32"
			height="32"
			fill="currentColor"
			viewBox="0 0 256 256"
			class="size-6"
		>
			<path
				d="M224,128a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM40,72H216a8,8,0,0,0,0-16H40a8,8,0,0,0,0,16ZM216,184H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16Z"
			></path>
		</svg>
	</button>
</div>

<div
	class="overlay fixed inset-0 z-50 bg-background/80 backdrop-blur-sm lg:hidden"
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
	class="sidebar fixed inset-y-0 right-0 z-50 w-3/4 max-w-sm border-l border-border bg-background text-foreground/70 shadow-xl lg:hidden"
	class:active={isOpen}
>
	<div class="absolute top-0 right-0 flex justify-end p-4">
		<button onclick={close} aria-label="Close menu">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="32"
				height="32"
				fill="currentColor"
				viewBox="0 0 256 256"
				class="size-6"
			>
				<path
					d="M205.66,194.34a8,8,0,0,1-11.32,11.32L128,139.31,61.66,205.66a8,8,0,0,1-11.32-11.32L116.69,128,50.34,61.66A8,8,0,0,1,61.66,50.34L128,116.69l66.34-66.35a8,8,0,0,1,11.32,11.32L139.31,128Z"
				></path>
			</svg>
		</button>
	</div>
	<DocsSidebar />
</div>

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
