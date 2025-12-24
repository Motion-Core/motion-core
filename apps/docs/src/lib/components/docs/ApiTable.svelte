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
						<span class="inline-flex items-center">
							<code
								class={cn(
									"rounded bg-card-muted border border-border px-1.5 py-0.5 mono font-normal text-foreground",
								)}
							>
								{row[key]}
							</code>
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
