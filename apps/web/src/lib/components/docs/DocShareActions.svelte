<script lang="ts">
	import { fly } from "svelte/transition";
	import { backOut } from "svelte/easing";
	import { docsUiConfig, resolveDocAssistantUrls } from "$lib/config/docs-ui";
	import Checkmark from "carbon-icons-svelte/lib/Checkmark.svelte";
	import LogoGithub from "carbon-icons-svelte/lib/LogoGithub.svelte";
	import Launch from "carbon-icons-svelte/lib/Launch.svelte";

	type Props = {
		rawPath?: string | null;
		rawUrl?: string | null;
		githubUrl?: string | null;
	};

	let { rawPath, rawUrl, githubUrl }: Props = $props();

	let buttonRef = $state<HTMLButtonElement | null>(null);

	type CopyState = "idle" | "copying" | "success" | "error";
	let copyState = $state<CopyState>("idle");
	let resetTimer = $state<ReturnType<typeof setTimeout> | null>(null);

	const assistantUrls = $derived(resolveDocAssistantUrls(rawUrl));
	const chatGptUrl = $derived(assistantUrls.chatGptUrl);
	const claudeUrl = $derived(assistantUrls.claudeUrl);
	const canShowCopy = $derived(
		docsUiConfig.docActions.showCopyMarkdown && Boolean(rawPath),
	);
	const canShowRepository = $derived(
		docsUiConfig.docActions.showRepositoryLink && Boolean(githubUrl),
	);
	const hasActions = $derived(
		canShowCopy ||
			canShowRepository ||
			Boolean(chatGptUrl) ||
			Boolean(claudeUrl),
	);

	const copyLabel = $derived(
		copyState === "copying"
			? docsUiConfig.docActions.copyLabels.copying
			: copyState === "success"
				? docsUiConfig.docActions.copyLabels.success
				: copyState === "error"
					? docsUiConfig.docActions.copyLabels.error
					: docsUiConfig.docActions.copyLabels.desktopIdle,
	);

	async function handleCopy() {
		if (
			!canShowCopy ||
			!rawPath ||
			copyState === "copying" ||
			copyState === "success"
		)
			return;

		copyState = "copying";

		try {
			const response = await fetch(rawPath);
			if (!response.ok) {
				throw new Error("Failed to load document");
			}

			const content = await response.text();
			if (typeof navigator === "undefined" || !navigator.clipboard?.writeText) {
				throw new Error("Clipboard unavailable");
			}

			await navigator.clipboard.writeText(content);
			copyState = "success";
		} catch {
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

	$effect(() => {
		return () => {
			if (resetTimer) {
				clearTimeout(resetTimer);
			}
		};
	});
</script>

{#if hasActions}
	<div class="mt-auto">
		<div class="flex flex-col gap-1 text-sm">
			{#if canShowCopy}
				<button
					bind:this={buttonRef}
					type="button"
					onclick={handleCopy}
					aria-live="polite"
					aria-disabled={copyState === "success"}
					class="group/option flex items-center gap-2 overflow-hidden rounded-sm px-3 py-1.5 text-left font-medium tracking-normal text-foreground-muted transition-colors duration-150 ease-out hover:bg-background-muted hover:text-foreground"
				>
					<span class="grid" style="grid-template-areas: 'content';">
						{#key copyState}
							<span
								class="flex items-center gap-2"
								style="grid-area: content;"
								in:fly={{ y: 20, duration: 450, easing: backOut }}
								out:fly={{ y: -20, duration: 300, easing: backOut }}
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
			{/if}

			{#if canShowRepository}
				<a
					class="group/option flex items-center gap-2 rounded-sm px-3 py-1.5 text-left font-medium tracking-normal text-foreground-muted transition-colors duration-150 ease-out hover:bg-background-muted hover:text-foreground"
					href={githubUrl}
					target="_blank"
					rel="noreferrer"
				>
					<LogoGithub class="size-4 flex-none" />
					<span>{docsUiConfig.docActions.repositoryLinkLabel}</span>
					<Launch class="ml-auto size-4 flex-none" />
				</a>
			{/if}

			{#if chatGptUrl}
				<a
					class="group/option flex items-center gap-2 rounded-sm px-3 py-1.5 text-left font-medium tracking-normal text-foreground-muted transition-colors duration-150 ease-out hover:bg-background-muted hover:text-foreground"
					href={chatGptUrl}
					target="_blank"
					rel="noreferrer"
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
					<span>{docsUiConfig.docActions.assistants.chatgpt.label}</span>
					<Launch class="ml-auto size-4 flex-none" />
				</a>
			{/if}

			{#if claudeUrl}
				<a
					class="group/option flex items-center gap-2 rounded-sm px-3 py-1.5 text-left font-medium tracking-normal text-foreground-muted transition-colors duration-150 ease-out hover:bg-background-muted hover:text-foreground"
					href={claudeUrl}
					target="_blank"
					rel="noreferrer"
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
					<span>{docsUiConfig.docActions.assistants.claude.label}</span>
					<Launch class="ml-auto size-4 flex-none" />
				</a>
			{/if}
		</div>
	</div>
{/if}
