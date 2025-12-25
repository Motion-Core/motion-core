<script lang="ts">
	import { searchState } from "$lib/stores/search.svelte";
	import { onMount } from "svelte";
	import { cn } from "$lib/utils/cn";

	let { class: className }: { class?: string } = $props();

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === "k") {
			e.preventDefault();
			searchState.toggle();
		}
	}

	onMount(() => {
		window.addEventListener("keydown", handleKeydown);
		return () => {
			window.removeEventListener("keydown", handleKeydown);
		};
	});
</script>

<button
	type="button"
	class={cn(
		"group flex h-9 w-full items-center gap-2 rounded-md border border-border bg-card-muted/60 px-3 text-sm text-foreground/50 transition-[color] duration-150 ease-out hover:bg-card-muted hover:text-foreground",
		className,
	)}
	onclick={() => searchState.open()}
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
		class="shrink-0 opacity-50"
	>
		<circle cx="11" cy="11" r="8" />
		<path d="m21 21-4.3-4.3" />
	</svg>
	<span class="flex-1 text-left">Search...</span>
	<kbd
		class="pointer-events-none hidden h-5 items-center gap-1 rounded border border-border bg-card px-1.5 text-[10px] font-medium text-foreground/45 opacity-100 select-none mono sm:flex"
	>
		<span class="text-xs">⌘</span>K
	</kbd>
</button>
