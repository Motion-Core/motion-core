<script lang="ts">
	import { resolve } from "$app/paths";
	import { tick } from "svelte";
	import Close from "carbon-icons-svelte/lib/Close.svelte";
	import LogoGithub from "carbon-icons-svelte/lib/LogoGithub.svelte";
	import Menu from "carbon-icons-svelte/lib/Menu.svelte";
	import Button from "../ui/Button.svelte";
	import ThemeToggle from "../ui/ThemeToggle.svelte";
	import { brandingConfig } from "$lib/config/branding";
	import { siteConfig } from "$lib/config/site";

	const homeRoute = "/" as const;
	const focusableSelectors =
		'a[href],button:not([disabled]),[tabindex]:not([tabindex="-1"])';

	let mobileOpen = $state(false);
	let restoreFocusOnClose = true;
	let mobilePanel = $state<HTMLDivElement | null>(null);
	let previouslyFocused: HTMLElement | null = null;

	function toggleMobileMenu() {
		if (mobileOpen) {
			closeMobileMenu();
			return;
		}

		restoreFocusOnClose = true;
		mobileOpen = true;
	}

	function closeMobileMenu(options: { restoreFocus?: boolean } = {}) {
		restoreFocusOnClose = options.restoreFocus ?? true;
		mobileOpen = false;
	}

	function handleMenuLinkSelect() {
		closeMobileMenu({ restoreFocus: false });
	}

	function getFocusableElements(): HTMLElement[] {
		if (!mobilePanel) return [];
		return Array.from(
			mobilePanel.querySelectorAll<HTMLElement>(focusableSelectors),
		);
	}

	function handleMobilePanelKeydown(event: KeyboardEvent) {
		if (event.key === "Escape") {
			event.preventDefault();
			closeMobileMenu();
			return;
		}

		if (event.key !== "Tab") return;

		const focusable = getFocusableElements();
		if (focusable.length === 0) {
			event.preventDefault();
			return;
		}

		const first = focusable[0];
		const last = focusable[focusable.length - 1];
		const activeElement = document.activeElement;

		if (event.shiftKey && activeElement === first) {
			event.preventDefault();
			last.focus();
		} else if (!event.shiftKey && activeElement === last) {
			event.preventDefault();
			first.focus();
		}
	}

	$effect(() => {
		if (!mobileOpen) return;

		const mainContent = document.getElementById("main-content");
		previouslyFocused =
			document.activeElement instanceof HTMLElement
				? document.activeElement
				: null;

		document.body.style.overflow = "hidden";
		mainContent?.setAttribute("inert", "");

		void tick().then(() => {
			const focusable = getFocusableElements();
			if (focusable.length > 0) {
				focusable[0].focus();
				return;
			}

			mobilePanel?.focus();
		});

		return () => {
			document.body.style.overflow = "";
			mainContent?.removeAttribute("inert");
			if (
				restoreFocusOnClose &&
				previouslyFocused &&
				document.contains(previouslyFocused)
			) {
				previouslyFocused.focus();
			}
			restoreFocusOnClose = true;
		};
	});
</script>

