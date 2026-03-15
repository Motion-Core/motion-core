<script lang="ts">
	import { TextScramble } from "motion-core";

	let hoverTarget = $state<HTMLElement | null>(null);
</script>

<button
	bind:this={hoverTarget}
	class="h-8 w-26 gap-1.5 rounded border border-border bg-background px-3 text-xs font-medium tracking-wide text-foreground uppercase shadow-md"
>
	<TextScramble cycles={3} scrambleDuration={0.15} {hoverTarget}
		>Hover Me</TextScramble
	>
</button>
