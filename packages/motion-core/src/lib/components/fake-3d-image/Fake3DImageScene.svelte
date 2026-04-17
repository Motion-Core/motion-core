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
		 * Source URL of the color texture.
		 */
		colorSrc: string;
		/**
		 * Source URL of the grayscale depth map texture.
		 */
		depthSrc: string;
		/**
		 * Horizontal displacement threshold.
		 * @default 8
		 */
		xThreshold?: number;
		/**
		 * Vertical displacement threshold.
		 * @default 8
		 */
		yThreshold?: number;
		/**
		 * Pointer sensitivity multiplier applied before displacement thresholding.
		 * @default 0.25
		 */
		sensitivity?: number;
	}

	let {
		colorSrc,
		depthSrc,
		xThreshold = 8,
		yThreshold = 8,
		sensitivity = 0.25,
	}: Props = $props();

	type UniformState = {
		uOriginalTexture: { value: Texture };
		uDepthTexture: { value: Texture };
		uMouse: { value: Vec2 };
		uThreshold: { value: Vec2 };
		uResolution: { value: Vec2 };
		uOriginalTextureSize: { value: Vec2 };
		uDepthTextureSize: { value: Vec2 };
	};

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<UniformState>();
	let setColorSource = $state<(source: string) => void>();
	let setDepthSource = $state<(source: string) => void>();

	const targetPointer = new Vec2(0, 0);
	const smoothPointer = new Vec2(0, 0);

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
		precision mediump float;

		uniform sampler2D uOriginalTexture;
		uniform sampler2D uDepthTexture;
		uniform vec2 uMouse;
		uniform vec2 uThreshold;
		uniform vec2 uResolution;
		uniform vec2 uOriginalTextureSize;
		uniform vec2 uDepthTextureSize;
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
			vec2 baseUv = mirrored(getCoverUV(vUv, uOriginalTextureSize));
			vec2 depthUv = mirrored(getCoverUV(vUv, uDepthTextureSize));
			float depth = texture2D(uDepthTexture, depthUv).r;

			vec2 safeThreshold = max(uThreshold, vec2(0.00001));
			vec2 fake3d = vec2(
				baseUv.x + (depth - 0.5) * uMouse.x / safeThreshold.x,
				baseUv.y + (depth - 0.5) * uMouse.y / safeThreshold.y
			);

			gl_FragColor = texture2D(uOriginalTexture, mirrored(fake3d));
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		uniforms.uThreshold.value.set(xThreshold, yThreshold);
	});

	$effect(() => {
		if (!setColorSource) return;
		setColorSource(colorSrc);
	});

	$effect(() => {
		if (!setDepthSource) return;
		setDepthSource(depthSrc);
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

		const colorTexture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: true,
			flipY: true,
		});

		const depthTexture = new Texture(gl, {
			image: new Uint8Array([127, 127, 127, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: true,
			flipY: true,
		});

		const resolutionUniform = new Vec2(1, 1);
		const mouseUniform = new Vec2(0, 0);
		const thresholdUniform = new Vec2(xThreshold, yThreshold);
		const colorTextureSizeUniform = new Vec2(1, 1);
		const depthTextureSizeUniform = new Vec2(1, 1);

		const localUniforms: UniformState = {
			uOriginalTexture: { value: colorTexture },
			uDepthTexture: { value: depthTexture },
			uMouse: { value: mouseUniform },
			uThreshold: { value: thresholdUniform },
			uResolution: { value: resolutionUniform },
			uOriginalTextureSize: { value: colorTextureSizeUniform },
			uDepthTextureSize: { value: depthTextureSizeUniform },
		};
		uniforms = localUniforms;

		let colorToken = 0;
		const loadColor = (source: string) => {
			colorToken += 1;
			const token = colorToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (token !== colorToken) return;
				colorTexture.image = img;
				colorTextureSizeUniform.set(
					img.naturalWidth || img.width || 1,
					img.naturalHeight || img.height || 1,
				);
			};
			img.src = source;
		};

		let depthToken = 0;
		const loadDepth = (source: string) => {
			depthToken += 1;
			const token = depthToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (token !== depthToken) return;
				depthTexture.image = img;
				depthTextureSizeUniform.set(
					img.naturalWidth || img.width || 1,
					img.naturalHeight || img.height || 1,
				);
			};
			img.src = source;
		};

		setColorSource = loadColor;
		setDepthSource = loadDepth;

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
			resolutionUniform.set(gl.canvas.width, gl.canvas.height);
		};

		const handlePointerMove = (event: PointerEvent) => {
			const rect = targetCanvas.getBoundingClientRect();
			const x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
			const y = -(((event.clientY - rect.top) / rect.height) * 2 - 1);
			targetPointer.set(x, y);
		};

		const handlePointerLeave = () => {
			targetPointer.set(0, 0);
		};

		targetCanvas.addEventListener("pointermove", handlePointerMove);
		targetCanvas.addEventListener("pointerleave", handlePointerLeave);

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

			const targetX = targetPointer.x * sensitivity;
			const targetY = targetPointer.y * sensitivity;
			const lerp = Math.min(1, 5 * delta);
			smoothPointer.x += (targetX - smoothPointer.x) * lerp;
			smoothPointer.y += (targetY - smoothPointer.y) * lerp;
			mouseUniform.set(smoothPointer.x, smoothPointer.y);

			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			targetCanvas.removeEventListener("pointermove", handlePointerMove);
			targetCanvas.removeEventListener("pointerleave", handlePointerLeave);
			setColorSource = undefined;
			setDepthSource = undefined;
			if (colorTexture.texture) {
				gl.deleteTexture(colorTexture.texture);
			}
			if (depthTexture.texture) {
				gl.deleteTexture(depthTexture.texture);
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
