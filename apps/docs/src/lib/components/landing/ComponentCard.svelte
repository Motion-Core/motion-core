<script lang="ts">
	import type { ComponentInfo } from "./types";

	export let component: ComponentInfo;
</script>

<article
	class="break-inside-avoid overflow-hidden rounded-xl border border-border bg-card p-1 flex flex-col items-center justify-center gap-2 shadow-md"
>
	<div class="relative h-56 w-full bg-muted">
		<img
			loading="lazy"
			alt={`${component.name} preview`}
			src={component.image}
			class="h-full w-full object-cover rounded-lg"
		/>
	</div>
	<button
		class="w-full px-4 py-3 text-xs font-medium uppercase tracking-wide text-foreground rounded-lg bg-card-muted hover:bg-card-elevated transition-[background-color] duration-150 ease-out"
	>
		{component.name}
	</button>
</article>
