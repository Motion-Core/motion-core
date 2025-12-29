<script lang="ts">
	import { page } from "$app/state";
	import {
		gettingStartedManifest,
		componentManifest,
	} from "$lib/docs/manifest";
	import { cn } from "$lib/utils/cn";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";
	import ThemeToggle from "$lib/components/landing/ThemeToggle.svelte";
	import SearchTrigger from "../search/SearchTrigger.svelte";
	import ScrollArea from "$lib/components/ui/ScrollArea.svelte";

	const currentPath = $derived(page.url.pathname);
	const githubUrl = "https://github.com/motion-core/motion-core";

	const groupedComponents = componentManifest.reduce(
		(acc, component) => {
			const category = component.category || "Components";
			if (!acc[category]) {
				acc[category] = [];
			}
			acc[category].push(component);
			return acc;
		},
		{} as Record<string, typeof componentManifest>,
	);

	const sortedCategories = Object.keys(groupedComponents).sort();
</script>

<aside class="flex h-dvh flex-col bg-card">
	<div class="flex flex-col gap-8 p-4">
		<a href="/" class="flex items-center gap-2">
			<span
				class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
				aria-hidden="true"
			>
				{@html motionCoreLogo}
			</span>
			<span class="text-xl font-medium text-foreground font-display"
				>Motion Core</span
			>
		</a>

		<SearchTrigger />
	</div>

	<ScrollArea
		class="flex-1"
		viewportClass="p-4"
		viewportStyle="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
	>
		<nav class="flex flex-col gap-1">
			<h4
				class="mb-2 ml-2 text-xs font-medium tracking-wider text-foreground/45 uppercase"
			>
				Getting Started
			</h4>
			{#each gettingStartedManifest as doc (doc.slug)}
				{@const href = `/docs/${doc.slug}`}
				{@const isActive = currentPath === href}
				<a
					{href}
					class={cn(
						"block rounded-xl px-3 py-1.5 text-sm transition-all duration-150 ease-out",
						isActive
							? "bg-accent/10 text-accent"
							: "text-foreground/70 hover:bg-card-muted hover:text-foreground",
					)}
				>
					{doc.name}
				</a>
			{/each}

			{#each sortedCategories as category (category)}
				<h4
					class="mt-4 mb-2 ml-2 text-xs font-medium tracking-wider text-foreground/45 uppercase"
				>
					{category}
				</h4>
				{#each groupedComponents[category] as doc (doc.slug)}
					{@const href = `/docs/${doc.slug}`}
					{@const isActive = currentPath === href}
					<a
						{href}
						class={cn(
							"block rounded-xl px-3 py-1.5 text-sm transition-all duration-150 ease-out",
							isActive
								? "bg-accent/10 text-accent"
								: "text-foreground/70 hover:bg-card-muted hover:text-foreground",
						)}
					>
						{doc.name}
					</a>
				{/each}
			{/each}
		</nav>
	</ScrollArea>

	<div class="flex items-center gap-2 p-4">
		<a
			class="inline-flex size-7 items-center justify-center gap-2 rounded-lg text-sm font-medium text-foreground transition-colors duration-150 ease-out hover:bg-card-muted hover:text-foreground"
			href={githubUrl}
			target="_blank"
			rel="noreferrer"
			aria-label="Open Motion Core on GitHub"
		>
			<svg
				role="img"
				viewBox="0 0 24 24"
				fill="currentColor"
				aria-hidden="true"
				class="size-4 flex-none"
			>
				<title>GitHub</title>
				<path
					d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
				/>
			</svg>
		</a>
		<div class="ml-auto">
			<ThemeToggle />
		</div>
	</div>
</aside>
