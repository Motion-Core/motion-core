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
		 * Grid resolution (number of cells per row/column).
		 */
		grid: number;
		/**
		 * Radius of mouse influence.
		 */
		mouseSize: number;
		/**
		 * Strength of the distortion effect.
		 */
		strength: number;
		/**
		 * Relaxation factor for returning to original state (0-1).
		 */
		relaxation: number;
		/**
		 * Current normalized mouse X position.
		 */
		mouseX: number;
		/**
		 * Current normalized mouse Y position.
		 */
		mouseY: number;
	}

	let { image, grid, mouseSize, strength, relaxation, mouseX, mouseY }: Props =
		$props();

	type GridState = {
		size: number;
		data: Float32Array;
		texture: Texture;
	};

	type UniformState = {
		time: { value: number };
		uResolution: { value: Vec2 };
		uTextureSize: { value: Vec2 };
		uDataTexture: { value: Texture };
		uTexture: { value: Texture };
	};

	let canvas = $state<HTMLCanvasElement>();
	let setImageSource = $state<(source: string) => void>();
	let setGridSize = $state<(value: number) => void>();

	let currentVX = 0;
	let currentVY = 0;
	let prevX = 0;
	let prevY = 0;

	const normalizeGridSize = (value: number) => Math.max(1, Math.round(value));

	const createGridState = (gl: Renderer["gl"], gridSize: number): GridState => {
		const size = normalizeGridSize(gridSize);
		const total = size * size;
		const data = new Float32Array(4 * total);

		for (let i = 0; i < total; i++) {
			const r = Math.random() * 255 - 125;
			const r1 = Math.random() * 255 - 125;
			const stride = i * 4;
			data[stride] = r;
			data[stride + 1] = r1;
			data[stride + 2] = r;
			data[stride + 3] = 255;
		}

		const internalFormat = gl.renderer.isWebgl2
			? (gl as WebGL2RenderingContext).RGBA32F
			: gl.RGBA;

		const texture = new Texture(gl, {
			image: data,
			width: size,
			height: size,
			format: gl.RGBA,
			internalFormat,
			type: gl.FLOAT,
			minFilter: gl.NEAREST,
			magFilter: gl.NEAREST,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: false,
			flipY: false,
		});

		return { size, data, texture };
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

		uniform float time;
		uniform vec2 uResolution;
		uniform vec2 uTextureSize;
		uniform sampler2D uDataTexture;
		uniform sampler2D uTexture;
		varying vec2 vUv;

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 s = uResolution / textureSize;
			float scale = max(s.x, s.y);
			vec2 scaledSize = textureSize * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		void main() {
			vec2 coverUv = getCoverUV(vUv, uTextureSize);
			vec4 data = texture2D(uDataTexture, vUv);
			vec2 displacedUV = coverUv - 0.02 * data.rg;
			vec4 color = texture2D(uTexture, displacedUV);
			gl_FragColor = color;
		}
	`;

	$effect(() => {
		currentVX = mouseX - prevX;
		currentVY = mouseY - prevY;
		prevX = mouseX;
		prevY = mouseY;
	});

	$effect(() => {
		if (!setImageSource) return;
		setImageSource(image);
	});

	$effect(() => {
		if (!setGridSize) return;
		setGridSize(grid);
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
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: false,
			flipY: true,
		});

		let localUniforms: UniformState;
		let imageLoadToken = 0;
		const loadImage = (source: string) => {
			imageLoadToken += 1;
			const token = imageLoadToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (token !== imageLoadToken) return;
				imageTexture.image = img;
				localUniforms.uTextureSize.value.set(
					img.naturalWidth || img.width,
					img.naturalHeight || img.height,
				);
			};
			img.src = source;
		};

		let gridState = createGridState(gl, grid);
		const replaceGrid = (value: number) => {
			const nextSize = normalizeGridSize(value);
			if (nextSize === gridState.size) return;
			const previousTexture = gridState.texture;
			gridState = createGridState(gl, nextSize);
			localUniforms.uDataTexture.value = gridState.texture;
			if (previousTexture.texture) {
				gl.deleteTexture(previousTexture.texture);
			}
		};

		localUniforms = {
			time: { value: 0 },
			uResolution: { value: new Vec2(1, 1) },
			uTextureSize: { value: new Vec2(1, 1) },
			uDataTexture: { value: gridState.texture },
			uTexture: { value: imageTexture },
		};
		setImageSource = loadImage;
		setGridSize = replaceGrid;

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
				localUniforms.uResolution.value.set(w, h);
			}
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;
			localUniforms.time.value += delta;

			const gridSize = gridState.size;
			const data = gridState.data;
			const gridMouseX = gridSize * mouseX;
			const gridMouseY = gridSize * (1 - mouseY);
			const maxDist = gridSize * mouseSize;
			const width = localUniforms.uResolution.value.x;
			const height = localUniforms.uResolution.value.y;
			const aspect = width > 0 ? height / width : 1;
			const maxDistSq = maxDist * maxDist;

			for (let i = 0; i < gridSize; i++) {
				for (let j = 0; j < gridSize; j++) {
					const distance =
						((gridMouseX - i) * (gridMouseX - i)) / aspect +
						(gridMouseY - j) * (gridMouseY - j);

					if (distance < maxDistSq) {
						const index = 4 * (i + gridSize * j);
						let power = maxDist / Math.sqrt(distance);
						if (!Number.isFinite(power) || power > 10) power = 10;

						data[index] += strength * 100 * currentVX * power;
						data[index + 1] -= strength * 100 * currentVY * power;
					}

					const idx = 4 * (i + gridSize * j);
					data[idx] *= relaxation;
					data[idx + 1] *= relaxation;
				}
			}

			currentVX *= 0.9;
			currentVY *= 0.9;
			gridState.texture.needsUpdate = true;

			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			setImageSource = undefined;
			setGridSize = undefined;
			if (gridState.texture.texture) {
				gl.deleteTexture(gridState.texture.texture);
			}
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
