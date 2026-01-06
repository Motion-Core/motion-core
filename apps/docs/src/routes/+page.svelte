<script lang="ts">
	import { ComponentGrid } from "$lib";
	import Hero from "$lib/components/landing/Hero.svelte";
	import Footer from "$lib/components/landing/Footer.svelte";
	import type { PageData } from "./$types";

	const { data } = $props<{ data: PageData }>();

	const components = $derived(data.components ?? []);
</script>

<main class="min-h-dvh w-full">
	<div class="mx-auto space-y-2 p-2">
		<Hero />
		<ComponentGrid {components} />
		<Footer />
	</div>
</main>
