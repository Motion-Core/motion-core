<script lang="ts">
	import { Canvas } from "@threlte/core";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";
	import Scene from "./GlitterClothScene.svelte";
	import { cn } from "../../utils/cn";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Primary color used to derive the full shader palette.
		 * @default "#FF6900"
		 */
		color?: SceneProps["color"];
		/**
		 * Speed multiplier for the full shader animation timeline.
		 * @default 1.0
		 */
		speed?: SceneProps["speed"];
		/**
		 * Global intensity multiplier for the base silk color.
		 * @default 1.0
		 */
		brightness?: SceneProps["brightness"];
		/**
		 * Opacity of the vivid-light glitter blend.
		 * @default 0.02
		 */
		blendStrength?: SceneProps["blendStrength"];
		/**
		 * Spatial scale for simplex noise sampling.
		 * Lower values create finer glitter.
		 * @default 4.0
		 */
		noiseScale?: SceneProps["noiseScale"];
		/**
		 * Base strength of vignette falloff.
		 * @default 15.0
		 */
		vignetteStrength?: SceneProps["vignetteStrength"];
		/**
		 * Vignette curve exponent.
		 * Lower values produce a softer rolloff.
		 * @default 0.25
		 */
		vignettePower?: SceneProps["vignettePower"];
		/**
		 * Opacity of the vignette effect.
		 * `0` disables vignette influence, `1` applies full vignette.
		 * @default 1.0
		 */
		vignetteOpacity?: SceneProps["vignetteOpacity"];
		[key: string]: unknown;
	}

	let {
		class: className = "",
		color = "#FF6900",
		speed = 1.0,
		brightness = 1.0,
		blendStrength = 0.02,
		noiseScale = 4.0,
		vignetteStrength = 15.0,
		vignettePower = 0.25,
		vignetteOpacity = 1.0,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene
				{color}
				{speed}
				{brightness}
				{blendStrength}
				{noiseScale}
				{vignetteStrength}
				{vignettePower}
				{vignetteOpacity}
			/>
		</Canvas>
	</div>
</div>
