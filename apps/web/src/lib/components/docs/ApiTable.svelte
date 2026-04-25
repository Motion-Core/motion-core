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
											"relative inline-flex rounded-sm bg-background-inset p-1 text-sm font-medium text-foreground inset-shadow",
										)}
									>
										<span
											class="rounded-[calc(var(--radius-base)*1.25)] bg-background px-1 py-0.5 font-mono card"
										>
											{type.trim()}
										</span>
									</code>
								{/each}
							{:else}
								<code
									class={cn(
										"relative inline-flex rounded-sm bg-background-inset p-1 text-sm font-medium text-foreground inset-shadow",
									)}
								>
									<span
										class="rounded-[calc(var(--radius-base)*1.25)] bg-background px-1 py-0.5 font-mono card"
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
