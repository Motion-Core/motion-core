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

<div class="flex w-full h-full p-4 items-center justify-center">
	{#key `${activeMode}-${replayKey}`}
		<SplitReveal mode={activeMode} class="block text-xl text-center max-w-3xl">
			We’re using GSAP’s SplitText to break this content into lines, words, and
			individual characters. Experiment with staggered tweens, custom ease
			functions, and dynamic transforms to bring your headlines to life.
		</SplitReveal>
	{/key}
	<div
		class="absolute left-1/2 -translate-x-1/2 w-full bottom-4 flex justify-center flex-wrap gap-2"
	>
		{#each modes as mode (mode)}
			<button
				type="button"
				class={`border bg-card h-8 rounded-md gap-1.5 px-3 shadow-sm font-medium text-xs uppercase tracking-wide ${
					mode === activeMode
						? "border-foreground text-foreground"
						: "border-border text-foreground/70 hover:text-foreground"
				}`}
				onclick={() => handleModeChange(mode)}
			>
				{mode}
			</button>
		{/each}
	</div>
</div>
