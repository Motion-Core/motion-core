<script lang="ts">
	import { FluidImageReveal } from "motion-core";

	const baseImage = "/images/demos/night.png";
	const revealImage = "/images/demos/day.png";
</script>

<FluidImageReveal {baseImage} {revealImage} class="h-full min-h-96 w-full" />
