<script lang="ts">
	import gsap from "gsap";
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn";

	type TrailConfig = {
		imageLifespan?: number;
		removalTickMs?: number;
		mouseThreshold?: number;
		minMovementForImage?: number;
		inDuration?: number;
		outDuration?: number;
		maxRotationFactor?: number;
		baseRotation?: number;
		speedSmoothingFactor?: number;
		minImageSize?: number;
		maxImageSize?: number;
		staggerOut?: number;
	};

	const DEFAULT_CONFIG: Required<TrailConfig> = {
		imageLifespan: 600,
		removalTickMs: 16,
		mouseThreshold: 40,
		minMovementForImage: 5,
		inDuration: 600,
		outDuration: 800,
		maxRotationFactor: 3,
		baseRotation: 30,
		speedSmoothingFactor: 0.25,
		minImageSize: 260,
		maxImageSize: 340,
		staggerOut: 40,
	};

	const POOL_CAP = 24;

	type ComponentProps = {
		images: string[];
		children?: Snippet;
		class?: string;
		config?: TrailConfig;
		[prop: string]: unknown;
	};

	const props: ComponentProps = $props();
	const children = $derived(props.children);
	const className = $derived(props.class ?? "");
	const images = $derived(props.images ?? []);
	const cfg = $derived<Required<TrailConfig>>({
		...DEFAULT_CONFIG,
		...(props.config ?? {}),
	});
	const restProps = $derived(() => {
		const {
			children: _children,
			class: _class,
			images: _images,
			config: _config,
			...rest
		} = props;
		return rest;
	});

	let containerRef: HTMLDivElement | null = null;

	type TrailItem = {
		el: HTMLImageElement;
		rotation: number;
		removeAt: number;
	};

	const state = {
		imageIndex: 0,
		isPointerIn: false,
		isMoving: false,
		lastMouseX: 0,
		lastMouseY: 0,
		mouseX: 0,
		mouseY: 0,
		prevMouseX: 0,
		prevMouseY: 0,
		lastMoveTime: Date.now(),
		lastRemovalTime: 0,
		smoothedSpeed: 0,
		maxSpeed: 0.5,
		raf: 0,
	};

	const trail: TrailItem[] = [];
	const pool: HTMLImageElement[] = [];

	const getNextImageSrc = () => {
		if (!images.length) return "";
		const idx = state.imageIndex % images.length;
		state.imageIndex = (state.imageIndex + 1) % images.length;
		return images[idx] ?? "";
	};

	const hasMovedEnough = () => {
		const dx = state.mouseX - state.lastMouseX;
		const dy = state.mouseY - state.lastMouseY;
		return Math.hypot(dx, dy) > cfg.mouseThreshold;
	};

	const hasMovedAtAll = () => {
		const dx = state.mouseX - state.prevMouseX;
		const dy = state.mouseY - state.prevMouseY;
		return Math.hypot(dx, dy) > cfg.minMovementForImage;
	};

	const calcSpeed = () => {
		const now = Date.now();
		const dt = now - state.lastMoveTime;
		if (dt <= 0) return state.smoothedSpeed;

		const dist = Math.hypot(
			state.mouseX - state.prevMouseX,
			state.mouseY - state.prevMouseY,
		);
		const raw = dist / dt;

		if (raw > state.maxSpeed) state.maxSpeed = raw;

		const norm = Math.min(raw / (state.maxSpeed || 0.5), 1);
		state.smoothedSpeed =
			state.smoothedSpeed * (1 - cfg.speedSmoothingFactor) +
			norm * cfg.speedSmoothingFactor;
		state.lastMoveTime = now;

		return state.smoothedSpeed;
	};

	const getPooledImage = () => {
		const el = pool.pop();
		if (el) return el;

		const img = document.createElement("img");
		img.className =
			"pointer-events-none select-none absolute will-change-transform";
		img.style.transformOrigin = "50% 50%";
		img.draggable = false;
		return img;
	};

	const recycleImage = (img: HTMLImageElement) => {
		gsap.killTweensOf(img);
		img.remove();
		img.removeAttribute("style");
		img.className =
			"pointer-events-none select-none absolute will-change-transform";
		if (pool.length < POOL_CAP) pool.push(img);
	};

	const isInsideContainer = (clientX: number, clientY: number) => {
		const node = containerRef;
		if (!node) return false;
		const rect = node.getBoundingClientRect();
		return (
			clientX >= rect.left &&
			clientX <= rect.right &&
			clientY >= rect.top &&
			clientY <= rect.bottom
		);
	};

	const spawnTrail = (speed = 0.5) => {
		const node = containerRef;
		if (!node || !images.length) return;

		const rect = node.getBoundingClientRect();
		const x = state.mouseX - rect.left;
		const y = state.mouseY - rect.top;

		const size = Math.round(
			cfg.minImageSize + (cfg.maxImageSize - cfg.minImageSize) * speed,
		);
		const rotFactor = 1 + speed * (cfg.maxRotationFactor - 1);
		const rot = (Math.random() - 0.5) * cfg.baseRotation * rotFactor;

		const img = getPooledImage();
		img.src = getNextImageSrc();
		img.width = size;
		img.height = size;

		img.style.left = `${x}px`;
		img.style.top = `${y}px`;
		img.style.transform = "translate(-50%, -50%) scale(0)";

		node.appendChild(img);

		gsap.set(img, { rotation: rot });
		gsap.to(img, {
			scale: 1,
			duration: cfg.inDuration / 1000,
			ease: "power2.out",
		});

		trail.push({
			el: img,
			rotation: rot,
			removeAt: Date.now() + cfg.imageLifespan,
		});
	};

	const tryEmit = () => {
		if (!state.isPointerIn) return;
		if (hasMovedEnough() && hasMovedAtAll()) {
			state.lastMouseX = state.mouseX;
			state.lastMouseY = state.mouseY;
			const speed = calcSpeed();
			spawnTrail(speed);
			state.prevMouseX = state.mouseX;
			state.prevMouseY = state.mouseY;
		}
	};

	const cullOld = () => {
		const now = Date.now();
		if (now - state.lastRemovalTime < cfg.removalTickMs) return;
		if (!trail.length) return;

		const expired = trail.filter((item) => now >= item.removeAt);
		if (!expired.length) return;

		expired.forEach((item, index) => {
			const { el } = item;
			gsap.to(el, {
				duration: cfg.outDuration / 1000,
				scale: 0,
				ease: "power4.inOut",
				delay: (index * cfg.staggerOut) / 1000,
				onComplete: () => recycleImage(el),
			});
		});

		for (let i = trail.length - 1; i >= 0; i -= 1) {
			if (now >= trail[i].removeAt) {
				trail.splice(i, 1);
			}
		}

		state.lastRemovalTime = now;
	};

	const tick = () => {
		if (state.isMoving) tryEmit();
		cullOld();
		state.raf = requestAnimationFrame(tick);
	};

	const resetTrail = () => {
		trail.forEach(({ el }) => {
			gsap.killTweensOf(el);
			el.remove();
		});
		trail.length = 0;
	};

	$effect(() => {
		if (typeof window === "undefined") return;

		const node = containerRef;
		if (!node || !images.length) return;

		let pointerIdleTimeout: number | null = null;

		const onPointerMove = (event: PointerEvent) => {
			state.prevMouseX = state.mouseX;
			state.prevMouseY = state.mouseY;
			state.mouseX = event.clientX;
			state.mouseY = event.clientY;
			state.isPointerIn = isInsideContainer(event.clientX, event.clientY);

			if (state.isPointerIn) {
				state.isMoving = true;
				if (pointerIdleTimeout) window.clearTimeout(pointerIdleTimeout);
				pointerIdleTimeout = window.setTimeout(() => {
					state.isMoving = false;
					pointerIdleTimeout = null;
				}, 100);
			}
		};

		const onPointerEnter = (event: PointerEvent) => {
			state.isPointerIn = true;
			state.isMoving = false;
			state.mouseX = event.clientX;
			state.mouseY = event.clientY;
			state.lastMouseX = event.clientX;
			state.lastMouseY = event.clientY;
			state.prevMouseX = event.clientX;
			state.prevMouseY = event.clientY;
			state.lastMoveTime = Date.now();
		};

		const onPointerLeave = () => {
			state.isPointerIn = false;
			state.isMoving = false;
		};

		const onTouchMove = (event: TouchEvent) => {
			if (!event.touches.length) return;
			const touch = event.touches[0];
			const dx = Math.abs(touch.clientX - state.prevMouseX);
			const dy = Math.abs(touch.clientY - state.prevMouseY);
			if (dy > dx) return;

			state.prevMouseX = state.mouseX;
			state.prevMouseY = state.mouseY;
			state.mouseX = touch.clientX;
			state.mouseY = touch.clientY;
			state.isPointerIn = isInsideContainer(touch.clientX, touch.clientY);
			if (state.isPointerIn) {
				state.isMoving = true;
			}
		};

		node.addEventListener("pointermove", onPointerMove, { passive: true });
		node.addEventListener("pointerenter", onPointerEnter, { passive: true });
		node.addEventListener("pointerleave", onPointerLeave, { passive: true });
		node.addEventListener("touchmove", onTouchMove, { passive: true });

		state.raf = requestAnimationFrame(tick);

		return () => {
			if (pointerIdleTimeout) window.clearTimeout(pointerIdleTimeout);
			cancelAnimationFrame(state.raf);
			node.removeEventListener("pointermove", onPointerMove);
			node.removeEventListener("pointerenter", onPointerEnter);
			node.removeEventListener("pointerleave", onPointerLeave);
			node.removeEventListener("touchmove", onTouchMove);
			resetTrail();
		};
	});
</script>

<div
	{...restProps}
	class={cn("relative overflow-hidden", className)}
	bind:this={containerRef}
>
	{@render children?.()}
</div>
