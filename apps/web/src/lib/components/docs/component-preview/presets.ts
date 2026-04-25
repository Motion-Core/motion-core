import type { ComponentPreviewControl } from "./types";

type PreviewControlPreset = ComponentPreviewControl[];

const number = (
	name: string,
	label: string,
	options: {
		defaultValue: number;
		min: number;
		max: number;
		step?: number;
		unit?: string;
	},
): ComponentPreviewControl => ({
	name,
	label,
	type: "number",
	...options,
});

const color = (
	name: string,
	label: string,
	defaultValue: string,
): ComponentPreviewControl => ({
	name,
	label,
	type: "color",
	defaultValue,
});

const bool = (
	name: string,
	label: string,
	defaultValue: boolean,
): ComponentPreviewControl => ({
	name,
	label,
	type: "boolean",
	defaultValue,
});

const text = (
	name: string,
	label: string,
	defaultValue: string,
	placeholder?: string,
): ComponentPreviewControl => ({
	name,
	label,
	type: "text",
	defaultValue,
	placeholder,
});

const select = (
	name: string,
	label: string,
	defaultValue: string | number,
	options: Array<{ label: string; value: string | number }>,
): ComponentPreviewControl => ({
	name,
	label,
	type: "select",
	defaultValue,
	options,
});

