<script lang="ts">
	import { Canvas } from "@threlte/core";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";
	import Scene from "./OrbScene.svelte";
	import { cn } from "../../utils/cn";
	import type { OrbState } from "./types";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Current animation state of the orb.
		 * - `idle`: gentle drift, purple palette.
		 * - `attune`: tangential swirl — colors rotate around the sphere.
		 * - `pulse`: radial breathing — colors expand and contract.
		 * - `surge`: lissajous orbital churn — rapid multi-axis movement.
		 * @default "idle"
		 */
		state?: OrbState;
		/**
		 * Audio amplitude in [0, 1] that adds a reactive pulse layer.
		 * @default 0
		 */
		amplitude?: SceneProps["amplitude"];
		/**
		 * Base color for the orb palette and rim glow. Accepts any CSS color string.
		 * @default "#6933ff"
		 */
		color?: SceneProps["color"];
		/**
		 * Speed multiplier applied to the entire animation.
		 * @default 1
		 */
		speed?: SceneProps["speed"];

		[key: string]: unknown;
	}

	let {
		class: className = "",
		state = "idle",
		amplitude = 0,
		color = "#6933ff",
		speed = 1,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0">
		<Canvas toneMapping={NoToneMapping} rendererParameters={{ alpha: true }}>
			<Scene orbState={state} {amplitude} {color} {speed} />
		</Canvas>
	</div>
</div>
