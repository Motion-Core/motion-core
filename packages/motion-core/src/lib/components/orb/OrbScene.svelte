<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { Color, ShaderMaterial, Mesh, Vector3 } from "three";
	import { orbVertex, orbFragment } from "./orb.glsl";
	import type { OrbState, StateConfig } from "./types";

	interface Props {
		/**
		 * Current animation state of the orb.
		 * @default "idle"
		 */
		orbState?: OrbState;
		/**
		 * Audio amplitude in [0, 1] that drives reactive pulse.
		 * @default 0
		 */
		amplitude?: number;
		/**
		 * Base color for the orb palette and rim glow. The full 5-color palette
		 * is derived from this hue using HSL variations.
		 * @default "#6933ff"
		 */
		color?: string;
		/**
		 * Speed multiplier applied to the entire animation. 1 is default, 2 is double speed.
		 * @default 1
		 */
		speed?: number;
	}

	let {
		orbState = "idle",
		amplitude = 0,
		color = "#6933ff",
		speed = 1,
	}: Props = $props();

	const idlePalette: StateConfig["palette"] = [
		[0.04, 0.01, 0.25],
		[0.92, 0.42, 0.5],
		[0.22, 0.04, 0.5],
		[0.88, 0.58, 0.48],
		[0.1, 0.04, 0.45],
	];

	const STATES: Record<OrbState, StateConfig> = {
		idle: {
			palette: idlePalette,
			fresnelColor: [0.55, 0.3, 0.82],
			fresnelStrength: 0.8,
			noiseSpeed: 0.5,
			flowDrift: 1,
			flowOut: 0,
			flowIn: 0,
			flowPulse: 0,
			flowSwirl: 0,
			basePulse: 0,
		},
		attune: {
			palette: idlePalette,
			fresnelColor: [0.55, 0.3, 0.82],
			fresnelStrength: 1.1,
			noiseSpeed: 0.9,
			flowDrift: 0,
			flowOut: 0,
			flowIn: 1.6,
			flowPulse: 0,
			flowSwirl: 0,
			basePulse: 0.01,
		},
		pulse: {
			palette: idlePalette,
			fresnelColor: [0.55, 0.3, 0.82],
			fresnelStrength: 1.3,
			noiseSpeed: 0.8,
			flowDrift: 0,
			flowOut: 0,
			flowIn: 0,
			flowPulse: 1,
			flowSwirl: 0,
			basePulse: 0.01,
		},
		surge: {
			palette: idlePalette,
			fresnelColor: [0.55, 0.3, 0.82],
			fresnelStrength: 1.2,
			noiseSpeed: 2.0,
			flowDrift: 0,
			flowOut: 0,
			flowIn: 0,
			flowPulse: 0,
			flowSwirl: 1.5,
			basePulse: 0.03,
		},
	};

	const LERP_KEYS: (keyof StateConfig)[] = [
		"noiseSpeed",
		"fresnelStrength",
		"flowDrift",
		"flowOut",
		"flowIn",
		"flowPulse",
		"flowSwirl",
	];

	// Material refs — updated by useTask
	let orbMaterial = $state<ShaderMaterial>();
	let orbMesh = $state<Mesh>();

	// Pre-allocated Color objects for palette derivation (reused each frame)
	const _base = new Color();
	const _hsl = { h: 0, s: 0, l: 0 };
	const _pal = Array.from({ length: 5 }, () => new Color());
	const _fresnel = new Color();

	function applyCustomColor(
		hex: string,
		u: Record<string, { value: unknown }>,
	) {
		_base.set(hex);
		_base.getHSL(_hsl);
		const { h, s } = _hsl;

		// Derive 5 harmonious palette entries from the base hue
		_pal[0].setHSL(h, Math.min(s * 1.2, 1), 0.08); // very dark
		_pal[1].setHSL((h + 0.05) % 1, s * 0.85, 0.65); // bright, slight shift
		_pal[2].setHSL(h, Math.min(s * 1.1, 1), 0.22); // mid
		_pal[3].setHSL((h + 0.08) % 1, s * 0.65, 0.72); // light, more shift
		_pal[4].setHSL(h, s * 0.9, 0.14); // deep

		for (let i = 0; i < 5; i++) {
			const c = _pal[i];
			(u[`uPalA${i}`].value as Vector3).set(c.r, c.g, c.b);
			(u[`uPalB${i}`].value as Vector3).set(c.r, c.g, c.b);
		}
		u.uPaletteBlend.value = 0;

		// Fresnel: brighter, saturated version of the base hue
		_fresnel.setHSL(h, Math.min(s * 1.3, 1), 0.68);
		(u.uFresnelColor.value as Vector3).set(_fresnel.r, _fresnel.g, _fresnel.b);
	}

	// Transition mutable state (not reactive — mutated in task)
	let elapsed = 0;
	let smoothAmp = 0;
	let currentOrbState: OrbState = "idle";
	let transitionProgress = 1;
	let snapScalars: Partial<Record<keyof StateConfig, number>> = {};
	let snapFresnel = [0.55, 0.3, 0.82];
	let targetConfig: StateConfig = STATES.idle;

	const initCfg = STATES.idle;

	// Uniform initial values (passed once; updated imperatively via ref)
	const orbUniforms = {
		uTime: { value: 0 },
		uLightPos: { value: new Vector3(-2.5, 1.0, 3.0) },
		uAmplitude: { value: 0 },
		uNoiseSpeed: { value: initCfg.noiseSpeed },
		uFlowDrift: { value: initCfg.flowDrift },
		uFlowOut: { value: 0 },
		uFlowIn: { value: 0 },
		uFlowPulse: { value: 0 },
		uFlowSwirl: { value: 0 },
		uFresnelColor: { value: new Vector3(...initCfg.fresnelColor) },
		uFresnelStrength: { value: initCfg.fresnelStrength },
		uPaletteBlend: { value: 0 },
		uPalA0: { value: new Vector3(...initCfg.palette[0]) },
		uPalA1: { value: new Vector3(...initCfg.palette[1]) },
		uPalA2: { value: new Vector3(...initCfg.palette[2]) },
		uPalA3: { value: new Vector3(...initCfg.palette[3]) },
		uPalA4: { value: new Vector3(...initCfg.palette[4]) },
		uPalB0: { value: new Vector3(...initCfg.palette[0]) },
		uPalB1: { value: new Vector3(...initCfg.palette[1]) },
		uPalB2: { value: new Vector3(...initCfg.palette[2]) },
		uPalB3: { value: new Vector3(...initCfg.palette[3]) },
		uPalB4: { value: new Vector3(...initCfg.palette[4]) },
	};

	function lerp(a: number, b: number, t: number) {
		return a + (b - a) * t;
	}

	function startTransition(newOrbState: OrbState) {
		if (!orbMaterial) return;
		const u = orbMaterial.uniforms;

		// Snapshot palette A from current blended position
		if (transitionProgress < 1) {
			const b = transitionProgress;
			for (let i = 0; i < 5; i++) {
				const a = u[`uPalA${i}`].value as Vector3;
				const bv = u[`uPalB${i}`].value as Vector3;
				a.lerpVectors(a.clone(), bv, b);
			}
		} else {
			for (let i = 0; i < 5; i++) {
				(u[`uPalA${i}`].value as Vector3).copy(u[`uPalB${i}`].value as Vector3);
			}
		}

		const cfg = STATES[newOrbState];
		for (let i = 0; i < 5; i++) {
			(u[`uPalB${i}`].value as Vector3).set(...cfg.palette[i]);
		}

		// Snapshot current scalar uniform values
		snapScalars = {};
		for (const k of LERP_KEYS) {
			const uKey = `u${k[0].toUpperCase()}${k.slice(1)}`;
			snapScalars[k] = u[uKey].value as number;
		}
		const fc = u.uFresnelColor.value as Vector3;
		snapFresnel = [fc.x, fc.y, fc.z];

		targetConfig = cfg;
		currentOrbState = newOrbState;
		transitionProgress = 0;
	}

	const { scene } = useThrelte();
	scene.background = null;

	useTask((delta) => {
		if (!orbMaterial || !orbMesh) return;

		elapsed += delta * speed;
		const u = orbMaterial.uniforms;

		if (orbState !== currentOrbState) startTransition(orbState);

		// Advance transition
		if (transitionProgress < 1) {
			transitionProgress = Math.min(1, transitionProgress + delta / 0.4);
			const ease = transitionProgress * (2 - transitionProgress);
			u.uPaletteBlend.value = ease;

			for (const k of LERP_KEYS) {
				const uKey = `u${k[0].toUpperCase()}${k.slice(1)}`;
				u[uKey].value = lerp(
					snapScalars[k] ?? (targetConfig[k] as number),
					targetConfig[k] as number,
					ease,
				);
			}

			const fc = u.uFresnelColor.value as Vector3;
			fc.set(
				lerp(snapFresnel[0], targetConfig.fresnelColor[0], ease),
				lerp(snapFresnel[1], targetConfig.fresnelColor[1], ease),
				lerp(snapFresnel[2], targetConfig.fresnelColor[2], ease),
			);
		}

		// Smooth amplitude: fast attack, slow release
		const rising = amplitude > smoothAmp;
		smoothAmp +=
			(amplitude - smoothAmp) * Math.min(1, delta * (rising ? 14 : 4));

		u.uTime.value = elapsed;
		u.uAmplitude.value = smoothAmp;

		// Color override: derive full palette + fresnel from user-provided color
		if (color) applyCustomColor(color, u);

		// Gentle breathing
		const basePulse = STATES[currentOrbState].basePulse;
		orbMesh.scale.setScalar(1 + basePulse * Math.sin(elapsed * 2.5) * 0.5);
		orbMesh.rotation.y = elapsed * 0.12;
	});
</script>

<T.PerspectiveCamera
	makeDefault
	fov={50}
	near={0.1}
	far={100}
	position={[0, 0, 4]}
/>

<T.Mesh bind:ref={orbMesh}>
	<T.SphereGeometry args={[0.75, 128, 128]} />
	<T.ShaderMaterial
		bind:ref={orbMaterial}
		vertexShader={orbVertex}
		fragmentShader={orbFragment}
		uniforms={orbUniforms}
	/>
</T.Mesh>
