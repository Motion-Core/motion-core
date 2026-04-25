<script lang="ts">
	import { GlassPane } from "motion-core";
	import type { ComponentProps } from "svelte";

	const demoImage = "/images/demos/sample-6.jpg";
	type Props = Partial<ComponentProps<typeof GlassPane>>;

	let {
		distortion = 0.5,
		chromaticAberration = 0.005,
		waviness = 0,
		speed = 0.5,
		rods = 3,
	}: Props = $props();
</script>

<GlassPane
	image={demoImage}
	class="h-full min-h-96 w-full"
	{distortion}
	{chromaticAberration}
	{waviness}
	{speed}
	{rods}
/>
