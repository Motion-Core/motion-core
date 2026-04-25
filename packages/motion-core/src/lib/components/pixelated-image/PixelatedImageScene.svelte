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
		 * Initial grid size for the pixelation effect.
		 * @default 6.0
		 */
		initialGridSize?: number;
		/**
		 * Duration of each step in the depixelation animation.
		 * @default 0.15
		 */
		stepDuration?: number;
	}

	let { image, initialGridSize = 6.0, stepDuration = 0.15 }: Props = $props();

	type UniformState = {
		uTexture: { value: Texture };
		uResolution: { value: Vec2 };
		uTextureSize: { value: Vec2 };
		uGridSize: { value: number };
		uIsDone: { value: number };
	};

	type RuntimeConfig = {
		initialGridSize: number;
		stepDuration: number;
	};

	let canvas = $state<HTMLCanvasElement>();
	let setImageSource = $state<(source: string) => void>();
	let setRuntimeConfig = $state<(config: RuntimeConfig) => void>();

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

		uniform sampler2D uTexture;
		uniform vec2 uResolution;
		uniform vec2 uTextureSize;
		uniform float uGridSize;
		uniform float uIsDone;
		varying vec2 vUv;

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 safeTexture = max(textureSize, vec2(1.0));
			vec2 s = uResolution / safeTexture;
			float scale = max(s.x, s.y);
			vec2 scaledSize = safeTexture * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		void main() {
			vec2 s = uResolution;
			float rs = s.x / max(s.y, 0.00001);

			vec2 grid = vec2(uGridSize * rs, uGridSize);
			vec2 pixelatedScreenUv = floor(vUv * grid) / grid + (0.5 / grid);

			vec2 finalUv = mix(pixelatedScreenUv, vUv, clamp(uIsDone, 0.0, 1.0));
			vec2 coverUv = getCoverUV(finalUv, uTextureSize);

			gl_FragColor = texture2D(uTexture, coverUv);
		}
	`;

	$effect(() => {
		if (!setImageSource) return;
		setImageSource(image);
	});

	$effect(() => {
		if (!setRuntimeConfig) return;
		setRuntimeConfig({ initialGridSize, stepDuration });
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

		const imageTexture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.NEAREST,
			magFilter: gl.NEAREST,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: false,
			flipY: true,
		});

		const resolutionUniform = new Vec2(1, 1);
		const textureSizeUniform = new Vec2(1, 1);
		const localUniforms: UniformState = {
			uTexture: { value: imageTexture },
			uResolution: { value: resolutionUniform },
			uTextureSize: { value: textureSizeUniform },
			uGridSize: { value: Math.max(1, initialGridSize) },
			uIsDone: { value: 0 },
		};

		let currentInitialGridSize = Math.max(1, initialGridSize);
		let currentStepDuration = Math.max(0.0001, stepDuration);
		let currentGridSize = currentInitialGridSize;
		let isDone = false;
		let elapsed = 0;

		const resetState = () => {
			currentGridSize = currentInitialGridSize;
			isDone = false;
			elapsed = 0;
			imageTexture.minFilter = gl.NEAREST;
			imageTexture.magFilter = gl.NEAREST;
			imageTexture.needsUpdate = true;
			localUniforms.uGridSize.value = currentGridSize;
			localUniforms.uIsDone.value = 0;
		};
		setRuntimeConfig = (config) => {
			const nextInitialGridSize = Math.max(1, config.initialGridSize);
			const nextStepDuration = Math.max(0.0001, config.stepDuration);
			const shouldReset =
				nextInitialGridSize !== currentInitialGridSize ||
				nextStepDuration !== currentStepDuration;

			currentInitialGridSize = nextInitialGridSize;
			currentStepDuration = nextStepDuration;

			if (shouldReset) {
				resetState();
			}
		};

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
				textureSizeUniform.set(
					img.naturalWidth || img.width || 1,
					img.naturalHeight || img.height || 1,
				);
				resetState();
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
				resolutionUniform.set(bufW, bufH);
			}
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;

			if (!isDone) {
				elapsed += delta;
				const step = Math.floor(elapsed / currentStepDuration);
				const nextGrid = Math.max(
					1,
					currentInitialGridSize * Math.pow(2, step),
				);
				currentGridSize = nextGrid;

				if (currentGridSize > resolutionUniform.y) {
					isDone = true;
					imageTexture.minFilter = gl.LINEAR;
					imageTexture.magFilter = gl.LINEAR;
					imageTexture.needsUpdate = true;
				}
			}

			localUniforms.uGridSize.value = currentGridSize;
			localUniforms.uIsDone.value = isDone ? 1 : 0;

			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			setImageSource = undefined;
			setRuntimeConfig = undefined;
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
