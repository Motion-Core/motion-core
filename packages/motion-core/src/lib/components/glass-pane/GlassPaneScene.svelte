<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Program,
		Renderer,
		Texture,
		Transform,
		Triangle,
		Vec2,
	} from "ogl";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Strength of the refraction/distortion effect.
		 * @default 1.0
		 */
		distortion?: number;
		/**
		 * Amount of chromatic aberration (color splitting).
		 * @default 0.005
		 */
		chromaticAberration?: number;
		/**
		 * Speed of the wave animation.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Amplitude of the wave distortion.
		 * @default 0.05
		 */
		waviness?: number;
		/**
		 * Frequency of the wave distortion.
		 * @default 6.0
		 */
		frequency?: number;
		/**
		 * Density of the glass rods.
		 * @default 5.0
		 */
		rods?: number;
	}

	let {
		image,
		distortion = 1.0,
		chromaticAberration = 0.005,
		speed = 1.0,
		waviness = 0.05,
		frequency = 6.0,
		rods = 5.0,
	}: Props = $props();

	type UniformState = {
		uTime: { value: number };
		uResolution: { value: Vec2 };
		uTextureSize: { value: Vec2 };
		uTexture: { value: Texture };
		uDistortion: { value: number };
		uChromaticAberration: { value: number };
		uWaviness: { value: number };
		uFrequency: { value: number };
		uRods: { value: number };
	};

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<UniformState>();
	let setImageSource = $state<(source: string) => void>();

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

		uniform float uTime;
		uniform vec2 uResolution;
		uniform vec2 uTextureSize;
		uniform sampler2D uTexture;
		uniform float uDistortion;
		uniform float uChromaticAberration;
		uniform float uWaviness;
		uniform float uFrequency;
		uniform float uRods;
		varying vec2 vUv;

		vec2 mirrored(vec2 value) {
			vec2 m = mod(value, 2.0);
			return mix(m, 2.0 - m, step(1.0, m));
		}

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 safeTexture = max(textureSize, vec2(1.0));
			vec2 s = uResolution / safeTexture;
			float scale = max(s.x, s.y);
			vec2 scaledSize = safeTexture * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		void main() {
			vec2 p = (vUv * 2.0 - 1.0);
			p.x *= uResolution.x / uResolution.y;

			float angle = radians(45.0);
			mat2 rot = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
			vec2 p_rot = rot * p;

			float wave = uWaviness * sin(p_rot.y * uFrequency);
			float rod_x = fract((p_rot.x + wave) * uRods) * 2.0 - 1.0;

			float rod_z_sq = 1.0 - rod_x * rod_x;
			float rod_z = sqrt(max(rod_z_sq, 0.0));

			vec3 n = vec3(rod_x, 0.0, -rod_z);
			vec3 rd = vec3(0.0, 0.0, -1.0);
			float refractive_index = 0.6;
			vec3 refracted_ray = mix(n, rd, refractive_index);

			float z_dist = 0.5 / (abs(refracted_ray.z) + 0.001);
			vec3 hit_pos = vec3(p_rot, 0.0) + (z_dist * uDistortion) * refracted_ray;

			mat2 rot_inv = mat2(cos(-angle), -sin(-angle), sin(-angle), cos(-angle));
			vec2 uv_hit = rot_inv * hit_pos.xy;

			uv_hit.x /= (uResolution.x / uResolution.y);
			uv_hit = uv_hit * 0.5 + 0.5;

			vec2 coverUv = getCoverUV(uv_hit, uTextureSize);

			float t = uTime * 0.1;
			vec2 flow = vec2(sin(t), cos(t * 0.8)) * 0.05;
			float dispersion = uChromaticAberration;

			vec2 coverUvFlow = mirrored(coverUv + flow);
			float r = texture2D(uTexture, mirrored(coverUvFlow + vec2(dispersion, 0.0))).r;
			float g = texture2D(uTexture, coverUvFlow).g;
			float b = texture2D(uTexture, mirrored(coverUvFlow - vec2(dispersion, 0.0))).b;

			float g_factor = 1.0 - abs(n.z);
			g_factor = smoothstep(0.0, 1.0, g_factor);
			float glass = g_factor * 0.0025;

			vec3 finalColor = vec3(r, g, b) + glass;
			gl_FragColor = vec4(finalColor, 1.0);
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		uniforms.uDistortion.value = distortion;
		uniforms.uChromaticAberration.value = chromaticAberration;
		uniforms.uWaviness.value = waviness;
		uniforms.uFrequency.value = frequency;
		uniforms.uRods.value = rods;
	});

	$effect(() => {
		if (!setImageSource) return;
		setImageSource(image);
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

		const imageTexture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: false,
			flipY: true,
		});

		const localUniforms: UniformState = {
			uTime: { value: 0 },
			uResolution: { value: new Vec2(1, 1) },
			uTextureSize: { value: new Vec2(1, 1) },
			uTexture: { value: imageTexture },
			uDistortion: { value: distortion },
			uChromaticAberration: { value: chromaticAberration },
			uWaviness: { value: waviness },
			uFrequency: { value: frequency },
			uRods: { value: rods },
		};
		uniforms = localUniforms;

		let imageToken = 0;
		const loadImage = (source: string) => {
			imageToken += 1;
			const token = imageToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (token !== imageToken) return;
				imageTexture.image = img;
				localUniforms.uTextureSize.value.set(
					img.naturalWidth || img.width || 1,
					img.naturalHeight || img.height || 1,
				);
			};
			img.src = source;
		};
		setImageSource = loadImage;

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
			localUniforms.uResolution.value.set(gl.canvas.width, gl.canvas.height);
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
			localUniforms.uTime.value += delta * speed;

			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			setImageSource = undefined;
			if (imageTexture.texture) {
				gl.deleteTexture(imageTexture.texture);
			}
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
