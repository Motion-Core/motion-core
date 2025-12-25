<script lang="ts">
	import type { Snippet } from "svelte";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";
	import ShikiCodeBlock from "./ShikiCodeBlock.svelte";
	import ScrollArea from "../ui/ScrollArea.svelte";
	import { getHighlighter } from "$lib/utils/highlighter";
	import gsap from "gsap";
	import { Flip } from "gsap/Flip";
	import { tick, onMount } from "svelte";
	import { cn } from "$lib/utils/cn";

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
		[key: string]: unknown;
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

	let isFullScreen = $state(false);
	let previewKey = $state(0);
	let previewRef: HTMLElement;
	let placeholderRef: HTMLElement;

	const reloadPreview = () => {
		previewKey += 1;
	};

	onMount(() => {
		gsap.registerPlugin(Flip);
	});

	const toggleFullScreen = async () => {
		if (!previewRef || !placeholderRef) return;

		/* eslint-disable svelte/no-dom-manipulating */
		const state = Flip.getState(previewRef);

		if (!isFullScreen) {
			const rect = previewRef.getBoundingClientRect();
			placeholderRef.style.height = `${rect.height}px`;
			placeholderRef.style.width = `${rect.width}px`;

			isFullScreen = true;
			await tick();

			document.body.appendChild(previewRef);

			previewRef.style.setProperty("position", "fixed", "important");
			previewRef.style.setProperty("top", "0", "important");
			previewRef.style.setProperty("left", "0", "important");
			previewRef.style.setProperty("width", "100vw", "important");
			previewRef.style.setProperty("height", "100dvh", "important");
			previewRef.style.setProperty("margin", "0", "important");
		} else {
			isFullScreen = false;
			placeholderRef.appendChild(previewRef);

			await tick();

			placeholderRef.style.height = "";
			placeholderRef.style.width = "";
			previewRef.style.cssText = "";
		}

		Flip.from(state, {
			duration: 0.5,
			ease: "power3.inOut",
			absolute: true,
			zIndex: 50,
		});
	};
	const tabs = $derived(
		(() => {
			const normalized =
				providedSources?.filter((tab): tab is SourceTab =>
					Boolean(tab?.code),
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
		void tabs;
		if (activeTab > tabs.length - 1) {
			activeTab = 0;
		}
	});

	const activeSource = $derived(
		(tabs.at(activeTab) ?? null) as SourceTab | null,
	);

	let highlightedSources = $state<
		Record<string, { light: string; dark: string }>
	>({});

	$effect(() => {
		getHighlighter().then((highlighter) => {
			tabs.forEach((tab) => {
				// Actually using tab.name as key in record implies names are unique per preview.
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

<section
	class="relative max-w-[calc(var(--container-4xl)-2rem)] rounded-lg border border-border bg-background shadow-sm"
	{...restProps}
>
	<div class="flex flex-col">
		<div
			bind:this={placeholderRef}
			class="relative flex min-h-96 flex-1 flex-col items-center justify-center rounded-t-lg border-b border-border"
		>
			<div
				bind:this={previewRef}
				class={cn(
					"relative flex items-center justify-center overflow-hidden bg-card",
					isFullScreen ? "" : "w-full flex-1 rounded-t-lg",
				)}
			>
				<button
					onclick={reloadPreview}
					class="transition-scale absolute top-2 right-11 z-30 flex size-7 cursor-pointer items-center justify-center rounded-sm border border-border bg-card text-foreground shadow-sm duration-150 ease-out active:scale-[0.95]"
					aria-label="Reload Preview"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
						<path d="M21 3v5h-5" />
					</svg>
				</button>
				<button
					onclick={toggleFullScreen}
					class="transition-scale absolute top-2 right-2 z-30 flex size-7 cursor-pointer items-center justify-center rounded-sm border border-border bg-card text-foreground shadow-sm duration-150 ease-out active:scale-[0.95]"
					aria-label={isFullScreen ? "Exit Fullscreen" : "Enter Fullscreen"}
				>
					{#if isFullScreen}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="lucide lucide-minimize-2"
							><polyline points="4 14 10 14 10 20" /><polyline
								points="20 10 14 10 14 4"
							/><line x1="14" x2="21" y1="10" y2="3" /><line
								x1="3"
								x2="10"
								y1="21"
								y2="14"
							/></svg
						>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							><polyline points="15 3 21 3 21 9" /><polyline
								points="9 21 3 21 3 15"
							/><line x1="21" x2="14" y1="3" y2="10" /><line
								x1="3"
								x2="10"
								y1="21"
								y2="14"
							/></svg
						>
					{/if}
				</button>
				{#key previewKey}
					{@render children?.()}
				{/key}
			</div>
		</div>
		<div class="flex flex-1 flex-col overflow-hidden rounded-b-lg bg-card">
			{#if tabs.length}
				<div
					class="flex items-center border-b border-border bg-card-muted/50 text-sm"
				>
					<div
						class="no-scrollbar mask-gradient-r flex flex-1 items-center overflow-x-auto"
					>
						{#each tabs as tab, index (tab.name)}
							<button
								type="button"
								class={cn(
									"border-b-2 px-4 py-2.5 whitespace-nowrap transition-all duration-150 ease-out",
									index === activeTab
										? "border-accent bg-accent/5 font-medium text-foreground"
										: "hover:bg-muted/60 border-transparent text-foreground/60 hover:text-foreground",
								)}
								onclick={() => (activeTab = index)}
							>
								{tab.name}
							</button>
						{/each}
					</div>
					<div class="flex-none px-2">
						{#if activeSource}
							<CopyCodeButton class="size-6" code={activeSource.code} />
						{/if}
					</div>
				</div>
			{/if}
			<ScrollArea class="relative max-h-96 flex-1">
				<div class="p-1 text-sm">
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
	</div>
</section>
