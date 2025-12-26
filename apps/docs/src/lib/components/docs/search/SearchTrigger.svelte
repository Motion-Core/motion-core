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
		"group flex h-9 w-full items-center gap-2 rounded-lg border border-border bg-background px-3 text-sm text-foreground/45 transition-[color] duration-150 ease-out hover:text-foreground/70",
		className,
	)}
	onclick={() => searchState.open()}
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		aria-hidden="true"
		width="18"
		height="18"
		fill="currentColor"
		class="text-foreground/45"
		viewBox="0 0 256 256"
		><path
			d="M229.66,218.34l-50.07-50.06a88.11,88.11,0,1,0-11.31,11.31l50.06,50.07a8,8,0,0,0,11.32-11.32ZM40,112a72,72,0,1,1,72,72A72.08,72.08,0,0,1,40,112Z"
		></path></svg
	>
	<span class="flex-1 text-left">Search...</span>
	<kbd
		class="pointer-events-none hidden h-5 items-center gap-1 rounded-md border border-border bg-card px-1.5 text-[10px] font-medium text-foreground/45 opacity-100 shadow-sm select-none mono sm:flex"
	>
		<span class="text-xs">⌘</span>K
	</kbd>
</button>
