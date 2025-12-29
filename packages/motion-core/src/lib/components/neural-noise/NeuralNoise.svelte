<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./NeuralNoiseScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";

	interface Props {
		class?: string;
		speed?: number;
		[key: string]: unknown;
	}

	let {
		class: className = "",
		speed = 1.0,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene {speed} />
		</Canvas>
	</div>
</div>
