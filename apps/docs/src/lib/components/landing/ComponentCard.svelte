<script lang="ts">
	import type { ComponentInfo } from "./types";

	let { component }: { component: ComponentInfo } = $props();
</script>

<a
	href={`/docs/${component.slug}`}
	class="group group relative block break-inside-avoid rounded-xl border border-border bg-card p-1 shadow-sm transition-[background-color] duration-150 ease-out hover:border-transparent hover:bg-accent"
	data-component-card
>
	<div
		class="relative aspect-video overflow-hidden rounded-lg border border-border/60 transition-[border] duration-150 ease-out group-hover:border-transparent"
	>
		{#if component.video}
			<video
				src={component.video}
				poster={component.poster}
				autoplay
				loop
				muted
				playsinline
				class="h-full w-full scale-110 object-cover"
			>
			</video>
		{/if}
	</div>
	<div
		class="pointer-events-none m-2 text-sm font-medium text-foreground transition-[color,border] duration-150 ease-out font-display group-hover:text-card"
	>
		{component.name}
	</div>
</a>
