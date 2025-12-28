<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import {
		Vector4,
		DataTexture,
		RGBAFormat,
		FloatType,
		NearestFilter,
		ClampToEdgeWrapping,
		LinearFilter,
		ShaderMaterial,
	} from "three";
	import { useTexture } from "@threlte/extras";

	type Props = {
		image: string;
		grid: number;
		mouseSize: number;
		strength: number;
		relaxation: number;
		mouseX: number;
		mouseY: number;
	};

	let { image, grid, mouseSize, strength, relaxation, mouseX, mouseY }: Props =
		$props();

	const { size } = useThrelte();

	let time = 0;
	let material = $state<ShaderMaterial>();

	let currentVX = $state(0);
	let currentVY = $state(0);
	let prevX = 0;
	let prevY = 0;

	let canvasWidth = $state(1);
	let canvasHeight = $state(1);
	let imageWidth = $state(1);
	let imageHeight = $state(1);

	const uCoverUniforms = new Vector4(1, 1, 0, 0);

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

		uCoverUniforms.set(scaleX, scaleY, offsetX, offsetY);
	};

	$effect(() => {
		canvasWidth = $size.width;
		canvasHeight = $size.height;
		updateCoverUniforms();
	});

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
    uniform sampler2D uDataTexture;
    uniform sampler2D uTexture;
    uniform vec4 uCoverUniforms;
    varying vec2 vUv;

    void main() {
        vec2 scale = uCoverUniforms.xy;
        vec2 offset = uCoverUniforms.zw;
        vec2 coverUv = vUv * scale + offset;

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
			imageWidth = tex.image.width;
			imageHeight = tex.image.height;
			updateCoverUniforms();
		}
	});

	useTask((delta) => {
		time += delta;
		if (material && dataTexture) {
			material.uniforms.time.value = time;
			material.uniforms.uCoverUniforms.value.copy(uCoverUniforms);

			const data = dataTexture.image.data;
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
				uCoverUniforms: { value: uCoverUniforms },
				uTexture: { value: $texture },
				uDataTexture: { value: dataTexture },
			}}
		/>
	</T.Mesh>
{/if}
