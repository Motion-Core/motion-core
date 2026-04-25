<script lang="ts">
	import {
		InfinitePhysicsGallery,
		type InfinitePhysicsGalleryItem,
	} from "motion-core";

	type Props = {
		cellWidth?: number;
		cellHeight?: number;
		gap?: number;
		friction?: number;
		wheelSpeed?: number;
		edgeThreshold?: number;
		edgeScrollSpeed?: number;
	};

	const items: InfinitePhysicsGalleryItem[] = [
		{
			id: "img-1",
			type: "image",
			image: {
				src: "/images/demos/sample-1.jpg",
				alt: "Alt 1",
			},
		},
		{
			id: "img-2",
			type: "image",
			image: {
				src: "/images/demos/sample-4.jpg",
				alt: "Alt 2",
			},
		},
		{
			id: "img-3",
			type: "image",
			image: {
				src: "/images/demos/sample-10.jpg",
				alt: "Alt 3",
			},
		},
		{
			id: "img-4",
			type: "image",
			image: {
				src: "/images/demos/sample-12.jpg",
				alt: "Alt 4",
			},
		},
	];

	let {
		cellWidth = 280,
		cellHeight = 360,
		gap = 20,
		friction = 0.95,
		wheelSpeed = 1.4,
		edgeThreshold = 120,
		edgeScrollSpeed = 1.6,
	}: Props = $props();
</script>

<InfinitePhysicsGallery
	class="h-full min-h-160 w-full"
	{items}
	{cellWidth}
	{cellHeight}
	{gap}
	cellClass="rounded-md"
	{friction}
	scrollSettings={{
		wheelSpeed,
		friction,
		edgeThreshold,
		edgeScrollSpeed,
	}}
/>
