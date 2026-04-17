<script lang="ts" generics="T">
	import { onDestroy } from "svelte";
	import { gsap } from "gsap";
	import { cn } from "../../utils/cn";
	import type { Snippet } from "svelte";

	interface Props<T> {
		/**
		 * Items rendered as cards in the stack.
		 */
		items: T[];
		/**
		 * Snippet used to render each card. Receives item and original index.
		 */
		children: Snippet<[T, number]>;
		/**
		 * Vertical spacing between cards.
		 * @default 8
		 */
		stackOffset?: number;
		/**
		 * Rotation step (deg) for each card below the top card.
		 * @default -10
		 */
		stackRotation?: number;
		/**
		 * Minimum drag distance required to send the top card to the back.
		 * @default 80
		 */
		dragThreshold?: number;
		/**
		 * Stack animation duration in seconds.
		 * @default 0.3
		 */
		duration?: number;
		/**
		 * GSAP ease string.
		 * @default "power2.out"
		 */
		ease?: string;
		/**
		 * Additional CSS classes for the root container.
		 */
		class?: string;
		[prop: string]: unknown;
	}

	interface CardTransform {
		zIndex: number;
		y: number;
		rotation: number;
		scale: number;
	}

	interface DragState {
		pointerId: number;
		startX: number;
		startY: number;
		currentX: number;
		currentY: number;
	}

	let {
		items,
		children,
		stackOffset = 8,
		stackRotation = -10,
		dragThreshold = 80,
		duration = 0.3,
		ease = "power2.out",
		class: className = "",
		...restProps
	}: Props<T> = $props();

	let cardRefs = $state<Array<HTMLDivElement | null>>([]);
	let hasInitialized = false;
	let draggingCardIndex = $state<number | null>(null);
	let cardOrder = $state<number[]>([]);
	let dragState: DragState | null = null;

	const topCardIndex = $derived(cardOrder[cardOrder.length - 1] ?? -1);
	const resolvedEase = $derived(ease);

	function getCardTransform(cardIndex: number): CardTransform | null {
		const stackPosition = cardOrder.indexOf(cardIndex);
		if (stackPosition === -1) return null;

		const positionFromBottom = cardOrder.length - 1 - stackPosition;
		return {
			zIndex: stackPosition + 1,
			y: -positionFromBottom * stackOffset,
			rotation: positionFromBottom * stackRotation,
			scale: 1 - positionFromBottom * 0.02,
		};
	}

	function setCardRef(index: number, node: HTMLDivElement | null) {
		const next = [...cardRefs];
		next[index] = node;
		cardRefs = next;
	}

	function registerCardRef(node: HTMLDivElement, index: number) {
		setCardRef(index, node);
		return {
			update(nextIndex: number) {
				if (nextIndex === index) return;
				setCardRef(index, null);
				index = nextIndex;
				setCardRef(index, node);
			},
			destroy() {
				setCardRef(index, null);
			},
		};
	}

	function animateStack(immediate: boolean) {
		for (const [index] of items.entries()) {
			const node = cardRefs[index];
			if (!node) continue;

			const transform = getCardTransform(index);
			if (!transform) continue;
			if (draggingCardIndex === index) continue;

			const vars = {
				x: 0,
				y: transform.y,
				rotation: transform.rotation,
				scale: transform.scale,
			};

			if (immediate) {
				gsap.set(node, vars);
			} else {
				gsap.to(node, {
					...vars,
					duration,
					ease: resolvedEase,
					overwrite: true,
				});
			}
		}
	}

	function resetDraggedCard(node: HTMLDivElement) {
		gsap.to(node, {
			x: 0,
			y: 0,
			rotation: 0,
			scale: 1,
			duration,
			ease: resolvedEase,
			overwrite: true,
		});
	}

	function handlePointerDown(event: PointerEvent, cardIndex: number) {
		if (cardIndex !== topCardIndex) return;
		const node = cardRefs[cardIndex];
		if (!node) return;

		event.preventDefault();
		draggingCardIndex = cardIndex;
		dragState = {
			pointerId: event.pointerId,
			startX: event.clientX,
			startY: event.clientY,
			currentX: 0,
			currentY: 0,
		};

		try {
			node.setPointerCapture(event.pointerId);
		} catch {
			// Some pointer sources may not allow capture; drag still works.
		}

		gsap.killTweensOf(node);
		gsap.to(node, {
			scale: 1.05,
			rotation: 0,
			duration: 0.05,
			ease: "power2.out",
			overwrite: true,
		});
	}

	function handlePointerMove(event: PointerEvent, cardIndex: number) {
		if (cardIndex !== draggingCardIndex || !dragState) return;
		if (event.pointerId !== dragState.pointerId) return;

		const node = cardRefs[cardIndex];
		if (!node) return;

		const dx = event.clientX - dragState.startX;
		const dy = event.clientY - dragState.startY;
		const x = gsap.utils.clamp(-150, 150, dx);
		const y = gsap.utils.clamp(-150, 150, dy);

		dragState.currentX = x;
		dragState.currentY = y;

		gsap.set(node, {
			x,
			y,
			rotation: 0,
			scale: 1.05,
		});
	}

	function handlePointerEnd(event: PointerEvent, cardIndex: number) {
		if (cardIndex !== draggingCardIndex || !dragState) return;
		if (event.pointerId !== dragState.pointerId) return;

		const node = cardRefs[cardIndex];
		if (!node) {
			dragState = null;
			draggingCardIndex = null;
			return;
		}

		try {
			if (node.hasPointerCapture(event.pointerId)) {
				node.releasePointerCapture(event.pointerId);
			}
		} catch {
			// Ignore capture-release failures from unsupported pointer sources.
		}

		const dragDistance =
			Math.abs(dragState.currentX) + Math.abs(dragState.currentY);
		const shouldMoveToBack = dragDistance >= dragThreshold;

		dragState = null;
		draggingCardIndex = null;

		if (!shouldMoveToBack) {
			resetDraggedCard(node);
			return;
		}

		const draggedPosition = cardOrder.indexOf(cardIndex);
		if (draggedPosition === -1) {
			resetDraggedCard(node);
			return;
		}

		const nextOrder = [...cardOrder];
		const [dragged] = nextOrder.splice(draggedPosition, 1);
		if (dragged === undefined) {
			resetDraggedCard(node);
			return;
		}

		nextOrder.unshift(dragged);
		cardOrder = nextOrder;
	}

	$effect.pre(() => {
		const allIndexes = items.map((_, index) => index);
		const isSameSet =
			allIndexes.length === cardOrder.length &&
			allIndexes.every((index) => cardOrder.includes(index));
		if (!isSameSet) {
			cardOrder = allIndexes;
		}
	});

	$effect(() => {
		if (!items.length) {
			hasInitialized = false;
			return;
		}

		const allReady = items.every((_, index) => Boolean(cardRefs[index]));
		if (!allReady) return;

		animateStack(!hasInitialized);
		hasInitialized = true;
	});

	onDestroy(() => {
		for (const node of cardRefs) {
			if (node) {
				gsap.killTweensOf(node);
			}
		}
	});
</script>

<div
	class={cn("relative inline-grid [perspective:1000px]", className)}
	{...restProps}
>
	{#if items.length > 0}
		{#each cardOrder as cardIndex (`card-${cardIndex}`)}
			{@const item = items[cardIndex]}
			{@const transform = getCardTransform(cardIndex)}
			{#if item}
				<div
					use:registerCardRef={cardIndex}
					class="col-start-1 row-start-1 select-none"
					role="presentation"
					style={`z-index:${draggingCardIndex === cardIndex ? items.length + 10 : (transform?.zIndex ?? 1)};touch-action:${cardIndex === topCardIndex ? "none" : "auto"};cursor:${cardIndex === topCardIndex ? (draggingCardIndex === cardIndex ? "grabbing" : "grab") : "default"};`}
					onpointerdown={(event) => handlePointerDown(event, cardIndex)}
					onpointermove={(event) => handlePointerMove(event, cardIndex)}
					onpointerup={(event) => handlePointerEnd(event, cardIndex)}
					onpointercancel={(event) => handlePointerEnd(event, cardIndex)}
				>
					{@render children(item, cardIndex)}
				</div>
			{/if}
		{/each}
	{/if}
</div>
