<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import {
		Vector2,
		ShaderMaterial,
		LinearFilter,
		ClampToEdgeWrapping,
		SRGBColorSpace,
	} from "three";
	import { tweened } from "svelte/motion";
	import { cubicInOut } from "svelte/easing";

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

	const { size } = useThrelte();

	let material = $state<ShaderMaterial>();
	const progress = tweened(0, {
		duration: 0,
		easing: cubicInOut,
	});

	const textures = $derived(
		useTexture(images, {
			transform: (tex) => {
				tex.minFilter = LinearFilter;
				tex.magFilter = LinearFilter;
				tex.wrapS = ClampToEdgeWrapping;
				tex.wrapT = ClampToEdgeWrapping;
				tex.colorSpace = SRGBColorSpace;
				return tex;
			},
		}),
	);

	let currentIndex = $state(0);
	let nextIndex = $state(0);
	let isTransitioning = $state(false);

	$effect(() => {
		const totalImages = images.length;

		if (totalImages === 0) {
			if (currentIndex !== 0) {
				currentIndex = 0;
			}
			if (nextIndex !== 0) {
				nextIndex = 0;
			}
			isTransitioning = false;
			return;
		}

		const normalizedIndex = ((index % totalImages) + totalImages) % totalImages;

		if (normalizedIndex === currentIndex || isTransitioning) {
			return;
		}

		isTransitioning = true;
		nextIndex = normalizedIndex;

		progress.set(1, { duration: transitionDuration }).then(() => {
			currentIndex = nextIndex;
			progress.set(0, { duration: 0 });
			isTransitioning = false;
		});
	});

	const vertexShader = `
        varying vec2 vUv;
        void main() {
            vUv = uv;
            gl_Position = vec4(position, 1.0);
        }
    `;

	const fragmentShader = `
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

                float r = texture2D(uTexture2, uv_r).r;
                float g = texture2D(uTexture2, uv_g).g;
                float b = texture2D(uTexture2, uv_b).b;
                newImg = vec4(r, g, b, 1.0);
            } else {
                newImg = texture2D(uTexture2, uv2_base);
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

            vec4 currentImg = texture2D(uTexture1, uv1);

            if (progress > 0.95) {
                vec4 pureNewImg = texture2D(uTexture2, uv2_base);
                float endTransition = (progress - 0.95) / 0.05;
                newImg = mix(newImg, pureNewImg, endTransition);
            }

            return mix(currentImg, newImg, inside);
        }

        void main() {
            gl_FragColor = glassEffect(vUv, uProgress);
            #include <colorspace_fragment>
        }
    `;

	useTask(() => {
		if (material && $textures) {
			const tex1 = $textures[currentIndex];
			const tex2 = $textures[nextIndex];

			if (tex1 && tex2) {
				material.uniforms.uResolution.value.set($size.width, $size.height);
				material.uniforms.uProgress.value = $progress;
				material.uniforms.uTexture1.value = tex1;
				material.uniforms.uTexture2.value = tex2;

				if (tex1.image) {
					material.uniforms.uTexture1Size.value.set(
						tex1.image.width,
						tex1.image.height,
					);
				}
				if (tex2.image) {
					material.uniforms.uTexture2Size.value.set(
						tex2.image.width,
						tex2.image.height,
					);
				}
			}
		}
	});
</script>

{#if $textures}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={material}
			{vertexShader}
			{fragmentShader}
			uniforms={{
				uTexture1: { value: null },
				uTexture2: { value: null },
				uProgress: { value: 0 },
				uResolution: { value: new Vector2(1, 1) },
				uTexture1Size: { value: new Vector2(1, 1) },
				uTexture2Size: { value: new Vector2(1, 1) },
				uGlobalIntensity: { value: intensity },
				uDistortionStrength: { value: distortion },
				uSpeedMultiplier: { value: 1.0 },
				uColorEnhancement: { value: 1.0 },
				uGlassRefractionStrength: { value: refraction },
				uGlassChromaticAberration: { value: chromaticAberration },
				uGlassBubbleClarity: { value: 1.0 },
				uGlassEdgeGlow: { value: 1.0 },
				uGlassLiquidFlow: { value: 1.0 },
			}}
		/>
	</T.Mesh>
{/if}
