import type { DocItem } from "$lib/types/doc";

/**
 * Manual documentation navigation tree.
 * The order of items controls sidebar rendering and previous/next doc navigation.
 */
export const docsNavigation: DocItem[] = [
	{
		slug: "getting-started",
		name: "Getting Started",
		items: [
			{
				slug: "introduction",
				name: "Introduction",
			},
			{
				slug: "cli-guide/quick-start",
				name: "Quick Start",
			},
			{
				slug: "cli-guide/init",
				name: "Init",
			},
			{
				slug: "cli-guide/add",
				name: "Add",
			},
			{
				slug: "cli-guide/list",
				name: "List",
			},
			{
				slug: "cli-guide/cache",
				name: "Cache",
			},
		],
	},
	{
		slug: "resources",
		name: "Resources",
		items: [
			{
				slug: "changelog/registry",
				name: "Registry Changelog",
			},
			{
				slug: "changelog/cli",
				name: "CLI Changelog",
			},
		],
	},
	{
		slug: "canvas",
		name: "Canvas",
		items: [
			{
				slug: "ascii-renderer",
				name: "ASCII Renderer",
			},
			{
				slug: "card-3d",
				name: "Card 3D",
			},
			{
				slug: "dithered-image",
				name: "Dithered Image",
			},
			{
				slug: "fluid-simulation",
				name: "Fluid Simulation",
			},
			{
				slug: "glass-pane",
				name: "Glass Pane",
			},
			{
				slug: "glass-slideshow",
				name: "Glass Slideshow",
			},
			{
				slug: "glitter-cloth",
				name: "Glitter Cloth",
			},
			{
				slug: "globe",
				name: "Globe",
			},
			{
				slug: "infinite-gallery",
				name: "Infinite Gallery",
			},
			{
				slug: "interactive-grid",
				name: "Interactive Grid",
			},
			{
				slug: "lava-lamp",
				name: "Lava Lamp",
			},
			{
				slug: "neural-noise",
				name: "Neural Noise",
			},
			{
				slug: "pixelated-image",
				name: "Pixelated Image",
			},
			{
				slug: "plasma-grid",
				name: "Plasma Grid",
			},
			{
				slug: "rubiks-cube",
				name: "Rubiks Cube",
			},
			{
				slug: "water-ripple",
				name: "Water Ripple",
			},
		],
	},
	{
		slug: "layout",
		name: "Layout",
		items: [
			{
				slug: "flip-grid",
				name: "Flip Grid",
			},
		],
	},
	{
		slug: "navigation",
		name: "Navigation",
		items: [
			{
				slug: "floating-menu",
				name: "Floating Menu",
			},
		],
	},
	{
		slug: "pointer",
		name: "Pointer",
		items: [
			{
				slug: "image-trail",
				name: "Image Trail",
			},
			{
				slug: "macos-dock",
				name: "MacOS Dock",
			},
			{
				slug: "magnetic",
				name: "Magnetic",
			},
		],
	},
	{
		slug: "showcase",
		name: "Showcase",
		items: [
			{
				slug: "card-stack",
				name: "Card Stack",
			},
			{
				slug: "logo-carousel",
				name: "Logo Carousel",
			},
			{
				slug: "marquee",
				name: "Marquee",
			},
			{
				slug: "radial-gallery",
				name: "Radial Gallery",
			},
			{
				slug: "slideshow",
				name: "Slideshow",
			},
			{
				slug: "video-player",
				name: "Video Player",
			},
		],
	},
	{
		slug: "transition",
		name: "Transition",
		items: [
			{
				slug: "preloader",
				name: "Preloader",
			},
		],
	},
	{
		slug: "typography",
		name: "Typography",
		items: [
			{
				slug: "split-hover",
				name: "Split Hover",
			},
			{
				slug: "split-reveal",
				name: "Split Reveal",
			},
			{
				slug: "text-loop",
				name: "Text Loop",
			},
			{
				slug: "text-scramble",
				name: "Text Scramble",
			},
			{
				slug: "weight-wave",
				name: "Weight Wave",
			},
		],
	},
];
