<script lang="ts">
	import { TextScramble } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof TextScramble>>;

	let hoverTarget = $state<HTMLElement | null>(null);
	let {
		scrambleDuration = 0.6,
		stagger = 0.03,
		cycles = 12,
		characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*",
	}: Props = $props();
</script>

<button
	bind:this={hoverTarget}
	class="h-8 gap-1.5 rounded-sm bg-background-muted px-3 text-xs font-medium tracking-wide text-foreground uppercase card"
>
	<TextScramble
		class="font-mono"
		{hoverTarget}
		{scrambleDuration}
		{stagger}
		{cycles}
		{characters}>Hover Me</TextScramble
	>
</button>
