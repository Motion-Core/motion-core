<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import CopyCodeButton from "./markdown/CopyCodeButton.svelte";

	type Props = {
		component: string;
	};

	let { component }: Props = $props();

	const packageManagers = ["npm", "pnpm", "bun", "yarn"] as const;
	type PackageManager = (typeof packageManagers)[number];

	let activeTab = $state<PackageManager>("npm");

	const prefixes: Record<PackageManager, string> = {
		npm: "npx",
		pnpm: "pnpm dlx",
		bun: "bunx",
		yarn: "yarn dlx",
	};

	const activePrefix = $derived(prefixes[activeTab]);

	const commands: Record<PackageManager, string> = $derived({
		npm: `npx @motion-core/cli add ${component}`,
		pnpm: `pnpm dlx @motion-core/cli add ${component}`,
		bun: `bunx @motion-core/cli add ${component}`,
		yarn: `yarn dlx @motion-core/cli add ${component}`,
	});

	const activeCommand = $derived(commands[activeTab]);
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
						<div
							class="absolute bottom-0 left-0 h-0.5 w-full bg-accent"
							style="view-transition-name: install-tabs-cursor"
						></div>
					{/if}
				</button>
			{/each}
		</div>

		<CopyCodeButton
			code={activeCommand}
			class="bg-transparent mr-2 border-none shadow-none"
		/>
	</div>

	<div class="p-4">
		<div class="overflow-x-auto">
			<code
				class="block text-sm leading-relaxed whitespace-pre mono font-medium"
			>
				<span class="text-accent mr-1 select-none">$</span>
				<span class="light:text-[#6F42C1] dark:text-[#B392F0]"
					>{activePrefix}</span
				>
				<span class="light:text-[#032F62] dark:text-[#9DCBFF]"
					>@motion-core/cli</span
				>
				<span class="light:text-[#032F62] dark:text-[#9DCBFF]">add</span>
				<span class="light:text-[#032F62] dark:text-[#9DCBFF]">{component}</span
				>
			</code>
		</div>
	</div>
</div>
