<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";
	import ShikiCodeBlock from "./ShikiCodeBlock.svelte";
	import { getHighlighter } from "$lib/utils/highlighter";
	import {
		packageManagers,
		packageManagerStore,
		type PackageManager,
	} from "$lib/stores/package-manager.svelte";

	type Props = {
		component?: string;
		args?: string;
		mode?: "execute" | "global";
	};

	let { component, args, mode = "execute" }: Props = $props();

	const commands: Record<PackageManager, string> = $derived(
		mode === "global"
			? {
					npm: "npm install -g @motion-core/cli",
					pnpm: "pnpm add -g @motion-core/cli",
					bun: "bun add -g @motion-core/cli",
					yarn: "yarn global add @motion-core/cli",
				}
			: {
					npm: `npx @motion-core/cli ${args ?? `add ${component}`}`,
					pnpm: `pnpm dlx @motion-core/cli ${args ?? `add ${component}`}`,
					bun: `bunx @motion-core/cli ${args ?? `add ${component}`}`,
					yarn: `yarn dlx @motion-core/cli ${args ?? `add ${component}`}`,
				},
	);

	const activeCommand = $derived(commands[packageManagerStore.active]);

	let highlightedCommands = $state<
		Record<PackageManager, { light: string; dark: string } | null>
	>({
		npm: null,
		pnpm: null,
		bun: null,
		yarn: null,
	});

	$effect(() => {
		getHighlighter().then((highlighter) => {
			for (const pm of packageManagers) {
				const cmd = commands[pm];
				highlightedCommands[pm] = {
					light: highlighter.codeToHtml(cmd, {
						lang: "bash",
						theme: "github-light",
					}),
					dark: highlighter.codeToHtml(cmd, {
						lang: "bash",
						theme: "github-dark",
					}),
				};
			}
		});
	});
</script>

<div class="card-highlight relative my-6 w-full rounded-xl bg-card shadow-sm">
	<div
		class="flex items-center justify-between rounded-t-xl border-b border-border bg-card-muted/60"
	>
		<div class="flex items-center">
			{#each packageManagers as pm (pm)}
				<button
					onclick={() => (packageManagerStore.active = pm)}
					class={cn(
						"relative px-4 py-2.5 text-sm font-medium transition-colors outline-none select-none",
						packageManagerStore.active === pm
							? "text-foreground"
							: "text-foreground/45 hover:text-foreground/70",
					)}
				>
					{pm}
					{#if packageManagerStore.active === pm}
						<div class="absolute bottom-0 left-0 h-0.5 w-full bg-accent"></div>
					{/if}
				</button>
			{/each}
		</div>

		<CopyCodeButton code={activeCommand} class="mr-2" />
	</div>

	<div class="min-h-12.5 p-4">
		{#if highlightedCommands[packageManagerStore.active]}
			<ShikiCodeBlock
				code=""
				htmlLight={highlightedCommands[packageManagerStore.active]!.light}
				htmlDark={highlightedCommands[packageManagerStore.active]!.dark}
				unstyled={true}
			/>
		{:else}
			<code
				class="block text-sm leading-relaxed whitespace-pre text-foreground mono"
			>
				{activeCommand}
			</code>
		{/if}
	</div>
</div>
