<script lang="ts">
	import ComponentCard from "./ComponentCard.svelte";
	import type { ComponentInfo } from "./types";

	export let components: ComponentInfo[] = [];
</script>

<section
	class="relative lg:col-span-3 lg:max-h-[calc(100svh-2rem)] rounded-xl bg-background border border-border card-highlight"
>
	<div
		class="w-full h-full p-4 lg:overflow-y-auto"
		style="mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent); -webkit-mask-image: linear-gradient(to bottom, transparent, black 16px, black calc(100% - 16px), transparent);"
	>
		<div class="columns-1 gap-4 space-y-4 sm:columns-2 lg:columns-2 [column-fill:balance]">
			{#each components as component}
				<ComponentCard {component} />
			{/each}
		</div>
	</div>
</section>
