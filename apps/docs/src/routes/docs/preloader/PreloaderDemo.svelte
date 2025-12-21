<script lang="ts">
	import { Preloader } from "motion-core";

	const images = [
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_1.jpg",
			alt: "Alt 1",
		},
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_10.jpg",
			alt: "Alt 2",
		},
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_7.jpg",
			alt: "Alt 3",
		},
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_4.jpg",
			alt: "Alt 4",
		},
		{
			src: "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_5.jpg",
			alt: "Alt 5",
		},
	];

	let showPreloader = $state(false);

	function startPreloader() {
		showPreloader = true;
	}

	function portal(node: HTMLElement) {
		document.body.appendChild(node);
		return {
			destroy() {
				if (node.parentNode) {
					node.parentNode.removeChild(node);
				}
			},
		};
	}
</script>

<div class="flex flex-col items-center justify-center">
	<button
		onclick={startPreloader}
		class="rounded-full border bg-card px-3 py-1 text-xs uppercase tracking-wide border-border text-foreground/70"
	>
		Trigger Preloader
	</button>

	{#if showPreloader}
		<div use:portal>
			<Preloader {images} onComplete={() => (showPreloader = false)} />
		</div>
	{/if}
</div>
