<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./Scene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";

	interface Props {
		src: string;
		class?: string;
		density?: number;
		strength?: number;
		color?: string;
		backgroundColor?: string;
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

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene image={src} {density} {strength} {color} {backgroundColor} />
		</Canvas>
	</div>
</div>
