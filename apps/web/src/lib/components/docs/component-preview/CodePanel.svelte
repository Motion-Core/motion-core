<script lang="ts">
	import type { Snippet } from "svelte";
	import { getHighlighter } from "$lib/utils/highlighter";
	import ScrollArea from "../../ui/ScrollArea.svelte";
	import ShikiCodeBlock from "../ShikiCodeBlock.svelte";
	import CopyCodeButton from "../markdown/CopyCodeButton.svelte";
	import type { SourceTab } from "./types";
	import { cn } from "$lib/utils/cn";

	type Props = {
		tabs: SourceTab[];
		codeSlot?: Snippet;
	};

	let { tabs, codeSlot }: Props = $props();
	let activeTab = $state(0);
	let highlightedSources = $state<
		Record<string, { light: string; dark: string }>
	>({});

	$effect(() => {
		void tabs;
		if (activeTab > tabs.length - 1) {
			activeTab = 0;
		}
	});

	const activeSource = $derived(
		(tabs.at(activeTab) ?? null) as SourceTab | null,
	);

	$effect(() => {
		getHighlighter().then((highlighter) => {
			tabs.forEach((tab) => {
				if (!highlightedSources[tab.name]) {
					const lang = tab.language ?? "typescript";
					const light = highlighter.codeToHtml(tab.code, {
						lang,
						theme: "github-light",
					});
					const dark = highlighter.codeToHtml(tab.code, {
						lang,
						theme: "github-dark",
					});
					highlightedSources[tab.name] = { light, dark };
				}
			});
		});
	});
</script>

<div
	class="mt-2 flex flex-1 flex-col overflow-hidden rounded-md rounded-b-md bg-background card"
>
	{#if tabs.length}
		<div class="flex items-center border-b border-border bg-background text-sm">
			<div class="flex flex-1 items-center overflow-x-auto">
				{#each tabs as tab, index (tab.name)}
					<button
						type="button"
						class={cn(
							"border-b-2 px-4 py-2.5 text-sm font-medium tracking-normal whitespace-nowrap transition-colors duration-150 ease-out",
							index === activeTab
								? "border-accent text-foreground"
								: "border-transparent text-foreground-muted hover:text-foreground",
						)}
						onclick={() => (activeTab = index)}
					>
						{tab.name}
					</button>
				{/each}
			</div>
			<div class="mr-2 w-fit flex-none">
				{#if activeSource}
					<CopyCodeButton class="size-6" code={activeSource.code} />
				{/if}
			</div>
		</div>
	{/if}
	<ScrollArea id="component-preview" class="relative max-h-96 flex-1">
		<div
			class="p-4 text-sm *:mt-0 *:rounded-none *:border-0 *:bg-transparent *:p-0 *:inset-shadow-none"
		>
			{#if activeSource}
				{#if highlightedSources[activeSource.name]}
					<ShikiCodeBlock
						code=""
						htmlLight={highlightedSources[activeSource.name].light}
						htmlDark={highlightedSources[activeSource.name].dark}
						unstyled={true}
					/>
				{:else}
					<pre class="p-4">{activeSource.code}</pre>
				{/if}
			{:else}
				{@render codeSlot?.()}
			{/if}
		</div>
	</ScrollArea>
</div>
