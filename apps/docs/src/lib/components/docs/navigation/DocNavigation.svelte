<script lang="ts">
	import DocNavButton from "./DocNavButton.svelte";

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

{#if previous || next}
	<nav class="mt-16 border-t border-border pt-8">
		<div class="grid gap-4 sm:grid-cols-2">
			{#if previous}
				<DocNavButton label="Previous" {...previous} />
			{/if}

			{#if next}
				<DocNavButton
					label="Next"
					align="right"
					forceSecondColumn={!previous}
					{...next}
				/>
			{/if}
		</div>
	</nav>
{/if}
