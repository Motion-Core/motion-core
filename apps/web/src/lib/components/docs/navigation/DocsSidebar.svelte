<script lang="ts">
	import { page } from "$app/state";
	import { slide } from "svelte/transition";
	import { docsNavigation } from "$lib/config/navigation";
	import { brandingConfig } from "$lib/config/branding";
	import { docsUiConfig } from "$lib/config/docs-ui";
	import { siteConfig } from "$lib/config/site";
	import { cn } from "$lib/utils/cn";
	import SearchTrigger from "../search/SearchTrigger.svelte";
	import ScrollArea from "$lib/components/ui/ScrollArea.svelte";
	import ThemeToggle from "$lib/components/ui/ThemeToggle.svelte";
	import ChevronRight from "carbon-icons-svelte/lib/ChevronRight.svelte";
	import LogoGithub from "carbon-icons-svelte/lib/LogoGithub.svelte";

	const currentPath = $derived(
		page.url.pathname.length > 1
			? page.url.pathname.replace(/\/+$/, "")
			: page.url.pathname,
	);
	const githubUrl = siteConfig.links.github;

	let expandedGroups = $state<Record<string, boolean>>({});

	const docHref = (slug: string) => (slug ? `/docs/${slug}` : "/docs");

	function toggleGroup(slug: string) {
		expandedGroups[slug] = !expandedGroups[slug];
	}

	$effect(() => {
		const allDocs = [...docsNavigation];
		for (const doc of allDocs) {
			if (doc.items?.length) {
				const isChildActive = doc.items.some(
					(item) => docHref(item.slug) === currentPath,
				);
				if (isChildActive && expandedGroups[doc.slug] === undefined) {
					expandedGroups[doc.slug] = true;
				}
			}
		}
	});
</script>

<aside class="flex h-dvh flex-col bg-background">
	<div class="flex flex-col gap-8 p-4">
		<a href="/" class="flex items-center gap-2">
			<span
				class="inline-flex shrink-0 items-center text-accent [&>svg]:size-6 [&>svg]:fill-current"
				aria-hidden="true"
			>
				{@html brandingConfig.logoRaw}
			</span>
			<span class="text-xl font-medium tracking-tight text-foreground"
				>{brandingConfig.name}</span
			>
		</a>

		{#if docsUiConfig.search.enabled}
			<SearchTrigger />
		{/if}
	</div>

	<ScrollArea
		class="flex-1"
		viewportClass="p-4"
		viewportStyle="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
	>
		<nav class="flex flex-col space-y-1">
			<h4
				class="mb-2 ml-2 text-xs font-medium tracking-wide text-foreground/45 uppercase"
			>
				{docsUiConfig.sidebar.navigationLabel}
			</h4>
			{#each docsNavigation as doc (doc.slug)}
				{#if doc.items?.length}
					{@const isGroupActive =
						expandedGroups[doc.slug] ??
						doc.items.some((item) => docHref(item.slug) === currentPath)}
					<button
						onclick={() => toggleGroup(doc.slug)}
						class={cn(
							"flex w-full items-center justify-between rounded-sm px-3 py-1.5 text-sm font-medium tracking-normal transition-all duration-150 ease-out hover:bg-background-muted hover:text-foreground",
							isGroupActive ? "text-foreground" : "text-foreground-muted",
						)}
					>
						<span>{doc.name}</span>
						<ChevronRight
							class={cn(
								"size-4 transition-transform duration-150",
								isGroupActive && "rotate-90",
							)}
						/>
					</button>
					{#if isGroupActive}
						<div
							transition:slide={{ duration: 220 }}
							class="relative flex flex-col gap-1 overflow-hidden pl-5 before:absolute before:top-1 before:bottom-1 before:left-3 before:w-px before:bg-border"
						>
							{#each doc.items as item (item.slug)}
								{@const href = docHref(item.slug)}
								{@const isActive = currentPath === href}
								<a
									{href}
									class={cn(
										"block rounded-sm px-3 py-1.5 text-sm font-medium tracking-normal transition-all duration-150 ease-out",
										isActive
											? "bg-accent/10 text-accent"
											: "text-foreground-muted hover:bg-background-muted hover:text-foreground",
									)}
								>
									{item.name}
								</a>
							{/each}
						</div>
					{/if}
				{:else}
					{@const href = docHref(doc.slug)}
					{@const isActive = currentPath === href}
					<a
						{href}
						class={cn(
							"block rounded-sm px-3 py-1.5 text-sm tracking-normal transition-all duration-150 ease-out",
							isActive
								? "bg-accent/10 text-accent"
								: "text-foreground-muted hover:bg-background-muted hover:text-foreground",
						)}
					>
						{doc.name}
					</a>
				{/if}
			{/each}
		</nav>
	</ScrollArea>

	<div class="flex items-center gap-1 p-4">
		{#if docsUiConfig.sidebar.showThemeToggle}
			<ThemeToggle />
		{/if}
		{#if docsUiConfig.sidebar.showRepositoryLink}
			<a
				class="group transition-scale inset-shadow relative inline-flex size-7 items-center justify-center rounded-sm border border-border bg-background-inset text-foreground duration-150 ease-out active:scale-[0.95]"
				href={githubUrl}
				target="_blank"
				rel="noreferrer"
				aria-label={docsUiConfig.sidebar.repositoryAriaLabel}
			>
				<LogoGithub class="size-4 flex-none" />
			</a>
		{/if}
	</div>
</aside>
