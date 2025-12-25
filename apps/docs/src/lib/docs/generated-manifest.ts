import type { ComponentInfo } from "../components/landing";

export const docsManifest: ComponentInfo[] = [
	{
		slug: "glass-pane",
		name: "Glass Pane",
		video: "/previews/glass-pane/preview.mp4",
		poster: "/previews/glass-pane/poster.webp",
	},
	{
		slug: "image-trail",
		name: "Image Trail",
		video: "/previews/image-trail/preview.mp4",
		poster: "/previews/image-trail/poster.webp",
	},
	{
		slug: "infinite-gallery",
		name: "Infinite Gallery",
		video: "/previews/infinite-gallery/preview.mp4",
		poster: "/previews/infinite-gallery/poster.webp",
	},
	{
		slug: "logo-carousel",
		name: "Logo Carousel",
		video: "/previews/logo-carousel/preview.mp4",
		poster: "/previews/logo-carousel/poster.webp",
	},
	{
		slug: "macos-dock",
		name: "MacOS Dock",
		video: "/previews/macos-dock/preview.mp4",
		poster: "/previews/macos-dock/poster.webp",
	},
	{
		slug: "magnetic",
		name: "Magnetic",
		video: "/previews/magnetic/preview.mp4",
		poster: "/previews/magnetic/poster.webp",
	},
	{
		slug: "marquee",
		name: "Marquee",
		video: "/previews/marquee/preview.mp4",
		poster: "/previews/marquee/poster.webp",
	},
	{
		slug: "pixelated-image",
		name: "Pixelated Image",
		video: "/previews/pixelated-image/preview.mp4",
		poster: "/previews/pixelated-image/poster.webp",
	},
	{
		slug: "preloader",
		name: "Preloader",
		video: "/previews/preloader/preview.mp4",
		poster: "/previews/preloader/poster.webp",
	},
	{
		slug: "slideshow",
		name: "Slideshow",
		video: "/previews/slideshow/preview.mp4",
		poster: "/previews/slideshow/poster.webp",
	},
	{
		slug: "split-hover",
		name: "Split Hover",
		video: "/previews/split-hover/preview.mp4",
		poster: "/previews/split-hover/poster.webp",
	},
	{
		slug: "split-reveal",
		name: "Split Reveal",
		video: "/previews/split-reveal/preview.mp4",
		poster: "/previews/split-reveal/poster.webp",
	},
	{
		slug: "text-loop",
		name: "Text Loop",
		video: "/previews/text-loop/preview.mp4",
		poster: "/previews/text-loop/poster.webp",
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
