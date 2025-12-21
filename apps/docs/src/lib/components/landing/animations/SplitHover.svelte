<script lang="ts">
	import gsap from "gsap";
	import SplitText from "gsap/SplitText";
	import type { Snippet } from "svelte";

	gsap.registerPlugin(SplitText);

	type ComponentProps = {
		children?: Snippet;
		class?: string;
		hoverTarget?: HTMLElement | null;
		[prop: string]: unknown;
	};

	const {
		children,
		class: className = "",
		hoverTarget = null,
		...restProps
	}: ComponentProps = $props();

	let wrapperRef: HTMLSpanElement;
	let originalSpan: HTMLSpanElement;
	let cloneSpan: HTMLSpanElement;
	let timeline: gsap.core.Timeline | null = null;

	function setupAnimation() {
		if (!wrapperRef || !originalSpan) return;

		timeline?.kill();

		cloneSpan.textContent = originalSpan.textContent;

		const originalSplit = SplitText.create(originalSpan, {
			type: "chars",
			charsClass: "inline-block",
		});

		const cloneSplit = SplitText.create(cloneSpan, {
			type: "chars",
			charsClass: "inline-block",
		});

		gsap.set(originalSplit.chars, { yPercent: 0 });
		gsap.set(cloneSplit.chars, { yPercent: 100 });

		timeline = gsap
			.timeline({ paused: true })
			.to(originalSplit.chars, {
				yPercent: -100,
				stagger: 0.02,
				duration: 0.5,
				ease: "power3.out",
			}, 0)
			.to(cloneSplit.chars, {
				yPercent: 0,
				stagger: 0.02,
				duration: 0.5,
				ease: "power3.out",
			}, 0);
	}

	$effect(() => {
		if (typeof window === "undefined") return;

		setupAnimation();

		const target = hoverTarget ?? wrapperRef;
		if (!target) return;

		const onEnter = () => timeline?.play();
		const onLeave = () => timeline?.reverse();

		target.addEventListener("mouseenter", onEnter);
		target.addEventListener("mouseleave", onLeave);

		return () => {
			target.removeEventListener("mouseenter", onEnter);
			target.removeEventListener("mouseleave", onLeave);
			timeline?.kill();
		};
});
</script>

<span
	{...restProps}
	class="relative inline-flex overflow-hidden align-baseline font-inherit text-inherit leading-none {className}"
	bind:this={wrapperRef}
>
	<span bind:this={originalSpan} class="inline-block">
		{@render children?.()}
	</span>
	<span
		bind:this={cloneSpan}
		class="pointer-events-none absolute inset-0 inline-block"
		aria-hidden="true"
	></span>
</span>
