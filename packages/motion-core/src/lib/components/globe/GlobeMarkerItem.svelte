<script lang="ts">
	import { T, useTask } from "@threlte/core";
	import { HTML } from "@threlte/extras";
	import * as THREE from "three";
	import type { GlobeMarker } from "./types";

	interface Props {
		/**
		 * The marker data object containing location, color, size, etc.
		 */
		marker: GlobeMarker;
		/**
		 * The radius of the globe sphere. Used to calculate surface projection.
		 */
		radius: number;
		/**
		 * The 3D world position of the marker [x, y, z].
		 */
		position: [number, number, number] | { x: number; y: number; z: number };
		/**
		 * Callback when the pointer enters the marker area.
		 */
		onpointerenter?: () => void;
		/**
		 * Callback when the pointer leaves the marker area.
		 */
		onpointerleave?: () => void;
	}

	let { marker, position, onpointerenter, onpointerleave }: Props = $props();

	let isHovered = $state(false);

	function handlePointerEnter() {
		isHovered = true;
		onpointerenter?.();
	}

	function handlePointerLeave() {
		isHovered = false;
		onpointerleave?.();
	}

	let pinHeight = $derived(marker.pinHeight ?? 0.75);
	let headRadius = $derived(marker.headRadius ?? 0.1);

	let group = $state<THREE.Group>();
	let pulseMesh = $state<THREE.Mesh>();
	let pulseMaterial = $state<THREE.MeshBasicMaterial>();

	let pulseTimer = 0;

	useTask((delta) => {
		if (pulseMesh && pulseMaterial) {
			pulseTimer += delta * 1.5;
			const t = pulseTimer % 1;

			const scale = 1.0 + t * 1.5;
			pulseMesh.scale.setScalar(scale);

			pulseMaterial.opacity = Math.max(0, 0.8 * (1.0 - t));
		}
	});

	let color = $derived(new THREE.Color(marker.color || "#ffffff"));
	let scale = $derived((marker.size || 0.1) * 3);
	let normalizedPosition = $derived(
		Array.isArray(position)
			? position
			: ([position.x, position.y, position.z] as [number, number, number]),
	);

	$effect(() => {
		if (group) {
			group.lookAt(new THREE.Vector3(0, 0, 0));
		}
	});
</script>

<T.Group
	bind:ref={group}
	position={normalizedPosition}
	scale={[scale, scale, scale]}
>
	<T.Mesh position.z={-pinHeight / 2} rotation.x={Math.PI / 2}>
		<T.CylinderGeometry args={[0.02, 0.02, pinHeight, 8]} />
		<T.MeshBasicMaterial {color} />
	</T.Mesh>

	<T.Mesh
		position.z={-pinHeight}
		onpointerenter={handlePointerEnter}
		onpointerleave={handlePointerLeave}
	>
		<T.SphereGeometry args={[headRadius, 16, 16]} />
		<T.MeshBasicMaterial {color} />
	</T.Mesh>

	<T.Mesh bind:ref={pulseMesh} position.z={-pinHeight}>
		<T.SphereGeometry args={[headRadius, 16, 16]} />
		<T.MeshBasicMaterial
			bind:ref={pulseMaterial}
			{color}
			transparent
			depthWrite={false}
		/>
	</T.Mesh>

	{#if marker.label}
		<HTML
			position={[0, 0, -pinHeight - headRadius * 2]}
			center
			pointerEvents="none"
		>
			<div
				class="pointer-events-none absolute -top-6 left-1/2 -translate-x-1/2 rounded bg-neutral-900/80 px-2 py-1 text-xs whitespace-nowrap text-white backdrop-blur-sm transition-opacity duration-200"
				class:opacity-100={isHovered}
				class:opacity-0={!isHovered}
			>
				{marker.label}
			</div>
		</HTML>
	{/if}
</T.Group>
