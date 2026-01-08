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
	 * @default "#ff0000"
	 */
	color?: string;
	/**
	 * Optional text label to display on hover.
	 */
	label?: string;
}
