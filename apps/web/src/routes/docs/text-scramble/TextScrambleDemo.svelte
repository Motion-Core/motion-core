<script lang="ts">
	import { TextScramble } from "motion-core";

	let hoverTarget = $state<HTMLElement | null>(null);
</script>

<button
	bind:this={hoverTarget}
	class="card h-8 gap-1.5 rounded-sm bg-background-muted px-3 text-xs font-medium tracking-wide text-foreground uppercase"
>
	<TextScramble class="font-mono" {hoverTarget}>Hover Me</TextScramble>
</button>
