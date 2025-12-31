<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./LavaLampScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Color of the rim light.
		 * @default "#ffffff"
		 */
		color?: SceneProps["color"];
		/**
		 * Background color of the scene.
		 * @default "#ffffff"
		 */
		backgroundColor?: SceneProps["backgroundColor"];
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
		color = "#ffffff",
		backgroundColor = "#ffffff",
		speed = 1.0,
		fresnelPower = 3.0,
		radius = 1,
		smoothness = 0.1,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene
				{color}
				{backgroundColor}
				{speed}
				{fresnelPower}
				{radius}
				{smoothness}
			/>
		</Canvas>
	</div>
</div>
