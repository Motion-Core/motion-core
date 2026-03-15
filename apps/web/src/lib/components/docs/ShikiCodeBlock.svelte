<script lang="ts">
	import Pre from "./markdown/Pre.svelte";

	type Props = {
		code: string;
		htmlLight: string;
		htmlDark?: string;
		lang?: string;
		class?: string;
		unstyled?: boolean;
	};

	let {
		code,
		htmlLight,
		htmlDark,
		lang,
		class: className,
		unstyled = false,
	}: Props = $props();
</script>

<Pre {code} class={className} data-language={lang} {unstyled}>
	<div class="shiki-theme-light">
		{@html htmlLight}
	</div>
	<div class="shiki-theme-dark">
		{@html htmlDark ?? htmlLight}
	</div>
</Pre>

<style>
	.shiki-theme-dark {
		display: none;
	}

	:global(.dark) :global(.shiki-theme-light) {
		display: none;
	}

	:global(.dark) :global(.shiki-theme-dark) {
		display: block;
	}
</style>
