<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import * as THREE from "three";

	interface Props {
		/**
		 * Source URL of the color texture.
		 */
		colorSrc: string;
		/**
		 * Source URL of the grayscale depth map texture.
		 */
		depthSrc: string;
		/**
		 * Horizontal displacement threshold.
		 * @default 8
		 */
		xThreshold?: number;
		/**
		 * Vertical displacement threshold.
		 * @default 8
		 */
		yThreshold?: number;
		/**
		 * Pointer sensitivity multiplier applied before displacement thresholding.
		 * @default 0.25
		 */
		sensitivity?: number;
	}

	let {
		colorSrc,
		depthSrc,
		xThreshold = 8,
		yThreshold = 8,
		sensitivity = 0.25,
	}: Props = $props();

	const { size, renderer, camera } = useThrelte();

	let meshRef = $state<THREE.Mesh>();
	let materialRef = $state<THREE.ShaderMaterial>();

	const targetPointer = new THREE.Vector2(0, 0);
	const smoothPointer = new THREE.Vector2(0, 0);

	const thresholdUniform = new THREE.Vector2(0, 0);
	const uniforms = {
		u_original_texture: { value: null as THREE.Texture | null },
		u_depth_texture: { value: null as THREE.Texture | null },
		u_mouse: { value: new THREE.Vector2(0, 0) },
		u_threshold: { value: thresholdUniform },
	};

	const vertexShader = `
		varying vec2 vUv;
		void main() {
			vUv = uv;
			vec4 modelViewPosition = modelViewMatrix * vec4(position, 1.0);
			gl_Position = projectionMatrix * modelViewPosition;
		}
	`;

	const fragmentShader = `
		precision mediump float;
		uniform sampler2D u_original_texture;
		uniform sampler2D u_depth_texture;
		uniform vec2 u_mouse;
		uniform vec2 u_threshold;

		varying vec2 vUv;

		vec2 mirrored(vec2 v) {
			vec2 m = mod(v, 2.0);
			return mix(m, 2.0 - m, step(1.0, m));
		}

		void main() {
			vec4 depthMap = texture2D(u_depth_texture, mirrored(vUv));
			vec2 fake3d = vec2(
				vUv.x + (depthMap.r - 0.5) * u_mouse.x / u_threshold.x,
				vUv.y + (depthMap.r - 0.5) * u_mouse.y / u_threshold.y
			);
			gl_FragColor = texture2D(u_original_texture, mirrored(fake3d));
			#include <colorspace_fragment>
		}
	`;

	const colorTexture = $derived(
		useTexture(colorSrc, {
			transform: (texture) => {
				texture.wrapS = THREE.ClampToEdgeWrapping;
				texture.wrapT = THREE.ClampToEdgeWrapping;
				texture.minFilter = THREE.LinearMipmapLinearFilter;
				texture.magFilter = THREE.LinearFilter;
				texture.colorSpace = THREE.SRGBColorSpace;
				texture.generateMipmaps = true;
				texture.needsUpdate = true;
				return texture;
			},
		}),
	);

	const depthTexture = $derived(
		useTexture(depthSrc, {
			transform: (texture) => {
				texture.wrapS = THREE.ClampToEdgeWrapping;
				texture.wrapT = THREE.ClampToEdgeWrapping;
				texture.minFilter = THREE.LinearMipmapLinearFilter;
				texture.magFilter = THREE.LinearFilter;
				texture.colorSpace = THREE.NoColorSpace;
				texture.generateMipmaps = true;
				texture.needsUpdate = true;
				return texture;
			},
		}),
	);

	let imageAspect = 1;

	$effect(() => {
		thresholdUniform.set(xThreshold, yThreshold);
	});

	$effect(() => {
		const texture = $colorTexture;
		if (!texture) return;

		uniforms.u_original_texture.value = texture;

		const image = texture.image as
			| { width?: number; height?: number }
			| undefined;
		const width = image?.width ?? 1;
		const height = image?.height ?? 1;
		imageAspect = width / height;
	});

	$effect(() => {
		const texture = $depthTexture;
		if (!texture) return;
		uniforms.u_depth_texture.value = texture;
	});

	$effect(() => {
		const canvas = renderer.domElement;

		const handlePointerMove = (event: PointerEvent) => {
			const rect = canvas.getBoundingClientRect();
			const x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
			const y = -(((event.clientY - rect.top) / rect.height) * 2 - 1);
			targetPointer.set(x, y);
		};

		canvas.addEventListener("pointermove", handlePointerMove);
		return () => {
			canvas.removeEventListener("pointermove", handlePointerMove);
		};
	});

	useTask((delta) => {
		if (!meshRef || !materialRef) return;

		const targetX = targetPointer.x * sensitivity;
		const targetY = targetPointer.y * sensitivity;
		const lerp = Math.min(1, 5 * delta);

		smoothPointer.x += (targetX - smoothPointer.x) * lerp;
		smoothPointer.y += (targetY - smoothPointer.y) * lerp;

		materialRef.uniforms.u_mouse.value.set(smoothPointer.x, smoothPointer.y);

		const perspectiveCamera = $camera as THREE.PerspectiveCamera;
		const distance = Math.abs(
			perspectiveCamera.position.z - meshRef.position.z,
		);
		const fovRadians = THREE.MathUtils.degToRad(perspectiveCamera.fov);
		const planeHeight = 2 * Math.tan(fovRadians / 2) * distance;
		const planeWidth = (planeHeight * $size.width) / $size.height;

		const viewportAspect = planeWidth / planeHeight;
		const finalWidth =
			viewportAspect > imageAspect ? planeWidth : planeHeight * imageAspect;
		const finalHeight =
			viewportAspect > imageAspect ? planeWidth / imageAspect : planeHeight;

		meshRef.scale.set(finalWidth / 2, finalHeight / 2, 1);
	});
</script>

{#if $colorTexture && $depthTexture}
	<T.Mesh bind:ref={meshRef}>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={materialRef}
			{vertexShader}
			{fragmentShader}
			{uniforms}
			transparent
			depthTest={false}
			depthWrite={false}
		/>
	</T.Mesh>
{/if}
