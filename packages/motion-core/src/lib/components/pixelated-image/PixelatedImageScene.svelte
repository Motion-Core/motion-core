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
	let canvasWidth = $state(1);
	let canvasHeight = $state(1);
	let imageWidth = $state(1);
	let imageHeight = $state(1);

	const resolutionUniform = new Vector2(1, 1);
	const coverScaleUniform = new Vector2(1, 1);
	const coverOffsetUniform = new Vector2(0, 0);

	const updateCoverUniforms = () => {
		if (
			canvasWidth <= 0 ||
			canvasHeight <= 0 ||
			imageWidth <= 0 ||
			imageHeight <= 0
		) {
			return;
		}

		const screenAspect = canvasWidth / canvasHeight;
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

		scaleX = Math.max(scaleX, 0.00001);
		scaleY = Math.max(scaleY, 0.00001);

		coverScaleUniform.set(scaleX, scaleY);
		coverOffsetUniform.set(offsetX, offsetY);
	};

	$effect(() => {
		const nextWidth = $size.width;
		const nextHeight = $size.height;
		canvasWidth = nextWidth;
		canvasHeight = nextHeight;
		resolutionUniform.set(nextWidth, nextHeight);
		updateCoverUniforms();
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
			imageWidth = tex.image.width;
			imageHeight = tex.image.height;
			updateCoverUniforms();
		}
	});

	useTask((delta) => {
		if (!isDone) {
			time += delta;

			const step = Math.floor(time / stepDuration);
			const nextGrid = initialGridSize * Math.pow(2, step);

			currentGridSize = nextGrid;

			if (currentGridSize > canvasHeight) {
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
    uniform vec2 uCoverScale;
    uniform vec2 uCoverOffset;
    uniform float uGridSize;
    uniform bool uIsDone;

    varying vec2 vUv;

    void main() {
       vec2 s = uResolution;
       float rs = s.x / max(s.y, 0.00001);

       vec2 grid = vec2(uGridSize * rs, uGridSize);
       vec2 pixelatedScreenUv = floor(vUv * grid) / grid + (0.5 / grid);

       vec2 finalUv = uIsDone ? vUv : pixelatedScreenUv;
       vec2 coverScale = max(uCoverScale, vec2(0.00001));
       vec2 coverUv = coverScale * finalUv + uCoverOffset;

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
				uCoverScale: { value: coverScaleUniform },
				uCoverOffset: { value: coverOffsetUniform },
				uGridSize: { value: initialGridSize },
				uIsDone: { value: false },
			}}
		/>
	</T.Mesh>
{/if}
