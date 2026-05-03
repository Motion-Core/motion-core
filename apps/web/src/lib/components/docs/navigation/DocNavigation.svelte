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
	<nav
		class="relative mt-16 pt-9 after:pointer-events-none after:absolute after:inset-x-0 after:top-0 after:h-px after:bg-border after:shadow-2xs after:shadow-white after:content-[''] dark:after:bg-black dark:after:shadow-border"
	>
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
