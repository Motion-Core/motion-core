<script lang="ts">
	import {
		DocNavigation,
		CommandPalette,
		DocShareActions,
		DocsSidebar,
		MobileDocShareActions,
		MobileSidebar,
		ScrollArea,
		TableOfContents,
		docsManifest,
		docsUiConfig,
		getDocHref,
		resolveRepositoryDocUrl,
		resolveTocSelector,
		siteConfig,
	} from "$lib";
	import type { LayoutData } from "./$types";
	import type { Snippet } from "svelte";
	import { page } from "$app/state";
	import { tick } from "svelte";
	import { beforeNavigate, afterNavigate } from "$app/navigation";
	import { SvelteMap } from "svelte/reactivity";

	const props = $props<{ data: LayoutData; children?: Snippet }>();
	const previousLink = $derived(
		props.data.previousDoc
			? {
					title: props.data.previousDoc.name,
					href: getDocHref(props.data.previousDoc.slug),
				}
			: null,
	);
	const nextLink = $derived(
		props.data.nextDoc
			? {
					title: props.data.nextDoc.name,
					href: getDocHref(props.data.nextDoc.slug),
				}
			: null,
	);
	const metadata = $derived(props.data.metadata);
	const renderChildren = $derived(props.children);
	const componentDependencies = $derived(
		props.data.componentDependencies ?? [],
	);
	const docSlug = $derived(metadata?.slug);
	const currentDoc = $derived(docsManifest.find((d) => d.slug === docSlug));
	const siteOrigin = new URL(siteConfig.url).origin;
	const canonicalUrl = $derived(
		metadata ? new URL(metadata.href, siteOrigin).href : null,
	);
	const docOgImage = $derived(
		metadata
			? new URL(
					metadata.slug ? `/docs/og/${metadata.slug}` : "/docs/og",
					siteOrigin,
				).href
			: new URL(siteConfig.ogImage, siteOrigin).href,
	);
	const docTitle = $derived(
		metadata?.name || metadata?.title || currentDoc?.name || siteConfig.name,
	);
	const docDescription = $derived(
		metadata?.description || siteConfig.description,
	);
	const docStructuredData = $derived.by(() => {
		if (!canonicalUrl) return null;
		return JSON.stringify({
			"@context": "https://schema.org",
			"@type": "TechArticle",
			headline: docTitle,
			description: docDescription,
			url: canonicalUrl,
			author: {
				"@type": "Person",
				name: siteConfig.author,
			},
			publisher: {
				"@type": "Organization",
				name: siteConfig.name,
			},
			mainEntityOfPage: canonicalUrl,
		});
	});
	const breadcrumbStructuredData = $derived.by(() => {
		if (!canonicalUrl) return null;
		return JSON.stringify({
			"@context": "https://schema.org",
			"@type": "BreadcrumbList",
			itemListElement: [
				{
					"@type": "ListItem",
					position: 1,
					name: "Home",
					item: siteOrigin,
				},
				{
					"@type": "ListItem",
					position: 2,
					name: "Documentation",
					item: new URL("/docs", siteOrigin).href,
				},
				{
					"@type": "ListItem",
					position: 3,
					name: docTitle,
					item: canonicalUrl,
				},
			],
		});
	});
	const rawDocSlug = $derived(
		metadata
			? metadata.href.replace(/^\/docs(?:\/|$)/, "").replace(/\/+$/, "") ||
					"index"
			: null,
	);
	const rawPath = $derived(rawDocSlug ? `/docs/raw/${rawDocSlug}` : null);
	const docOrigin = $derived(props.data.docOrigin);
	const rawUrl = $derived(
		rawPath && docOrigin ? new URL(rawPath, docOrigin).href : null,
	);
	const repoRelativePath = $derived(
		metadata ? `/apps/web/src/routes${metadata.href}/+page.svx` : null,
	);
	const githubUrl = $derived(
		repoRelativePath
			? resolveRepositoryDocUrl(siteConfig.links.github, repoRelativePath)
			: null,
	);
	const showDocActions = $derived(
		docsUiConfig.docActions.enabled && Boolean(metadata),
	);
	const showToc = $derived(docsUiConfig.toc.enabled);

	const tocSelector = $derived(resolveTocSelector(docSlug));

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
		let fallbackTimer: ReturnType<typeof setTimeout> | null = null;
		if (hash) {
			const id = hash.substring(1);

			const scrollToElement = () => {
				const element = document.getElementById(id);
				if (element) {
					element.scrollIntoView({
						behavior: "smooth",
						block: "start",
					});
					return true;
				}
				return false;
			};

			tick().then(() => {
				if (!scrollToElement()) {
					fallbackTimer = setTimeout(scrollToElement, 100);
				}
			});
		}

		return () => {
			if (fallbackTimer) {
				clearTimeout(fallbackTimer);
			}
		};
	});
