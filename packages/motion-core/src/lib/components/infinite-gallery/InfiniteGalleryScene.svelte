<script lang="ts">
	import { useTask } from "@threlte/core";
	import * as THREE from "three";
	import { SRGBColorSpace } from "three";
	import { useTexture } from "@threlte/extras";
	import ImagePlane from "./ImagePlane.svelte";

	type ImageItem = string | { src: string; alt?: string };

	interface Props {
		/**
		 * Array of images to display. Can be strings (URL) or objects with src and alt.
		 */
		images: ImageItem[];
		/**
		 * Scroll speed multiplier.
		 * @default 1
		 */
		speed?: number;
		/**
		 * Number of images visible in the tunnel at once.
		 * @default 8
		 */
		visibleCount?: number;
		/**
		 * Configuration for fade in/out effects based on depth.
		 */
		fadeSettings?: {
			fadeIn: { start: number; end: number };
			fadeOut: { start: number; end: number };
		};
		/**
		 * Configuration for blur in/out effects based on depth.
		 */
		blurSettings?: {
			blurIn: { start: number; end: number };
			blurOut: { start: number; end: number };
			maxBlur: number;
		};
	}

	let {
		images,
		speed = 1,
		visibleCount = 8,
		fadeSettings = {
			fadeIn: { start: 0.05, end: 0.15 },
			fadeOut: { start: 0.85, end: 0.95 },
		},
		blurSettings = {
			blurIn: { start: 0.0, end: 0.1 },
			blurOut: { start: 0.9, end: 1.0 },
			maxBlur: 3.0,
		},
	}: Props = $props();

	const DEFAULT_DEPTH_RANGE = 50;
	const MAX_HORIZONTAL_OFFSET = 8;
	const MAX_VERTICAL_OFFSET = 8;

	const vertexShader = `
uniform float scrollForce;
uniform float time;
uniform float isHovered;
varying vec2 vUv;
varying vec3 vNormal;

void main() {
    vUv = uv;
    vNormal = normal;

    vec3 pos = position;

    float curveIntensity = scrollForce * 0.3;

    float distanceFromCenter = length(pos.xy);
    float curve = distanceFromCenter * distanceFromCenter * curveIntensity;

    float ripple1 = sin(pos.x * 2.0 + scrollForce * 3.0) * 0.02;
    float ripple2 = sin(pos.y * 2.5 + scrollForce * 2.0) * 0.015;
    float clothEffect = (ripple1 + ripple2) * abs(curveIntensity) * 2.0;

    float flagWave = 0.0;
    if (isHovered > 0.5) {
        float wavePhase = pos.x * 3.0 + time * 8.0;
        float waveAmplitude = sin(wavePhase) * 0.1;
        float dampening = smoothstep(-0.5, 0.5, pos.x);
        flagWave = waveAmplitude * dampening;

        float secondaryWave = sin(pos.x * 5.0 + time * 12.0) * 0.03 * dampening;
        flagWave += secondaryWave;
    }

    pos.z -= (curve + clothEffect + flagWave);

    gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
}
`;

	const fragmentShader = `
uniform sampler2D map;
uniform float opacity;
uniform float blurAmount;
uniform float scrollForce;
varying vec2 vUv;
varying vec3 vNormal;

void main() {
    vec4 color = texture2D(map, vUv);

    if (blurAmount > 0.0) {
        vec2 texelSize = 1.0 / vec2(textureSize(map, 0));
        vec4 blurred = vec4(0.0);
        float total = 0.0;

        for (float x = -2.0; x <= 2.0; x += 1.0) {
            for (float y = -2.0; y <= 2.0; y += 1.0) {
                vec2 offset = vec2(x, y) * texelSize * blurAmount;
                float weight = 1.0 / (1.0 + length(vec2(x, y)));
                blurred += texture2D(map, vUv + offset) * weight;
                total += weight;
            }
        }
        color = blurred / total;
    }

    float curveHighlight = abs(scrollForce) * 0.05;
    color.rgb += vec3(curveHighlight * 0.1);

    gl_FragColor = vec4(color.rgb, color.a * opacity);
    #include <colorspace_fragment>
}
`;

	const normalizedImages = $derived(
		images.map((img) =>
			typeof img === "string" ? { src: img, alt: "" } : img,
		),
	);

	const textureUrls = $derived(normalizedImages.map((img) => img.src));

	const textures = $derived(
		useTexture(textureUrls, {
			transform: (tex) => {
				tex.colorSpace = SRGBColorSpace;
				return tex;
			},
		}),
	);

	const materials = $derived.by(() => {
		return Array.from(
			{ length: visibleCount },
			() =>
				new THREE.ShaderMaterial({
					transparent: true,
					uniforms: {
						map: { value: null },
						opacity: { value: 1.0 },
						blurAmount: { value: 0.0 },
						scrollForce: { value: 0.0 },
						time: { value: 0.0 },
						isHovered: { value: 0.0 },
					},
					vertexShader,
					fragmentShader,
				}),
		);
	});

	const spatialPositions = $derived.by(() => {
		const positions: { x: number; y: number }[] = [];
		const maxHorizontalOffset = MAX_HORIZONTAL_OFFSET;
		const maxVerticalOffset = MAX_VERTICAL_OFFSET;

		for (let i = 0; i < visibleCount; i++) {
			const horizontalAngle = (i * 2.618) % (Math.PI * 2);
			const verticalAngle = (i * 1.618 + Math.PI / 3) % (Math.PI * 2);

			const horizontalRadius = (i % 3) * 1.2;
			const verticalRadius = ((i + 1) % 4) * 0.8;

			const x =
				(Math.sin(horizontalAngle) * horizontalRadius * maxHorizontalOffset) /
				3;
			const y =
				(Math.cos(verticalAngle) * verticalRadius * maxVerticalOffset) / 4;

			positions.push({ x, y });
		}
		return positions;
	});

	let scrollVelocity = 0;
	let autoPlay = true;
	let lastInteraction = Date.now();
	let clock = new THREE.Clock();

	type PlaneData = {
		index: number;
		z: number;
		imageIndex: number;
		x: number;
		y: number;
	};

	const totalImages = $derived(normalizedImages.length);
	const depthRange = DEFAULT_DEPTH_RANGE;

	// eslint-disable-next-line svelte/prefer-writable-derived
	let planesData: PlaneData[] = $state([]);

	$effect(() => {
		planesData = Array.from({ length: visibleCount }, (_, i) => ({
			index: i,
			z: visibleCount > 0 ? ((depthRange / visibleCount) * i) % depthRange : 0,
			imageIndex: totalImages > 0 ? i % totalImages : 0,
			x: spatialPositions[i]?.x ?? 0,
			y: spatialPositions[i]?.y ?? 0,
		}));
	});

	const handleWheel = (event: WheelEvent) => {
		event.preventDefault();
		scrollVelocity += event.deltaY * 0.01 * speed;
		autoPlay = false;
		lastInteraction = Date.now();
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
			scrollVelocity -= 2 * speed;
			autoPlay = false;
			lastInteraction = Date.now();
		} else if (event.key === "ArrowDown" || event.key === "ArrowRight") {
			scrollVelocity += 2 * speed;
			autoPlay = false;
			lastInteraction = Date.now();
		}
	};

	$effect(() => {
		const canvas = document.querySelector("canvas");
		if (canvas) {
			canvas.addEventListener("wheel", handleWheel, { passive: false });
		}
		window.addEventListener("keydown", handleKeyDown);

		return () => {
			if (canvas) canvas.removeEventListener("wheel", handleWheel);
			window.removeEventListener("keydown", handleKeyDown);
		};
	});

	$effect(() => {
		const interval = setInterval(() => {
			if (Date.now() - lastInteraction > 3000) {
				autoPlay = true;
			}
		}, 1000);
		return () => clearInterval(interval);
	});

	useTask((delta) => {
		if (autoPlay) {
			scrollVelocity += 0.3 * delta;
		}

		scrollVelocity *= 0.95;

		const time = clock.getElapsedTime();

		materials.forEach((material) => {
			if (material && material.uniforms) {
				material.uniforms.time.value = time;
				material.uniforms.scrollForce.value = scrollVelocity;
			}
		});

		const imageAdvance =
			totalImages > 0 ? visibleCount % totalImages || totalImages : 0;
		const totalRange = depthRange;

		for (let i = 0; i < planesData.length; i++) {
			const plane = planesData[i];
			let newZ = plane.z + scrollVelocity * delta * 10;
			let wrapsForward = 0;
			let wrapsBackward = 0;

			if (newZ >= totalRange) {
				wrapsForward = Math.floor(newZ / totalRange);
				newZ -= totalRange * wrapsForward;
			} else if (newZ < 0) {
				wrapsBackward = Math.ceil(-newZ / totalRange);
				newZ += totalRange * wrapsBackward;
			}

			if (wrapsForward > 0 && imageAdvance > 0 && totalImages > 0) {
				plane.imageIndex =
					(plane.imageIndex + wrapsForward * imageAdvance) % totalImages;
			}

			if (wrapsBackward > 0 && imageAdvance > 0 && totalImages > 0) {
				const step = plane.imageIndex - wrapsBackward * imageAdvance;
				plane.imageIndex = ((step % totalImages) + totalImages) % totalImages;
			}

			plane.z = ((newZ % totalRange) + totalRange) % totalRange;
			plane.x = spatialPositions[i]?.x ?? 0;
			plane.y = spatialPositions[i]?.y ?? 0;

			const normalizedPosition = plane.z / totalRange;
			let opacity = 1;

			if (
				normalizedPosition >= fadeSettings.fadeIn.start &&
				normalizedPosition <= fadeSettings.fadeIn.end
			) {
				const fadeInProgress =
					(normalizedPosition - fadeSettings.fadeIn.start) /
					(fadeSettings.fadeIn.end - fadeSettings.fadeIn.start);
				opacity = fadeInProgress;
			} else if (normalizedPosition < fadeSettings.fadeIn.start) {
				opacity = 0;
			} else if (
				normalizedPosition >= fadeSettings.fadeOut.start &&
				normalizedPosition <= fadeSettings.fadeOut.end
			) {
				const fadeOutProgress =
					(normalizedPosition - fadeSettings.fadeOut.start) /
					(fadeSettings.fadeOut.end - fadeSettings.fadeOut.start);
				opacity = 1 - fadeOutProgress;
			} else if (normalizedPosition > fadeSettings.fadeOut.end) {
				opacity = 0;
			}

			opacity = Math.max(0, Math.min(1, opacity));

			let blur = 0;

			if (
				normalizedPosition >= blurSettings.blurIn.start &&
				normalizedPosition <= blurSettings.blurIn.end
			) {
				const blurInProgress =
					(normalizedPosition - blurSettings.blurIn.start) /
					(blurSettings.blurIn.end - blurSettings.blurIn.start);
				blur = blurSettings.maxBlur * (1 - blurInProgress);
			} else if (normalizedPosition < blurSettings.blurIn.start) {
				blur = blurSettings.maxBlur;
			} else if (
				normalizedPosition >= blurSettings.blurOut.start &&
				normalizedPosition <= blurSettings.blurOut.end
			) {
				const blurOutProgress =
					(normalizedPosition - blurSettings.blurOut.start) /
					(blurSettings.blurOut.end - blurSettings.blurOut.start);
				blur = blurSettings.maxBlur * blurOutProgress;
			} else if (normalizedPosition > blurSettings.blurOut.end) {
				blur = blurSettings.maxBlur;
			}

			blur = Math.max(0, Math.min(blurSettings.maxBlur, blur));

			const material = materials[i];
			if (material && material.uniforms) {
				material.uniforms.opacity.value = opacity;
				material.uniforms.blurAmount.value = blur;
			}
		}
	});
</script>

{#if $textures}
	{#each planesData as plane, i (plane.index)}
		{@const texture = $textures[plane.imageIndex]}
		{@const material = materials[i]}
		{@const worldZ = plane.z - depthRange / 2}

		{#if texture && material}
			{@const aspect = texture.image
				? texture.image.width / texture.image.height
				: 1}
			{@const scale = aspect > 1 ? [2 * aspect, 2, 1] : [2, 2 / aspect, 1]}

			<ImagePlane
				{texture}
				{material}
				position={[plane.x, plane.y, worldZ]}
				scale={scale as [number, number, number]}
			/>
		{/if}
	{/each}
{/if}
