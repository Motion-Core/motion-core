<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import { themeStore } from "$lib/stores/theme.svelte";
	import Sun from "carbon-icons-svelte/lib/Sun.svelte";
	import Moon from "carbon-icons-svelte/lib/Moon.svelte";

	type Props = {
		class?: string;
	};

	const props = $props();
	const className = $derived((props as Props).class ?? "");
</script>

<button
	type="button"
	class={cn(
		"group transition-scale inset-shadow relative inline-flex size-7 items-center justify-center rounded-sm bg-background-inset text-foreground duration-150 ease-out active:scale-[0.95]",
		className,
	)}
	onclick={themeStore.toggle}
	aria-label={themeStore.isDark
		? "Switch to light mode"
		: "Switch to dark mode"}
>
	<span class="sr-only"
		>{themeStore.isDark ? "Switch to light mode" : "Switch to dark mode"}</span
	>
	<span
		class={cn(
			"transition-transform duration-150 ease-out",
			themeStore.isDark && "scale-0 blur-[2px]",
		)}
	>
		<Sun size={16} />
	</span>
	<span
		class={cn(
			"absolute transition-transform duration-150 ease-out",
			!themeStore.isDark && "scale-0 blur-[2px]",
		)}
	>
		<Moon size={16} />
	</span>
</button>
