<script lang="ts">
	import Scene from "./PixelatedImageScene.svelte";
	import { cn } from "../../utils/cn";
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
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene image={src} {initialGridSize} {stepDuration} />
	</div>
</div>
