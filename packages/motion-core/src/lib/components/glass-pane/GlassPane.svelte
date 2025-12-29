<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./GlassPaneScene.svelte";
	import { cn } from "../../utils/cn";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Strength of the refraction/distortion effect.
		 * @default 1.0
		 */
		distortion?: number;
		/**
		 * Amount of chromatic aberration (color splitting).
		 * @default 0.005
		 */
		chromaticAberration?: number;
		/**
		 * Speed of the wave animation.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Amplitude of the wave distortion.
		 * @default 0.05
		 */
		waviness?: number;
		/**
		 * Frequency of the wave distortion.
		 * @default 6.0
		 */
		frequency?: number;
		[key: string]: unknown;
	}

	let {
		image,
		class: className = "",
		distortion = 1.0,
		chromaticAberration = 0.005,
		speed = 1.0,
		waviness = 0.05,
		frequency = 6.0,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr}>
			<Scene
				{image}
				{distortion}
				{chromaticAberration}
				{speed}
				{waviness}
				{frequency}
			/>
		</Canvas>
	</div>
</div>
