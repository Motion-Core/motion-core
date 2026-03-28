type ComponentInfo = {
	slug: string;
	name: string;
	category?: string;
	introducedAt?: string;
	newUntil?: string;
	video?: string;
	poster?: string;
	dependencies?: Record<string, string>;
};

export const docsManifest: ComponentInfo[] = [
	{
		slug: "ascii-renderer",
		name: "ASCII Renderer",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "card-3d",
		name: "Card 3D",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.0.0",
			"@mediapipe/tasks-vision": "^0.10.22-rc.20250304",
			three: "^0.182.0",
		},
	},
	{
		slug: "card-stack",
		name: "Card Stack",
		category: "showcase",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "dithered-image",
		name: "Dithered Image",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "flip-card-stack",
		name: "Flip Card Stack",
		category: "pointer",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "flip-grid",
		name: "Flip Grid",
		category: "layout",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "floating-menu",
		name: "Floating Menu",
		category: "navigation",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "fluid-simulation",
		name: "Fluid Simulation",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "glass-pane",
		name: "Glass Pane",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "glass-slideshow",
		name: "Glass Slideshow",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			gsap: "^3.14.2",
			three: "^0.182.0",
		},
	},
	{
		slug: "glitter-cloth",
		name: "Glitter Cloth",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "globe",
		name: "Globe",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			gsap: "^3.14.2",
			three: "^0.182.0",
		},
	},
	{
		slug: "god-rays",
		name: "God Rays",
		category: "canvas",
		introducedAt: "2026-03-18",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "halo",
		name: "Halo",
		category: "canvas",
		introducedAt: "2026-03-18",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "image-trail",
		name: "Image Trail",
		category: "pointer",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "infinite-gallery",
		name: "Infinite Gallery",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "interactive-grid",
		name: "Interactive Grid",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "lava-lamp",
		name: "Lava Lamp",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "logo-carousel",
		name: "Logo Carousel",
		category: "showcase",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "macos-dock",
		name: "MacOS Dock",
		category: "pointer",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "magnetic",
		name: "Magnetic",
		category: "pointer",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "marquee",
		name: "Marquee",
		category: "showcase",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "neural-noise",
		name: "Neural Noise",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "pixelated-image",
		name: "Pixelated Image",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "plasma-grid",
		name: "Plasma Grid",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "preloader",
		name: "Preloader",
		category: "transition",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "radial-gallery",
		name: "Radial Gallery",
		category: "showcase",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "rubiks-cube",
		name: "Rubiks Cube",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "slideshow",
		name: "Slideshow",
		category: "showcase",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "specular-band",
		name: "Specular Band",
		category: "canvas",
		introducedAt: "2026-03-18",
		dependencies: {
			"@threlte/core": "^8.3.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "split-hover",
		name: "Split Hover",
		category: "typography",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "split-reveal",
		name: "Split Reveal",
		category: "typography",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "stacking-words",
		name: "Stacking Words",
		category: "typography",
		introducedAt: "2026-03-25",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "text-loop",
		name: "Text Loop",
		category: "typography",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "text-scramble",
		name: "Text Scramble",
		category: "typography",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "video-player",
		name: "Video Player",
		category: "showcase",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
	{
		slug: "water-ripple",
		name: "Water Ripple",
		category: "canvas",
		dependencies: {
			"@threlte/core": "^8.3.1",
			"@threlte/extras": "^9.7.1",
			three: "^0.182.0",
		},
	},
	{
		slug: "weight-wave",
		name: "Weight Wave",
		category: "typography",
		dependencies: {
			gsap: "^3.14.2",
		},
	},
];

export const getAdjacentDocs = (slug: string) => {
	const index = docsManifest.findIndex((doc) => doc.slug === slug);
	if (index === -1) {
		return { previous: null, next: null };
	}
	const previous = index > 0 ? docsManifest[index - 1] : null;
	const next = index < docsManifest.length - 1 ? docsManifest[index + 1] : null;
	return { previous, next };
};
