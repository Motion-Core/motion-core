<script lang="ts">
	import Section from "./Section.svelte";
	import InsetShadowContainer from "./InsetShadowContainer.svelte";
	import ArrowUpRight from "carbon-icons-svelte/lib/ArrowUpRight.svelte";
	import { docsManifest } from "$lib/docs/generated-manifest";
	import { cn } from "$lib/utils/cn";

	type ComponentInfo = {
		slug: string;
		name: string;
		video?: string;
		poster?: string;
	};

	const FEATURED_LAYOUT = [
		{ slug: "dithered-image", className: "md:col-span-1 md:row-span-1" },
		{ slug: "text-loop", className: "md:col-span-2 md:row-span-1" },
		{ slug: "water-ripple", className: "md:col-span-1 md:row-span-2" },
		{ slug: "image-trail", className: "md:col-span-2 md:row-span-1" },
		{ slug: "globe", className: "md:col-span-1 md:row-span-1" },
		{ slug: "magnetic", className: "md:col-span-2 md:row-span-1" },
		{ slug: "rubiks-cube", className: "md:col-span-2 md:row-span-1" },
	] as const;

	const components = $derived(docsManifest as ComponentInfo[]);

	const featuredComponents = $derived(
		FEATURED_LAYOUT.flatMap((conf) => {
			const component = components.find((entry) => entry.slug === conf.slug);
			return component ? [{ component, className: conf.className }] : [];
		}),
	);
</script>

<Section class="flex flex-col items-center justify-center gap-4 p-0 md:p-4">
	<InsetShadowContainer>
		<div class="grid grid-cols-1 gap-2 md:auto-rows-[280px] md:grid-cols-4">
			{#each featuredComponents as item (item.component.slug)}
				<article
					data-component-card
					class={cn(
						"group relative min-h-64 overflow-hidden rounded-md border border-border bg-background",
						item.className,
					)}
				>
					{#if item.component.video}
						<video
							class="absolute inset-0 h-full w-full object-cover"
							src={item.component.video}
							poster={item.component.poster}
							autoplay
							loop
							muted
							playsinline
							preload="metadata"
						></video>
					{/if}

					<div class="absolute right-3 bottom-3 left-3 z-10">
						<div class="flex items-end justify-between gap-3">
							<h3
								class="max-w-[60%] text-base font-medium tracking-tight text-fixed-light sm:text-lg"
							>
								{item.component.name}
							</h3>
							<a
								href={`/docs/${item.component.slug}`}
								aria-label={`Open ${item.component.name}`}
								class="inset-shadow inline-flex size-9 items-center justify-center rounded-full border border-border bg-background-inset text-foreground"
							>
								<ArrowUpRight size={16} />
							</a>
						</div>
					</div>
				</article>
			{/each}
		</div>
	</InsetShadowContainer>
</Section>
