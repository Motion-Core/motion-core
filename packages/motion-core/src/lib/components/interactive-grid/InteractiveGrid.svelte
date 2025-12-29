<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./InteractiveGridScene.svelte";
	import { cn } from "../../utils/cn";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Grid resolution (number of cells per row/column).
		 * @default 15
		 */
		grid?: number;
		/**
		 * Radius of mouse influence.
		 * @default 0.15
		 */
		mouseSize?: number;
		/**
		 * Strength of the distortion effect.
		 * @default 0.35
		 */
		strength?: number;
		/**
		 * Relaxation factor for returning to original state (0-1).
		 * @default 0.9
		 */
		relaxation?: number;
		[key: string]: unknown;
	}

	let {
		image,
		class: className = "",
		grid = 15,
		mouseSize = 0.15,
		strength = 0.35,
		relaxation = 0.9,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
	let container = $state<HTMLElement>();
	let mouseX = $state(0);
	let mouseY = $state(0);

	function handleMouseMove(e: MouseEvent) {
		if (!container) return;
		const rect = container.getBoundingClientRect();
		mouseX = (e.clientX - rect.left) / rect.width;
		mouseY = (e.clientY - rect.top) / rect.height;
	}
</script>

<div
	bind:this={container}
	class={cn("relative h-full w-full overflow-hidden", className)}
	onmousemove={handleMouseMove}
	{...rest}
>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr}>
			<Scene
				{image}
				{grid}
				{mouseSize}
				{strength}
				{relaxation}
				{mouseX}
				{mouseY}
			/>
		</Canvas>
	</div>
</div>
