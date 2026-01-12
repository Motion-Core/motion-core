<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./Card3DScene.svelte";
	import FaceTracker from "./Card3DFaceTracker.svelte";
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
		 * The image source URL.
		 */
		image: SceneProps["image"];
		/**
		 * Width of the card.
		 * @default 3.2
		 */
		width?: SceneProps["width"];
		/**
		 * Height of the card.
		 * @default 2
		 */
		height?: SceneProps["height"];
		/**
		 * Depth/thickness of the card.
		 * @default 0.08
		 */
		depth?: SceneProps["depth"];
		/**
		 * Corner radius of the card.
		 * @default 0.15
		 */
		radius?: SceneProps["radius"];
		/**
		 * Show the camera preview when tracking is enabled.
		 * @default false
		 */
		showPreview?: boolean;

		[key: string]: unknown;
	}

	let {
		class: className = "",
		image,
		width = 3.2,
		height = 2,
		depth = 0.08,
		radius = 0.15,
		showPreview = false,
		...rest
	}: Props = $props();

	let headPosition = $state({ x: 0, y: 0, z: 0 });

	function handleHeadMove(position: { x: number; y: number; z: number }) {
		headPosition = position;
	}

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene {image} {width} {height} {depth} {radius} {headPosition} />
		</Canvas>
	</div>
</div>

<FaceTracker onHeadMove={handleHeadMove} {showPreview} />
