<script lang="ts">
	import { FlipCardStack } from "motion-core";

	type DemoCard = {
		image: string;
		alt: string;
		title: string;
		description: string;
	};

	const items: DemoCard[] = [
		{
			image: "/images/demos/sample-9.jpg",
			alt: "Blue and orange abstract texture",
			title: "Frame One",
			description: "Drag the top card and release past threshold.",
		},
		{
			image: "/images/demos/sample-12.jpg",
			alt: "Neon gradient waves",
			title: "Frame Two",
			description: "Only the top card is draggable at any moment.",
		},
		{
			image: "/images/demos/sample-15.jpg",
			alt: "Warm red and yellow gradient",
			title: "Frame Three",
			description: "Content layout is fully controlled in the snippet.",
		},
	];
</script>

<div class="flex min-h-150 w-full items-center justify-center p-8">
	<FlipCardStack {items} stackOffset={10} stackRotation={-8}>
		{#snippet children(item)}
			<div class="relative h-96 w-[320px] overflow-hidden rounded-lg">
				<img
					src={item.image}
					alt={item.alt}
					class="pointer-events-none h-full w-full object-cover select-none"
				/>
				<div
					class="absolute inset-x-0 bottom-0 bg-linear-to-t from-black/80 via-black/35 to-transparent p-6 text-left text-white"
				>
					<h3 class="text-lg font-medium text-white">{item.title}</h3>
					<p class="mt-1 text-sm text-white/80">{item.description}</p>
				</div>
			</div>
		{/snippet}
	</FlipCardStack>
</div>
