<script lang="ts">
	import { WaterRipple } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof WaterRipple>>;

	let { brushSize = 100 }: Props = $props();
</script>

<WaterRipple
	src="/images/demos/sample-15.jpg"
	class="h-full min-h-96 w-full"
	{brushSize}
/>
