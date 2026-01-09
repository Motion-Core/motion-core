<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./PlasmaGridScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * The base background color of the effect.
		 * @default "#18181B"
		 */
		color?: SceneProps["color"];
		/**
		 * The color used for the plasma noise gradients.
		 * @default "#572400"
		 */
		highlightColor?: SceneProps["highlightColor"];
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		[key: string]: unknown;
	}

	let {
		color,
		highlightColor,
		class: className = "",
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<Canvas {dpr} toneMapping={NoToneMapping}>
		<Scene {color} {highlightColor} />
	</Canvas>
</div>
