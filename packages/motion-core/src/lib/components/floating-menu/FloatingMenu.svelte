<script lang="ts">
	import { untrack } from "svelte";
	import { gsap } from "gsap/dist/gsap";
	import { SplitText } from "gsap/dist/SplitText";
	import { CustomEase } from "gsap/dist/CustomEase";
	import { onMount } from "svelte";

	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn";
	import { portal } from "../../utils/use-portal";

	type MenuVariant = "default" | "muted";

	interface MenuLink {
		label: string;
		href: string;
	}

	interface MenuButton {
		label: string;
		href: string;
	}

	interface MenuGroup {
		title: string;
		variant?: MenuVariant;
		links: MenuLink[];
	}

	interface Props {
		/**
		 * Groups of links to display in the menu.
		 */
		menuGroups: MenuGroup[];
		/**
		 * Snippet for the logo icon (and optional text).
		 */
		logo?: Snippet;
		/**
		 * Configuration for the primary button in the header.
		 */
		primaryButton?: MenuButton;
		/**
		 * Configuration for the secondary button in the header.
		 */
		secondaryButton?: MenuButton;
		/**
		 * Additional classes for the container.
		 */
		class?: string;
		/**
		 * The target element or selector to append the menu to.
		 * Useful for containment in demos or specific containers.
		 * @default "body"
		 */
		portalTarget?: HTMLElement | string;
	}

	let {
		menuGroups,
		logo,
		primaryButton,
		secondaryButton,
		class: className,
		portalTarget = "body",
	}: Props = $props();

	let isOpen = $state(false);
	let timeline: gsap.core.Timeline | null = null;

	let containerRef: HTMLElement;
	let menuWrapperRef: HTMLElement;
	let line1Ref: HTMLElement;
	let line2Ref: HTMLElement;
	let overlayRef: HTMLElement;

	function toggle() {
		if (!timeline) return;
		isOpen = !isOpen;
		if (isOpen) {
			timeline.play();
		} else {
			timeline.reverse();
		}
	}

	onMount(() => {
		gsap.registerPlugin(SplitText);
		gsap.registerPlugin(CustomEase);
		CustomEase.create("motion-core-ease", "0.625, 0.05, 0, 1");
	});

	$effect(() => {
		if (!menuGroups.length) return;

		let cancelled = false;
		let splits: SplitText[] = [];

		const init = async () => {
			await document.fonts.ready;
			if (cancelled) return;

			const width = window.innerWidth;
			const isMobile = width < 768;
			const isTablet = width >= 768 && width < 1024;

			let maxWidthOpen = "75%";
			let maxWidthInitial = "50%";

			if (isMobile) {
				maxWidthOpen = "100%";
				maxWidthInitial = "95%";
			} else if (isTablet) {
				maxWidthOpen = "85%";
				maxWidthInitial = "70%";
			}

			gsap.set(overlayRef, { autoAlpha: 0 });
			gsap.set(containerRef, { maxWidth: maxWidthInitial });
			gsap.set(menuWrapperRef, { height: 0, autoAlpha: 0 });

			const linkElements = gsap.utils.toArray(
				".menu-link-text",
				menuWrapperRef,
			) as HTMLElement[];

			splits = linkElements.map((el) =>
				SplitText.create(el, { type: "lines", mask: "lines" }),
			);
			const allLines = splits.flatMap((s) => s.lines);

			timeline = gsap.timeline({
				paused: true,
				defaults: { ease: "motion-core-ease", duration: 0.5 },
			});

			timeline
				.to(
					containerRef,
					{
						maxWidth: maxWidthOpen,
						...(isMobile
							? {
									top: 0,
									paddingTop: "0.5rem",
									borderTopLeftRadius: 0,
									borderTopRightRadius: 0,
								}
							: {}),
					},
					0,
				)
				.to(overlayRef, { autoAlpha: 1 }, 0)
				.to(menuWrapperRef, { height: "auto", autoAlpha: 1 }, 0.2)
				.to([line1Ref, line2Ref], { y: 0, duration: 0.4 }, 0.2)
				.to(line1Ref, { rotation: 45, duration: 0.4 }, 0.2)
				.to(line2Ref, { rotation: -45, duration: 0.4 }, 0.2);

			if (allLines.length) {
				timeline.from(
					allLines,
					{
						yPercent: 100,
						autoAlpha: 0,
						stagger: 0.02,
					},
					0.3,
				);
			}

			if (untrack(() => isOpen)) {
				timeline.progress(1);
			}
		};

		init();

		return () => {
			cancelled = true;
			if (timeline) {
				timeline.kill();
				timeline = null;
			}
			splits.forEach((s) => s.revert());
		};
	});
