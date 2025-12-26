<script lang="ts">
	import TableOfContents from "$lib/components/docs/TableOfContents.svelte";
	import DocsSidebar from "$lib/components/docs/navigation/DocsSidebar.svelte";
	import MobileSidebar from "$lib/components/docs/navigation/MobileSidebar.svelte";
	import ScrollArea from "$lib/components/ui/ScrollArea.svelte";
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
	class="h-svh bg-card text-foreground lg:py-4"
>
	<MobileSidebar />

	<aside class="fixed top-0 left-0 hidden w-88 shrink-0 lg:block">
		<DocsSidebar />
	</aside>

	<ScrollArea
		id="docs-content-container"
		class="mx-auto w-full max-w-4xl border border-border bg-background lg:ml-88 lg:max-h-[calc(100svh-2rem)] lg:rounded-xl xl:mr-88"
	>
		<div class="flex flex-col gap-8 px-4 py-8 lg:px-8">
			<section class="min-w-0 flex-1 space-y-8">
				{#if metadata}
					<div class="space-y-4">
						<h1 class="scroll-m-20 text-3xl font-medium">
							{metadata.name || metadata.title}
						</h1>
						{#if metadata.description}
							<p class="max-w-4xl text-base text-foreground/70">
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
	</ScrollArea>

	<aside
		class="fixed top-8 right-8 hidden h-[calc(100svh-5rem)] w-50 shrink-0 flex-col xl:flex"
	>
		<TableOfContents />
	</aside>
</main>
