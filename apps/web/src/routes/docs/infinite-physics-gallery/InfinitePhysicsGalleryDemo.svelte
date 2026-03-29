<script lang="ts">
	import {
		InfinitePhysicsGallery,
		type InfinitePhysicsGalleryItem,
	} from "motion-core";

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
</script>

<InfinitePhysicsGallery
	class="h-full min-h-160 w-full"
	{items}
	cellWidth={280}
	cellHeight={360}
	gap={20}
	cellClass="rounded-md"
	friction={0.95}
	scrollSettings={{
		wheelSpeed: 1.4,
		friction: 0.94,
		edgeThreshold: 120,
		edgeScrollSpeed: 1.6,
	}}
/>
