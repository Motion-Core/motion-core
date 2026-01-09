<script lang="ts">
	import { Globe, type GlobeMarker } from "motion-core";
	import { cn } from "$lib/utils/cn";

	const locations: (GlobeMarker & { name: string })[] = [
		{
			name: "Warsaw",
			location: [52.2297, 21.0122],
			label: "Warsaw",
			size: 0.1,
			color: "#00c2a8",
		},
		{
			name: "New York",
			location: [40.7128, -74.006],
			label: "New York",
			size: 0.1,
			color: "#ff2376",
		},
		{
			name: "Tokyo",
			location: [35.6762, 139.6503],
			label: "Tokyo",
			size: 0.1,
			color: "#e6f0ff",
		},
	];

	let focusOn = $state<[number, number] | null>(null);
</script>

<Globe
	radius={3}
	pointCount={25000}
	class="h-full min-h-96 w-full"
	markers={locations}
	{focusOn}
	autoRotate={!focusOn}
	lockedPolarAngle={false}
/>
<div
	class="absolute bottom-4 left-1/2 z-10 flex w-fit -translate-x-1/2 flex-wrap justify-center gap-1 rounded-lg border border-border bg-background p-1 shadow-sm"
>
	<button
		class={cn(
			"gap-1.5 rounded-md px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-colors duration-150 ease-out",
			focusOn === null
				? "bg-accent dark:text-foreground light:text-card"
				: "text-foreground/70 hover:text-foreground",
		)}
		onclick={() => (focusOn = null)}
	>
		Auto Rotate
	</button>
	{#each locations as loc (loc.name)}
		<button
			class={cn(
				"gap-1.5 rounded-md px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-colors duration-150 ease-out",
				focusOn?.[0] === loc.location[0] && focusOn?.[1] === loc.location[1]
					? "bg-accent dark:text-foreground light:text-card"
					: "text-foreground/70 hover:text-foreground",
			)}
			onclick={() => (focusOn = loc.location)}
		>
			{loc.name}
		</button>
	{/each}
</div>
