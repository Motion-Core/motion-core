<script lang="ts">
	import { WeightWave } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof WeightWave>>;

	let {
		baseWeight = 200,
		hoverWeight = 800,
		influenceRadius = 3,
		falloffPower = 1.5,
		duration = 1,
	}: Props = $props();
</script>

<WeightWave
	class="font-variable text-6xl md:text-8xl"
	{baseWeight}
	{hoverWeight}
	{influenceRadius}
	{falloffPower}
	{duration}
>
	Motion Core
</WeightWave>
