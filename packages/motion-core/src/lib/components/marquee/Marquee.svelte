<script lang="ts">
	import { onMount } from "svelte";
	import { gsap } from "gsap";
	import { ScrollTrigger } from "gsap/dist/ScrollTrigger";
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn";

	interface Props {
		class?: string;
		gap?: number;
		children?: Snippet;
		repeat?: number;
		duration?: number;
		velocity?: number;
		reversed?: boolean;
		scrollElement?: string | HTMLElement;
	}

	let {
		class: className = "",
		gap = 32,
		children,
		repeat = 3,
		duration = 5,
		velocity = 0.5,
		reversed = false,
		scrollElement,
	}: Props = $props();

	let container = $state<HTMLElement>();

	onMount(() => {
		gsap.registerPlugin(ScrollTrigger);

		const parts = container?.querySelectorAll(".marquee-part");
		if (!parts?.length) return;

		let direction = reversed ? -1 : 1;

		const timeline = gsap.timeline({
			repeat: -1,
			onReverseComplete() {
				this.totalTime(this.rawTime() + this.duration() * 10);
			},
		});

		timeline.to(parts, {
			xPercent: -100,
			ease: "none",
			duration,
		});

		if (reversed) {
			timeline.progress(1);
			timeline.timeScale(-1);
		}

		const trigger = ScrollTrigger.create({
			scroller: scrollElement,
			onUpdate(self) {
				const currentScrollDir = self.direction;
				const targetDir = reversed ? -currentScrollDir : currentScrollDir;

				if (direction !== targetDir) {
					direction = targetDir;
					gsap.to(timeline, { timeScale: direction, overwrite: true });
				}

				const scrollVel = self.getVelocity();
				if (Math.abs(scrollVel) > 0) {
					const timeScale =
						direction * (1 + Math.abs(scrollVel * velocity) / 1000);
					gsap.to(timeline, { timeScale, overwrite: true, duration: 0.1 });
					gsap.to(timeline, {
						timeScale: direction,
						duration: 0.5,
						delay: 0.1,
						overwrite: "auto",
					});
				}
			},
		});

		return () => {
			timeline.kill();
			trigger.kill();
		};
	});
</script>

<div
	bind:this={container}
	class={cn("flex h-full w-full overflow-hidden", className)}
>
	{#each Array(repeat) as _, i (i)}
		<div
			class="marquee-part flex shrink-0"
			style:gap="{gap}px"
			style:padding-left="{gap / 2}px"
			style:padding-right="{gap / 2}px"
			aria-hidden={i > 0}
		>
			{@render children?.()}
		</div>
	{/each}
</div>
