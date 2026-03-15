<script lang="ts">
	import type { PageData } from "./$types";
	import Features from "$lib/components/home/Features.svelte";
	import Footer from "$lib/components/home/Footer.svelte";
	import Home from "$lib/components/home/Home.svelte";
	import Menubar from "$lib/components/home/Menubar.svelte";

	let { data }: { data: PageData } = $props();
</script>

<a
	href="#main-content"
	class="sr-only fixed top-3 left-3 z-100 bg-foreground px-4 py-2 text-sm text-background-inset focus:not-sr-only"
>
	Skip to main content
</a>

<Menubar />
<main
	id="main-content"
	tabindex="-1"
	class="mx-auto flex min-h-dvh w-full max-w-6xl flex-col items-center justify-center border-x border-border bg-background"
>
	<Home githubStars={data.githubStars} />
	<Features />
	<div class="w-full border-t border-border bg-dashed">
		<div class="mx-auto w-full max-w-5xl border-x border-border bg-background">
			<Footer />
		</div>
	</div>
</main>
