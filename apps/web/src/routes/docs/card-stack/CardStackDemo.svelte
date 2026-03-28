<script lang="ts">
	import { CardStack, CardStackItem } from "motion-core";
	import { onMount } from "svelte";

	const items = [
		"/images/demos/sample-9.jpg",
		"/images/demos/sample-12.jpg",
		"/images/demos/sample-15.jpg",
		"/images/demos/sample-5.jpg",
	];
	let scrollElement = $state<HTMLElement | null>(null);

	onMount(() => {
		scrollElement = document.getElementById("component-preview-live");
	});
</script>

<div
	class="flex h-full min-h-150 w-full items-center justify-center text-center"
>
	<p class="text-3xl font-medium tracking-tight text-foreground">Scroll Down</p>
</div>
<CardStack
	{scrollElement}
	offset={32}
	topOffset={32}
	class="flex flex-col gap-8"
>
	{#each items as item (item)}
		<CardStackItem
			class="inset-shadow mx-auto flex h-96 w-80 flex-col rounded-lg bg-background-inset p-1.5 md:w-120 md:group-data-[fullscreen=true]/preview:h-dvh md:group-data-[fullscreen=true]/preview:w-7xl"
		>
			<div class="card relative flex-1 overflow-hidden rounded-md">
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
	class="flex h-full min-h-150 w-full items-center justify-center text-center"
>
	<p class="text-3xl font-medium tracking-tight text-foreground">
		Nothing Here
	</p>
</div>
