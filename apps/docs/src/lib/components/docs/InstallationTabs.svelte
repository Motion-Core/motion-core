<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";
	import ShikiCodeBlock from "./ShikiCodeBlock.svelte";
	import { getHighlighter } from "$lib/utils/highlighter";

	type Props = {
		component: string;
	};

	let { component }: Props = $props();

	const packageManagers = ["npm", "pnpm", "bun", "yarn"] as const;
	type PackageManager = (typeof packageManagers)[number];

	let activeTab = $state<PackageManager>("npm");

	const commands: Record<PackageManager, string> = $derived({
		npm: `npx @motion-core/cli add ${component}`,
		pnpm: `pnpm dlx @motion-core/cli add ${component}`,
		bun: `bunx @motion-core/cli add ${component}`,
		yarn: `yarn dlx @motion-core/cli add ${component}`,
	});

	const activeCommand = $derived(commands[activeTab]);

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

<div
	class="my-6 w-full overflow-hidden rounded-md border border-border bg-card shadow-sm"
>
	<div
		class="flex items-center justify-between border-b border-border bg-card-muted/60"
	>
		<div class="flex items-center">
			{#each packageManagers as pm (pm)}
				<button
					onclick={() => (activeTab = pm)}
					class={cn(
						"relative px-4 py-2.5 text-sm font-medium transition-colors outline-none select-none",
						activeTab === pm
							? "text-foreground"
							: "text-foreground/45 hover:text-foreground/70",
					)}
				>
					{pm}
					{#if activeTab === pm}
						<div class="absolute bottom-0 left-0 h-0.5 w-full bg-accent"></div>
					{/if}
				</button>
			{/each}
		</div>

		<CopyCodeButton code={activeCommand} class="mr-2" />
	</div>

	<div class="min-h-12.5 p-4">
		{#if highlightedCommands[activeTab]}
			<ShikiCodeBlock
				code=""
				htmlLight={highlightedCommands[activeTab]!.light}
				htmlDark={highlightedCommands[activeTab]!.dark}
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
