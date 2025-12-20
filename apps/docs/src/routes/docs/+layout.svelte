<script lang="ts">
	import TableOfContents from "$lib/components/docs/TableOfContents.svelte";
	import type { LayoutData } from "./$types";
	import type { Snippet } from "svelte";

	const props = $props<{ data: LayoutData; children?: Snippet }>();
	const previousLink = $derived(props.data.previousLink);
	const nextLink = $derived(props.data.nextLink);
	const renderChildren = $derived(props.children);
</script>

<main class="min-h-svh bg-background text-foreground">
	<div class="mx-auto flex w-full max-w-3xl flex-col gap-8 px-4 py-8 lg:flex-row">
		<section class="flex-1 space-y-8 lg:order-2">
			<a
				href="/"
				class="inline-flex items-center text-sm font-medium text-foreground/70 transition-colors hover:text-foreground"
				data-sveltekit-preload-data
			>
				<span aria-hidden="true">←</span>
				<span class="ml-2">Back to library</span>
			</a>
			<div>
				{@render renderChildren?.()}

				{#if previousLink || nextLink}
					<nav class="mt-16 border-t border-border pt-8">
						<div class="grid gap-4 sm:grid-cols-2">
							{#if previousLink}
								<a
									class="relative group flex flex-col rounded-md border border-border bg-card px-4 py-3 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted card-highlight"
									href={previousLink.href}
									data-sveltekit-preload-data
								>
									<span class="text-[10px] font-medium uppercase tracking-wide text-foreground/45">
										Previous
									</span>
									<span class="text-base font-normal text-foreground">
										{previousLink.title}
									</span>
								</a>
							{/if}

							{#if nextLink}
								<a
									class={`relative group flex flex-col rounded-md border border-border bg-card px-4 py-3 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted sm:text-right card-highlight ${previousLink ? "" : "sm:col-start-2"}`}
									href={nextLink.href}
									data-sveltekit-preload-data
								>
									<span class="text-[10px] font-medium uppercase tracking-wide text-foreground/45">
										Next
									</span>
									<span class="text-base font-normal text-foreground">
										{nextLink.title}
									</span>
								</a>
							{/if}
						</div>
					</nav>
				{/if}
			</div>
		</section>
	</div>
	<aside class="fixed right-8 top-8 hidden h-[calc(100svh-5rem)] w-64 shrink-0 flex-col lg:flex">
		<TableOfContents />
	</aside>
</main>
