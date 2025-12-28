<script lang="ts">
	import { AsciiRenderer } from "motion-core";

	const demoImage = "/images/demos/sample-11.jpg";
</script>

<AsciiRenderer
	src={demoImage}
	density={25}
	strength={25.0}
	color="#00ff00"
	backgroundColor="#111113"
	class="h-full min-h-96 w-full"
/>
