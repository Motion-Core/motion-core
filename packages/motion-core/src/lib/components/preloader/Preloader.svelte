<script lang="ts">
	import gsap from "gsap";
	import { onMount } from "svelte";
	import { cn } from "../../utils/cn";

	type Image = {
		src: string;
		alt?: string;
	};

	type ComponentProps = {
		images: Image[];
		class?: string;
		onComplete?: () => void;
		[prop: string]: unknown;
	};

	let {
		images,
		class: className = "",
		onComplete,
		...restProps
	}: ComponentProps = $props();

	let containerRef = $state<HTMLElement>();
	let revealImagesRef: HTMLElement[] = $state([]);
	let isScaleUpRef: HTMLElement[] = $state([]);
	let secondLoopImagesRef: HTMLImageElement[] = $state([]);

	onMount(() => {
		const middleIndex = Math.floor(images.length / 2);
		const radiusTarget = isScaleUpRef[images.length + middleIndex];
		const isScaleDownTargets = secondLoopImagesRef.filter(
			(_, i) => i !== middleIndex,
		);

		const tl = gsap.timeline({
			defaults: {
				ease: "expo.inOut",
			},
			onComplete: () => {
				if (onComplete) onComplete();
				if (containerRef) containerRef.style.display = "none";
			},
		});

		if (revealImagesRef.length) {
			tl.fromTo(
				revealImagesRef,
				{
					xPercent: 500,
				},
				{
					xPercent: -500,
					duration: 2.5,
					stagger: 0.05,
				},
			);
		}

		if (isScaleDownTargets.length) {
			tl.to(
				isScaleDownTargets,
				{
					scale: 0.5,
					duration: 2,
					stagger: {
						each: 0.05,
						from: "edges",
						ease: "none",
					},
					onComplete: () => {
						if (radiusTarget) {
							radiusTarget.style.borderRadius = "0";
						}
					},
				},
				"-=0.1",
			);
		}

		if (isScaleUpRef.length) {
			tl.fromTo(
				isScaleUpRef,
				{
					width: "10em",
					height: "10em",
				},
				{
					width: "100vw",
					height: "100dvh",
					duration: 2,
				},
				"< 0.5",
			);
		}
	});
</script>

<div
	bind:this={containerRef}
	class={cn(
		"fixed inset-0 z-999 flex items-center justify-center overflow-hidden",
		className,
	)}
	{...restProps}
>
	<div
		class="relative flex items-center justify-center"
		style="mask-image: linear-gradient(to right, transparent, black 5em, black calc(100% - 5em), transparent); -webkit-mask-image: linear-gradient(to right, transparent, black 5em, black calc(100% - 5em), transparent);"
	>
		<div class="relative overflow-hidden">
			<div class="absolute flex items-center justify-center rounded-[0.5em]">
				{#each images as image, i (image.src)}
					<div bind:this={revealImagesRef[i]} class="relative px-[1em]">
						<div
							bind:this={isScaleUpRef[i]}
							class="relative flex h-[10em] w-[10em] items-center justify-center rounded-[0.5em]"
						>
							<img
								loading="eager"
								src={image.src}
								alt={image.alt ?? ""}
								class="absolute h-full w-full rounded-[inherit] object-cover"
							/>
						</div>
					</div>
				{/each}
			</div>

			<div
				class="relative left-full flex items-center justify-center rounded-[0.5em]"
			>
				{#each images as image, i (image.src)}
					{@const isMiddle = i === Math.floor(images.length / 2)}
					<div
						bind:this={revealImagesRef[images.length + i]}
						class="relative px-[1em]"
					>
						<div
							bind:this={isScaleUpRef[images.length + i]}
							class:is--radius={isMiddle}
							style={isMiddle
								? "transition: border-radius 0.5s cubic-bezier(1, 0, 0, 1);"
								: ""}
							class="relative flex h-[10em] w-[10em] items-center justify-center rounded-[0.5em] {isMiddle
								? 'will-change-transform'
								: ''}"
						>
							<img
								bind:this={secondLoopImagesRef[i]}
								loading="eager"
								src={image.src}
								alt={image.alt ?? ""}
								class="absolute h-full w-full rounded-[inherit] object-cover {isMiddle
									? ''
									: 'will-change-transform'}"
							/>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>
