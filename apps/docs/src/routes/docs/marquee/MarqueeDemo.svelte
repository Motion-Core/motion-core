<script lang="ts">
	import { Marquee } from "motion-core";

	const companies = [
		"Microsoft",
		"Amazon",
		"Netflix",
		"YouTube",
		"Instagram",
		"Uber",
		"Spotify",
	];
</script>

<div
	class="h-full min-h-80 w-full flex flex-col items-center justify-center gap-4"
>
	<Marquee class="h-fit" gap={32} duration={20} velocity={1}>
		{#each companies as company (company)}
			<div class="flex items-center text-4xl font-medium">
				{company}
			</div>
		{/each}
	</Marquee>

	<Marquee class="h-fit" gap={32} duration={20} velocity={1} reversed>
		{#each companies as company (company)}
			<div class="flex items-center text-4xl font-medium">
				{company}
			</div>
		{/each}
	</Marquee>
</div>