</script>

<svelte:head>
	{#if metadata}
		<title>{docTitle} - {siteConfig.name}</title>
		<meta name="description" content={docDescription} />
		<link rel="canonical" href={canonicalUrl} />

		<meta property="og:type" content="article" />
		<meta property="og:title" content={docTitle} />
		<meta property="og:description" content={docDescription} />
		<meta property="og:url" content={canonicalUrl} />
		<meta property="og:image" content={docOgImage} />
		<meta
			property="og:image:alt"
			content={`${siteConfig.name} documentation`}
		/>
		<meta property="og:image:type" content="image/png" />
		<meta property="og:image:width" content="1200" />
		<meta property="og:image:height" content="630" />
		<meta name="twitter:card" content="summary_large_image" />
		<meta name="twitter:title" content={docTitle} />
		<meta name="twitter:description" content={docDescription} />
		<meta name="twitter:image" content={docOgImage} />
		{#if docStructuredData}
			<svelte:element this={"script"} type="application/ld+json">
				{docStructuredData}
			</svelte:element>
		{/if}
		{#if breadcrumbStructuredData}
			<svelte:element this={"script"} type="application/ld+json">
				{breadcrumbStructuredData}
			</svelte:element>
		{/if}
	{/if}
</svelte:head>

{#if docsUiConfig.search.enabled}
	<CommandPalette />
{/if}
<main class="relative h-dvh bg-background text-foreground">
	<MobileSidebar />

	<div
		class="flex h-full w-full min-w-0 lg:grid lg:grid-cols-[22rem_minmax(0,1fr)] xl:grid-cols-[22rem_minmax(0,56rem)_18rem] xl:justify-center"
	>
		<aside class="hidden lg:block">
			<DocsSidebar />
		</aside>

		<div
			class="relative mx-auto h-full w-full max-w-4xl min-w-0 overflow-hidden bg-background-inset pt-12 inset-shadow lg:my-4 lg:mr-4 lg:max-h-[calc(100dvh-2rem)] lg:overflow-visible lg:rounded-xl lg:pt-0 xl:mr-0"
		>
			<ScrollArea
				id="docs-content-container"
				class="mx-auto h-full w-full lg:max-h-[calc(100dvh-2rem)]"
				viewportClass="rounded-lg overscroll-none flex flex-col gap-8 px-4 py-8 lg:px-8"
				viewportStyle="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
			>
				<section class="min-w-0 flex-1 space-y-8">
					{#if metadata}
						<div class="space-y-4">
							{#if currentDoc?.category}
								<p
									class="mb-2 text-sm font-medium tracking-normal text-foreground-muted/70 capitalize"
								>
									{currentDoc.category}
								</p>
							{/if}
							<h1
								class="scroll-m-20 text-3xl font-medium tracking-tight text-foreground"
							>
								{metadata.name || metadata.title}
							</h1>
							{#if metadata.description}
								<p
									class="max-w-4xl text-base font-normal tracking-normal text-pretty text-foreground-muted"
								>
									{metadata.description}
								</p>
							{/if}
							{#if componentDependencies.length > 0}
								<div class="flex flex-wrap items-center gap-2">
									{#each componentDependencies as dependency (dependency.name)}
										<a
											href={dependency.href}
											target="_blank"
											rel="noreferrer"
											aria-label={`Open ${dependency.name} on npm`}
											class="relative inline-block w-fit rounded-sm bg-background-inset px-0.75 py-1 font-mono text-sm font-medium whitespace-nowrap text-foreground inset-shadow"
										>
											<code
												class="rounded-[calc(var(--radius-base)*1.25)] bg-background px-1.5 py-0.5 card transition-colors duration-150 ease-out hover:bg-background-muted"
											>
												{dependency.label}
											</code>
										</a>
									{/each}
								</div>
							{/if}

							{#if showDocActions}
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
			</ScrollArea>
		</div>

		<aside class="hidden xl:block xl:w-full xl:py-8 xl:pr-4 xl:pl-4">
			<div
				class="sticky top-8 flex h-full max-h-[calc(100dvh-4rem)] min-h-0 flex-col"
			>
				{#if showToc}
					<div class="min-h-0 flex-1">
						<TableOfContents
							selector={tocSelector}
							title={docsUiConfig.toc.title}
							emptyLabel={docsUiConfig.toc.emptyLabel}
							minViewportWidth={docsUiConfig.toc.minViewportWidth}
						/>
					</div>
				{/if}
				{#if showDocActions}
					<DocShareActions {rawPath} {rawUrl} {githubUrl} />
				{/if}
			</div>
		</aside>
	</div>
</main>
