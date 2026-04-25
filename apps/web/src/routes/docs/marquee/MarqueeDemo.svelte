<script lang="ts">
	import { Marquee } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof Marquee>>;

	const texts = [
		"Motion・",
		"Craft・",
		"Ecosystem・",
		"Initiative・",
		"Transition・",
		"Interaction・",
		"Experience・",
		"Playground・",
		"Journey・",
	];

	let { duration = 20, velocity = 1, repeat = 3, gap = 0 }: Props = $props();
</script>

<div
	class="flex h-full min-h-96 w-full flex-col items-center justify-center gap-4"
>
	<Marquee
		class="h-fit"
		{gap}
		{duration}
		{velocity}
		{repeat}
		scrollElement="#docs-content-container"
	>
		{#each texts as text (text)}
			<div class="flex items-center text-4xl font-medium">
				{text}
			</div>
		{/each}
	</Marquee>

	<Marquee
		class="h-fit"
		{gap}
		{duration}
		{velocity}
		{repeat}
		reversed
		scrollElement="#docs-content-container"
	>
		{#each texts as text (text)}
			<div class="flex items-center text-4xl font-medium">
				{text}
			</div>
		{/each}
	</Marquee>
</div>
