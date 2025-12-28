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

<div class="relative h-full min-h-96 w-full">
	<div class="h-full p-4">
		<FlipGrid class="w-full gap-4" {columns} duration={0.5} stagger={0.1}>
			{#each images as src, i (src)}
				<FlipGridItem
					id={`img-${i}`}
					class="relative aspect-square w-full overflow-hidden rounded-lg shadow-md"
				>
					<img {src} alt="Demo {i}" class="h-full w-full object-cover" />
				</FlipGridItem>
			{/each}
		</FlipGrid>
	</div>

	<div
		class="absolute bottom-4 left-1/2 flex w-fit -translate-x-1/2 justify-center gap-1 rounded-lg border border-border bg-background p-1 shadow-sm"
	>
		{#each [1, 2, 4] as col (col)}
			<button
				class={cn(
					"gap-1.5 rounded-md px-3 py-1 text-xs font-medium tracking-wide whitespace-nowrap uppercase transition-colors duration-150 ease-out",
					columns === col
						? "bg-accent dark:text-foreground light:text-card"
						: "text-foreground/70 hover:text-foreground",
				)}
				onclick={() => (columns = col)}
			>
				{col} Cols
			</button>
		{/each}
	</div>
</div>
