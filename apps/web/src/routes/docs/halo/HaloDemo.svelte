<script lang="ts">
	import { Halo } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof Halo>>;

	let {
		rotationSpeed = 0.5,
		backgroundColor = "#17181A",
		cameraDistance = 3,
		fov = 55,
		intensity = 2.2,
	}: Props = $props();
</script>

<Halo
	class="h-full min-h-96 w-full"
	{rotationSpeed}
	{backgroundColor}
	{cameraDistance}
	{fov}
	{intensity}
/>
