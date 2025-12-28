<script lang="ts">
	import { searchState } from "$lib/stores/search.svelte";
	import { searchDocs } from "$lib/utils/search";
	import { fade, scale } from "svelte/transition";
	import { cubicOut } from "svelte/easing";
	import { goto } from "$app/navigation";
	import { onNavigate } from "$app/navigation";
	import { cn } from "$lib/utils/cn";
	import ScrollArea from "$lib/components/ui/ScrollArea.svelte";

	let query = $state("");
	let results = $derived(searchDocs(query));
	let selectedIndex = $state(0);
	let inputRef = $state<HTMLInputElement>();
	let contentHeight = $state(0);

	$effect(() => {
		if (searchState.isOpen && inputRef) {
			inputRef.focus();
		}
	});

	$effect(() => {
		void results;
		selectedIndex = 0;
	});

	function close() {
		searchState.close();
		query = "";
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!searchState.isOpen) return;

		if (e.key === "ArrowDown") {
			e.preventDefault();
			selectedIndex = (selectedIndex + 1) % results.length;
		} else if (e.key === "ArrowUp") {
			e.preventDefault();
			selectedIndex = (selectedIndex - 1 + results.length) % results.length;
		} else if (e.key === "Enter") {
			e.preventDefault();
			if (results[selectedIndex]) {
				selectResult(results[selectedIndex]);
			}
		} else if (e.key === "Escape") {
			e.preventDefault();
			close();
		}
	}

	$effect(() => {
		if (searchState.isOpen) {
			window.addEventListener("keydown", handleKeydown);
			return () => window.removeEventListener("keydown", handleKeydown);
		}
	});

	function selectResult(result: ReturnType<typeof searchDocs>[number]) {
		const href = `${result.slug}${result.anchor || ""}`;
		goto(href);
		close();
	}

	onNavigate(() => {
		close();
	});

	function highlight(text: string, search: string) {
		if (!search.trim()) return [{ text, highlight: false }];

		const escapedSearch = search.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
		const regex = new RegExp(`(${escapedSearch})`, "gi");
		const parts = text.split(regex);

		return parts.map((part) => ({
			text: part,
			highlight: part.toLowerCase() === search.toLowerCase(),
		}));
	}
</script>

{#if searchState.isOpen}
	<div
		class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
		onclick={close}
		role="presentation"
	></div>

	<div
		class="fixed inset-0 z-50 flex items-start justify-center p-4 sm:pt-[10vh]"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={(e) => e.target === e.currentTarget && close()}
		onkeydown={(e) => e.key === "Escape" && close()}
	>
		<div
			class="w-full max-w-164 overflow-hidden rounded-xl border border-border bg-card shadow-2xl"
			role="document"
			transition:scale={{
				duration: 200,
				start: 0.95,
				easing: cubicOut,
			}}
		>
			<div class="flex items-center border-b border-border px-3">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					aria-hidden="true"
					width="24"
					height="24"
					fill="currentColor"
					class="mr-2 text-foreground/45"
					viewBox="0 0 256 256"
					><path
						d="M229.66,218.34l-50.07-50.06a88.11,88.11,0,1,0-11.31,11.31l50.06,50.07a8,8,0,0,0,11.32-11.32ZM40,112a72,72,0,1,1,72,72A72.08,72.08,0,0,1,40,112Z"
					></path></svg
				>
				<input
					bind:this={inputRef}
					bind:value={query}
					class="flex h-12 w-full bg-transparent text-base text-foreground placeholder:text-foreground/45 focus:outline-none"
					placeholder="Search documentation..."
					aria-label="Search documentation"
				/>
				<kbd
					class="pointer-events-none hidden h-5 items-center gap-1 rounded-md border border-border bg-card-muted px-1.5 text-[10px] font-medium text-foreground/45 shadow-sm select-none mono sm:flex"
				>
					ESC
				</kbd>
			</div>

			<div
				class="overflow-hidden transition-[height] duration-200 ease-out"
				style="height: {contentHeight}px"
			>
				<div bind:clientHeight={contentHeight}>
					{#if results.length > 0}
						<ScrollArea viewportClass="max-h-96 p-2">
							{#each results as result, i (result.slug + (result.anchor || "") + i)}
								{@const isChild =
									result.matchType === "heading" || result.matchType === "content"}
								<button
									class={cn(
										"group relative flex w-full flex-col items-start gap-1 rounded-lg px-3 py-2 text-sm",
										isChild && "pl-8",
										i === selectedIndex
											? "bg-accent/10 text-accent"
											: "text-foreground hover:bg-card-muted",
									)}
									onclick={() => selectResult(result)}
									onmouseenter={() => (selectedIndex = i)}
								>
									{#if isChild}
										<div
											class={cn(
												"absolute top-0 bottom-0 left-3 w-px",
												i === selectedIndex ? "bg-accent/30" : "bg-border",
											)}
										></div>
									{/if}

									<div class="flex flex-col items-start gap-0.5 w-full">
										{#if result.matchType !== "content"}
											<div class="flex items-center gap-2 font-medium">
												{#if result.matchType === "heading"}
													<span class="opacity-70">#</span>
												{/if}
												<span>
													{#each highlight(result.heading || result.title, query) as part, index (index)}
														{#if part.highlight}
															<span class="text-accent">{part.text}</span>
														{:else}
															{part.text}
														{/if}
													{/each}
												</span>
											</div>
										{/if}
										{#if result.snippet}
											<div
												class={cn(
													"text-xs line-clamp-1 text-left",
													i === selectedIndex
														? "text-accent/80"
														: "text-foreground/60",
												)}
											>
												{#each highlight(result.snippet, query) as part, index (index)}
													{#if part.highlight}
														<span class="text-accent">{part.text}</span>
													{:else}
														{part.text}
													{/if}
												{/each}
											</div>
										{/if}
									</div>
								</button>
							{/each}
						</ScrollArea>
					{:else if query}
						<div class="py-6 text-center text-sm text-foreground/45">
							No results found.
						</div>
					{:else}
						<div class="px-4 py-4 text-sm text-foreground/45">
							Type to search...
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
