<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import { Vector2, NearestFilter, LinearFilter, ShaderMaterial } from "three";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Initial grid size for the pixelation effect.
		 * @default 6.0
		 */
		initialGridSize?: number;
		/**
		 * Duration of each step in the depixelation animation.
		 * @default 0.15
		 */
		stepDuration?: number;
	}

	let { image, initialGridSize = 6.0, stepDuration = 0.15 }: Props = $props();

	const { size } = useThrelte();
	let time = 0;
	let currentGridSize = $state(6.0);
	let isDone = $state(false);
	let material = $state<ShaderMaterial>();

	const resolutionUniform = new Vector2(1, 1);
	const textureSizeUniform = new Vector2(1, 1);

	$effect(() => {
		const nextWidth = $size.width;
		const nextHeight = $size.height;
		resolutionUniform.set(nextWidth, nextHeight);
	});

	$effect(() => {
		currentGridSize = initialGridSize;
		time = 0;
		isDone = false;
	});

	const texture = $derived(
		useTexture(image, {
			transform: (tex) => {
				tex.magFilter = NearestFilter;
				tex.minFilter = NearestFilter;
				tex.generateMipmaps = false;
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
		if (!isDone) {
			time += delta;

			const step = Math.floor(time / stepDuration);
			const nextGrid = initialGridSize * Math.pow(2, step);

			currentGridSize = nextGrid;

			if (currentGridSize > $size.height) {
				isDone = true;
				if ($texture) {
					$texture.magFilter = LinearFilter;
					$texture.minFilter = LinearFilter;
					$texture.needsUpdate = true;
				}
			}
		}
		if (material) {
			material.uniforms.uGridSize.value = currentGridSize;
			material.uniforms.uIsDone.value = isDone;
		}
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
    uniform vec2 uResolution;
    uniform vec2 uTextureSize;
    uniform float uGridSize;
    uniform bool uIsDone;

    varying vec2 vUv;

    vec2 getCoverUV(vec2 uv, vec2 textureSize) {
       vec2 s = uResolution / textureSize;
       float scale = max(s.x, s.y);
       vec2 scaledSize = textureSize * scale;
       vec2 offset = (uResolution - scaledSize) * 0.5;
       return (uv * uResolution - offset) / scaledSize;
    }

    void main() {
       vec2 s = uResolution;
       float rs = s.x / max(s.y, 0.00001);

       vec2 grid = vec2(uGridSize * rs, uGridSize);
       vec2 pixelatedScreenUv = floor(vUv * grid) / grid + (0.5 / grid);

       vec2 finalUv = uIsDone ? vUv : pixelatedScreenUv;
       vec2 coverUv = getCoverUV(finalUv, uTextureSize);

       vec4 color = texture2D(uTexture, coverUv);
       gl_FragColor = color;
       #include <colorspace_fragment>
    }
  `;
</script>

{#if $texture}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={material}
			{vertexShader}
			{fragmentShader}
			uniforms={{
				uTexture: { value: $texture },
				uResolution: { value: resolutionUniform },
				uTextureSize: { value: textureSizeUniform },
				uGridSize: { value: initialGridSize },
				uIsDone: { value: false },
			}}
		/>
	</T.Mesh>
{/if}
