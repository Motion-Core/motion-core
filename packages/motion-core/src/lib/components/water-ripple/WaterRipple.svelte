<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./WaterRippleScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";

	interface Props {
		src: string;
		class?: string;
		brushSize?: number;
		[key: string]: unknown;
	}

	let {
		src,
		class: className = "",
		brushSize = 100,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene image={src} {brushSize} />
		</Canvas>
	</div>
</div>
