<script lang="ts">
	import Scene from "./FluidSimulationScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Dissipation factor for the fluid.
		 * @default 0.96
		 */
		dissipation?: SceneProps["dissipation"];
		/**
		 * Radius of the pointer influence.
		 * @default 0.005
		 */
		pointerSize?: SceneProps["pointerSize"];
		/**
		 * Fluid splat color.
		 * @default "#ff6900"
		 */
		color?: SceneProps["color"];
		/**
		 * Fluid velocity dissipation.
		 * @default 0.96
		 */
		velocityDissipation?: SceneProps["velocityDissipation"];
		/**
		 * Pressure iterations. More iterations = more accurate but slower.
		 * @default 10
		 */
		pressureIterations?: SceneProps["pressureIterations"];

		[key: string]: unknown;
	}

	let {
		class: className = "",
		dissipation = 0.96,
		pointerSize = 0.005,
		color = "#ff6900",
		velocityDissipation = 0.96,
		pressureIterations = 10,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene
			{dissipation}
			{pointerSize}
			{color}
			{velocityDissipation}
			{pressureIterations}
		/>
	</div>
</div>
