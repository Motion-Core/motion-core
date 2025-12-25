<script lang="ts">
	import { Preloader } from "motion-core";

	const images = [
		{
			src: "/images/demos/sample-1.jpg",
			alt: "Alt 1",
		},
		{
			src: "/images/demos/sample-2.jpg",
			alt: "Alt 2",
		},
		{
			src: "/images/demos/sample-6.jpg",
			alt: "Alt 3",
		},
		{
			src: "/images/demos/sample-4.jpg",
			alt: "Alt 4",
		},
		{
			src: "/images/demos/sample-5.jpg",
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
		class="border bg-card h-8 rounded-md gap-1.5 px-3 shadow-sm text-xs font-medium uppercase tracking-wide border-border text-foreground"
	>
		Trigger Preloader
	</button>

	{#if showPreloader}
		<div use:portal>
			<Preloader
				class="bg-card"
				{images}
				onComplete={() => (showPreloader = false)}
			/>
		</div>
	{/if}
</div>
