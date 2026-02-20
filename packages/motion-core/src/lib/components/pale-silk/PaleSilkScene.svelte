<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";

	const BLEND_MODE_MAP = {
		vividLight: 0,
		softLight: 1,
		hardLight: 2,
		pinLight: 3,
		linearLight: 4,
	} as const;

	type BlendMode = keyof typeof BLEND_MODE_MAP;

	interface Props {
		/**
		 * Base color of the shader before highlights and blending.
		 * @default "#422042"
		 */
		baseColor?: THREE.ColorRepresentation;
		/**
		 * Global intensity multiplier applied to the base color.
		 * @default 1.7
		 */
		brightness?: number;
		/**
		 * Vertical wave displacement amplitude.
		 * @default 0.03
		 */
		waveAmplitude?: number;
		/**
		 * Horizontal frequency of the wave distortion.
		 * @default 8.0
		 */
		waveFrequency?: number;
		/**
		 * Speed multiplier for the wave and procedural animation.
		 * @default 1.0
		 */
		waveSpeed?: number;
		/**
		 * Simplex noise coefficient controlling spot falloff.
		 * @default 0.61
		 */
		noiseSizeCoeff?: number;
		/**
		 * Simplex noise density multiplier.
		 * @default 53.0
		 */
		noiseDensity?: number;
		/**
		 * Spatial scale for simplex noise sampling.
		 * @default 10.0
		 */
		noiseScale?: number;
		/**
		 * Strength of the glitter/noise layer before blend.
		 * @default 8.0
		 */
		noiseStrength?: number;
		/**
		 * Blend mode used to combine glitter noise with base color.
		 * @default "vividLight"
		 */
		blendMode?: BlendMode;
		/**
		 * Opacity of the selected blend mode.
		 * @default 0.02
		 */
		blendStrength?: number;
		/**
		 * Vignette intensity coefficient.
		 * @default 15.0
		 */
		vignetteStrength?: number;
		/**
		 * Vignette curve exponent.
		 * @default 0.25
		 */
		vignettePower?: number;
		/**
		 * Strength of the extra flow shading pass.
		 * @default 1.0
		 */
		flowShadingStrength?: number;
	}

	let {
		baseColor = "#422042",
		brightness = 1.7,
		waveAmplitude = 0.03,
		waveFrequency = 8.0,
		waveSpeed = 1.0,
		noiseSizeCoeff = 0.61,
		noiseDensity = 53.0,
		noiseScale = 10.0,
		noiseStrength = 8.0,
		blendMode = "vividLight",
		blendStrength = 0.02,
		vignetteStrength = 15.0,
		vignettePower = 0.25,
		flowShadingStrength = 1.0,
	}: Props = $props();

	let material = $state<THREE.ShaderMaterial>();
	const { size } = useThrelte();
	const resolutionUniform = new THREE.Vector2(1, 1);
	const baseColorUniform = new THREE.Color();

	const vertexShader = `
		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = vec4(position, 1.0);
		}
	`;

	const fragmentShader = `
		precision highp float;
		varying vec2 vUv;

		uniform float uTime;
		uniform vec2 uResolution;
		uniform vec3 uBaseColor;
		uniform float uBrightness;
		uniform float uWaveAmplitude;
		uniform float uWaveFrequency;
		uniform float uWaveSpeed;
		uniform float uNoiseSizeCoeff;
		uniform float uNoiseDensity;
		uniform float uNoiseScale;
		uniform float uNoiseStrength;
		uniform int uBlendMode;
		uniform float uBlendStrength;
		uniform float uVignetteStrength;
		uniform float uVignettePower;
		uniform float uFlowShadingStrength;

		vec3 mod289(vec3 x) {
			return x - floor(x * (1.0 / 289.0)) * 289.0;
		}

		vec4 mod289(vec4 x) {
			return x - floor(x * (1.0 / 289.0)) * 289.0;
		}

		vec4 permute(vec4 x) {
			return mod289(((x * 34.0) + 1.0) * x);
		}

		vec4 taylorInvSqrt(vec4 r) {
			return 1.79284291400159 - 0.85373472095314 * r;
		}

		float snoise(vec3 v) {
			const vec2 C = vec2(1.0 / 6.0, 1.0 / 3.0);
			const vec4 D = vec4(0.0, 0.5, 1.0, 2.0);

			vec3 i = floor(v + dot(v, C.yyy));
			vec3 x0 = v - i + dot(i, C.xxx);

			vec3 g = step(x0.yzx, x0.xyz);
			vec3 l = 1.0 - g;
			vec3 i1 = min(g.xyz, l.zxy);
			vec3 i2 = max(g.xyz, l.zxy);

			vec3 x1 = x0 - i1 + C.xxx;
			vec3 x2 = x0 - i2 + C.yyy;
			vec3 x3 = x0 - D.yyy;

			i = mod289(i);
			vec4 p = permute(permute(permute(i.z + vec4(0.0, i1.z, i2.z, 1.0)) +
				i.y + vec4(0.0, i1.y, i2.y, 1.0)) +
				i.x + vec4(0.0, i1.x, i2.x, 1.0));

			float n_ = 0.142857142857;
			vec3 ns = n_ * D.wyz - D.xzx;

			vec4 j = p - 49.0 * floor(p * ns.z * ns.z);

			vec4 x_ = floor(j * ns.z);
			vec4 y_ = floor(j - 7.0 * x_);

			vec4 x = x_ * ns.x + ns.yyyy;
			vec4 y = y_ * ns.x + ns.yyyy;
			vec4 h = 1.0 - abs(x) - abs(y);

			vec4 b0 = vec4(x.xy, y.xy);
			vec4 b1 = vec4(x.zw, y.zw);

			vec4 s0 = floor(b0) * 2.0 + 1.0;
			vec4 s1 = floor(b1) * 2.0 + 1.0;
			vec4 sh = -step(h, vec4(0.0));

			vec4 a0 = b0.xzyw + s0.xzyw * sh.xxyy;
			vec4 a1 = b1.xzyw + s1.xzyw * sh.zzww;

			vec3 p0 = vec3(a0.xy, h.x);
			vec3 p1 = vec3(a0.zw, h.y);
			vec3 p2 = vec3(a1.xy, h.z);
			vec3 p3 = vec3(a1.zw, h.w);

			vec4 norm = taylorInvSqrt(vec4(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
			p0 *= norm.x;
			p1 *= norm.y;
			p2 *= norm.z;
			p3 *= norm.w;

			vec4 m = max(uNoiseSizeCoeff - vec4(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), 0.0);
			m = m * m;

			return uNoiseDensity * dot(m * m, vec4(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
		}

		float softLight(float s, float d) {
			return (s < 0.5)
				? d - (1.0 - 2.0 * s) * d * (1.0 - d)
				: (d < 0.25)
					? d + (2.0 * s - 1.0) * d * ((16.0 * d - 12.0) * d + 3.0)
					: d + (2.0 * s - 1.0) * (sqrt(d) - d);
		}

		vec3 softLight(vec3 s, vec3 d) {
			return vec3(softLight(s.x, d.x), softLight(s.y, d.y), softLight(s.z, d.z));
		}

		float hardLight(float s, float d) {
			return (s < 0.5) ? 2.0 * s * d : 1.0 - 2.0 * (1.0 - s) * (1.0 - d);
		}

		vec3 hardLight(vec3 s, vec3 d) {
			return vec3(hardLight(s.x, d.x), hardLight(s.y, d.y), hardLight(s.z, d.z));
		}

		float vividLight(float s, float d) {
			float safeS = clamp(s, 0.001, 0.999);
			return (safeS < 0.5)
				? 1.0 - (1.0 - d) / (2.0 * safeS)
				: d / (2.0 * (1.0 - safeS));
		}

		vec3 vividLight(vec3 s, vec3 d) {
			return vec3(vividLight(s.x, d.x), vividLight(s.y, d.y), vividLight(s.z, d.z));
		}

		vec3 linearLight(vec3 s, vec3 d) {
			return 2.0 * s + d - 1.0;
		}

		float pinLight(float s, float d) {
			return (2.0 * s - 1.0 > d) ? 2.0 * s - 1.0 : (s < 0.5 * d) ? 2.0 * s : d;
		}

		vec3 pinLight(vec3 s, vec3 d) {
			return vec3(pinLight(s.x, d.x), pinLight(s.y, d.y), pinLight(s.z, d.z));
		}

		vec3 applyBlend(vec3 source, vec3 destination, int mode) {
			if (mode == 1) {
				return softLight(source, destination);
			}
			if (mode == 2) {
				return hardLight(source, destination);
			}
			if (mode == 3) {
				return pinLight(source, destination);
			}
			if (mode == 4) {
				return linearLight(source, destination);
			}
			return vividLight(source, destination);
		}

		float vignette(vec2 uv) {
			uv *= 1.0 - uv.yx;
			float vig = uv.x * uv.y * uVignetteStrength;
			return clamp(pow(vig, uVignettePower), 0.0, 1.0);
		}

		void mainImage(out vec4 col, vec2 fragCoord) {
			vec2 uv = fragCoord / uResolution.y;
			float t = 0.5 * uTime * uWaveSpeed;
			uv.y += uWaveAmplitude * sin(uWaveFrequency * uv.x - t);

			float f = 0.6 + 0.4 * sin(5.0 * (uv.x + uv.y + cos(3.0 * uv.x + 5.0 * uv.y) + 0.02 * t) +
				sin(20.0 * (uv.x + uv.y - 0.1 * t)));

			vec3 base = uBaseColor * uBrightness * f;

			vec2 uvScreen = fragCoord / uResolution.xy;
			float vig = vignette(uvScreen);

			float fadeLR = 0.7 - abs(uvScreen.x - 0.4);
			float fadeTB = 1.1 - uvScreen.y;
			vec3 pos = vec3(uvScreen * vec2(3.0, 1.0) - vec2(0.0, uTime * 0.00005), uTime * 0.006);

			float simplex = snoise(pos * (uResolution.y / max(uNoiseScale, 0.001)));
			float n = fadeLR * fadeTB * smoothstep(0.5, 1.0, simplex) * uNoiseStrength;

			vec3 noiseGreyShifted = min((vec3(n) + 1.0) / 3.0 + 0.3, vec3(1.0)) * 0.91;
			vec3 blended = applyBlend(noiseGreyShifted, base, uBlendMode);
			vec3 mixed = mix(base, blended, uBlendStrength);

			float k = (sin(uTime) + 1.0) / 4.0 + 0.75;
			vec2 flowOffset = vec2(
				sin(uvScreen.x + uTime) * 10.0,
				cos(uvScreen.x * 10.0 + 0.01 * sin(uTime) + uTime) * 15.0 * (1.5 - uvScreen.y) * 0.4
			);

			vec3 shaded = mixed;
			shaded -= (flowOffset.y + flowOffset.x) * 0.01 * k * (1.0 - uvScreen.y) * uFlowShadingStrength;
			shaded -= vec3((1.0 - uvScreen.y) * 0.1 * k * uFlowShadingStrength, 0.0, 0.0);
			shaded -= vec3(uvScreen.y, uvScreen.y * 0.8, uvScreen.y) / 8.0 * uFlowShadingStrength;

			col = vec4(clamp(shaded, 0.0, 1.0) * vig, 1.0);
		}

		void main() {
			vec4 fragColor;
			vec2 fragCoord = vUv * uResolution.xy;
			mainImage(fragColor, fragCoord);
			gl_FragColor = fragColor;
			#include <colorspace_fragment>
		}
	`;

	$effect(() => {
		resolutionUniform.set($size.width, $size.height);
	});

	$effect(() => {
		baseColorUniform.set(baseColor);
		if (!material) return;

		material.uniforms.uBaseColor.value.copy(baseColorUniform);
		material.uniforms.uBrightness.value = brightness;
		material.uniforms.uWaveAmplitude.value = waveAmplitude;
		material.uniforms.uWaveFrequency.value = waveFrequency;
		material.uniforms.uWaveSpeed.value = waveSpeed;
		material.uniforms.uNoiseSizeCoeff.value = noiseSizeCoeff;
		material.uniforms.uNoiseDensity.value = noiseDensity;
		material.uniforms.uNoiseScale.value = noiseScale;
		material.uniforms.uNoiseStrength.value = noiseStrength;
		material.uniforms.uBlendMode.value = BLEND_MODE_MAP[blendMode];
		material.uniforms.uBlendStrength.value = blendStrength;
		material.uniforms.uVignetteStrength.value = vignetteStrength;
		material.uniforms.uVignettePower.value = vignettePower;
		material.uniforms.uFlowShadingStrength.value = flowShadingStrength;
	});

	useTask((delta) => {
		if (!material) return;
		material.uniforms.uTime.value += delta;
	});
</script>

<T.Mesh>
	<T.PlaneGeometry args={[2, 2]} />
	<T.ShaderMaterial
		bind:ref={material}
		{vertexShader}
		{fragmentShader}
		transparent
		depthTest={false}
		depthWrite={false}
		uniforms={{
			uTime: { value: 0.0 },
			uResolution: { value: resolutionUniform },
			uBaseColor: { value: baseColorUniform },
			uBrightness: { value: brightness },
			uWaveAmplitude: { value: waveAmplitude },
			uWaveFrequency: { value: waveFrequency },
			uWaveSpeed: { value: waveSpeed },
			uNoiseSizeCoeff: { value: noiseSizeCoeff },
			uNoiseDensity: { value: noiseDensity },
			uNoiseScale: { value: noiseScale },
			uNoiseStrength: { value: noiseStrength },
			uBlendMode: { value: BLEND_MODE_MAP[blendMode] },
			uBlendStrength: { value: blendStrength },
			uVignetteStrength: { value: vignetteStrength },
			uVignettePower: { value: vignettePower },
			uFlowShadingStrength: { value: flowShadingStrength },
		}}
	/>
</T.Mesh>
