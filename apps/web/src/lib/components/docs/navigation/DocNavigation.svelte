<script lang="ts">
	import DocNavButton from "./DocNavButton.svelte";
	import { docsUiConfig } from "$lib/config/docs-ui";

	export type DocNavLink = {
		title: string;
		href: string;
	};

	const props = $props<{
		previous?: DocNavLink | null;
		next?: DocNavLink | null;
	}>();
	const previous = $derived(props.previous ?? null);
	const next = $derived(props.next ?? null);
</script>

{#if docsUiConfig.pagination.enabled && (previous || next)}
	<nav class="relative mt-16 border-t border-border pt-9">
		<div class="grid gap-4 sm:grid-cols-2">
			{#if previous}
				<DocNavButton
					label={docsUiConfig.pagination.previousLabel}
					{...previous}
				/>
			{/if}

			{#if next}
				<DocNavButton
					label={docsUiConfig.pagination.nextLabel}
					align="right"
					forceSecondColumn={!previous}
					{...next}
				/>
			{/if}
		</div>
	</nav>
{/if}
