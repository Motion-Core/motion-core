<script lang="ts">
	import Scene from "./NeuralNoiseScene.svelte";
	import { cn } from "../../utils/cn";
	import type { ComponentProps } from "svelte";

	type SceneProps = ComponentProps<typeof Scene>;

	interface Props {
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
		/**
		 * Speed of the noise animation.
		 * @default 1.0
		 */
		speed?: SceneProps["speed"];
		[key: string]: unknown;
	}

	let { class: className = "", speed = 1.0, ...rest }: Props = $props();
</script>

<div class={cn("relative h-full w-full overflow-hidden", className)} {...rest}>
	<div class="absolute inset-0 z-0">
		<Scene {speed} />
	</div>
</div>
