<script lang="ts">
	import Table from "./markdown/Table.svelte";
	import Thead from "./markdown/Thead.svelte";
	import Tbody from "./markdown/Tbody.svelte";
	import Tr from "./markdown/Tr.svelte";
	import Th from "./markdown/Th.svelte";
	import Td from "./markdown/Td.svelte";
	import InfoPopover from "./InfoPopover.svelte";
	import { cn } from "$lib/utils/cn";

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
											"rounded-lg border border-border bg-card-muted px-1.5 py-0.5 font-normal whitespace-nowrap text-foreground shadow-sm mono",
										)}
									>
										{type.trim()}
									</code>
								{/each}
							{:else}
								<code
									class={cn(
										"rounded-lg border border-border bg-card-muted px-1.5 py-0.5 font-normal text-foreground shadow-sm mono",
									)}
								>
									{row[key]}
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
