<script lang="ts">
	import type { ComponentInfo } from "./types";
	import { onMount } from "svelte";
	import { cn } from "$lib/utils/cn";

	let {
		component,
		class: className,
	}: {
		component: ComponentInfo;
		class?: string;
	} = $props();

	let cardElement: HTMLAnchorElement;
	let videoElement = $state<HTMLVideoElement>();
	let shouldLoad = $state(false);
	let isLoaded = $state(false);

	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (entry.isIntersecting) {
						shouldLoad = true;
					}
				}
			},
			{
				rootMargin: "180px",
				threshold: 0.1,
			},
		);

		if (cardElement) {
			observer.observe(cardElement);
		}

		return () => observer.disconnect();
	});

	$effect(() => {
		if (!videoElement || !cardElement) return;

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (entry.isIntersecting) {
						videoElement?.play().catch(() => {});
					} else {
						videoElement?.pause();
					}
				}
			},
			{
				threshold: 0.2,
			},
		);

		observer.observe(cardElement);
		return () => observer.disconnect();
	});
</script>

<a
	bind:this={cardElement}
	href={`/docs/${component.slug}`}
	class={cn(
		"group/card card-highlight relative block h-full min-h-64 rounded-2xl border border-border bg-card opacity-0 shadow-sm transition-[background-color] duration-150 ease-out hover:bg-card-muted",
		className,
	)}
	data-component-card
>
	<div class="relative h-full overflow-hidden rounded-2xl">
		{#if component.poster}
			<img
				src={component.poster}
				alt={component.name}
				class={cn(
					"absolute inset-0 h-full w-full object-cover transition-all duration-500",
					isLoaded ? "scale-103 opacity-0" : "scale-100 opacity-100",
				)}
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
				class={cn(
					"absolute inset-0 h-full w-full object-cover transition-all duration-500",
					isLoaded ? "scale-100 opacity-100" : "scale-103 opacity-0",
				)}
			>
			</video>
		{/if}

		<div class="absolute top-3 left-3">
			<span
				class="input-highlight relative inline-flex items-center justify-center rounded-full bg-background px-2 py-0.5 text-[10px] text-foreground uppercase"
			>
				{component.category}
			</span>
		</div>

		<div class="absolute right-0 bottom-0 left-0 p-4">
			<div class="flex items-end justify-between gap-3">
				<div>
					<p
						class="text-base leading-tight font-medium text-fixed-light font-display"
					>
						{component.name}
					</p>
				</div>
				<span
					class="input-highlight relative inline-flex size-8 items-center justify-center rounded-full bg-background text-foreground"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						aria-hidden="true"
						width="16"
						height="16"
						fill="currentColor"
						viewBox="0 0 256 256"
					>
						<path
							d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
						></path>
					</svg>
				</span>
			</div>
		</div>
	</div>
</a>
