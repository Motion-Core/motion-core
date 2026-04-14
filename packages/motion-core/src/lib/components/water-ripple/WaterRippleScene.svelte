<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Plane,
		Program,
		RenderTarget,
		Renderer,
		Texture,
		Transform,
		Triangle,
		Vec2,
	} from "ogl";
	import brushUrl from "../../assets/water-ripple-brush.png";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Size of the ripple brush.
		 * @default 100
		 */
		brushSize?: number;
	}

	let { image, brushSize = 100 }: Props = $props();

	type UniformState = {
		uTexture: { value: Texture };
		uDisplacement: { value: Texture };
		uResolution: { value: Vec2 };
		uTextureSize: { value: Vec2 };
	};

	type BrushWave = {
		mesh: Mesh;
		opacityUniform: { value: number };
		scaleX: number;
		scaleY: number;
	};

	const MAX_WAVES = 100;

	let canvas = $state<HTMLCanvasElement>();
	let setImageSource = $state<(source: string) => void>();
	let setBrushSize = $state<(value: number) => void>();

	const resolutionUniform = new Vec2(1, 1);
	const textureSizeUniform = new Vec2(1, 1);

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
		uniform sampler2D uDisplacement;
		uniform vec2 uResolution;
		uniform vec2 uTextureSize;

		varying vec2 vUv;
		const float PI = 3.141592653589793238;

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 safeTexture = max(textureSize, vec2(1.0));
			vec2 s = uResolution / safeTexture;
			float scale = max(s.x, s.y);
			vec2 scaledSize = safeTexture * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		void main() {
			vec2 coverUv = getCoverUV(vUv, uTextureSize);
			vec4 displacement = texture2D(uDisplacement, vUv);
			float theta = displacement.r * 2.0 * PI;
			vec2 dir = vec2(sin(theta), cos(theta));
			vec2 finalUv = coverUv + dir * displacement.r * 0.05;
			gl_FragColor = texture2D(uTexture, finalUv);
		}
	`;

	const brushVertexShader = `
		attribute vec3 position;
		attribute vec2 uv;
		uniform mat4 modelViewMatrix;
		uniform mat4 projectionMatrix;
		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
		}
	`;

	const brushFragmentShader = `
		precision highp float;

		uniform sampler2D uBrush;
		uniform float uOpacity;
		varying vec2 vUv;

		void main() {
			vec4 tex = texture2D(uBrush, vUv);
			gl_FragColor = vec4(tex.rgb * uOpacity, tex.a * uOpacity);
		}
	`;

	const disposeTarget = (gl: Renderer["gl"], target: RenderTarget) => {
		target.textures.forEach((texture) => {
			if (texture.texture) gl.deleteTexture(texture.texture);
		});
		if (target.depthTexture?.texture)
			gl.deleteTexture(target.depthTexture.texture);
		if (target.depthBuffer) gl.deleteRenderbuffer(target.depthBuffer);
		if (target.stencilBuffer) gl.deleteRenderbuffer(target.stencilBuffer);
		if (target.depthStencilBuffer)
			gl.deleteRenderbuffer(target.depthStencilBuffer);
		if (target.buffer) gl.deleteFramebuffer(target.buffer);
	};

	$effect(() => {
		if (!setImageSource) return;
		setImageSource(image);
	});

	$effect(() => {
		if (!setBrushSize) return;
		setBrushSize(brushSize);
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

		const brushCamera = new Camera(gl, {
			left: -1,
			right: 1,
			top: 1,
			bottom: -1,
			near: 0,
			far: 10,
		});
		brushCamera.position.z = 1;

		const scene = new Transform();
		const brushScene = new Transform();

		const fullscreenGeometry = new Triangle(gl);
		let brushPixelSize = Math.max(1, brushSize);
		const brushGeometry = new Plane(gl, {
			width: brushPixelSize,
			height: brushPixelSize,
		});

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
			generateMipmaps: true,
			flipY: true,
		});

		const brushTexture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 0]),
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

		const displacementTarget = new RenderTarget(gl, {
			width: 1,
			height: 1,
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
		});

		const mainUniforms: UniformState = {
			uTexture: { value: imageTexture },
			uDisplacement: { value: displacementTarget.texture },
			uResolution: { value: resolutionUniform },
			uTextureSize: { value: textureSizeUniform },
		};

		const mainProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: mainUniforms,
			transparent: true,
			depthTest: false,
			depthWrite: false,
		});

		const mainMesh = new Mesh(gl, {
			geometry: fullscreenGeometry,
			program: mainProgram,
			frustumCulled: false,
		});
		mainMesh.setParent(scene);

		const waves: BrushWave[] = [];
		for (let i = 0; i < MAX_WAVES; i++) {
			const opacityUniform = { value: 0 };
			const brushProgram = new Program(gl, {
				vertex: brushVertexShader,
				fragment: brushFragmentShader,
				uniforms: {
					uBrush: { value: brushTexture },
					uOpacity: opacityUniform,
				},
				transparent: true,
				depthTest: false,
				depthWrite: false,
				cullFace: null,
			});
			brushProgram.setBlendFunc(gl.SRC_ALPHA, gl.ONE);

			const brushMesh = new Mesh(gl, {
				geometry: brushGeometry,
				program: brushProgram,
				frustumCulled: false,
			});
			brushMesh.visible = false;
			brushMesh.rotation.z = Math.random() * Math.PI * 2;
			brushMesh.setParent(brushScene);

			waves.push({
				mesh: brushMesh,
				opacityUniform,
				scaleX: 1.5,
				scaleY: 1.5,
			});
		}

		let imageToken = 0;
		let brushToken = 0;
		let disposed = false;

		const loadImage = (source: string) => {
			imageToken += 1;
			const token = imageToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (disposed || token !== imageToken) return;
				imageTexture.image = img;
				textureSizeUniform.set(
					img.naturalWidth || img.width || 1,
					img.naturalHeight || img.height || 1,
				);
			};
			img.src = source;
		};

		const loadBrush = (source: string) => {
			brushToken += 1;
			const token = brushToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (disposed || token !== brushToken) return;
				brushTexture.image = img;
			};
			img.src = source;
		};

		setImageSource = loadImage;
		setBrushSize = (value: number) => {
			brushPixelSize = Math.max(1, value);
		};

		let currentWave = 0;
		const prevMouse = new Vec2(0, 0);

		const setNewWave = (x: number, y: number, index: number) => {
			const wave = waves[index];
			if (!wave) return;
			wave.mesh.position.x = x;
			wave.mesh.position.y = y;
			wave.mesh.visible = true;
			wave.opacityUniform.value = 1;
			wave.scaleX = 1.5;
			wave.scaleY = 1.5;
			wave.mesh.scale.set(wave.scaleX, wave.scaleY, 1.5);
		};

		const onPointerMove = (event: PointerEvent) => {
			const rect = targetCanvas.getBoundingClientRect();
			const x = event.clientX - rect.left - rect.width / 2;
			const y = -(event.clientY - rect.top - rect.height / 2);

			if (Math.abs(x - prevMouse.x) > 4 || Math.abs(y - prevMouse.y) > 4) {
				currentWave = (currentWave + 1) % MAX_WAVES;
				setNewWave(x, y, currentWave);
				prevMouse.set(x, y);
			}
		};

		targetCanvas.addEventListener("pointermove", onPointerMove);

		const resize = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));

			renderer.setSize(width, height);
			resolutionUniform.set(width, height);

			brushCamera.left = -width / 2;
			brushCamera.right = width / 2;
			brushCamera.top = height / 2;
			brushCamera.bottom = -height / 2;
			brushCamera.updateProjectionMatrix();

			displacementTarget.setSize(width, height);
		};

		resize();
		loadImage(image);
		loadBrush(brushUrl);

		const observer = new ResizeObserver(resize);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		let raf = 0;
		let previousTime = 0;
		const tick = (now: number) => {
			const delta = previousTime ? (now - previousTime) / 1000 : 0;
			previousTime = now;
			const timeScale = delta * 60;

			for (let i = 0; i < waves.length; i++) {
				const wave = waves[i];
				if (!wave.mesh.visible) continue;

				wave.mesh.rotation.z += 0.02 * timeScale;
				wave.opacityUniform.value *= Math.pow(0.96, timeScale);
				wave.scaleX = 0.982 * wave.scaleX + 0.108 * timeScale;

				const decay = Math.pow(0.99, timeScale);
				wave.opacityUniform.value *= decay;
				wave.scaleX = 0.982 * wave.scaleX + 0.108;
				wave.scaleY = 0.982 * wave.scaleY + 0.108;
				wave.mesh.scale.set(wave.scaleX, wave.scaleY, 1.5);

				if (wave.opacityUniform.value < 0.002) {
					wave.mesh.visible = false;
				}
			}

			renderer.render({
				scene: brushScene,
				camera: brushCamera,
				target: displacementTarget,
				clear: true,
			});

			renderer.render({ scene, camera, clear: true });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			disposed = true;
			imageToken += 1;
			brushToken += 1;
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			targetCanvas.removeEventListener("pointermove", onPointerMove);
			setImageSource = undefined;
			setBrushSize = undefined;

			mainProgram.remove();
			waves.forEach((wave) => wave.mesh.program.remove());
			fullscreenGeometry.remove();
			brushGeometry.remove();
			disposeTarget(gl, displacementTarget);
			if (imageTexture.texture) gl.deleteTexture(imageTexture.texture);
			if (brushTexture.texture) gl.deleteTexture(brushTexture.texture);
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
