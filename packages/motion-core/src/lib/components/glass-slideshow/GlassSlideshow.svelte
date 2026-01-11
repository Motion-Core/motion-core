<script lang="ts">
	import { Canvas } from "@threlte/core";
	import Scene from "./GlassSlideshowScene.svelte";
	import { cn } from "../../utils/cn";
	import { NoToneMapping } from "three";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Array of image URLs to cycle through.
		 */
		images: SceneProps["images"];
		/**
		 * The current image index. Change this to trigger a transition.
		 * @default 0
		 */
		index?: SceneProps["index"];
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Duration of the transition in milliseconds.
		 * @default 2000
		 */
		transitionDuration?: SceneProps["transitionDuration"];
		/**
		 * Intensity of the glass effect.
		 * @default 1.0
		 */
		intensity?: SceneProps["intensity"];
		/**
		 * Strength of the distortion.
		 * @default 1.0
		 */
		distortion?: SceneProps["distortion"];
		/**
		 * Strength of the chromatic aberration.
		 * @default 1.0
		 */
		chromaticAberration?: SceneProps["chromaticAberration"];
		/**
		 * Strength of the refraction.
		 * @default 1.0
		 */
		refraction?: SceneProps["refraction"];
		/**
		 * Automatically cycle through the provided images.
		 * @default true
		 */
		autoplay?: boolean;
		/**
		 * Delay between automatic transitions in milliseconds.
		 * @default 5000
		 */
		autoplayInterval?: number;
		[key: string]: unknown;
	}

	let {
		images,
		index = 0,
		class: className = "",
		transitionDuration = 2000,
		intensity = 1.0,
		distortion = 1.0,
		chromaticAberration = 1.0,
		refraction = 1.0,
		autoplay = true,
		autoplayInterval = 5000,
		...rest
	}: Props = $props();

	const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;

	let autoplayIndex = $state(0);
	let hasInitializedAutoplayIndex = false;

	const currentIndex = $derived(autoplay ? autoplayIndex : index);

	$effect(() => {
		if (hasInitializedAutoplayIndex) {
			return;
		}

		autoplayIndex = index;
		hasInitializedAutoplayIndex = true;
	});

	$effect(() => {
		const total = images.length;

		if (total === 0) {
			if (autoplayIndex !== 0) {
				autoplayIndex = 0;
			}
			return;
		}

		const normalized = ((autoplayIndex % total) + total) % total;

		if (normalized !== autoplayIndex) {
			autoplayIndex = normalized;
		}
	});

	$effect(() => {
		if (!autoplay) {
			const total = images.length;
			if (total === 0) {
				if (autoplayIndex !== 0) {
					autoplayIndex = 0;
				}
				return;
			}

			const normalized = ((index % total) + total) % total;
			if (normalized !== autoplayIndex) {
				autoplayIndex = normalized;
			}
		}
	});

	$effect(() => {
		const total = images.length;
		const isAutoplaying = autoplay && total > 1;
		const delay = Math.max(autoplayInterval, 0);

		if (!isAutoplaying) {
			return;
		}

		const interval = setInterval(() => {
			const length = images.length;
			if (length === 0) {
				autoplayIndex = 0;
				return;
			}
			autoplayIndex = (autoplayIndex + 1) % length;
		}, delay || 1);

		return () => clearInterval(interval);
	});
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Canvas {dpr} toneMapping={NoToneMapping}>
			<Scene
				{images}
				index={currentIndex}
				{transitionDuration}
				{intensity}
				{distortion}
				{chromaticAberration}
				{refraction}
			/>
		</Canvas>
	</div>
</div>
