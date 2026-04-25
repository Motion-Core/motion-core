<script lang="ts">
	import { TextLoop } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof TextLoop>>;

	let { interval = 2000 }: Props = $props();
</script>

<p class="text-center text-5xl font-medium text-foreground">
	Create <TextLoop
		class="text-accent"
		texts={["magic", "wonder", "impact", "value"]}
		{interval}
	/> today
</p>
