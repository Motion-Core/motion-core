<script lang="ts">
	import { onDestroy } from "svelte";
	import { cn } from "$lib/utils/cn";

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
		"group transition-scale relative flex size-7 cursor-pointer items-center justify-center rounded-sm border border-border bg-card text-foreground shadow-sm duration-150 ease-out active:scale-[0.95]",
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
			"transition-transform duration-150 ease-out",
			copied && "scale-0",
		)}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			aria-hidden="true"
			width="16"
			height="16"
			fill="currentColor"
			viewBox="0 0 256 256"
			><path
				d="M200,32H163.74a47.92,47.92,0,0,0-71.48,0H56A16,16,0,0,0,40,48V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V48A16,16,0,0,0,200,32Zm-72,0a32,32,0,0,1,32,32H96A32,32,0,0,1,128,32Zm72,184H56V48H82.75A47.93,47.93,0,0,0,80,64v8a8,8,0,0,0,8,8h80a8,8,0,0,0,8-8V64a47.93,47.93,0,0,0-2.75-16H200Z"
			></path></svg
		>
	</span>
	<span
		class={cn(
			"absolute transition-transform duration-150 ease-out",
			!copied && "scale-0",
		)}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			aria-hidden="true"
			width="16"
			height="16"
			fill="currentColor"
			viewBox="0 0 256 256"
			><path
				d="M229.66,77.66l-128,128a8,8,0,0,1-11.32,0l-56-56a8,8,0,0,1,11.32-11.32L96,188.69,218.34,66.34a8,8,0,0,1,11.32,11.32Z"
			></path></svg
		>
	</span>
</button>
