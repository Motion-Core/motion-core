<script lang="ts">
	import { Fake3DImage } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof Fake3DImage>>;

	let { xThreshold = 8, yThreshold = 8, sensitivity = 0.25 }: Props = $props();
</script>

<Fake3DImage
	colorSrc="/images/demos/sample-16.jpg"
	depthSrc="/images/demos/sample-16-depth-map.webp"
	class="h-full min-h-96 w-full"
	{xThreshold}
	{yThreshold}
	{sensitivity}
/>
