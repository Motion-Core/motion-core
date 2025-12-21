<script lang="ts">
	import type { ComponentInfo } from "./types";

	export let component: ComponentInfo;
</script>

<a
	href={`/docs/${component.slug}`}
	class="relative block break-inside-avoid rounded-xl border border-border bg-card shadow-sm card-highlight"
	data-sveltekit-preload-data
	data-component-card
>
	<div class="relative rounded-xl overflow-hidden">
		<img
			loading="lazy"
			alt={`${component.name} preview`}
			src={component.image}
			class="h-56 w-full object-cover"
		/>
		<div class="pointer-events-none absolute bottom-4 left-4 text-xs font-mono uppercase tracking-wide text-white mix-blend-difference">
			{component.name}
		</div>
	</div>
</a>
