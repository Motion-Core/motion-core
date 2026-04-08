export type OrbState = "idle" | "attune" | "pulse" | "surge";

export type StateConfig = {
	palette: [number[], number[], number[], number[], number[]];
	fresnelColor: [number, number, number];
	fresnelStrength: number;
	noiseSpeed: number;
	flowDrift: number;
	flowOut: number;
	flowIn: number;
	flowPulse: number;
	flowSwirl: number;
	basePulse: number;
};
