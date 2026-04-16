<script lang="ts">
	import Scene from "./LavaLampScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Base color of the lava blobs.
		 * @default "#17181A"
		 */
		color?: SceneProps["color"];
		/**
		 * Color of the fresnel effect.
		 * @default "#ff6900"
		 */
		fresnelColor?: SceneProps["fresnelColor"];
		/**
		 * Speed of the lava animation.
		 * @default 1.0
		 */
		speed?: SceneProps["speed"];
		/**
		 * Fresnel power for the edge lighting effect.
		 * @default 3.0
		 */
		fresnelPower?: SceneProps["fresnelPower"];
		/**
		 * Base radius of the blobs.
		 * @default 1
		 */
		radius?: SceneProps["radius"];
		/**
		 * Smoothness of the blob blending (metaball effect).
		 * @default 0.1
		 */
		smoothness?: SceneProps["smoothness"];
		[key: string]: unknown;
	}

	let {
		class: className = "",
		color = "#17181A",
		fresnelColor = "#ff6900",
		speed = 1.0,
		fresnelPower = 3.0,
		radius = 1,
		smoothness = 0.1,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene
			{color}
			{fresnelColor}
			{speed}
			{fresnelPower}
			{radius}
			{smoothness}
		/>
	</div>
</div>
