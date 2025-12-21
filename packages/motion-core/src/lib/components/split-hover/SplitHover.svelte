<script lang="ts">
	import gsap from "gsap";
	import SplitText from "gsap/SplitText";
	import type { Snippet } from "svelte";
	import { motionCoreEase } from "../../helpers/gsap";
	import { cn } from "../../utils/cn";

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

	if (typeof window !== "undefined") {
		gsap.registerPlugin(SplitText);
	}

	let wrapperRef: HTMLSpanElement;
	let originalSpan: HTMLSpanElement;
	let cloneSpan: HTMLSpanElement;
	let originalSplit: SplitText | null = null;
	let cloneSplit: SplitText | null = null;

	$effect(() => {
		if (typeof window === "undefined") return;

		const node = hoverTarget ?? wrapperRef;
		if (!node || !originalSpan || !cloneSpan) return;

		let timeline: gsap.core.Timeline | null = null;

		originalSplit = SplitText.create(originalSpan, {
			type: "chars",
			charsClass: "inline-block",
			onSplit: (self) => {
				if (cloneSplit) cloneSplit.revert();
				cloneSplit = SplitText.create(cloneSpan, {
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
							ease: motionCoreEase,
						},
						0,
					)
					.to(
						cloneSplit.chars,
						{
							yPercent: 0,
							stagger: 0.02,
							duration: 0.35,
							ease: motionCoreEase,
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
		"relative inline-flex overflow-hidden align-baseline font-inherit text-inherit leading-none",
		className,
	)}
	bind:this={wrapperRef}
>
	<span bind:this={originalSpan}>
		{@render children?.()}
	</span>
	<span
		bind:this={cloneSpan}
		class="pointer-events-none absolute inset-0"
		aria-hidden="true"
	>
		{@render children?.()}
	</span>
</span>
