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
	{#snippet children(item, i)}
		<div
			class="h-56 w-40 rounded-xl border border-border bg-background p-1 shadow-sm"
		>
			<div
				class="relative h-44 w-full overflow-hidden rounded-lg border border-border/60 bg-card-muted"
			>
				<img src={item} class="h-full w-full object-cover" alt="Gallery item" />
			</div>
			<div class="flex h-12 w-full items-center justify-between px-2">
				<p class="text-xs font-medium text-foreground font-display">
					Motion Core
				</p>
				<p class="text-[10px] text-foreground/45">
					Item {i}
				</p>
			</div>
		</div>
	{/snippet}
</RadialGallery>
