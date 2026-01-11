<script lang="ts">
	import HeroBackground from "./HeroBackground.svelte";

	const GITHUB_URL = "https://github.com/motion-core/motion-core";
	const DOCS_URL = "/docs/introduction";
</script>

<div
	class="card-highlight relative mt-16 flex flex-col items-center justify-center rounded-2xl border border-border bg-background px-4 py-24 text-center md:mt-20"
>
	<HeroBackground />

	<h1
		class="relative z-10 text-3xl font-medium tracking-tight text-foreground font-display md:text-5xl"
	>
		High-quality<br />
		<span class="text-accent">motion components</span><br />
		for Svelte.
	</h1>

	<p
		class="relative z-10 mt-4 max-w-2xl text-base leading-relaxed text-foreground/70"
	>
		Animated Svelte component library powered by GSAP and Three.js. <br
			class="hidden md:block"
		/>
		Drop-in solutions for motion design, 3D canvases, and interactive animations.
	</p>

	<div class="relative z-10 mt-8 flex items-center justify-center gap-x-4">
		<a
			href={DOCS_URL}
			class="transition-[background-color, filter] relative inline-flex h-9 shrink-0 items-center justify-center gap-2 rounded-lg bg-linear-to-b from-accent to-accent-secondary px-4 py-2 text-sm font-medium whitespace-nowrap shadow-[inset_0_1px_0_0_rgb(255_255_255/.32),0px_1px_1px_-0.5px_rgba(9,9,11,0.05),0px_3px_3px_-1.5px_rgba(9,9,11,0.05),0px_6px_6px_-3px_rgba(9,9,11,0.05)] duration-150 ease-out hover:contrast-110 disabled:pointer-events-none disabled:opacity-50 has-[>svg]:px-3 dark:text-foreground dark:shadow-[inset_0_1px_0_0_rgb(255_255_255/.32),0px_1px_1px_-0.5px_rgba(9,9,11,0.05),0px_3px_3px_-1.5px_rgba(9,9,11,0.05),0px_6px_6px_-3px_rgba(9,9,11,0.05)] light:text-card [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
		>
			Documentation
		</a>
		<a
			href={GITHUB_URL}
			target="_blank"
			rel="noreferrer"
			class="card-highlight relative inline-flex h-9 shrink-0 items-center justify-center gap-2 rounded-lg border border-border bg-card px-4 py-2 text-sm font-medium whitespace-nowrap text-foreground shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted disabled:pointer-events-none disabled:opacity-50 has-[>svg]:px-3 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
		>
			GitHub <span aria-hidden="true"
				><svg
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
				</svg></span
			>
		</a>
	</div>
</div>
