<script lang="ts">
	import { GlassPane } from "motion-core";

	const demoImage = "/images/demos/sample-6.jpg";
</script>

<GlassPane
	image={demoImage}
	distortion={0.5}
	chromaticAberration={0.005}
	waviness={0.0}
	speed={0.5}
	class="h-full min-h-96 w-full"
/>
