<script lang="ts">
	import { Canvas } from "@threlte/core";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";
	import Scene from "./PaleSilkScene.svelte";
	import { cn } from "../../utils/cn";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Base color of the shader before highlights and blending.
		 * @default "#422042"
		 */
		baseColor?: SceneProps["baseColor"];
		/**
		 * Global intensity multiplier applied to the base color.
		 * @default 1.7
		 */
		brightness?: SceneProps["brightness"];
		/**
		 * Vertical wave displacement amplitude.
		 * @default 0.03
		 */
		waveAmplitude?: SceneProps["waveAmplitude"];
		/**
		 * Horizontal frequency of the wave distortion.
		 * @default 8.0
		 */
		waveFrequency?: SceneProps["waveFrequency"];
		/**
		 * Speed multiplier for the wave and procedural animation.
		 * @default 1.0
		 */
		waveSpeed?: SceneProps["waveSpeed"];
		/**
		 * Simplex noise coefficient controlling spot falloff.
		 * @default 0.61
		 */
		noiseSizeCoeff?: SceneProps["noiseSizeCoeff"];
		/**
		 * Simplex noise density multiplier.
		 * @default 53.0
		 */
		noiseDensity?: SceneProps["noiseDensity"];
		/**
		 * Spatial scale for simplex noise sampling.
		 * @default 10.0
		 */
		noiseScale?: SceneProps["noiseScale"];
		/**
		 * Strength of the glitter/noise layer before blend.
		 * @default 8.0
		 */
		noiseStrength?: SceneProps["noiseStrength"];
		/**
		 * Blend mode used to combine glitter noise with base color.
		 * @default "vividLight"
		 */
		blendMode?: SceneProps["blendMode"];
		/**
		 * Opacity of the selected blend mode.
		 * @default 0.02
		 */
		blendStrength?: SceneProps["blendStrength"];
		/**
		 * Vignette intensity coefficient.
		 * @default 15.0
		 */
		vignetteStrength?: SceneProps["vignetteStrength"];
		/**
		 * Vignette curve exponent.
		 * @default 0.25
		 */
		vignettePower?: SceneProps["vignettePower"];
		/**
		 * Strength of the extra flow shading pass.
		 * @default 1.0
		 */
		flowShadingStrength?: SceneProps["flowShadingStrength"];
		[key: string]: unknown;
	}

	let {
		class: className = "",
		baseColor = "#422042",
		brightness = 1.7,
		waveAmplitude = 0.03,
		waveFrequency = 8.0,
		waveSpeed = 1.0,
		noiseSizeCoeff = 0.61,
		noiseDensity = 53.0,
		noiseScale = 10.0,
		noiseStrength = 8.0,
		blendMode = "vividLight",
		blendStrength = 0.02,
		vignetteStrength = 15.0,
		vignettePower = 0.25,
		flowShadingStrength = 1.0,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene
				{baseColor}
				{brightness}
				{waveAmplitude}
				{waveFrequency}
				{waveSpeed}
				{noiseSizeCoeff}
				{noiseDensity}
				{noiseScale}
				{noiseStrength}
				{blendMode}
				{blendStrength}
				{vignetteStrength}
				{vignettePower}
				{flowShadingStrength}
			/>
		</Canvas>
	</div>
</div>
