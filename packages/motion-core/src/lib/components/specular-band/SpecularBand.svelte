<script lang="ts">
	import type { ComponentProps } from "svelte";
	import Scene from "./SpecularBandScene.svelte";
	import { cn } from "../../utils/cn";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Base color of the specular bands.
		 * @default "#FF6900"
		 */
		color?: SceneProps["color"];
		/**
		 * Color of the background.
		 * @default "#17181A"
		 */
		backgroundColor?: SceneProps["backgroundColor"];
		/**
		 * Animation speed multiplier.
		 * @default 1.0
		 */
		speed?: SceneProps["speed"];
		/**
		 * Lens distortion intensity.
		 * @default 0.2
		 */
		distortion?: SceneProps["distortion"];
		/**
		 * Amount of hue shift for secondary bands (in degrees).
		 * @default 30.0
		 */
		hueShift?: SceneProps["hueShift"];
		/**
		 * Global intensity/brightness of the effect.
		 * @default 1.0
		 */
		intensity?: SceneProps["intensity"];
		[key: string]: unknown;
	}

	let {
		class: className = "",
		color = "#FF6900",
		backgroundColor = "#17181A",
		speed = 1.0,
		distortion = 0.2,
		hueShift = 30.0,
		intensity = 1.0,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene
			{color}
			{backgroundColor}
			{speed}
			{distortion}
			{hueShift}
			{intensity}
		/>
	</div>
</div>
