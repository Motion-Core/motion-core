<script lang="ts">
	import { SplitReveal } from "motion-core";

	type SplitMode = "lines" | "words" | "chars";

	const modes: SplitMode[] = ["lines", "words", "chars"];
	let activeMode: SplitMode = $state("lines");
	let replayKey = $state(0);

	const handleModeChange = (value: SplitMode) => {
		activeMode = value;
		replayKey += 1;
	};
</script>

<div class="flex h-full w-full items-center justify-center p-4">
	{#key `${activeMode}-${replayKey}`}
		<SplitReveal mode={activeMode} class="block max-w-3xl text-center text-xl">
			We’re using GSAP’s SplitText to break this content into lines, words, and
			individual characters. Experiment with staggered tweens, custom ease
			functions, and dynamic transforms to bring your headlines to life.
		</SplitReveal>
	{/key}
	<div
		class="absolute bottom-4 left-1/2 flex w-full -translate-x-1/2 flex-wrap justify-center gap-2"
	>
		{#each modes as mode (mode)}
			<button
				type="button"
				class={`h-8 gap-1.5 rounded-md border bg-card px-3 text-xs font-medium tracking-wide uppercase shadow-sm ${
					mode === activeMode
						? "border-accent text-foreground"
						: "border-border text-foreground/70 hover:text-foreground"
				}`}
				onclick={() => handleModeChange(mode)}
			>
				{mode}
			</button>
		{/each}
	</div>
</div>
