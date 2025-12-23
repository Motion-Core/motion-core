<script lang="ts">
	import { TextLoop } from "motion-core";
</script>

<div class="text-5xl flex gap-2">
	<span>Create</span>
	<TextLoop texts={["magic", "wonder", "impact", "value"]} interval={2000} />
	<span>today</span>
</div>
