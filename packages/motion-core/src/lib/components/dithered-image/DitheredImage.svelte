<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./DitheredImageScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * The image source URL.
		 */
		src: SceneProps["image"];
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Type of dithering map to use.
		 * @default "bayer4x4"
		 */
		ditherMap?: SceneProps["ditherMap"];
		/**
		 * Pixel size of the dithering effect.
		 * @default 1
		 */
		pixelSize?: SceneProps["pixelSize"];
		/**
		 * Foreground color (dots).
		 * @default "#ff6900"
		 */
		color?: SceneProps["color"];
		/**
		 * Background color.
		 * @default "#111113"
		 */
		backgroundColor?: SceneProps["backgroundColor"];

		[key: string]: unknown;
	}

	let {
		src,
		class: className = "",
		ditherMap = "bayer4x4",
		pixelSize = 1,
		color = "#ff6900",
		backgroundColor = "#111113",
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene image={src} {ditherMap} {pixelSize} {color} {backgroundColor} />
		</Canvas>
	</div>
</div>
