<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import {
		Vector2,
		DataTexture,
		RGBAFormat,
		FloatType,
		NearestFilter,
		ClampToEdgeWrapping,
		LinearFilter,
		ShaderMaterial,
	} from "three";
	import { useTexture } from "@threlte/extras";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Grid resolution (number of cells per row/column).
		 */
		grid: number;
		/**
		 * Radius of mouse influence.
		 */
		mouseSize: number;
		/**
		 * Strength of the distortion effect.
		 */
		strength: number;
		/**
		 * Relaxation factor for returning to original state (0-1).
		 */
		relaxation: number;
		/**
		 * Current normalized mouse X position.
		 */
		mouseX: number;
		/**
		 * Current normalized mouse Y position.
		 */
		mouseY: number;
	}

	let { image, grid, mouseSize, strength, relaxation, mouseX, mouseY }: Props =
		$props();

	const { size } = useThrelte();

	let time = 0;
	let material = $state<ShaderMaterial>();

	let currentVX = $state(0);
	let currentVY = $state(0);
	let prevX = 0;
	let prevY = 0;

	const resolutionUniform = new Vector2(1, 1);
	const textureSizeUniform = new Vector2(1, 1);

	function regenerateGrid(gridSize: number) {
		const size = gridSize * gridSize;
		const data = new Float32Array(4 * size);

		for (let i = 0; i < size; i++) {
			const r = Math.random() * 255 - 125;
			const r1 = Math.random() * 255 - 125;
			const stride = i * 4;

			data[stride] = r;
			data[stride + 1] = r1;
			data[stride + 2] = r;
			data[stride + 3] = 255;
		}

		const texture = new DataTexture(
			data,
			gridSize,
			gridSize,
			RGBAFormat,
			FloatType,
		);
		texture.magFilter = NearestFilter;
		texture.minFilter = NearestFilter;
		texture.needsUpdate = true;
		return texture;
	}

	let dataTexture = $derived.by(() => {
		return regenerateGrid(grid);
	});

	$effect(() => {
		currentVX = mouseX - prevX;
		currentVY = mouseY - prevY;
		prevX = mouseX;
		prevY = mouseY;
	});

	const vertexShader = `
    varying vec2 vUv;
    void main() {
      vUv = uv;
      gl_Position = vec4(position, 1.0);
    }
  `;

	const fragmentShader = `
    uniform float time;
    uniform vec2 uResolution;
    uniform vec2 uTextureSize;
    uniform sampler2D uDataTexture;
    uniform sampler2D uTexture;
    varying vec2 vUv;

    vec2 getCoverUV(vec2 uv, vec2 textureSize) {
        vec2 s = uResolution / textureSize;
        float scale = max(s.x, s.y);
        vec2 scaledSize = textureSize * scale;
        vec2 offset = (uResolution - scaledSize) * 0.5;
        return (uv * uResolution - offset) / scaledSize;
    }

    void main() {
        vec2 coverUv = getCoverUV(vUv, uTextureSize);

        vec4 data = texture2D(uDataTexture, vUv);
        vec2 displacedUV = coverUv - 0.02 * data.rg;

        vec4 color = texture2D(uTexture, displacedUV);
        gl_FragColor = color;
        #include <colorspace_fragment>
    }
  `;

	const texture = $derived(
		useTexture(image, {
			transform: (tex) => {
				tex.wrapS = ClampToEdgeWrapping;
				tex.wrapT = ClampToEdgeWrapping;
				tex.minFilter = LinearFilter;
				tex.magFilter = LinearFilter;
				return tex;
			},
		}),
	);

	$effect(() => {
		const tex = $texture;
		if (tex && tex.image) {
			textureSizeUniform.set(tex.image.width, tex.image.height);
		}
	});

	useTask((delta) => {
		time += delta;
		resolutionUniform.set($size.width, $size.height);
		if (material && dataTexture) {
			material.uniforms.time.value = time;

			const data = dataTexture.image.data;
			if (!data) return;
			const sizeSq = grid;
			const gridMouseX = sizeSq * mouseX;
			const gridMouseY = sizeSq * (1 - mouseY);
			const maxDist = sizeSq * mouseSize;
			const aspect = $size.height / $size.width;
			const maxDistSq = maxDist ** 2;

			for (let i = 0; i < sizeSq; i++) {
				for (let j = 0; j < sizeSq; j++) {
					const distance =
						(gridMouseX - i) ** 2 / aspect + (gridMouseY - j) ** 2;

					if (distance < maxDistSq) {
						const index = 4 * (i + sizeSq * j);
						let power = maxDist / Math.sqrt(distance);
						if (power > 10) power = 10;

						data[index] += strength * 100 * currentVX * power;
						data[index + 1] -= strength * 100 * currentVY * power;
					}

					const idx = 4 * (i + sizeSq * j);
					data[idx] *= relaxation;
					data[idx + 1] *= relaxation;
				}
			}

			currentVX *= 0.9;
			currentVY *= 0.9;
			dataTexture.needsUpdate = true;
		}
	});
</script>

{#if $texture && dataTexture}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={material}
			{vertexShader}
			{fragmentShader}
			transparent
			uniforms={{
				time: { value: 0 },
				uResolution: { value: resolutionUniform },
				uTextureSize: { value: textureSizeUniform },
				uTexture: { value: $texture },
				uDataTexture: { value: dataTexture },
			}}
		/>
	</T.Mesh>
{/if}
