<script lang="ts">
	import { Marquee } from "motion-core";

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
</script>

<div
	class="flex h-full min-h-96 w-full flex-col items-center justify-center gap-4"
>
	<Marquee
		class="h-fit"
		gap={0}
		duration={20}
		velocity={1}
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
		gap={0}
		duration={20}
		velocity={1}
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
