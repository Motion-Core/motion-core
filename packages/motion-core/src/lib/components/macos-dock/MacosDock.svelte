<script lang="ts">
	import { cn } from "../../utils/cn";
	import gsap from "gsap";

	export interface DockItem {
		src: string;
		alt: string;
		label?: string;
		href?: string;
	}

	interface Props {
		items: DockItem[];
		class?: string;
		baseWidth?: number;
		magnification?: number;
		distance?: number;
	}

	let {
		items,
		class: className,
		baseWidth = 4,
		magnification = 1.5,
		distance: influenceDistance = 3,
	}: Props = $props();

	let hoveredIndex: number | null = $state(null);
	let dockItems: (HTMLLIElement | null)[] = $state([]);
	let dockTooltips: (HTMLDivElement | null)[] = $state([]);

	let maxWidth = $derived(baseWidth * magnification);

	$effect(() => {
		dockItems.forEach((el, index) => {
			if (!el) return;

			let targetWidth = baseWidth;

			if (hoveredIndex !== null) {
				const dist = Math.abs(hoveredIndex - index);

				if (dist < influenceDistance) {
					const ratio = (influenceDistance - dist) / influenceDistance;
					targetWidth = baseWidth + (maxWidth - baseWidth) * ratio;
				}
			}

			gsap.to(el, {
				width: `${targetWidth}em`,
				duration: 0.5,
				ease: "power4.out",
				overwrite: true,
			});
		});

		dockTooltips.forEach((el, index) => {
			if (!el) return;

			if (hoveredIndex === index) {
				gsap.to(el, {
					opacity: 1,
					yPercent: -100,
					xPercent: -50,
					duration: 0.5,
					ease: "power4.out",
					overwrite: true,
				});
			} else {
				gsap.to(el, {
					opacity: 0,
					yPercent: -80,
					xPercent: -50,
					duration: 0.5,
					ease: "power4.out",
					overwrite: true,
				});
			}
		});
	});
</script>

<nav class={cn("flex items-end justify-center p-4", className)}>
	<ul
		class="m-0 flex list-none items-end justify-center gap-0 p-0"
		onmouseleave={() => (hoveredIndex = null)}
	>
		{#each items as item, index (index)}
			<li
				bind:this={dockItems[index]}
				class="relative flex items-center justify-center"
				style="width: {baseWidth}em;"
				onmouseenter={() => (hoveredIndex = index)}
			>
				<a
					href={item.href || "#"}
					class="z-10 flex h-full w-full cursor-pointer items-center justify-center p-2"
				>
					<img
						src={item.src}
						alt={item.alt}
						class="pointer-events-none h-full w-full object-contain"
						loading="eager"
					/>
				</a>

				{#if item.label}
					<div
						bind:this={dockTooltips[index]}
						class="pointer-events-none absolute top-0 left-1/2 z-0 rounded border border-neutral-200/50 bg-neutral-100 px-2 py-1 text-sm whitespace-nowrap text-black opacity-0 shadow-md"
					>
						{item.label}
					</div>
				{/if}
			</li>
		{/each}
	</ul>
</nav>
