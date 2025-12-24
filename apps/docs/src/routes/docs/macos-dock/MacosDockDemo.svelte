<script lang="ts">
	import { MacosDock } from "motion-core";

	const items = [
		{
			src: "/components/macos-dock/notion.png",
			alt: "Notion",
			label: "Notion",
			href: "#",
		},
		{
			src: "/components/macos-dock/asana.png",
			alt: "Asana",
			label: "Asana",
			href: "#",
		},
		{
			src: "/components/macos-dock/slack.png",
			alt: "Slack",
			label: "Slack",
			href: "#",
		},
		{
			src: "/components/macos-dock/loom.png",
			alt: "Loom",
			label: "Loom",
			href: "#",
		},
		{
			src: "/components/macos-dock/spotify.png",
			alt: "Spotify",
			label: "Spotify",
			href: "#",
		},
		{
			src: "/components/macos-dock/illustrator.png",
			alt: "Adobe Illustrator",
			label: "Adobe Illustrator",
			href: "#",
		},
		{
			src: "/components/macos-dock/figma.png",
			alt: "Figma",
			label: "Figma",
			href: "#",
		},
		{
			src: "/components/macos-dock/photoshop.png",
			alt: "Adobe Photoshop",
			label: "Adobe Photoshop",
			href: "#",
		},
		{
			src: "/components/macos-dock/premiere.png",
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
