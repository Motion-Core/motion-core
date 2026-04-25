<script lang="ts">
	import { FluidImageReveal } from "motion-core";
	import type { ComponentProps } from "svelte";

	const baseImage = "/images/demos/night.png";
	const revealImage = "/images/demos/day.png";
	type Props = Partial<ComponentProps<typeof FluidImageReveal>>;

	let {
		dissipation = 0.96,
		pointerSize = 0.005,
		velocityDissipation = 0.96,
		pressureIterations = 10,
		blendSoftness = 0.22,
	}: Props = $props();
</script>

<FluidImageReveal
	{baseImage}
	{revealImage}
	class="h-full min-h-96 w-full"
	{dissipation}
	{pointerSize}
	{velocityDissipation}
	{pressureIterations}
	{blendSoftness}
/>
