<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { cn } from "../../utils/cn";

	interface Props {
		/**
		 * The current playback time in seconds.
		 */
		currentTime: number;
		/**
		 * The total duration of the video in seconds.
		 */
		duration: number;
		/**
		 * Callback function triggered when scrubbing starts.
		 */
		onScrubStart: () => void;
		/**
		 * Callback function triggered when scrubbing ends.
		 */
		onScrubEnd: () => void;
		/**
		 * Callback function triggered when seeking to a specific time.
		 */
		onSeek: (time: number) => void;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
	}

	let {
		currentTime,
		duration,
		onScrubStart,
		onScrubEnd,
		onSeek,
		class: className,
	}: Props = $props();

	let sliderRef: HTMLElement;
	let thumbRef: HTMLElement;
	let hoverTimeRef: HTMLElement;

	let isHovered = $state(false);
	let isDragging = $state(false);
	let hoverTime = $state(0);
	let hoverX = $state(0);

	function formatTime(seconds: number) {
		if (!Number.isFinite(seconds) || seconds < 0) return "00:00";
		const m = Math.floor(seconds / 60);
		const s = Math.floor(seconds % 60);
		return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
	}

	function handlePointerMove(e: PointerEvent) {
		const bounds = sliderRef.getBoundingClientRect();
		const clientX = e.clientX;

		let x = clientX - bounds.left;
		const clampedX = Math.max(0, Math.min(x, bounds.width));
		const percent = clampedX / bounds.width;

		hoverTime = percent * duration;
		hoverX = clampedX;

		if (isDragging) {
			onSeek(hoverTime);
		}
	}

	function handlePointerDown(e: PointerEvent) {
		sliderRef.setPointerCapture(e.pointerId);
		isDragging = true;
		onScrubStart();
		handlePointerMove(e);
	}

	function handlePointerUp(e: PointerEvent) {
		sliderRef.releasePointerCapture(e.pointerId);
		isDragging = false;
		onScrubEnd();
	}

	function handlePointerLeave() {
		if (!isDragging) {
			isHovered = false;
		}
	}

	let progressPercent = $derived((currentTime / duration) * 100 || 0);

	$effect(() => {
		if (isHovered || isDragging) {
			gsap.to(sliderRef, {
				height: 28,
				duration: 0.3,
				ease: "power4.out",
			});
			gsap.to(thumbRef, {
				opacity: 1,
				x: hoverX,
				duration: 0.1,
				overwrite: true,
			});
			gsap.to(hoverTimeRef, {
				opacity: 1,
				x: hoverX,
				duration: 0.1,
				overwrite: true,
			});
		} else {
			gsap.to(sliderRef, {
				height: 6,
				duration: 0.3,
				ease: "power4.out",
			});
			gsap.to(thumbRef, {
				opacity: 0,
				duration: 0.2,
			});
			gsap.to(hoverTimeRef, {
				opacity: 0,
				duration: 0.2,
			});
		}
	});
</script>

<div
	class={cn(
		"relative flex h-10 w-full touch-none items-center justify-center select-none",
		className,
	)}
	onpointerenter={() => (isHovered = true)}
	onpointerleave={handlePointerLeave}
	onpointermove={handlePointerMove}
	onpointerdown={handlePointerDown}
	onpointerup={handlePointerUp}
>
	<div
		bind:this={sliderRef}
		class="relative h-1.5 w-full overflow-hidden rounded-lg bg-fixed-light/10 backdrop-blur-md transition-[height]"
	>
		<div
			class="absolute inset-0 h-full w-full origin-left bg-fixed-light/20 backdrop-blur-md"
			style="transform: scaleX({progressPercent / 100})"
		></div>
	</div>

	<div
		bind:this={thumbRef}
		class="pointer-events-none absolute top-0 bottom-0 left-0 h-full w-px bg-accent opacity-0"
	></div>

	<div
		bind:this={hoverTimeRef}
		class="pointer-events-none absolute -top-8 left-0 -translate-x-1/2 rounded bg-fixed-light/10 px-1.5 py-0.5 font-mono text-[10px] leading-none text-fixed-light opacity-0 shadow-sm backdrop-blur-md"
	>
		{formatTime(hoverTime)}
	</div>
</div>
