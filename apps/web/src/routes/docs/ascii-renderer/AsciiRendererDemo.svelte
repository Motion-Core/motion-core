<script lang="ts">
	import { AsciiRenderer } from "motion-core";
	import type { ComponentProps } from "svelte";

	const demoImage = "/images/demos/sample-11.jpg";
	type Props = Partial<ComponentProps<typeof AsciiRenderer>>;

	let {
		density = 25,
		strength = 3,
		color = "#00ff00",
		backgroundColor = "#17181A",
	}: Props = $props();
</script>

<AsciiRenderer
	src={demoImage}
	class="h-full min-h-96 w-full"
	{density}
	{strength}
	{color}
	{backgroundColor}
/>
