<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import type { ComponentPreviewControl, ComponentPreviewValue } from "./types";
	import { getDefaultControlValue } from "./types";
	import ColorPicker from "./ColorPicker.svelte";

	type Props = {
		control: ComponentPreviewControl;
		value?: ComponentPreviewValue;
		onChange: (name: string, value: ComponentPreviewValue) => void;
	};

	let { control, value, onChange }: Props = $props();

	const inputId = $derived(`preview-control-${control.name}`);
	const currentValue = $derived(value ?? getDefaultControlValue(control));
	const stringValue = $derived(String(currentValue ?? ""));
	const numberValue = $derived(
		typeof currentValue === "number" ? currentValue : Number(currentValue),
	);
	const numberMin = $derived(
		control.type === "number" ? (control.min ?? 0) : 0,
	);
	const numberMax = $derived(
		control.type === "number" ? (control.max ?? 100) : 100,
	);
	const isFloatNumberControl = $derived.by(() => {
		if (control.type !== "number") {
			return false;
		}

		return [
			control.defaultValue,
			control.min,
			control.max,
			control.step,
			numberValue,
		].some(
			(candidate) =>
				typeof candidate === "number" &&
				Number.isFinite(candidate) &&
				!Number.isInteger(candidate),
		);
	});
	const displayedNumberValue = $derived(
		Number.isFinite(numberValue)
			? isFloatNumberControl
				? numberValue.toFixed(3)
				: String(numberValue)
			: "",
	);
	const sliderProgress = $derived.by(() => {
		const range = numberMax - numberMin;
		if (range <= 0 || !Number.isFinite(numberValue)) {
			return 0;
		}

		const progress = ((numberValue - numberMin) / range) * 100;
		return Math.min(100, Math.max(0, progress));
	});
	const booleanValue = $derived(Boolean(currentValue));
	let dragProgress = $state<number | null>(null);
	const visualSliderProgress = $derived(dragProgress ?? sliderProgress);
	const visualSliderProgressRatio = $derived(visualSliderProgress / 100);
	const visualSliderWidth = $derived(
		`calc(${visualSliderProgress}% + ${1.75 - 2.5 * visualSliderProgressRatio}rem)`,
	);
	const SLIDER_INSET_PX = 6;
	const SLIDER_MIN_WIDTH_PX = 28;

	const getStepPrecision = (step: number) => {
		const normalized = step.toString().toLowerCase();
		const decimal = normalized.split(".")[1];
		const exponent = normalized.split("e-")[1];

		if (exponent) return Number(exponent);
		return decimal?.length ?? 0;
	};

	const quantizeNumber = (rawValue: number) => {
		if (control.type !== "number") return rawValue;

		const step = control.step ?? 1;
		const safeStep = step > 0 ? step : 1;
		const clamped = Math.min(numberMax, Math.max(numberMin, rawValue));
		const snapped =
			numberMin + Math.round((clamped - numberMin) / safeStep) * safeStep;
		const precision = getStepPrecision(safeStep);

		return Number(
			Math.min(numberMax, Math.max(numberMin, snapped)).toFixed(precision),
		);
	};

	const updateNumberFromPointer = (event: PointerEvent, node: HTMLElement) => {
		const rect = node.getBoundingClientRect();
		if (rect.width <= 0) return;

		const trackWidth = Math.max(
			1,
			rect.width - SLIDER_INSET_PX * 2 - SLIDER_MIN_WIDTH_PX,
		);
		const x = event.clientX - rect.left - SLIDER_INSET_PX - SLIDER_MIN_WIDTH_PX;
		const progress = Math.min(100, Math.max(0, (x / trackWidth) * 100));
		const rawValue = numberMin + (progress / 100) * (numberMax - numberMin);
		const next = quantizeNumber(rawValue);

		dragProgress = progress;

		if (next !== numberValue) {
			onChange(control.name, next);
		}
	};

	const handleSliderPointerDown = (event: PointerEvent) => {
		if (control.type !== "number") return;

		const node = event.currentTarget as HTMLElement;
		node.setPointerCapture(event.pointerId);
		updateNumberFromPointer(event, node);
	};

	const handleSliderPointerMove = (event: PointerEvent) => {
		if (control.type !== "number") return;

		const node = event.currentTarget as HTMLElement;
		if (!node.hasPointerCapture(event.pointerId)) return;
		updateNumberFromPointer(event, node);
	};

	const handleSliderPointerEnd = (event: PointerEvent) => {
		const node = event.currentTarget as HTMLElement;

		if (node.hasPointerCapture(event.pointerId)) {
			node.releasePointerCapture(event.pointerId);
		}

		dragProgress = null;
	};

	const handleSliderKeydown = (event: KeyboardEvent) => {
		if (control.type !== "number") return;

		const step = control.step ?? 1;
		const safeStep = step > 0 ? step : 1;
		let next: number;

		switch (event.key) {
			case "ArrowLeft":
			case "ArrowDown":
				next = quantizeNumber(numberValue - safeStep);
				break;
			case "ArrowRight":
			case "ArrowUp":
				next = quantizeNumber(numberValue + safeStep);
				break;
			case "PageDown":
				next = quantizeNumber(numberValue - safeStep * 10);
				break;
			case "PageUp":
				next = quantizeNumber(numberValue + safeStep * 10);
				break;
			case "Home":
				next = numberMin;
				break;
			case "End":
				next = numberMax;
				break;
			default:
				return;
		}

		event.preventDefault();

		if (next !== numberValue) {
			onChange(control.name, next);
		}
	};

	const updateSelect = (rawValue: string) => {
		if (control.type !== "select") return;

		const matchedOption = control.options.find(
			(option) => String(option.value) === rawValue,
		);

		onChange(control.name, matchedOption?.value ?? rawValue);
	};
