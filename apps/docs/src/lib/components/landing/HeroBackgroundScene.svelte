<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";
	import { onMount } from "svelte";

	interface Props {
		shadow?: string;
		highlight?: string;
	}

	let { shadow, highlight }: Props = $props();

	const COLOR_PRESETS = {
		dark: {
			shadow: "#18181B",
			highlight: "#572400",
		},
		light: {
			shadow: "#FFFFFF",
			highlight: "#FF6900",
		},
	} as const;

	const { size } = useThrelte();

	let mainMaterial = $state<THREE.ShaderMaterial>();
	let isDark = $state(false);

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
		uniform vec3 u_colorShadow;
		uniform vec3 u_colorHighlight;

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

			vec3 col = mix(u_colorShadow, u_colorHighlight,
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
	const colorShadowUniform = new THREE.Vector3();
	const colorHighlightUniform = new THREE.Vector3();

	onMount(() => {
		const observer = new MutationObserver(() => {
			isDark = document.documentElement.classList.contains("dark");
		});
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class"],
		});
		isDark = document.documentElement.classList.contains("dark");
		return () => observer.disconnect();
	});

	$effect(() => {
		const palette = isDark ? COLOR_PRESETS.dark : COLOR_PRESETS.light;
		const shadowColor = new THREE.Color(shadow || palette.shadow);
		const highlightColor = new THREE.Color(highlight || palette.highlight);

		colorShadowUniform.set(shadowColor.r, shadowColor.g, shadowColor.b);
		colorHighlightUniform.set(
			highlightColor.r,
			highlightColor.g,
			highlightColor.b,
		);
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
			u_colorShadow: { value: colorShadowUniform },
			u_colorHighlight: { value: colorHighlightUniform },
		}}
	/>
</T.Mesh>
