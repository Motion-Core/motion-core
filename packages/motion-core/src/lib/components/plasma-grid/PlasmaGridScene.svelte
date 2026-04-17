<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Program,
		Renderer,
		Transform,
		Triangle,
		Vec3,
	} from "ogl";
	import { type ColorRepresentation, toLinearRgb } from "../../helpers/color";

	interface Props {
		/**
		 * The base background color of the effect.
		 * @default "#17181A"
		 */
		color?: ColorRepresentation;
		/**
		 * The color used for the plasma noise gradients.
		 * @default "#FF6900"
		 */
		highlightColor?: ColorRepresentation;
	}

	let { color = "#17181A", highlightColor = "#FF6900" }: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let uniforms = $state<{
		u_time: { value: number };
		u_resolution: { value: Vec3 };
		u_baseColor: { value: Vec3 };
		u_gradientColor: { value: Vec3 };
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

		vec3 linearToSrgb(vec3 color) {
			vec3 safe = max(color, vec3(0.0));
			vec3 low = safe * 12.92;
			vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
			vec3 cutoff = step(vec3(0.0031308), safe);
			return mix(low, high, cutoff);
		}

		void main() {
			vec4 fragColor;
			vec2 fragCoord = vUv * u_resolution.xy;
			mainImage(fragColor, fragCoord);
			fragColor.rgb = linearToSrgb(fragColor.rgb);
			gl_FragColor = fragColor;
		}
	`;

	$effect(() => {
		if (!uniforms) return;
		applyColor(uniforms.u_baseColor.value, color, [
			17 / 255,
			17 / 255,
			19 / 255,
		]);
		applyColor(uniforms.u_gradientColor.value, highlightColor, [
			1,
			105 / 255,
			0,
		]);
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

		const initialBaseColor = toLinearRgb(color, [17 / 255, 17 / 255, 19 / 255]);
		const initialHighlightColor = toLinearRgb(highlightColor, [
			1,
			105 / 255,
			0,
		]);

		const localUniforms = {
			u_time: { value: 0 },
			u_resolution: { value: new Vec3(1, 1, 1) },
			u_baseColor: {
				value: new Vec3(
					initialBaseColor[0],
					initialBaseColor[1],
					initialBaseColor[2],
				),
			},
			u_gradientColor: {
				value: new Vec3(
					initialHighlightColor[0],
					initialHighlightColor[1],
					initialHighlightColor[2],
				),
			},
		};

		uniforms = localUniforms;

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms: localUniforms,
			transparent: true,
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
			localUniforms.u_resolution.value.set(width, height, 1);
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
			localUniforms.u_time.value += delta * 0.5;

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
