<script lang="ts">
	import { T } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		texture: THREE.Texture;
		position: [number, number, number];
		scale: [number, number, number];
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