<nav aria-label="Primary navigation" class="fixed top-0 z-60 w-full">
	<div class="mx-auto max-w-6xl border-b border-border bg-background sm:border">
		<div class="relative flex items-center justify-between gap-3 px-4 py-1.5">
			<a
				href={resolve(homeRoute)}
				class="inline-flex items-center gap-1 px-2 py-2 text-sm tracking-tight text-foreground transition-colors duration-150 ease-out hover:text-foreground"
			>
				<span
					class="inline-flex shrink-0 items-center text-accent [&>svg]:size-4 [&>svg]:fill-current"
					aria-hidden="true"
				>
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html brandingConfig.logoRaw}
				</span>
				<span class="font-medium tracking-tight text-foreground"
					>{brandingConfig.name}</span
				>
			</a>

			<div
				class="absolute top-1/2 left-1/2 hidden -translate-x-1/2 -translate-y-1/2 items-center gap-1 sm:flex"
			>
				<a
					href={resolve("/docs/introduction")}
					class="inline-flex items-center gap-2 px-2 py-2 text-sm font-normal tracking-tight text-foreground-muted transition-colors duration-150 ease-out hover:text-foreground"
				>
					Introduction
				</a>
				<a
					href={resolve("/docs/cli-guide/quick-start")}
					class="inline-flex items-center gap-2 px-2 py-2 text-sm font-normal tracking-tight text-foreground-muted transition-colors duration-150 ease-out hover:text-foreground"
				>
					CLI Quick Start
				</a>
				<a
					href={resolve("/docs/changelog/registry")}
					class="inline-flex items-center gap-2 px-2 py-2 text-sm font-normal tracking-tight text-foreground-muted transition-colors duration-150 ease-out hover:text-foreground"
				>
					Changelog
				</a>
			</div>

			<div class="hidden items-center gap-2 sm:flex">
				<ThemeToggle />
				<Button
					href={siteConfig.links.github}
					target="_blank"
					rel="noreferrer"
					variant="secondary"
					size="md"
				>
					<LogoGithub size={16} />
					<span>GitHub</span>
				</Button>
			</div>

			<button
				type="button"
				class="-mr-2 inline-flex size-10 items-center justify-center gap-2 text-sm whitespace-nowrap text-foreground transition-colors duration-150 ease-out hover:bg-background-inset sm:hidden"
				aria-label={mobileOpen
					? "Close navigation menu"
					: "Open navigation menu"}
				aria-expanded={mobileOpen}
				aria-controls="mobile-menubar-panel"
				aria-haspopup="dialog"
				onclick={toggleMobileMenu}
			>
				{#if mobileOpen}
					<Close size={20} />
				{:else}
					<Menu size={20} />
				{/if}
			</button>
		</div>
	</div>
</nav>

<div
	class="mobile-overlay fixed inset-0 z-40 bg-background-inset/80 backdrop-blur-sm sm:hidden"
	class:active={mobileOpen}
	aria-label="Close mobile navigation overlay"
	onclick={() => closeMobileMenu()}
	onkeydown={(event) => {
		if (event.key === "Escape") {
			event.preventDefault();
			closeMobileMenu();
		}
	}}
	role="button"
	tabindex="-1"
	aria-hidden={!mobileOpen}
></div>

<div
	id="mobile-menubar-panel"
	role="dialog"
	aria-modal="true"
	aria-label="Mobile navigation"
	tabindex="-1"
	class="mobile-panel fixed top-16 left-1/2 z-50 grid w-[min(92vw,30rem)] gap-2 rounded-lg border border-border bg-background p-3 sm:hidden"
	class:active={mobileOpen}
	onkeydown={handleMobilePanelKeydown}
	bind:this={mobilePanel}
	aria-hidden={!mobileOpen}
>
	<Button
		href={resolve("/docs/introduction")}
		onclick={handleMenuLinkSelect}
		variant="ghost"
		size="none"
		class="justify-start px-3 py-2 font-normal"
	>
		<span>Introduction</span>
	</Button>
	<Button
		href={resolve("/docs/cli-guide/quick-start")}
		onclick={handleMenuLinkSelect}
		variant="ghost"
		size="none"
		class="justify-start px-3 py-2 font-normal"
	>
		<span>CLI Quick Start</span>
	</Button>
	<Button
		href={resolve("/docs/changelog/registry")}
		onclick={handleMenuLinkSelect}
		variant="ghost"
		size="none"
		class="justify-start px-3 py-2 font-normal"
	>
		<span>Registry Changelog</span>
	</Button>

	<div class="mt-1 grid grid-cols-1 gap-2">
		<Button
			href={siteConfig.links.github}
			target="_blank"
			rel="noreferrer"
			onclick={handleMenuLinkSelect}
			variant="secondary"
			class="col-span-2 justify-center"
		>
			<LogoGithub size={16} />
			<span>GitHub</span>
		</Button>
		<ThemeToggle class="col-span-2 ml-auto size-8 sm:hidden" />
	</div>
</div>

<style>
	.mobile-overlay {
		opacity: 0;
		pointer-events: none;
		transition: opacity 200ms ease-out;
		will-change: opacity;
	}

	.mobile-overlay.active {
		opacity: 1;
		pointer-events: auto;
	}

	.mobile-panel {
		opacity: 0;
		pointer-events: none;
		transition:
			opacity 200ms ease-out,
			transform 200ms ease-out;
		transform: translate(-50%, -12px);
		will-change: opacity, transform;
	}

	.mobile-panel.active {
		opacity: 1;
		pointer-events: auto;
		transform: translate(-50%, 0);
	}

	@media (prefers-reduced-motion: reduce) {
		.mobile-overlay,
		.mobile-panel {
			transition: none;
		}

		.mobile-panel {
			transform: translate(-50%, 0);
		}
	}
</style>
