<script lang="ts">
	import type { PageData } from "./$types";
	import Home from "$lib/components/home/Home.svelte";

	let { data }: { data: PageData } = $props();
</script>

<a
	href="#main-content"
	class="sr-only fixed top-3 left-3 z-100 bg-foreground px-4 py-2 text-sm text-background-inset focus:not-sr-only"
>
	Skip to main content
</a>
<main
	id="main-content"
	tabindex="-1"
	class="mx-auto flex min-h-dvh w-full flex-col items-center justify-center bg-background-inset"
>
	<Home githubStars={data.githubStars} />
</main>
