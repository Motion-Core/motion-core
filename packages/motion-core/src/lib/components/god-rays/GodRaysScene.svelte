<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		/**
		 * Base color of the rays.
		 * @default "#FFFFFF"
		 */
		color?: THREE.ColorRepresentation;
		/**
		 * Color of the background.
		 * @default "#000000"
		 */
		backgroundColor?: THREE.ColorRepresentation;
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
	}: Props = $props();

	let material = $state<THREE.ShaderMaterial>();
	const { size } = useThrelte();
	const resolutionUniform = new THREE.Vector2(1, 1);
	const primaryColorUniform = new THREE.Color();
	const backgroundColorUniform = new THREE.Color();

	const vertexShader = `
		varying vec2 vUv;
		void main() {
			vUv = uv;
			gl_Position = vec4(position, 1.0);
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
		uniform bool uPulsating;
		uniform float uFadeDistance;
		uniform float uSaturation;
		uniform float uNoiseAmount;
		uniform float uDistortion;

		float noise2(vec2 st) {
			return fract(sin(dot(st, vec2(12.9898, 78.233))) * 43758.5453123);
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

			float pulse = uPulsating ? (0.8 + 0.2 * sin(time * speed * 3.0)) : 1.0;

			float baseStrength = clamp(
				(0.45 + 0.15 * sin(distortedAngle * seedA + time * speed)) +
				(0.3  + 0.2  * cos(-distortedAngle * seedB + time * speed)),
				0.0, 1.0
			);

			return baseStrength * lengthFalloff * fadeFalloff * spreadFactor * pulse;
		}

		void mainImage(out vec4 col, vec2 fragCoord) {
			vec2 resolution = uResolution;
			float time       = uTime;

			vec2 coord  = fragCoord;
			vec2 rayPos = vec2(uAnchorX, uAnchorY) * resolution;
			vec2 rayDir = normalize(uRayDir);

			float maxDim = length(resolution);

			float rs1 = rayStrength(rayPos, rayDir, coord, 36.2214, 21.11349, 1.5 * uSpeed, time, maxDim);
			float rs2 = rayStrength(rayPos, rayDir, coord, 22.3991, 18.0234,  1.1 * uSpeed, time, maxDim);

			float strength = rs1 * 0.5 + rs2 * 0.4;

			vec3 rgb = mix(uBackgroundColor, uColor, clamp(strength, 0.0, 1.0));

			if (uNoiseAmount > 0.0) {
				float n = noise2(coord * 0.01 + time * 0.1);
				rgb *= (1.0 - uNoiseAmount + uNoiseAmount * n);
			}

			if (uSaturation != 1.0) {
				float gray = dot(rgb, vec3(0.299, 0.587, 0.114));
				rgb = mix(vec3(gray), rgb, uSaturation);
			}

			col = vec4(rgb, 1.0);
		}

		void main() {
			vec4 fragColor;
			vec2 fragCoord = vUv * uResolution.xy;
			mainImage(fragColor, fragCoord);
			gl_FragColor = fragColor;
		}
	`;

	$effect(() => {
		resolutionUniform.set($size.width, $size.height);
	});

	$effect(() => {
		primaryColorUniform.set(color);
		backgroundColorUniform.set(backgroundColor);

		if (!material) return;
		material.uniforms.uColor.value.copy(primaryColorUniform);
		material.uniforms.uBackgroundColor.value.copy(backgroundColorUniform);
		material.uniforms.uAnchorX.value = anchorX;
		material.uniforms.uAnchorY.value = anchorY;
		material.uniforms.uRayDir.value.set(directionX, directionY);
		material.uniforms.uSpeed.value = speed;
		material.uniforms.uLightSpread.value = lightSpread;
		material.uniforms.uRayLength.value = rayLength;
		material.uniforms.uPulsating.value = pulsating;
		material.uniforms.uFadeDistance.value = fadeDistance;
		material.uniforms.uSaturation.value = saturation;
		material.uniforms.uNoiseAmount.value = noiseAmount;
		material.uniforms.uDistortion.value = distortion;
	});

	useTask((delta) => {
		if (!material) return;
		material.uniforms.uTime.value += delta;
	});
</script>

<T.Mesh>
	<T.PlaneGeometry args={[2, 2]} />
	<T.ShaderMaterial
		bind:ref={material}
		{vertexShader}
		{fragmentShader}
		depthTest={false}
		depthWrite={false}
		uniforms={{
			uTime: { value: 0.0 },
			uResolution: { value: resolutionUniform },
			uColor: { value: primaryColorUniform },
			uBackgroundColor: { value: backgroundColorUniform },
			uAnchorX: { value: anchorX },
			uAnchorY: { value: anchorY },
			uRayDir: { value: new THREE.Vector2(directionX, directionY) },
			uSpeed: { value: speed },
			uLightSpread: { value: lightSpread },
			uRayLength: { value: rayLength },
			uPulsating: { value: pulsating },
			uFadeDistance: { value: fadeDistance },
			uSaturation: { value: saturation },
			uNoiseAmount: { value: noiseAmount },
			uDistortion: { value: distortion },
		}}
	/>
</T.Mesh>
