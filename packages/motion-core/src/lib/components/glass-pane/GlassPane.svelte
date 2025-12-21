<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./Scene.svelte";
	import type { Snippet } from "svelte";

	type Props = {
		image: string;
		class?: string;
		children?: Snippet;
		distortion?: number;
		chromaticAberration?: number;
		speed?: number;
		waviness?: number;
		frequency?: number;
		[key: string]: unknown;
	};

	let {
		image,
		class: className = "",
		children,
		distortion = 1.0,
		chromaticAberration = 0.005,
		speed = 1.0,
		waviness = 0.05,
		frequency = 6.0,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div class="relative w-full h-full overflow-hidden {className}" {...rest}>
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

	{#if children}
		<div class="relative z-10 w-full h-full pointer-events-none">
			{@render children()}
		</div>
	{/if}
</div>
