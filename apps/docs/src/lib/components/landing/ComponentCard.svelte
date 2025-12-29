<script lang="ts">
	import type { ComponentInfo } from "./types";

	let { component }: { component: ComponentInfo } = $props();
</script>

<a
	href={`/docs/${component.slug}`}
	class="group relative block break-inside-avoid rounded-2xl border border-border bg-card p-1 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted"
	data-component-card
>
	<div
		class="relative aspect-video overflow-hidden rounded-xl border border-border/60"
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
	<div class="flex items-center justify-between w-full p-2">
		<p
			class="pointer-events-none text-sm font-medium text-foreground font-display"
		>
			{component.name}
		</p>
		<p class="pointer-events-none text-xs font-medium text-foreground/45">
			{component.category}
		</p>
	</div>
</a>
