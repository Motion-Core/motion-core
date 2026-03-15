<script lang="ts">
	import { searchState } from "$lib/stores/search.svelte";
	import { docsUiConfig } from "$lib/config/docs-ui";
	import { cn } from "$lib/utils/cn";
	import Search from "carbon-icons-svelte/lib/Search.svelte";

	let { class: className }: { class?: string } = $props();
</script>

{#if docsUiConfig.search.enabled}
	<button
		type="button"
		class={cn(
			"group inset-shadow relative flex h-9 w-full items-center gap-2 rounded-sm border border-border bg-background-inset px-3 py-1.5 text-sm font-medium text-foreground/45 transition-[color] duration-150 ease-out hover:text-foreground-muted",
			className,
		)}
		onclick={() => searchState.open()}
	>
		<Search size={16} class="text-foreground/45" />
		<span class="flex-1 text-left"
			>{docsUiConfig.search.triggerPlaceholder}</span
		>
		{#if docsUiConfig.search.hotkey.enabled}
			<kbd
				class="pointer-events-none relative hidden h-5 items-center gap-1 rounded-xs border border-border bg-background px-1.5 font-mono text-[10px] font-medium text-foreground/45 shadow-sm select-none sm:flex"
			>
				{docsUiConfig.search.hotkey.label}
			</kbd>
		{/if}
	</button>
{/if}
