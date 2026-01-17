<script lang="ts">
	import { RadialGallery } from "motion-core";

	const images = [
		"/images/demos/sample-1.jpg",
		"/images/demos/sample-2.jpg",
		"/images/demos/sample-3.jpg",
		"/images/demos/sample-4.jpg",
		"/images/demos/sample-5.jpg",
		"/images/demos/sample-6.jpg",
		"/images/demos/sample-7.jpg",
		"/images/demos/sample-8.jpg",
	];
</script>

<RadialGallery
	class="h-full min-h-96 w-full"
	items={images}
	radius={850}
	duration={70}
	offset={-600}
	gap={100}
	elementSize={160}
>
	{#snippet children(item)}
		<div
			class="relative h-48 w-48 rounded-xl border border-border bg-card shadow-sm group-data-[fullscreen=true]/preview:h-96 group-data-[fullscreen=true]/preview:w-96"
		>
			<div
				class="absolute inset-1 overflow-hidden rounded-lg border border-border/60 bg-card-muted"
			>
				<img src={item} class="h-full w-full object-cover" alt="Gallery item" />
			</div>
		</div>
	{/snippet}
</RadialGallery>
