<script lang="ts">
	import { tick } from "svelte";
	import { gsap } from "gsap/dist/gsap";
	import { SplitText } from "gsap/dist/SplitText";
	import { onMount } from "svelte";
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn";

	interface ComponentProps {
		/**
		 * The content to be split and animated.
		 */
		children?: Snippet;
		/**
		 * The standard font weight when not hovering.
		 * @default 350
		 */
		baseWeight?: number;
		/**
		 * The target font weight for the character under the cursor.
		 * @default 750
		 */
		hoverWeight?: number;
		/**
		 * How many characters around the cursor are affected.
		 * @default 3
		 */
		influenceRadius?: number;
		/**
		 * Controls the curve of the weight drop-off. Higher values create a sharper peak.
		 * @default 1.5
		 */
		falloffPower?: number;
		/**
		 * Animation duration in seconds.
		 * @default 1.0
		 */
		duration?: number;
		/**
		 * GSAP easing function string.
		 * @default "power3.out"
		 */
		ease?: string;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		[prop: string]: unknown;
	}

	let {
		children,
		baseWeight = 350,
		hoverWeight = 750,
		influenceRadius = 3,
		falloffPower = 1.5,
		duration = 1.0,
		ease = "power3.out",
		class: className = "",
		...restProps
	}: ComponentProps = $props();

	onMount(() => {
		gsap.registerPlugin(SplitText);
	});

	let wrapperRef: HTMLSpanElement;
	let splitInstance: SplitText | null = null;
	let charNodes: HTMLElement[] = [];
	let charPositions: { x: number; width: number }[] = [];

	$effect(() => {
		if (typeof window === "undefined") return;
		if (!wrapperRef) return;

		if (splitInstance) {
			splitInstance.revert();
		}

		splitInstance = new SplitText(wrapperRef, {
			type: "chars",
			reduceWhiteSpace: false,
		});

		charNodes = (splitInstance.chars ?? []) as HTMLElement[];

		charNodes.forEach((node) => {
			node.style.fontWeight = String(baseWeight);
			node.style.fontVariationSettings = `"wght" ${baseWeight}`;
			node.style.display = "inline-block";

			if (!node.textContent?.trim()) {
				node.style.whiteSpace = "pre";
				node.style.pointerEvents = "none";
				node.style.minWidth = "0.25em";
			}
		});

		tick().then(() => {
			charPositions = charNodes.map((node) => {
				const rect = node.getBoundingClientRect();
				return {
					x: rect.left + rect.width / 2,
					width: rect.width,
				};
			});
		});

		const calculateWeight = (distance: number) => {
			if (influenceRadius <= 0) return baseWeight;
			if (distance > influenceRadius + 1) return baseWeight;

			const normalized = Math.max(0, 1 - distance / (influenceRadius + 1));
			const shaped = Math.pow(normalized, falloffPower);

			return baseWeight + (hoverWeight - baseWeight) * shaped;
		};

		const applyGsap = (node: HTMLElement, weight: number) => {
			gsap.to(node, {
				fontWeight: weight,
				fontVariationSettings: `"wght" ${weight}`,
				duration,
				ease,
				overwrite: "auto",
			});
		};

		const animateWeights = (targetIndex: number | null) => {
			if (!charNodes.length) return;
			charNodes.forEach((node, i) => {
				const weight =
					targetIndex === null
						? baseWeight
						: calculateWeight(Math.abs(i - targetIndex));
				applyGsap(node, weight);
			});
		};

		const handleMove = (e: PointerEvent) => {
			if (!charNodes.length || !charPositions.length) return;
			const mouseX = e.clientX;

			charNodes.forEach((node, i) => {
				const { x, width } = charPositions[i];
				const distance = Math.abs(mouseX - x) / width;
				const weight = calculateWeight(distance);
				applyGsap(node, weight);
			});
		};

		const handleLeave = () => {
			animateWeights(null);
		};

		wrapperRef.addEventListener("pointermove", handleMove);
		wrapperRef.addEventListener("pointerleave", handleLeave);

		return () => {
			wrapperRef.removeEventListener("pointermove", handleMove);
			wrapperRef.removeEventListener("pointerleave", handleLeave);
			splitInstance?.revert();
			splitInstance = null;
			charNodes = [];
			charPositions = [];
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
