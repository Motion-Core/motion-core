<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		/**
		 * The base background color of the effect.
		 * @default "#111113"
		 */
		color?: string;
		/**
		 * The color used for the plasma noise gradients.
		 * @default "#FF6900"
		 */
		highlightColor?: string;
	}

	let { color = "#111113", highlightColor = "#FF6900" }: Props = $props();

	const { size } = useThrelte();

	let mainMaterial = $state<THREE.ShaderMaterial>();

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
		uniform float u_time;
		uniform vec3 u_resolution;
		uniform vec3 u_baseColor;
		uniform vec3 u_gradientColor;

		float rand(vec2 p) {
			return fract(sin(dot(p, vec2(12.543,514.123)))*4732.12);
		}

		float noise(vec2 p) {
			vec2 f = smoothstep(0.0, 1.0, fract(p));
			vec2 i = floor(p);
			float a = rand(i);
			float b = rand(i+vec2(1.0,0.0));
			float c = rand(i+vec2(0.0,1.0));
			float d = rand(i+vec2(1.0,1.0));
			return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
		}

		void mainImage( out vec4 fragColor, in vec2 fragCoord ) {
			float n = 2.0;
			vec2 uv = fragCoord/u_resolution.y;
			vec2 uvp = fragCoord/u_resolution.xy;
			uv += 0.75*noise(uv*3.0+u_time/2.0+noise(uv*7.0-u_time/3.0)/2.0)/2.0;

			float grid = (mod(floor((uvp.x)*u_resolution.x/n),2.0)==0.0?1.0:0.0) *
						 (mod(floor((uvp.y)*u_resolution.y/n),2.0)==0.0?1.0:0.0);

			vec3 col = mix(u_baseColor, u_gradientColor,
						   5.0 * vec3(pow(1.0-noise(uv*4.0-vec2(0.0, u_time/2.0)), 5.0)));

			col = pow(col, vec3(1.0));
			float alpha = grid;
			fragColor = vec4(col, alpha);
		}

		void main() {
			vec4 fragColor;
			vec2 fragCoord = vUv * u_resolution.xy;
			mainImage(fragColor, fragCoord);
			gl_FragColor = fragColor;
            #include <colorspace_fragment>
		}
	`;

	const resolutionUniform = new THREE.Vector3(1, 1, 1);
	const baseColorUniform = new THREE.Vector3();
	const gradientColorUniform = new THREE.Vector3();

	$effect(() => {
		const base = new THREE.Color(color);
		const gradient = new THREE.Color(highlightColor);

		baseColorUniform.set(base.r, base.g, base.b);
		gradientColorUniform.set(gradient.r, gradient.g, gradient.b);
	});

	$effect(() => {
		resolutionUniform.set($size.width, $size.height, 1.0);
	});

	useTask((delta) => {
		if (mainMaterial) {
			mainMaterial.uniforms.u_time.value += delta * 0.5;
		}
	});
</script>

<T.Mesh>
	<T.PlaneGeometry args={[2, 2]} />
	<T.ShaderMaterial
		bind:ref={mainMaterial}
		{vertexShader}
		{fragmentShader}
		transparent
		depthTest={false}
		depthWrite={false}
		uniforms={{
			u_time: { value: 0 },
			u_resolution: { value: resolutionUniform },
			u_baseColor: { value: baseColorUniform },
			u_gradientColor: { value: gradientColorUniform },
		}}
	/>
</T.Mesh>
