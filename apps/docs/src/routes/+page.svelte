<script lang="ts">
	import { ComponentGrid, Sidebar } from "$lib";

	const socialLinks = [
		{ label: "GitHub", href: "https://github.com/motion-core" },
		{ label: "Community", href: "https://discord.gg/motion-core" },
		{ label: "Docs", href: "/docs" },
	];

	const mockComponents = [
		{ name: "Nebula Background", image: "/og-image.jpg" },
		{ name: "Aurora Curtain", image: "/og-image.jpg" },
		{ name: "Magnetic Text", image: "/og-image.jpg" },
		{ name: "Glyph Trail", image: "/og-image.jpg" },
		{ name: "Prism Grid", image: "/og-image.jpg" },
		{ name: "Particle Vortex", image: "/og-image.jpg" },
		{ name: "Nebula Background", image: "/og-image.jpg" },
		{ name: "Aurora Curtain", image: "/og-image.jpg" },
		{ name: "Magnetic Text", image: "/og-image.jpg" },
		{ name: "Glyph Trail", image: "/og-image.jpg" },
		{ name: "Prism Grid", image: "/og-image.jpg" },
		{ name: "Particle Vortex", image: "/og-image.jpg" },
	];

	const landingDescription =
		"Motion Core is your playground of GSAP primitives and Three.js canvases ready to drop into Svelte experiences. Compose storytelling backdrops, kinetic typography, and interactive UI states without starting from scratch.";
</script>

<main class="min-h-svh w-full bg-background p-4">
	<div class="mx-auto grid h-full gap-6 lg:grid-cols-4">
		<Sidebar description={landingDescription} {socialLinks} />
		<ComponentGrid components={mockComponents} />
	</div>
</main>
