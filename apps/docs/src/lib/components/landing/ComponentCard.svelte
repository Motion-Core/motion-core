<script lang="ts">
	import type { ComponentInfo } from "./types";

	let { component }: { component: ComponentInfo } = $props();
</script>

<a
	href={`/docs/${component.slug}`}
	class="relative block break-inside-avoid rounded-xl border border-border bg-card shadow-sm p-1 group"
	data-component-card
>
	<div
		class="relative rounded-lg overflow-hidden aspect-video border border-border/60"
	>
		{#if component.video}
			<video
				src={component.video}
				poster={component.poster}
				autoplay
				loop
				muted
				playsinline
				class="h-full w-full object-cover scale-110"
			>
			</video>
		{/if}
	</div>
	<div class="pointer-events-none m-2 text-sm text-foreground">
		{component.name}
	</div>
</a>
