<script lang="ts">
	import type { Snippet } from "svelte";
	import { browser } from "$app/environment";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";
	import Highlight from "svelte-highlight";
	import typescript from "svelte-highlight/languages/typescript";
	import xml from "svelte-highlight/languages/xml";
	import githubDark from "svelte-highlight/styles/github-dark";
	import github from "svelte-highlight/styles/github";
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
			isDark = document.documentElement.classList.contains("dark");
		};

		updateTheme();

		const observer = new MutationObserver(updateTheme);
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class"],
		});

		return () => observer.disconnect();
	});

	const highlightStyle = $derived(isDark ? githubDark : github);
</script>

<svelte:head>
	{@html highlightStyle}
</svelte:head>

<section
	class="relative rounded-lg border bg-background border-border shadow-sm max-w-[calc(var(--container-4xl)-2rem)]"
	{...restProps}
>
	<div class="flex flex-col">
		<div
			bind:this={placeholderRef}
			class="relative flex min-h-80 flex-1 flex-col items-center justify-center rounded-t-lg border-b border-border"
		>
			<div
				bind:this={previewRef}
				class={cn(
					"relative flex items-center justify-center overflow-hidden bg-card",
					isFullScreen ? "" : "flex-1 w-full rounded-t-lg",
				)}
			>
				<button
					onclick={reloadPreview}
					class="absolute right-8 top-1 z-50 flex size-6 items-center justify-center bg-card rounded-sm border border-border text-foreground cursor-pointer active:scale-[0.95] transition-scale duration-150 ease-out shadow-sm"
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
						class="lucide lucide-rotate-cw"
					>
						<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
						<path d="M21 3v5h-5" />
					</svg>
				</button>
				<button
					onclick={toggleFullScreen}
					class="absolute right-1 top-1 z-50 flex size-6 items-center justify-center bg-card rounded-sm border border-border text-foreground cursor-pointer active:scale-[0.95] transition-scale duration-150 ease-out shadow-sm"
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
							class="lucide lucide-maximize-2"
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
		<div class="flex flex-1 flex-col bg-card rounded-b-lg">
			{#if tabs.length}
				<div class="relative flex items-center text-xs">
					{#each tabs as tab, index (tab.name)}
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
