<script lang="ts">
	import { Slideshow } from "motion-core";

	const images = [
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_1.jpg",
			alt: "Alt 1",
		},
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_10.jpg",
			alt: "Alt 2",
		},
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_3.jpg",
			alt: "Alt 3",
		},
	];
</script>

<Slideshow class="w-full h-full min-h-80" {images} />
