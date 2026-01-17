<script lang="ts">
	import { onMount, onDestroy, tick } from "svelte";
	import { gsap } from "gsap/dist/gsap";
	import { Flip } from "gsap/dist/Flip";
	import { MorphSVGPlugin } from "gsap/dist/MorphSVGPlugin";
	import { cn } from "../../utils/cn";
	import VideoSlider from "./VideoSlider.svelte";

	interface Props {
		/**
		 * The source URL of the video.
		 */
		src: string;
		/**
		 * The URL of the video poster image.
		 */
		poster?: string;
		/**
		 * Whether the video should start playing automatically.
		 * @default false
		 */
		autoplay?: boolean;
		/**
		 * Whether the video should be muted by default.
		 * @default true
		 */
		muted?: boolean;

		/**
		 * Whether the video should loop.
		 * @default false
		 */
		loop?: boolean;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
	}

	let {
		src,
		poster,
		autoplay = false,
		muted = true,
		loop = false,
		class: className,
	}: Props = $props();

	let videoRef: HTMLVideoElement;
	let containerRef: HTMLElement;
	let pathRef: SVGPathElement;
	let playPathRef: SVGPathElement;
	let isPlaying = $state(false);
	let currentTime = $state(0);
	let duration = $state(0);
	let isScrubbing = $state(false);
	let isLayoutFullscreen = $state(false);
	let isExpanded = $state(false);
	let placeholder: HTMLElement | null = null;
	let originalParent: ParentNode | null = null;
	let originalNextSibling: Node | null = null;
	let currentTimeStr = $derived(formatTime(currentTime));
	let durationStr = $derived(formatTime(duration));
	let rafId: number;

	const enterPath = "M15 3h6v6 M9 21H3v-6 M21 3l-7 7 M3 21l7-7";
	const exitPath =
		"M8 3v3a2 2 0 0 1-2 2H3 M21 8h-3a2 2 0 0 1-2-2V3 M3 16h3a2 2 0 0 1 2 2v3 M16 21v-3a2 2 0 0 1 2-2h3";
	const iconPlay = "M7 5v14l11-7z";
	const iconPause = "M6 5h4v14H6zm8 0h4v14h-4z";

	onMount(() => {
		gsap.registerPlugin(MorphSVGPlugin);
		gsap.registerPlugin(Flip);
	});

	async function toggleFullscreen() {
		if (!containerRef) return;
		const state = Flip.getState(containerRef);
		if (!isExpanded) {
			isExpanded = true;

			if (!isLayoutFullscreen) {
				originalParent = containerRef.parentNode;
				originalNextSibling = containerRef.nextSibling;
				const rect = containerRef.getBoundingClientRect();
				placeholder = document.createElement("div");
				placeholder.style.width = `${rect.width}px`;
				placeholder.style.height = `${rect.height}px`;

				if (originalParent) {
					originalParent.insertBefore(placeholder, containerRef);
				}
				document.body.appendChild(containerRef);
				isLayoutFullscreen = true;
			}
			await tick();

			Flip.from(state, {
				duration: 0.5,
				ease: "power4.inOut",
				absolute: true,
				zIndex: 9999,
			});
		} else {
			isExpanded = false;

			if (placeholder) {
				Flip.fit(containerRef, placeholder, {
					duration: 0.5,
					ease: "power4.inOut",
					absolute: true,
					onComplete: () => {
						isLayoutFullscreen = false;

						if (placeholder && placeholder.parentNode) {
							placeholder.parentNode.insertBefore(containerRef, placeholder);
							placeholder.remove();
							placeholder = null;
						} else if (originalParent) {
							if (originalNextSibling) {
								originalParent.insertBefore(containerRef, originalNextSibling);
							} else {
								originalParent.appendChild(containerRef);
							}
						}
						gsap.set(containerRef, { clearProps: "all" });
					},
				});
			} else {
				isLayoutFullscreen = false;
				await tick();
			}
		}
	}

	function formatTime(seconds: number) {
		if (!Number.isFinite(seconds) || seconds < 0) return "0:00";
		const m = Math.floor(seconds / 60);
		const s = Math.floor(seconds % 60);
		return `${m}:${s.toString().padStart(2, "0")}`;
	}

	function updateTime() {
		if (videoRef && !isScrubbing && !videoRef.paused) {
			currentTime = videoRef.currentTime;
		}
		rafId = requestAnimationFrame(updateTime);
	}

	function togglePlay() {
		if (!videoRef) return;

		if (videoRef.paused) {
			videoRef.play();
		} else {
			videoRef.pause();
		}
	}

	function onPlay() {
		isPlaying = true;
		rafId = requestAnimationFrame(updateTime);
	}

	function onPause() {
		isPlaying = false;
		cancelAnimationFrame(rafId);
	}

	function onLoadedMetadata() {
		duration = videoRef.duration;
		currentTime = videoRef.currentTime;
	}

	function onEnded() {
		isPlaying = false;
		currentTime = duration;
	}

	function handleScrubStart() {
		isScrubbing = true;
		if (isPlaying) {
			videoRef.pause();
		}
	}

	function handleScrubEnd() {
		isScrubbing = false;
		if (isPlaying) {
			videoRef.play();
		}
	}

	function handleSeek(time: number) {
		if (videoRef) {
			videoRef.currentTime = Math.min(Math.max(time, 0), duration);
			currentTime = time;
		}
	}

	onMount(() => {
		if (videoRef) {
			if (videoRef.readyState >= 1) {
				onLoadedMetadata();
			}
		}
	});

	onDestroy(() => {
		if (typeof cancelAnimationFrame !== "undefined") {
			cancelAnimationFrame(rafId);
		}
		if (
			isLayoutFullscreen &&
			containerRef &&
			document.body.contains(containerRef)
		) {
			document.body.removeChild(containerRef);
		}
		if (placeholder && placeholder.parentNode) {
			placeholder.remove();
		}
	});

	$effect(() => {
		if (!playPathRef) return;

		if (isPlaying) {
			gsap.to(playPathRef, {
				morphSVG: iconPause,
				duration: 0.3,
				ease: "power4.inOut",
			});
		} else {
			gsap.to(playPathRef, {
				morphSVG: iconPlay,
				duration: 0.3,
				ease: "power4.inOut",
			});
		}
	});

	$effect(() => {
		if (!pathRef) return;

		if (isExpanded) {
			gsap.to(pathRef, {
				morphSVG: exitPath,
				duration: 0.3,
				ease: "power4.inOut",
			});
		} else {
			gsap.to(pathRef, {
				morphSVG: enterPath,
				duration: 0.3,
				ease: "power4.inOut",
			});
		}
	});
