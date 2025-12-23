<script lang="ts">
	import { ImageTrail } from "motion-core";

	const demoImages = [
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_1.jpg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_10.jpg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_3.jpg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_4.jpg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_5.jpg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_7.jpg",
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_8.jpg",
	];
</script>

<ImageTrail
	images={demoImages}
	class="absolute inset-0 h-full w-full flex items-center justify-center"
/>
