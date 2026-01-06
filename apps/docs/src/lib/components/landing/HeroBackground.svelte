<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./HeroBackgroundScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		class?: string;
		shadow?: SceneProps["shadow"];
		highlight?: SceneProps["highlight"];
	}

	let { class: className = "", shadow, highlight }: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;
</script>

<div
	class={cn(
		"pointer-events-none absolute inset-2 z-0 overflow-hidden rounded-xl border border-border",
		className,
	)}
>
	<Canvas {dpr}>
		<Scene {shadow} {highlight} />
	</Canvas>
	<div
		class="absolute inset-x-0 bottom-0 z-10 h-96 overflow-hidden rounded-b-xl bg-linear-to-b from-transparent to-card"
	></div>
</div>
