export interface GlobeMarker {
	/**
	 * Latitude and Longitude coordinates [lat, lon].
	 */
	location: [number, number];
	/**
	 * Size of the marker in world units.
	 * @default 0.05
	 */
	size?: number;
	/**
	 * Color of the marker.
	 * @default "#ffffff"
	 */
	color?: string;
	/**
	 * Optional fallback tooltip text (used when no custom tooltip renderer is provided).
	 */
	label?: string;
}

export interface GlobeMarkerTooltipContext {
	/**
	 * Marker currently being rendered.
	 */
	marker: GlobeMarker;
	/**
	 * Marker index in the markers array.
	 */
	index: number;
	/**
	 * Marker visibility factor in range [0, 1].
	 * 0 means fully hidden behind the globe, 1 means fully visible.
	 */
	visibility: number;
}
