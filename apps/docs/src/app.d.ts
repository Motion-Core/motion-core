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

export {};
