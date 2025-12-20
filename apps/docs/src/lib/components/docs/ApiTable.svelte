<script lang="ts">
	import Table from './markdown/Table.svelte';
	import Thead from './markdown/Thead.svelte';
	import Tbody from './markdown/Tbody.svelte';
	import Tr from './markdown/Tr.svelte';
	import Th from './markdown/Th.svelte';
	import Td from './markdown/Td.svelte';
	import InfoPopover from './InfoPopover.svelte';
	import { cn } from '$lib/utils/cn';

	type Row = {
		description?: string;
		[key: string]: any;
	};

	let {
		headers = [],
		keys = [],
		data = []
	}: {
		headers: string[];
		keys: string[];
		data: Row[];
	} = $props();
</script>

<Table>
	<Thead>
		<Tr>
			{#each headers as header}
				<Th>{header}</Th>
			{/each}
		</Tr>
	</Thead>
	<Tbody>
		{#each data as row}
			<Tr>
				{#each keys as key, index}
					<Td>
						<span class="inline-flex items-center">
							<code
								class={cn(
									'rounded border border-border/50 px-1.5 py-0.5 font-mono font-normal',
									index === 0
										? 'bg-primary/10 text-primary font-medium border-primary/20'
										: 'bg-muted/40 text-muted-foreground'
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
