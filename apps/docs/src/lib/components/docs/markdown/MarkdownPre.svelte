<script lang="ts">
import type { Snippet } from "svelte";
import Pre from "./Pre.svelte";

type Props = {
	htmlLight: string;
	htmlDark?: string;
	code?: Snippet;
	lang?: string;
	raw?: string;
};

const props = $props();
const htmlLight = $derived((props as Props).htmlLight);
const htmlDark = $derived((props as Props).htmlDark ?? htmlLight);
const code = $derived((props as Props).code);
const lang = $derived((props as Props).lang);
const raw = $derived((props as Props).raw ?? "");
</script>

<Pre data-language={lang} code={raw}>
	{#if code}
		{@render code?.()}
	{:else}
		<div class="shiki-theme shiki-theme-light" aria-hidden="false">
			{@html htmlLight}
		</div>
		<div class="shiki-theme shiki-theme-dark" aria-hidden="true">
			{@html htmlDark}
		</div>
	{/if}
</Pre>
