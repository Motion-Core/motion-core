declare module "*.png" {
	const value: string;
	export default value;
}

declare module "*.jpg" {
	const value: string;
	export default value;
}

declare module "*.jpeg" {
	const value: string;
	export default value;
}

declare module "*.webp" {
	const value: string;
	export default value;
}

declare module "*.svg" {
	const value: string;
	export default value;
}

declare module "*.geojson?raw" {
	const value: string;
	export default value;
}

declare module "gsap/dist/gsap" {
	export * from "gsap";
	export { gsap } from "gsap";
	export { gsap as default } from "gsap";
}

declare module "gsap/dist/ScrollTrigger" {
	export * from "gsap/ScrollTrigger";
	export { ScrollTrigger } from "gsap/ScrollTrigger";
	export { ScrollTrigger as default } from "gsap/ScrollTrigger";
}

declare module "gsap/dist/SplitText" {
	export * from "gsap/SplitText";
	export { SplitText } from "gsap/SplitText";
	export { SplitText as default } from "gsap/SplitText";
}
