<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./RubiksCubeScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * The size of the individual cubelets.
		 * @default 1
		 */
		size?: SceneProps["size"];
		/**
		 * Duration of the rotation animation in seconds.
		 * @default 1.5
		 */
		duration?: SceneProps["duration"];
		/**
		 * Gap between the cubelets.
		 * @default 0.015
		 */
		gap?: SceneProps["gap"];
		/**
		 * Corner radius of the cubelets.
		 * @default 0.125
		 */
		radius?: SceneProps["radius"];
		/**
		 * Configuration for the Fresnel shader uniforms.
		 */
		fresnelConfig?: SceneProps["fresnelConfig"];

		[key: string]: unknown;
	}

	let {
		class: className = "",
		size = 1,
		duration = 1.5,
		gap = 0.015,
		radius = 0.125,
		fresnelConfig,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr}>
			<Scene {size} {duration} {gap} {radius} {fresnelConfig} />
		</Canvas>
	</div>
</div>
