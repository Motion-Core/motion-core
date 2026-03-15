<script lang="ts">
	import { ImageTrail } from "motion-core";

	const demoImages = [
		"/images/demos/sample-1.jpg",
		"/images/demos/sample-2.jpg",
		"/images/demos/sample-3.jpg",
		"/images/demos/sample-4.jpg",
		"/images/demos/sample-5.jpg",
		"/images/demos/sample-6.jpg",
		"/images/demos/sample-7.jpg",
	];
</script>

<ImageTrail
	images={demoImages}
	class="absolute inset-0 flex h-full w-full items-center justify-center"
/>
