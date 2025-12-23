<script lang="ts">
	import { InfiniteGallery } from "motion-core";

	const images = [
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img1.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img2.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img3.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img4.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img5.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img6.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img7.jpeg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img8.jpeg",
	];
</script>

<InfiniteGallery
	class="h-full min-h-80 w-full"
	{images}
	speed={1.5}
	visibleCount={10}
/>
