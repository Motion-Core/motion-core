<script lang="ts">
	import type { ComponentProps } from "svelte";
	import Scene from "./GodRaysScene.svelte";
	import { cn } from "../../utils/cn";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Base color of the rays.
		 * @default "#FFFFFF"
		 */
		color?: SceneProps["color"];
		/**
		 * Color of the background.
		 * @default "#000000"
		 */
		backgroundColor?: SceneProps["backgroundColor"];
		/**
		 * Horizontal anchor point of the ray source (0-1).
		 * @default 0.5
		 */
		anchorX?: SceneProps["anchorX"];
		/**
		 * Vertical anchor point of the ray source (0-1).
		 * @default 1.2
		 */
		anchorY?: SceneProps["anchorY"];
		/**
		 * Horizontal direction of the rays.
		 * @default 0.0
		 */
		directionX?: SceneProps["directionX"];
		/**
		 * Vertical direction of the rays.
		 * @default -1.0
		 */
		directionY?: SceneProps["directionY"];
		/**
		 * Speed multiplier for the animation.
		 * @default 1.0
		 */
		speed?: SceneProps["speed"];
		/**
		 * The spread of the light rays.
		 * @default 1.0
		 */
		lightSpread?: SceneProps["lightSpread"];
		/**
		 * The length of the rays.
		 * @default 1.0
		 */
		rayLength?: SceneProps["rayLength"];
		/**
		 * Whether the rays should pulsate.
		 * @default false
		 */
		pulsating?: SceneProps["pulsating"];
		/**
		 * Distance at which the rays start to fade out.
		 * @default 1.0
		 */
		fadeDistance?: SceneProps["fadeDistance"];
		/**
		 * Saturation of the final ray colors.
		 * @default 1.0
		 */
		saturation?: SceneProps["saturation"];
		/**
		 * Amount of grain/noise applied to the rays.
		 * @default 0.0
		 */
		noiseAmount?: SceneProps["noiseAmount"];
		/**
		 * Amount of wave distortion applied to the rays.
		 * @default 0.0
		 */
		distortion?: SceneProps["distortion"];
		/**
		 * Global ray intensity. Lower values dim and tighten rays, higher values brighten them.
		 * @default 1.0
		 */
		intensity?: SceneProps["intensity"];
		[key: string]: unknown;
	}

	let {
		class: className = "",
		color = "#FFFFFF",
		backgroundColor = "#000000",
		anchorX = 0.5,
		anchorY = 1.2,
		directionX = 0.0,
		directionY = -1.0,
		speed = 1.0,
		lightSpread = 1.0,
		rayLength = 1.0,
		pulsating = false,
		fadeDistance = 1.0,
		saturation = 1.0,
		noiseAmount = 0.0,
		distortion = 0.0,
		intensity = 1.0,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene
			{color}
			{backgroundColor}
			{anchorX}
			{anchorY}
			{directionX}
			{directionY}
			{speed}
			{lightSpread}
			{rayLength}
			{pulsating}
			{fadeDistance}
			{saturation}
			{noiseAmount}
			{distortion}
			{intensity}
		/>
	</div>
</div>
