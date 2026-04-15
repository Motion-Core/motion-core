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
		 * Base color of the rays.
		 * @default "#FFFFFF"
		 */
		color?: ColorRepresentation;
		/**
		 * Color of the background.
		 * @default "#000000"
		 */
		backgroundColor?: ColorRepresentation;
		/**
		 * Horizontal anchor point of the ray source (0-1).
		 * @default 0.5
		 */
		anchorX?: number;
		/**
		 * Vertical anchor point of the ray source (0-1).
		 * @default 1.2
		 */
		anchorY?: number;
		/**
		 * Horizontal direction of the rays.
		 * @default 0.0
		 */
		directionX?: number;
		/**
		 * Vertical direction of the rays.
		 * @default -1.0
		 */
		directionY?: number;
		/**
		 * Speed multiplier for the animation.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * The spread of the light rays.
		 * @default 1.0
		 */
		lightSpread?: number;
		/**
		 * The length of the rays.
		 * @default 1.0
		 */
		rayLength?: number;
		/**
		 * Whether the rays should pulsate.
		 * @default false
		 */
		pulsating?: boolean;
		/**
		 * Distance at which the rays start to fade out.
		 * @default 1.0
		 */
		fadeDistance?: number;
		/**
		 * Saturation of the final ray colors.
		 * @default 1.0
		 */
		saturation?: number;
		/**
		 * Amount of grain/noise applied to the rays.
		 * @default 0.0
		 */
		noiseAmount?: number;
		/**
		 * Amount of wave distortion applied to the rays.
		 * @default 0.0
		 */
		distortion?: number;
		/**
		 * Global ray intensity. Lower values dim and tighten rays, higher values brighten them.
		 * @default 1.0
		 */
		intensity?: number;
	}

	let {
		color = "#FFFFFF",
		backgroundColor = "#000000",
		anchorX = 0.5,
		anchorY = 1.2,
		directionX = 0.0,
		directionY = -1.0,
		speed = 1.0,
		lightSpread = 1.0,
		rayLength = 1.0,
		pulsating = false,
		fadeDistance = 1.0,
		saturation = 1.0,
		noiseAmount = 0.0,
		distortion = 0.0,
		intensity = 1.0,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<{
		uTime: { value: number };
		uResolution: { value: Vec2 };
		uColor: { value: Vec3 };
		uBackgroundColor: { value: Vec3 };
		uAnchorX: { value: number };
		uAnchorY: { value: number };
		uRayDir: { value: Vec2 };
		uSpeed: { value: number };
		uLightSpread: { value: number };
		uRayLength: { value: number };
		uPulsating: { value: number };
		uFadeDistance: { value: number };
		uSaturation: { value: number };
		uNoiseAmount: { value: number };
		uDistortion: { value: number };
		uIntensity: { value: number };
	}>();

	const applyColor = (
		target: Vec3,
		value: ColorRepresentation,
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
		uniform vec2 uResolution;
		uniform vec3 uColor;
		uniform vec3 uBackgroundColor;
		uniform float uAnchorX;
		uniform float uAnchorY;
		uniform vec2 uRayDir;
		uniform float uSpeed;
		uniform float uLightSpread;
		uniform float uRayLength;
		uniform float uPulsating;
		uniform float uFadeDistance;
		uniform float uSaturation;
		uniform float uNoiseAmount;
		uniform float uDistortion;
		uniform float uIntensity;

		float noise2(vec2 st) {
			return fract(sin(dot(st, vec2(12.9898, 78.233))) * 43758.5453123);
		}

		float ditherNoise(vec2 p) {
			return fract(52.9829189 * fract(dot(p, vec2(0.06711056, 0.00583715))));
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
			float tintEnergy = 1.0 - exp(-4.0 * colorLuma(effect));

			vec3 additive = bg + effect;
			vec3 effectHue = hueFromColor(effect, vec3(1.0));
			vec3 tintTarget = mix(bg, effectHue, 0.9);
			vec3 tint = mix(bg, tintTarget, edge * tintEnergy);

			return mix(additive, tint, lightBg);
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		float rayStrength(
			vec2 raySource,
			vec2 rayDir,
			vec2 coord,
			float seedA,
			float seedB,
			float speed,
			float time,
			float maxDim
		) {
			vec2 sourceToCoord = coord - raySource;
			vec2 dirNorm = normalize(sourceToCoord);
			float cosAngle = dot(dirNorm, rayDir);

			float distortedAngle = cosAngle
				+ uDistortion * sin(time * 2.0 + length(sourceToCoord) * 0.01) * 0.2;

			float spreadFactor = pow(max(distortedAngle, 0.0), 1.0 / max(uLightSpread, 0.001));

			float dist = length(sourceToCoord);
			float maxDist = maxDim * uRayLength;
			float lengthFalloff = clamp((maxDist - dist) / maxDist, 0.0, 1.0);

			float fadeFalloff = clamp(
				(maxDim * uFadeDistance - dist) / (maxDim * uFadeDistance),
				0.5, 1.0
			);

			float pulse = (uPulsating > 0.5) ? (0.8 + 0.2 * sin(time * speed * 3.0)) : 1.0;

			float baseStrength = clamp(
				(0.45 + 0.15 * sin(distortedAngle * seedA + time * speed)) +
				(0.3  + 0.2  * cos(-distortedAngle * seedB + time * speed)),
				0.0, 1.0
			);

			return baseStrength * lengthFalloff * fadeFalloff * spreadFactor * pulse;
		}

		void mainImage(out vec4 col, vec2 fragCoord) {
			vec2 resolution = uResolution;
			float time = uTime;

			vec2 coord = fragCoord;
			vec2 rayPos = vec2(uAnchorX, uAnchorY) * resolution;
			vec2 rayDir = normalize(uRayDir);

			float maxDim = length(resolution);

			float rs1 = rayStrength(rayPos, rayDir, coord, 36.2214, 21.11349, 1.5 * uSpeed, time, maxDim);
			float rs2 = rayStrength(rayPos, rayDir, coord, 22.3991, 18.0234, 1.1 * uSpeed, time, maxDim);

			float intensityScale = max(uIntensity, 0.0);
			float intensityForShape = clamp(intensityScale, 0.0, 1.0);
			float shapeExponent = mix(2.35, 1.35, intensityForShape);
			float strength = rs1 * 0.5 + rs2 * 0.4;
			float shapedStrength = pow(clamp(strength, 0.0, 1.0), shapeExponent);
			float softMask = 1.0 - exp(-3.0 * shapedStrength);
			vec3 rayColor = uColor * shapedStrength * intensityScale;

			if (uNoiseAmount > 0.0) {
				float n = noise2(coord * 0.01 + time * 0.1);
				float noiseMix = 1.0 - uNoiseAmount + uNoiseAmount * n;
				rayColor *= noiseMix;
				softMask *= mix(1.0, noiseMix, 0.5);
			}

			vec3 rgb = blendAdaptive(uBackgroundColor, rayColor, softMask);

			if (uSaturation != 1.0) {
				float gray = dot(rgb, vec3(0.299, 0.587, 0.114));
				rgb = mix(vec3(gray), rgb, uSaturation);
			}

			rgb += (ditherNoise(fragCoord + vec2(uTime * 60.0)) - 0.5) / 255.0;
			rgb = clamp(rgb, 0.0, 1.0);

			col = vec4(rgb, 1.0);
		}

		void main() {
			vec4 fragColor;
			vec2 fragCoord = vUv * uResolution.xy;
			mainImage(fragColor, fragCoord);
			fragColor.rgb = linearToSrgb(fragColor.rgb);
			gl_FragColor = fragColor;
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		applyColor(uniforms.uColor.value, color, [1, 1, 1]);
		applyColor(uniforms.uBackgroundColor.value, backgroundColor, [0, 0, 0]);
		uniforms.uAnchorX.value = anchorX;
		uniforms.uAnchorY.value = anchorY;
		uniforms.uRayDir.value.set(directionX, directionY);
		uniforms.uSpeed.value = speed;
		uniforms.uLightSpread.value = lightSpread;
		uniforms.uRayLength.value = rayLength;
		uniforms.uPulsating.value = pulsating ? 1 : 0;
		uniforms.uFadeDistance.value = fadeDistance;
		uniforms.uSaturation.value = saturation;
		uniforms.uNoiseAmount.value = noiseAmount;
		uniforms.uDistortion.value = distortion;
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

		const camera = new Camera(gl);
		camera.position.z = 1;

		const scene = new Transform();
		const geometry = new Triangle(gl);

		const initialColor = toLinearRgb(color, [1, 1, 1]);
		const initialBackground = toLinearRgb(backgroundColor, [0, 0, 0]);

		const localUniforms = {
			uTime: { value: 0.0 },
			uResolution: { value: new Vec2(1, 1) },
			uColor: {
				value: new Vec3(initialColor[0], initialColor[1], initialColor[2]),
			},
			uBackgroundColor: {
				value: new Vec3(
					initialBackground[0],
					initialBackground[1],
					initialBackground[2],
				),
			},
			uAnchorX: { value: anchorX },
			uAnchorY: { value: anchorY },
			uRayDir: { value: new Vec2(directionX, directionY) },
			uSpeed: { value: speed },
			uLightSpread: { value: lightSpread },
			uRayLength: { value: rayLength },
			uPulsating: { value: pulsating ? 1 : 0 },
			uFadeDistance: { value: fadeDistance },
			uSaturation: { value: saturation },
			uNoiseAmount: { value: noiseAmount },
			uDistortion: { value: distortion },
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

		const resize = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));
			renderer.setSize(width, height);
			localUniforms.uResolution.value.set(width, height);
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
			localUniforms.uTime.value += delta;
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
