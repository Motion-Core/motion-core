import { gsap } from "gsap";
import { CustomEase } from "gsap/dist/CustomEase";

const registeredPlugins = new Set<object>();

const MOTION_CORE_EASE_NAME = "motion-core-ease";
const MOTION_CORE_EASE_CURVE = "0.625, 0.05, 0, 1";

let motionCoreEaseRegistered = false;

export function registerPluginOnce(...plugins: object[]) {
	const unique = plugins.filter((plugin) => !registeredPlugins.has(plugin));
	if (!unique.length) return;

	gsap.registerPlugin(...unique);
	unique.forEach((plugin) => {
		registeredPlugins.add(plugin);
	});
}

export function ensureMotionCoreEase() {
	registerPluginOnce(CustomEase);
	if (!motionCoreEaseRegistered) {
		CustomEase.create(MOTION_CORE_EASE_NAME, MOTION_CORE_EASE_CURVE);
		motionCoreEaseRegistered = true;
	}
	return MOTION_CORE_EASE_NAME;
}
