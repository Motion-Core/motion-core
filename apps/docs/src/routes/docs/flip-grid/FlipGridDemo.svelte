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
		class="absolute bottom-4 left-1/2 flex -translate-x-1/2 justify-center gap-1 bg-background border border-border w-fit p-1 rounded-lg shadow-sm"
	>
		{#each [1, 2, 4] as col (col)}
			<button
				class={cn(
					"py-1 gap-1.5 whitespace-nowrap rounded-md px-3 text-xs font-medium tracking-wide uppercase transition-colors duration-150 ease-out",
					columns === col
						? "bg-accent light:text-card dark:text-foreground"
						: "text-foreground/70 hover:text-foreground",
				)}
				onclick={() => (columns = col)}
			>
				{col} Cols
			</button>
		{/each}
	</div>
</div>
