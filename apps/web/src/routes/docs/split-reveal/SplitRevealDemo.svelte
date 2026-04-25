<script lang="ts">
	import { SplitReveal } from "motion-core";
	import type { ComponentProps } from "svelte";

	type SplitRevealProps = ComponentProps<typeof SplitReveal>;
	type SplitMode = NonNullable<SplitRevealProps["mode"]>;

	type Props = Partial<
		Pick<SplitRevealProps, "mode" | "delay"> & {
			duration: number;
			stagger: number;
		}
	>;

	let {
		mode = "lines",
		duration = 0.8,
		stagger = 0.08,
		delay = 0,
	}: Props = $props();

	const config = $derived({
		[mode]: {
			duration,
			stagger,
		},
	});
</script>

{#key `${mode}-${duration}-${stagger}-${delay}`}
	<SplitReveal
		{mode}
		{config}
		{delay}
		class="absolute top-1/2 left-1/2 block w-sm -translate-x-1/2 -translate-y-1/2 p-8 text-center text-lg md:w-3xl"
	>
		We’re using GSAP’s SplitText to break this content into lines, words, and
		individual characters. Experiment with staggered tweens, custom ease
		functions, and dynamic transforms to bring your headlines to life.
	</SplitReveal>
{/key}
