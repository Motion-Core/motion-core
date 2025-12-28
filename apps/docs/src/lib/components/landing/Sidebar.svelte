<script lang="ts">
	import ThemeToggle from "./ThemeToggle.svelte";
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";
	import type { SocialLink } from "./types";

	const props = $props<{
		title?: string;
		description?: string;
		socialLinks?: SocialLink[];
	}>();
	const DEFAULT_GITHUB_URL = "https://github.com/motion-core/motion-core";

	const title = $derived(props.title ?? "Motion Core");
	const description = $derived(props.description ?? "");
</script>

<aside
	class="flex flex-col justify-between gap-8 lg:sticky lg:top-4 lg:col-span-1 lg:h-[calc(100svh-2rem)] lg:self-start"
>
	<header class="flex items-center gap-2">
		<span
			class="inline-flex shrink-0 items-center text-accent [&>svg]:h-3 [&>svg]:w-8 [&>svg]:fill-current"
			aria-hidden="true"
		>
			{@html motionCoreLogo}
		</span>
		<span class="text-xl font-medium text-foreground font-display">{title}</span
		>
	</header>
	<div class="space-y-4">
		<p
			class="overflow-hidden text-sm leading-relaxed text-balance text-foreground/70"
		>
			{description}
		</p>
		<div class="flex items-center gap-2">
			<a
				class="inline-flex size-7 items-center justify-center gap-2 rounded-lg text-sm font-medium text-foreground transition-colors duration-150 ease-out hover:bg-card-muted hover:text-foreground"
				href={DEFAULT_GITHUB_URL}
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
	</div>
</aside>
