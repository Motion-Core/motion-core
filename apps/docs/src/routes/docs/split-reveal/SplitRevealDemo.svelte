<script lang="ts">
	import { SplitReveal } from "motion-core";
	import { cn } from "$lib/utils/cn";

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
		<SplitReveal
			triggerOnScroll
			scrolleElement="#docs-content-container"
			mode={activeMode}
			class="block max-w-3xl text-center text-xl"
		>
			We’re using GSAP’s SplitText to break this content into lines, words, and
			individual characters. Experiment with staggered tweens, custom ease
			functions, and dynamic transforms to bring your headlines to life.
		</SplitReveal>
	{/key}
	<div
		class="absolute bottom-4 left-1/2 flex w-fit -translate-x-1/2 justify-center gap-1 rounded-lg border border-border bg-background p-1 shadow-sm"
	>
		{#each modes as mode (mode)}
			<button
				type="button"
				class={cn(
					"gap-1.5 rounded-md px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-colors duration-150 ease-out",
					mode === activeMode
						? "bg-accent dark:text-foreground light:text-card"
						: "text-foreground/70 hover:text-foreground",
				)}
				onclick={() => handleModeChange(mode)}
			>
				{mode}
			</button>
		{/each}
	</div>
</div>
