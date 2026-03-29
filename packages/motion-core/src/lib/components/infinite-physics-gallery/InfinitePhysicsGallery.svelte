<script lang="ts">
	import { gsap } from "gsap/dist/gsap";
	import { cn } from "../../utils/cn";
	import type { InfinitePhysicsGalleryItem } from "./types";

	interface ScrollSettings {
		/**
		 * Wheel speed multiplier.
		 * @default 2
		 */
		wheelSpeed?: number;
		/**
		 * Additional friction applied to inertia movement.
		 * @default 0.95
		 */
		friction?: number;
		/**
		 * Edge threshold in pixels where auto-scroll starts.
		 * @default 100
		 */
		edgeThreshold?: number;
		/**
		 * Edge auto-scroll acceleration.
		 * @default 2
		 */
		edgeScrollSpeed?: number;
	}

	interface Props {
		/**
		 * Gallery items (images/videos).
		 */
		items?: InfinitePhysicsGalleryItem[];
		/**
		 * Cell width in pixels.
		 * @default 320
		 */
		cellWidth?: number;
		/**
		 * Cell height in pixels.
		 * @default 400
		 */
		cellHeight?: number;
		/**
		 * Gap between cells in pixels.
		 * @default 24
		 */
		gap?: number;
		/**
		 * Base inertia friction.
		 * @default 0.95
		 */
		friction?: number;
		/**
		 * Additional classes applied to each media cell wrapper.
		 */
		cellClass?: string;
		/**
		 * Scroll and edge-behavior settings.
		 */
		scrollSettings?: ScrollSettings;
		/**
		 * Additional root classes.
		 */
		class?: string;
		[prop: string]: unknown;
	}

	type Dimensions = {
		cellWidth: number;
		cellHeight: number;
		gap: number;
	};

	type VisibleCell = {
		id: string;
		item: InfinitePhysicsGalleryItem;
		posX: number;
		posY: number;
	};

	const DEFAULT_SCROLL_SETTINGS: Required<ScrollSettings> = {
		wheelSpeed: 2,
		friction: 0.95,
		edgeThreshold: 100,
		edgeScrollSpeed: 2,
	};

	const MIN_POINTER_DT = 8;
	const WHEEL_DELTA_LIMIT = 220;
	const WHEEL_IMPULSE = 0.04;
	const DRAG_VELOCITY_BLEND = 0.3;
	const EDGE_RESPONSE = 0.22;
	const EDGE_VELOCITY_MULTIPLIER = 2.2;
	const MAX_VELOCITY = 24;

	let {
		items = [],
		cellWidth = 320,
		cellHeight = 400,
		gap = 24,
		friction = 0.95,
		cellClass = "",
		scrollSettings = {},
		class: className = "",
		...restProps
	}: Props = $props();

	let containerRef: HTMLDivElement | null = null;

	const attachContainerRef = (node: HTMLDivElement) => {
		containerRef = node;
		return () => {
			if (containerRef === node) {
				containerRef = null;
			}
		};
	};

	let viewportSize = $state({ width: 0, height: 0 });
	let isDragging = $state(false);
	let offset = $state({ x: 0, y: 0 });

	const offsetRef = { x: 0, y: 0 };
	const velocityRef = { x: 0, y: 0 };
	const edgeVelocityRef = { x: 0, y: 0 };
	const mousePosRef = { x: 0, y: 0 };

	let pointerId: number | null = null;
	let lastPointer = { x: 0, y: 0 };
	let lastPointerTime = 0;
	let draggingRef = false;

	const normalizedItems = $derived(items);

	const resolvedScrollSettings = $derived({
		...DEFAULT_SCROLL_SETTINGS,
		...scrollSettings,
	});

	const responsiveDimensions = $derived.by<Dimensions>(() => {
		const width = viewportSize.width;

		if (width > 0 && width < 640) {
			return {
				cellWidth: Math.min(cellWidth * 0.7, width * 0.85),
				cellHeight: Math.min(cellHeight * 0.7, width * 1.1),
				gap: Math.max(gap * 0.5, 12),
			};
		}

		if (width > 0 && width < 1024) {
			return {
				cellWidth: Math.min(cellWidth * 0.85, width * 0.6),
				cellHeight: Math.min(cellHeight * 0.85, width * 0.75),
				gap: Math.max(gap * 0.75, 16),
			};
		}

		return {
			cellWidth,
			cellHeight,
			gap,
		};
	});

	const totalCellWidth = $derived(
		responsiveDimensions.cellWidth + responsiveDimensions.gap,
	);
	const totalCellHeight = $derived(
		responsiveDimensions.cellHeight + responsiveDimensions.gap,
	);

	const startX = $derived(Math.floor(-offset.x / totalCellWidth) - 1);
	const endX = $derived(
		Math.ceil((viewportSize.width - offset.x) / totalCellWidth) + 1,
	);
	const startY = $derived(Math.floor(-offset.y / totalCellHeight) - 1);
	const endY = $derived(
		Math.ceil((viewportSize.height - offset.y) / totalCellHeight) + 1,
	);

	const visibleCells = $derived.by<VisibleCell[]>(() => {
		const resolvedItems = normalizedItems;
		if (!resolvedItems.length) return [];

		const cells: VisibleCell[] = [];
		const mod = (value: number, divisor: number) =>
			((value % divisor) + divisor) % divisor;

		for (let i = startX; i <= endX; i += 1) {
			for (let j = startY; j <= endY; j += 1) {
				const itemIndex = mod(i + j * 7, resolvedItems.length);
				const item = resolvedItems[itemIndex];
				cells.push({
					id: `${i}-${j}`,
					item,
					posX: i * totalCellWidth + offset.x,
					posY: j * totalCellHeight + offset.y,
				});
			}
		}

		return cells;
	});

	function resolveMediaSrc(item: InfinitePhysicsGalleryItem) {
		if (item.type === "image") {
			return item.image.src;
		}
		return item.video.src;
	}

	function resolveAlt(item: InfinitePhysicsGalleryItem) {
		return item.type === "image" ? item.image.alt || "" : "";
	}

	function updateViewport() {
		const node = containerRef;
		if (!node) return;
		viewportSize = {
			width: node.clientWidth,
			height: node.clientHeight,
		};
	}

	function commitOffset() {
		offset = {
			x: offsetRef.x,
			y: offsetRef.y,
		};
	}

	function clamp(value: number, min: number, max: number) {
		return Math.max(min, Math.min(max, value));
	}

	function damp(
		current: number,
		target: number,
		smoothing: number,
		deltaFrames: number,
	) {
		const factor = 1 - Math.pow(1 - smoothing, deltaFrames);
		return current + (target - current) * factor;
	}

	function edgeInfluence(position: number, size: number, threshold: number) {
		if (position <= 0 || size <= 0 || threshold <= 0) return 0;
		if (position < threshold) return 1 - position / threshold;
		if (position > size - threshold) return -1 + (size - position) / threshold;
		return 0;
	}

	function setMousePosition(clientX: number, clientY: number) {
		const node = containerRef;
		if (!node) return;
		const rect = node.getBoundingClientRect();
		const local = {
			x: clientX - rect.left,
			y: clientY - rect.top,
		};
		mousePosRef.x = local.x;
		mousePosRef.y = local.y;
	}

	function resetMousePosition() {
		mousePosRef.x = 0;
		mousePosRef.y = 0;
	}

	function handlePointerDown(event: PointerEvent) {
		if (event.button !== 0 && event.pointerType !== "touch") return;
		if (!containerRef) return;

		draggingRef = true;
		isDragging = true;
		pointerId = event.pointerId;
		velocityRef.x = 0;
		velocityRef.y = 0;
		edgeVelocityRef.x = 0;
		edgeVelocityRef.y = 0;

		lastPointer = {
			x: event.clientX,
			y: event.clientY,
		};
		lastPointerTime = Date.now();

		setMousePosition(event.clientX, event.clientY);

		try {
			containerRef.setPointerCapture(event.pointerId);
		} catch {
			// Some pointer sources may not support capture.
		}
	}

	function handlePointerMove(event: PointerEvent) {
		setMousePosition(event.clientX, event.clientY);

		if (!draggingRef) return;
		if (pointerId !== event.pointerId) return;

		const now = Date.now();
		const dt = Math.max(now - lastPointerTime, MIN_POINTER_DT);
		const dx = event.clientX - lastPointer.x;
		const dy = event.clientY - lastPointer.y;

		offsetRef.x += dx;
		offsetRef.y += dy;
		const pointerVelocityX = dx * (16 / dt);
		const pointerVelocityY = dy * (16 / dt);
		velocityRef.x =
			velocityRef.x * DRAG_VELOCITY_BLEND +
			pointerVelocityX * (1 - DRAG_VELOCITY_BLEND);
		velocityRef.y =
			velocityRef.y * DRAG_VELOCITY_BLEND +
			pointerVelocityY * (1 - DRAG_VELOCITY_BLEND);
		velocityRef.x = clamp(velocityRef.x, -MAX_VELOCITY, MAX_VELOCITY);
		velocityRef.y = clamp(velocityRef.y, -MAX_VELOCITY, MAX_VELOCITY);

		lastPointer = {
			x: event.clientX,
			y: event.clientY,
		};
		lastPointerTime = now;
		commitOffset();
	}

	function handlePointerEnd(event: PointerEvent) {
		if (pointerId !== event.pointerId) return;

		draggingRef = false;
		isDragging = false;

		const node = containerRef;
		if (node) {
			try {
				if (node.hasPointerCapture(event.pointerId)) {
					node.releasePointerCapture(event.pointerId);
				}
			} catch {
				// Ignore capture release failures.
			}
		}

		pointerId = null;
	}

	function handlePointerLeave() {
		if (!draggingRef) {
			resetMousePosition();
		}
	}

	function handleWheel(event: WheelEvent) {
		event.preventDefault();
		const wheelSpeed = resolvedScrollSettings.wheelSpeed;
		const deltaScale =
			event.deltaMode === 1
				? 16
				: event.deltaMode === 2
					? viewportSize.height || 800
					: 1;
		const deltaX = clamp(
			event.deltaX * deltaScale,
			-WHEEL_DELTA_LIMIT,
			WHEEL_DELTA_LIMIT,
		);
		const deltaY = clamp(
			event.deltaY * deltaScale,
			-WHEEL_DELTA_LIMIT,
			WHEEL_DELTA_LIMIT,
		);

		velocityRef.x -= deltaX * wheelSpeed * WHEEL_IMPULSE;
		velocityRef.y -= deltaY * wheelSpeed * WHEEL_IMPULSE;
		velocityRef.x = clamp(velocityRef.x, -MAX_VELOCITY, MAX_VELOCITY);
		velocityRef.y = clamp(velocityRef.y, -MAX_VELOCITY, MAX_VELOCITY);
	}

	$effect(() => {
		if (typeof window === "undefined") return;
		if (!containerRef) return;

		updateViewport();
		const observer = new ResizeObserver(updateViewport);
		observer.observe(containerRef);

		return () => {
			observer.disconnect();
		};
	});

	$effect(() => {
		if (typeof window === "undefined") return;

		const edgeThreshold = resolvedScrollSettings.edgeThreshold;
		const edgeScrollSpeed = resolvedScrollSettings.edgeScrollSpeed;
		const combinedFriction = friction * resolvedScrollSettings.friction;

		const updatePhysics = (_time: number, deltaMs: number) => {
			if (!draggingRef) {
				const deltaFrames = Math.max(deltaMs / 16.6667, 0.0001);
				const frictionFactor = Math.pow(combinedFriction, deltaFrames);
				velocityRef.x *= frictionFactor;
				velocityRef.y *= frictionFactor;

				if (Math.abs(velocityRef.x) < 0.01) velocityRef.x = 0;
				if (Math.abs(velocityRef.y) < 0.01) velocityRef.y = 0;

				const localMouseX = mousePosRef.x;
				const localMouseY = mousePosRef.y;
				const width = viewportSize.width;
				const height = viewportSize.height;
				const edgeX = edgeInfluence(localMouseX, width, edgeThreshold);
				const edgeY = edgeInfluence(localMouseY, height, edgeThreshold);
				const edgeVelocityTargetX =
					edgeX * edgeScrollSpeed * EDGE_VELOCITY_MULTIPLIER;
				const edgeVelocityTargetY =
					edgeY * edgeScrollSpeed * EDGE_VELOCITY_MULTIPLIER;

				edgeVelocityRef.x = damp(
					edgeVelocityRef.x,
					edgeVelocityTargetX,
					EDGE_RESPONSE,
					deltaFrames,
				);
				edgeVelocityRef.y = damp(
					edgeVelocityRef.y,
					edgeVelocityTargetY,
					EDGE_RESPONSE,
					deltaFrames,
				);

				const velocityX = clamp(
					velocityRef.x + edgeVelocityRef.x,
					-MAX_VELOCITY,
					MAX_VELOCITY,
				);
				const velocityY = clamp(
					velocityRef.y + edgeVelocityRef.y,
					-MAX_VELOCITY,
					MAX_VELOCITY,
				);

				if (velocityX !== 0 || velocityY !== 0) {
					offsetRef.x += velocityX * deltaFrames;
					offsetRef.y += velocityY * deltaFrames;
					commitOffset();
				}
			}
		};

		gsap.ticker.add(updatePhysics);
		return () => {
			gsap.ticker.remove(updatePhysics);
		};
	});
