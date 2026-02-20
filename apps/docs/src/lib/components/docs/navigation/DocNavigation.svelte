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
	<nav
		class="relative mt-16 pt-9 before:absolute before:top-0 before:left-0 before:h-px before:w-full before:border-0 before:shadow-2xs dark:before:bg-black dark:before:shadow-border light:before:bg-black/15 light:before:shadow-white"
	>
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
