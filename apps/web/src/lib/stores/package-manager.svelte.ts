import { browser } from "$app/environment";
import {
	docsUiConfig,
	availablePackageManagers,
	type PackageManagerOption,
} from "$lib/config/docs-ui";

export type PackageManager = PackageManagerOption;

const enabledManagers = Array.from(
	new Set(
		docsUiConfig.packageManager.enabled.filter((pm): pm is PackageManager =>
			availablePackageManagers.includes(pm),
		),
	),
);

export const packageManagers: PackageManager[] =
	enabledManagers.length > 0 ? enabledManagers : ["npm"];

function createPackageManagerStore() {
	const configuredDefault = docsUiConfig.packageManager.default;
	let active = $state<PackageManager>(
		packageManagers.includes(configuredDefault)
			? configuredDefault
			: packageManagers[0],
	);

	if (browser) {
		const stored = localStorage.getItem(
			docsUiConfig.packageManager.storageKey,
		) as PackageManager;
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
				localStorage.setItem(docsUiConfig.packageManager.storageKey, v);
			}
		},
	};
}

export const packageManagerStore = createPackageManagerStore();
