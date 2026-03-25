<script lang="ts">
	import { StackingWords } from "motion-core";
	import { onMount } from "svelte";

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

<StackingWords
	{scrollElement}
	class="mx-auto w-sm p-4 text-2xl md:w-3xl md:text-5xl"
>
	<p>
		We craft interfaces where motion carries meaning, guiding each transition
		with rhythm, contrast, and intention so every section unfolds with clarity
		as you scroll.
	</p>
</StackingWords>

<div
	class="flex h-full min-h-150 w-full items-center justify-center text-center"
>
	<p class="text-3xl font-medium tracking-tight text-foreground">You Made It</p>
</div>
