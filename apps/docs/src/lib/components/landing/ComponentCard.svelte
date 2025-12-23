<script lang="ts">
	import type { ComponentInfo } from "./types";

	let { component }: { component: ComponentInfo } = $props();
</script>

<a
	href={`/docs/${component.slug}`}
	class="relative block break-inside-avoid rounded-xl border border-border bg-card shadow-sm group"
	data-component-card
>
	<div class="relative rounded-xl overflow-hidden aspect-video">
		{#if component.video}
			<video
				src={component.video}
				poster={component.poster}
				autoplay
				loop
				muted
				playsinline
				class="h-full w-full object-cover scale-105"
			>
			</video>
		{/if}
		<div
			class="pointer-events-none absolute bottom-4 left-4 text-xs font-mono uppercase tracking-wide text-white mix-blend-difference"
		>
			{component.name}
		</div>
	</div>
</a>
