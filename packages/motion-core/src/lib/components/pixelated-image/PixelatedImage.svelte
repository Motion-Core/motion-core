<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./PixelatedImageScene.svelte";
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
		 * Initial grid size for the pixelation effect.
		 * @default 6.0
		 */
		initialGridSize?: SceneProps["initialGridSize"];
		/**
		 * Duration of each step in the depixelation animation.
		 * @default 0.15
		 */
		stepDuration?: SceneProps["stepDuration"];
		[key: string]: unknown;
	}

	let {
		src,
		class: className = "",
		initialGridSize = 6.0,
		stepDuration = 0.15,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene image={src} {initialGridSize} {stepDuration} />
		</Canvas>
	</div>
</div>