export const componentPreviewControls = {
	"ascii-renderer": [
		number("density", "Density", {
			defaultValue: 25,
			min: 5,
			max: 80,
			step: 1,
		}),
		number("strength", "Strength", {
			defaultValue: 3,
			min: 0.5,
			max: 8,
			step: 0.1,
		}),
		color("color", "Color", "#00ff00"),
		color("backgroundColor", "Background", "#17181A"),
	],
	"card-3d": [
		number("width", "Width", {
			defaultValue: 3.2,
			min: 1.5,
			max: 5,
			step: 0.1,
		}),
		number("height", "Height", {
			defaultValue: 2,
			min: 1,
			max: 4,
			step: 0.1,
		}),
		number("depth", "Depth", {
			defaultValue: 0.08,
			min: 0.02,
			max: 0.3,
			step: 0.01,
		}),
		number("radius", "Radius", {
			defaultValue: 0.25,
			min: 0,
			max: 0.5,
			step: 0.01,
		}),
		bool("showPreview", "Camera Preview", true),
	],
	"card-stack": [
		number("scaleFactor", "Scale Factor", {
			defaultValue: 0.05,
			min: 0,
			max: 0.15,
			step: 0.001,
		}),
		number("offset", "Offset", {
			defaultValue: 32,
			min: 0,
			max: 80,
			step: 1,
		}),
		number("topOffset", "Top Offset", {
			defaultValue: 32,
			min: 0,
			max: 120,
			step: 1,
		}),
	],
	"dithered-image": [
		{
			name: "ditherMap",
			label: "Dither Map",
			type: "select",
			defaultValue: "halftone",
			options: [
				{ label: "Halftone", value: "halftone" },
				{ label: "Bayer 4x4", value: "bayer4x4" },
				{ label: "Bayer 8x8", value: "bayer8x8" },
			],
		},
		number("pixelSize", "Pixel Size", {
			defaultValue: 1,
			min: 1,
			max: 8,
			step: 1,
		}),
		number("threshold", "Threshold", {
			defaultValue: 0.2,
			min: 0,
			max: 1,
			step: 0.01,
		}),
		color("color", "Color", "#ff6900"),
		color("backgroundColor", "Background", "#17181A"),
	],
	"fake-3d-image": [
		number("xThreshold", "X Threshold", {
			defaultValue: 8,
			min: 0,
			max: 24,
			step: 1,
		}),
		number("yThreshold", "Y Threshold", {
			defaultValue: 8,
			min: 0,
			max: 24,
			step: 1,
		}),
		number("sensitivity", "Sensitivity", {
			defaultValue: 0.25,
			min: 0,
			max: 1,
			step: 0.01,
		}),
	],
	"flip-card-stack": [
		number("stackOffset", "Stack Offset", {
			defaultValue: 10,
			min: 0,
			max: 40,
			step: 1,
		}),
		number("stackRotation", "Stack Rotation", {
			defaultValue: -8,
			min: -24,
			max: 24,
			step: 1,
		}),
		number("dragThreshold", "Drag Threshold", {
			defaultValue: 80,
			min: 20,
			max: 180,
			step: 1,
		}),
		number("duration", "Duration", {
			defaultValue: 0.3,
			min: 0.05,
			max: 1,
			step: 0.01,
			unit: "s",
		}),
	],
	"fluid-image-reveal": [
		number("dissipation", "Dissipation", {
			defaultValue: 0.96,
			min: 0.85,
			max: 0.995,
			step: 0.001,
		}),
		number("pointerSize", "Pointer Size", {
			defaultValue: 0.005,
			min: 0.001,
			max: 0.03,
			step: 0.001,
		}),
		number("velocityDissipation", "Velocity Dissipation", {
			defaultValue: 0.96,
			min: 0.85,
			max: 0.995,
			step: 0.001,
		}),
		number("pressureIterations", "Pressure Iterations", {
			defaultValue: 10,
			min: 1,
			max: 30,
			step: 1,
		}),
		number("blendSoftness", "Blend Softness", {
			defaultValue: 0.22,
			min: 0,
			max: 1,
			step: 0.01,
		}),
	],
	"fluid-simulation": [
		number("dissipation", "Dissipation", {
			defaultValue: 0.96,
			min: 0.85,
			max: 0.995,
			step: 0.001,
		}),
		number("pointerSize", "Pointer Size", {
			defaultValue: 0.005,
			min: 0.001,
			max: 0.03,
			step: 0.001,
		}),
		color("color", "Color", "#ff6900"),
		number("velocityDissipation", "Velocity Dissipation", {
			defaultValue: 0.96,
			min: 0.85,
			max: 0.995,
			step: 0.001,
		}),
		number("pressureIterations", "Pressure Iterations", {
			defaultValue: 10,
			min: 1,
			max: 30,
			step: 1,
		}),
	],
	"glass-pane": [
		number("distortion", "Distortion", {
			defaultValue: 0.5,
			min: 0,
			max: 2,
			step: 0.01,
		}),
		number("chromaticAberration", "Chromatic Aberration", {
			defaultValue: 0.005,
			min: 0,
			max: 0.03,
			step: 0.001,
		}),
		number("waviness", "Waviness", {
			defaultValue: 0,
			min: 0,
			max: 0.25,
			step: 0.01,
		}),
		number("speed", "Speed", {
			defaultValue: 0.5,
			min: 0,
			max: 2,
			step: 0.01,
		}),
		number("rods", "Rods", { defaultValue: 3, min: 1, max: 12, step: 1 }),
	],
	"glitter-cloth": [
		color("color", "Color", "#222326"),
		number("speed", "Speed", { defaultValue: 1, min: 0, max: 3, step: 0.01 }),
		number("brightness", "Brightness", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		number("blendStrength", "Blend Strength", {
			defaultValue: 0.02,
			min: 0,
			max: 0.2,
			step: 0.001,
		}),
		number("noiseScale", "Noise Scale", {
			defaultValue: 4,
			min: 1,
			max: 12,
			step: 0.1,
		}),
		number("vignetteStrength", "Vignette Strength", {
			defaultValue: 50,
			min: 0,
			max: 80,
			step: 1,
		}),
		number("vignettePower", "Vignette Power", {
			defaultValue: 0.5,
			min: 0,
			max: 2,
			step: 0.01,
		}),
	],
	globe: [
		number("radius", "Radius", {
			defaultValue: 2,
			min: 1,
			max: 3,
			step: 0.1,
		}),
		number("pointCount", "Point Count", {
			defaultValue: 25000,
			min: 5000,
			max: 60000,
			step: 1000,
		}),
		number("pointSize", "Point Size", {
			defaultValue: 0.05,
			min: 0.01,
			max: 0.12,
			step: 0.001,
		}),
		color("landPointColor", "Land Points", "#f77114"),
		bool("autoRotate", "Auto Rotate", true),
		bool("lockedPolarAngle", "Lock Polar Angle", false),
	],
	"god-rays": [
		color("color", "Color", "#FFFFFF"),
		number("intensity", "Intensity", {
			defaultValue: 0.35,
			min: 0,
			max: 2,
			step: 0.01,
		}),
		number("speed", "Speed", { defaultValue: 1, min: 0, max: 3, step: 0.01 }),
		number("lightSpread", "Light Spread", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		number("rayLength", "Ray Length", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		bool("pulsating", "Pulsating", false),
	],
	halo: [
		number("rotationSpeed", "Rotation Speed", {
			defaultValue: 0.5,
			min: 0,
			max: 2,
			step: 0.01,
		}),
		color("backgroundColor", "Background", "#17181A"),
		number("cameraDistance", "Camera Distance", {
			defaultValue: 3,
			min: 1,
			max: 8,
			step: 0.1,
		}),
		number("fov", "FOV", { defaultValue: 55, min: 20, max: 90, step: 1 }),
		number("intensity", "Intensity", {
			defaultValue: 2.2,
			min: 0,
			max: 5,
			step: 0.1,
		}),
	],
	"interactive-grid": [
		number("grid", "Grid", { defaultValue: 15, min: 4, max: 40, step: 1 }),
		number("mouseSize", "Mouse Size", {
			defaultValue: 0.15,
			min: 0.01,
			max: 0.5,
			step: 0.01,
		}),
		number("strength", "Strength", {
			defaultValue: 0.35,
			min: 0,
			max: 1,
			step: 0.01,
		}),
		number("relaxation", "Relaxation", {
			defaultValue: 0.9,
			min: 0.5,
			max: 0.99,
			step: 0.01,
		}),
	],
	"infinite-gallery": [
		number("speed", "Speed", {
			defaultValue: 1,
			min: 0,
			max: 4,
			step: 0.01,
		}),
		number("visibleCount", "Visible Count", {
			defaultValue: 8,
			min: 3,
			max: 16,
			step: 1,
		}),
	],
	"infinite-physics-gallery": [
		number("cellWidth", "Cell Width", {
			defaultValue: 280,
			min: 160,
			max: 520,
			step: 1,
		}),
		number("cellHeight", "Cell Height", {
			defaultValue: 360,
			min: 180,
			max: 640,
			step: 1,
		}),
		number("gap", "Gap", {
			defaultValue: 20,
			min: 0,
			max: 80,
			step: 1,
		}),
		number("friction", "Friction", {
			defaultValue: 0.95,
			min: 0.8,
			max: 0.995,
			step: 0.001,
		}),
		number("wheelSpeed", "Wheel Speed", {
			defaultValue: 1.4,
			min: 0.2,
			max: 4,
			step: 0.01,
		}),
		number("edgeThreshold", "Edge Threshold", {
			defaultValue: 120,
			min: 0,
			max: 240,
			step: 1,
		}),
		number("edgeScrollSpeed", "Edge Speed", {
			defaultValue: 1.6,
			min: 0,
			max: 4,
			step: 0.01,
		}),
	],
	"image-trail": [
		number("mouseThreshold", "Mouse Threshold", {
			defaultValue: 40,
			min: 5,
			max: 120,
			step: 1,
		}),
		number("imageLifespan", "Image Lifespan", {
			defaultValue: 600,
			min: 100,
			max: 2000,
			step: 50,
			unit: "ms",
		}),
		number("inDuration", "In Duration", {
			defaultValue: 600,
			min: 100,
			max: 2000,
			step: 50,
			unit: "ms",
		}),
		number("outDuration", "Out Duration", {
			defaultValue: 800,
			min: 100,
			max: 2400,
			step: 50,
			unit: "ms",
		}),
		number("baseRotation", "Base Rotation", {
			defaultValue: 30,
			min: -90,
			max: 90,
			step: 1,
		}),
		number("maxRotationFactor", "Rotation Factor", {
			defaultValue: 3,
			min: 0,
			max: 8,
			step: 0.1,
		}),
		number("minImageSize", "Min Image Size", {
			defaultValue: 260,
			min: 120,
			max: 480,
			step: 1,
		}),
		number("maxImageSize", "Max Image Size", {
			defaultValue: 340,
			min: 160,
			max: 640,
			step: 1,
		}),
	],
	"macos-dock": [
		number("baseWidth", "Base Width", {
			defaultValue: 4,
			min: 2,
			max: 7,
			step: 0.1,
		}),
		number("magnification", "Magnification", {
			defaultValue: 1.5,
			min: 1,
			max: 2.5,
			step: 0.01,
		}),
		number("distance", "Distance", {
			defaultValue: 3,
			min: 1,
			max: 6,
			step: 1,
		}),
	],
	magnetic: [
		number("duration", "Duration", {
			defaultValue: 1,
			min: 0.1,
			max: 2,
			step: 0.01,
			unit: "s",
		}),
		select("ease", "Ease", "elastic.out(1, 0.3)", [
			{ label: "Elastic", value: "elastic.out(1, 0.3)" },
			{ label: "Power 3", value: "power3.out" },
			{ label: "Expo", value: "expo.out" },
			{ label: "Back", value: "back.out(1.7)" },
		]),
	],
	marquee: [
		number("duration", "Duration", {
			defaultValue: 20,
			min: 4,
			max: 40,
			step: 0.5,
			unit: "s",
		}),
		number("velocity", "Velocity", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		number("repeat", "Repeat", {
			defaultValue: 3,
			min: 2,
			max: 8,
			step: 1,
		}),
		number("gap", "Gap", {
			defaultValue: 0,
			min: 0,
			max: 64,
			step: 1,
		}),
	],
	"lava-lamp": [
		color("color", "Color", "#17181A"),
		color("fresnelColor", "Fresnel Color", "#ff6900"),
		number("speed", "Speed", { defaultValue: 1, min: 0, max: 3, step: 0.01 }),
		number("fresnelPower", "Fresnel Power", {
			defaultValue: 3,
			min: 0.5,
			max: 8,
			step: 0.1,
		}),
		number("radius", "Radius", {
			defaultValue: 1,
			min: 0.5,
			max: 2,
			step: 0.1,
		}),
		number("smoothness", "Smoothness", {
			defaultValue: 0.1,
			min: 0,
			max: 0.5,
			step: 0.01,
		}),
	],
	"glass-slideshow": [
		number("transitionDuration", "Transition Duration", {
			defaultValue: 2000,
			min: 300,
			max: 5000,
			step: 100,
			unit: "ms",
		}),
		number("intensity", "Intensity", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		number("distortion", "Distortion", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		number("chromaticAberration", "Chromatic Aberration", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		number("refraction", "Refraction", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
		bool("autoplay", "Autoplay", true),
		number("autoplayInterval", "Autoplay Interval", {
			defaultValue: 5000,
			min: 1000,
			max: 10000,
			step: 500,
			unit: "ms",
		}),
	],
	"flip-grid": [
		number("columns", "Columns", {
			defaultValue: 4,
			min: 1,
			max: 4,
			step: 1,
		}),
		number("duration", "Duration", {
			defaultValue: 0.5,
			min: 0.05,
			max: 2,
			step: 0.01,
			unit: "s",
		}),
		number("stagger", "Stagger", {
			defaultValue: 0.1,
			min: 0,
			max: 0.5,
			step: 0.001,
			unit: "s",
		}),
		select("ease", "Ease", "power2.inOut", [
			{ label: "Power 2", value: "power2.inOut" },
			{ label: "Power 4", value: "power4.inOut" },
			{ label: "Expo", value: "expo.inOut" },
			{ label: "Back", value: "back.out(1.7)" },
		]),
	],
	"logo-carousel": [
		number("columnCount", "Columns", {
			defaultValue: 3,
			min: 1,
			max: 5,
			step: 1,
		}),
		number("cycleInterval", "Cycle Interval", {
			defaultValue: 2000,
			min: 500,
			max: 5000,
			step: 100,
			unit: "ms",
		}),
	],
	"neural-noise": [
		number("speed", "Speed", { defaultValue: 0.5, min: 0, max: 3, step: 0.01 }),
	],
	"pixelated-image": [
		number("initialGridSize", "Initial Grid Size", {
			defaultValue: 2,
			min: 1,
			max: 20,
			step: 1,
		}),
		number("stepDuration", "Step Duration", {
			defaultValue: 0.2,
			min: 0.03,
			max: 1,
			step: 0.01,
			unit: "s",
		}),
	],
	"plasma-grid": [
		color("color", "Background", "#17181A"),
		color("highlightColor", "Highlight", "#572400"),
	],
	"rubiks-cube": [
		number("size", "Size", { defaultValue: 1, min: 0.5, max: 2, step: 0.1 }),
		number("duration", "Duration", {
			defaultValue: 1.5,
			min: 0.2,
			max: 4,
			step: 0.1,
			unit: "s",
		}),
		number("gap", "Gap", {
			defaultValue: 0.015,
			min: 0,
			max: 0.08,
			step: 0.001,
		}),
		number("radius", "Radius", {
			defaultValue: 0.125,
			min: 0,
			max: 0.3,
			step: 0.01,
		}),
	],
	"radial-gallery": [
		number("radius", "Radius", {
			defaultValue: 850,
			min: 300,
			max: 1200,
			step: 10,
		}),
		number("duration", "Duration", {
			defaultValue: 70,
			min: 10,
			max: 120,
			step: 1,
			unit: "s",
		}),
		bool("reversed", "Reversed", false),
		number("offset", "Offset", {
			defaultValue: -600,
			min: -1000,
			max: 200,
			step: 10,
		}),
		number("gap", "Gap", {
			defaultValue: 100,
			min: 0,
			max: 240,
			step: 1,
		}),
		number("elementSize", "Element Size", {
			defaultValue: 160,
			min: 80,
			max: 320,
			step: 1,
		}),
	],
	"split-reveal": [
		select("mode", "Mode", "lines", [
			{ label: "Lines", value: "lines" },
			{ label: "Words", value: "words" },
			{ label: "Chars", value: "chars" },
		]),
		number("duration", "Duration", {
			defaultValue: 0.8,
			min: 0.1,
			max: 2,
			step: 0.01,
			unit: "s",
		}),
		number("stagger", "Stagger", {
			defaultValue: 0.08,
			min: 0,
			max: 0.2,
			step: 0.001,
			unit: "s",
		}),
		number("delay", "Delay", {
			defaultValue: 0,
			min: 0,
			max: 1,
			step: 0.01,
			unit: "s",
		}),
	],
	"stacking-words": [
		number("scrub", "Scrub", {
			defaultValue: 1.234,
			min: 0,
			max: 3,
			step: 0.001,
		}),
		number("stagger", "Stagger", {
			defaultValue: 0.21,
			min: 0,
			max: 0.5,
			step: 0.001,
			unit: "s",
		}),
	],
	"specular-band": [
		color("color", "Color", "#FF6900"),
		color("backgroundColor", "Background", "#17181A"),
		number("speed", "Speed", { defaultValue: 0.8, min: 0, max: 3, step: 0.01 }),
		number("distortion", "Distortion", {
			defaultValue: 0.2,
			min: 0,
			max: 1,
			step: 0.01,
		}),
		number("hueShift", "Hue Shift", {
			defaultValue: 30,
			min: 0,
			max: 360,
			step: 1,
		}),
		number("intensity", "Intensity", {
			defaultValue: 1,
			min: 0,
			max: 3,
			step: 0.01,
		}),
	],
	"water-ripple": [
		number("brushSize", "Brush Size", {
			defaultValue: 100,
			min: 20,
			max: 240,
			step: 1,
		}),
	],
	"text-loop": [
		number("interval", "Interval", {
			defaultValue: 2000,
			min: 500,
			max: 5000,
			step: 100,
			unit: "ms",
		}),
	],
	"text-scramble": [
		number("scrambleDuration", "Duration", {
			defaultValue: 0.6,
			min: 0.1,
			max: 2,
			step: 0.01,
			unit: "s",
		}),
		number("stagger", "Stagger", {
			defaultValue: 0.03,
			min: 0,
			max: 0.12,
			step: 0.001,
			unit: "s",
		}),
		number("cycles", "Cycles", {
			defaultValue: 12,
			min: 1,
			max: 30,
			step: 1,
		}),
		text(
			"characters",
			"Characters",
			"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*",
		),
	],
	"video-player": [
		bool("autoplay", "Autoplay", false),
		bool("muted", "Muted", true),
		bool("loop", "Loop", true),
		bool("hideControls", "Hide Controls", false),
	],
	"weight-wave": [
		number("baseWeight", "Base Weight", {
			defaultValue: 200,
			min: 100,
			max: 700,
			step: 10,
		}),
		number("hoverWeight", "Hover Weight", {
			defaultValue: 800,
			min: 300,
			max: 1000,
			step: 10,
		}),
		number("influenceRadius", "Influence Radius", {
			defaultValue: 3,
			min: 0,
			max: 8,
			step: 1,
		}),
		number("falloffPower", "Falloff Power", {
			defaultValue: 1.5,
			min: 0.25,
			max: 4,
			step: 0.01,
		}),
		number("duration", "Duration", {
			defaultValue: 1,
			min: 0.05,
			max: 2,
			step: 0.01,
			unit: "s",
		}),
	],
} satisfies Record<string, PreviewControlPreset>;
