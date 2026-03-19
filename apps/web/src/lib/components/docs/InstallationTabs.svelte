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
	import { siteConfig } from "$lib/config/site";

	type Props = {
		pkg?: string;
		args?: string;
		isDev?: boolean;
	};

	let { pkg = siteConfig.package.name, args, isDev = false }: Props = $props();

	const commands: Record<PackageManager, string> = $derived(
		isDev
			? {
					npm: `npm install -D ${pkg} ${args ?? ""}`,
					pnpm: `pnpm add -D ${pkg} ${args ?? ""}`,
					bun: `bun add -D ${pkg} ${args ?? ""}`,
					yarn: `yarn add -D ${pkg} ${args ?? ""}`,
				}
			: {
					npm: `npm install ${pkg} ${args ?? ""}`,
					pnpm: `pnpm add ${pkg} ${args ?? ""}`,
					bun: `bun add ${pkg} ${args ?? ""}`,
					yarn: `yarn add ${pkg} ${args ?? ""}`,
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

<div class="inset-shadow my-6 rounded-lg bg-background-inset p-1.5">
	<div class="card relative w-full rounded-md bg-background">
		<div
			class="flex items-center justify-between rounded-t-md border-b border-border"
		>
			<div class="flex items-center">
				{#each packageManagers as pm (pm)}
					<button
						onclick={() => (packageManagerStore.active = pm)}
						class={cn(
							"relative px-4 py-2.5 text-sm font-medium tracking-normal transition-colors duration-150 ease-out outline-none select-none",
							packageManagerStore.active === pm
								? "text-foreground"
								: "text-foreground-muted hover:text-foreground",
						)}
					>
						{pm}
						{#if packageManagerStore.active === pm}
							<div
								class="absolute bottom-0 left-0 h-0.5 w-full bg-accent"
							></div>
						{/if}
					</button>
				{/each}
			</div>

			<CopyCodeButton code={activeCommand} class="mr-2" />
		</div>

		<div
			class="min-h-12.5 p-4 [&>div]:mt-0 [&>div]:rounded-none [&>div]:border-0 [&>div]:bg-transparent [&>div]:p-0 [&>div]:shadow-none [&>div]:[box-shadow:none]!"
		>
			{#if highlightedCommands[packageManagerStore.active]}
				<ShikiCodeBlock
					code=""
					htmlLight={highlightedCommands[packageManagerStore.active]!.light}
					htmlDark={highlightedCommands[packageManagerStore.active]!.dark}
					unstyled={true}
				/>
			{:else}
				<code
					class="block font-mono text-sm leading-relaxed whitespace-pre text-foreground"
				>
					{activeCommand}
				</code>
			{/if}
		</div>
	</div>
</div>
