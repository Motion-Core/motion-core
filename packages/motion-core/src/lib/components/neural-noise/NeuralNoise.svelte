<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./NeuralNoiseScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Speed of the noise animation.
		 * @default 1.0
		 */
		speed?: SceneProps["speed"];
		[key: string]: unknown;
	}

	let { class: className = "", speed = 1.0, ...rest }: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene {speed} />
		</Canvas>
	</div>
</div>
