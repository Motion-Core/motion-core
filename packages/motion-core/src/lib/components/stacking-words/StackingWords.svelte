<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { ScrollTrigger } from "gsap/dist/ScrollTrigger";
	import { SplitText } from "gsap/dist/SplitText";
	import { onMount } from "svelte";
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn";

	interface Props {
		/**
		 * Text/content to split into lines and words.
		 */
		children?: Snippet;
		/**
		 * Additional CSS classes for the wrapper.
		 */
		class?: string;
		/**
		 * ScrollTrigger start position.
		 * @default "top 90%"
		 */
		start?: string;
		/**
		 * ScrollTrigger end position.
		 * @default "top 30%"
		 */
		end?: string;
		/**
		 * ScrollTrigger scrub value.
		 * @default 1.234
		 */
		scrub?: boolean | number;
		/**
		 * Stagger applied across words inside each line.
		 * @default 0.21
		 */
		stagger?: number;
		/**
		 * Easing used for word translation.
		 * @default "power3.out"
		 */
		ease?: string;
		/**
		 * The element to use as the scroller. Defaults to window.
		 */
		scrollElement?: string | HTMLElement | null;
		[prop: string]: unknown;
	}

	let {
		children,
		class: className = "",
		start = "top 90%",
		end = "top 30%",
		scrub = 1.234,
		stagger = 0.21,
		ease = "power3.out",
		scrollElement,
		...restProps
	}: Props = $props();

	let wrapperRef: HTMLElement | null = null;
	let splitInstance: SplitText | null = null;
	let lineTweens: gsap.core.Tween[] = [];

	onMount(() => {
		gsap.registerPlugin(ScrollTrigger);
		gsap.registerPlugin(SplitText);
	});

	function killLineTweens() {
		lineTweens.forEach((tween) => tween.kill());
		lineTweens = [];
	}

	async function waitForLayout() {
		await document.fonts.ready;
		await new Promise<void>((resolve) =>
			requestAnimationFrame(() => resolve()),
		);
		await new Promise<void>((resolve) =>
			requestAnimationFrame(() => resolve()),
		);
	}

	$effect(() => {
		if (typeof window === "undefined") return;
		const node = wrapperRef;
		if (!node) return;
		const triggerStart = start;
		const triggerEnd = end;
		const triggerScrub = scrub;
		const wordStagger = stagger;
		const wordEase = ease;
		const triggerScroller = scrollElement || window;

		let cancelled = false;

		const init = async () => {
			await waitForLayout();
			if (cancelled || !wrapperRef) return;

			splitInstance?.revert();
			killLineTweens();

			splitInstance = SplitText.create(wrapperRef, {
				aria: "hidden",
				autoSplit: true,
				linesClass: "stacking-words-line",
				onSplit: (self) => {
					killLineTweens();

					const words = (self.words ?? []) as HTMLElement[];
					words.forEach((word) => {
						gsap.set(word, {
							x: window.innerWidth - word.getBoundingClientRect().left,
						});
					});

					(self.lines ?? []).forEach((line) => {
						const tween = gsap.to(
							line.querySelectorAll(".stacking-words-word"),
							{
								ease: wordEase,
								stagger: wordStagger,
								x: 0,
								scrollTrigger: {
									trigger: line,
									start: triggerStart,
									end: triggerEnd,
									scrub: triggerScrub,
									scroller: triggerScroller,
									invalidateOnRefresh: true,
								},
							},
						);
						lineTweens.push(tween);
					});

					ScrollTrigger.refresh();
				},
				tag: "span",
				type: "lines, words",
				wordsClass: "stacking-words-word",
			});

			gsap.set(wrapperRef, { autoAlpha: 1 });
		};

		void init();

		return () => {
			cancelled = true;
			killLineTweens();
			splitInstance?.revert();
			splitInstance = null;
		};
	});
</script>

<div
	{...restProps}
	class={cn("stacking-words", className)}
	bind:this={wrapperRef}
>
	{@render children?.()}
</div>

<style>
	.stacking-words {
		visibility: hidden;
	}

	.stacking-words :global(.stacking-words-line),
	.stacking-words :global(.stacking-words-line-mask) {
		display: block;
	}

	.stacking-words :global(.stacking-words-word) {
		display: inline-block;
		will-change: transform;
	}
</style>
