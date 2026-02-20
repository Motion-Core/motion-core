<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		/**
		 * Primary color used to derive the full shader palette.
		 * @default "#FF6900"
		 */
		color?: THREE.ColorRepresentation;
		/**
		 * Speed multiplier for the full shader animation timeline.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Global intensity multiplier for the base silk color.
		 * @default 1.0
		 */
		brightness?: number;
		/**
		 * Opacity of the vivid-light glitter blend.
		 * @default 0.02
		 */
		blendStrength?: number;
		/**
		 * Spatial scale for simplex noise sampling.
		 * Lower values create finer glitter.
		 * @default 4.0
		 */
		noiseScale?: number;
		/**
		 * Base strength of vignette falloff.
		 * @default 15.0
		 */
		vignetteStrength?: number;
		/**
		 * Vignette curve exponent.
		 * Lower values produce a softer rolloff.
		 * @default 0.25
		 */
		vignettePower?: number;
		/**
		 * Opacity of the vignette effect.
		 * `0` disables vignette influence, `1` applies full vignette.
		 * @default 1.0
		 */
		vignetteOpacity?: number;
	}

	let {
		color = "#FF6900",
		speed = 1.0,
		brightness = 1.0,
		blendStrength = 0.02,
		noiseScale = 4.0,
		vignetteStrength = 15.0,
		vignettePower = 0.25,
		vignetteOpacity = 1.0,
	}: Props = $props();

	let material = $state<THREE.ShaderMaterial>();
	const { size } = useThrelte();
	const resolutionUniform = new THREE.Vector2(1, 1);
	const primaryColorUniform = new THREE.Color();
	const accentColorUniform = new THREE.Color();
	const shadowColorUniform = new THREE.Color();

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
		uniform vec3 uPrimaryColor;
		uniform vec3 uAccentColor;
		uniform vec3 uShadowColor;
		uniform float uSpeed;
		uniform float uBrightness;
		uniform float uBlendStrength;
		uniform float uNoiseScale;
		uniform float uVignetteStrength;
		uniform float uVignettePower;
		uniform float uVignetteOpacity;

		const float noiseSizeCoeff = 0.61;
		const float noiseDensity = 53.0;

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

			vec4 m = max(noiseSizeCoeff - vec4(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), 0.0);
			m = m * m;
			return noiseDensity * dot(m * m, vec4(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
		}

		float softLight(float s, float d) {
			return (s < 0.5)
				? d - (1.0 - 2.0 * s) * d * (1.0 - d)
				: (d < 0.25)
					? d + (2.0 * s - 1.0) * d * ((16.0 * d - 12.0) * d + 3.0)
					: d + (2.0 * s - 1.0) * (sqrt(d) - d);
		}

		vec3 softLight(vec3 s, vec3 d) {
			vec3 c;
			c.x = softLight(s.x, d.x);
			c.y = softLight(s.y, d.y);
			c.z = softLight(s.z, d.z);
			return c;
		}

		float hardLight(float s, float d) {
			return (s < 0.5) ? 2.0 * s * d : 1.0 - 2.0 * (1.0 - s) * (1.0 - d);
		}

		vec3 hardLight(vec3 s, vec3 d) {
			vec3 c;
			c.x = hardLight(s.x, d.x);
			c.y = hardLight(s.y, d.y);
			c.z = hardLight(s.z, d.z);
			return c;
		}

		float vividLight(float s, float d) {
			return (s < 0.5) ? 1.0 - (1.0 - d) / (2.0 * s) : d / (2.0 * (1.0 - s));
		}

		vec3 vividLight(vec3 s, vec3 d) {
			vec3 c;
			c.x = vividLight(s.x, d.x);
			c.y = vividLight(s.y, d.y);
			c.z = vividLight(s.z, d.z);
			return c;
		}

		vec3 linearLight(vec3 s, vec3 d) {
			return 2.0 * s + d - 1.0;
		}

		float pinLight(float s, float d) {
			return (2.0 * s - 1.0 > d) ? 2.0 * s - 1.0 : (s < 0.5 * d) ? 2.0 * s : d;
		}

		vec3 pinLight(vec3 s, vec3 d) {
			vec3 c;
			c.x = pinLight(s.x, d.x);
			c.y = pinLight(s.y, d.y);
			c.z = pinLight(s.z, d.z);
			return c;
		}

		float vignette(vec2 uv, float strength, float power) {
			uv *= 1.0 - uv.yx;
			float vig = uv.x * uv.y * strength;
			vig = pow(vig, power);
			return vig;
		}

		void mainImage(out vec4 col, vec2 fragCoord) {
			float time = uTime * uSpeed;
			vec2 uResolution = uResolution;

			vec2 uv = fragCoord / uResolution.y;
			float t = 0.5 * time;
			uv.y += 0.03 * sin(8.0 * uv.x - t);

			float f = 0.6 + 0.4 * sin(5.0 * (uv.x + uv.y + cos(3.0 * uv.x + 5.0 * uv.y) + 0.02 * t) +
				sin(20.0 * (uv.x + uv.y - 0.1 * t)));

			float b = uBrightness;
			col = vec4(uPrimaryColor * b, 1.0) * vec4(f);

			uv = fragCoord.xy / uResolution.xy;
			float vig = vignette(uv, uVignetteStrength, uVignettePower);
			float vignetteMask = mix(1.0, vig, uVignetteOpacity);

			float fadeLR = 0.7 - abs(uv.x - 0.4);
			float fadeTB = 1.1 - uv.y;
			vec3 pos = vec3(uv * vec2(3.0, 1.0) - vec2(0.0, time * 0.00005), time * 0.006);

			float n = fadeLR * fadeTB * smoothstep(0.50, 1.0, snoise(pos * uResolution.y / uNoiseScale)) * 8.0;
			vec3 noiseGreyShifted = min((vec3(n) + 1.0) / 3.0 + 0.3, vec3(1.0)) * 0.91;

			vec3 mixed = col.xyz;
			mixed = mix(col.xyz, vividLight(noiseGreyShifted, col.xyz), uBlendStrength);
			col = vec4(mixed, 1.0) * vignetteMask;

			float k = (sin(time / 1.0) + 1.0) / 4.0 + 0.75;


			col.rgb -= uAccentColor * ((1.0 - uv.y) * 0.1 * k);
			col.rgb -= vec3(uShadowColor.r, uShadowColor.g * 0.8, uShadowColor.b) * (uv.y / 8.0);
			col.a = 1.0;
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
		primaryColorUniform.set(color);
		const hsl = { h: 0, s: 0, l: 0 };
		primaryColorUniform.getHSL(hsl);

		accentColorUniform.setHSL(
			(hsl.h + 0.04) % 1,
			THREE.MathUtils.clamp(hsl.s * 1.1, 0, 1),
			THREE.MathUtils.clamp(hsl.l * 1.1 + 0.04, 0, 1),
		);
		shadowColorUniform.setHSL(
			hsl.h,
			THREE.MathUtils.clamp(hsl.s * 0.55, 0, 1),
			THREE.MathUtils.clamp(hsl.l * 0.45, 0, 1),
		);

		if (!material) return;
		material.uniforms.uPrimaryColor.value.copy(primaryColorUniform);
		material.uniforms.uAccentColor.value.copy(accentColorUniform);
		material.uniforms.uShadowColor.value.copy(shadowColorUniform);
		material.uniforms.uSpeed.value = speed;
		material.uniforms.uBrightness.value = brightness;
		material.uniforms.uBlendStrength.value = blendStrength;
		material.uniforms.uNoiseScale.value = noiseScale;
		material.uniforms.uVignetteStrength.value = vignetteStrength;
		material.uniforms.uVignettePower.value = vignettePower;
		material.uniforms.uVignetteOpacity.value = vignetteOpacity;
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
		depthTest={false}
		depthWrite={false}
		uniforms={{
			uTime: { value: 0.0 },
			uResolution: { value: resolutionUniform },
			uPrimaryColor: { value: primaryColorUniform },
			uAccentColor: { value: accentColorUniform },
			uShadowColor: { value: shadowColorUniform },
			uSpeed: { value: speed },
			uBrightness: { value: brightness },
			uBlendStrength: { value: blendStrength },
			uNoiseScale: { value: noiseScale },
			uVignetteStrength: { value: vignetteStrength },
			uVignettePower: { value: vignettePower },
			uVignetteOpacity: { value: vignetteOpacity },
		}}
	/>
</T.Mesh>
