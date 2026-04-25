<script lang="ts">
	import { DitheredImage } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof DitheredImage>>;

	let {
		ditherMap = "halftone",
		pixelSize = 1,
		threshold = 0.2,
		color = "#ff6900",
		backgroundColor = "#17181A",
	}: Props = $props();
</script>

<DitheredImage
	src="/images/demos/sample-6.jpg"
	class="h-full min-h-96 w-full"
	{ditherMap}
	{pixelSize}
	{threshold}
	{color}
	{backgroundColor}
/>
