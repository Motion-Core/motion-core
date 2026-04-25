<script lang="ts">
	import { tick } from "svelte";
	import { scale } from "svelte/transition";
	import { portal } from "$lib/utils/use-portal";

	type Props = {
		id: string;
		label: string;
		value: string;
		onChange: (value: string) => void;
	};

	type Rgb = {
		r: number;
		g: number;
		b: number;
	};

	type Hsv = {
		h: number;
		s: number;
		v: number;
	};

	let { id, label, value, onChange }: Props = $props();

	let isOpen = $state(false);
	let rootRef: HTMLDivElement | null = null;
	let buttonRef: HTMLButtonElement | null = null;
	let popoverRef: HTMLDivElement | null = null;
	let popoverStyle = $state("");
	let hueValue = $state<number | null>(null);
	let lastEmittedHex = "";
	let positionFrame: number | null = null;

	const POPOVER_WIDTH = 208;
	const POPOVER_GAP = 8;

	const clamp = (value: number, min: number, max: number) =>
		Math.min(max, Math.max(min, value));

	const normalizeHex = (rawValue: string) => {
		const trimmed = rawValue.trim();
		const withHash = trimmed.startsWith("#") ? trimmed : `#${trimmed}`;
		const shorthandMatch = /^#([\da-f]{3})$/i.exec(withHash);

		if (shorthandMatch) {
			return `#${shorthandMatch[1]
				.split("")
				.map((char) => char + char)
				.join("")}`.toLowerCase();
		}

		if (/^#[\da-f]{6}$/i.test(withHash)) {
			return withHash.toLowerCase();
		}

		return "#ff6900";
	};

	const hexToRgb = (hex: string): Rgb => {
		const normalized = normalizeHex(hex);
		const value = Number.parseInt(normalized.slice(1), 16);

		return {
			r: (value >> 16) & 255,
			g: (value >> 8) & 255,
			b: value & 255,
		};
	};

	const rgbToHex = ({ r, g, b }: Rgb) =>
		`#${[r, g, b]
			.map((channel) => Math.round(channel).toString(16).padStart(2, "0"))
			.join("")}`;

	const rgbToHsv = ({ r, g, b }: Rgb): Hsv => {
		const red = r / 255;
		const green = g / 255;
		const blue = b / 255;
		const max = Math.max(red, green, blue);
		const min = Math.min(red, green, blue);
		const delta = max - min;
		let h = 0;

		if (delta !== 0) {
			if (max === red) {
				h = ((green - blue) / delta) % 6;
			} else if (max === green) {
				h = (blue - red) / delta + 2;
			} else {
				h = (red - green) / delta + 4;
			}
		}

		h = Math.round(h * 60);
		if (h < 0) h += 360;

		return {
			h,
			s: max === 0 ? 0 : (delta / max) * 100,
			v: max * 100,
		};
	};

	const hsvToRgb = ({ h, s, v }: Hsv): Rgb => {
		const normalizedS = clamp(s, 0, 100) / 100;
		const normalizedV = clamp(v, 0, 100) / 100;
		const chroma = normalizedV * normalizedS;
		const x = chroma * (1 - Math.abs(((h / 60) % 2) - 1));
		const m = normalizedV - chroma;
		let rgb: Rgb;

		if (h < 60) rgb = { r: chroma, g: x, b: 0 };
		else if (h < 120) rgb = { r: x, g: chroma, b: 0 };
		else if (h < 180) rgb = { r: 0, g: chroma, b: x };
		else if (h < 240) rgb = { r: 0, g: x, b: chroma };
		else if (h < 300) rgb = { r: x, g: 0, b: chroma };
		else rgb = { r: chroma, g: 0, b: x };

		return {
			r: (rgb.r + m) * 255,
			g: (rgb.g + m) * 255,
			b: (rgb.b + m) * 255,
		};
	};

	const hsvToHex = (hsv: Hsv) => rgbToHex(hsvToRgb(hsv));

	const hexValue = $derived(normalizeHex(value));
	const parsedHsvValue = $derived(rgbToHsv(hexToRgb(hexValue)));
	const hsvValue = $derived({
		...parsedHsvValue,
		h: hueValue ?? parsedHsvValue.h,
	});
	const hueColor = $derived(hsvToHex({ h: hsvValue.h, s: 100, v: 100 }));

	const commitColor = (nextHex: string) => {
		lastEmittedHex = nextHex;
		onChange(nextHex);
	};

	const updateFromPlane = (event: PointerEvent, node: HTMLElement) => {
		const rect = node.getBoundingClientRect();
		if (rect.width <= 0 || rect.height <= 0) return;

		const s = clamp(((event.clientX - rect.left) / rect.width) * 100, 0, 100);
		const v = clamp(
			100 - ((event.clientY - rect.top) / rect.height) * 100,
			0,
			100,
		);

		hueValue = hsvValue.h;
		commitColor(hsvToHex({ h: hsvValue.h, s, v }));
	};

	const updateFromHue = (event: PointerEvent, node: HTMLElement) => {
		const rect = node.getBoundingClientRect();
		if (rect.width <= 0) return;

		const h = clamp(((event.clientX - rect.left) / rect.width) * 360, 0, 360);
		hueValue = h;
		commitColor(hsvToHex({ h, s: hsvValue.s, v: hsvValue.v }));
	};

	const handlePlanePointerDown = (event: PointerEvent) => {
		const node = event.currentTarget as HTMLElement;
		node.setPointerCapture(event.pointerId);
		updateFromPlane(event, node);
	};

	const handlePlanePointerMove = (event: PointerEvent) => {
		const node = event.currentTarget as HTMLElement;
		if (!node.hasPointerCapture(event.pointerId)) return;
		updateFromPlane(event, node);
	};

	const handleHuePointerDown = (event: PointerEvent) => {
		const node = event.currentTarget as HTMLElement;
		node.setPointerCapture(event.pointerId);
		updateFromHue(event, node);
	};

	const handleHuePointerMove = (event: PointerEvent) => {
		const node = event.currentTarget as HTMLElement;
		if (!node.hasPointerCapture(event.pointerId)) return;
		updateFromHue(event, node);
	};

	const releasePointer = (event: PointerEvent) => {
		const node = event.currentTarget as HTMLElement;
		if (node.hasPointerCapture(event.pointerId)) {
			node.releasePointerCapture(event.pointerId);
		}
	};

	const handleHexInput = (event: Event) => {
		const next = (event.currentTarget as HTMLInputElement).value;
		const withHash = next.startsWith("#") ? next : `#${next}`;

		if (/^#[\da-f]{6}$/i.test(withHash) || /^#[\da-f]{3}$/i.test(withHash)) {
			const normalized = normalizeHex(withHash);
			hueValue = rgbToHsv(hexToRgb(normalized)).h;
			commitColor(normalized);
		}
	};

	const handleDocumentClick = (event: MouseEvent) => {
		if (!isOpen) return;
		const target = event.target as Node;
		if (rootRef?.contains(target)) return;
		if (popoverRef?.contains(target)) return;
		isOpen = false;
	};

	const handleDocumentKeydown = (event: KeyboardEvent) => {
		if (event.key === "Escape") {
			isOpen = false;
		}
	};

	const updatePopoverPosition = () => {
		if (!buttonRef) return;

		const rect = buttonRef.getBoundingClientRect();
		const viewportPadding = 12;
		const left = clamp(
			rect.right - POPOVER_WIDTH,
			viewportPadding,
			window.innerWidth - POPOVER_WIDTH - viewportPadding,
		);
		const top = Math.min(
			rect.bottom + POPOVER_GAP,
			window.innerHeight - viewportPadding,
		);

		popoverStyle = `top: ${top}px; left: ${left}px; width: ${POPOVER_WIDTH}px;`;
	};

	const schedulePopoverPosition = () => {
		if (positionFrame !== null) return;

		positionFrame = requestAnimationFrame(() => {
			positionFrame = null;
			updatePopoverPosition();
		});
	};

	const toggleOpen = async () => {
		isOpen = !isOpen;

		if (isOpen) {
			await tick();
			updatePopoverPosition();
		}
	};

	$effect(() => {
		if (!isOpen) return;
		updatePopoverPosition();

		window.addEventListener("scroll", schedulePopoverPosition, true);
		window.addEventListener("resize", schedulePopoverPosition);

		return () => {
			window.removeEventListener("scroll", schedulePopoverPosition, true);
			window.removeEventListener("resize", schedulePopoverPosition);

			if (positionFrame !== null) {
				cancelAnimationFrame(positionFrame);
				positionFrame = null;
			}
		};
	});

	$effect(() => {
		if (hexValue === lastEmittedHex) return;

		hueValue = parsedHsvValue.h;
		lastEmittedHex = hexValue;
	});
