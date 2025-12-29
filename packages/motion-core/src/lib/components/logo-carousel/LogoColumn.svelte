<script lang="ts">
	import { onMount, type Component } from "svelte";
	import { gsap } from "gsap/dist/gsap";
	import { cn } from "../../utils/cn";

	interface Logo {
		name: string;
		id: number;
		component: Component;
	}

	interface Props {
		/**
		 * Array of logos for this specific column.
		 */
		logos: Logo[];
		/**
		 * Index of the column (used for offset calculation).
		 */
		index: number;
		/**
		 * Interval in milliseconds between logo cycles.
		 * @default 2000
		 */
		cycleInterval?: number;
		/**
		 * Additional CSS classes for the column.
		 */
		class?: string;
	}

	let {
		logos,
		index,
		cycleInterval = 2000,
		class: className,
	}: Props = $props();

	let currentIndex = $state(0);
	let isFirst = $state(true);

	function gsapTransition(
		node: HTMLElement,
		params: { direction: "in" | "out" },
	) {
		gsap.killTweensOf(node);

		if (params.direction === "in") {
			if (isFirst) {
				gsap.set(node, {
					yPercent: 0,
					opacity: 1,
					filter: "blur(0px)",
				});
				return {
					duration: 0,
					tick: () => {},
				};
			}

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
		let timeout: ReturnType<typeof setTimeout>;

		const startTime = Date.now();
		let targetTime = startTime + cycleInterval + index * 200;

		const tick = () => {
			isFirst = false;
			currentIndex = (currentIndex + 1) % logos.length;

			targetTime += cycleInterval;

			const now = Date.now();
			if (targetTime <= now) {
				const drift = now - targetTime;
				const cyclesMissed = Math.floor(drift / cycleInterval) + 1;
				targetTime += cyclesMissed * cycleInterval;
			}

			timeout = setTimeout(tick, targetTime - now);
		};

		timeout = setTimeout(tick, targetTime - startTime);

		return () => {
			clearTimeout(timeout);
		};
	});

	let CurrentLogoComponent = $derived(logos[currentIndex].component);
</script>

<div
	class={cn("relative h-14 w-24 overflow-hidden md:h-24 md:w-48", className)}
>
	{#key currentIndex}
		<div
			class="absolute inset-0 flex items-center justify-center"
			style="opacity: 1;"
			in:gsapTransition={{ direction: "in" }}
			out:gsapTransition={{ direction: "out" }}
		>
			<CurrentLogoComponent
				class="h-auto max-h-[70%] w-auto max-w-[70%] object-contain"
			/>
		</div>
	{/key}
</div>
