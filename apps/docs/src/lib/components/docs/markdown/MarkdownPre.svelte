<script lang="ts">
	import type { Snippet } from "svelte";
	import Pre from "./Pre.svelte";
	import ShikiCodeBlock from "../ShikiCodeBlock.svelte";

	type Props = {
		htmlLight: string;
		htmlDark?: string;
		code?: Snippet;
		lang?: string;
		raw?: string;
	};

	const props = $props();
	const htmlLight = $derived((props as Props).htmlLight);
	const htmlDark = $derived((props as Props).htmlDark);
	const code = $derived((props as Props).code);
	const lang = $derived((props as Props).lang);
	const raw = $derived((props as Props).raw ?? "");
</script>

{#if code}
	<Pre data-language={lang} code={raw}>
		{@render code?.()}
	</Pre>
{:else}
	<ShikiCodeBlock code={raw} {htmlLight} {htmlDark} {lang} />
{/if}
