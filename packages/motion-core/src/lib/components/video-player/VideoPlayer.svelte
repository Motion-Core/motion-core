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
		 * Whether to permanently hide controls and disable interaction.
		 * @default false
		 */
		hideControls?: boolean;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
	}

	let {
		src,
		poster,
		autoplay = false,
		muted = $bindable(true),
		loop = false,
		hideControls = false,
		class: className,
	}: Props = $props();

	let videoRef: HTMLVideoElement;
	let containerRef: HTMLElement;
	let controlsRef: HTMLElement;
	let bgRef: HTMLElement;
	let pathRef: SVGPathElement;
	let playPathRef: SVGPathElement;
	let mutePathRef: SVGPathElement;
	let isPlaying = $state(false);
	let isHovered = $state(false);
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
	const iconVolume =
		"M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z";
	const iconMute =
		"M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73 4.27 3zM12 4L9.91 6.09 12 8.18V4z";

	onMount(() => {
		gsap.registerPlugin(MorphSVGPlugin);
		gsap.registerPlugin(Flip);
	});

	async function toggleFullscreen() {
		if (!containerRef) return;
		const state = Flip.getState(containerRef, { props: "borderRadius" });
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
					borderRadius: "1rem",
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
		if (hideControls) return;
		if (!videoRef) return;

		if (videoRef.paused) {
			videoRef.play();
		} else {
			videoRef.pause();
		}
	}

	function toggleMute() {
		if (videoRef) {
			muted = !muted;
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
		if (!mutePathRef) return;

		if (muted) {
			gsap.to(mutePathRef, {
				morphSVG: iconMute,
				duration: 0.3,
				ease: "power4.inOut",
			});
		} else {
			gsap.to(mutePathRef, {
				morphSVG: iconVolume,
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

	$effect(() => {
		if (!controlsRef || !bgRef || hideControls) return;

		if (isHovered || !isPlaying) {
			gsap.to(bgRef, {
				opacity: 1,
				duration: 0.3,
				ease: "power4.out",
				overwrite: true,
			});
			gsap.to(controlsRef.children, {
				y: 0,
				opacity: 1,
				duration: 0.3,
				ease: "power4.out",
				overwrite: true,
			});
			gsap.set(controlsRef, { pointerEvents: "auto" });
		} else {
			gsap.to(bgRef, {
				opacity: 0,
				duration: 0.3,
				ease: "power4.in",
				overwrite: true,
			});
			gsap.to(controlsRef.children, {
				y: 20,
				opacity: 0,
				duration: 0.3,
				ease: "power4.in",
				overwrite: true,
			});
			gsap.set(controlsRef, { pointerEvents: "none" });
		}
	});
</script>

<div
	bind:this={containerRef}
	onmouseenter={() => (isHovered = true)}
	onmouseleave={() => (isHovered = false)}
	role="region"
	aria-label="Video Player"
	class={cn(
		"group relative flex overflow-hidden bg-fixed-dark shadow-sm",
		isLayoutFullscreen
			? "fixed inset-0 z-50 h-screen w-screen rounded-none"
			: "aspect-video w-full max-w-3xl rounded-2xl",
		className,
	)}
>
	<!-- svelte-ignore a11y_media_has_caption -->
	<video
		bind:this={videoRef}
		{src}
		{poster}
		{autoplay}
		bind:muted
		{loop}
		class="h-full w-full object-cover"
		playsinline
		onplay={onPlay}
		onpause={onPause}
		onloadedmetadata={onLoadedMetadata}
		onended={onEnded}
		onclick={togglePlay}
	></video>

	<div
		bind:this={bgRef}
		class={cn(
			"pointer-events-none absolute right-0 bottom-0 left-0 z-10 h-32 bg-linear-to-t from-fixed-dark/70 via-fixed-dark/45 to-transparent opacity-0",
			hideControls && "hidden",
		)}
	></div>

	<div
		bind:this={controlsRef}
		class={cn(
			"pointer-events-none absolute right-0 bottom-0 left-0 z-20 flex items-center gap-3 px-4 pt-10 pb-4",
			hideControls && "hidden",
		)}
	>
		<button
			onclick={togglePlay}
			class="flex size-8 translate-y-5 items-center justify-center rounded-full bg-fixed-light/10 text-fixed-light opacity-0 backdrop-blur-md transition-[background-color,scale] duration-150 ease-out hover:bg-fixed-light/20 active:scale-95"
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

		<div class="flex-1 translate-y-5 opacity-0">
			<VideoSlider
				{currentTime}
				{duration}
				onScrubStart={handleScrubStart}
				onScrubEnd={handleScrubEnd}
				onSeek={handleSeek}
			/>
		</div>

		<div
			class="flex translate-y-5 items-center gap-1 font-mono text-[10px] font-medium text-fixed-light opacity-0"
		>
			<span class="text-fixed-light/70">{currentTimeStr}</span>
			<span>/</span>
			<span>{durationStr}</span>
		</div>

		<button
			onclick={toggleMute}
			class="flex size-8 translate-y-5 items-center justify-center rounded-full bg-fixed-light/10 text-fixed-light opacity-0 backdrop-blur-md transition-[background-color,scale] duration-150 ease-out hover:bg-fixed-light/20"
			aria-label={muted ? "Unmute" : "Mute"}
		>
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="currentColor"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path bind:this={mutePathRef} d={iconVolume} />
			</svg>
		</button>

		<button
			onclick={toggleFullscreen}
			class="flex size-8 translate-y-5 items-center justify-center rounded-full bg-fixed-light/10 text-fixed-light opacity-0 backdrop-blur-md transition-[background-color,scale] duration-150 ease-out hover:bg-fixed-light/20"
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
