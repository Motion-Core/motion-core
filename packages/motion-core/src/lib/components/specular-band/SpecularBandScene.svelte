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
	import { type ColorRepresentation, toLinearRgb } from "../../helpers/color";

	interface Props {
		/**
		 * Base color of the specular bands.
		 * @default "#FF6900"
		 */
		color?: ColorRepresentation;
		/**
		 * Color of the background.
		 * @default "#17181A"
		 */
		backgroundColor?: ColorRepresentation;
		/**
		 * Animation speed multiplier.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Lens distortion intensity.
		 * @default 0.2
		 */
		distortion?: number;
		/**
		 * Amount of hue shift for secondary bands (in degrees).
		 * @default 30.0
		 */
		hueShift?: number;
		/**
		 * Global intensity/brightness of the effect.
		 * @default 1.0
		 */
		intensity?: number;
	}

	let {
		color = "#FF6900",
		backgroundColor = "#17181A",
		speed = 1.0,
		distortion = 0.2,
		hueShift = 30.0,
		intensity = 1.0,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<{
		uTime: { value: number };
		uResolution: { value: Vec2 };
		uColor: { value: Vec3 };
		uBackgroundColor: { value: Vec3 };
		uSpeed: { value: number };
		uDistortion: { value: number };
		uHueShift: { value: number };
		uIntensity: { value: number };
	}>();

	const applyColor = (
		target: Vec3,
		value: ColorRepresentation,
		fallback: [number, number, number],
	) => {
		const [r, g, b] = toLinearRgb(value, fallback);
		target.set(r, g, b);
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
		uniform vec3 uColor;
		uniform vec3 uBackgroundColor;
		uniform float uSpeed;
		uniform float uDistortion;
		uniform float uHueShift;
		uniform float uIntensity;

		mat3 hueRot(float a) {
			float c = cos(a), s = sin(a), t = 1.0 - c;
			return mat3(
			t*.333+c,    t*.333-s*.577, t*.333+s*.577,
			t*.333+s*.577, t*.333+c,   t*.333-s*.577,
			t*.333-s*.577, t*.333+s*.577, t*.333+c
			);
		}

		float colorLuma(vec3 c) {
			return dot(c, vec3(0.2126, 0.7152, 0.0722));
		}

		vec3 hueFromColor(vec3 c, vec3 fallback) {
			float m = max(max(c.r, c.g), c.b);
			if (m < 1e-5) return fallback;
			return clamp(c / m, 0.0, 1.0);
		}

		vec3 blendAdaptive(vec3 bg, vec3 effect, float softness) {
			float bgLum = colorLuma(bg);
			float lightBg = smoothstep(0.45, 0.95, bgLum);
			float edge = clamp(softness, 0.0, 1.0);

			vec3 additive = bg + effect;
			vec3 effectHue = hueFromColor(effect, vec3(1.0));
			vec3 tintTarget = mix(bg, effectHue, 0.9);
			vec3 tint = mix(bg, tintTarget, edge);

			return mix(additive, tint, lightBg);
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void mainImage(out vec4 o, vec2 uv) {
			vec2 u = (uv * 2.0 - 1.0);
			u.x *= uResolution.x / uResolution.y;

			float time = uTime * uSpeed;

			u /= 0.5 + uDistortion * dot(u, u);
			u += 0.2 * cos(time) - 7.56;

			vec3 baseColor = uColor;

			vec3 palette[3];
			palette[0] = baseColor;
			palette[1] = hueRot(radians(uHueShift)) * baseColor;
			palette[2] = hueRot(radians(-uHueShift)) * baseColor;

			vec3 col = vec3(0.0);
			float edgeField = 0.0;
			for(int i = 0; i < 3; i++) {
				vec2 uv_loop = sin(1.5 * u.yx + 2.0 * cos(u -= 0.01));
				float val = 1.0 - exp(-6.0 / exp(6.0 * length(uv_loop + sin(5.0 * uv_loop.y - 3.0 * time) / 4.0)));
				val = pow(clamp(val, 0.0, 1.0), 1.4);
				edgeField += val;
				col += val * palette[i];
			}
			vec3 bands = col * uIntensity;
			float softMask = 1.0 - exp(-0.85 * edgeField * uIntensity);
			vec3 rgb = blendAdaptive(uBackgroundColor, bands, softMask);
			o = vec4(rgb, 1.0);
		}

		void main() {
			vec4 fragColor;
			mainImage(fragColor, vUv);
			fragColor.rgb = linearToSrgb(fragColor.rgb);
			gl_FragColor = fragColor;
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		applyColor(uniforms.uColor.value, color, [1, 105 / 255, 0]);
		applyColor(uniforms.uBackgroundColor.value, backgroundColor, [
			23 / 255,
			24 / 255,
			26 / 255,
		]);
		uniforms.uSpeed.value = speed;
		uniforms.uDistortion.value = distortion;
		uniforms.uHueShift.value = hueShift;
		uniforms.uIntensity.value = intensity;
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

		const initialColor = toLinearRgb(color, [1, 105 / 255, 0]);
		const initialBackgroundColor = toLinearRgb(backgroundColor, [
			23 / 255,
			24 / 255,
			26 / 255,
		]);
		const localUniforms = {
			uTime: { value: 0 },
			uResolution: { value: new Vec2(1, 1) },
			uColor: {
				value: new Vec3(initialColor[0], initialColor[1], initialColor[2]),
			},
			uBackgroundColor: {
				value: new Vec3(
					initialBackgroundColor[0],
					initialBackgroundColor[1],
					initialBackgroundColor[2],
				),
			},
			uSpeed: { value: speed },
			uDistortion: { value: distortion },
			uHueShift: { value: hueShift },
			uIntensity: { value: intensity },
		};

		uniforms = localUniforms;

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: localUniforms,
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
