<script lang="ts">
	import { FluidImageReveal } from "motion-core";

	const baseImage = "/images/demos/night.png";
	const revealImage = "/images/demos/day.png";
</script>

<FluidImageReveal
	{baseImage}
	{revealImage}
	dissipation={0.97}
	pointerSize={0.007}
	blendSoftness={0.3}
	class="h-full min-h-96 w-full"
/>
