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

	const currentPath = $derived(page.url.pathname);
</script>

<aside class="flex h-[calc(100svh)] flex-col bg-card">
	<div class="flex flex-1 flex-col gap-8 overflow-y-auto p-4">
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
						"block rounded-md px-3 py-1.5 text-sm transition-all duration-150 ease-out",
						isActive
							? "bg-accent/10 font-medium text-accent"
							: "text-foreground/70 hover:bg-card-muted/60 hover:text-foreground",
					)}
				>
					{doc.name}
				</a>
			{/each}

			<h4
				class="mt-6 mb-2 ml-2 text-xs font-medium tracking-wider text-foreground/45 uppercase"
			>
				Components
			</h4>
			{#each componentManifest as doc (doc.slug)}
				{@const href = `/docs/${doc.slug}`}
				{@const isActive = currentPath === href}
				<a
					{href}
					class={cn(
						"block rounded-md px-3 py-1.5 text-sm transition-all duration-150 ease-out",
						isActive
							? "bg-accent/10 font-medium text-accent"
							: "text-foreground/70 hover:bg-card-muted/60 hover:text-foreground",
					)}
				>
					{doc.name}
				</a>
			{/each}
		</nav>
	</div>

	<div class="flex justify-end p-4">
		<ThemeToggle />
	</div>
</aside>
