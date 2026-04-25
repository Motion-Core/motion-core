<script lang="ts">
	import { GodRays } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof GodRays>>;

	let {
		color = "#FFFFFF",
		intensity = 0.35,
		speed = 1,
		lightSpread = 1,
		rayLength = 1,
		pulsating = false,
	}: Props = $props();
</script>

<GodRays
	class="h-full min-h-96 w-full"
	{color}
	{intensity}
	{speed}
	{lightSpread}
	{rayLength}
	{pulsating}
/>