</script>

<div
	class={cn(
		"group/control relative flex h-11 min-w-0 items-center rounded-md bg-background-inset pr-1.5 pl-4 inset-shadow transition-[border-color,background-color] duration-150 ease-out",
		control.type === "color" ? "overflow-visible" : "overflow-hidden",
	)}
	title={control.description}
>
	{#if control.type === "number"}
		<div
			id={inputId}
			role="slider"
			tabindex="0"
			aria-label={control.label}
			aria-valuemin={numberMin}
			aria-valuemax={numberMax}
			aria-valuenow={numberValue}
			aria-valuetext={`${displayedNumberValue}${control.unit ?? ""}`}
			class="peer absolute inset-0 z-20 h-full w-full cursor-pointer touch-none outline-none"
			onpointerdown={handleSliderPointerDown}
			onpointermove={handleSliderPointerMove}
			onpointerup={handleSliderPointerEnd}
			onpointercancel={handleSliderPointerEnd}
			onkeydown={handleSliderKeydown}
		></div>
		<div
			aria-hidden="true"
			class="pointer-events-none absolute inset-0"
			style:--slider-progress={`${visualSliderProgress}%`}
		>
			<div
				class="absolute inset-y-1.5 left-1.5 min-w-7 rounded-sm bg-background card"
				style:width={visualSliderWidth}
			>
				<span
					class="absolute top-1/2 right-1 h-4 w-1 -translate-y-1/2 rounded-full bg-background-muted"
				></span>
			</div>
		</div>
	{/if}

	<label
		for={inputId}
		class={cn(
			"relative z-10 min-w-0 flex-1 truncate pr-3 text-sm tracking-normal text-foreground-muted",
			control.type === "number" && "pointer-events-none text-foreground-muted",
		)}
	>
		{control.label}
	</label>

	<div class="relative z-10 flex shrink-0 items-center justify-end gap-2">
		{#if control.type === "number"}
			<span
				class="pointer-events-none min-w-12 pr-2.5 text-right font-mono text-xs text-foreground tabular-nums"
			>
				{displayedNumberValue}{control.unit ?? ""}
			</span>
		{:else if control.type === "boolean"}
			<button
				id={inputId}
				type="button"
				role="switch"
				aria-label={control.label}
				aria-checked={booleanValue}
				class={cn(
					"relative h-8 w-14 shrink-0 rounded-sm p-1 inset-shadow transition-colors duration-150 ease-out",
					booleanValue ? "bg-accent" : "bg-background-inset",
				)}
				onclick={() => onChange(control.name, !booleanValue)}
			>
				<span
					class={cn(
						"block transition-transform duration-150 ease-out",
						booleanValue && "translate-x-5.25",
					)}
				>
					<span
						class="block h-5.5 w-7 rounded-[calc(var(--radius-base)*1.25)] bg-background card"
					></span>
				</span>
			</button>
		{:else if control.type === "text"}
			<input
				id={inputId}
				type="text"
				placeholder={control.placeholder}
				value={stringValue}
				class="h-8 w-36 rounded-sm bg-background px-3 text-right font-mono text-sm text-foreground inset-shadow transition-colors duration-150 ease-out outline-none focus:outline-none"
				oninput={(event) => onChange(control.name, event.currentTarget.value)}
			/>
		{:else if control.type === "color"}
			<ColorPicker
				id={inputId}
				value={stringValue}
				label={control.label}
				onChange={(next) => onChange(control.name, next)}
			/>
		{:else if control.type === "select"}
			<div class="relative">
				<select
					id={inputId}
					value={stringValue}
					class="h-8 appearance-none rounded-sm bg-background pr-8 pl-3 text-left text-sm font-medium text-foreground card transition-colors duration-150 ease-out focus-visible:outline-transparent focus-visible:transition-none"
					onchange={(event) => updateSelect(event.currentTarget.value)}
				>
					{#each control.options as option (option.value)}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
				<span
					aria-hidden="true"
					class="pointer-events-none absolute top-1/2 right-3 h-2 w-2 -translate-y-1/2 rotate-45 border-r-2 border-b-2 border-foreground-muted"
				></span>
			</div>
		{/if}
	</div>
</div>
