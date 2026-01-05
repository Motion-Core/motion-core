import { browser } from "$app/environment";

export const packageManagers = ["npm", "pnpm", "bun", "yarn"] as const;
export type PackageManager = (typeof packageManagers)[number];

function createPackageManagerStore() {
	let active = $state<PackageManager>("npm");

	if (browser) {
		const stored = localStorage.getItem("package-manager") as PackageManager;
		if (stored && packageManagers.includes(stored)) {
			active = stored;
		}
	}

	return {
		get active() {
			return active;
		},
		set active(v: PackageManager) {
			active = v;
			if (browser) {
				localStorage.setItem("package-manager", v);
			}
		},
	};
}

export const packageManagerStore = createPackageManagerStore();
