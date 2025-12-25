import { CustomEase } from "gsap/CustomEase";
import gsap from "gsap";

gsap.registerPlugin(CustomEase);

CustomEase.create("motion-core-ease", "0.625, 0.05, 0, 1");

export const motionCoreEase = "motion-core-ease";
