<script lang="ts">
	import { PlasmaGrid } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof PlasmaGrid>>;

	let { color = "#17181A", highlightColor = "#572400" }: Props = $props();
</script>

<PlasmaGrid
	class="h-full min-h-96 w-full bg-[#17181A]"
	{color}
	{highlightColor}
/>