</script>

<svelte:document
	onclick={handleDocumentClick}
	onkeydown={handleDocumentKeydown}
/>

<div
	class="relative"
	{@attach (node) => {
		rootRef = node;
		return () => {
			if (rootRef === node) rootRef = null;
		};
	}}
>
	<button
		{id}
		type="button"
		{@attach (node) => {
			buttonRef = node;
			return () => {
				if (buttonRef === node) buttonRef = null;
			};
		}}
		aria-label={label}
		aria-expanded={isOpen}
		class="flex h-7.5 items-center gap-2 rounded-sm bg-background p-1 pr-2 card"
		onclick={toggleOpen}
	>
		<span
			class="block h-full w-8 rounded-[calc(var(--radius-base)*1.25)] border border-border"
			style:background={hexValue}
		></span>
		<span
			class="hidden font-mono text-xs text-foreground tabular-nums sm:block"
		>
			{hexValue}
		</span>
	</button>

	{#if isOpen}
		<div
			use:portal
			{@attach (node) => {
				popoverRef = node;
				return () => {
					if (popoverRef === node) popoverRef = null;
				};
			}}
			transition:scale={{ duration: 120, start: 0.96 }}
			class="fixed z-50 rounded-md bg-background p-1.5 shadow-2xl card"
			style={popoverStyle}
		>
			<div
				role="slider"
				tabindex="0"
				aria-label={`${label} saturation and brightness`}
				aria-valuenow={Math.round(hsvValue.s)}
				aria-valuetext={hexValue}
				class="relative h-24 cursor-crosshair touch-none overflow-hidden rounded-sm"
				style:background={hueColor}
				onpointerdown={handlePlanePointerDown}
				onpointermove={handlePlanePointerMove}
				onpointerup={releasePointer}
				onpointercancel={releasePointer}
			>
				<div
					class="pointer-events-none absolute inset-0"
					style="background: linear-gradient(to right, #fff, transparent);"
				></div>
				<div
					class="pointer-events-none absolute inset-0"
					style="background: linear-gradient(to top, #000, transparent);"
				></div>
				<span
					class="pointer-events-none absolute size-3 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white shadow-[0_0_0_1px_rgba(0,0,0,0.5)]"
					style:left={`${hsvValue.s}%`}
					style:top={`${100 - hsvValue.v}%`}
				></span>
			</div>

			<div
				role="slider"
				tabindex="0"
				aria-label={`${label} hue`}
				aria-valuemin="0"
				aria-valuemax="360"
				aria-valuenow={Math.round(hsvValue.h)}
				class="relative mt-1.5 h-5 cursor-pointer touch-none rounded-sm"
				style="background: linear-gradient(to right, #f00, #ff0, #0f0, #0ff, #00f, #f0f, #f00);"
				onpointerdown={handleHuePointerDown}
				onpointermove={handleHuePointerMove}
				onpointerup={releasePointer}
				onpointercancel={releasePointer}
			>
				<span
					class="pointer-events-none absolute top-1/2 h-4 w-1 -translate-x-1/2 -translate-y-1/2 rounded-full bg-white shadow-[0_0_0_1px_rgba(0,0,0,0.5)]"
					style:left={`${(hsvValue.h / 360) * 100}%`}
				></span>
			</div>

			<input
				type="text"
				value={hexValue}
				spellcheck="false"
				class="mt-1.5 h-7 w-full rounded-sm bg-background-inset px-2 font-mono text-xs text-foreground uppercase inset-shadow outline-none focus:outline-none"
				oninput={handleHexInput}
			/>
		</div>
	{/if}
</div>
