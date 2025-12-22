<script lang="ts">
	import { scale } from "svelte/transition";
	import { onMount } from "svelte";

	let { description }: { description: string } = $props();

	let isOpen = $state(false);
	let popoverRef: HTMLDivElement | undefined = $state();
	let triggerRef: HTMLButtonElement | undefined = $state();
	let coords = $state({ top: 0, left: 0 });

	function toggle() {
		isOpen = !isOpen;
	}

	function updatePosition() {
		if (triggerRef) {
			const rect = triggerRef.getBoundingClientRect();
			coords.left = rect.left + rect.width / 2;
			coords.top = rect.top;
		}
	}

	function handleClickOutside(event: MouseEvent) {
		if (
			isOpen &&
			popoverRef &&
			!popoverRef.contains(event.target as Node) &&
			triggerRef &&
			!triggerRef.contains(event.target as Node)
		) {
			isOpen = false;
		}
	}

	$effect(() => {
		if (isOpen) {
			updatePosition();
			window.addEventListener("scroll", updatePosition, true);
			window.addEventListener("resize", updatePosition);
			return () => {
				window.removeEventListener("scroll", updatePosition, true);
				window.removeEventListener("resize", updatePosition);
			};
		}
	});

	onMount(() => {
		document.addEventListener("click", handleClickOutside);
		return () => {
			document.removeEventListener("click", handleClickOutside);
		};
	});
</script>

<div class="relative inline-flex items-center ml-1.5 align-middle">
	<button
		bind:this={triggerRef}
		onclick={toggle}
		class="text-foreground/70 hover:text-foreground transition-colors"
		aria-label="More info"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="14"
			height="14"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
		>
			<circle cx="12" cy="12" r="10" />
			<path d="M12 16v-4" />
			<path d="M12 8h.01" />
		</svg>
	</button>

	{#if isOpen}
		<div
			bind:this={popoverRef}
			transition:scale={{ duration: 150, start: 0.95 }}
			class="fixed w-64 p-3 rounded-md border border-border bg-card text-foreground shadow-lg z-50 text-xs leading-normal"
			style="top: {coords.top -
				8}px; left: {coords.left}px; transform: translate(-50%, -100%);"
		>
			{description}
		</div>
	{/if}
</div>
