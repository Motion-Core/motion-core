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
	let safeDuration = $derived(Math.max(duration || 0, 0));
	let safeCurrentTime = $derived(
		Math.max(0, Math.min(currentTime || 0, safeDuration)),
	);
	let isDisabled = $derived(safeDuration <= 0);

	function formatTime(seconds: number) {
		if (!Number.isFinite(seconds) || seconds < 0) return "00:00";
		const m = Math.floor(seconds / 60);
		const s = Math.floor(seconds % 60);
		return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
	}

	function handlePointerMove(e: PointerEvent) {
		if (isDisabled) return;
		const bounds = sliderRef.getBoundingClientRect();
		if (bounds.width <= 0) return;
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
		if (isDisabled) return;
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

	function clampTime(time: number) {
		return Math.max(0, Math.min(time, safeDuration));
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (isDisabled) return;

		const smallStep = Math.max(safeDuration / 100, 0.1);
		const largeStep = Math.max(safeDuration / 10, 1);
		let nextTime: number | null = null;

		switch (e.key) {
			case "ArrowLeft":
			case "ArrowDown":
				nextTime = safeCurrentTime - smallStep;
				break;
			case "ArrowRight":
			case "ArrowUp":
				nextTime = safeCurrentTime + smallStep;
				break;
			case "PageDown":
				nextTime = safeCurrentTime - largeStep;
				break;
			case "PageUp":
				nextTime = safeCurrentTime + largeStep;
				break;
			case "Home":
				nextTime = 0;
				break;
			case "End":
				nextTime = safeDuration;
				break;
		}

		if (nextTime === null) return;

		e.preventDefault();
		onScrubStart();
		onSeek(clampTime(nextTime));
		onScrubEnd();
	}

	let progressPercent = $derived(
		safeDuration > 0 ? (safeCurrentTime / safeDuration) * 100 : 0,
	);

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
	role="slider"
	tabindex={isDisabled ? -1 : 0}
	aria-label="Video timeline"
	aria-disabled={isDisabled}
	aria-valuemin={0}
	aria-valuemax={safeDuration}
	aria-valuenow={safeCurrentTime}
	aria-valuetext={formatTime(safeCurrentTime)}
	onpointerenter={() => (isHovered = true)}
	onpointerleave={handlePointerLeave}
	onpointermove={handlePointerMove}
	onpointerdown={handlePointerDown}
	onpointerup={handlePointerUp}
	onkeydown={handleKeyDown}
>
	<div
		bind:this={sliderRef}
		class="bg-fixed-light/10 relative h-1.5 w-full overflow-hidden rounded-lg backdrop-blur-md transition-[height]"
	>
		<div
			class="bg-fixed-light/20 absolute inset-0 h-full w-full origin-left backdrop-blur-md"
			style="transform: scaleX({progressPercent / 100})"
		></div>
	</div>

	<div
		bind:this={thumbRef}
		class="pointer-events-none absolute top-0 bottom-0 left-0 h-full w-px bg-accent opacity-0"
	></div>

	<div
		bind:this={hoverTimeRef}
		class="bg-fixed-light/10 pointer-events-none absolute -top-8 left-0 -translate-x-1/2 rounded px-1.5 py-0.5 font-mono text-[10px] leading-none text-white-fixed opacity-0 shadow-sm backdrop-blur-md"
	>
		{formatTime(hoverTime)}
	</div>
</div>
