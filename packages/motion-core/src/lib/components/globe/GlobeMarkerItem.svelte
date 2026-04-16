<script lang="ts">
	import type { Snippet } from "svelte";
	import type { GlobeMarker, GlobeMarkerTooltipContext } from "./types";

	interface Props {
		/**
		 * The marker data object containing location, color, size, etc.
		 */
		marker: GlobeMarker;
		/**
		 * Marker index in the markers array.
		 */
		index: number;
		/**
		 * Horizontal marker position in normalized [0, 1] viewport space.
		 */
		screenX: number;
		/**
		 * Vertical marker position in normalized [0, 1] viewport space.
		 */
		screenY: number;
		/**
		 * Marker visibility factor in range [0, 1].
		 */
		visibility: number;
		/**
		 * Optional custom tooltip snippet.
		 */
		tooltip?: Snippet<[GlobeMarkerTooltipContext]>;
	}

	let { marker, index, screenX, screenY, visibility, tooltip }: Props =
		$props();

	const MAX_TOOLTIP_BLUR = 8;

	let tooltipBlur = $derived((1 - visibility) * MAX_TOOLTIP_BLUR);
	let tooltipContext = $derived<GlobeMarkerTooltipContext>({
		marker,
		index,
		visibility,
	});
</script>

<div
	class="pointer-events-none absolute"
	style:left={`${screenX * 100}%`}
	style:top={`${screenY * 100}%`}
	style:transform="translate(-50%, -50%)"
>
	{#if tooltip || marker.label}
		<div
			class="pointer-events-none absolute top-0 left-1/2 inline-flex -translate-x-1/2 -translate-y-8 flex-col items-center transition-[opacity,filter] duration-200 ease-out will-change-transform"
			style:opacity={visibility}
			style:filter={`blur(${tooltipBlur}px)`}
		>
			{#if tooltip}
				{@render tooltip(tooltipContext)}
			{:else}
				<div
					class="bg-fixed-dark/80 rounded-xs px-2 py-1 text-xs whitespace-nowrap text-fixed-light backdrop-blur-sm"
				>
					{marker.label}
				</div>
			{/if}
		</div>
	{/if}
</div>
