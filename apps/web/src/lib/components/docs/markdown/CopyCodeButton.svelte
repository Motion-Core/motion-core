<script lang="ts">
	import { onDestroy } from "svelte";
	import { cn } from "$lib/utils/cn";
	import Copy from "carbon-icons-svelte/lib/Copy.svelte";
	import Checkmark from "carbon-icons-svelte/lib/Checkmark.svelte";

	type Props = {
		code: string;
		class?: string;
	};

	const props = $props();
	const className = $derived((props as Props).class ?? "");
	const code = $derived((props as Props).code ?? "");

	let copied = $state(false);
	let timeoutId: number | null = null;
	let lastCode: string | null = null;

	async function handleCopy(value: string) {
		if (!value || typeof navigator === "undefined" || !navigator.clipboard) {
			return;
		}

		try {
			await navigator.clipboard.writeText(value);
			copied = true;
			if (timeoutId) {
				window.clearTimeout(timeoutId);
			}
			timeoutId = window.setTimeout(() => {
				copied = false;
				timeoutId = null;
			}, 2000);
		} catch (error) {
			console.error("Failed to copy code snippet", error);
		}
	}

	onDestroy(() => {
		if (timeoutId) {
			window.clearTimeout(timeoutId);
			timeoutId = null;
		}
	});

	$effect(() => {
		if (lastCode === code) {
			return;
		}

		lastCode = code;
		copied = false;
		if (timeoutId) {
			window.clearTimeout(timeoutId);
			timeoutId = null;
		}
	});
</script>

<button
	type="button"
	class={cn(
		"group transition-scale inset-shadow relative flex size-7 items-center justify-center rounded-sm bg-background-inset text-foreground duration-150 ease-out active:scale-[0.95]",
		className,
	)}
	onclick={(event) => {
		event.stopPropagation();
		event.preventDefault();
		handleCopy(code);
	}}
	aria-label={copied ? "Copied code" : "Copy code"}
>
	<span class="sr-only">{copied ? "Copied code" : "Copy code"}</span>
	<span
		class={cn(
			"absolute transition-[opacity,filter,scale] duration-150 ease-out will-change-[opacity,filter,scale]",
			copied
				? "scale-[0.25] opacity-0 blur-xs"
				: "blur-0 scale-100 opacity-100",
		)}
	>
		<Copy size={16} />
	</span>
	<span
		class={cn(
			"absolute transition-[opacity,filter,scale] duration-150 ease-out will-change-[opacity,filter,scale]",
			copied
				? "blur-0 scale-100 opacity-100"
				: " scale-[0.25] opacity-0 blur-xs",
		)}
	>
		<Checkmark size={16} />
	</span>
</button>
