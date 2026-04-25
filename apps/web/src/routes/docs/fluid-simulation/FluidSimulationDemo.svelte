<script lang="ts">
	import { FluidSimulation } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof FluidSimulation>>;

	let {
		dissipation = 0.96,
		pointerSize = 0.005,
		color = "#ff6900",
		velocityDissipation = 0.96,
		pressureIterations = 10,
	}: Props = $props();
</script>

<FluidSimulation
	class="h-full min-h-96 w-full"
	{dissipation}
	{pointerSize}
	{color}
	{velocityDissipation}
	{pressureIterations}
/>
