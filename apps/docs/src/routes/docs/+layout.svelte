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
	import DocShareActions from "$lib/components/docs/DocShareActions.svelte";
	import { docsManifest } from "$lib/docs/manifest";

	const props = $props<{ data: LayoutData; children?: Snippet }>();
	const previousLink = $derived(props.data.previousLink);
	const nextLink = $derived(props.data.nextLink);
	const metadata = $derived(props.data.metadata);
	const renderChildren = $derived(props.children);
	const docSlug = $derived(metadata?.slug);
	const currentDoc = $derived(docsManifest.find((d) => d.slug === docSlug));
	const dependencies = $derived(
		currentDoc?.dependencies ? Object.keys(currentDoc.dependencies) : [],
	);
	const rawPath = $derived(docSlug ? `/docs/raw/${docSlug}` : null);
	const docOrigin = $derived(props.data.docOrigin);
	const rawUrl = $derived(
		rawPath && docOrigin ? new URL(rawPath, docOrigin).href : null,
	);
	const repoRelativePath = $derived(
		metadata ? `/apps/docs/src/routes${metadata.href}/+page.svx` : null,
	);
	const githubUrl = $derived(
		repoRelativePath
			? `https://github.com/motion-core/motion-core/blob/master${repoRelativePath}`
			: null,
	);

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

<main class="h-svh bg-card text-foreground lg:py-4">
	<MobileSidebar />

	<aside class="fixed top-0 left-0 hidden w-88 shrink-0 lg:block">
		<DocsSidebar />
	</aside>

	<ScrollArea
		id="docs-content-container"
		class="mx-auto mt-12 w-full max-w-4xl border-border bg-background lg:mt-0 lg:ml-88 lg:max-h-[calc(100svh-2rem)] lg:rounded-2xl lg:border xl:mr-88"
	>
		<div class="flex flex-col gap-8 px-4 py-8 lg:px-8">
			<section class="min-w-0 flex-1 space-y-8">
				{#if metadata}
					<div class="space-y-4">
						<h1
							class="scroll-m-20 text-3xl font-medium text-foreground font-display"
						>
							{metadata.name || metadata.title}
						</h1>
						{#if metadata.description}
							<p class="max-w-4xl text-base text-foreground/70">
								{metadata.description}
							</p>
						{/if}
						{#if dependencies.length > 0}
							<div class="flex gap-2">
								{#each dependencies as dep (dep)}
									<a
										href={`https://www.npmjs.com/package/${dep}`}
										target="_blank"
										rel="noreferrer"
										class="inline-flex items-center rounded-full border border-border bg-card px-2.5 py-0.5 text-xs font-medium text-foreground/70 transition-[color] duration-150 ease-out hover:text-foreground"
									>
										{dep}
									</a>
								{/each}
							</div>
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
		class="fixed top-8 right-8 hidden h-[calc(100svh-4rem)] w-53 shrink-0 flex-col xl:flex"
	>
		<TableOfContents />
		{#if metadata && rawPath && rawUrl && githubUrl}
			<DocShareActions {rawPath} {rawUrl} {githubUrl} />
		{/if}
	</aside>
</main>
