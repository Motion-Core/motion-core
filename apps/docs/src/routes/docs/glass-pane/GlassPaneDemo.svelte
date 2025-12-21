<script lang="ts">
	import { GlassPane } from "motion-core";

	const demoImage = "https://raw.githubusercontent.com/66HEX/free-photos/refs/heads/main/img_10.jpg";
</script>

<div class="h-80 w-full">
    <GlassPane
        image={demoImage}
        distortion={1.5}
        chromaticAberration={0.005}
        waviness={0.1}
        frequency={4.0}
        speed={0.5}
        class="h-full w-full"
    >
    </GlassPane>
</div>
