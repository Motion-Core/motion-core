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

<div class="relative ml-1.5 inline-flex items-center align-middle">
	<button
		bind:this={triggerRef}
		onclick={toggle}
		class="text-foreground/70 transition-[color] duration-150 ease-out hover:text-accent"
		aria-label="More info"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			aria-hidden="true"
			width="16"
			height="16"
			fill="currentColor"
			viewBox="0 0 256 256"
			><path
				d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm16-40a8,8,0,0,1-8,8,16,16,0,0,1-16-16V128a8,8,0,0,1,0-16,16,16,0,0,1,16,16v40A8,8,0,0,1,144,176ZM112,84a12,12,0,1,1,12,12A12,12,0,0,1,112,84Z"
			></path></svg
		>
	</button>

	{#if isOpen}
		<div
			bind:this={popoverRef}
			transition:scale={{ duration: 150, start: 0.95 }}
			class="fixed z-50 w-64 rounded-md border border-border bg-card p-3 text-sm leading-normal text-foreground shadow-lg"
			style="top: {coords.top -
				8}px; left: {coords.left}px; transform: translate(-50%, -100%);"
		>
			{description}
		</div>
	{/if}
</div>
