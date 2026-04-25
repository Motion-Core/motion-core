<script lang="ts">
	import { RadialGallery } from "motion-core";

	type Props = {
		radius?: number;
		duration?: number;
		reversed?: boolean;
		offset?: number;
		gap?: number;
		elementSize?: number;
	};

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

	let {
		radius = 850,
		duration = 70,
		reversed = false,
		offset = -600,
		gap = 100,
		elementSize = 160,
	}: Props = $props();
</script>

<RadialGallery
	class="h-full min-h-96 w-full"
	items={images}
	{radius}
	{duration}
	{reversed}
	{offset}
	{gap}
	{elementSize}
>
	{#snippet children(item)}
		<div
			class="relative h-48 w-48 rounded-lg bg-background-inset p-1.5 inset-shadow group-data-[fullscreen=true]/preview:h-96 group-data-[fullscreen=true]/preview:w-96"
		>
			<div class="h-full overflow-hidden rounded-md bg-background card">
				<img src={item} class="h-full w-full object-cover" alt="Gallery item" />
			</div>
		</div>
	{/snippet}
</RadialGallery>
