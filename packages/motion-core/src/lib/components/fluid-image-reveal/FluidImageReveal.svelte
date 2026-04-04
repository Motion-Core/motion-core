<script lang="ts">
	import { Canvas } from "@threlte/core";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";
	import Scene from "./FluidImageRevealScene.svelte";
	import { cn } from "../../utils/cn";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Source URL of the base image.
		 */
		baseImage: SceneProps["baseImage"];
		/**
		 * Source URL of the image revealed by the fluid mask.
		 */
		revealImage: SceneProps["revealImage"];
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Dissipation factor for the reveal mask.
		 * @default 0.96
		 */
		dissipation?: SceneProps["dissipation"];
		/**
		 * Radius of the pointer influence.
		 * @default 0.005
		 */
		pointerSize?: SceneProps["pointerSize"];
		/**
		 * Fluid velocity dissipation.
		 * @default 0.96
		 */
		velocityDissipation?: SceneProps["velocityDissipation"];
		/**
		 * Pressure iterations. More iterations = more accurate but slower.
		 * @default 10
		 */
		pressureIterations?: SceneProps["pressureIterations"];
		/**
		 * Softness of the reveal transition edge.
		 * @default 0.22
		 */
		blendSoftness?: SceneProps["blendSoftness"];
		[key: string]: unknown;
	}

	let {
		baseImage,
		revealImage,
		class: className = "",
		dissipation = 0.96,
		pointerSize = 0.005,
		velocityDissipation = 0.96,
		pressureIterations = 10,
		blendSoftness = 0.22,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene
				{baseImage}
				{revealImage}
				{dissipation}
				{pointerSize}
				{velocityDissipation}
				{pressureIterations}
				{blendSoftness}
			/>
		</Canvas>
	</div>
</div>
