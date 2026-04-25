<script lang="ts">
	import { InfiniteGallery } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof InfiniteGallery>>;

	let { speed = 1, visibleCount = 8 }: Props = $props();

	const images = [
		"/images/demos/sample-8.jpg",
		"/images/demos/sample-9.jpg",
		"/images/demos/sample-10.jpg",
		"/images/demos/sample-11.jpg",
		"/images/demos/sample-12.jpg",
		"/images/demos/sample-13.jpg",
		"/images/demos/sample-14.jpg",
		"/images/demos/sample-15.jpg",
	];
</script>

<InfiniteGallery
	class="h-full min-h-96 w-full"
	{images}
	{speed}
	{visibleCount}
/>
