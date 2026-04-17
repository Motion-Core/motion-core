<script lang="ts">
	import { untrack } from "svelte";
	import { gsap } from "gsap";
	import { SplitText } from "gsap/SplitText";
	import { onMount } from "svelte";
	import type { ClassValue } from "clsx";

	import type { Snippet } from "svelte";
	import { ensureMotionCoreEase, registerPluginOnce } from "../../helpers/gsap";
	import { cn } from "../../utils/cn";
	import { portal } from "../../utils/use-portal";

	type MenuVariant = "default" | "muted";

	interface MenuLink {
		/**
		 * The text to display for the link.
		 */
		label: string;
		/**
		 * The URL the link points to.
		 */
		href: string;
	}

	interface MenuButton {
		/**
		 * The text to display on the button.
		 */
		label: string;
		/**
		 * The URL the button links to.
		 */
		href: string;
	}

	interface MenuGroup {
		/**
		 * The title of the menu group, displayed above the links.
		 */
		title: string;
		/**
		 * The visual style variant of the group.
		 * 'muted' adds a background color.
		 */
		variant?: MenuVariant;
		/**
		 * Array of links to display within this group.
		 */
		links: MenuLink[];
	}

	interface FloatingMenuClasses {
		root?: ClassValue;
		overlay?: ClassValue;
		header?: ClassValue;
		toggleButton?: ClassValue;
		toggleLine?: ClassValue;
		logo?: ClassValue;
		actions?: ClassValue;
		primaryButton?: ClassValue;
		secondaryButton?: ClassValue;
		menuWrapper?: ClassValue;
		grid?: ClassValue;
		group?: ClassValue;
		groupMuted?: ClassValue;
		groupTitle?: ClassValue;
		link?: ClassValue;
		linkText?: ClassValue;
		linkUnderline?: ClassValue;
		divider?: ClassValue;
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
		 * Additional classes for specific menu slots.
		 */
		classes?: FloatingMenuClasses;
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
		classes,
		portalTarget = "body",
	}: Props = $props();

	let isOpen = $state(false);
	let timeline: gsap.core.Timeline | null = null;

	let containerRef: HTMLElement;
	let menuWrapperRef: HTMLElement;
	let line1Ref: HTMLElement;
	let line2Ref: HTMLElement;
	let overlayRef: HTMLElement;

	const attachContainerRef = (node: HTMLElement) => {
		containerRef = node;
	};

	const attachMenuWrapperRef = (node: HTMLElement) => {
		menuWrapperRef = node;
	};

	const attachLine1Ref = (node: HTMLElement) => {
		line1Ref = node;
	};

	const attachLine2Ref = (node: HTMLElement) => {
		line2Ref = node;
	};

	const attachOverlayRef = (node: HTMLElement) => {
		overlayRef = node;
	};

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
		registerPluginOnce(SplitText);
		ensureMotionCoreEase();
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
				`[data-slot="link-text"]`,
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
	{@attach attachOverlayRef}
	data-slot="overlay"
	class={cn(
		"pointer-events-none fixed inset-0 z-40 bg-background-inset/80 opacity-0 data-[open=true]:pointer-events-auto",
		classes?.overlay,
	)}
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
	{@attach attachContainerRef}
	data-slot="root"
	class={cn(
		"fixed top-2 left-1/2 z-50 w-full max-w-[95vw] -translate-x-1/2 rounded-md border border-border bg-background text-foreground shadow-md md:top-4 md:max-w-[70vw] lg:max-w-[50vw]",
		className,
		classes?.root,
	)}
