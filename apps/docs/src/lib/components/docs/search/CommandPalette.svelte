<script lang="ts">
	import { searchState } from "$lib/stores/search.svelte";
	import { searchDocs } from "$lib/utils/search";
	import { fade, scale } from "svelte/transition";
	import { cubicOut } from "svelte/easing";
	import { goto } from "$app/navigation";
	import { onNavigate } from "$app/navigation";
	import { cn } from "$lib/utils/cn";

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
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="mr-2 text-foreground/50"
				>
					<circle cx="11" cy="11" r="8" />
					<path d="m21 21-4.3-4.3" />
				</svg>
				<input
					bind:this={inputRef}
					bind:value={query}
					class="flex h-12 w-full bg-transparent text-base text-foreground placeholder:text-foreground/45 focus:outline-none"
					placeholder="Search documentation..."
					aria-label="Search documentation"
				/>
				<kbd
					class="pointer-events-none hidden h-5 items-center gap-1 rounded border border-border bg-card-muted/50 px-1.5 text-[10px] font-medium text-foreground/50 select-none mono sm:flex"
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
						<div class="max-h-96 overflow-y-auto p-2">
							{#each results as result, i (result.slug + (result.anchor || "") + i)}
								{@const isChild = result.matchType === "heading"}
								<button
									class={cn(
										"group relative flex w-full flex-col items-start gap-1 rounded-md px-3 py-2 text-sm",
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

									<div class="flex items-center gap-2 font-medium">
										{#if isChild}
											<span class="opacity-70">#</span>
										{/if}
										{isChild ? result.heading : result.title}
									</div>
								</button>
							{/each}
						</div>
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
