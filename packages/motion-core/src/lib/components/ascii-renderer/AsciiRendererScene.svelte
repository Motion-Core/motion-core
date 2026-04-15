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
		Vec3,
	} from "ogl";
	import { toLinearRgb } from "../../helpers/color";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Grid density for the ASCII effect.
		 * @default 25.0
		 */
		density?: number;
		/**
		 * Intensity of the ASCII character generation threshold.
		 * @default 25.0
		 */
		strength?: number;
		/**
		 * Foreground color of the ASCII characters.
		 * @default "#00ff00"
		 */
		color?: string;
		/**
		 * Background color.
		 * @default "#17181A"
		 */
		backgroundColor?: string;
	}

	let {
		image,
		density = 25.0,
		strength = 25.0,
		color = "#00ff00",
		backgroundColor = "#17181A",
	}: Props = $props();

	type UniformState = {
		uTime: { value: number };
		uResolution: { value: Vec2 };
		uTexture: { value: Texture };
		uCoverScale: { value: Vec2 };
		uCoverOffset: { value: Vec2 };
		uDensity: { value: number };
		uStrength: { value: number };
		uColor: { value: Vec3 };
		uBackgroundColor: { value: Vec3 };
	};

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<UniformState>();
	let setImageSource = $state<(source: string) => void>();

	const resolutionUniform = new Vec2(1, 1);
	const coverScaleUniform = new Vec2(1, 1);
	const coverOffsetUniform = new Vec2(0, 0);
	const colorUniform = new Vec3(0, 1, 0);
	const backgroundColorUniform = new Vec3(23 / 255, 24 / 255, 26 / 255);

	let canvasWidth = 1;
	let canvasHeight = 1;
	let imageWidth = 1;
	let imageHeight = 1;

	const updateCoverUniforms = () => {
		if (
			canvasWidth <= 0 ||
			canvasHeight <= 0 ||
			imageWidth <= 0 ||
			imageHeight <= 0
		) {
			return;
		}

		const screenAspect = canvasWidth / canvasHeight;
		const imageAspect = imageWidth / imageHeight;

		let scaleX = 1;
		let scaleY = 1;
		let offsetX = 0;
		let offsetY = 0;

		if (screenAspect > imageAspect) {
			scaleY = imageAspect / screenAspect;
			offsetY = (1 - scaleY) * 0.5;
		} else {
			scaleX = screenAspect / imageAspect;
			offsetX = (1 - scaleX) * 0.5;
		}

		coverScaleUniform.set(scaleX, scaleY);
		coverOffsetUniform.set(offsetX, offsetY);
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

		uniform float uTime;
		uniform vec2 uResolution;
		uniform sampler2D uTexture;
		uniform vec2 uCoverScale;
		uniform vec2 uCoverOffset;
		uniform float uDensity;
		uniform float uStrength;
		uniform vec3 uColor;
		uniform vec3 uBackgroundColor;

		varying vec2 vUv;

		vec2 mirrored(vec2 value) {
			vec2 m = mod(value, 2.0);
			return mix(m, 2.0 - m, step(1.0, m));
		}

		float digit(vec2 p, float intensity){
			p = (fract(p) - 0.5) * 1.2 + 0.5;

			if (p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) return 0.0;

			float x = fract(p.x * 5.0);
			float y = fract((1.0 - p.y) * 5.0);
			int i = int(floor((1.0 - p.y) * 5.0));
			int j = int(floor(p.x * 5.0));
			int n = (i-2)*(i-2)+(j-2)*(j-2);
			float f = float(n)/16.0;
			float isOn = smoothstep(0.1, 0.2, intensity - f);
			return isOn * (0.2 + y*4.0/5.0) * (0.75 + x/4.0);
		}

		float onOff(float a, float b, float c) {
			return step(c, sin(uTime + a*cos(uTime*b)));
		}

		float displace(vec2 look) {
			float y = (look.y - mod(uTime/4.0, 1.0));
			float window = 1.0 / (1.0 + 50.0 * y * y);
			return sin(look.y * 20.0 + uTime)/80.0 * onOff(4.0, 2.0, 0.8) * (1.0 + cos(uTime*60.0)) * window;
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void main() {
			vec2 p = vUv;
			float aspect = uResolution.x / max(uResolution.y, 1.0);
			p.x *= aspect;

			vec2 pDisplaced = p;
			pDisplaced.x += displace(p) * 0.5;

			vec2 grid = vec2(3.0, 1.0) * uDensity;

			vec2 cellIndex = floor(pDisplaced * grid);
			vec2 cellCenterP = (cellIndex + 0.5) / grid;

			vec2 cellCenterUV = cellCenterP;
			cellCenterUV.x /= aspect;

			vec2 cellCenterUVCover = (cellCenterUV * uCoverScale) + uCoverOffset;
			vec3 texColor = texture2D(uTexture, mirrored(cellCenterUVCover)).rgb;

			float intensity = dot(texColor, vec3(0.299, 0.587, 0.114));
			intensity = pow(intensity, 2.8);

			float bar = mod(p.y + uTime * 20.0, 1.0) < 0.2 ? 1.4 : 1.0;

			vec2 gridP = pDisplaced * grid;
			float middle = digit(gridP, intensity * 1.3 * uStrength);

			float off = 0.002;
			float sum = 0.0;
			for (float i = -1.0; i < 2.0; i += 1.0) {
				for (float j = -1.0; j < 2.0; j += 1.0) {
					vec2 offsetGridP = gridP + vec2(off * i * grid.x, off * j * grid.y);
					sum += digit(offsetGridP, intensity * 1.3 * uStrength);
				}
			}

			vec3 emission = vec3(0.6) * middle + sum / 15.0 * uColor * bar;
			vec3 finalColor = uBackgroundColor + emission;

			gl_FragColor = vec4(linearToSrgb(finalColor), 1.0);
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		uniforms.uDensity.value = density;
	});

	$effect(() => {
		if (!uniforms) return;
		uniforms.uStrength.value = strength;
	});

	$effect(() => {
		const [r, g, b] = toLinearRgb(color, [0, 1, 0]);
		colorUniform.set(r, g, b);
	});

	$effect(() => {
		const [r, g, b] = toLinearRgb(backgroundColor, [
			23 / 255,
			24 / 255,
			26 / 255,
		]);
		backgroundColorUniform.set(r, g, b);
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
			uResolution: { value: resolutionUniform },
			uTexture: { value: imageTexture },
			uCoverScale: { value: coverScaleUniform },
			uCoverOffset: { value: coverOffsetUniform },
			uDensity: { value: density },
			uStrength: { value: strength },
			uColor: { value: colorUniform },
			uBackgroundColor: { value: backgroundColorUniform },
		};
		uniforms = localUniforms;

		let imageLoadToken = 0;
		let disposed = false;
		const loadImage = (source: string) => {
			imageLoadToken += 1;
			const token = imageLoadToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (disposed || token !== imageLoadToken) return;
				imageTexture.image = img;
				imageWidth = img.naturalWidth || img.width || 1;
				imageHeight = img.naturalHeight || img.height || 1;
				updateCoverUniforms();
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

		const mesh = new Mesh(gl, { geometry, program, frustumCulled: false });
		mesh.setParent(scene);

		const resize = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));

			renderer.setSize(width, height);
			canvasWidth = width;
			canvasHeight = height;
			resolutionUniform.set(width, height);
			updateCoverUniforms();
		};

		resize();
		loadImage(image);

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
			disposed = true;
			imageLoadToken += 1;
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			setImageSource = undefined;
			program.remove();
			geometry.remove();
			if (imageTexture.texture) gl.deleteTexture(imageTexture.texture);
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
