<script lang="ts">
	import gsap from "gsap";
	import { onMount } from "svelte";

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

	let containerRef: HTMLElement;
	let revealImagesRef: HTMLElement[] = [];
	let isScaleDownRef: HTMLElement[] = [];
	let isScaleUpRef: HTMLElement[] = [];
	let isRadiusRef: HTMLElement[] = [];

	onMount(() => {
		const tl = gsap.timeline({
			defaults: {
				ease: "expo.inOut",
			},
			onComplete: () => {
				if (onComplete) onComplete();
				containerRef.style.display = "none";
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

		if (isScaleDownRef.length) {
			tl.to(
				isScaleDownRef,
				{
					scale: 0.5,
					duration: 2,
					stagger: {
						each: 0.05,
						from: "edges",
						ease: "none",
					},
					onComplete: () => {
						if (isRadiusRef.length) {
							isRadiusRef.forEach((el) => (el.style.borderRadius = "0"));
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
	class="fixed inset-0 z-999 flex items-center justify-center overflow-hidden bg-white {className}"
	{...restProps}
>
	<div class="relative flex items-center justify-center">
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
							bind:this={isRadiusRef[isMiddle ? 0 : -1]}
							style={isMiddle
								? "transition: border-radius 0.5s cubic-bezier(1, 0, 0, 1);"
								: ""}
							class="relative flex h-[10em] w-[10em] items-center justify-center rounded-[0.5em] {isMiddle
								? 'will-change-transform'
								: ''}"
						>
							<img
								bind:this={
									isScaleDownRef[isMiddle ? -1 : isScaleDownRef.length]
								}
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

		<div
			class="pointer-events-none absolute -left-px -top-px h-[calc(100%+2px)] w-[5em]"
			style="background-image: linear-gradient(90deg, #ffffff 20%, #0000);"
		></div>
		<div
			class="pointer-events-none absolute -right-px -top-px h-[calc(100%+2px)] w-[5em] scale-x-[-1]"
			style="background-image: linear-gradient(90deg, #ffffff 20%, #0000);"
		></div>
	</div>
</div>
