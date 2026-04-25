<script lang="ts">
	import { onMount, tick } from "svelte";
	import { scale } from "svelte/transition";
	import Information from "carbon-icons-svelte/lib/Information.svelte";

	let { description }: { description: string } = $props();

	let isOpen = $state(false);
	let popoverRef: HTMLDivElement | undefined = $state();
	let triggerRef: HTMLButtonElement | undefined = $state();
	let coords = $state({ top: 0, left: 0 });
	let transform = $state("translate(-50%, -100%)");

	function toggle() {
		isOpen = !isOpen;
	}

	async function updatePosition() {
		if (!triggerRef) return;

		if (!popoverRef) await tick();
		if (!popoverRef) return;

		const triggerRect = triggerRef.getBoundingClientRect();
		const popoverRect = popoverRef.getBoundingClientRect();
		const padding = 10;

		let left = triggerRect.left + triggerRect.width / 2;
		const halfWidth = popoverRect.width / 2;

		if (left - halfWidth < padding) {
			left = padding + halfWidth;
		} else if (left + halfWidth > window.innerWidth - padding) {
			left = window.innerWidth - padding - halfWidth;
		}
		coords.left = left;

		const gap = 8;
		const spaceAbove = triggerRect.top - gap - padding;
		const spaceBelow =
			window.innerHeight - (triggerRect.bottom + gap + padding);

		let top = triggerRect.top - gap;
		let trans = "translate(-50%, -100%)";

		if (spaceAbove < popoverRect.height && spaceBelow > spaceAbove) {
			top = triggerRect.bottom + gap;
			trans = "translate(-50%, 0)";
		}

		coords.top = top;
		transform = trans;
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
		class="text-foreground-muted transition-[color] duration-150 ease-out hover:text-accent"
		aria-label="More info"
	>
		<Information class="size-4 flex-none" />
	</button>

	{#if isOpen}
		<div
			bind:this={popoverRef}
			transition:scale={{ duration: 150, start: 0.95 }}
			class="shadow-3xl fixed z-50 w-64 rounded-md bg-background p-3 text-sm leading-normal text-foreground card"
			style="top: {coords.top}px; left: {coords.left}px; transform: {transform};"
		>
			{description}
		</div>
	{/if}
</div>
