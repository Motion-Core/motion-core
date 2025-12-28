<script lang="ts">
	import { ComponentGrid, Sidebar } from "$lib";
	import type { PageData } from "./$types";

	const { data } = $props<{ data: PageData }>();

	const socialLinks = [
		{ label: "GitHub", href: "https://github.com/motion-core" },
		{ label: "Community", href: "https://discord.gg/motion-core" },
		{ label: "Docs", href: "/docs/introduction" },
	];

	const landingDescription =
		"Animated Svelte component library powered by GSAP and Three.js. Drop-in solutions for motion design, 3D canvases, and interactive animations — ready to use, easy to customize.";

	const components = $derived(data.components ?? []);
</script>

<main class="h-svh w-full p-4">
	<div class="mx-auto grid h-full gap-4 lg:grid-cols-4">
		<Sidebar description={landingDescription} {socialLinks} />
		<ComponentGrid {components} />
	</div>
</main>
