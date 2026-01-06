<script lang="ts">
	import motionCoreLogo from "$lib/assets/motion-core-logo.svg?raw";
	import HeroBackground from "./HeroBackground.svelte";

	const GITHUB_URL = "https://github.com/motion-core/motion-core";
	const DOCS_URL = "/docs/introduction";
</script>

<div
	class="relative flex flex-col items-center justify-center overflow-hidden rounded-2xl border border-border bg-background px-4 py-24 text-center"
>
	<HeroBackground />

	<div
		class="relative z-10 mb-4 flex items-center justify-center gap-2 text-accent [&>svg]:h-8 [&>svg]:w-20 [&>svg]:fill-current"
	>
		{@html motionCoreLogo}
	</div>

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
			class="inline-flex h-9 shrink-0 items-center justify-center gap-2 rounded-lg bg-accent px-4 py-2 text-sm font-medium whitespace-nowrap shadow-sm transition-[background-color] duration-150 ease-out hover:bg-accent/80 disabled:pointer-events-none disabled:opacity-50 has-[>svg]:px-3 dark:text-foreground light:text-card [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
		>
			Documentation
		</a>
		<a
			href={GITHUB_URL}
			target="_blank"
			rel="noreferrer"
			class="inline-flex h-9 shrink-0 items-center justify-center gap-2 rounded-lg border border-border bg-card px-4 py-2 text-sm font-medium whitespace-nowrap text-foreground shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted disabled:pointer-events-none disabled:opacity-50 has-[>svg]:px-3 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
		>
			GitHub <span aria-hidden="true">→</span>
		</a>
	</div>
</div>
