<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		/**
		 * Base color of the specular bands.
		 * @default "#FF6900"
		 */
		color?: THREE.ColorRepresentation;
		/**
		 * Color of the background.
		 * @default "#000000"
		 */
		backgroundColor?: THREE.ColorRepresentation;
		/**
		 * Animation speed multiplier.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Lens distortion intensity.
		 * @default 0.2
		 */
		distortion?: number;
		/**
		 * Amount of hue shift for secondary bands (in degrees).
		 * @default 30.0
		 */
		hueShift?: number;
		/**
		 * Global intensity/brightness of the effect.
		 * @default 1.0
		 */
		intensity?: number;
	}

	let {
		color = "#FF6900",
		backgroundColor = "#000000",
		speed = 1.0,
		distortion = 0.2,
		hueShift = 30.0,
		intensity = 1.0,
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
		uniform float uSpeed;
		uniform float uDistortion;
		uniform float uHueShift;
		uniform float uIntensity;

		mat3 hueRot(float a) {
			float c = cos(a), s = sin(a), t = 1.0 - c;
			return mat3(
				t*.333+c,    t*.333-s*.577, t*.333+s*.577,
				t*.333+s*.577, t*.333+c,   t*.333-s*.577,
				t*.333-s*.577, t*.333+s*.577, t*.333+c
			);
		}

		void mainImage(out vec4 o, vec2 uv) {
			vec2 u = (uv * 2.0 - 1.0);
			u.x *= uResolution.x / uResolution.y;

			float time = uTime * uSpeed;

			u /= 0.5 + uDistortion * dot(u, u);
			u += 0.2 * cos(time) - 7.56;

			vec3 baseColor = uColor;

			vec3 palette[3];
			palette[0] = baseColor;
			palette[1] = hueRot(radians(uHueShift)) * baseColor;
			palette[2] = hueRot(radians(-uHueShift)) * baseColor;

			vec3 col = vec3(0.0);
			for(int i = 0; i < 3; i++) {
				vec2 uv_loop = sin(1.5 * u.yx + 2.0 * cos(u -= 0.01));
				float val = 1.0 - exp(-6.0 / exp(6.0 * length(uv_loop + sin(5.0 * uv_loop.y - 3.0 * time) / 4.0)));
				// sharpening only the shape scalar to preserve hue
				val = pow(val, 2.2);
				col += val * palette[i];
			}
			vec3 bands = col * uIntensity;
			o = vec4(uBackgroundColor + bands, 1.0);
		}

		void main() {
			vec4 fragColor;
			mainImage(fragColor, vUv);
			gl_FragColor = fragColor;
			#include <colorspace_fragment>
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
		material.uniforms.uSpeed.value = speed;
		material.uniforms.uDistortion.value = distortion;
		material.uniforms.uHueShift.value = hueShift;
		material.uniforms.uIntensity.value = intensity;
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
			uSpeed: { value: speed },
			uDistortion: { value: distortion },
			uHueShift: { value: hueShift },
			uIntensity: { value: intensity },
		}}
	/>
</T.Mesh>
