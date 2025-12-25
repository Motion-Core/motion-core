<script lang="ts">
	import TableOfContents from "$lib/components/docs/TableOfContents.svelte";
	import DocsSidebar from "$lib/components/docs/navigation/DocsSidebar.svelte";
	import MobileSidebar from "$lib/components/docs/navigation/MobileSidebar.svelte";
	import { DocNavigation } from "$lib";
	import type { LayoutData } from "./$types";
	import type { Snippet } from "svelte";
	import { page } from "$app/state";
	import { tick } from "svelte";

	const props = $props<{ data: LayoutData; children?: Snippet }>();
	const previousLink = $derived(props.data.previousLink);
	const nextLink = $derived(props.data.nextLink);
	const metadata = $derived(props.data.metadata);
	const renderChildren = $derived(props.children);

	$effect(() => {
		const hash = page.url.hash;
		if (hash) {
			const id = hash.substring(1);

			const scrollToElement = () => {
				const element = document.getElementById(id);
				if (element) {
					element.scrollIntoView({ behavior: "smooth", block: "start" });
					return true;
				}
				return false;
			};

			tick().then(() => {
				if (!scrollToElement()) {
					setTimeout(scrollToElement, 100);
				}
			});
		}
	});
</script>

<svelte:head>
	{#if metadata}
		<title>{metadata.title} - Motion Core</title>
		<meta name="description" content={metadata.description} />

		<meta property="og:title" content={metadata.title} />
		<meta property="og:description" content={metadata.description} />
		<meta property="twitter:title" content={metadata.title} />
		<meta property="twitter:description" content={metadata.description} />
	{/if}
</svelte:head>

<main
	style="view-transition-name: homepage-content"
	class="h-svh bg-card lg:py-4 text-foreground"
>
	<MobileSidebar />

	<aside class="fixed left-0 top-0 hidden w-88 shrink-0 lg:block">
		<DocsSidebar />
	</aside>

	<div
		id="docs-content-container"
		class="mx-auto flex w-full bg-background border border-border lg:rounded-xl lg:max-h-[calc(100svh-2rem)] overflow-auto max-w-4xl flex-col gap-8 px-4 py-8 lg:ml-88 lg:px-8 xl:mr-88"
	>
		<section class="flex-1 space-y-8 min-w-0">
			{#if metadata}
				<div class="space-y-4">
					<h1 class="scroll-m-20 text-3xl font-medium">
						{metadata.name || metadata.title}
					</h1>
					{#if metadata.description}
						<p class="text-base text-foreground/70 max-w-4xl">
							{metadata.description}
						</p>
					{/if}
				</div>
				<hr class="text-border" />
			{/if}

			<div>
				{@render renderChildren?.()}

				<DocNavigation previous={previousLink} next={nextLink} />
			</div>
		</section>
	</div>

	<aside
		class="fixed right-8 top-8 hidden h-[calc(100svh-5rem)] w-48 shrink-0 flex-col xl:flex"
	>
		<TableOfContents />
	</aside>
</main>
