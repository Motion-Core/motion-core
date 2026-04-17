<script lang="ts">
	import Scene from "./WaterRippleScene.svelte";
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
		 * Size of the ripple brush.
		 * @default 100
		 */
		brushSize?: SceneProps["brushSize"];
		[key: string]: unknown;
	}

	let {
		src,
		class: className = "",
		brushSize = 100,
		...rest
	}: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene image={src} {brushSize} />
	</div>
</div>