</script>

<div
	{@attach attachContainerRef}
	class={cn("relative h-full w-full overflow-hidden select-none", className)}
	style={`cursor:${isDragging ? "grabbing" : "grab"};touch-action:none;`}
	onpointerdown={handlePointerDown}
	onpointermove={handlePointerMove}
	onpointerup={handlePointerEnd}
	onpointercancel={handlePointerEnd}
	onpointerleave={handlePointerLeave}
	onwheel={handleWheel}
	role="presentation"
	{...restProps}
>
	<div class="pointer-events-none absolute inset-0">
		{#each visibleCells as cell (cell.id)}
			{@const mediaSrc = resolveMediaSrc(cell.item)}
			<div
				class="pointer-events-auto absolute top-0 left-0 will-change-transform"
				style={`width:${responsiveDimensions.cellWidth}px;height:${responsiveDimensions.cellHeight}px;transform:translate3d(${cell.posX}px, ${cell.posY}px, 0);`}
			>
				<div class={cn("relative h-full w-full overflow-hidden", cellClass)}>
					<div class="absolute inset-0 h-full w-full">
						{#if cell.item.type === "image" && mediaSrc}
							<img
								src={mediaSrc}
								alt={resolveAlt(cell.item)}
								draggable="false"
								class="h-full w-full object-cover"
							/>
						{/if}
						{#if cell.item.type === "video" && mediaSrc}
							<video
								src={mediaSrc}
								autoplay
								muted
								loop
								playsinline
								draggable="false"
								class="h-full w-full object-cover"
							></video>
						{/if}
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
