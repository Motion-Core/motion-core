<script lang="ts">
	import type { Snippet } from "svelte";
	import { browser } from "$app/environment";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";
	import Highlight from "svelte-highlight";
	import typescript from "svelte-highlight/languages/typescript";
	import xml from "svelte-highlight/languages/xml";
	import githubDark from "svelte-highlight/styles/github-dark";
	import github from "svelte-highlight/styles/github";

	type SourceTab = {
		name: string;
		code: string;
		language?: string;
	};

	type ComponentProps = {
		code?: string;
		language?: string;
		label?: string;
		children?: Snippet;
		codeSlot?: Snippet;
		sources?: SourceTab[];
	};

	const {
		children,
		codeSlot,
		code: providedCode,
		language: providedLanguage,
		label: providedLabel,
		sources: providedSources,
		...restProps
	}: ComponentProps = $props();

const tabs = $derived(
	(() => {
		const normalized = providedSources?.filter(
			(tab): tab is SourceTab => Boolean(tab?.code),
		) ?? [];

		if (normalized.length > 0) {
			return normalized;
		}

		if (providedCode) {
			return [
				{
					name: providedLabel ?? "Code",
					code: providedCode,
					language: providedLanguage,
				},
			];
		}

		return [];
	})() as SourceTab[],
);

	let activeTab = $state(0);

	$effect(() => {
		tabs;
		if (activeTab > tabs.length - 1) {
			activeTab = 0;
		}
	});

const activeSource = $derived(
	(tabs.at(activeTab) ?? null) as SourceTab | null,
);
	const resolveLanguage = (language?: string) => {
		switch (language?.toLowerCase()) {
			case "svelte":
			case "xml":
			case "html":
				return xml;
			default:
				return typescript;
		}
	};
	const highlightLanguage = $derived(resolveLanguage(activeSource?.language));

	let isDark = $state(false);

	$effect(() => {
		if (!browser) return;

		const updateTheme = () => {
			isDark = document.documentElement.classList.contains('dark');
		};

		updateTheme();

		const observer = new MutationObserver(updateTheme);
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class']
		});

		return () => observer.disconnect();
	});

	const highlightStyle = $derived(isDark ? githubDark : github);
</script>

<svelte:head>
	{@html highlightStyle}
</svelte:head>

<section
	class="relative rounded-lg border bg-background border-border card-highlight shadow-sm max-w-[calc(var(--container-3xl)-2rem)]"
	{...restProps}
>
	<div class="flex flex-col">
		<div class="relative flex-1 flex items-center justify-center border-b border-border min-h-80">
			{@render children?.()}
		</div>
		<div class="flex flex-1 flex-col bg-card rounded-b-lg">
			{#if tabs.length}
				<div class="relative flex items-center text-xs font-medium">
					{#each tabs as tab, index}
						<button
							type="button"
							class={`border-b-2 px-3 py-2 transition-colors ${
								index === activeTab
									? "border-primary text-foreground"
									: "border-transparent text-foreground/70 hover:text-foreground"
							}`}
							onclick={() => (activeTab = index)}
						>
							{tab.name}
						</button>
					{/each}
					<div class="absolute top-1/2 -translate-y-1/2 right-1">
						{#if activeSource}
							<CopyCodeButton class="size-6" code={activeSource.code} />
						{/if}
					</div>
				</div>
			{/if}
			<div class="relative max-h-80 flex-1 overflow-auto p-1 text-xs">
				{#if activeSource}
					<Highlight
						class="bg-transparent h-full"
						language={highlightLanguage}
						code={activeSource.code}
					/>
				{:else}
					{@render codeSlot?.()}
				{/if}
			</div>
		</div>
	</div>
</section>

<style>
	:global(.hljs) {
		background-color: transparent !important;
	}
</style>
