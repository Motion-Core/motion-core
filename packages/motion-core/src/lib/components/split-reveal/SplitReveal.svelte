<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { CustomEase } from "gsap/dist/CustomEase";
	import { SplitText } from "gsap/dist/SplitText";
	import { ScrollTrigger } from "gsap/dist/ScrollTrigger";
	import type { Snippet } from "svelte";
	import { onMount } from "svelte";
	import { cn } from "../../utils/cn";

	type SplitMode = "lines" | "words" | "chars";

	interface ModeSettings {
		duration?: number;
		stagger?: number;
	}

	type SplitRevealConfig = Partial<Record<SplitMode, ModeSettings>>;

	interface ComponentProps {
		/**
		 * The content to be split and revealed.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * The splitting mode: 'lines', 'words', or 'chars'.
		 * @default "lines"
		 */
		mode?: SplitMode;
		/**
		 * Configuration for animation duration and stagger for each mode.
		 */
		config?: SplitRevealConfig;
		/**
		 * Delay before the animation starts (in seconds).
		 * @default 0
		 */
		delay?: number;
		/**
		 * Whether to trigger the animation on scroll.
		 * @default false
		 */
		triggerOnScroll?: boolean;
		/**
		 * The element to use as the scroll trigger (optional).
		 */
		scrollElement?: string | HTMLElement | null;
		/**
		 * The HTML tag to use for the wrapper.
		 * @default "div"
		 */
		as?: keyof HTMLElementTagNameMap;
		[prop: string]: unknown;
	}

	type RequiredConfig = Record<
		SplitMode,
		{ duration: number; stagger: number }
	>;

	const DEFAULT_CONFIG: RequiredConfig = {
		lines: { duration: 0.8, stagger: 0.08 },
		words: { duration: 0.6, stagger: 0.06 },
		chars: { duration: 0.4, stagger: 0.008 },
	};

	onMount(() => {
		gsap.registerPlugin(SplitText);
		gsap.registerPlugin(CustomEase);
		gsap.registerPlugin(ScrollTrigger);
		CustomEase.create("motion-core-ease", "0.625, 0.05, 0, 1");
	});

	let {
		children,
		class: className = "",
		mode = "lines" as SplitMode,
		config,
		as = "div" as keyof HTMLElementTagNameMap,
		delay = 0,
		triggerOnScroll = false,
		scrollElement,
		...restProps
	}: ComponentProps = $props();

	const resolvedConfig = $derived.by(() => {
		const overrides = config?.[mode];
		const defaults = DEFAULT_CONFIG[mode];
		return {
			duration: overrides?.duration ?? defaults.duration,
			stagger: overrides?.stagger ?? defaults.stagger,
		};
	});

	let wrapperRef: HTMLSpanElement | null = null;

	$effect(() => {
		if (typeof window === "undefined") return;

		const node = wrapperRef;
		if (!node) return;

		const split = SplitText.create(node, {
			type: "lines, words, chars",
			tag: as,
			mask: "lines",
		});

		const targets =
			mode === "lines"
				? (split.lines ?? [])
				: mode === "words"
					? (split.words ?? [])
					: (split.chars ?? []);

		if (!targets.length) {
			split.revert();
			return;
		}

		gsap.set(targets, { yPercent: 110 });

		const tween = gsap.to(targets, {
			yPercent: 0,
			duration: resolvedConfig.duration,
			stagger: resolvedConfig.stagger,
			ease: "motion-core-ease",
			lazy: false,
			delay: delay,
			scrollTrigger: triggerOnScroll
				? {
						trigger: node,
						scroller: scrollElement,
						start: "top 85%",
					}
				: undefined,
		});

		return () => {
			tween.kill();
			split.revert();
		};
	});
</script>

<span
	{...restProps}
	class={cn("font-inherit relative align-baseline text-inherit", className)}
	bind:this={wrapperRef}
>
	{@render children?.()}
</span>
