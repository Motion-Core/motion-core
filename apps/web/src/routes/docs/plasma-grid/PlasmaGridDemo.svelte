<script lang="ts">
	import { PlasmaGrid } from "motion-core";
	import { onMount } from "svelte";

	const COLOR_PRESETS = {
		dark: {
			color: "#18181B",
			highlightColor: "#572400",
		},
		light: {
			color: "#FFFFFF",
			highlightColor: "#FF6900",
		},
	};

	let isDark = $state(false);

	onMount(() => {
		isDark = document.documentElement.classList.contains("dark");

		const observer = new MutationObserver(() => {
			isDark = document.documentElement.classList.contains("dark");
		});

		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class"],
		});

		return () => observer.disconnect();
	});
</script>

<PlasmaGrid
	color={isDark ? COLOR_PRESETS.dark.color : COLOR_PRESETS.light.color}
	highlightColor={isDark
		? COLOR_PRESETS.dark.highlightColor
		: COLOR_PRESETS.light.highlightColor}
	class="h-full min-h-96 w-full"
/>
