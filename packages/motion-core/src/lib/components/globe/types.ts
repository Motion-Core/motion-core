export interface GlobeMarker {
	/**
	 * Latitude and Longitude coordinates [lat, lon].
	 */
	location: [number, number];
	/**
	 * Size of the marker in world units.
	 * @default 0.1
	 */
	size?: number;
	/**
	 * Color of the marker.
	 * @default "#ffffff"
	 */
	color?: string;
	/**
	 * Optional text label to display on hover.
	 */
	label?: string;
	/**
	 * Height of the pin stem relative to marker size.
	 * @default 0.75
	 */
	pinHeight?: number;
	/**
	 * Radius of the pin head relative to marker size.
	 * @default 0.1
	 */
	headRadius?: number;
}
