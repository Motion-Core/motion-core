<script lang="ts">
	import type { ComponentProps } from "svelte";
	import Scene from "./HaloScene.svelte";
	import { cn } from "../../utils/cn";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Camera rotation speed multiplier.
		 * @default 0.5
		 */
		rotationSpeed?: SceneProps["rotationSpeed"];
		/**
		 * Color of the background.
		 * @default "#000000"
		 */
		backgroundColor?: SceneProps["backgroundColor"];
		/**
		 * Distance of the camera from the center.
		 * @default 3.0
		 */
		cameraDistance?: SceneProps["cameraDistance"];
		/**
		 * Field of View (FOV) of the camera in degrees.
		 * @default 55.0
		 */
		fov?: SceneProps["fov"];
		/**
		 * Sun light direction vector (X).
		 * @default 0.0
		 */
		sunX?: SceneProps["sunX"];
		/**
		 * Sun light direction vector (Y).
		 * @default 0.0
		 */
		sunY?: SceneProps["sunY"];
		/**
		 * Sun light direction vector (Z).
		 * @default 1.0
		 */
		sunZ?: SceneProps["sunZ"];
		/**
		 * Overall intensity/brightness of the scattering effect.
		 * @default 1.0
		 */
		intensity?: SceneProps["intensity"];
		[key: string]: unknown;
	}

	let {
		class: className = "",
		rotationSpeed = 0.5,
		backgroundColor = "#000000",
		cameraDistance = 3.0,
		fov = 55.0,
		sunX = 0.0,
		sunY = 0.0,
		sunZ = 1.0,
		intensity = 1.0,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene
			{rotationSpeed}
			{backgroundColor}
			{cameraDistance}
			{fov}
			{sunX}
			{sunY}
			{sunZ}
			{intensity}
		/>
	</div>
</div>
