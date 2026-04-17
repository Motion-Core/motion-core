export type ColorRepresentation =
	| string
	| number
	| readonly [number, number, number]
	| { r: number; g: number; b: number };

const clamp = (value: number, min: number, max: number) =>
	Math.min(max, Math.max(min, value));

export const clamp01 = (value: number) => clamp(value, 0, 1);

export const srgbToLinear = (value: number) =>
	value <= 0.04045 ? value / 12.92 : Math.pow((value + 0.055) / 1.055, 2.4);

export const normalizeTriplet = (
	r: number,
	g: number,
	b: number,
): [number, number, number] => {
	const scale = Math.max(r, g, b) > 1 ? 255 : 1;
	return [clamp01(r / scale), clamp01(g / scale), clamp01(b / scale)];
};

export const parseHexColor = (
	value: string,
): [number, number, number] | null => {
	const hex = value.replace("#", "").trim();
	if (hex.length === 3 || hex.length === 4) {
		const r = Number.parseInt(hex[0] + hex[0], 16);
		const g = Number.parseInt(hex[1] + hex[1], 16);
		const b = Number.parseInt(hex[2] + hex[2], 16);
		return [r / 255, g / 255, b / 255];
	}
	if (hex.length === 6 || hex.length === 8) {
		const r = Number.parseInt(hex.slice(0, 2), 16);
		const g = Number.parseInt(hex.slice(2, 4), 16);
		const b = Number.parseInt(hex.slice(4, 6), 16);
		return [r / 255, g / 255, b / 255];
	}
	return null;
};

let cssColorContext: CanvasRenderingContext2D | null | undefined;

export const parseCssColor = (
	value: string,
): [number, number, number] | null => {
	if (typeof document === "undefined") return null;
	if (cssColorContext === undefined) {
		const parserCanvas = document.createElement("canvas");
		parserCanvas.width = 1;
		parserCanvas.height = 1;
		cssColorContext = parserCanvas.getContext("2d");
	}
	if (!cssColorContext) return null;

	cssColorContext.fillStyle = "#000000";
	cssColorContext.fillStyle = value;
	const normalized = cssColorContext.fillStyle;

	if (normalized.startsWith("#")) {
		return parseHexColor(normalized);
	}

	const match = normalized.match(/rgba?\(([^)]+)\)/i);
	if (!match) return null;
	const parts = match[1]
		.split(",")
		.map((part) => Number.parseFloat(part.trim()))
		.filter((part) => Number.isFinite(part));
	if (parts.length < 3) return null;
	return normalizeTriplet(parts[0], parts[1], parts[2]);
};

export const toRgb = (
	value: ColorRepresentation,
	fallback: [number, number, number],
): [number, number, number] => {
	if (typeof value === "number" && Number.isFinite(value)) {
		const int = Math.min(0xffffff, Math.max(0, Math.floor(value)));
		return [
			((int >> 16) & 255) / 255,
			((int >> 8) & 255) / 255,
			(int & 255) / 255,
		];
	}

	if (typeof value === "string") {
		const trimmed = value.trim();
		const parsed = trimmed.startsWith("#")
			? parseHexColor(trimmed)
			: parseCssColor(trimmed);
		return parsed ?? fallback;
	}

	if (Array.isArray(value) && value.length >= 3) {
		return normalizeTriplet(value[0], value[1], value[2]);
	}

	if (
		value &&
		typeof value === "object" &&
		"r" in value &&
		"g" in value &&
		"b" in value
	) {
		const rgb = value as { r: number; g: number; b: number };
		return normalizeTriplet(rgb.r, rgb.g, rgb.b);
	}

	return fallback;
};

export const toLinearRgb = (
	value: ColorRepresentation,
	fallback: [number, number, number],
): [number, number, number] => {
	const [r, g, b] = toRgb(value, fallback);
	return [srgbToLinear(r), srgbToLinear(g), srgbToLinear(b)];
};
