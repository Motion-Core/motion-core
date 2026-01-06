<script lang="ts">
	import type { Snippet } from "svelte";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";
	import ShikiCodeBlock from "./ShikiCodeBlock.svelte";
	import ScrollArea from "../ui/ScrollArea.svelte";
	import { getHighlighter } from "$lib/utils/highlighter";
	import { gsap } from "gsap/dist/gsap";
	import { Flip } from "gsap/dist/Flip";
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

		if (!isFullScreen) {
			const state = Flip.getState(previewRef);
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

			Flip.from(state, {
				duration: 0.5,
				ease: "power3.inOut",
				absolute: true,
				zIndex: 50,
			});
		} else {
			Flip.fit(previewRef, placeholderRef, {
				duration: 0.5,
				ease: "power3.inOut",
				absolute: true,
				zIndex: 50,
				onComplete: () => {
					isFullScreen = false;
					placeholderRef.appendChild(previewRef);
					placeholderRef.style.height = "";
					placeholderRef.style.width = "";
					previewRef.style.cssText = "";
				},
			});
		}
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
	class="relative max-w-[calc(var(--container-4xl)-2rem)] rounded-xl border border-border bg-background shadow-sm"
	{...restProps}
>
	<div class="flex flex-col">
		<div
			bind:this={placeholderRef}
			class="relative flex min-h-96 flex-1 flex-col items-center justify-center rounded-t-xl border-b border-border"
		>
			<div
				bind:this={previewRef}
				class={cn(
					"relative flex items-center justify-center overflow-hidden bg-card",
					isFullScreen ? "z-50" : "w-full flex-1 rounded-t-xl",
				)}
			>
				<button
					onclick={reloadPreview}
					class="transition-scale absolute top-2 right-11 z-30 flex size-7 cursor-pointer items-center justify-center rounded-lg border border-border bg-background text-foreground shadow-sm duration-150 ease-out active:scale-[0.95]"
					aria-label="Reload Preview"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						aria-hidden="true"
						width="16"
						height="16"
						fill="currentColor"
						viewBox="0 0 256 256"
						><path
							d="M224,48V96a8,8,0,0,1-8,8H168a8,8,0,0,1,0-16h28.69L182.06,73.37a79.56,79.56,0,0,0-56.13-23.43h-.45A79.52,79.52,0,0,0,69.59,72.71,8,8,0,0,1,58.41,61.27a96,96,0,0,1,135,.79L208,76.69V48a8,8,0,0,1,16,0ZM186.41,183.29a80,80,0,0,1-112.47-.66L59.31,168H88a8,8,0,0,0,0-16H40a8,8,0,0,0-8,8v48a8,8,0,0,0,16,0V179.31l14.63,14.63A95.43,95.43,0,0,0,130,222.06h.53a95.36,95.36,0,0,0,67.07-27.33,8,8,0,0,0-11.18-11.44Z"
						></path></svg
					>
				</button>
				<button
					onclick={toggleFullScreen}
					class="transition-scale absolute top-2 right-2 z-30 flex size-7 cursor-pointer items-center justify-center rounded-lg border border-border bg-background text-foreground shadow-sm duration-150 ease-out active:scale-[0.95]"
					aria-label={isFullScreen ? "Exit Fullscreen" : "Enter Fullscreen"}
				>
					{#if isFullScreen}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							aria-hidden="true"
							width="16"
							height="16"
							fill="currentColor"
							viewBox="0 0 256 256"
							><path
								d="M213.66,53.66,163.31,104H192a8,8,0,0,1,0,16H144a8,8,0,0,1-8-8V64a8,8,0,0,1,16,0V92.69l50.34-50.35a8,8,0,0,1,11.32,11.32ZM112,136H64a8,8,0,0,0,0,16H92.69L42.34,202.34a8,8,0,0,0,11.32,11.32L104,163.31V192a8,8,0,0,0,16,0V144A8,8,0,0,0,112,136Z"
							></path></svg
						>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							aria-hidden="true"
							width="16"
							height="16"
							fill="currentColor"
							viewBox="0 0 256 256"
							><path
								d="M216,48V96a8,8,0,0,1-16,0V67.31l-50.34,50.35a8,8,0,0,1-11.32-11.32L188.69,56H160a8,8,0,0,1,0-16h48A8,8,0,0,1,216,48ZM106.34,138.34,56,188.69V160a8,8,0,0,0-16,0v48a8,8,0,0,0,8,8H96a8,8,0,0,0,0-16H67.31l50.35-50.34a8,8,0,0,0-11.32-11.32Z"
							></path></svg
						>
					{/if}
				</button>
				{#key previewKey}
					{@render children?.()}
				{/key}
			</div>
		</div>
		<div class="flex flex-1 flex-col overflow-hidden rounded-b-xl bg-card">
			{#if tabs.length}
				<div
					class="flex items-center border-b border-border bg-card-muted/60 text-sm"
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
										? "border-accent text-foreground"
										: "border-transparent text-foreground/70 hover:text-foreground",
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
				<div class="p-4 text-sm">
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