>
	<div
		data-slot="header"
		class={cn(
			"relative z-20 flex w-full items-center justify-between p-1",
			classes?.header,
		)}
	>
		<button
			onclick={toggle}
			data-slot="toggle-button"
			class={cn(
				"group relative flex h-10 items-center justify-center rounded-sm pr-2 transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:bg-accent/10",
				classes?.toggleButton,
			)}
			aria-label="Toggle menu"
		>
			<div class="relative flex h-10 w-10 items-center justify-center">
				<span
					{@attach attachLine1Ref}
					data-slot="toggle-line"
					class={cn(
						"absolute h-px w-6 bg-foreground transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover:bg-accent",
						classes?.toggleLine,
					)}
					style="transform: translateY(4px)"
				></span>
				<span
					{@attach attachLine2Ref}
					data-slot="toggle-line"
					class={cn(
						"absolute h-px w-6 bg-foreground transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover:bg-accent",
						classes?.toggleLine,
					)}
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
				<div
					data-slot="logo"
					class={cn("flex items-center gap-3", classes?.logo)}
				>
					{@render logo()}
				</div>
			{/if}
		</div>

		<div
			data-slot="actions"
			class={cn("flex items-center gap-1", classes?.actions)}
		>
			{#if secondaryButton}
				<a
					href={secondaryButton.href}
					data-slot="secondary-button"
					class={cn(
						"hidden h-10 items-center justify-center rounded-sm px-4 text-sm font-medium text-foreground transition-[background-color,color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:bg-background-muted hover:text-foreground md:flex",
						classes?.secondaryButton,
					)}
				>
					{secondaryButton.label}
				</a>
			{/if}
			{#if primaryButton}
				<a
					href={primaryButton.href}
					data-slot="primary-button"
					class={cn(
						"flex h-10 items-center justify-center rounded-sm bg-accent/10 px-4 text-sm font-medium text-accent transition-[background-color] duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:bg-accent/20",
						classes?.primaryButton,
					)}
				>
					{primaryButton.label}
				</a>
			{/if}
		</div>
	</div>

	<div
		{@attach attachMenuWrapperRef}
		data-slot="menu-wrapper"
		class={cn(
			"h-0 w-full overflow-hidden border-t border-border opacity-0",
			classes?.menuWrapper,
		)}
	>
		<div
			data-slot="grid"
			class={cn(
				"grid max-h-[65vh] grid-cols-1 gap-4 overflow-y-auto overscroll-contain p-4 md:max-h-none md:grid-cols-3 md:overflow-visible",
				classes?.grid,
			)}
		>
			{#each menuGroups as group (group.title)}
				<div
					data-slot="group"
					class={cn(
						"flex flex-col gap-4 rounded-sm p-4 transition-colors ease-[cubic-bezier(0.625,0.05,0,1)]",
						group.variant === "muted"
							? "bg-background-muted"
							: "bg-transparent",
						classes?.group,
						group.variant === "muted" && classes?.groupMuted,
					)}
				>
					<h3
						data-slot="group-title"
						class={cn(
							"mono text-xs font-medium tracking-wider text-foreground-muted/50 uppercase",
							classes?.groupTitle,
						)}
					>
						{group.title}
					</h3>
					<div class="mt-4 flex flex-col gap-4">
						{#each group.links as link, i (link.href + link.label)}
							<a
								href={link.href}
								data-slot="link"
								class={cn(
									"group/link relative block w-fit text-2xl font-normal text-foreground-muted transition-colors duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] hover:text-foreground",
									classes?.link,
								)}
							>
								<span class="relative z-10 block leading-tight">
									<span
										data-slot="link-text"
										class={cn(
											"menu-link-text block whitespace-nowrap",
											classes?.linkText,
										)}
									>
										{link.label}
									</span>
								</span>
								<span
									data-slot="link-underline"
									class={cn(
										"absolute -bottom-1 left-0 h-px w-full origin-right scale-x-0 bg-foreground transition-transform duration-400 ease-[cubic-bezier(0.625,0.05,0,1)] group-hover/link:origin-left group-hover/link:scale-x-100",
										classes?.linkUnderline,
									)}
								></span>
							</a>
							{#if i < group.links.length - 1}
								<hr
									data-slot="divider"
									class={cn("border-border", classes?.divider)}
								/>
							{/if}
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
