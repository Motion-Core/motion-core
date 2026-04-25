<script lang="ts">
	import { ImageTrail } from "motion-core";

	type Props = {
		mouseThreshold?: number;
		imageLifespan?: number;
		inDuration?: number;
		outDuration?: number;
		baseRotation?: number;
		maxRotationFactor?: number;
		minImageSize?: number;
		maxImageSize?: number;
	};

	const demoImages = [
		"/images/demos/sample-1.jpg",
		"/images/demos/sample-2.jpg",
		"/images/demos/sample-3.jpg",
		"/images/demos/sample-4.jpg",
		"/images/demos/sample-5.jpg",
		"/images/demos/sample-6.jpg",
		"/images/demos/sample-7.jpg",
	];

	let {
		mouseThreshold = 40,
		imageLifespan = 600,
		inDuration = 600,
		outDuration = 800,
		baseRotation = 30,
		maxRotationFactor = 3,
		minImageSize = 260,
		maxImageSize = 340,
	}: Props = $props();
</script>

<ImageTrail
	images={demoImages}
	class="absolute inset-0 flex h-full w-full items-center justify-center"
	config={{
		mouseThreshold,
		imageLifespan,
		inDuration,
		outDuration,
		baseRotation,
		maxRotationFactor,
		minImageSize,
		maxImageSize,
	}}
/>
