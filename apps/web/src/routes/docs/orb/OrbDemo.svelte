<script lang="ts">
	import { Orb, type OrbState } from "motion-core";
	import { cn } from "$lib/utils/cn";

	const states: OrbState[] = ["surge", "attune", "pulse", "idle"];

	let activeState = $state<OrbState>("surge");
	let color = $state<string>("#6933ff");
	let speed = $state<number>(1);
</script>

<Orb state={activeState} {color} {speed} class="h-full min-h-[36rem] w-full" />

<div
	class="absolute bottom-14 left-1/2 z-10 flex w-fit -translate-x-1/2 flex-col items-center gap-3 rounded-sm bg-background-inset px-3 py-2.5 inset-shadow"
>
	<div class="flex gap-1">
		{#each states as s (s)}
			<button
				class={cn(
					"gap-1.5 rounded-xs px-3 py-1 text-sm font-medium tracking-wide whitespace-nowrap uppercase transition-all duration-150 ease-out",
					activeState === s
						? "light:text-card bg-background-muted shadow-md card dark:text-foreground"
						: "text-foreground-muted hover:text-foreground",
				)}
				onclick={() => (activeState = s)}
			>
				{s}
			</button>
		{/each}
	</div>
	<div class="flex items-center gap-4">
		<label class="flex items-center gap-2 text-sm text-foreground-muted">
			Color
			<input
				type="color"
				bind:value={color}
				class="h-6 w-14 cursor-pointer rounded-xs border-0 bg-transparent p-0"
			/>
		</label>
		<label class="flex items-center gap-2 text-sm text-foreground-muted">
			Speed
			<input
				type="range"
				min="0.1"
				max="3"
				step="0.1"
				bind:value={speed}
				class="w-28 accent-foreground"
			/>
			<span class="w-7 text-right tabular-nums">{speed.toFixed(1)}x</span>
		</label>
	</div>
</div>
