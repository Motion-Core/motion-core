<script lang="ts">
	import { Slideshow } from "motion-core";

	const images = [
		{
			src: "/images/demos/sample-1.jpg",
			alt: "Alt 1",
		},
		{
			src: "/images/demos/sample-2.jpg",
			alt: "Alt 2",
		},
		{
			src: "/images/demos/sample-3.jpg",
			alt: "Alt 3",
		},
	];
</script>

<Slideshow class="w-full h-full min-h-80" {images} />
