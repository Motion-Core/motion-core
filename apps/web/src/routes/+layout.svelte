<script lang="ts">
	import "./layout.css";
	import { page } from "$app/state";
	import { CommandPalette, docsUiConfig, siteConfig } from "$lib";

	const { children } = $props();

	const currentPage = page;

	const isHomePath = (path?: string) => path === "/";
	const isDocsPath = (path?: string) => path?.startsWith("/docs");

	const currentUrl = $derived(currentPage.url);
	const currentPath = $derived(currentUrl.pathname);
	const isHomeRoute = $derived(isHomePath(currentPath));
	const isDocsRoute = $derived(isDocsPath(currentPath));
	const siteOrigin = new URL(siteConfig.url).origin;
	const canonicalUrl = $derived(new URL(currentPath || "/", siteOrigin).href);

	const siteName = siteConfig.name;
	const authorName = siteConfig.author;
	const homeTitle = `${siteConfig.name} — ${siteConfig.description.split(".")[0]}`;
	const homeDescription = siteConfig.description;
	const homeKeywords = siteConfig.keywords.join(", ");
	const sharedOgImage = $derived(new URL(siteConfig.ogImage, siteOrigin).href);
	const homeStructuredData = $derived.by(() =>
		JSON.stringify({
			"@context": "https://schema.org",
			"@type": "SoftwareApplication",
			name: siteName,
			alternateName: siteConfig.shortName,
			url: canonicalUrl,
			applicationCategory: "DeveloperApplication",
			operatingSystem: "Any",
			description: homeDescription,
			image: sharedOgImage,
			offers: {
				"@type": "Offer",
				price: "0",
				priceCurrency: "USD",
			},
			provider: {
				"@type": "Person",
				name: authorName,
			},
		}),
	);
</script>

<svelte:head>
	<meta name="theme-color" content="#ffffff" />
	<meta property="og:site_name" content={siteName} />
	<meta property="og:locale" content="en_US" />
	<meta name="twitter:card" content="summary_large_image" />
	<link rel="icon" type="image/svg+xml" href="/favicon.svg" />
	<link rel="icon" type="image/png" sizes="96x96" href="/favicon-96x96.png" />
	<link rel="icon" type="image/x-icon" href="/favicon.ico" />
	<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
	<link rel="manifest" href="/site.webmanifest" />
	<link rel="mask-icon" href="/favicon.svg" color="#1f2125" />
	<meta name="mobile-web-app-capable" content="yes" />
	<meta
		name="apple-mobile-web-app-status-bar-style"
		content="black-translucent"
	/>
	<meta name="apple-mobile-web-app-title" content={siteName} />
	{#if isHomeRoute}
		<title>{homeTitle}</title>
		<meta name="description" content={homeDescription} />
		<meta name="keywords" content={homeKeywords} />
		<meta name="author" content={authorName} />
		<link rel="canonical" href={canonicalUrl} />
		<meta property="og:title" content={homeTitle} />
		<meta property="og:description" content={homeDescription} />
		<meta property="og:type" content="website" />
		<meta property="og:url" content={canonicalUrl} />
		<meta property="og:image" content={sharedOgImage} />
		<meta property="og:image:alt" content={`${siteName} logomark`} />
		<meta property="og:image:type" content="image/png" />
		<meta name="twitter:title" content={homeTitle} />
		<meta name="twitter:description" content={homeDescription} />
		<meta name="twitter:image" content={sharedOgImage} />
		<svelte:element this={"script"} type="application/ld+json">
			{homeStructuredData}
		</svelte:element>
	{:else if !isDocsRoute}
		<title>{siteName}</title>
		<meta name="description" content={homeDescription} />
		<link rel="canonical" href={canonicalUrl} />
	{/if}
</svelte:head>

{#if docsUiConfig.search.enabled}
	<CommandPalette />
{/if}
{@render children()}
