<script lang="ts">
	import { GlassSlideshow } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof GlassSlideshow>>;

	const images = [
		"/images/demos/sample-7.jpg",
		"/images/demos/sample-5.jpg",
		"/images/demos/sample-4.jpg",
	];

	let {
		transitionDuration = 2000,
		intensity = 1,
		distortion = 1,
		chromaticAberration = 1,
		refraction = 1,
		autoplay = true,
		autoplayInterval = 5000,
	}: Props = $props();
</script>

<GlassSlideshow
	class="h-full min-h-96 w-full"
	{images}
	{transitionDuration}
	{intensity}
	{distortion}
	{chromaticAberration}
	{refraction}
	{autoplay}
	{autoplayInterval}
/>
