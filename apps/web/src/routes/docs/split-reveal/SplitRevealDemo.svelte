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

{#key `${activeMode}-${replayKey}`}
	<SplitReveal
		mode={activeMode}
		class="absolute top-1/2 left-1/2 block w-sm -translate-x-1/2 -translate-y-1/2 p-8 text-center text-lg md:w-3xl"
	>
		We’re using GSAP’s SplitText to break this content into lines, words, and
		individual characters. Experiment with staggered tweens, custom ease
		functions, and dynamic transforms to bring your headlines to life.
	</SplitReveal>
{/key}
<div
	class="inset-shadow absolute bottom-4 left-1/2 z-10 flex w-fit -translate-x-1/2 justify-center gap-1 rounded-sm border border-border bg-background-inset p-1"
>
	{#each modes as mode (mode)}
		<button
			type="button"
			class={cn(
				"gap-1.5 rounded px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-all duration-150 ease-out",
				mode === activeMode
					? "light:text-card bg-background shadow-md dark:text-foreground"
					: "text-foreground-muted hover:text-foreground",
			)}
			onclick={() => handleModeChange(mode)}
		>
			{mode}
		</button>
	{/each}
</div>
