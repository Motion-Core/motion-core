<script lang="ts">
	import { GlitterCloth } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof GlitterCloth>>;

	let {
		color = "#222326",
		speed = 1,
		brightness = 1,
		blendStrength = 0.02,
		noiseScale = 4,
		vignetteStrength = 50,
		vignettePower = 0.5,
	}: Props = $props();
</script>

<GlitterCloth
	class="h-full min-h-96 w-full"
	{color}
	{speed}
	{brightness}
	{blendStrength}
	{noiseScale}
	{vignetteStrength}
	{vignettePower}
/>
