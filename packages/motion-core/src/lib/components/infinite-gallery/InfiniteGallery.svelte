<script lang="ts">
	import { Canvas, T } from "@threlte/core";
	import { NoToneMapping } from "three";
	import GalleryScene from "./InfiniteGalleryScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof GalleryScene>;

	interface Props {
		/**
		 * Array of images to display. Can be strings (URL) or objects with src and alt.
		 */
		images: SceneProps["images"];
		/**
		 * Scroll speed multiplier.
		 * @default 1
		 */
		speed?: SceneProps["speed"];
		/**
		 * Number of images visible in the tunnel at once.
		 * @default 8
		 */
		visibleCount?: SceneProps["visibleCount"];
		/**
		 * Configuration for fade in/out effects based on depth.
		 */
		fadeSettings?: SceneProps["fadeSettings"];
		/**
		 * Configuration for blur in/out effects based on depth.
		 */
		blurSettings?: SceneProps["blurSettings"];
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		[key: string]: unknown;
	}

	let {
		images,
		speed = 1,
		visibleCount = 8,
		fadeSettings = {
			fadeIn: { start: 0.01, end: 0.25 },
			fadeOut: { start: 0.43, end: 0.46 },
		},
		blurSettings = {
			blurIn: { start: 0.0, end: 0.2 },
			blurOut: { start: 0.43, end: 0.46 },
			maxBlur: 8.0,
		},
		class: className = "",
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<T.PerspectiveCamera makeDefault position={[0, 0, 0]} fov={55} />
			<GalleryScene
				{images}
				{speed}
				{visibleCount}
				{fadeSettings}
				{blurSettings}
			/>
		</Canvas>
	</div>
</div>
