<script lang="ts">
	import ThemeToggle from "./ThemeToggle.svelte";
	import type { SocialLink } from "./types";

	export let title = "Motion Core";
	export let description = "";
	export let socialLinks: SocialLink[] = [];
</script>

<aside
	class="flex flex-col justify-between gap-8 lg:sticky lg:top-4 lg:col-span-1 lg:h-[calc(100svh-2rem)] lg:self-start"
>
	<header class="flex items-center gap-2">
		<span class="text-xl font-medium text-foreground">{title}</span>
	</header>
	<div class="space-y-4">
		<p class="text-xs leading-relaxed text-foreground/70 text-balance">
			{description}
		</p>
		<nav class="flex items-center gap-2 text-xs font-medium uppercase tracking-wide font-mono">
			{#each socialLinks as link}
				<a
					class="text-foreground underline-offset-4 hover:underline"
					href={link.href}
				>
					{link.label}
				</a>
			{/each}
			<ThemeToggle />
		</nav>
	</div>
</aside>
