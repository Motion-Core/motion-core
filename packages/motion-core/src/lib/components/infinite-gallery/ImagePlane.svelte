<script lang="ts">
	import { T } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		/**
		 * The texture to apply to the plane.
		 */
		texture: THREE.Texture;
		/**
		 * Position of the plane in 3D space [x, y, z].
		 */
		position: [number, number, number];
		/**
		 * Scale of the plane [x, y, z].
		 */
		scale: [number, number, number];
		/**
		 * Shader material to use for the plane.
		 */
		material: THREE.ShaderMaterial;
	}

	let { texture, position, scale, material }: Props = $props();

	let isHovered = $state(false);

	$effect(() => {
		if (material && texture) {
			material.uniforms.map.value = texture;
			material.needsUpdate = true;
		}
	});

	$effect(() => {
		if (material && material.uniforms) {
			material.uniforms.isHovered.value = isHovered ? 1.0 : 0.0;
		}
	});
</script>

<T.Mesh
	{position}
	{scale}
	{material}
	onpointerenter={() => (isHovered = true)}
	onpointerleave={() => (isHovered = false)}
>
	<T.PlaneGeometry args={[1, 1, 32, 32]} />
</T.Mesh>
