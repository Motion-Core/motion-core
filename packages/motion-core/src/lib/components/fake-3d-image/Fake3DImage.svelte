<script lang="ts">
	import Scene from "./Fake3DImageScene.svelte";
	import { cn } from "../../utils/cn";

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
		 * Pointer sensitivity multiplier applied before displacement thresholding.
		 * @default 0.25
		 */
		sensitivity?: number;
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
		sensitivity = 0.25,
		class: className = "",
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene {colorSrc} {depthSrc} {xThreshold} {yThreshold} {sensitivity} />
	</div>
</div>
