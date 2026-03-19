<script lang="ts">
	import { fly } from "svelte/transition";
	import { backOut } from "svelte/easing";
	import { docsUiConfig, resolveDocAssistantUrls } from "$lib/config/docs-ui";
	import { portal } from "$lib/utils/use-portal";
	import Checkmark from "carbon-icons-svelte/lib/Checkmark.svelte";
	import LogoGithub from "carbon-icons-svelte/lib/LogoGithub.svelte";
	import OverflowMenuHorizontal from "carbon-icons-svelte/lib/OverflowMenuHorizontal.svelte";

	type Props = {
		rawPath?: string | null;
		rawUrl?: string | null;
		githubUrl?: string | null;
	};

	let { rawPath, rawUrl, githubUrl }: Props = $props();

	let copyState = $state<"idle" | "copying" | "success" | "error">("idle");
	let resetTimer = $state<ReturnType<typeof setTimeout> | null>(null);
	let isDropdownOpen = $state(false);
	let dropdownRef = $state<HTMLDivElement | null>(null);
	let triggerRef = $state<HTMLButtonElement | null>(null);
	let dropdownStyle = $state("");
	let prefetchedContent = $state<string | null>(null);

	const assistantUrls = $derived(resolveDocAssistantUrls(rawUrl));
	const chatGptUrl = $derived(assistantUrls.chatGptUrl);
	const claudeUrl = $derived(assistantUrls.claudeUrl);
	const canShowCopy = $derived(
		docsUiConfig.docActions.showCopyMarkdown && Boolean(rawPath),
	);
	const canShowRepository = $derived(
		docsUiConfig.docActions.showRepositoryLink && Boolean(githubUrl),
	);
	const hasMenuActions = $derived(
		canShowRepository || Boolean(chatGptUrl) || Boolean(claudeUrl),
	);
	const hasActions = $derived(canShowCopy || hasMenuActions);

	const copyLabel = $derived(
		copyState === "copying"
			? docsUiConfig.docActions.copyLabels.copying
			: copyState === "success"
				? docsUiConfig.docActions.copyLabels.success
				: copyState === "error"
					? docsUiConfig.docActions.copyLabels.error
					: docsUiConfig.docActions.copyLabels.mobileIdle,
	);

	async function prefetchContent() {
		if (!canShowCopy || !rawPath) return;
		try {
			const response = await fetch(rawPath);
			if (response.ok) {
				prefetchedContent = await response.text();
			}
		} catch (e) {
			console.warn("Failed to prefetch document content:", e);
		}
	}

	$effect(() => {
		if (!canShowCopy) return;
		prefetchContent();
	});

	async function handleCopy() {
		if (!canShowCopy || copyState === "copying" || copyState === "success")
			return;

		copyState = "copying";

		try {
			let content = prefetchedContent;
			if (!content) {
				if (!rawPath) throw new Error("No path to fetch");
				const response = await fetch(rawPath);
				if (!response.ok) throw new Error("Failed to load document");
				content = await response.text();
			}

			let success = false;

			// 1. Try modern Clipboard API
			if (navigator?.clipboard?.writeText) {
				try {
					await navigator.clipboard.writeText(content);
					success = true;
				} catch (err) {
					console.warn("Clipboard API failed, trying fallback...", err);
				}
			}

			// 2. Fallback for Mobile Safari
			if (!success) {
				try {
					const textArea = document.createElement("textarea");
					textArea.value = content;

					// Ensure it's not visible but part of the DOM
					textArea.style.position = "fixed";
					textArea.style.left = "-9999px";
					textArea.style.top = "0";
					textArea.setAttribute("readonly", "");
					document.body.appendChild(textArea);

					textArea.focus();
					textArea.select();

					const supported = document.queryCommandSupported("copy");
					if (supported) {
						success = document.execCommand("copy");
					}
					document.body.removeChild(textArea);
				} catch (err) {
					console.warn("Fallback copy mechanism failed:", err);
				}
			}

			if (!success) {
				throw new Error("All copy methods failed");
			}

			copyState = "success";
		} catch (e) {
			console.error("Copy failed:", e);
			copyState = "error";
		} finally {
			if (resetTimer) {
				clearTimeout(resetTimer);
			}

			resetTimer = setTimeout(() => {
				copyState = "idle";
			}, 2000);
		}
	}

	function toggleDropdown() {
		if (!hasMenuActions) return;
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown() {
		isDropdownOpen = false;
	}

	function handleClickOutside(event: MouseEvent) {
		if (
			isDropdownOpen &&
			dropdownRef &&
			!dropdownRef.contains(event.target as Node) &&
			triggerRef &&
			!triggerRef.contains(event.target as Node)
		) {
			closeDropdown();
		}
	}

	function updatePosition() {
		if (!triggerRef) return;
		const rect = triggerRef.getBoundingClientRect();
		dropdownStyle = `top: ${rect.bottom + 8}px; right: ${window.innerWidth - rect.right}px; position: fixed;`;
	}

	$effect(() => {
		if (isDropdownOpen) {
			updatePosition();
			window.addEventListener("click", handleClickOutside);
			window.addEventListener("scroll", updatePosition, true);
			window.addEventListener("resize", updatePosition);
		} else {
			window.removeEventListener("click", handleClickOutside);
			window.removeEventListener("scroll", updatePosition, true);
			window.removeEventListener("resize", updatePosition);
		}
		return () => {
			window.removeEventListener("click", handleClickOutside);
			window.removeEventListener("scroll", updatePosition, true);
			window.removeEventListener("resize", updatePosition);
		};
	});

	$effect(() => {
		return () => {
			if (resetTimer) {
				clearTimeout(resetTimer);
			}
		};
	});

	const buttonClass =
		"card relative inline-flex h-9 w-full font-medium shrink-0 overflow-hidden items-center justify-center gap-2 rounded-sm bg-background px-4 py-2 text-sm whitespace-nowrap text-foreground transition-[background-color] duration-150 ease-out hover:bg-background-muted disabled:pointer-events-none disabled:opacity-50 has-[>svg]:px-3 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 flex-1";
</script>

{#if hasActions}
	<div class="relative z-20 mt-8 flex w-full gap-2 lg:hidden">
		{#if canShowCopy}
			<div class="inset-shadow w-full rounded-md bg-background-inset p-1.5">
				<button
					type="button"
					onclick={handleCopy}
					aria-live="polite"
					aria-disabled={copyState === "success"}
					class={buttonClass}
				>
					<span
						class="grid place-items-center"
						style="grid-template-areas: 'content';"
					>
						{#key copyState}
							<span
								class="flex items-center gap-2 font-medium tracking-normal text-foreground will-change-transform"
								style="grid-area: content;"
								in:fly={{ y: 20, duration: 300, easing: backOut }}
								out:fly={{ y: -20, duration: 200, easing: backOut }}
							>
								{#if copyState === "success"}
									<Checkmark class="size-4 flex-none" />
								{:else}
									<svg
										role="img"
										viewBox="0 0 24 24"
										fill="none"
										aria-hidden="true"
										class="size-4 flex-none"
									>
										<title>Markdown</title>
										<path
											class="stroke-current"
											d="M1.212 5.5h21.576c.407 0 .712.317.712.679v11.549a.695.695 0 0 1-.712.677H1.212a.695.695 0 0 1-.712-.678V6.18c0-.362.305-.679.712-.679Z"
										/>
										<path
											class="fill-current"
											d="M3.03 15.96V7.946h2.425l2.424 2.946 2.424-2.946h2.424v8.014h-2.424v-4.596L7.88 14.31l-2.424-2.946v4.596H3.03Zm15.152 0-3.636-3.89h2.424V7.947h2.424v4.125h2.424l-3.636 3.889Z"
										/>
									</svg>
								{/if}
								<span>{copyLabel}</span>
							</span>
						{/key}
					</span>
				</button>
			</div>
		{/if}

		{#if hasMenuActions}
			<div class="inset-shadow relative rounded-md bg-background-inset p-1.5">
				<button
					bind:this={triggerRef}
					type="button"
					onclick={toggleDropdown}
					class="{buttonClass} w-auto! px-2.5!"
					aria-label={docsUiConfig.docActions.moreActionsAriaLabel}
					aria-haspopup="true"
					aria-expanded={isDropdownOpen}
				>
					<OverflowMenuHorizontal class="size-4" />
				</button>

				{#if isDropdownOpen}
					<div
						use:portal
						bind:this={dropdownRef}
						style={dropdownStyle}
						class="card z-50 flex w-48 origin-top-right flex-col gap-0.5 rounded-md bg-background p-1"
						in:fly={{ y: -5, duration: 200, easing: backOut }}
						out:fly={{ y: -5, duration: 150, easing: backOut }}
					>
						{#if canShowRepository}
							<a
								href={githubUrl}
								target="_blank"
								rel="noreferrer"
								class="group flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm font-medium tracking-normal text-foreground-muted transition-colors hover:bg-background-muted hover:text-foreground"
							>
								<LogoGithub class="size-4 flex-none" />
								{docsUiConfig.docActions.repositoryLinkLabel}
							</a>
						{/if}

						{#if chatGptUrl}
							<a
								href={chatGptUrl}
								target="_blank"
								rel="noreferrer"
								class="group flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm font-medium tracking-normal text-foreground-muted transition-colors hover:bg-background-muted hover:text-foreground"
							>
								<svg
									role="img"
									viewBox="0 0 24 24"
									fill="currentColor"
									aria-hidden="true"
									class="size-4 flex-none"
								>
									<title>OpenAI</title>
									<path
										d="M22.2819 9.8211a5.9847 5.9847 0 0 0-.5157-4.9108 6.0462 6.0462 0 0 0-6.5098-2.9A6.0651 6.0651 0 0 0 4.9807 4.1818a5.9847 5.9847 0 0 0-3.9977 2.9 6.0462 6.0462 0 0 0 .7427 7.0966 5.98 5.98 0 0 0 .511 4.9107 6.051 6.051 0 0 0 6.5146 2.9001A5.9847 5.9847 0 0 0 13.2599 24a6.0557 6.0557 0 0 0 5.7718-4.2058 5.9894 5.9894 0 0 0 3.9977-2.9001 6.0557 6.0557 0 0 0-.7475-7.0729zm-9.022 12.6081a4.4755 4.4755 0 0 1-2.8764-1.0408l.1419-.0804 4.7783-2.7582a.7948.7948 0 0 0 .3927-.6813v-6.7369l2.02 1.1686a.071.071 0 0 1 .038.052v5.5826a4.504 4.504 0 0 1-4.4945 4.4944zm-9.6607-4.1254a4.4708 4.4708 0 0 1-.5346-3.0137l.142.0852 4.783 2.7582a.7712.7712 0 0 0 .7806 0l5.8428-3.3685v2.3324a.0804.0804 0 0 1-.0332.0615L9.74 19.9502a4.4992 4.4992 0 0 1-6.1408-1.6464zM2.3408 7.8956a4.485 4.485 0 0 1 2.3655-1.9728V11.6a.7664.7664 0 0 0 .3879.6765l5.8144 3.3543-2.0201 1.1685a.0757.0757 0 0 1-.071 0l-4.8303-2.7865A4.504 4.504 0 0 1 2.3408 7.872zm16.5963 3.8558L13.1038 8.364 15.1192 7.2a.0757.0757 0 0 1 .071 0l4.8303 2.7913a4.4944 4.4944 0 0 1-.6765 8.1042v-5.6772a.79.79 0 0 0-.407-.667zm2.0107-3.0231l-.142-.0852-4.7735-2.7818a.7759.7759 0 0 0-.7854 0L9.409 9.2297V6.8974a.0662.0662 0 0 1 .0284-.0615l4.8303-2.7866a4.4992 4.4992 0 0 1 6.6802 4.66zM8.3065 12.863l-2.02-1.1638a.0804.0804 0 0 1-.038-.0567V6.0742a4.4992 4.4992 0 0 1 7.3757-3.4537l-.142.0805L8.704 5.459a.7948.7948 0 0 0-.3927.6813zm1.0976-2.3654l2.602-1.4998 2.6069 1.4998v2.9994l-2.5974 1.4997-2.6067-1.4997Z"
									/>
								</svg>
								{docsUiConfig.docActions.assistants.chatgpt.label}
							</a>
						{/if}

						{#if claudeUrl}
							<a
								href={claudeUrl}
								target="_blank"
								rel="noreferrer"
								class="group flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm font-medium tracking-normal text-foreground-muted transition-colors hover:bg-background-muted hover:text-foreground"
							>
								<svg
									role="img"
									viewBox="0 0 24 24"
									fill="currentColor"
									aria-hidden="true"
									class="size-4 flex-none"
								>
									<title>Anthropic</title>
									<path
										d="M17.3041 3.541h-3.6718l6.696 16.918H24Zm-10.6082 0L0 20.459h3.7442l1.3693-3.5527h7.0052l1.3693 3.5528h3.7442L10.5363 3.5409Zm-.3712 10.2232 2.2914-5.9456 2.2914 5.9456Z"
									/>
								</svg>
								{docsUiConfig.docActions.assistants.claude.label}
							</a>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}
