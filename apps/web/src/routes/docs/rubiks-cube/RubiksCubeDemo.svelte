<script lang="ts">
	import { RubiksCube } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof RubiksCube>>;

	let {
		size = 1,
		duration = 1.5,
		gap = 0.015,
		radius = 0.125,
	}: Props = $props();
</script>

<RubiksCube class="h-full min-h-96 w-full" {size} {duration} {gap} {radius} />
