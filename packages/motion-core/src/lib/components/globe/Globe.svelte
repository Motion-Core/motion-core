<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./GlobeScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";
	import type { GlobeMarker } from "./types";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Radius of the sphere.
		 * @default 2
		 */
		radius?: SceneProps["radius"];
		/**
		 * Optional overrides for the Fresnel shader uniforms.
		 */
		fresnelConfig?: SceneProps["fresnelConfig"];
		/**
		 * Optional configuration for the atmospheric halo.
		 */
		atmosphereConfig?: SceneProps["atmosphereConfig"];
		/**
		 * Number of points rendered on the surface.
		 * @default 15000
		 */
		pointCount?: SceneProps["pointCount"];
		/**
		 * Color applied to points that fall on land.
		 * @default "#f77114"
		 */
		landPointColor?: SceneProps["landPointColor"];
		/**
		 * Size of each point in world units.
		 * @default 0.05
		 */
		pointSize?: SceneProps["pointSize"];
		/**
		 * Whether the globe should auto-rotate.
		 * @default true
		 */
		autoRotate?: SceneProps["autoRotate"];
		/**
		 * Whether to lock the camera's polar angle (vertical rotation).
		 * If true, limits the vertical view to a narrow band.
		 * @default true
		 */
		lockedPolarAngle?: boolean;
		/**
		 * Array of markers to display on the globe.
		 */
		markers?: GlobeMarker[];
		/**
		 * Coordinates [lat, lon] to focus the camera on.
		 * When set, auto-rotation will be disabled temporarily.
		 */
		focusOn?: [number, number] | null;

		[key: string]: unknown;
	}

	let {
		class: className = "",
		radius = 2,
		fresnelConfig,
		atmosphereConfig,
		pointCount,
		landPointColor,
		pointSize,
		autoRotate = true,
		lockedPolarAngle = true,
		markers = [],
		focusOn = null,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr}>
			<Scene
				{radius}
				{fresnelConfig}
				{atmosphereConfig}
				{pointCount}
				{landPointColor}
				{pointSize}
				{autoRotate}
				{lockedPolarAngle}
				{markers}
				{focusOn}
			/>
		</Canvas>
	</div>
</div>
