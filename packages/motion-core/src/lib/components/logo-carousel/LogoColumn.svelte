<script lang="ts">
	import { onMount, type Component } from "svelte";
	import gsap from "gsap";
	import { cn } from "../../utils/cn";

	interface Logo {
		name: string;
		id: number;
		component: Component;
	}

	interface Props {
		logos: Logo[];
		index: number;
		cycleInterval?: number;
		class?: string;
	}

	let {
		logos,
		index,
		cycleInterval = 2000,
		class: className,
	}: Props = $props();

	const columnDelay = $derived(index * 200);
	let currentIndex = $state(0);
	let isPlaying = $state(false);
	function gsapTransition(
		node: HTMLElement,
		params: { direction: "in" | "out" },
	) {
		gsap.killTweensOf(node);

		if (params.direction === "in") {
			gsap.fromTo(
				node,
				{ yPercent: 10, opacity: 0, filter: "blur(8px)" },
				{
					yPercent: 0,
					opacity: 1,
					filter: "blur(0px)",
					duration: 0.5,
					delay: 0.35,
					ease: "back.out(1.2)",
				},
			);
			return {
				duration: 900,
				tick: () => {},
			};
		} else {
			gsap.to(node, {
				yPercent: -20,
				opacity: 0,
				filter: "blur(6px)",
				duration: 0.3,
				ease: "power2.in",
			});
			return {
				duration: 300,
				tick: () => {},
			};
		}
	}

	onMount(() => {
		const startTimeout = setTimeout(() => {
			isPlaying = true;
			const interval = setInterval(() => {
				currentIndex = (currentIndex + 1) % logos.length;
			}, cycleInterval);

			return () => clearInterval(interval);
		}, columnDelay);

		return () => clearTimeout(startTimeout);
	});

	let CurrentLogoComponent = $derived(logos[currentIndex].component);
</script>

<div
	class={cn("relative h-14 w-24 overflow-hidden md:h-24 md:w-48", className)}
>
	{#if isPlaying}
		{#key currentIndex}
			<div
				class="absolute inset-0 flex items-center justify-center"
				style="opacity: 0;"
				in:gsapTransition={{ direction: "in" }}
				out:gsapTransition={{ direction: "out" }}
			>
				<CurrentLogoComponent
					class="h-auto w-auto max-h-[70%] max-w-[70%] object-contain"
				/>
			</div>
		{/key}
	{/if}
</div>
