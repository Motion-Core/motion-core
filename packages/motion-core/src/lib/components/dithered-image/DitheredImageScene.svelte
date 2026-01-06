<script lang="ts">
	import { T, useThrelte } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import * as THREE from "three";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Type of dithering map to use.
		 * @default "bayer4x4"
		 */
		ditherMap?: "bayer4x4" | "bayer8x8" | "halftone" | "voidAndCluster";
		/**
		 * Pixel size of the dithering effect.
		 * @default 1
		 */
		pixelSize?: number;
		/**
		 * Foreground color (dots).
		 * @default "#ff6900"
		 */
		color?: string;
		/**
		 * Background color.
		 * @default "#111113"
		 */
		backgroundColor?: string;
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

	const { size, dpr } = useThrelte();

	const thresholdMapsData: Record<string, number[]> = {
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

	let imageWidth = 1;
	let imageHeight = 1;

	const tex = $derived(
		useTexture(image, {
			transform: (t) => {
				t.colorSpace = THREE.SRGBColorSpace;
				return t;
			},
		}),
	);

	$effect(() => {
		if ($tex && $tex.image) {
			imageWidth = $tex.image.width;
			imageHeight = $tex.image.height;
			updateCoverUniforms();
		}
	});

	let thresholdTexture = $state<THREE.DataTexture | null>(null);
	let mapSizeUniform = $state(new THREE.Vector2(4, 4));

	$effect(() => {
		const data = thresholdMapsData[ditherMap] || thresholdMapsData["bayer4x4"];
		const size = Math.sqrt(data.length);
		const count = data.length;

		const floatData = new Float32Array(count);
		for (let i = 0; i < count; i++) {
			floatData[i] = data[i] / count;
		}

		const texture = new THREE.DataTexture(
			floatData,
			size,
			size,
			THREE.RedFormat,
			THREE.FloatType,
		);
		texture.minFilter = THREE.NearestFilter;
		texture.magFilter = THREE.NearestFilter;
		texture.wrapS = THREE.RepeatWrapping;
		texture.wrapT = THREE.RepeatWrapping;
		texture.needsUpdate = true;

		thresholdTexture = texture;
		mapSizeUniform.set(size, size);
	});

	const resolutionUniform = new THREE.Vector2(1, 1);
	const coverScaleUniform = new THREE.Vector2(1, 1);
	const coverOffsetUniform = new THREE.Vector2(0, 0);
	const colorUniform = new THREE.Color();
	const backgroundColorUniform = new THREE.Color();

	const updateCoverUniforms = () => {
		const screenAspect = $size.width / $size.height;
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

	$effect(() => {
		resolutionUniform.set($size.width * $dpr, $size.height * $dpr);
		updateCoverUniforms();
	});

	$effect(() => {
		colorUniform.set(color);
		backgroundColorUniform.set(backgroundColor);
	});

	const vertexShader = `
		varying vec2 vUv;
		void main() {
			vUv = uv;
			gl_Position = vec4(position, 1.0);
		}
	`;

	const fragmentShader = `
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

		float getLuminance(vec3 color) {
			return dot(color, vec3(0.299, 0.587, 0.114));
		}

		void main() {
			vec2 pixelCoord = floor(gl_FragCoord.xy / uPixelSize);

			vec2 centerPixel = pixelCoord * uPixelSize + (uPixelSize * 0.5);
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

			gl_FragColor = vec4(ditheredColor, 1.0);
			#include <colorspace_fragment>
		}
	`;
</script>

{#if $tex && thresholdTexture}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			{vertexShader}
			{fragmentShader}
			uniforms={{
				uTexture: { value: $tex },
				uThresholdMap: { value: thresholdTexture },
				uResolution: { value: resolutionUniform },
				uMapSize: { value: mapSizeUniform },
				uCoverScale: { value: coverScaleUniform },
				uCoverOffset: { value: coverOffsetUniform },
				uPixelSize: { value: pixelSize },
				uThreshold: { value: threshold },
				uColor: { value: colorUniform },
				uBackgroundColor: { value: backgroundColorUniform },
			}}
		/>
	</T.Mesh>
{/if}
