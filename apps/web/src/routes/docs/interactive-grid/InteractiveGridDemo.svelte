<script lang="ts">
	import { InteractiveGrid } from "motion-core";
	import type { ComponentProps } from "svelte";

	const demoImage = "/images/demos/sample-14.jpg";
	type Props = Partial<ComponentProps<typeof InteractiveGrid>>;

	let {
		grid = 15,
		mouseSize = 0.15,
		strength = 0.35,
		relaxation = 0.9,
	}: Props = $props();
</script>

<InteractiveGrid
	image={demoImage}
	class="h-full min-h-96 w-full"
	{grid}
	{mouseSize}
	{strength}
	{relaxation}
/>
