<script lang="ts">
	import type { ComponentInfo } from "./types";
	import { onMount } from "svelte";
	import { cn } from "$lib/utils/cn";

	let {
		component,
		class: className,
		featured = false,
	}: {
		component: ComponentInfo;
		class?: string;
		featured?: boolean;
	} = $props();

	let cardElement: HTMLAnchorElement;
	let videoElement = $state<HTMLVideoElement>();
	let shouldLoad = $state(false);
	let isLoaded = $state(false);

	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						shouldLoad = true;
					}
				});
			},
			{
				rootMargin: "200px",
				threshold: 0.1,
			},
		);

		if (cardElement) {
			observer.observe(cardElement);
		}

		return () => {
			observer.disconnect();
		};
	});

	$effect(() => {
		if (!videoElement || !cardElement) return;

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						videoElement?.play().catch(() => {});
					} else {
						videoElement?.pause();
					}
				});
			},
			{
				threshold: 0.1,
			},
		);

		observer.observe(cardElement);

		const rect = cardElement.getBoundingClientRect();
		const isVisible = rect.top < window.innerHeight && rect.bottom > 0;
		if (isVisible) {
			videoElement?.play().catch(() => {});
		}

		return () => {
			observer.disconnect();
		};
	});
</script>

<a
	bind:this={cardElement}
	href={`/docs/${component.slug}`}
	class={cn(
		"group card-highlight relative block min-h-64 break-inside-avoid rounded-xl border border-border bg-card p-1 opacity-0 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted md:min-h-0",
		featured ? "flex h-full flex-col" : "",
		className,
	)}
	data-component-card
>
	<div
		class={cn(
			"bg-muted relative overflow-hidden rounded-lg border border-border/60",
			featured ? "h-full min-h-0 flex-1" : "aspect-video",
		)}
	>
		{#if component.poster}
			<img
				src={component.poster}
				alt={component.name}
				class="absolute inset-0 h-full w-full object-cover transition-opacity duration-300 {isLoaded
					? 'opacity-0'
					: 'opacity-100'}"
			/>
		{/if}
		{#if component.video && shouldLoad}
			<video
				bind:this={videoElement}
				src={component.video}
				poster={component.poster}
				loop
				muted
				playsinline
				preload="metadata"
				oncanplay={() => (isLoaded = true)}
				class="absolute inset-0 h-full w-full object-cover transition-opacity duration-300 {isLoaded
					? 'opacity-100'
					: 'opacity-0'}"
			>
			</video>
		{/if}
	</div>
	<div class="mt-2 flex w-full items-center justify-between p-2">
		<p
			class="pointer-events-none text-sm font-medium text-foreground font-display"
		>
			{component.name}
		</p>
		<p class="pointer-events-none text-xs font-medium text-foreground/45">
			{component.category}
		</p>
	</div>
</a>
