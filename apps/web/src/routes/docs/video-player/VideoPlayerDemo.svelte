<script lang="ts">
	import { VideoPlayer } from "motion-core";
	import type { ComponentProps } from "svelte";

	type Props = Partial<ComponentProps<typeof VideoPlayer>>;

	let {
		autoplay = false,
		muted = true,
		loop = true,
		hideControls = false,
	}: Props = $props();
</script>

<div class="flex items-center justify-center px-8 py-16">
	<VideoPlayer
		src="https://videos.pexels.com/video-files/35217201/14919167_1920_1080_30fps.mp4"
		{autoplay}
		{muted}
		{loop}
		{hideControls}
	/>
</div>
