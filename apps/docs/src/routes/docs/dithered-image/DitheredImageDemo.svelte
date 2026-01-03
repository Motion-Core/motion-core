<script lang="ts">
	import { DitheredImage } from "motion-core";
</script>

<DitheredImage
	src="/images/demos/sample-6.jpg"
	ditherMap="bayer8x8"
	class="h-full min-h-96 w-full"
	pixelSize={2}
	threshold={0.05}
/>
