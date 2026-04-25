<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Program,
		Renderer,
		Transform,
		Triangle,
		Vec2,
		Vec3,
	} from "ogl";
	import {
		clamp01,
		type ColorRepresentation,
		srgbToLinear,
		toRgb,
	} from "../../helpers/color";

	interface Props {
		/**
		 * Primary color used to derive the full shader palette.
		 * @default "#FF6900"
		 */
		color?: ColorRepresentation;
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

	type UniformState = {
		uTime: { value: number };
		uResolution: { value: Vec2 };
		uPrimaryColor: { value: Vec3 };
		uAccentColor: { value: Vec3 };
		uShadowColor: { value: Vec3 };
		uSpeed: { value: number };
		uBrightness: { value: number };
		uBlendStrength: { value: number };
		uNoiseScale: { value: number };
		uVignetteStrength: { value: number };
		uVignettePower: { value: number };
		uVignetteOpacity: { value: number };
	};

	const DEFAULT_PRIMARY: [number, number, number] = [1, 105 / 255, 0];

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state.raw<UniformState>();

	const toLinearTriplet = (
		value: [number, number, number],
	): [number, number, number] => [
		srgbToLinear(value[0]),
		srgbToLinear(value[1]),
		srgbToLinear(value[2]),
	];

	const rgbToHsl = (
		r: number,
		g: number,
		b: number,
	): { h: number; s: number; l: number } => {
		const max = Math.max(r, g, b);
		const min = Math.min(r, g, b);
		const l = (max + min) * 0.5;

		if (max === min) return { h: 0, s: 0, l };

		const d = max - min;
		const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
		let h = 0;

		switch (max) {
			case r:
				h = (g - b) / d + (g < b ? 6 : 0);
				break;
			case g:
				h = (b - r) / d + 2;
				break;
			default:
				h = (r - g) / d + 4;
				break;
		}

		h /= 6;
		return { h, s, l };
	};

	const hue2rgb = (p: number, q: number, tInput: number) => {
		let t = tInput;
		if (t < 0) t += 1;
		if (t > 1) t -= 1;
		if (t < 1 / 6) return p + (q - p) * 6 * t;
		if (t < 1 / 2) return q;
		if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
		return p;
	};

	const hslToRgb = (
		h: number,
		s: number,
		l: number,
	): [number, number, number] => {
		if (s === 0) return [l, l, l];
		const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
		const p = 2 * l - q;
		return [
			hue2rgb(p, q, h + 1 / 3),
			hue2rgb(p, q, h),
			hue2rgb(p, q, h - 1 / 3),
		];
	};

	const derivePalette = (
		primary: [number, number, number],
	): {
		accent: [number, number, number];
		shadow: [number, number, number];
	} => {
		const hsl = rgbToHsl(primary[0], primary[1], primary[2]);
		const accent = hslToRgb(
			(hsl.h + 0.04) % 1,
			clamp01(hsl.s * 1.1),
			clamp01(hsl.l * 1.1 + 0.04),
		);
		const shadow = hslToRgb(
			hsl.h,
			clamp01(hsl.s * 0.55),
			clamp01(hsl.l * 0.45),
		);
		return { accent, shadow };
	};

	const vertexShader = `
		attribute vec2 uv;
		attribute vec2 position;
		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = vec4(position, 0.0, 1.0);
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

		float vignette(vec2 uv, float strength, float power) {
			uv *= 1.0 - uv.yx;
			float vig = uv.x * uv.y * strength;
			vig = pow(vig, power);
			return vig;
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void mainImage(out vec4 col, vec2 fragCoord) {
			float time = uTime * uSpeed;
			vec2 resolution = uResolution;

			vec2 uv = fragCoord / resolution.y;
			float t = 0.5 * time;
			uv.y += 0.03 * sin(8.0 * uv.x - t);

			float f = 0.6 + 0.4 * sin(5.0 * (uv.x + uv.y + cos(3.0 * uv.x + 5.0 * uv.y) + 0.02 * t) +
				sin(20.0 * (uv.x + uv.y - 0.1 * t)));

			float b = uBrightness;
			col = vec4(uPrimaryColor * b, 1.0) * vec4(f);

			uv = fragCoord.xy / resolution.xy;
			float vig = vignette(uv, uVignetteStrength, uVignettePower);
			float vignetteMask = mix(1.0, vig, uVignetteOpacity);

			float fadeLR = 0.7 - abs(uv.x - 0.4);
			float fadeTB = 1.1 - uv.y;
			vec3 pos = vec3(uv * vec2(3.0, 1.0) - vec2(0.0, time * 0.00005), time * 0.006);

			float n = fadeLR * fadeTB * smoothstep(0.50, 1.0, snoise(pos * resolution.y / uNoiseScale)) * 8.0;
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
			fragColor.rgb = linearToSrgb(fragColor.rgb);
			gl_FragColor = fragColor;
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		const primarySrgb = toRgb(color, DEFAULT_PRIMARY);
		const paletteSrgb = derivePalette(primarySrgb);
		const primary = toLinearTriplet(primarySrgb);
		const accent = toLinearTriplet(paletteSrgb.accent);
		const shadow = toLinearTriplet(paletteSrgb.shadow);
		uniforms.uPrimaryColor.value.set(primary[0], primary[1], primary[2]);
		uniforms.uAccentColor.value.set(accent[0], accent[1], accent[2]);
		uniforms.uShadowColor.value.set(shadow[0], shadow[1], shadow[2]);
		uniforms.uSpeed.value = speed;
		uniforms.uBrightness.value = brightness;
		uniforms.uBlendStrength.value = blendStrength;
		uniforms.uNoiseScale.value = noiseScale;
		uniforms.uVignetteStrength.value = vignetteStrength;
		uniforms.uVignettePower.value = vignettePower;
		uniforms.uVignetteOpacity.value = vignetteOpacity;
	});

	onMount(() => {
		const targetCanvas = canvas;
		if (!targetCanvas) return;

		const renderer = new Renderer({
			canvas: targetCanvas,
			alpha: true,
			dpr: typeof window !== "undefined" ? window.devicePixelRatio : 1,
		});
		const gl = renderer.gl;
		gl.clearColor(0, 0, 0, 0);

		const camera = new Camera(gl);
		camera.position.z = 1;

		const scene = new Transform();
		const geometry = new Triangle(gl);

		const primarySrgb = toRgb(color, DEFAULT_PRIMARY);
		const paletteSrgb = derivePalette(primarySrgb);
		const primary = toLinearTriplet(primarySrgb);
		const accent = toLinearTriplet(paletteSrgb.accent);
		const shadow = toLinearTriplet(paletteSrgb.shadow);
		const localUniforms: UniformState = {
			uTime: { value: 0 },
			uResolution: { value: new Vec2(1, 1) },
			uPrimaryColor: { value: new Vec3(primary[0], primary[1], primary[2]) },
			uAccentColor: {
				value: new Vec3(accent[0], accent[1], accent[2]),
			},
			uShadowColor: {
				value: new Vec3(shadow[0], shadow[1], shadow[2]),
			},
			uSpeed: { value: speed },
			uBrightness: { value: brightness },
			uBlendStrength: { value: blendStrength },
			uNoiseScale: { value: noiseScale },
			uVignetteStrength: { value: vignetteStrength },
			uVignettePower: { value: vignettePower },
			uVignetteOpacity: { value: vignetteOpacity },
		};
		uniforms = localUniforms;

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: localUniforms,
			transparent: true,
			depthTest: false,
			depthWrite: false,
		});

		const mesh = new Mesh(gl, { geometry, program });
		mesh.setParent(scene);

		const resize = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));
			renderer.setSize(width, height);
			localUniforms.uResolution.value.set(width, height);
		};

		resize();
		const observer = new ResizeObserver(resize);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		let raf = 0;
		let previous = 0;
		const tick = (now: number) => {
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;
			localUniforms.uTime.value += delta;

			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			observer.disconnect();
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