</script>

<div
	bind:this={containerRef}
	class={cn(
		"group flex flex-col overflow-hidden rounded-2xl border border-border bg-background p-2 shadow-sm",

		isLayoutFullscreen
			? "fixed inset-0 z-50 h-screen w-screen"
			: "w-full max-w-3xl",

		className,
	)}
>
	<div
		class={cn(
			"relative overflow-hidden rounded-xl bg-background",
			!isLayoutFullscreen && "aspect-video",
			isLayoutFullscreen && "flex-1",
		)}
	>
		<video
			bind:this={videoRef}
			{src}
			{poster}
			{autoplay}
			{muted}
			{loop}
			class="h-full w-full object-cover"
			playsinline
			onplay={onPlay}
			onpause={onPause}
			onloadedmetadata={onLoadedMetadata}
			onended={onEnded}
			onclick={togglePlay}
		></video>
	</div>

	<div class="flex items-center gap-3 px-2 py-3">
		<button
			onclick={togglePlay}
			class="flex size-8 items-center justify-center rounded-full bg-foreground/45 text-background transition-[background-color,scale] duration-150 ease-out hover:bg-foreground/70 active:scale-95"
			aria-label={isPlaying ? "Pause" : "Play"}
		>
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="currentColor"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path bind:this={playPathRef} d={iconPlay} />
			</svg>
		</button>

		<div class="flex-1">
			<VideoSlider
				{currentTime}
				{duration}
				onScrubStart={handleScrubStart}
				onScrubEnd={handleScrubEnd}
				onSeek={handleSeek}
			/>
		</div>

		<div
			class="flex items-center gap-1 font-mono text-[10px] font-medium text-foreground/70"
		>
			<span class="text-foreground/45">{currentTimeStr}</span>
			<span>/</span>
			<span>{durationStr}</span>
		</div>

		<button
			onclick={toggleFullscreen}
			class="flex size-8 items-center justify-center rounded-full bg-foreground/45 text-background transition-[background-color,scale] duration-150 ease-out hover:bg-foreground/70"
			aria-label={isExpanded ? "Exit Fullscreen" : "Enter Fullscreen"}
		>
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path bind:this={pathRef} d={enterPath} />
			</svg>
		</button>
	</div>
</div>
