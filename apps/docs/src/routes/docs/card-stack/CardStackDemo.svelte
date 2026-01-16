<script lang="ts">
	import { CardStack, CardStackItem } from "motion-core";
	const items = [
		"/images/demos/sample-9.jpg",
		"/images/demos/sample-12.jpg",
		"/images/demos/sample-15.jpg",
		"/images/demos/sample-5.jpg",
	];
	let scrollElement: HTMLElement;
</script>

<div
	bind:this={scrollElement}
	class="h-150 w-full overflow-y-auto group-data-[fullscreen=true]:h-full"
>
	<div
		class="flex h-full min-h-96 w-full items-center justify-center text-center"
	>
		<p class="text-3xl font-medium tracking-tight text-foreground font-display">
			Scroll Down
		</p>
	</div>
	<CardStack {scrollElement} offset={40} topOffset={32}>
		{#each items as item (item)}
			<CardStackItem
				class="card-highlight mx-auto mb-8 flex h-96 w-96 flex-col rounded-xl border border-border bg-background p-2 shadow-sm md:w-120 md:group-data-[fullscreen=true]:h-150 md:group-data-[fullscreen=true]:w-220"
			>
				<div
					class="relative flex-1 overflow-hidden rounded-lg border border-border/60"
				>
					<img
						src={item}
						alt="Sample"
						class="absolute inset-0 h-full w-full object-cover"
					/>
				</div>
			</CardStackItem>
		{/each}
	</CardStack>
	<div
		class="flex h-full min-h-96 w-full items-center justify-center text-center"
	>
		<p class="text-3xl font-medium tracking-tight text-foreground font-display">
			Nothing Here
		</p>
	</div>
</div>
