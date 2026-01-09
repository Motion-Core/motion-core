<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { SplitText } from "gsap/dist/SplitText";
	import { onMount } from "svelte";
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn";

	interface ComponentProps {
		/**
		 * The content that will scramble on hover.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * An optional external element that triggers the hover effect.
		 * @default null
		 */
		hoverTarget?: HTMLElement | null;
		/**
		 * Total duration of the scramble animation (in seconds).
		 * @default 0.6
		 */
		scrambleDuration?: number;
		/**
		 * Delay between each character's animation start (in seconds).
		 * @default 0.03
		 */
		stagger?: number;
		/**
		 * Number of scramble steps each character goes through before settling.
		 * @default 12
		 */
		cycles?: number;
		/**
		 * Characters used while scrambling. Defaults to a mix of letters, numbers, and symbols.
		 */
		characters?: string;
		[prop: string]: unknown;
	}

	const {
		children,
		class: className = "",
		hoverTarget = null,
		scrambleDuration = 0.6,
		stagger = 0.03,
		cycles = 12,
		characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*",
		...restProps
	}: ComponentProps = $props();

	onMount(() => {
		gsap.registerPlugin(SplitText);
	});

	let wrapperRef: HTMLSpanElement;
	let splitInstance: SplitText | null = null;
	let hoverTimeline: gsap.core.Timeline | null = null;

	const getRandomChar = (pool: string) => {
		if (!pool.length) return "";
		const index = Math.floor(Math.random() * pool.length);
		return pool[index] ?? "";
	};

	const createScrambleTimeline = (nodes: HTMLElement[]) => {
		if (!nodes.length) return null;

		const pool = characters.length
			? characters
			: "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
		const timeline = gsap.timeline({ paused: true });
		const totalDuration = Math.max(0.1, scrambleDuration);
		const stepCount = Math.max(1, Math.floor(cycles));
		const stepDuration = totalDuration / stepCount;

		nodes.forEach((node, index) => {
			const finalChar = node.dataset.originalChar ?? node.textContent ?? "";
			const charTimeline = gsap.timeline();

			if (finalChar.trim().length === 0) {
				charTimeline.call(() => {
					node.textContent = finalChar;
				});
			} else {
				for (let i = 0; i < stepCount; i += 1) {
					charTimeline.call(() => {
						node.textContent = getRandomChar(pool);
					});
					charTimeline.to({}, { duration: stepDuration });
				}
				charTimeline.call(() => {
					node.textContent = finalChar;
				});
			}

			timeline.add(charTimeline, index * stagger);
		});

		return timeline;
	};

	$effect(() => {
		if (typeof window === "undefined") return;
		if (!wrapperRef) return;

		const target = hoverTarget ?? wrapperRef;
		if (!target) return;

		hoverTimeline?.kill();
		hoverTimeline = null;
		splitInstance?.revert();

		splitInstance = SplitText.create(wrapperRef, {
			type: "chars",
			reduceWhiteSpace: false,
			charsClass: "inline-block",
		});

		const charNodes = (splitInstance.chars ?? []) as HTMLElement[];

		charNodes.forEach((node) => {
			node.style.display = "inline-block";
			node.dataset.originalChar = node.textContent ?? "";

			if (!node.textContent?.trim()) {
				node.style.whiteSpace = "pre";
				node.style.pointerEvents = "none";
			}
		});

		hoverTimeline = createScrambleTimeline(charNodes);

		const handleEnter = () => {
			if (!hoverTimeline) {
				hoverTimeline = createScrambleTimeline(charNodes);
			}

			hoverTimeline?.restart();
		};

		const handleLeave = () => {
			hoverTimeline?.progress(1);
		};

		target.addEventListener("mouseenter", handleEnter);
		target.addEventListener("mouseleave", handleLeave);

		return () => {
			target.removeEventListener("mouseenter", handleEnter);
			target.removeEventListener("mouseleave", handleLeave);
			hoverTimeline?.kill();
			hoverTimeline = null;
			splitInstance?.revert();
			splitInstance = null;
		};
	});
</script>

<span
	{...restProps}
	class={cn("font-inherit inline-block align-baseline text-inherit", className)}
	bind:this={wrapperRef}
>
	{@render children?.()}
</span>
