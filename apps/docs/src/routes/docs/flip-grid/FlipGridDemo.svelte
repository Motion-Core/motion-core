<script lang="ts">
	import { FlipGrid, FlipGridItem } from "motion-core";
	import { cn } from "$lib/utils/cn";

	let columns = $state(4);

	const images = [
		"/images/demos/sample-8.jpg",
		"/images/demos/sample-9.jpg",
		"/images/demos/sample-10.jpg",
		"/images/demos/sample-11.jpg",
	];
</script>

<div class="relative h-full w-full min-h-96">
	<div class="p-4 h-full">
		<FlipGrid class="gap-4 w-full" {columns} duration={0.5} stagger={0.1}>
			{#each images as src, i (src)}
				<FlipGridItem
					id={`img-${i}`}
					class="relative w-full aspect-square rounded-lg overflow-hidden shadow-md"
				>
					<img {src} alt="Demo {i}" class="w-full h-full object-cover" />
				</FlipGridItem>
			{/each}
		</FlipGrid>
	</div>

	<div
		class="absolute bottom-4 left-1/2 flex w-full -translate-x-1/2 flex-wrap justify-center gap-2"
	>
		{#each [1, 2, 4] as col (col)}
			<button
				class={cn(
					"h-8 gap-1.5 rounded-md border bg-card px-3 text-xs font-medium tracking-wide uppercase shadow-sm",
					columns === col
						? "border-accent text-foreground"
						: "border-border text-foreground/70 hover:text-foreground",
				)}
				onclick={() => (columns = col)}
			>
				{col} Cols
			</button>
		{/each}
	</div>
</div>
