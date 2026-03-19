<script lang="ts">
	import "./layout.css";
	import { page } from "$app/state";
	import { CommandPalette, docsUiConfig, siteConfig } from "$lib";
	import { FloatingMenu } from "motion-core";
	import { brandingConfig } from "$lib/config/branding";

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

	const menuGroups = [
		{
			title: "Getting Started",
			variant: "muted" as const,
			links: [
				{ label: "Home", href: "/" },
				{ label: "Introduction", href: "/docs/introduction" },
				{ label: "CLI Quick Start", href: "/docs/cli-guide/quick-start" },
			],
		},
		{
			title: "CLI Commands",
			variant: "default" as const,
			links: [
				{ label: "init", href: "/docs/cli-guide/init" },
				{ label: "add", href: "/docs/cli-guide/add" },
				{ label: "list", href: "/docs/cli-guide/list" },
				{ label: "cache", href: "/docs/cli-guide/cache" },
			],
		},
		{
			title: "Resources",
			variant: "muted" as const,
			links: [
				{
					label: "Registry Changelog",
					href: "/docs/changelog/registry",
				},
				{
					label: "CLI Changelog",
					href: "/docs/changelog/cli",
				},
				{
					label: "NPM",
					href: "https://www.npmjs.com/package/@motion-core/cli",
				},
			],
		},
	];
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
{#if isHomeRoute}
	<FloatingMenu
		classes={{
			root: "bg-background-inset/40 dark:bg-background-inset/80 backdrop-blur-xl card border-none",
			groupMuted: "bg-foreground/5 dark:bg-background-muted/40",
			secondaryButton: "hover:bg-foreground/10 dark:hover:bg-foreground/5",
			divider: "border-foreground/5",
			menuWrapper: "border-foreground/5",
		}}
		primaryButton={{ label: "Discord", href: "https://discord.gg/stZ8hqAvpE" }}
		secondaryButton={{
			label: "GitHub",
			href: "https://github.com/motion-core/motion-core",
		}}
		{menuGroups}
	>
		{#snippet logo()}
			<a href="/" class="flex items-center">
				<span
					class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
					aria-hidden="true"
				>
					{@html brandingConfig.logoRaw}
				</span>
			</a>
		{/snippet}
	</FloatingMenu>
{/if}
{@render children()}
