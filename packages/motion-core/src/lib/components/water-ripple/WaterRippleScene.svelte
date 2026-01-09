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
		uniform vec2 uTextureSize;

		varying vec2 vUv;
		float PI = 3.141592653589793238;

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 s = uResolution / textureSize;
			float scale = max(s.x, s.y);
			vec2 scaledSize = textureSize * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		void main() {
			vec2 coverUv = getCoverUV(vUv, uTextureSize);

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
	const textureSizeUniform = new THREE.Vector2(1, 1);
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

		return () => {
			target.dispose();
		};
	});

	$effect(() => {
		if ($textures && $textures[1] && $textures[1].image) {
			textureSizeUniform.set(
				$textures[1].image.width,
				$textures[1].image.height,
			);
		}
	});

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
				uTextureSize: { value: textureSizeUniform },
			}}
		/>
	</T.Mesh>
{/if}
