<script lang="ts">
	import { onDestroy, onMount } from "svelte";
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
	import { gsap } from "gsap/dist/gsap";

	interface Props {
		/** Array of image URLs used for textures. */
		images: string[];
		/** Index of the currently active image. */
		index?: number;
		/** Duration of a single transition in milliseconds. */
		transitionDuration?: number;
		/** Global intensity multiplier for the shader effect. */
		intensity?: number;
		/** Distortion strength applied during transitions. */
		distortion?: number;
		/** Chromatic aberration strength for the shader. */
		chromaticAberration?: number;
		/** Refraction strength for the shader. */
		refraction?: number;
	}

	let {
		images,
		index = 0,
		transitionDuration = 2000,
		intensity = 1.0,
		distortion = 1.0,
		chromaticAberration = 1.0,
		refraction = 1.0,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();

	let progress = $state({ value: 0 });
	let currentIndex = $state(0);
	let nextIndex = $state(0);
	let isTransitioning = $state(false);

	let setImageSources = $state<(sources: string[]) => void>();
	let setUniformParams =
		$state<
			(next: {
				intensity: number;
				distortion: number;
				chromaticAberration: number;
				refraction: number;
			}) => void
		>();

	onDestroy(() => {
		gsap.killTweensOf(progress);
	});

	$effect(() => {
		const totalImages = images.length;

		if (totalImages === 0) {
			if (currentIndex !== 0) currentIndex = 0;
			if (nextIndex !== 0) nextIndex = 0;
			isTransitioning = false;
			progress.value = 0;
			gsap.killTweensOf(progress);
			return;
		}

		const normalizedCurrent =
			((currentIndex % totalImages) + totalImages) % totalImages;
		const normalizedNext =
			((nextIndex % totalImages) + totalImages) % totalImages;
		if (normalizedCurrent !== currentIndex) currentIndex = normalizedCurrent;
		if (normalizedNext !== nextIndex) nextIndex = normalizedNext;
	});

	$effect(() => {
		const totalImages = images.length;
		if (totalImages === 0) return;

		const normalizedIndex = ((index % totalImages) + totalImages) % totalImages;

		if (normalizedIndex === currentIndex || isTransitioning) {
			return;
		}

		gsap.killTweensOf(progress);
		progress.value = 0;
		isTransitioning = true;
		nextIndex = normalizedIndex;

		gsap.to(progress, {
			value: 1,
			duration: transitionDuration / 1000,
			ease: "power3.inOut",
			onComplete: () => {
				currentIndex = nextIndex;
				progress.value = 0;
				isTransitioning = false;
			},
		});
	});

	$effect(() => {
		if (!setImageSources) return;
		setImageSources(images);
	});

	$effect(() => {
		if (!setUniformParams) return;
		setUniformParams({
			intensity,
			distortion,
			chromaticAberration,
			refraction,
		});
	});

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

		uniform sampler2D uTexture1;
		uniform sampler2D uTexture2;
		uniform float uProgress;
		uniform vec2 uResolution;
		uniform vec2 uTexture1Size;
		uniform vec2 uTexture2Size;

		uniform float uGlobalIntensity;
		uniform float uDistortionStrength;
		uniform float uSpeedMultiplier;
		uniform float uColorEnhancement;

		uniform float uGlassRefractionStrength;
		uniform float uGlassChromaticAberration;
		uniform float uGlassBubbleClarity;
		uniform float uGlassEdgeGlow;
		uniform float uGlassLiquidFlow;

		varying vec2 vUv;

		vec3 srgbToLinear(vec3 color) {
			vec3 low = color / 12.92;
			vec3 high = pow((color + 0.055) / 1.055, vec3(2.4));
			vec3 cutoff = step(vec3(0.04045), color);
			return mix(low, high, cutoff);
		}

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 s = uResolution / textureSize;
			float scale = max(s.x, s.y);
			vec2 scaledSize = textureSize * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		float noise(vec2 p) {
			return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
		}

		float smoothNoise(vec2 p) {
			vec2 i = floor(p);
			vec2 f = fract(p);
			f = f * f * (3.0 - 2.0 * f);

			return mix(
				mix(noise(i), noise(i + vec2(1.0, 0.0)), f.x),
				mix(noise(i + vec2(0.0, 1.0)), noise(i + vec2(1.0, 1.0)), f.x),
				f.y
			);
		}

		vec4 sampleLinear(sampler2D tex, vec2 uv) {
			vec4 c = texture2D(tex, uv);
			return vec4(srgbToLinear(c.rgb), c.a);
		}

		vec4 glassEffect(vec2 uv, float progress) {
			float glassStrength = 0.08 * uGlassRefractionStrength * uDistortionStrength * uGlobalIntensity;
			float chromaticAberration = 0.02 * uGlassChromaticAberration * uGlobalIntensity;
			float waveDistortion = 0.025 * uDistortionStrength;
			float clearCenterSize = 0.3 * uGlassBubbleClarity;
			float surfaceRipples = 0.004 * uDistortionStrength;
			float liquidFlow = 0.015 * uGlassLiquidFlow * uSpeedMultiplier;
			float rimLightWidth = 0.05;
			float glassEdgeWidth = 0.025;

			float brightnessPhase = smoothstep(0.8, 1.0, progress);
			float rimLightIntensity = 0.08 * (1.0 - brightnessPhase) * uGlassEdgeGlow * uGlobalIntensity;
			float glassEdgeOpacity = 0.06 * (1.0 - brightnessPhase) * uGlassEdgeGlow;

			vec2 center = vec2(0.5, 0.5);
			vec2 p = uv * uResolution;

			vec2 uv1 = getCoverUV(uv, uTexture1Size);
			vec2 uv2_base = getCoverUV(uv, uTexture2Size);

			float maxRadius = length(uResolution) * 0.85;
			float bubbleRadius = progress * maxRadius;
			vec2 sphereCenter = center * uResolution;

			float dist = length(p - sphereCenter);
			float normalizedDist = dist / max(bubbleRadius, 0.001);
			vec2 direction = (dist > 0.0) ? (p - sphereCenter) / dist : vec2(0.0);
			float inside = smoothstep(bubbleRadius + 3.0, bubbleRadius - 3.0, dist);

			float distanceFactor = smoothstep(clearCenterSize, 1.0, normalizedDist);
			float time = progress * 5.0 * uSpeedMultiplier;

			vec2 liquidSurface = vec2(
				smoothNoise(uv * 100.0 + time * 0.3),
				smoothNoise(uv * 100.0 + time * 0.2 + 50.0)
			) - 0.5;
			liquidSurface *= surfaceRipples * distanceFactor;

			vec2 distortedUV = uv2_base;
			if (inside > 0.0) {
				float refractionOffset = glassStrength * pow(distanceFactor, 1.5);
				vec2 flowDirection = normalize(direction + vec2(sin(time), cos(time * 0.7)) * 0.3);
				distortedUV -= flowDirection * refractionOffset;

				float wave1 = sin(normalizedDist * 22.0 - time * 3.5);
				float wave2 = sin(normalizedDist * 35.0 + time * 2.8) * 0.7;
				float wave3 = sin(normalizedDist * 50.0 - time * 4.2) * 0.5;
				float combinedWave = (wave1 + wave2 + wave3) / 3.0;

				float waveOffset = combinedWave * waveDistortion * distanceFactor;
				distortedUV -= direction * waveOffset + liquidSurface;

				vec2 flowOffset = vec2(
					sin(time + normalizedDist * 10.0),
					cos(time * 0.8 + normalizedDist * 8.0)
				) * liquidFlow * distanceFactor * inside;
				distortedUV += flowOffset;
			}

			vec4 newImg;
			if (inside > 0.0) {
				float aberrationOffset = chromaticAberration * pow(distanceFactor, 1.2);

				vec2 uv_r = distortedUV + direction * aberrationOffset * 1.2;
				vec2 uv_g = distortedUV + direction * aberrationOffset * 0.2;
				vec2 uv_b = distortedUV - direction * aberrationOffset * 0.8;

				vec3 sampleR = srgbToLinear(texture2D(uTexture2, uv_r).rgb);
				vec3 sampleG = srgbToLinear(texture2D(uTexture2, uv_g).rgb);
				vec3 sampleB = srgbToLinear(texture2D(uTexture2, uv_b).rgb);
				newImg = vec4(sampleR.r, sampleG.g, sampleB.b, 1.0);
			} else {
				newImg = sampleLinear(uTexture2, uv2_base);
			}

			if (inside > 0.0 && rimLightIntensity > 0.0) {
				float rim = smoothstep(1.0 - rimLightWidth, 1.0, normalizedDist) *
							(1.0 - smoothstep(1.0, 1.01, normalizedDist));
				newImg.rgb += rim * rimLightIntensity;

				float edge = smoothstep(1.0 - glassEdgeWidth, 1.0, normalizedDist) *
							 (1.0 - smoothstep(1.0, 1.01, normalizedDist));
				newImg.rgb = mix(newImg.rgb, vec3(1.0), edge * glassEdgeOpacity);
			}

			newImg.rgb = mix(newImg.rgb, newImg.rgb * 1.2, (uColorEnhancement - 1.0) * 0.5);

			vec4 currentImg = sampleLinear(uTexture1, uv1);

			if (progress > 0.95) {
				vec4 pureNewImg = sampleLinear(uTexture2, uv2_base);
				float endTransition = (progress - 0.95) / 0.05;
				newImg = mix(newImg, pureNewImg, endTransition);
			}

			return mix(currentImg, newImg, inside);
		}

		void main() {
			vec4 outColor = glassEffect(vUv, uProgress);
			gl_FragColor = vec4(linearToSrgb(outColor.rgb), outColor.a);
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

		const camera = new Camera(gl);
		camera.position.z = 1;

		const scene = new Transform();
		const geometry = new Triangle(gl);

		const createPlaceholderTexture = () =>
			new Texture(gl, {
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

		const placeholderTexture = createPlaceholderTexture();
		let slideTextures: Texture[] = [];
		let imageLoadToken = 0;

		const disposeTexture = (texture: Texture) => {
			if (texture.texture) gl.deleteTexture(texture.texture);
		};

		const loadTextureFromSource = (source: string, token: number) => {
			const texture = createPlaceholderTexture();
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (token !== imageLoadToken) return;
				texture.image = img;
			};
			img.src = source;
			return texture;
		};

		const replaceTextures = (sources: string[]) => {
			imageLoadToken += 1;
			const token = imageLoadToken;
			slideTextures.forEach(disposeTexture);
			slideTextures = sources.map((source) =>
				loadTextureFromSource(source, token),
			);
		};
		setImageSources = replaceTextures;
		replaceTextures(images);

		const getTextureSize = (texture: Texture): [number, number] => {
			const image = texture.image as
				| {
						width?: number;
						height?: number;
						naturalWidth?: number;
						naturalHeight?: number;
				  }
				| null
				| undefined;
			if (!image) return [1, 1];
			const width = image.naturalWidth ?? image.width ?? 1;
			const height = image.naturalHeight ?? image.height ?? 1;
			return [Math.max(1, width), Math.max(1, height)];
		};

		const localUniforms = {
			uTexture1: { value: placeholderTexture },
			uTexture2: { value: placeholderTexture },
			uProgress: { value: 0 },
			uResolution: { value: new Vec2(1, 1) },
			uTexture1Size: { value: new Vec2(1, 1) },
			uTexture2Size: { value: new Vec2(1, 1) },
			uGlobalIntensity: { value: intensity },
			uDistortionStrength: { value: distortion },
			uSpeedMultiplier: { value: 1.0 },
			uColorEnhancement: { value: 1.0 },
			uGlassRefractionStrength: { value: refraction },
			uGlassChromaticAberration: { value: chromaticAberration },
			uGlassBubbleClarity: { value: 1.0 },
			uGlassEdgeGlow: { value: 1.0 },
			uGlassLiquidFlow: { value: 1.0 },
		};

		setUniformParams = (next) => {
			localUniforms.uGlobalIntensity.value = next.intensity;
			localUniforms.uDistortionStrength.value = next.distortion;
			localUniforms.uGlassChromaticAberration.value = next.chromaticAberration;
			localUniforms.uGlassRefractionStrength.value = next.refraction;
		};
		setUniformParams({
			intensity,
			distortion,
			chromaticAberration,
			refraction,
		});

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: localUniforms,
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
			localUniforms.uResolution.value.set(width, height);
		};

		resize();
		const observer = new ResizeObserver(resize);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		let raf = 0;
		const tick = () => {
			const total = slideTextures.length;
			const safeCurrent =
				total > 0 ? ((currentIndex % total) + total) % total : 0;
			const safeNext =
				total > 0 ? ((nextIndex % total) + total) % total : safeCurrent;

			const tex1 = total > 0 ? slideTextures[safeCurrent] : placeholderTexture;
			const tex2 = total > 0 ? slideTextures[safeNext] : placeholderTexture;

			localUniforms.uProgress.value = progress.value;
			localUniforms.uTexture1.value = tex1;
			localUniforms.uTexture2.value = tex2;

			const [w1, h1] = getTextureSize(tex1);
			const [w2, h2] = getTextureSize(tex2);
			localUniforms.uTexture1Size.value.set(w1, h1);
			localUniforms.uTexture2Size.value.set(w2, h2);

			renderer.render({ scene, camera, clear: true });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			setImageSources = undefined;
			setUniformParams = undefined;
			imageLoadToken += 1;

			slideTextures.forEach(disposeTexture);
			disposeTexture(placeholderTexture);

			program.remove();
			geometry.remove();
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
