<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { CustomEase } from "gsap/dist/CustomEase";
	import { SplitText } from "gsap/dist/SplitText";
	import type { Snippet } from "svelte";
	import { onMount } from "svelte";
	import { cn } from "../../utils/cn";

	type SplitMode = "lines" | "words" | "chars";
	type ModeSettings = {
		duration?: number;
		stagger?: number;
	};
	type SplitRevealConfig = Partial<Record<SplitMode, ModeSettings>>;

	type ComponentProps = {
		children?: Snippet;
		class?: string;
		mode?: SplitMode;
		config?: SplitRevealConfig;
		as?: keyof HTMLElementTagNameMap;
		[prop: string]: unknown;
	};

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
	});
	CustomEase.create("motion-core-ease", "0.625, 0.05, 0, 1");

	const props: ComponentProps = $props();
	const children = $derived(props.children);
	const className = $derived(props.class ?? "");
	const mode = $derived<SplitMode>(props.mode ?? "lines");
	const as = $derived<keyof HTMLElementTagNameMap>(props.as ?? "div");

	const resolvedConfig = $derived.by(() => {
		const overrides = props.config?.[mode];
		const defaults = DEFAULT_CONFIG[mode];
		return {
			duration: overrides?.duration ?? defaults.duration,
			stagger: overrides?.stagger ?? defaults.stagger,
		};
	});

	const restProps = $derived(() => {
		const {
			children: _children,
			class: _class,
			mode: _mode,
			config: _config,
			as: _as,
			...rest
		} = props;
		return rest;
	});

	let wrapperRef: HTMLSpanElement | null = null;

	$effect(() => {
		const currentMode = mode;
		const config = resolvedConfig;
		const currentTag = as;

		if (typeof window === "undefined") return;
		const node = wrapperRef;
		if (!node) return;

		const split = SplitText.create(node, {
			type: "lines, words, chars",
			tag: currentTag,
			mask: "lines",
		});

		const targets =
			currentMode === "lines"
				? (split.lines ?? [])
				: currentMode === "words"
					? (split.words ?? [])
					: (split.chars ?? []);

		if (!targets.length) {
			split.revert();
			return;
		}

		gsap.set(targets, { yPercent: 110 });
		const tween = gsap.to(targets, {
			yPercent: 0,
			duration: config.duration,
			stagger: config.stagger,
			ease: "motion-core-ease",
			lazy: false,
		});

		return () => {
			tween.kill();
			split.revert();
		};
	});
</script>

<span
	{...restProps()}
	class={cn("font-inherit relative align-baseline text-inherit", className)}
	bind:this={wrapperRef}
>
	{@render children?.()}
</span>
