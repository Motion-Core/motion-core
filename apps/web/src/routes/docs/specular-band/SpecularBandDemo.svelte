<script lang="ts">
	import { SpecularBand } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof SpecularBand>>;

	let {
		color = "#FF6900",
		backgroundColor = "#17181A",
		speed = 0.8,
		distortion = 0.2,
		hueShift = 30,
		intensity = 1,
	}: Props = $props();
</script>

<SpecularBand
	class="h-full min-h-96 w-full"
	{color}
	{backgroundColor}
	{speed}
	{distortion}
	{hueShift}
	{intensity}
/>
