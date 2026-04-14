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

	type DitherMap = "bayer4x4" | "bayer8x8" | "halftone" | "voidAndCluster";
	type ColorRepresentation =
		| string
		| number
		| readonly [number, number, number]
		| { r: number; g: number; b: number };

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Type of dithering map to use.
		 * @default "bayer4x4"
		 */
		ditherMap?: DitherMap;
		/**
		 * Pixel size of the dithering effect.
		 * @default 1
		 */
		pixelSize?: number;
		/**
		 * Foreground color (dots).
		 * @default "#ff6900"
		 */
		color?: ColorRepresentation;
		/**
		 * Background color.
		 * @default "#111113"
		 */
		backgroundColor?: ColorRepresentation;
		/**
		 * Threshold for the dithering effect.
		 * @default 0.0
		 */
		threshold?: number;
	}

	let {
		image,
		ditherMap = "bayer4x4",
		pixelSize = 1,
		color = "#ff6900",
		backgroundColor = "#111113",
		threshold = 0.0,
	}: Props = $props();

	type ThresholdState = {
		size: number;
		texture: Texture;
	};

	type UniformState = {
		uTexture: { value: Texture };
		uThresholdMap: { value: Texture };
		uResolution: { value: Vec2 };
		uMapSize: { value: Vec2 };
		uCoverScale: { value: Vec2 };
		uCoverOffset: { value: Vec2 };
		uPixelSize: { value: number };
		uThreshold: { value: number };
		uColor: { value: Vec3 };
		uBackgroundColor: { value: Vec3 };
	};

	const thresholdMapsData: Record<DitherMap, number[]> = {
		bayer4x4: [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5],
		bayer8x8: [
			0, 32, 8, 40, 2, 34, 10, 42, 48, 16, 56, 24, 50, 18, 58, 26, 12, 44, 4,
			36, 14, 46, 6, 38, 60, 28, 52, 20, 62, 30, 54, 22, 3, 35, 11, 43, 1, 33,
			9, 41, 51, 19, 59, 27, 49, 17, 57, 25, 15, 47, 7, 39, 13, 45, 5, 37, 63,
			31, 55, 23, 61, 29, 53, 21,
		],
		halftone: [
			24, 10, 12, 26, 35, 47, 49, 37, 8, 0, 2, 14, 45, 59, 61, 51, 22, 6, 4, 16,
			43, 57, 63, 53, 30, 20, 18, 28, 33, 41, 55, 39, 34, 46, 48, 36, 25, 11,
			13, 27, 44, 58, 60, 50, 9, 1, 3, 15, 42, 56, 62, 52, 23, 7, 5, 17, 32, 40,
			54, 38, 31, 21, 19, 29,
		],
		voidAndCluster: [
			131, 187, 8, 78, 50, 18, 134, 89, 155, 102, 29, 95, 184, 73, 22, 86, 113,
			171, 142, 105, 34, 166, 9, 60, 151, 128, 40, 110, 168, 137, 45, 28, 64,
			188, 82, 54, 124, 189, 80, 13, 156, 56, 7, 61, 186, 121, 154, 6, 108, 177,
			24, 100, 38, 176, 93, 123, 83, 148, 96, 17, 88, 133, 44, 145, 69, 161,
			139, 72, 30, 181, 115, 27, 163, 47, 178, 65, 164, 14, 120, 48, 5, 127,
			153, 52, 190, 58, 126, 81, 116, 21, 106, 77, 173, 92, 191, 63, 99, 12, 76,
			144, 4, 185, 37, 149, 192, 39, 135, 23, 117, 31, 170, 132, 35, 172, 103,
			66, 129, 79, 3, 97, 57, 159, 70, 141, 53, 94, 114, 20, 49, 158, 19, 146,
			169, 122, 183, 11, 104, 180, 2, 165, 152, 87, 182, 118, 91, 42, 67, 25,
			84, 147, 43, 85, 125, 68, 16, 136, 71, 10, 193, 112, 160, 138, 51, 111,
			162, 26, 194, 46, 174, 107, 41, 143, 33, 74, 1, 101, 195, 15, 75, 140,
			109, 90, 32, 62, 157, 98, 167, 119, 179, 59, 36, 130, 175, 55, 0, 150,
		],
	};

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<UniformState>();
	let setImageSource = $state<(source: string) => void>();
	let setDitherMap = $state<(map: DitherMap) => void>();

	const resolutionUniform = new Vec2(1, 1);
	const mapSizeUniform = new Vec2(4, 4);
	const coverScaleUniform = new Vec2(1, 1);
	const coverOffsetUniform = new Vec2(0, 0);
	const colorUniform = new Vec3(1, 105 / 255, 0);
	const backgroundColorUniform = new Vec3(17 / 255, 17 / 255, 19 / 255);

	const clamp01 = (value: number) => Math.min(1, Math.max(0, value));
	const srgbToLinear = (value: number) =>
		value <= 0.04045 ? value / 12.92 : Math.pow((value + 0.055) / 1.055, 2.4);
	const normalizeTriplet = (
		r: number,
		g: number,
		b: number,
	): [number, number, number] => {
		const scale = Math.max(r, g, b) > 1 ? 255 : 1;
		return [clamp01(r / scale), clamp01(g / scale), clamp01(b / scale)];
	};

	const parseHexColor = (value: string): [number, number, number] | null => {
		const hex = value.replace("#", "").trim();
		if (hex.length === 3 || hex.length === 4) {
			const r = Number.parseInt(hex[0] + hex[0], 16);
			const g = Number.parseInt(hex[1] + hex[1], 16);
			const b = Number.parseInt(hex[2] + hex[2], 16);
			return [r / 255, g / 255, b / 255];
		}
		if (hex.length === 6 || hex.length === 8) {
			const r = Number.parseInt(hex.slice(0, 2), 16);
			const g = Number.parseInt(hex.slice(2, 4), 16);
			const b = Number.parseInt(hex.slice(4, 6), 16);
			return [r / 255, g / 255, b / 255];
		}
		return null;
	};

	let cssColorContext: CanvasRenderingContext2D | null | undefined;
	const parseCssColor = (value: string): [number, number, number] | null => {
		if (typeof document === "undefined") return null;
		if (cssColorContext === undefined) {
			const parserCanvas = document.createElement("canvas");
			parserCanvas.width = 1;
			parserCanvas.height = 1;
			cssColorContext = parserCanvas.getContext("2d");
		}
		if (!cssColorContext) return null;

		cssColorContext.fillStyle = "#000000";
		cssColorContext.fillStyle = value;
		const normalized = cssColorContext.fillStyle;

		if (normalized.startsWith("#")) {
			return parseHexColor(normalized);
		}

		const match = normalized.match(/rgba?\(([^)]+)\)/i);
		if (!match) return null;
		const parts = match[1]
			.split(",")
			.map((part) => Number.parseFloat(part.trim()))
			.filter((part) => Number.isFinite(part));
		if (parts.length < 3) return null;
		const scale = Math.max(parts[0], parts[1], parts[2]) > 1 ? 255 : 1;
		return [
			clamp01(parts[0] / scale),
			clamp01(parts[1] / scale),
			clamp01(parts[2] / scale),
		];
	};

	const toRgb = (
		value: ColorRepresentation,
		fallback: [number, number, number],
	): [number, number, number] => {
		if (typeof value === "number" && Number.isFinite(value)) {
			const int = Math.min(0xffffff, Math.max(0, Math.floor(value)));
			return [
				((int >> 16) & 255) / 255,
				((int >> 8) & 255) / 255,
				(int & 255) / 255,
			];
		}

		if (typeof value === "string") {
			const trimmed = value.trim();
			const parsed = trimmed.startsWith("#")
				? parseHexColor(trimmed)
				: parseCssColor(trimmed);
			return parsed ?? fallback;
		}

		if (Array.isArray(value) && value.length >= 3) {
			return normalizeTriplet(value[0], value[1], value[2]);
		}

		if (
			value &&
			typeof value === "object" &&
			"r" in value &&
			"g" in value &&
			"b" in value
		) {
			const rgb = value as { r: number; g: number; b: number };
			return normalizeTriplet(rgb.r, rgb.g, rgb.b);
		}

		return fallback;
	};

	const toLinearRgb = (
		value: ColorRepresentation,
		fallback: [number, number, number],
	): [number, number, number] => {
		const [r, g, b] = toRgb(value, fallback);
		return [srgbToLinear(r), srgbToLinear(g), srgbToLinear(b)];
	};

	const applyColor = (
		target: Vec3,
		value: ColorRepresentation,
		fallback: [number, number, number],
	) => {
		const [r, g, b] = toLinearRgb(value, fallback);
		target.set(r, g, b);
	};

	const createThresholdTexture = (
		gl: Renderer["gl"],
		map: DitherMap,
	): ThresholdState => {
		const data = thresholdMapsData[map] ?? thresholdMapsData.bayer4x4;
		const size = Math.max(1, Math.round(Math.sqrt(data.length)));
		const count = data.length;
		const pixels = new Uint8Array(size * size * 4);

		for (let i = 0; i < count; i++) {
			const stride = i * 4;
			const value = Math.round((data[i] / count) * 255);
			pixels[stride] = value;
			pixels[stride + 1] = value;
			pixels[stride + 2] = value;
			pixels[stride + 3] = 255;
		}

		const texture = new Texture(gl, {
			image: pixels,
			width: size,
			height: size,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.NEAREST,
			magFilter: gl.NEAREST,
			wrapS: gl.REPEAT,
			wrapT: gl.REPEAT,
			generateMipmaps: false,
			flipY: false,
		});

		return { size, texture };
	};

	const updateCoverUniforms = (
		resolutionWidth: number,
		resolutionHeight: number,
		imageWidth: number,
		imageHeight: number,
	) => {
		const safeWidth = Math.max(1, resolutionWidth);
		const safeHeight = Math.max(1, resolutionHeight);
		const safeImageWidth = Math.max(1, imageWidth);
		const safeImageHeight = Math.max(1, imageHeight);

		const screenAspect = safeWidth / safeHeight;
		const imageAspect = safeImageWidth / safeImageHeight;

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

		uniform sampler2D uTexture;
		uniform sampler2D uThresholdMap;
		uniform vec2 uResolution;
		uniform vec2 uMapSize;
		uniform vec2 uCoverScale;
		uniform vec2 uCoverOffset;
		uniform float uPixelSize;
		uniform float uThreshold;
		uniform vec3 uColor;
		uniform vec3 uBackgroundColor;

		varying vec2 vUv;

		float getLuminance(vec3 colorValue) {
			return dot(colorValue, vec3(0.299, 0.587, 0.114));
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void main() {
			float pixel = max(1.0, uPixelSize);
			vec2 pixelCoord = floor(gl_FragCoord.xy / pixel);
			vec2 centerPixel = pixelCoord * pixel + (pixel * 0.5);
			vec2 centerUv = centerPixel / uResolution;

			vec2 coverScale = max(uCoverScale, vec2(0.00001));
			vec2 imageUv = coverScale * centerUv + uCoverOffset;
			vec4 texColor = texture2D(uTexture, imageUv);

			vec2 mapUv = mod(pixelCoord, uMapSize) / uMapSize;
			mapUv += (0.5 / uMapSize);
			float thresholdValue = texture2D(uThresholdMap, mapUv).r;

			float lum = getLuminance(texColor.rgb);
			float dither = step(thresholdValue + uThreshold, lum);
			vec3 ditheredColor = mix(uBackgroundColor, uColor, dither);

			gl_FragColor = vec4(linearToSrgb(ditheredColor), 1.0);
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		uniforms.uPixelSize.value = Math.max(1, pixelSize);
		uniforms.uThreshold.value = threshold;
		applyColor(uniforms.uColor.value, color, [1, 105 / 255, 0]);
		applyColor(uniforms.uBackgroundColor.value, backgroundColor, [
			17 / 255,
			17 / 255,
			19 / 255,
		]);
	});

	$effect(() => {
		if (!setImageSource) return;
		setImageSource(image);
	});

	$effect(() => {
		if (!setDitherMap) return;
		setDitherMap(ditherMap);
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

		let currentImageWidth = 1;
		let currentImageHeight = 1;
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
				currentImageWidth = img.naturalWidth || img.width || 1;
				currentImageHeight = img.naturalHeight || img.height || 1;
				updateCoverUniforms(
					resolutionUniform.x,
					resolutionUniform.y,
					currentImageWidth,
					currentImageHeight,
				);
			};
			img.src = source;
		};

		let thresholdState = createThresholdTexture(gl, ditherMap);
		const setThresholdMapTexture = (map: DitherMap) => {
			thresholdState = createThresholdTexture(gl, map);
			if (uniforms) {
				uniforms.uThresholdMap.value = thresholdState.texture;
				uniforms.uMapSize.value.set(thresholdState.size, thresholdState.size);
			}
			mapSizeUniform.set(thresholdState.size, thresholdState.size);
		};

		const localUniforms: UniformState = {
			uTexture: { value: imageTexture },
			uThresholdMap: { value: thresholdState.texture },
			uResolution: { value: resolutionUniform },
			uMapSize: { value: mapSizeUniform },
			uCoverScale: { value: coverScaleUniform },
			uCoverOffset: { value: coverOffsetUniform },
			uPixelSize: { value: Math.max(1, pixelSize) },
			uThreshold: { value: threshold },
			uColor: { value: colorUniform },
			uBackgroundColor: { value: backgroundColorUniform },
		};
		uniforms = localUniforms;
		setImageSource = loadImage;
		setDitherMap = setThresholdMapTexture;

		applyColor(colorUniform, color, [1, 105 / 255, 0]);
		applyColor(backgroundColorUniform, backgroundColor, [
			17 / 255,
			17 / 255,
			19 / 255,
		]);

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: localUniforms,
			transparent: false,
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
			updateCoverUniforms(
				resolutionUniform.x,
				resolutionUniform.y,
				currentImageWidth,
				currentImageHeight,
			);
		};

		resize();
		loadImage(image);

		const observer = new ResizeObserver(resize);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		let raf = 0;
		const tick = () => {
			renderer.render({ scene, camera });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			setImageSource = undefined;
			setDitherMap = undefined;
			if (thresholdState.texture.texture) {
				gl.deleteTexture(thresholdState.texture.texture);
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
