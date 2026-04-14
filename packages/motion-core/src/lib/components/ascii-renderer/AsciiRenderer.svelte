<script lang="ts">
	import Scene from "./AsciiRendererScene.svelte";
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
		 * Grid density for the ASCII effect.
		 * @default 25
		 */
		density?: SceneProps["density"];
		/**
		 * Intensity of the ASCII character generation threshold.
		 * @default 25
		 */
		strength?: SceneProps["strength"];
		/**
		 * Foreground color of the ASCII characters.
		 * @default "#00ff00"
		 */
		color?: SceneProps["color"];
		/**
		 * Background color.
		 * @default "#000000"
		 */
		backgroundColor?: SceneProps["backgroundColor"];
		[key: string]: unknown;
	}

	let {
		src,
		class: className = "",
		density = 25,
		strength = 25,
		color = "#00ff00",
		backgroundColor = "#000000",
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene image={src} {density} {strength} {color} {backgroundColor} />
	</div>
</div>
