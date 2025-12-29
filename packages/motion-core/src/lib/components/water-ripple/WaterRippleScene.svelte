<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import * as THREE from "three";
	import brushUrl from "../../assets/water-ripple-brush.png";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Size of the ripple brush.
		 * @default 100
		 */
		brushSize?: number;
	}

	let { image, brushSize = 100 }: Props = $props();

	const { size, renderer } = useThrelte();

	const max = 100;
	const meshRefs: THREE.Mesh[] = [];
	const brushScene = new THREE.Scene();
	const brushCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 10);

	let fboBase = $state<THREE.WebGLRenderTarget | undefined>(undefined);

	let currentWave = 0;
	let prevMouse = new THREE.Vector2(0, 0);

	const vertexShader = `
		varying vec2 vUv;
		void main() {
			vUv = uv;
			gl_Position = vec4(position, 1.0);
		}
	`;

	const fragmentShader = `
		uniform sampler2D uTexture;
		uniform sampler2D uDisplacement;
		uniform vec2 uResolution;
		uniform vec2 uCoverScale;
		uniform vec2 uCoverOffset;

		varying vec2 vUv;
		float PI = 3.141592653589793238;

		void main() {
			vec2 coverScale = max(uCoverScale, vec2(0.00001));
			vec2 coverUv = coverScale * vUv + uCoverOffset;

			vec2 vUvScreen = vUv;

			vec4 displacement = texture2D(uDisplacement, vUvScreen);
			float theta = displacement.r * 2.0 * PI;

			vec2 dir = vec2(sin(theta), cos(theta));
			vec2 finalUv = coverUv + dir * displacement.r * 0.05;

			vec4 color = texture2D(uTexture, finalUv);

			gl_FragColor = color;
			#include <colorspace_fragment>
		}
	`;

	const resolutionUniform = new THREE.Vector2(1, 1);
	const coverScaleUniform = new THREE.Vector2(1, 1);
	const coverOffsetUniform = new THREE.Vector2(0, 0);
	let mainMaterial = $state<THREE.ShaderMaterial>();

	const textures = $derived(
		useTexture([brushUrl, image], {
			transform: (tex) => {
				tex.colorSpace = THREE.SRGBColorSpace;
				return tex;
			},
		}),
	);

	$effect(() => {
		if ($textures && $textures[0]) {
			const brushTexture = $textures[0];
			brushTexture.rotation = 0;

			while (brushScene.children.length > 0) {
				brushScene.remove(brushScene.children[0]);
			}
			meshRefs.length = 0;

			const geometry = new THREE.PlaneGeometry(brushSize, brushSize, 1, 1);
			const material = new THREE.MeshBasicMaterial({
				map: brushTexture,
				transparent: true,
				opacity: 0,
				depthTest: false,
				depthWrite: false,
				blending: THREE.AdditiveBlending,
			});

			for (let i = 0; i < max; i++) {
				const mesh = new THREE.Mesh(geometry, material.clone());
				mesh.visible = false;
				mesh.rotation.z = Math.random() * Math.PI * 2;
				brushScene.add(mesh);
				meshRefs.push(mesh);
			}
		}
	});

	$effect(() => {
		const width = $size.width;
		const height = $size.height;

		resolutionUniform.set(width, height);

		brushCamera.left = -width / 2;
		brushCamera.right = width / 2;
		brushCamera.top = height / 2;
		brushCamera.bottom = -height / 2;
		brushCamera.updateProjectionMatrix();

		const target = new THREE.WebGLRenderTarget(width, height, {
			minFilter: THREE.LinearFilter,
			magFilter: THREE.LinearFilter,
			format: THREE.RGBAFormat,
		});

		fboBase = target;

		updateCoverUniforms();

		return () => {
			target.dispose();
		};
	});

	let imageWidth = 1;
	let imageHeight = 1;

	$effect(() => {
		if ($textures && $textures[1] && $textures[1].image) {
			imageWidth = $textures[1].image.width;
			imageHeight = $textures[1].image.height;
			updateCoverUniforms();
		}
	});

	const updateCoverUniforms = () => {
		const screenAspect = $size.width / $size.height;
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

		coverScaleUniform.set(scaleX, scaleY);
		coverOffsetUniform.set(offsetX, offsetY);
	};

	const setNewWave = (x: number, y: number, index: number) => {
		const mesh = meshRefs[index];
		if (mesh) {
			mesh.position.x = x;
			mesh.position.y = y;
			mesh.visible = true;
			(mesh.material as THREE.MeshBasicMaterial).opacity = 1;
			mesh.scale.set(1.5, 1.5, 1.5);
		}
	};

	const onPointerMove = (e: PointerEvent) => {
		const rect = renderer.domElement.getBoundingClientRect();
		const x = e.clientX - rect.left - rect.width / 2;
		const y = -(e.clientY - rect.top - rect.height / 2);

		if (Math.abs(x - prevMouse.x) > 4 || Math.abs(y - prevMouse.y) > 4) {
			currentWave = (currentWave + 1) % max;
			setNewWave(x, y, currentWave);
			prevMouse.set(x, y);
		}
	};

	$effect(() => {
		const canvas = renderer.domElement;
		canvas.addEventListener("pointermove", onPointerMove);
		return () => {
			canvas.removeEventListener("pointermove", onPointerMove);
		};
	});

	useTask((delta) => {
		if (!fboBase || !$textures || !$textures[1]) return;

		const timeScale = delta * 60;

		meshRefs.forEach((mesh) => {
			if (mesh.visible) {
				mesh.rotation.z += 0.02 * timeScale;
				(mesh.material as THREE.MeshBasicMaterial).opacity *= Math.pow(
					0.96,
					timeScale,
				);
				mesh.scale.x = 0.982 * mesh.scale.x + 0.108 * timeScale;
				const decay = Math.pow(0.96, timeScale);
				(mesh.material as THREE.MeshBasicMaterial).opacity *= decay;

				mesh.scale.x = 0.982 * mesh.scale.x + 0.108;
				mesh.scale.y = 0.982 * mesh.scale.y + 0.108;

				if ((mesh.material as THREE.MeshBasicMaterial).opacity < 0.002) {
					mesh.visible = false;
				}
			}
		});

		const currentRenderTarget = renderer.getRenderTarget();
		renderer.setRenderTarget(fboBase);
		renderer.clear();
		renderer.render(brushScene, brushCamera);

		if (mainMaterial) {
			mainMaterial.uniforms.uDisplacement.value = fboBase.texture;
			mainMaterial.uniforms.uTexture.value = $textures[1];
		}

		renderer.setRenderTarget(currentRenderTarget);
	});
</script>

{#if $textures && $textures[1]}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={mainMaterial}
			{vertexShader}
			{fragmentShader}
			uniforms={{
				uTexture: { value: $textures[1] },
				uDisplacement: { value: null },
				uResolution: { value: resolutionUniform },
				uCoverScale: { value: coverScaleUniform },
				uCoverOffset: { value: coverOffsetUniform },
			}}
		/>
	</T.Mesh>
{/if}
