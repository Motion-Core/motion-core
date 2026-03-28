<script lang="ts">
	import {
		Globe,
		type GlobeMarker,
		type GlobeMarkerTooltipContext,
	} from "motion-core";
	import { cn } from "$lib/utils/cn";

	const locations: (GlobeMarker & { name: string })[] = [
		{
			name: "Warsaw",
			location: [52.2297, 21.0122],
			label: "Warsaw",
			color: "#4fb7ff",
		},
		{
			name: "New York",
			location: [40.7128, -74.006],
			label: "New York",
			color: "#4fb7ff",
		},
		{
			name: "Tokyo",
			location: [35.6762, 139.6503],
			label: "Tokyo",
			color: "#4fb7ff",
		},
	];

	let focusOn = $state<[number, number] | null>(null);
</script>

{#snippet markerTooltip(ctx: GlobeMarkerTooltipContext)}
	<div
		class="pointer-events-none relative rounded-xs bg-foreground px-1.5 py-0.75 font-mono text-[10px] font-medium tracking-wide whitespace-nowrap text-background uppercase"
	>
		{ctx.marker.label}
		<span
			class="absolute top-[calc(100%-1px)] left-1/2 h-0 w-0 -translate-x-1/2 border-x-[5px] border-t-[5px] border-x-transparent border-t-foreground"
		></span>
	</div>
{/snippet}

<Globe
	radius={3}
	pointCount={25000}
	class="h-full min-h-96 w-full"
	markers={locations}
	{markerTooltip}
	{focusOn}
	autoRotate={!focusOn}
	lockedPolarAngle={false}
/>
<div
	class="inset-shadow absolute bottom-4 left-1/2 z-10 flex w-fit -translate-x-1/2 justify-center gap-1 rounded-sm bg-background-inset p-1"
>
	<button
		class={cn(
			"gap-1.5 rounded-xs px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-all duration-150 ease-out",
			focusOn === null
				? "light:text-card card  bg-background-muted shadow-md dark:text-foreground"
				: "text-foreground-muted hover:text-foreground",
		)}
		onclick={() => (focusOn = null)}
	>
		Auto Rotate
	</button>
	{#each locations as loc (loc.name)}
		<button
			class={cn(
				"gap-1.5 rounded-xs px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-colors duration-150 ease-out",
				focusOn?.[0] === loc.location[0] && focusOn?.[1] === loc.location[1]
					? "light:text-card card  bg-background-muted shadow-md dark:text-foreground"
					: "text-foreground-muted hover:text-foreground",
			)}
			onclick={() => (focusOn = loc.location)}
		>
			{loc.name}
		</button>
	{/each}
</div>
