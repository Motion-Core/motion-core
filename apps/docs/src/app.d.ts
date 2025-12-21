// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

declare module "*.svx" {
	import type { SvelteComponentTyped } from "svelte";
	export default class extends SvelteComponentTyped<Record<string, unknown>> {}
}

declare module "*.svg?raw" {
	const content: string;
	export default content;
}

declare module "*.svelte?raw" {
	const content: string;
	export default content;
}

declare module "*.ts?raw" {
	const content: string;
	export default content;
}

export {};
