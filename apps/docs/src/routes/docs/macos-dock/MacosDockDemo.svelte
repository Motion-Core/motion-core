<script lang="ts">
	import { MacosDock } from "motion-core";

	const items = [
		{
			src: "/images/demos/logo-notion.png",
			alt: "Notion",
			label: "Notion",
			href: "#",
		},
		{
			src: "/images/demos/logo-asana.png",
			alt: "Asana",
			label: "Asana",
			href: "#",
		},
		{
			src: "/images/demos/logo-slack.png",
			alt: "Slack",
			label: "Slack",
			href: "#",
		},
		{
			src: "/images/demos/logo-loom.png",
			alt: "Loom",
			label: "Loom",
			href: "#",
		},
		{
			src: "/images/demos/logo-spotify.png",
			alt: "Spotify",
			label: "Spotify",
			href: "#",
		},
		{
			src: "/images/demos/logo-illustrator.png",
			alt: "Adobe Illustrator",
			label: "Adobe Illustrator",
			href: "#",
		},
		{
			src: "/images/demos/logo-figma.png",
			alt: "Figma",
			label: "Figma",
			href: "#",
		},
		{
			src: "/images/demos/logo-photoshop.png",
			alt: "Adobe Photoshop",
			label: "Adobe Photoshop",
			href: "#",
		},
		{
			src: "/images/demos/logo-premiere.png",
			alt: "Premiere Pro",
			label: "Premiere Pro",
			href: "#",
		},
	];
</script>

<div
	class="w-full h-full min-h-80 flex items-center justify-center relative overflow-hidden"
>
	<div class="absolute bottom-0">
		<MacosDock {items} />
	</div>
</div>
