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
	import { beforeNavigate, afterNavigate } from "$app/navigation";
	import { SvelteMap } from "svelte/reactivity";
	import DocShareActions from "$lib/components/docs/DocShareActions.svelte";
	import MobileDocShareActions from "$lib/components/docs/MobileDocShareActions.svelte";
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

	const tocSelector = $derived(
		docSlug?.startsWith("changelog/") ? "[data-doc-content] h2" : undefined,
	);

	const scrollContainerId = "docs-content-container";
	const scrollPositions = new SvelteMap<string, number>();

	beforeNavigate(() => {
		const elem = document.getElementById(scrollContainerId);
		if (elem) {
			scrollPositions.set(page.url.pathname, elem.scrollTop);
		}
	});

	afterNavigate((nav) => {
		const elem = document.getElementById(scrollContainerId);
		if (elem && !page.url.hash) {
			if (nav.type === "popstate") {
				const saved = scrollPositions.get(page.url.pathname);
				if (saved !== undefined) {
					elem.scrollTop = saved;
				}
			} else {
				elem.scrollTop = 0;
			}
		}
	});

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

<main class="relative h-dvh bg-card text-foreground lg:py-2">
	<MobileSidebar />

	<aside class="fixed top-0 left-0 hidden w-88 shrink-0 lg:block">
		<DocsSidebar />
	</aside>

	<div
		class="card-highlight relative mx-auto w-full max-w-4xl overflow-hidden border-border bg-background pt-12 md:overflow-visible lg:ml-88 lg:max-h-[calc(100dvh-1rem)] lg:rounded-2xl lg:border lg:pt-0 xl:mr-88"
	>
		<ScrollArea
			id="docs-content-container"
			class="mx-auto w-full lg:max-h-[calc(100dvh-1rem)]"
		>
			<div class="flex flex-col gap-8 px-4 py-8 lg:px-8">
				<section class="min-w-0 flex-1 space-y-8">
					{#if metadata}
						<div class="space-y-4">
							{#if currentDoc?.category}
								<p
									class="mb-2 text-sm font-medium text-foreground/45 capitalize"
								>
									{currentDoc.category}
								</p>
							{/if}
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
								<div class="flex flex-wrap gap-2">
									{#each dependencies as dep (dep)}
										<a
											href={`https://www.npmjs.com/package/${dep}`}
											target="_blank"
											rel="noreferrer"
											class="card-highlight relative inline-flex items-center rounded-full border border-border bg-card px-2.5 py-0.5 text-xs font-medium text-foreground/70 shadow-sm transition-[color] duration-150 ease-out hover:text-foreground"
										>
											{dep}
										</a>
									{/each}
								</div>
							{/if}

							{#if metadata && rawPath && rawUrl && githubUrl}
								<MobileDocShareActions {rawPath} {rawUrl} {githubUrl} />
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
	</div>

	<aside
		class="fixed top-8 right-8 hidden h-[calc(100dvh-4rem)] w-53 shrink-0 flex-col xl:flex"
	>
		<TableOfContents selector={tocSelector} />
		{#if metadata && rawPath && rawUrl && githubUrl}
			<DocShareActions {rawPath} {rawUrl} {githubUrl} />
		{/if}
	</aside>
</main>
