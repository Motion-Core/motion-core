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
		 * Camera rotation speed multiplier.
		 * @default 0.5
		 */
		rotationSpeed?: number;
		/**
		 * Color of the background.
		 * @default "#17181A"
		 */
		backgroundColor?: ColorRepresentation;
		/**
		 * Distance of the camera from the center.
		 * @default 3.0
		 */
		cameraDistance?: number;
		/**
		 * Field of View (FOV) of the camera in degrees.
		 * @default 55.0
		 */
		fov?: number;
		/**
		 * Sun light direction vector (X).
		 * @default 0.0
		 */
		sunX?: number;
		/**
		 * Sun light direction vector (Y).
		 * @default 0.0
		 */
		sunY?: number;
		/**
		 * Sun light direction vector (Z).
		 * @default 1.0
		 */
		sunZ?: number;
		/**
		 * Overall intensity/brightness of the scattering effect.
		 * @default 1.0
		 */
		intensity?: number;
	}

	let {
		rotationSpeed = 0.5,
		backgroundColor = "#17181A",
		cameraDistance = 3.0,
		fov = 55.0,
		sunX = 0.0,
		sunY = 0.0,
		sunZ = 1.0,
		intensity = 1.0,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state.raw<{
		uTime: { value: number };
		uResolution: { value: Vec2 };
		uBackgroundColor: { value: Vec3 };
		uRotationSpeed: { value: number };
		uCameraDistance: { value: number };
		uFov: { value: number };
		uSunDir: { value: Vec3 };
		uIntensity: { value: number };
	}>();

	const setColorUniform = (
		target: Vec3,
		value: ColorRepresentation,
		fallback: [number, number, number],
	) => {
		const [r, g, b] = toLinearRgb(value, fallback);
		target.set(r, g, b);
	};

	const setSunDirection = (target: Vec3, x: number, y: number, z: number) => {
		const len = Math.hypot(x, y, z);
		if (len < 1e-6) {
			target.set(0, 0, 1);
			return;
		}
		target.set(x / len, y / len, z / len);
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
		uniform vec3 uBackgroundColor;
		uniform float uRotationSpeed;
		uniform float uCameraDistance;
		uniform float uFov;
		uniform vec3 uSunDir;
		uniform float uIntensity;

		const float PI = 3.14159265359;
		const float MAX = 10000.0;
		const float R_INNER = 1.0;
		const float R = 1.5;
		const int NUM_OUT_SCATTER = 8;
		const int NUM_IN_SCATTER = 40;

		vec2 ray_vs_sphere(vec3 p, vec3 dir, float r) {
			float b = dot(p, dir);
			float c = dot(p, p) - r * r;
			float d = b * b - c;
			if (d < 0.0) {
				return vec2(MAX, -MAX);
			}
			d = sqrt(d);
			return vec2(-b - d, -b + d);
		}

		float phase_mie(float g, float c, float cc) {
			float gg = g * g;
			float a = (1.0 - gg) * (1.0 + cc);
			float b = 1.0 + gg - 2.0 * g * c;
			b *= sqrt(b);
			b *= 2.0 + gg;
			return (3.0 / 8.0 / PI) * a / b;
		}

		float phase_ray(float cc) {
			return (3.0 / 16.0 / PI) * (1.0 + cc);
		}

		float density(vec3 p, float ph) {
			return exp(-max(length(p) - R_INNER, 0.0) / ph);
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
			vec3 tintTarget = mix(bg, effectHue, 0.85);
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

		float optic(vec3 p, vec3 q, float ph) {
			vec3 s = (q - p) / float(NUM_OUT_SCATTER);
			vec3 v = p + s * 0.5;
			float sum = 0.0;
			for (int i = 0; i < NUM_OUT_SCATTER; i++) {
				sum += density(v, ph);
				v += s;
			}
			sum *= length(s);
			return sum;
		}

		vec3 in_scatter(vec3 o, vec3 dir, vec2 e, vec3 l) {
			const float ph_ray = 0.05;
			const float ph_mie = 0.02;
			const vec3 k_ray = vec3(3.8, 13.5, 33.1);
			const vec3 k_mie = vec3(21.0);
			const float k_mie_ex = 1.1;

			vec3 sum_ray = vec3(0.0);
			vec3 sum_mie = vec3(0.0);
			float n_ray0 = 0.0;
			float n_mie0 = 0.0;
			float len = (e.y - e.x) / float(NUM_IN_SCATTER);
			vec3 s = dir * len;
			vec3 v = o + dir * (e.x + len * 0.5);

			for (int i = 0; i < NUM_IN_SCATTER; i++) {
				float d_ray = density(v, ph_ray) * len;
				float d_mie = density(v, ph_mie) * len;
				n_ray0 += d_ray;
				n_mie0 += d_mie;

				vec2 f = ray_vs_sphere(v, l, R);
				vec3 u = v + l * f.y;
				float n_ray1 = optic(v, u, ph_ray);
				float n_mie1 = optic(v, u, ph_mie);
				vec3 att = exp(-(n_ray0 + n_ray1) * k_ray - (n_mie0 + n_mie1) * k_mie * k_mie_ex);
				sum_ray += d_ray * att;
				sum_mie += d_mie * att;
				v += s;
			}
			float c = dot(dir, -l);
			float cc = c * c;
			vec3 scatter = sum_ray * k_ray * phase_ray(cc) + sum_mie * k_mie * phase_mie(-0.78, c, cc);
			return scatter;
		}

		mat3 rot3xy(vec2 angle) {
			vec2 c = cos(angle);
			vec2 s = sin(angle);
			return mat3(
				c.y, 0.0, -s.y,
				s.y * s.x, c.x, c.y * s.x,
				s.y * c.x, -s.x, c.y * c.x
			);
		}

		vec3 ray_dir(float fov, vec2 size, vec2 uv) {
			vec2 xy = uv * size - size * 0.5;
			float cot_half_fov = tan(radians(90.0 - fov * 0.5));
			float z = size.y * 0.5 * cot_half_fov;
			return normalize(vec3(xy, -z));
		}

		void mainImage(out vec4 fragColor, in vec2 uv) {
			vec3 dir = ray_dir(uFov, uResolution.xy, uv);
			vec3 eye = vec3(0.0, 0.0, uCameraDistance);
			mat3 rot = rot3xy(vec2(0.0, uTime * uRotationSpeed));
			dir = rot * dir;
			eye = rot * eye;
			vec3 l = normalize(uSunDir);
			vec2 e = ray_vs_sphere(eye, dir, R);
			if (e.x > e.y) {
				fragColor = vec4(uBackgroundColor, 1.0);
				return;
			}
			vec2 f = ray_vs_sphere(eye, dir, R_INNER);
			e.y = min(e.y, f.x);
			vec3 I = in_scatter(eye, dir, e, l);
			vec3 halo = I * uIntensity * 10.0;
			float softMask = 1.0 - exp(-1.2 * colorLuma(halo));
			vec3 rgb = blendAdaptive(uBackgroundColor, halo, softMask);
			fragColor = vec4(rgb, 1.0);
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
		setColorUniform(
			uniforms.uBackgroundColor.value,
			backgroundColor,
			[0, 0, 0],
		);
		uniforms.uRotationSpeed.value = rotationSpeed;
		uniforms.uCameraDistance.value = cameraDistance;
		uniforms.uFov.value = fov;
		setSunDirection(uniforms.uSunDir.value, sunX, sunY, sunZ);
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

		targetCanvas.style.width = "100%";
		targetCanvas.style.height = "100%";

		const camera = new Camera(gl);
		camera.position.z = 1;

		const scene = new Transform();
		const geometry = new Triangle(gl);

		const initialBackground = toLinearRgb(backgroundColor, [
			23 / 255,
			24 / 255,
			26 / 255,
		]);
		const initialSun = new Vec3(0, 0, 1);
		setSunDirection(initialSun, sunX, sunY, sunZ);

		const localUniforms = {
			uTime: { value: 0.0 },
			uResolution: { value: new Vec2(1, 1) },
			uBackgroundColor: {
				value: new Vec3(
					initialBackground[0],
					initialBackground[1],
					initialBackground[2],
				),
			},
			uRotationSpeed: { value: rotationSpeed },
			uCameraDistance: { value: cameraDistance },
			uFov: { value: fov },
			uSunDir: { value: initialSun },
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

		let raf = 0;
		let previous = 0;
		const tick = (now: number) => {
			const w = Math.max(1, targetCanvas.clientWidth);
			const h = Math.max(1, targetCanvas.clientHeight);
			const bufW = Math.round(w * renderer.dpr);
			const bufH = Math.round(h * renderer.dpr);
			if (targetCanvas.width !== bufW || targetCanvas.height !== bufH) {
				targetCanvas.width = bufW;
				targetCanvas.height = bufH;
				renderer.width = w;
				renderer.height = h;
				renderer.state.viewport = { x: 0, y: 0, width: null, height: null };
				localUniforms.uResolution.value.set(w, h);
			}
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;
			localUniforms.uTime.value += delta;

			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
