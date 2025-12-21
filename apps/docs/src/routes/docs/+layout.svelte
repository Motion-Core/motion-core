<script lang="ts">
	import TableOfContents from "$lib/components/docs/TableOfContents.svelte";
	import { DocNavigation } from "$lib";
	import type { LayoutData } from "./$types";
	import type { Snippet } from "svelte";

	const props = $props<{ data: LayoutData; children?: Snippet }>();
	const previousLink = $derived(props.data.previousLink);
	const nextLink = $derived(props.data.nextLink);
	const renderChildren = $derived(props.children);
</script>

<main
	style="view-transition-name: homepage-content"
	class="min-h-svh bg-background text-foreground"
>
	<div
		class="mx-auto flex w-full max-w-3xl flex-col gap-8 px-4 py-8 lg:flex-row"
	>
		<section class="flex-1 space-y-8 lg:order-2">
			<a
				href="/"
				class="inline-flex items-center text-sm text-foreground/70 transition-[color] duration-150 ease-out hover:text-foreground"
				data-sveltekit-preload-data
			>
				<span aria-hidden="true">←</span>
				<span class="ml-2">Back to library</span>
			</a>
			<div>
				{@render renderChildren?.()}

				<DocNavigation previous={previousLink} next={nextLink} />
			</div>
		</section>
	</div>
	<aside
		class="fixed right-8 top-8 hidden h-[calc(100svh-5rem)] w-64 shrink-0 flex-col lg:flex"
	>
		<TableOfContents />
	</aside>
</main>
