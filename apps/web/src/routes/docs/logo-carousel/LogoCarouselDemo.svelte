<script lang="ts">
	import { LogoCarousel } from "motion-core";
	import Apple from "./logos/Apple.svelte";
	import Supabase from "./logos/Supabase.svelte";
	import Ally from "./logos/Ally.svelte";
	import Claude from "./logos/Claude.svelte";
	import Nextjs from "./logos/Nextjs.svelte";
	import Tailwind from "./logos/Tailwind.svelte";
	import Typescript from "./logos/Typescript.svelte";
	import Stripe from "./logos/Stripe.svelte";
	import OpenAI from "./logos/OpenAI.svelte";

	const allLogos = [
		{ name: "Apple", id: 1, component: Apple },
		{ name: "Supabase", id: 2, component: Supabase },
		{ name: "Ally", id: 3, component: Ally },
		{ name: "Claude", id: 4, component: Claude },
		{ name: "Nextjs", id: 5, component: Nextjs },
		{ name: "Tailwind", id: 6, component: Tailwind },
		{ name: "Typescript", id: 7, component: Typescript },
		{ name: "Stripe", id: 8, component: Stripe },
		{ name: "OpenAI", id: 9, component: OpenAI },
	];
</script>

<LogoCarousel columnCount={3} logos={allLogos} />
