<script lang="ts">
	import { page } from "$app/state";
	import { slide } from "svelte/transition";
	import { docsNavigation } from "$lib/config/navigation";
	import { brandingConfig } from "$lib/config/branding";
	import { docsUiConfig } from "$lib/config/docs-ui";
	import { siteConfig } from "$lib/config/site";
	import { isNewComponentDoc } from "$lib/docs/new-badges";
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
	const docsSectionSlugs = new Set(["getting-started", "resources"]);
	const navigationGroups = $derived(
		[
			{
				label: docsUiConfig.sidebar.navigationLabel,
				items: docsNavigation.filter((doc) => docsSectionSlugs.has(doc.slug)),
			},
			{
				label: "Components",
				items: docsNavigation.filter((doc) => !docsSectionSlugs.has(doc.slug)),
			},
		].filter((group) => group.items.length > 0),
	);

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
				class="[&>svg:w-auto inline-flex shrink-0 items-center text-accent [&>svg]:h-6 [&>svg]:fill-current"
				aria-hidden="true"
			>
				{@html brandingConfig.logoRaw}
			</span>
			<span class="pt-0.5 text-xl font-medium tracking-tight text-foreground"
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
			{#each navigationGroups as group, groupIndex (group.label)}
				<h4
					class={cn(
						"mb-2 ml-2 text-xs font-medium tracking-wide text-foreground-muted/70 uppercase",
						groupIndex > 0 && "mt-8",
					)}
				>
					{group.label}
				</h4>
				{#each group.items as doc (doc.slug)}
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
									{@const isNew = isNewComponentDoc(item.slug)}
									<a
										{href}
										class={cn(
											"flex items-center justify-between gap-2 rounded-sm px-3 py-1.5 text-sm font-medium tracking-normal transition-all duration-150 ease-out",
											isActive
												? "bg-accent/10 text-accent"
												: "text-foreground-muted hover:bg-background-muted hover:text-foreground",
										)}
									>
										<span>{item.name}</span>
										{#if isNew}
											<div
												class="relative inline-flex rounded-sm bg-background-inset p-1 text-[10px] font-medium text-foreground inset-shadow"
											>
												<span
													class="rounded-[calc(var(--radius-base)*1.25)] bg-background px-1 py-0.5 card"
												>
													{docsUiConfig.sidebar.newBadge.label}
												</span>
											</div>
										{/if}
									</a>
								{/each}
							</div>
						{/if}
					{:else}
						{@const href = docHref(doc.slug)}
						{@const isActive = currentPath === href}
						{@const isNew = isNewComponentDoc(doc.slug)}
						<a
							{href}
							class={cn(
								"flex items-center justify-between gap-2 rounded-sm px-3 py-1.5 text-sm tracking-normal transition-all duration-150 ease-out",
								isActive
									? "bg-accent/10 text-accent"
									: "text-foreground-muted hover:bg-background-muted hover:text-foreground",
							)}
						>
							<span>{doc.name}</span>
							{#if isNew}
								<div
									class="relative inline-flex w-fit rounded-sm bg-background-inset p-1 text-[10px] font-medium whitespace-nowrap text-foreground inset-shadow"
								>
									<span
										class="bg-backgroundp-1 rounded-[calc(var(--radius-base)*1.25)] card"
									>
										{docsUiConfig.sidebar.newBadge.label}
									</span>
								</div>
							{/if}
						</a>
					{/if}
				{/each}
			{/each}
		</nav>
	</ScrollArea>

	<div class="flex items-center gap-1 p-4">
		{#if docsUiConfig.sidebar.showThemeToggle}
			<ThemeToggle />
		{/if}
		{#if docsUiConfig.sidebar.showRepositoryLink}
			<a
				class="group transition-scale relative inline-flex size-7 cursor-pointer items-center justify-center rounded-sm bg-background-inset text-foreground inset-shadow duration-150 ease-out active:scale-[0.95]"
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
