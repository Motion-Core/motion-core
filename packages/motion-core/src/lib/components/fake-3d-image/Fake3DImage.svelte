<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./Fake3DImageScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";

	interface Props {
		/**
		 * Source URL of the color texture.
		 */
		colorSrc: string;
		/**
		 * Source URL of the grayscale depth map texture.
		 */
		depthSrc: string;
		/**
		 * Horizontal displacement threshold.
		 * @default 8
		 */
		xThreshold?: number;
		/**
		 * Vertical displacement threshold.
		 * @default 8
		 */
		yThreshold?: number;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		[key: string]: unknown;
	}

	let {
		colorSrc,
		depthSrc,
		xThreshold = 8,
		yThreshold = 8,
		class: className = "",
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene {colorSrc} {depthSrc} {xThreshold} {yThreshold} />
		</Canvas>
	</div>
</div>
