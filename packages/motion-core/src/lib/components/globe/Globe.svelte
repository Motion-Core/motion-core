<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./GlobeScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

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
		 * @default 12000
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
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full", className)} {...rest}>
	<Canvas {dpr}>
		<Scene
			{radius}
			{fresnelConfig}
			{atmosphereConfig}
			{pointCount}
			{landPointColor}
			{pointSize}
			{autoRotate}
		/>
	</Canvas>
</div>
