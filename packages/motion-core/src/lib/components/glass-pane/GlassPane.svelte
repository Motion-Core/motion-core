<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./GlassPaneScene.svelte";
	import { cn } from "../../utils/cn";

	interface Props {
		image: string;
		class?: string;
		distortion?: number;
		chromaticAberration?: number;
		speed?: number;
		waviness?: number;
		frequency?: number;
		[key: string]: unknown;
	}

	let {
		image,
		class: className = "",
		distortion = 1.0,
		chromaticAberration = 0.005,
		speed = 1.0,
		waviness = 0.05,
		frequency = 6.0,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr}>
			<Scene
				{image}
				{distortion}
				{chromaticAberration}
				{speed}
				{waviness}
				{frequency}
			/>
		</Canvas>
	</div>
</div>
