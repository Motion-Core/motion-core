<script lang="ts">
	import { Card3D } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof Card3D>>;

	let {
		width = 3.2,
		height = 2,
		depth = 0.08,
		radius = 0.25,
		showPreview = true,
	}: Props = $props();
</script>

<Card3D
	class="h-full min-h-96 w-full"
	image="/images/demos/sample-2.jpg"
	{width}
	{height}
	{depth}
	{radius}
	{showPreview}
/>
