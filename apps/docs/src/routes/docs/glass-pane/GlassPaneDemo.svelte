<script lang="ts">
	import { GlassPane } from "motion-core";

	const demoImage =
		"https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_4.jpg";
</script>

<GlassPane
	image={demoImage}
	distortion={0.5}
	chromaticAberration={0.005}
	waviness={0.1}
	frequency={4.0}
	speed={0.5}
	class="h-full min-h-80 w-full"
></GlassPane>
