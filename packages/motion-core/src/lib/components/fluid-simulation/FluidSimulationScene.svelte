<script lang="ts">
	import { onMount } from "svelte";
	import {
		Mesh,
		Program,
		RenderTarget,
		Renderer,
		Triangle,
		Vec2,
		Vec3,
	} from "ogl";

	type ColorRepresentation =
		| string
		| number
		| readonly [number, number, number]
		| { r: number; g: number; b: number };

	interface Props {
		/**
		 * Dissipation factor for the fluid.
		 * @default 0.96
		 */
		dissipation?: number;
		/**
		 * Radius of the pointer influence.
		 * @default 0.005
		 */
		pointerSize?: number;
		/**
		 * Fluid splat color.
		 * @default "#ff6900"
		 */
		color?: ColorRepresentation;
		/**
		 * Fluid velocity dissipation.
		 * @default 0.96
		 */
		velocityDissipation?: number;
		/**
		 * Pressure iterations. More iterations = more accurate but slower.
		 * @default 10
		 */
		pressureIterations?: number;
	}

	type PointerState = {
		x: number;
		y: number;
		dx: number;
		dy: number;
		moved: boolean;
		initialized: boolean;
	};

	type PreviewState = {
		enabled: boolean;
		timeMs: number;
	};

	type CanvasMetrics = {
		width: number;
		height: number;
	};

	type DoubleFBO = {
		read: RenderTarget;
		write: RenderTarget;
		swap: () => void;
	};

	let {
		dissipation = 0.96,
		pointerSize = 0.005,
		color = "#ff6900",
		velocityDissipation = 0.96,
		pressureIterations = 10,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();

	const pointerState = $state<PointerState>({
		x: 0,
		y: 0,
		dx: 0,
		dy: 0,
		moved: false,
		initialized: false,
	});
	const previewState = $state<PreviewState>({
		enabled: true,
		timeMs: 0,
	});
	const canvasMetrics = $state<CanvasMetrics>({
		width: 1,
		height: 1,
	});

	const pointerUv = new Vec2();
	const splatColor = new Vec3();

	const pointerForceClamp = 450;
	const pointerForceInitialLerp = 0.2;
	const pointerForceLerp = 0.55;

	const clamp = (value: number, min: number, max: number) =>
		Math.min(max, Math.max(min, value));
	const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
	const clamp01 = (value: number) => clamp(value, 0, 1);
	const srgbToLinear = (value: number) =>
		value <= 0.04045 ? value / 12.92 : Math.pow((value + 0.055) / 1.055, 2.4);

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

	const normalizeTriplet = (
		r: number,
		g: number,
		b: number,
	): [number, number, number] => {
		const scale = Math.max(r, g, b) > 1 ? 255 : 1;
		return [clamp01(r / scale), clamp01(g / scale), clamp01(b / scale)];
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

	$effect(() => {
		const [r, g, b] = toLinearRgb(color, [1, 105 / 255, 0]);
		splatColor.set(r, g, b);
	});

	const updatePointerPosition = (
		px: number,
		py: number,
		width: number,
		height: number,
	) => {
		const prevX = pointerState.x;
		const prevY = pointerState.y;
		const targetDx = clamp(
			5 * (px - prevX),
			-pointerForceClamp,
			pointerForceClamp,
		);
		const targetDy = clamp(
			5 * (py - prevY),
			-pointerForceClamp,
			pointerForceClamp,
		);
		const lerpFactor = pointerState.initialized
			? pointerForceLerp
			: pointerForceInitialLerp;

		pointerState.moved = true;
		pointerState.dx = lerp(pointerState.dx, targetDx, lerpFactor);
		pointerState.dy = lerp(pointerState.dy, targetDy, lerpFactor);
		pointerState.x = px;
		pointerState.y = py;
		pointerState.initialized = true;

		if (width > 0 && height > 0) {
			pointerUv.set(px / width, 1 - py / height);
		}
	};

	const vertexShader = `
		attribute vec2 uv;
		attribute vec2 position;
		varying vec2 vUv;
		varying vec2 vL;
		varying vec2 vR;
		varying vec2 vT;
		varying vec2 vB;
		uniform vec2 uTexel;

		void main () {
			vUv = uv;
			vL = vUv - vec2(uTexel.x, 0.);
			vR = vUv + vec2(uTexel.x, 0.);
			vT = vUv + vec2(0., uTexel.y);
			vB = vUv - vec2(0., uTexel.y);
			gl_Position = vec4(position, 0.0, 1.0);
		}
	`;

	const advectionShader = `
		precision highp float;
		varying vec2 vUv;
		uniform sampler2D uVelocity;
		uniform sampler2D uInput;
		uniform vec2 uTexel;
		uniform float uDt;
		uniform float uDissipation;

		vec4 bilerp (sampler2D sam, vec2 uv, vec2 tsize) {
			vec2 st = uv / tsize - 0.5;
			vec2 iuv = floor(st);
			vec2 fuv = fract(st);
			vec4 a = texture2D(sam, (iuv + vec2(0.5, 0.5)) * tsize);
			vec4 b = texture2D(sam, (iuv + vec2(1.5, 0.5)) * tsize);
			vec4 c = texture2D(sam, (iuv + vec2(0.5, 1.5)) * tsize);
			vec4 d = texture2D(sam, (iuv + vec2(1.5, 1.5)) * tsize);
			return mix(mix(a, b, fuv.x), mix(c, d, fuv.x), fuv.y);
		}

		void main () {
			vec2 coord = vUv - uDt * bilerp(uVelocity, vUv, uTexel).xy * uTexel;
			gl_FragColor = uDissipation * bilerp(uInput, coord, uTexel);
			gl_FragColor.a = 1.;
		}
	`;

	const divergenceShader = `
		precision highp float;
		varying vec2 vL;
		varying vec2 vR;
		varying vec2 vT;
		varying vec2 vB;
		uniform sampler2D uVelocity;

		void main () {
			float L = texture2D(uVelocity, vL).x;
			float R = texture2D(uVelocity, vR).x;
			float T = texture2D(uVelocity, vT).y;
			float B = texture2D(uVelocity, vB).y;
			float div = .6 * (R - L + T - B);
			gl_FragColor = vec4(div, 0., 0., 1.);
		}
	`;

	const pressureShader = `
		precision highp float;
		varying vec2 vUv;
		varying vec2 vL;
		varying vec2 vR;
		varying vec2 vT;
		varying vec2 vB;
		uniform sampler2D uPressure;
		uniform sampler2D uDivergence;

		void main () {
			float L = texture2D(uPressure, vL).x;
			float R = texture2D(uPressure, vR).x;
			float T = texture2D(uPressure, vT).x;
			float B = texture2D(uPressure, vB).x;
			float divergence = texture2D(uDivergence, vUv).x;
			float pressure = (L + R + B + T - divergence) * 0.25;
			gl_FragColor = vec4(pressure, 0., 0., 1.);
		}
	`;

	const gradientSubtractShader = `
		precision highp float;
		varying vec2 vUv;
		varying vec2 vL;
		varying vec2 vR;
		varying vec2 vT;
		varying vec2 vB;
		uniform sampler2D uPressure;
		uniform sampler2D uVelocity;

		void main () {
			float L = texture2D(uPressure, vL).x;
			float R = texture2D(uPressure, vR).x;
			float T = texture2D(uPressure, vT).x;
			float B = texture2D(uPressure, vB).x;
			vec2 velocity = texture2D(uVelocity, vUv).xy;
			velocity.xy -= vec2(R - L, T - B);
			gl_FragColor = vec4(velocity, 0., 1.);
		}
	`;

	const splatShader = `
		precision highp float;
		varying vec2 vUv;
		uniform sampler2D uInput;
		uniform float uRatio;
		uniform vec3 uPointValue;
		uniform vec2 uPoint;
		uniform float uPointSize;

		void main () {
			vec2 p = vUv - uPoint.xy;
			p.x *= uRatio;
			vec3 splat = pow(2., -dot(p, p) / uPointSize) * uPointValue;
			vec3 base = texture2D(uInput, vUv).xyz;
			gl_FragColor = vec4(base + splat, 1.);
		}
	`;

	const outputVertexShader = `
		attribute vec2 uv;
		attribute vec2 position;
		varying vec2 vUv;
		void main() {
			vUv = uv;
			gl_Position = vec4(position, 0.0, 1.0);
		}
	`;

	const outputShader = `
		precision highp float;
		varying vec2 vUv;
		uniform sampler2D uTexture;

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void main() {
			vec3 C = texture2D(uTexture, vUv).rgb;
			float a = max(C.r, max(C.g, C.b));
			gl_FragColor = vec4(linearToSrgb(C), a);
		}
	`;

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

		const halfFloatExt = gl.renderer.extensions["OES_texture_half_float"] as
			| { HALF_FLOAT_OES: number }
			| undefined;
		const textureType = gl.renderer.isWebgl2
			? (gl as WebGL2RenderingContext).HALF_FLOAT
			: (halfFloatExt?.HALF_FLOAT_OES ?? gl.FLOAT);
		const internalFormat = gl.renderer.isWebgl2
			? textureType === gl.FLOAT
				? (gl as WebGL2RenderingContext).RGBA32F
				: (gl as WebGL2RenderingContext).RGBA16F
			: gl.RGBA;

		const createFBO = (w: number, h: number) =>
			new RenderTarget(gl, {
				width: w,
				height: h,
				type: textureType,
				format: gl.RGBA,
				internalFormat,
				minFilter: gl.NEAREST,
				magFilter: gl.NEAREST,
				depth: false,
				stencil: false,
			});

		const createDoubleFBO = (w: number, h: number): DoubleFBO => {
			const doubleFBO: DoubleFBO = {
				read: createFBO(w, h),
				write: createFBO(w, h),
				swap: () => {
					const temp = doubleFBO.read;
					doubleFBO.read = doubleFBO.write;
					doubleFBO.write = temp;
				},
			};
			return doubleFBO;
		};

		const density = createDoubleFBO(128, 128);
		const velocity = createDoubleFBO(128, 128);
		const pressure = createDoubleFBO(128, 128);
		const divergence = createFBO(128, 128);

		const texel = new Vec2(1 / 128, 1 / 128);
		const advectionUniforms = {
			uVelocity: { value: velocity.read.texture },
			uInput: { value: velocity.read.texture },
			uTexel: { value: texel },
			uDt: { value: 1 / 60 },
			uDissipation: { value: velocityDissipation },
		};
		const divergenceUniforms = {
			uVelocity: { value: velocity.read.texture },
			uTexel: { value: texel },
		};
		const pressureUniforms = {
			uPressure: { value: pressure.read.texture },
			uDivergence: { value: divergence.texture },
			uTexel: { value: texel },
		};
		const gradientSubtractUniforms = {
			uPressure: { value: pressure.read.texture },
			uVelocity: { value: velocity.read.texture },
			uTexel: { value: texel },
		};
		const splatUniforms = {
			uInput: { value: velocity.read.texture },
			uRatio: { value: 1 },
			uPointValue: { value: new Vec3() },
			uPoint: { value: pointerUv },
			uPointSize: { value: pointerSize },
		};
		const outputUniforms = {
			uTexture: { value: density.read.texture },
		};

		const advectionProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: advectionShader,
			uniforms: advectionUniforms,
			depthTest: false,
			depthWrite: false,
		});
		const divergenceProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: divergenceShader,
			uniforms: divergenceUniforms,
			depthTest: false,
			depthWrite: false,
		});
		const pressureProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: pressureShader,
			uniforms: pressureUniforms,
			depthTest: false,
			depthWrite: false,
		});
		const gradientSubtractProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: gradientSubtractShader,
			uniforms: gradientSubtractUniforms,
			depthTest: false,
			depthWrite: false,
		});
		const splatProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: splatShader,
			uniforms: splatUniforms,
			depthTest: false,
			depthWrite: false,
		});
		const outputProgram = new Program(gl, {
			vertex: outputVertexShader,
			fragment: outputShader,
			uniforms: outputUniforms,
			depthTest: false,
			depthWrite: false,
			transparent: true,
		});

		const triangle = new Triangle(gl);
		const simMesh = new Mesh(gl, {
			geometry: triangle,
			program: advectionProgram,
		});
		const outputMesh = new Mesh(gl, {
			geometry: triangle,
			program: outputProgram,
		});

		const renderPass = (program: Program, target: RenderTarget) => {
			simMesh.program = program;
			renderer.render({ scene: simMesh, target, clear: true });
		};

		const handlePointerMove = (e: PointerEvent) => {
			const rect = targetCanvas.getBoundingClientRect();
			const x = e.clientX - rect.left;
			const y = e.clientY - rect.top;

			const wasPreview = previewState.enabled;
			previewState.enabled = false;
			if (wasPreview) {
				pointerState.initialized = false;
				pointerState.dx = 0;
				pointerState.dy = 0;
			}
			updatePointerPosition(x, y, rect.width, rect.height);
		};

		const handleTouchMove = (e: TouchEvent) => {
			e.preventDefault();
			const touch = e.touches[0];
			if (!touch) return;
			const rect = targetCanvas.getBoundingClientRect();
			const x = touch.clientX - rect.left;
			const y = touch.clientY - rect.top;

			const wasPreview = previewState.enabled;
			previewState.enabled = false;
			if (wasPreview) {
				pointerState.initialized = false;
				pointerState.dx = 0;
				pointerState.dy = 0;
			}
			updatePointerPosition(x, y, rect.width, rect.height);
		};

		targetCanvas.addEventListener("pointermove", handlePointerMove);
		targetCanvas.addEventListener("touchmove", handleTouchMove, {
			passive: false,
		});

		const resizeSimulation = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));

			renderer.setSize(width, height);
			canvasMetrics.width = width;
			canvasMetrics.height = height;

			const simResX = Math.max(1, Math.floor(width * 0.5));
			const simResY = Math.max(1, Math.floor(height * 0.5));

			density.read.setSize(simResX, simResY);
			density.write.setSize(simResX, simResY);
			velocity.read.setSize(simResX, simResY);
			velocity.write.setSize(simResX, simResY);
			pressure.read.setSize(simResX, simResY);
			pressure.write.setSize(simResX, simResY);
			divergence.setSize(simResX, simResY);

			const texelX = 1 / simResX;
			const texelY = 1 / simResY;
			texel.set(texelX, texelY);

			if (canvasMetrics.width > 0 && canvasMetrics.height > 0) {
				pointerUv.set(
					pointerState.x / canvasMetrics.width,
					1 - pointerState.y / canvasMetrics.height,
				);
			}
		};

		resizeSimulation();
		const resizeObserver = new ResizeObserver(resizeSimulation);
		resizeObserver.observe(targetCanvas);
		if (targetCanvas.parentElement) {
			resizeObserver.observe(targetCanvas.parentElement);
		}

		const disposeTarget = (target: RenderTarget) => {
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

		let raf = 0;
		let previous = 0;
		const tick = (now: number) => {
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;
			const dt = 1 / 60;
			const width = canvasMetrics.width || targetCanvas.clientWidth || 1;
			const height = canvasMetrics.height || targetCanvas.clientHeight || 1;
			const aspect = height > 0 ? width / height : 1;

			if (previewState.enabled && width > 0 && height > 0) {
				previewState.timeMs += delta * 1000;
				const previewX =
					(0.5 - 0.45 * Math.sin(0.003 * previewState.timeMs - 2)) * width;
				const previewY =
					(0.5 +
						0.1 * Math.sin(0.0025 * previewState.timeMs) +
						0.1 * Math.cos(0.002 * previewState.timeMs)) *
					height;
				updatePointerPosition(previewX, previewY, width, height);
			}

			if (pointerState.moved) {
				splatUniforms.uInput.value = velocity.read.texture;
				splatUniforms.uRatio.value = aspect;
				splatUniforms.uPoint.value.set(pointerUv.x, pointerUv.y);
				splatUniforms.uPointValue.value.set(
					pointerState.dx,
					-pointerState.dy,
					1,
				);
				splatUniforms.uPointSize.value = pointerSize;
				renderPass(splatProgram, velocity.write);
				velocity.swap();

				splatUniforms.uInput.value = density.read.texture;
				splatUniforms.uPointValue.value.set(
					splatColor.x,
					splatColor.y,
					splatColor.z,
				);
				renderPass(splatProgram, density.write);
				density.swap();

				if (!previewState.enabled) {
					pointerState.moved = false;
				}
			}

			divergenceUniforms.uVelocity.value = velocity.read.texture;
			renderPass(divergenceProgram, divergence);

			pressureUniforms.uDivergence.value = divergence.texture;
			const iterations = Math.max(0, Math.floor(pressureIterations));
			for (let i = 0; i < iterations; i++) {
				pressureUniforms.uPressure.value = pressure.read.texture;
				renderPass(pressureProgram, pressure.write);
				pressure.swap();
			}

			gradientSubtractUniforms.uPressure.value = pressure.read.texture;
			gradientSubtractUniforms.uVelocity.value = velocity.read.texture;
			renderPass(gradientSubtractProgram, velocity.write);
			velocity.swap();

			advectionUniforms.uDt.value = dt;
			advectionUniforms.uVelocity.value = velocity.read.texture;
			advectionUniforms.uInput.value = velocity.read.texture;
			advectionUniforms.uDissipation.value = velocityDissipation;
			renderPass(advectionProgram, velocity.write);
			velocity.swap();

			advectionUniforms.uVelocity.value = velocity.read.texture;
			advectionUniforms.uInput.value = density.read.texture;
			advectionUniforms.uDissipation.value = dissipation;
			renderPass(advectionProgram, density.write);
			density.swap();

			outputUniforms.uTexture.value = density.read.texture;
			renderer.render({ scene: outputMesh, clear: true });

			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			resizeObserver.disconnect();
			targetCanvas.removeEventListener("pointermove", handlePointerMove);
			targetCanvas.removeEventListener("touchmove", handleTouchMove);

			disposeTarget(density.read);
			disposeTarget(density.write);
			disposeTarget(velocity.read);
			disposeTarget(velocity.write);
			disposeTarget(pressure.read);
			disposeTarget(pressure.write);
			disposeTarget(divergence);

			advectionProgram.remove();
			divergenceProgram.remove();
			pressureProgram.remove();
			gradientSubtractProgram.remove();
			splatProgram.remove();
			outputProgram.remove();
			triangle.remove();
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
