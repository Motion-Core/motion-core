import gsap from "gsap";
import CustomEaseModule from "gsap/CustomEase";

type CustomEasePlugin = {
	create: (name: string, curve: string) => void;
};

const resolveCustomEase = (): CustomEasePlugin => {
	const module = CustomEaseModule as CustomEasePlugin & {
		CustomEase?: CustomEasePlugin;
	};
	return module.CustomEase ?? module;
};

if (typeof window !== "undefined") {
	const CustomEase = resolveCustomEase();
	gsap.registerPlugin(CustomEase as never);
	CustomEase.create("motion-core-ease", "0.625, 0.05, 0, 1");
}

export const motionCoreEase = "motion-core-ease";
