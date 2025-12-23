<script lang="ts">
	import { type Component } from "svelte";
	import LogoColumn from "./LogoColumn.svelte";
	import { cn } from "../../utils/cn";

	interface Logo {
		name: string;
		id: number;
		component: Component;
	}

	interface Props {
		columnCount?: number;
		logos: Logo[];
		cycleInterval?: number;
		class?: string;
	}

	let {
		columnCount = 2,
		logos,
		cycleInterval = 2000,
		class: className,
	}: Props = $props();

	const shuffleArray = <T,>(array: T[]): T[] => {
		const shuffled = [...array];
		for (let i = shuffled.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
		}
		return shuffled;
	};

	const distributeLogos = (allLogos: Logo[], columnCount: number): Logo[][] => {
		const shuffled = shuffleArray(allLogos);
		const columns: Logo[][] = Array.from({ length: columnCount }, () => []);

		shuffled.forEach((logo, index) => {
			columns[index % columnCount].push(logo);
		});

		const maxLength = Math.max(...columns.map((col) => col.length));
		columns.forEach((col) => {
			while (col.length < maxLength) {
				col.push(shuffled[Math.floor(Math.random() * shuffled.length)]);
			}
		});

		return columns;
	};

	let logoSets = $derived(distributeLogos(logos, columnCount));
</script>

<div class={cn("flex space-x-4", className)}>
	{#each logoSets as logos, index (index)}
		<LogoColumn {logos} {index} {cycleInterval} />
	{/each}
</div>
