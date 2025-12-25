<script lang="ts">
	import { onMount } from "svelte";
	import { cn } from "$lib/utils/cn";
	import type { Snippet } from "svelte";

	type Props = {
		class?: string;
		id?: string;
		children?: Snippet;
	};

	let { class: className, id, children }: Props = $props();

	let viewport: HTMLDivElement;
	let isDragging = $state(false);
	let startY = 0;
	let startScrollTop = 0;

	let thumbHeight = $state(0);
	let thumbTop = $state(0);
	let isVisible = $state(false);
	let isScrolling = $state(false);
	let isHoveringTrack = $state(false);
	let scrollTimeout: ReturnType<typeof setTimeout>;

	function updateThumb() {
		if (!viewport) return;
		const { clientHeight, scrollHeight, scrollTop } = viewport;

		isVisible = scrollHeight > clientHeight + 1;

		if (!isVisible) {
			thumbHeight = 0;
			return;
		}

		const heightRatio = clientHeight / scrollHeight;
		thumbHeight = Math.max(20, clientHeight * heightRatio);

		const maxScroll = scrollHeight - clientHeight;
		const scrollRatio = maxScroll > 0 ? scrollTop / maxScroll : 0;
		const maxThumbTop = clientHeight - thumbHeight;
		thumbTop = scrollRatio * maxThumbTop;
	}

	function handleScroll() {
		requestAnimationFrame(updateThumb);
		isScrolling = true;
		clearTimeout(scrollTimeout);
		scrollTimeout = setTimeout(() => {
			isScrolling = false;
		}, 600);
	}

	function onDragStart(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		isDragging = true;
		startY = e.clientY;
		startScrollTop = viewport.scrollTop;

		document.addEventListener("mousemove", onDragMove);
		document.addEventListener("mouseup", onDragEnd);
		document.body.style.userSelect = "none";
	}

	function onDragMove(e: MouseEvent) {
		if (!isDragging) return;
		const deltaY = e.clientY - startY;

		const { clientHeight, scrollHeight } = viewport;

		const maxThumbTop = clientHeight - thumbHeight;
		const maxScroll = scrollHeight - clientHeight;

		if (maxThumbTop === 0) return;

		const thumbRatio = deltaY / maxThumbTop;
		const scrollAmount = thumbRatio * maxScroll;

		viewport.scrollTop = startScrollTop + scrollAmount;
	}

	function onDragEnd() {
		isDragging = false;
		document.removeEventListener("mousemove", onDragMove);
		document.removeEventListener("mouseup", onDragEnd);
		document.body.style.userSelect = "";
	}

	onMount(() => {
		updateThumb();

		const observer = new ResizeObserver(() => {
			updateThumb();
		});

		if (viewport) {
			observer.observe(viewport);
			Array.from(viewport.children).forEach((child) => {
				observer.observe(child);
			});
		}

		return () => observer.disconnect();
	});
</script>

<div class={cn("relative overflow-hidden flex flex-col", className)}>
	<div
		bind:this={viewport}
		{id}
		class="scrollbar-hide flex-1 w-full min-h-0 overflow-y-auto overflow-x-hidden"
		onscroll={handleScroll}
	>
		{@render children?.()}
	</div>

	{#if isVisible}
		<div
			class={cn(
				"absolute top-0 right-0 bottom-0 w-2.5 p-px transition-opacity duration-300",
				isScrolling || isDragging || isHoveringTrack
					? "opacity-100"
					: "opacity-0",
			)}
			onmouseenter={() => (isHoveringTrack = true)}
			onmouseleave={() => (isHoveringTrack = false)}
			role="presentation"
		>
			<div
				role="scrollbar"
				aria-controls="viewport"
				aria-orientation="vertical"
				aria-valuenow={thumbTop}
				tabindex="0"
				class={cn(
					"relative rounded-full bg-foreground/10 transition-colors duration-150 hover:bg-foreground/30 active:bg-foreground/50",
					isDragging && "bg-foreground/50",
				)}
				style:height="{thumbHeight}px"
				style:transform="translate3d(0, {thumbTop}px, 0)"
				onmousedown={onDragStart}
			></div>
		</div>
	{/if}
</div>

<style>
	.scrollbar-hide {
		scrollbar-width: none;
		-ms-overflow-style: none;
	}
	.scrollbar-hide::-webkit-scrollbar {
		display: none;
	}
</style>
