<script lang="ts">
	import { Canvas, T } from "@threlte/core";
	import { NoToneMapping } from "three";
	import GalleryScene from "./InfiniteGalleryScene.svelte";
	import { cn } from "../../utils/cn";

	type ImageItem = string | { src: string; alt?: string };

	interface Props {
		images: ImageItem[];
		speed?: number;
		visibleCount?: number;
		fadeSettings?: {
			fadeIn: { start: number; end: number };
			fadeOut: { start: number; end: number };
		};
		blurSettings?: {
			blurIn: { start: number; end: number };
			blurOut: { start: number; end: number };
			maxBlur: number;
		};
		class?: string;
		[key: string]: unknown;
	}

	let {
		images,
		speed = 1,
		visibleCount = 8,
		fadeSettings = {
			fadeIn: { start: 0.05, end: 0.25 },
			fadeOut: { start: 0.4, end: 0.43 },
		},
		blurSettings = {
			blurIn: { start: 0.0, end: 0.1 },
			blurOut: { start: 0.4, end: 0.43 },
			maxBlur: 8.0,
		},
		class: className = "",
		...rest
	}: Props = $props();
</script>

<div class={cn("h-full w-full", className)} {...rest}>
	<Canvas toneMapping={NoToneMapping}>
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
