<script lang="ts">
	import ComponentCard from "./ComponentCard.svelte";
	import type { ComponentInfo } from "./types";

	export let components: ComponentInfo[] = [];
</script>

<section
	style="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
	class="relative lg:col-span-3 lg:max-h-[calc(100svh-2rem)] lg:overflow-y-auto lg:pr-2 py-4"
>
	<div class="columns-1 gap-4 space-y-4 sm:columns-2 lg:columns-2 [column-fill:balance]">
		{#each components as component}
			<ComponentCard {component} />
		{/each}
	</div>
</section>
