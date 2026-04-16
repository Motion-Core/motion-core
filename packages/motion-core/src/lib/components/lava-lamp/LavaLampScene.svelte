<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Program,
		Renderer,
		Transform,
		Triangle,
		Vec3,
		Vec4,
	} from "ogl";
	import { toLinearRgb } from "../../helpers/color";

	interface Props {
		/**
		 * Base color of the lava blobs.
		 * @default "#17181A"
		 */
		color?: string;
		/**
		 * Color of the fresnel effect.
		 * @default "#ff6900"
		 */
		fresnelColor?: string;
		/**
		 * Speed of the lava animation.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Fresnel power for the edge lighting effect.
		 * @default 3.0
		 */
		fresnelPower?: number;
		/**
		 * Base radius of the blobs.
		 * @default 1
		 */
		radius?: number;
		/**
		 * Smoothness of the blob blending (metaball effect).
		 * @default 0.1
		 */
		smoothness?: number;
	}

	let {
		color = "#17181A",
		fresnelColor = "#ff6900",
		speed = 1.0,
		fresnelPower = 3.0,
		radius = 1,
		smoothness = 0.1,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<{
		uTime: { value: number };
		uResolution: { value: Vec4 };
		uColor: { value: Vec3 };
		uFresnelColor: { value: Vec3 };
		uFresnelPower: { value: number };
		uRadius: { value: number };
		uSmoothness: { value: number };
	}>();

	const applyColor = (
		target: Vec3,
		value: string,
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
		uniform vec4 uResolution;
		uniform vec3 uColor;
		uniform vec3 uFresnelColor;
		uniform float uFresnelPower;
		uniform float uRadius;
		uniform float uSmoothness;

		float PI = 3.141592653589793238;

		mat4 rotationMatrix(vec3 axis, float angle) {
			axis = normalize(axis);
			float s = sin(angle);
			float c = cos(angle);
			float oc = 1.0 - c;
			return mat4(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
						oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
						oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
						0.0,                                0.0,                                0.0,                                1.0);
		}

		vec3 rotate(vec3 v, vec3 axis, float angle) {
			mat4 m = rotationMatrix(axis, angle);
			return (m * vec4(v, 1.0)).xyz;
		}

		float smin(float a, float b, float k) {
			k *= 6.0;
			float h = max(k-abs(a-b), 0.0)/k;
			return min(a,b) - h*h*h*k*(1.0/6.0);
		}

		float sphereSDF(vec3 p, float r) {
			return length(p) - r;
		}

		float sdf(vec3 p) {
			vec3 p1 = rotate(p, vec3(0.0, 0.0, 1.0), uTime/5.0);
			vec3 p2 = rotate(p, vec3(1.), -uTime/5.0);
			vec3 p3 = rotate(p, vec3(1., 1., 0.), -uTime/4.5);
			vec3 p4 = rotate(p, vec3(0., 1., 0.), -uTime/4.0);

			float r = uRadius;

			float final = sphereSDF(p1 - vec3(-0.5, 0.0, 0.0), 0.35 * r);
			float nextSphere = sphereSDF(p2 - vec3(0.55, 0.0, 0.0), 0.3 * r);
			final = smin(final, nextSphere, uSmoothness);
			nextSphere = sphereSDF(p2 - vec3(-0.8, 0.0, 0.0), 0.2 * r);
			final = smin(final, nextSphere, uSmoothness);
			nextSphere = sphereSDF(p3 - vec3(1.0, 0.0, 0.0), 0.15 * r);
			final = smin(final, nextSphere, uSmoothness);
			nextSphere = sphereSDF(p4 - vec3(0.45, -0.45, 0.0), 0.15 * r);
			final = smin(final, nextSphere, uSmoothness);

			return final;
		}

		vec3 getNormal(vec3 p) {
			float d = 0.001;
			return normalize(vec3(
				sdf(p + vec3(d, 0.0, 0.0)) - sdf(p - vec3(d, 0.0, 0.0)),
				sdf(p + vec3(0.0, d, 0.0)) - sdf(p - vec3(0.0, d, 0.0)),
				sdf(p + vec3(0.0, 0.0, d)) - sdf(p - vec3(0.0, 0.0, d))
			));
		}

		float rayMarch(vec3 rayOrigin, vec3 ray) {
			float t = 0.0;
			for (int i = 0; i < 100; i++) {
				vec3 p = rayOrigin + ray * t;
				float d = sdf(p);
				if (d < 0.001) return t;
				t += d;
				if (t > 100.0) break;
			}
			return -1.0;
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void main() {
			vec3 cameraPos = vec3(0.0, 0.0, 5.0);
			vec3 ray = normalize(vec3((vUv - vec2(0.5)) * uResolution.zw, -1));

			float t = rayMarch(cameraPos, ray);
			if (t > 0.0) {
				vec3 p = cameraPos + ray * t;
				vec3 normal = getNormal(p);
				float fresnel = pow(1.0 + dot(ray, normal), uFresnelPower);
				vec3 color = mix(uColor, uFresnelColor, fresnel);
				gl_FragColor = vec4(linearToSrgb(color), 1.0);
			} else {
				gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
			}
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		applyColor(uniforms.uColor.value, color, [24 / 255, 24 / 255, 27 / 255]);
		applyColor(uniforms.uFresnelColor.value, fresnelColor, [1, 105 / 255, 0]);
		uniforms.uFresnelPower.value = fresnelPower;
		uniforms.uRadius.value = radius;
		uniforms.uSmoothness.value = smoothness;
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

		const initialColor = toLinearRgb(color, [24 / 255, 24 / 255, 27 / 255]);
		const initialFresnelColor = toLinearRgb(fresnelColor, [1, 105 / 255, 0]);
		const localUniforms = {
			uTime: { value: 0 },
			uResolution: { value: new Vec4(1, 1, 1, 1) },
			uColor: {
				value: new Vec3(initialColor[0], initialColor[1], initialColor[2]),
			},
			uFresnelColor: {
				value: new Vec3(
					initialFresnelColor[0],
					initialFresnelColor[1],
					initialFresnelColor[2],
				),
			},
			uFresnelPower: { value: fresnelPower },
			uRadius: { value: radius },
			uSmoothness: { value: smoothness },
		};

		uniforms = localUniforms;

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: localUniforms,
			transparent: true,
		});

		const mesh = new Mesh(gl, { geometry, program });
		mesh.setParent(scene);

		const updateResolution = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));

			const imageAspect = 1;
			let a1 = 1;
			let a2 = 1;
			if (height / width > imageAspect) {
				a1 = (width / height) * imageAspect;
				a2 = 1;
			} else {
				a1 = 1;
				a2 = height / width / imageAspect;
			}

			renderer.setSize(width, height);
			localUniforms.uResolution.value.set(width, height, a1, a2);
		};

		updateResolution();
		const observer = new ResizeObserver(updateResolution);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		let raf = 0;
		let previous = 0;
		let time = 0;
		const tick = (now: number) => {
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;
			time += delta * speed;
			localUniforms.uTime.value = time;

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
