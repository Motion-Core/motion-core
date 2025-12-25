<script lang="ts">
	import gsap from "gsap";
	import { motionCoreEase } from "../../helpers/gsap";
	import { cn } from "../../utils/cn";
	type Image = {
		src: string;
		alt?: string;
	};

	type ComponentProps = {
		images: Image[];
		class?: string;
		[prop: string]: unknown;
	};
	const {
		images,
		class: className = "",
		...restProps
	}: ComponentProps = $props();
	let containerRef: HTMLElement;
	let slidesRef: HTMLElement[] = $state([]);
	let innersRef: HTMLElement[] = $state([]);
	let currentIndex = $state(0);
	let isAnimating = false;
	const animationDuration = 1.5;
	function navigate(targetIndex: number) {
		if (isAnimating || targetIndex === currentIndex) return;
		isAnimating = true;
		const direction = targetIndex > currentIndex ? 1 : -1;
		const previousIndex = currentIndex;
		currentIndex = targetIndex;
		const currentSlide = slidesRef[previousIndex];
		const currentInner = innersRef[previousIndex];
		const upcomingSlide = slidesRef[currentIndex];
		const upcomingInner = innersRef[currentIndex];
		gsap.set(upcomingSlide, { zIndex: 20 });
		gsap.set(currentSlide, { zIndex: 10 });
		const tl = gsap.timeline({
			defaults: { duration: animationDuration, ease: motionCoreEase },
			onComplete() {
				isAnimating = false;
				gsap.set(currentSlide, { zIndex: 0, xPercent: 0 });
				gsap.set(currentInner, { xPercent: 0 });
				gsap.set(upcomingSlide, { zIndex: 10 });
			},
		});
		tl.to(currentSlide, { xPercent: -direction * 100 }, 0)
			.to(currentInner, { xPercent: direction * 75 }, 0)
			.fromTo(upcomingSlide, { xPercent: direction * 100 }, { xPercent: 0 }, 0)
			.fromTo(upcomingInner, { xPercent: -direction * 75 }, { xPercent: 0 }, 0);
	}
	$effect(() => {
		if (slidesRef[currentIndex]) {
			gsap.set(slidesRef[currentIndex], { zIndex: 10 });
		}
	});
</script>

<div
	bind:this={containerRef}
	class={cn("relative h-full w-full overflow-hidden", className)}
	{...restProps}
>
	{#each images as image, i (image.src)}
		<div
			bind:this={slidesRef[i]}
			class="pointer-events-none absolute inset-0 z-0 overflow-hidden will-change-[transform,opacity]"
		>
			<img
				bind:this={innersRef[i]}
				src={image.src}
				alt={image.alt ?? ""}
				class="absolute h-full w-full object-cover will-change-transform"
				draggable="false"
			/>
		</div>
	{/each}
	<div
		class="group absolute bottom-4 left-1/2 z-50 flex -translate-x-1/2 gap-2"
	>
		{#each images as image, i (image.src)}
			<button
				onclick={() => navigate(i)}
				class="relative size-10 cursor-pointer overflow-hidden rounded-sm transition-all duration-700 ease-[cubic-bezier(0.625,0.05,0,1)]"
				aria-label="Go to slide {i + 1}"
			>
				<img
					src={image.src}
					alt={image.alt ?? ""}
					class="h-full w-full rounded-sm object-cover transition-transform duration-700 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover:scale-80 hover:scale-100"
				/>
			</button>
		{/each}
	</div>
</div>