</script>

<div
	use:portal={portalTarget}
	bind:this={overlayRef}
	class="pointer-events-none fixed inset-0 z-40 bg-background/60 opacity-0 data-[open=true]:pointer-events-auto"
	data-open={isOpen}
	onclick={toggle}
	onkeydown={(e) => {
		if (e.key === "Escape" && isOpen) {
			e.preventDefault();
			toggle();
		}
	}}
	role="button"
	tabindex="-1"
	aria-label="Close menu"
></div>

<div
	use:portal={portalTarget}
	bind:this={containerRef}
	class={cn(
		"card-highlight fixed top-2 left-1/2 z-50 w-full max-w-[95vw] -translate-x-1/2 rounded-lg border border-border bg-card text-foreground shadow-2xl md:top-4 md:max-w-[70vw] lg:max-w-[50vw]",
		className,
	)}
>
	<div class="relative z-20 flex w-full items-center justify-between p-1">
		<button
			onclick={toggle}
			class="group relative flex h-10 cursor-pointer items-center justify-center rounded-md pr-2 transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:bg-accent/10"
			aria-label="Toggle menu"
		>
			<div class="relative flex h-10 w-10 items-center justify-center">
				<span
					bind:this={line1Ref}
					class="absolute h-px w-6 bg-foreground transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover:bg-accent"
					style="transform: translateY(4px)"
				></span>
				<span
					bind:this={line2Ref}
					class="absolute h-px w-6 bg-foreground transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover:bg-accent"
					style="transform: translateY(-4px)"
				></span>
			</div>
			<span
				class="ml-1 text-sm font-medium text-foreground transition-[color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover:text-accent"
			>
				Menu
			</span>
		</button>

		<div
			class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 transform-gpu"
			style="backface-visibility: hidden;"
		>
			{#if logo}
				<div class="flex items-center gap-3">
					{@render logo()}
				</div>
			{/if}
		</div>

		<div class="flex items-center gap-1">
			{#if secondaryButton}
				<a
					href={secondaryButton.href}
					class="hidden h-10 items-center justify-center rounded-md px-4 text-sm font-medium text-foreground transition-[background-color,color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:bg-card-muted/60 hover:text-foreground md:flex"
				>
					{secondaryButton.label}
				</a>
			{/if}
			{#if primaryButton}
				<a
					href={primaryButton.href}
					class="flex h-10 items-center justify-center rounded-md bg-accent/10 px-4 text-sm font-medium text-accent transition-[background-color] ease-[cubic-bezier(0.625,0.05,0,1)] hover:bg-accent/20"
				>
					{primaryButton.label}
				</a>
			{/if}
		</div>
	</div>

	<div
		bind:this={menuWrapperRef}
		class="h-0 w-full overflow-hidden border-t border-border opacity-0"
	>
		<div
			class="grid max-h-[65vh] grid-cols-1 gap-4 overflow-y-auto overscroll-contain p-4 md:max-h-none md:grid-cols-3 md:overflow-visible"
		>
			{#each menuGroups as group (group.title)}
				<div
					class={cn(
						"flex flex-col gap-4 rounded-lg p-4 transition-colors ease-[cubic-bezier(0.625,0.05,0,1)]",
						group.variant === "muted" ? "bg-card-muted/60" : "bg-transparent",
					)}
				>
					<h3
						class="text-xs font-medium tracking-wider text-foreground/45 uppercase mono"
					>
						{group.title}
					</h3>
					<div class="mt-4 flex flex-col gap-4">
						{#each group.links as link, i (link.href + link.label)}
							<a
								href={link.href}
								class="group/link relative block w-fit text-2xl font-normal text-foreground/70 transition-colors duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:text-foreground"
							>
								<span class="relative z-10 block leading-tight">
									<span class="menu-link-text block whitespace-nowrap">
										{link.label}
									</span>
								</span>
								<span
									class="absolute -bottom-1 left-0 h-px w-full origin-right scale-x-0 bg-foreground transition-transform duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover/link:origin-left group-hover/link:scale-x-100"
								></span>
							</a>
							{#if i < group.links.length - 1}
								<hr class="border-border" />
							{/if}
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
