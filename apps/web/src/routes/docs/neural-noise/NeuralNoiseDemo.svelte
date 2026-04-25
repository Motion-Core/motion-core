<script lang="ts">
	import { NeuralNoise } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof NeuralNoise>>;

	let { speed = 0.5 }: Props = $props();
</script>

<NeuralNoise {speed} class="h-full min-h-96 w-full" />
