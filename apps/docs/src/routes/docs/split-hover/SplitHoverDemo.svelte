<script lang="ts">
	import { SplitHover } from "motion-core";

	let hoverTarget = $state<HTMLElement | null>(null);
</script>

<a bind:this={hoverTarget} class="text-lg" href="/docs/split-hover">
	<SplitHover {hoverTarget}>Orbit the future</SplitHover>
</a>
