<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { SplitText } from "gsap/dist/SplitText";
	import { onMount } from "svelte";
	import type { Snippet } from "svelte";
	import { ensureMotionCoreEase, registerPluginOnce } from "../../helpers/gsap";
	import { cn } from "../../utils/cn";

	interface ComponentProps {
		/**
		 * The content to duplicate and animate on hover.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * An optional external element that triggers the hover effect.
		 * If null, the component's wrapper triggers the effect.
		 * @default null
		 */
		hoverTarget?: HTMLElement | null;
		[prop: string]: unknown;
	}

	let {
		children,
		class: className = "",
		hoverTarget = null,
		...restProps
	}: ComponentProps = $props();

	onMount(() => {
		registerPluginOnce(SplitText);
		ensureMotionCoreEase();
	});

	let wrapperRef: HTMLSpanElement | undefined;
	let originalSpan: HTMLSpanElement | undefined;
	let cloneSpan: HTMLSpanElement | undefined;
	let originalSplit: SplitText | null = null;
	let cloneSplit: SplitText | null = null;

	const attachWrapperRef = (node: HTMLSpanElement) => {
		wrapperRef = node;
		return () => {
			if (wrapperRef === node) {
				wrapperRef = undefined;
			}
		};
	};

	const attachOriginalSpan = (node: HTMLSpanElement) => {
		originalSpan = node;
		return () => {
			if (originalSpan === node) {
				originalSpan = undefined;
			}
		};
	};

	const attachCloneSpan = (node: HTMLSpanElement) => {
		cloneSpan = node;
		return () => {
			if (cloneSpan === node) {
				cloneSpan = undefined;
			}
		};
	};

	$effect(() => {
		if (typeof window === "undefined") return;

		const node = hoverTarget ?? wrapperRef;
		if (!node || !originalSpan || !cloneSpan) return;

		let timeline: gsap.core.Timeline | null = null;

		originalSplit = SplitText.create(originalSpan, {
			type: "chars",
			charsClass: "inline-block",
			onSplit: (self) => {
				const cloneNode = cloneSpan;
				if (!cloneNode) return;

				if (cloneSplit) cloneSplit.revert();
				cloneSplit = SplitText.create(cloneNode, {
					type: "chars",
					charsClass: "inline-block",
				});

				gsap.set(self.chars, { yPercent: 0 });
				gsap.set(cloneSplit.chars, { yPercent: 100 });

				timeline?.kill();
				timeline = gsap
					.timeline({ paused: true })
					.to(
						self.chars,
						{
							yPercent: -100,
							stagger: 0.02,
							duration: 0.35,
							ease: "motion-core-ease",
						},
						0,
					)
					.to(
						cloneSplit.chars,
						{
							yPercent: 0,
							stagger: 0.02,
							duration: 0.35,
							ease: "motion-core-ease",
						},
						0,
					);

				return timeline;
			},
		});

		const handleEnter = () => timeline?.play();
		const handleLeave = () => timeline?.reverse();

		node.addEventListener("mouseenter", handleEnter);
		node.addEventListener("mouseleave", handleLeave);

		return () => {
			node.removeEventListener("mouseenter", handleEnter);
			node.removeEventListener("mouseleave", handleLeave);
			timeline?.kill();
			originalSplit?.revert();
			cloneSplit?.revert();
		};
	});
</script>

<span
	{...restProps}
	class={cn(
		"font-inherit relative inline-flex overflow-hidden align-baseline leading-none text-inherit",
		className,
	)}
	{@attach attachWrapperRef}
>
	<span {@attach attachOriginalSpan}>
		{@render children?.()}
	</span>
	<span
		{@attach attachCloneSpan}
		class="pointer-events-none absolute inset-0"
		aria-hidden="true"
	>
		{@render children?.()}
	</span>
</span>
