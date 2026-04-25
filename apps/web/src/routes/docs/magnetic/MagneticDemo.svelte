<script lang="ts">
	import { Magnetic } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof Magnetic>>;

	let { duration = 1, ease = "elastic.out(1, 0.3)" }: Props = $props();
</script>

<Magnetic {duration} {ease}>
	<button
		class="h-8 gap-1.5 rounded-sm bg-background-muted px-3 text-xs font-medium tracking-wide text-foreground uppercase card"
	>
		Magnetic Button
	</button>
</Magnetic>
