<script lang="ts">
	import { cn } from "$lib/utils/cn";
	import InfoPopover from "./InfoPopover.svelte";
	import Table from "./markdown/Table.svelte";
	import Tbody from "./markdown/Tbody.svelte";
	import Td from "./markdown/Td.svelte";
	import Th from "./markdown/Th.svelte";
	import Thead from "./markdown/Thead.svelte";
	import Tr from "./markdown/Tr.svelte";

	type Row = {
		description?: string;
		[key: string]: unknown;
	};

	let {
		headers = [],
		keys = [],
		data = [],
	}: {
		headers: string[];
		keys: string[];
		data: Row[];
	} = $props();
</script>

<Table>
	<Thead>
		<Tr>
			{#each headers as header (header)}
				<Th>{header}</Th>
			{/each}
		</Tr>
	</Thead>
	<Tbody>
		{#each data as row, i (i)}
			<Tr>
				{#each keys as key, index (key)}
					<Td>
						<span
							class={cn(
								"inline-flex items-center gap-1",
								key === "type" && "max-w-xs flex-wrap",
							)}
						>
							{#if key === "type" && typeof row[key] === "string" && row[key].includes("|")}
								{#each row[key].split("|") as type, i (type + i)}
									<code
										class={cn(
											"inset-shadow relative inline-flex rounded-sm border border-border bg-background-inset px-px py-0.5 text-sm font-medium whitespace-nowrap text-foreground",
										)}
									>
										<span
											class="rounded-[calc(var(--radius-base)*1.5)] border border-border bg-background px-1.5 py-0.5 font-mono shadow-sm"
										>
											{type.trim()}
										</span>
									</code>
								{/each}
							{:else}
								<code
									class={cn(
										"inset-shadow relative inline-flex rounded-sm border border-border bg-background-inset px-px py-0.5 text-sm font-medium text-foreground",
									)}
								>
									<span
										class="rounded-[calc(var(--radius-base)*1.5)] border border-border bg-background px-1.5 py-0.5 font-mono shadow-sm"
									>
										{row[key]}
									</span>
								</code>
							{/if}
							{#if index === 0 && row.description}
								<InfoPopover description={row.description} />
							{/if}
						</span>
					</Td>
				{/each}
			</Tr>
		{/each}
	</Tbody>
</Table>
